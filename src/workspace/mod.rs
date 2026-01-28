use std::{
    env, error::Error, fs, io, path::{Path, PathBuf}
};

use crate::workspace::{groups::NodeGroup, workspace::Workspace};

pub mod color;
pub mod groups;
pub mod nodes;
pub mod workspace;

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

pub fn load_workspace(path: &Path) -> io::Result<Workspace> {
    let mut ws_path = path.to_path_buf();
    ws_path.push("_Workspace.json");

    let content = fs::read_to_string(&ws_path)?;

    match serde_json::from_str::<Workspace>(&content) {
        Ok(workspace) => Ok(workspace),
        Err(err) => {
            eprintln!("Failed to parse JSON schema at: {:?}", err);
            Err(io::Error::new(io::ErrorKind::Other, err))
        }
    }
}

#[test]
pub fn loading_groups() {
    let mut schemas = Vec::new();
    let mut path = env::current_dir().unwrap();
    path.push("hytale_workspaces");

    let entries = fs::read_dir(path).expect("Failed to load groups");
    for entry in entries.flatten() {
        let sub_path= entry.path();
        // Read the directory
        schemas = load_groups(&sub_path).expect("Failed to load groups");
        println!("{:?}", schemas)
    }
    
}

#[test]
pub fn loading_workspaces() {
    
    let mut path = env::current_dir().unwrap();
    path.push("hytale_workspaces");

    let entries = fs::read_dir(path).expect("Failed to load groups");
    for entry in entries.flatten() {
        let sub_path= entry.path();
        // Read the directory
        let mut schemas ;
        schemas = load_workspace(&sub_path).expect("Failed to load groups");
        println!("{:?}", schemas)
    }

}
