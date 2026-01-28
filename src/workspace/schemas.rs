use std::collections::HashMap;

use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WorkspaceSchema {
    pub workspace_name: String,
    #[serde(default = "default_allow_exports")]
    pub export_defaults: bool,
    pub roots: HashMap<String, Root>,
    pub node_categories: HashMap<String, Vec<String>>,
    pub variants: HashMap<String, Variant>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Root {
    pub root_node_type: String,
    pub menu_name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Variant {
    pub variant_field_name: String,
    pub variants: HashMap<String, String>,
}


#[allow(unused)]
fn default_allow_exports() -> bool {
    false
}
