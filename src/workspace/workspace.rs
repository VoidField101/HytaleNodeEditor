use std::{borrow::Borrow, collections::HashMap};

use serde::{Deserialize, Deserializer, Serialize};

use crate::workspace::{color::ColorValue, groups::NodeGroup, nodes::NodeDescription};

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

#[derive(Debug, Clone)]
pub struct Workspace {
    pub workspace: WorkspaceSchema,
    pub groups: Vec<NodeGroup>
}

#[allow(unused)]
fn default_allow_exports() -> bool {
    false
}

impl Workspace {
    pub fn construct(schema: WorkspaceSchema, nodes: Vec<NodeDescription>) -> Workspace {
        let mut node_map = nodes.into_iter().map(|node| (node.id.clone(), node)).collect::<HashMap<_, _>>();
        let mut groups = Vec::with_capacity(schema.node_categories.len() + 1);

        schema.node_categories.iter().for_each(|category|{
            let nodes = category.1.iter().filter_map(|node_id| node_map.remove(node_id)).collect::<Vec<_>>();
            
            let mut color = Default::default();
            if let Some(color2) = nodes.first().map(|desc| desc.color) {
                if nodes.iter().find(|description| description.color != color2).is_none() {
                    color = color2;
                }
            }
            
            groups.push(NodeGroup { color: color, name: category.0.to_owned(), nodes });
        });

        if !node_map.is_empty(){
            let nodes = node_map.drain().map(|(k,v)| v).collect();
            groups.push(NodeGroup { color: Default::default(), name: "Uncategorized".to_owned(), nodes });
        }

        Workspace { workspace:schema, groups: groups}
    }
}
