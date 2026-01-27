use std::{fs, io, path::Path};

use crate::nodes::{color::ColorValue, schema::NodeDescription};

#[derive(Debug, Clone, Default)]
pub struct NodeGroup {
    pub color: Option<ColorValue>,
    pub name: String,
    pub nodes: Vec<NodeDescription>,
}

impl NodeGroup {
    pub fn load_group(path: &Path, name: String) -> io::Result<NodeGroup> {
        let entries = fs::read_dir(path)?;
        let mut schemas = Vec::new();

        for entry in entries.flatten() {
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Process JSON files
                if let Ok(content) = fs::read_to_string(&path) {
                    match serde_json::from_str::<NodeDescription>(&content) {
                        Ok(schema) => {
                            schemas.push(schema);
                        }
                        Err(err) => {
                            eprintln!("Failed to parse JSON schema at: {:?}", err);
                        }
                    }
                }
            }
        }

        let color = schemas.get(0).map(|node| node.color);
        Ok(NodeGroup {
            color: color,
            name: name,
            nodes: schemas,
        })
    }
}
