use std::{fs, io, path::Path};

use crate::nodes::groups::NodeGroup;

pub mod color;
pub mod groups;
pub mod schema;

pub fn load_groups(path: &Path) -> io::Result<Vec<NodeGroup>> {
    let entries = fs::read_dir(path)?;
    let mut schemas = Vec::new();
    let mut generics: usize = 0;

    for entry in entries.flatten() {
        let path = entry.path();
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }

        let name = entry
            .file_name()
            .into_string()
            .map(|n| n.to_string())
            .unwrap_or_else(|_| {
                generics += 1;
                format!("Generic Group {}", generics)
            });

        schemas.push(NodeGroup::load_group(&path, name)?);
    }

    Ok(schemas)
}
