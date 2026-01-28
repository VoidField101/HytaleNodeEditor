use std::{fs, io, path::Path};

use crate::workspace::{color::ColorValue, nodes::NodeDescription};

#[derive(Debug, Clone, Default)]
pub struct NodeGroup {
    pub color: ColorValue,
    pub name: String,
    pub nodes: Vec<NodeDescription>,
}
