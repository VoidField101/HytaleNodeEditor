use std::collections::HashMap;

use egui::{pos2, vec2};

use crate::{
    editor::{self, node::HyConnection, value::NodeEditorValueTypes},
    generator::{
        JsonValue,
        common::{NodeId, Position},
    },
    workspace::workspace::Workspace,
};

#[derive(Debug, Clone)]
pub struct NormalizedNode {
    pub position: Position,
    pub comment: Option<String>,
    pub node_id: Option<NodeId>,
    pub variant: String,
    pub values: HashMap<String, JsonValue>,
    pub outputs: HashMap<String, Vec<NormalizedNode>>,
}

impl NormalizedNode {
    pub fn to_editor<'a>(
        &self,
        workspace: &'a Workspace,
    ) -> (Vec<HyConnection>, Vec<editor::node::HyNodeProto<'a>>) {
        let mut connections = Vec::new();
        let mut nodes = Vec::new();

        let node_map = workspace
            .nodes
            .iter()
            .enumerate()
            .map(|node| (node.1.id.clone(), node.0))
            .collect::<HashMap<_, _>>();

        self.to_editor_internal(&node_map, workspace, &mut connections, &mut nodes);

        let mut x_offset = f32::INFINITY;
        let mut y_offset = f32::INFINITY;
        for pos in nodes.iter().map(|node| node.pos) {
            x_offset = x_offset.min(pos.x);
            y_offset = y_offset.min(pos.y);
        }

        nodes
            .iter_mut()
            .for_each(|node| node.pos = node.pos - vec2(x_offset, y_offset));

        (connections, nodes)
    }

    fn to_editor_internal<'a>(
        &self,
        node_map: &HashMap<String, usize>,
        workspace: &'a Workspace,
        connections: &mut Vec<HyConnection>,
        nodes: &mut Vec<editor::node::HyNodeProto<'a>>,
    ) -> usize {
        // FIXME: Replace unwrap with propper error handling!
        let desc_index = node_map.get(&self.variant).unwrap();
        let desc = &workspace.nodes[*desc_index];
        let new_id = nodes.len();

        let values = desc
            .content
            .iter()
            .map(|content| {
                (
                    content.id.clone(),
                    NodeEditorValueTypes::from_value(
                        self.values
                            .get(&content.id)
                            .map(Clone::clone)
                            .unwrap_or(content.options.get_default().0),
                        &content.options,
                    )
                    .unwrap(),
                )
            })
            .collect();

        nodes.push(editor::node::HyNodeProto {
            pos: pos2(self.position.x as f32, self.position.y as f32),
            variant_index: *desc_index,
            workspace,
            values: values,
        });

        self.outputs
            .iter()
            .enumerate()
            .for_each(|(index, (connector_name, new_nodes))| {
                // FIXME: Replace unwrap with propper error handling!
                let conn_index = desc.get_connector(connector_name).unwrap().0;

                new_nodes.iter().for_each(|node| {
                    let sub_id = node.to_editor_internal(node_map, workspace, connections, nodes);

                    connections.push(HyConnection {
                        from_node: new_id,
                        from_connector: conn_index,
                        to_node: sub_id,
                        to_connector: 0, //TODO: Figure out how multi-input nodes are handled
                    });
                });
            });

        new_id
    }
}
