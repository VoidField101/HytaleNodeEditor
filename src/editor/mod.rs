pub mod menu;
pub mod node;
pub mod striped_button;

pub mod value;
pub mod viewer;

#[derive(thiserror::Error, Debug)]
pub enum EditorError {
    #[error("Node variant not resolved {0} @ '{1}'")]
    NodeVariantIndexResolve(usize, String),
    #[error("Numeric value from JSON can't be converted {0}")]
    NumericValueNotParsable(serde_json::Number),
    #[error("Unexpected datatype {0} .. Expected: {1}")]
    UnexpectedDatatype(String, String),
}

#[cfg(test)]
mod test {
    use std::{env, fs};

    use crate::{
        generator::nodes_v1,
        workspace::{load_descriptions, load_workspace, workspace::Workspace},
    };

    #[test]
    fn test_basic_to_editor() {
        let mut path_workspace = env::current_dir().unwrap();
        path_workspace.push("hytale_workspaces");
        path_workspace.push("HytaleGenerator Java");
        // Read the directory
        let schema = load_workspace(&path_workspace).expect("Failed to load workspace");
        let descirption = load_descriptions(&path_workspace).expect("Failed to load descriptions");
        let workspace = Workspace::construct(schema, descirption);

        let mut path = env::current_dir().unwrap();
        path.push("hytale_assets");
        path.push("HytaleGenerator");
        path.push("Biomes");
        path.push("Basic.json");

        let content = fs::read_to_string(path).expect("Could not read file");
        let node = serde_json::from_str::<nodes_v1::RootNode>(&content).unwrap();
        let norm = node.normalize(&workspace, "Biome").expect("Faile");

        let (_, nodes) = norm.0.to_editor(&workspace);

        println!("{:?}", nodes);
    }
}
