use std::collections::HashMap;

use derive_where::derive_where;
use egui::{Color32, Pos2, Ui};
use serde_json::Value;

use crate::{
    editor::{
        EditorError
    },
    workspace::{
        self,
        nodes::{Connector, NodeDescription},
        workspace::Workspace,
    },
};

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNode<'a> {
    pub title: String,
    #[derive_where(skip)]
    pub description: &'a NodeDescription,
    pub values: HashMap<String, Value>,
}

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNodeProto<'a> {
    pub pos: Pos2,
    pub variant_index: usize,
    #[derive_where(skip)]
    pub workspace: &'a Workspace,
    pub values: HashMap<String, Value>,
}

#[derive(Default, Debug)]
pub struct HyConnection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

impl<'a> HyNode<'a> {
    pub fn new(description: &'a NodeDescription) -> Self {
        Self {
            title: description.title.clone(),
            description,
            values: description
                .content
                .iter()
                .map(|v| (v.id.clone(), v.options.get_default()))
                .collect(),
        }
    }

    pub fn draw_content(&mut self, ui: &mut Ui) {}
}

impl<'a> TryFrom<HyNodeProto<'a>> for HyNode<'a> {
    type Error = EditorError;

    fn try_from(mut value: HyNodeProto<'a>) -> Result<Self, Self::Error> {
        let desc = value
            .workspace
            .nodes
            .get(value.variant_index)
            .ok_or_else(|| {
                EditorError::NodeVariantIndexResolve(
                    value.variant_index,
                    value.workspace.workspace.workspace_name.clone(),
                )
            })?;

        let values = desc
            .content
            .iter()
            .map(|content| {
                (
                    content.id.clone(),
                    value
                        .values
                        .remove(&content.id)
                        .or_else(|| {
                            let dv = content.options.get_default();
                            if dv.is_null() { None } else { Some(dv) }
                        })
                        .unwrap_or(Value::Null),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            title: desc.title.to_string(),
            description: desc,
            values,
        })
    }
}
