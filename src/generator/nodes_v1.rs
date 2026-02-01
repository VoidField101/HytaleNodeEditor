use core::f32;
use std::collections::HashMap;

use egui::{pos2, vec2};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};

use crate::{
    editor::{self, node::HyConnection, value::NodeEditorValueTypes},
    generator::{
        GeneratorError,
        common::{Group, NodeId, Position, WorksheetInfo},
    },
    workspace::{nodes::NodeDescription, workspace::Workspace},
};

// FIXME: This type is problematic. It assumes (during deserialization) that every object could be a node.
// In reality we want to check the Workspace file for that. A 2-step deserialization is required
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum NodeValue {
    Node(Box<Node>),
    String(String),
    Number(f32),
    Bool(bool),
    List(Vec<NodeValue>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde_with::skip_serializing_none]
pub struct Node {
    #[serde(rename = "$Position", default)]
    pub position: Position,

    #[serde(rename = "$Comment")]
    pub comment: Option<String>,

    #[serde(rename = "$NodeId")]
    pub nodeId: Option<NodeId>,

    #[serde(flatten)]
    pub values: HashMap<String, NodeValue>,
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
    pub values: HashMap<String, NodeValue>,
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
                    NodeValue::Node(node) => {
                        let node_values = &node.values;
                        let sub_description = description
                            .get_variant(workspace, &value.0, |var_key| {
                                node_values.get(var_key).and_then(|val| match val {
                                    NodeValue::String(value) => Some(value.as_str()),
                                    _ => None,
                                })
                            })
                            .ok_or_else(|| {
                                super::GeneratorError::NodeVariantResolve(value.0.clone())
                            })?;

                        outputs.insert(value.0, vec![node.normalize(workspace, sub_description)?]);
                    }
                    NodeValue::List(node_values) => {
                        let mut list = Vec::with_capacity(node_values.len());
                        for elem in node_values.into_iter() {
                            match elem {
                                NodeValue::Node(node) => {
                                    let sub_description = description
                                        .get_variant(workspace, &value.0, |var_key| {
                                            node.values.get(var_key).and_then(|val| match val {
                                                NodeValue::String(value) => Some(value.as_str()),
                                                _ => None,
                                            })
                                        })
                                        .ok_or_else(|| {
                                            super::GeneratorError::NodeVariantResolve(
                                                value.0.clone(),
                                            )
                                        })?;

                                    list.push(node.normalize(workspace, sub_description)?);
                                }
                                _ => {
                                    return Err(super::GeneratorError::UnexpectedNodeType(
                                        value.0,
                                        "object".to_owned(),
                                    )
                                    .into());
                                }
                            }
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
                match value.1 {
                    val @ NodeValue::String(_) => {
                        remaining.insert(value.0, val);
                    }
                    val @ NodeValue::Number(_) => {
                        remaining.insert(value.0, val);
                    }
                    val @ NodeValue::Bool(_) => {
                        remaining.insert(value.0, val);
                    }
                    NodeValue::List(list) => {
                        if list
                            .iter()
                            .find(|elem| match elem {
                                NodeValue::Node(node) => true,
                                _ => false,
                            })
                            .is_some()
                        {
                            return Err(super::GeneratorError::UnexpectedNodeType(
                                value.0,
                                "any non-object".to_owned(),
                            )
                            .into());
                        }

                        remaining.insert(value.0, NodeValue::List(list));
                    }
                    _ => {
                        return Err(super::GeneratorError::UnexpectedNodeType(
                            value.0,
                            "any non-object".to_owned(),
                        )
                        .into());
                    }
                };
            }
        }

        Ok(NormalizedNode {
            position: self.position,
            comment: self.comment,
            node_id: self.nodeId,
            variant: description.id.clone(),
            values: remaining,
            outputs: outputs,
        })
    }
}

impl TryInto<Value> for NodeValue {
    type Error = super::GeneratorError;

    fn try_into(self) -> Result<Value, Self::Error> {
        match self {
            NodeValue::Node(_) => Err(GeneratorError::UnexpectedNodeType(
                "string, number, bool".to_string(),
                "object".to_string(),
            )),
            NodeValue::String(v) => Ok(Value::String(v)),
            NodeValue::Number(v) => {
                Number::from_f64(v as f64)
                    .map(Value::Number)
                    .ok_or_else(|| {
                        GeneratorError::UnexpectedNodeType(
                            "number".to_string(),
                            "unknown".to_string(),
                        )
                    })
            }
            NodeValue::Bool(v) => Ok(Value::Bool(v)),
            NodeValue::List(v) => v
                .into_iter()
                .map::<Result<Value, Self::Error>, _>(Self::try_into)
                .collect(),
        }
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

        // FIXME: Along with using the broken NodeValue type it also looses the desc.content ordering due to the hash map which is randomized every time
       
        let values = self
            .values
            .iter()
            .filter_map(|v: (&String, &NodeValue)| {
                desc.content.iter().find(|c| c.id == *v.0).map(|content| {
                    (
                        v.0.clone(),
                        NodeEditorValueTypes::from_nodevalue(Some(v.1.clone()), &content.options)
                            .unwrap(),
                    )
                })
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
