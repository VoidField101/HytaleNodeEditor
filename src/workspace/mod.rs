use crate::workspace::{nodes::NodeDescription, schemas::WorkspaceSchema};
use std::{
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

pub mod color;
pub mod nodes;
pub mod schemas;
pub mod workspace;
pub mod content;

#[derive(thiserror::Error, Debug)]
pub enum WorkspacePaserError {
    #[error("Failed to read the file {0}: {1}")]
    ReadError(PathBuf, anyhow::Error),
}

pub fn load_descriptions(path: &Path) -> anyhow::Result<Vec<NodeDescription>> {
    let entries = fs::read_dir(path)?;
    let mut schemas = Vec::new();
    for entry in entries.flatten() {
        load_descriptions_recurse(&entry, &mut schemas)?;
    }

    Ok(schemas)
}

fn load_descriptions_recurse(
    entry: &DirEntry,
    schemas: &mut Vec<NodeDescription>,
) -> anyhow::Result<()> {
    let path = entry.path();
    if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
        let entries = fs::read_dir(&path)?;
        for entry in entries.flatten() {
            load_descriptions_recurse(&entry, schemas)?;
        }
    } else if !entry.file_name().eq_ignore_ascii_case("_Workspace.json") {
        schemas.push(
            NodeDescription::load_from_file(&path)
                .map_err(|err| WorkspacePaserError::ReadError(path, err))?,
        );
    }

    Ok(())
}


pub fn load_workspace(path: &Path) -> io::Result<WorkspaceSchema> {
    let mut ws_path = path.to_path_buf();
    ws_path.push("_Workspace.json");

    let content = fs::read_to_string(&ws_path)?;

    match serde_json::from_str::<WorkspaceSchema>(&content) {
        Ok(workspace) => Ok(workspace),
        Err(err) => {
            eprintln!("Failed to parse JSON schema at: {:?}", err);
            Err(io::Error::new(io::ErrorKind::Other, err))
        }
    }
}

#[cfg(test)]
mod tests {
    use egui::ahash::HashSet;

    use crate::workspace::{load_descriptions, load_workspace, workspace::Workspace};
    use std::{env, fs};

    #[test]
    pub fn loading_descriptions() -> anyhow::Result<()> {
        let mut schemas = Vec::new();
        let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");
        path.push("ScriptableBrushes");

        //let entries = fs::read_dir(path).expect("Failed to load descriptions");
        // for entry in entries.flatten() {

        //schemas = load_descriptions(&path)?;
        //println!("{:?}", schemas);

                let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");
        path.push("HytaleGenerator Java");

        //let entries = fs::read_dir(path).expect("Failed to load descriptions");
        // for entry in entries.flatten() {

        schemas.extend(load_descriptions(&path)?);
        println!("{:?}", schemas);

         let types = schemas.iter().map(|schema| &schema.content).flatten().map(|c| c.options.clone()).collect::<Vec<_>>();
        
        //{"SmallString", "Enum", "Integer", "List", "IntSlider", "Bool", "String", "Checkbox", "Int", "Float", "Object"}
        println!("{:?}", types);

        Ok(())
        //}
    }

    #[test]
    pub fn loading_workspaces() {
        let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");

        let entries = fs::read_dir(path).expect("Failed to load workspaces");
        for entry in entries.flatten() {
            let sub_path = entry.path();
            // Read the directory
            let mut workspace = load_workspace(&sub_path).expect("Failed to load workspaces");
            println!("{:?}", workspace)
        }
    }

    #[test]
    pub fn generator_workspace_test() {
        let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");
        path.push("HytaleGenerator Java");

        let schema = load_workspace(&path).expect("Failed to load workspace");
        let descirption = load_descriptions(&path).expect("Failed to load descriptions");
        let workspace = Workspace::construct(schema, descirption);

        workspace.groups.iter().for_each(|group| {
            println!(
                "{} -> {:?} -> {}",
                group.name,
                group.color,
                group.nodes.len()
            )
        });
    }
}
