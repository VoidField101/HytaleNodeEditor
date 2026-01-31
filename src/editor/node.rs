use std::collections::HashMap;

use egui::{Color32, Pos2, Ui};

use crate::{
    editor::{
        EditorError,
        values::{HyNodeContent, NodeValue},
    },
    workspace::{
        self,
        nodes::{Connector, NodeDescription},
        workspace::Workspace,
    },
};

#[derive(Clone)]
pub struct HyNode<'a> {
    pub title: String,
    pub description: &'a NodeDescription,
    pub value: HashMap<String, NodeValue>
}

#[derive(Clone)]
pub struct HyNodeProto<'a> {
    pub pos: Pos2,
    pub variant_index: usize,
    pub workspace: &'a Workspace,
    pub value: HashMap<String, NodeValue>
}

#[derive(Default)]
pub struct HyConnection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

impl<'a> HyNode<'a> {
    pub fn draw_content(&mut self, ui: &mut Ui) {}
}

impl<'a> TryFrom<HyNodeProto<'a>> for HyNode<'a> {
    type Error = EditorError;

    fn try_from(value: HyNodeProto<'a>) -> Result<Self, Self::Error> {
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

        Ok(Self {
            title: desc.title.to_string(),
            description: desc,
        })
    }
}
/*
impl From<Connector> for HyNodePin {
    fn from(value: Connector) -> Self {
        Self {
            name: value.label,
            color: value.color.into(),
            allow_multiple: value.multiple,
        }
    }
}
*/
