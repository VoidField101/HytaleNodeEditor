use core::f32;
use std::collections::HashMap;

use egui::{pos2, vec2};
use serde::{Deserialize, Serialize};

use crate::{
    editor::{self, node::HyConnection, value::NodeEditorValueTypes},
    generator::{
        GeneratorError, JsonValue, common::{Group, NodeId, Position, WorksheetInfo}, norm::NormalizedNode
    },
    workspace::{nodes::NodeDescription, workspace::Workspace},
};

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
