use serde::{self, Deserialize, Serialize};

use crate::nodes::color::ColorValue;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub id: String,
    pub title: String,
    pub color: ColorValue,

    pub content: Vec<Content>,
    pub outputs: Vec<Connector>,
    pub inputs: Vec<Connector>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    pub id: String,
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
pub struct Connector {
    pub id: String,
    #[serde(rename = "Type")]
    pub typ: String,
    pub color: ColorValue,

    #[serde(default)]
    pub label: String,
    #[serde(default = "default_connector_multiple")]
    pub multiple: bool,
}

#[allow(unused)]
fn default_connector_multiple() -> bool {
    true
}
