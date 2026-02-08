use core::f32;
use std::collections::HashMap;

use egui::{pos2, vec2};
use serde::{Deserialize, Serialize};

use crate::{
    editor::{self, node::HyConnection, value::NodeEditorValueTypes},
    generator::{
        GeneratorError,
        common::{Group, NodeId, Position, WorksheetInfo},
    },
    workspace::{nodes::NodeDescription, workspace::Workspace},
};

pub type JsonValue = serde_json::Value;
pub type JsonNumber = serde_json::Number;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde_with::skip_serializing_none]
pub struct Node {
    #[serde(rename = "$Position", default)]
    pub position: Position,

    #[serde(rename = "$Comment")]
    pub comment: Option<String>,

    #[serde(rename = "$NodeId")]
    pub node_id: Option<NodeId>,

    #[serde(flatten)]
    pub values: HashMap<String, JsonValue>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootNode {
    #[serde(rename = "$Title", default)]
    pub title: String,
    #[serde(rename = "$WorkspaceID")]
    pub workspace_id: String,
    #[serde(rename = "$Groups")]
    pub groups: Vec<Group>,
    #[serde(flatten)]
    pub node: Node,
}

#[derive(Debug, Clone)]
pub struct NormalizedNode {
    pub position: Position,
    pub comment: Option<String>,
    pub node_id: Option<NodeId>,
    pub variant: String,
    pub values: HashMap<String, JsonValue>,
    pub outputs: HashMap<String, Vec<NormalizedNode>>,
}

impl RootNode {
    pub fn normalize(
        self,
        workspace: &Workspace,
        root_variant: &str,
    ) -> anyhow::Result<(NormalizedNode, WorksheetInfo)> {
        let root_desc = workspace
            .nodes
            .iter()
            .find(|node| node.id == root_variant)
            .expect("TODO");
        let normal = self.node.normalize(workspace, &root_desc)?;
        Ok((
            normal,
            WorksheetInfo {
                title: self.title,
                workspace_id: self.workspace_id,
                groups: self.groups,
            },
        ))
    }
}

impl Node {
    pub fn normalize(
        self,
        workspace: &Workspace,
        description: &NodeDescription,
    ) -> anyhow::Result<NormalizedNode> {
        let mut remaining = HashMap::new();
        let mut outputs = HashMap::new();

        for value in self.values.into_iter() {
            if let Some(_pin) = description.get_pin(&value.0) {
                match value.1 {
                    obj @ JsonValue::Object(_) => {
                        let node = serde_json::from_value::<Node>(obj)?;
                        let node_values: &_ = &node.values;
                        let sub_description = description
                            .get_variant(workspace, &value.0, |var_key| {
                                node_values.get(var_key).and_then(|val| match val {
                                    JsonValue::String(value) => Some(value.as_str()),
                                    _ => None,
                                })
                            })
                            .ok_or_else(|| {
                                super::GeneratorError::NodeVariantResolve(value.0.clone())
                            })?;

                        outputs.insert(value.0, vec![node.normalize(workspace, sub_description)?]);
                    }
                    JsonValue::Array(values) => {
                        let mut list = Vec::with_capacity(values.len());
                        for obj in values.into_iter() {
                            let node = serde_json::from_value::<Node>(obj)?;
                            let node_values: &_ = &node.values;
                            let sub_description = description
                                .get_variant(workspace, &value.0, |var_key| {
                                    node_values.get(var_key).and_then(|val| match val {
                                        JsonValue::String(value) => Some(value.as_str()),
                                        _ => None,
                                    })
                                })
                                .ok_or_else(|| {
                                    super::GeneratorError::NodeVariantResolve(value.0.clone())
                                })?;

                            list.push(node.normalize(workspace, sub_description)?);
                        }
                        outputs.insert(value.0, list);
                    }
                    _ => {
                        return Err(super::GeneratorError::UnexpectedNodeType(
                            value.0,
                            "object".to_owned(),
                        )
                        .into());
                    }
                };
            } else {
                remaining.insert(value.0, value.1);
            }
        }

        Ok(NormalizedNode {
            position: self.position,
            comment: self.comment,
            node_id: self.node_id,
            variant: description.id.clone(),
            values: remaining,
            outputs: outputs,
        })
    }
}

impl NormalizedNode {
    pub fn to_editor<'a>(
        &self,
        workspace: &'a Workspace,
    ) -> (Vec<HyConnection>, Vec<editor::node::HyNodeProto<'a>>) {
        let mut connections = Vec::new();
        let mut nodes = Vec::new();

        let node_map = workspace
            .nodes
            .iter()
            .enumerate()
            .map(|node| (node.1.id.clone(), node.0))
            .collect::<HashMap<_, _>>();

        self.to_editor_internal(&node_map, workspace, &mut connections, &mut nodes);

        let mut x_offset = f32::INFINITY;
        let mut y_offset = f32::INFINITY;
        for pos in nodes.iter().map(|node| node.pos) {
            x_offset = x_offset.min(pos.x);
            y_offset = y_offset.min(pos.y);
        }

        nodes
            .iter_mut()
            .for_each(|node| node.pos = node.pos - vec2(x_offset, y_offset));

        (connections, nodes)
    }

    fn to_editor_internal<'a>(
        &self,
        node_map: &HashMap<String, usize>,
        workspace: &'a Workspace,
        connections: &mut Vec<HyConnection>,
        nodes: &mut Vec<editor::node::HyNodeProto<'a>>,
    ) -> usize {
        // FIXME: Replace unwrap with propper error handling!
        let desc_index = node_map.get(&self.variant).unwrap();
        let desc = &workspace.nodes[*desc_index];
        let new_id = nodes.len();

        let values = desc
            .content
            .iter()
            .map(|content| {
                (
                    content.id.clone(),
                    NodeEditorValueTypes::from_value(
                        self.values
                            .get(&content.id)
                            .map(Clone::clone)
                            .unwrap_or(content.options.get_default().0),
                        &content.options,
                    )
                    .unwrap(),
                )
            })
            .collect();

        nodes.push(editor::node::HyNodeProto {
            pos: pos2(self.position.x as f32, self.position.y as f32),
            variant_index: *desc_index,
            workspace,
            values: values,
        });

        self.outputs
            .iter()
            .enumerate()
            .for_each(|(index, (connector_name, new_nodes))| {
                // FIXME: Replace unwrap with propper error handling!
                let conn_index = desc.get_connector(connector_name).unwrap().0;

                new_nodes.iter().for_each(|node| {
                    let sub_id = node.to_editor_internal(node_map, workspace, connections, nodes);

                    connections.push(HyConnection {
                        from_node: new_id,
                        from_connector: conn_index,
                        to_node: sub_id,
                        to_connector: 0, //TODO: Figure out how multi-input nodes are handled
                    });
                });
            });

        new_id
    }
}
