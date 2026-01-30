pub mod nodes_v1;
pub mod nodes_v2;
pub mod common;

#[cfg(test)]
mod tests {
    use crate::{
        generator::nodes_v1,
        workspace::{load_descriptions, load_workspace, workspace::Workspace},
    };
    use std::{env, fs};

    #[test]
    pub fn test_basic_biome() {
        let mut path = env::current_dir().unwrap();
        path.push("hytale_assets");
        path.push("HytaleGenerator");
        path.push("Biomes");
        path.push("Volcanic1");
        path.push("Volcanic1_Shore.json");

        let content = fs::read_to_string(path).expect("Could not read file");
        let node = serde_json::from_str::<nodes_v1::RootNode>(&content).unwrap();
        println!("{:?}", node);
    }

    #[test]
    pub fn test_basic_biome_norm() {
        let mut path = env::current_dir().unwrap();
        path.push("hytale_assets");
        path.push("HytaleGenerator");
        path.push("Biomes");
        path.push("Basic.json");

        let mut path_workspace = env::current_dir().unwrap();
        path_workspace.push("hytale_workspaces");
        path_workspace.push("HytaleGenerator Java");

        let schema = load_workspace(&path_workspace).expect("Failed to load workspace");
        let descirption = load_descriptions(&path_workspace).expect("Failed to load descriptions");
        let workspace = Workspace::construct(schema, descirption);

        let content = fs::read_to_string(path).expect("Could not read file");
        let node = serde_json::from_str::<nodes_v1::RootNode>(&content).unwrap();
        let norm = node.normalize(&workspace, "Biome").expect("Faile");
        println!("{:?}", norm);
    }
}
