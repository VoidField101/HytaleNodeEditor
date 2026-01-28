use std::collections::HashMap;

use crate::workspace::{color::ColorValue, nodes::NodeDescription, schemas::WorkspaceSchema};

#[derive(Debug, Clone, Default)]
pub struct NodeGroup {
    pub color: ColorValue,
    pub name: String,
    pub nodes: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct Workspace {
    pub workspace: WorkspaceSchema,
    pub groups: Vec<NodeGroup>,
    pub nodes: Vec<NodeDescription>,
}

impl Workspace {
    pub fn construct(schema: WorkspaceSchema, nodes: Vec<NodeDescription>) -> Workspace {
        let mut node_map = nodes
            .iter()
            .enumerate()
            .map(|node| (node.1.id.clone(), (node.0, node.1)))
            .collect::<HashMap<_, _>>();
        let mut groups = Vec::with_capacity(schema.node_categories.len() + 1);

        schema.node_categories.iter().for_each(|category| {
            let nodes = category
                .1
                .iter()
                .filter_map(|node_id| node_map.remove(node_id))
                .collect::<Vec<_>>();

            let mut color = Default::default();
            if let Some(color2) = nodes.first().map(|desc| desc.1.color) {
                if nodes
                    .iter()
                    .find(|description| description.1.color != color2)
                    .is_none()
                {
                    color = color2;
                }
            }

            groups.push(NodeGroup {
                color: color,
                name: category.0.to_owned(),
                nodes: nodes.iter().map(|desc| desc.0).collect(),
            });
        });

        if !node_map.is_empty() {
            let nodes = node_map.drain().map(|(k, v)| v.0).collect();
            groups.push(NodeGroup {
                color: Default::default(),
                name: "Uncategorized".to_owned(),
                nodes,
            });
        }

        Workspace {
            workspace: schema,
            groups: groups,
            nodes: nodes,
        }
    }
}
