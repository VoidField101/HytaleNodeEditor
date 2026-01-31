use std::collections::HashMap;

use egui::{Color32, Pos2, Ui};

use crate::{
    workspace::nodes::Connector,
};

#[derive(Clone)]
pub struct HyNodePin {
    pub name: String,
    pub color: Color32,
    pub allow_multiple: bool,
}

#[derive(Clone)]
pub struct HyNode {
    pub label: String,
    pub inputs: Vec<HyNodePin>,
    pub outputs: Vec<HyNodePin>,
}

#[derive(Clone)]
pub struct HyNodeProto {
    pub pos: Pos2,
    pub label: String,
    pub inputs: Vec<HyNodePin>,
    pub outputs: Vec<HyNodePin>,
}

#[derive(Default)]
pub struct HyConnection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

impl HyNode {
    pub fn draw_content(&mut self, ui: &mut Ui) {

        
    }
}

impl From<HyNodeProto> for HyNode {
    fn from(value: HyNodeProto) -> Self {
        Self {
            label: value.label,
            inputs: value.inputs,
            outputs: value.outputs,
        }
    }
}

impl From<Connector> for HyNodePin {
    fn from(value: Connector) -> Self {
        Self {
            name: value.label,
            color: value.color.into(),
            allow_multiple: value.multiple,
        }
    }
}
