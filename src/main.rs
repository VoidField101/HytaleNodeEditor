use std::{env, usize};

mod editor;
mod errors;
mod workspace;

use crate::{
    editor::{
        Action,
        connection::Connection,
        node::{Connector, Node},
    },
    workspace::{load_descriptions, load_workspace, workspace::Workspace},
};

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),

        ..Default::default()
    };
    eframe::run_native(
        "Open Hytale Nodeeditor",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<HyNodeEditor>::default())
        }),
    )
}

struct HyNodeEditor {
    workspace: Workspace,
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    next_id: usize,
    selected_node: Option<usize>,
}

impl Default for HyNodeEditor {
    // FIXME: This really shouldn't do any file operations.
    fn default() -> Self {
        let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");
        path.push("HytaleGenerator Java");
        // Read the directory
        let schema = load_workspace(&path).expect("Failed to load workspace");
        let descirption = load_descriptions(&path).expect("Failed to load descriptions");
        let workspace = Workspace::construct(schema, descirption);

        Self {
            nodes: vec![
                Node {
                    id: 0,
                    pos: egui::pos2(100.0, 100.0),
                    label: "Source".into(),
                    ..Default::default()
                },
                Node {
                    id: 1,
                    pos: egui::pos2(300.0, 150.0),
                    label: "Target".into(),
                    ..Default::default()
                },
            ],
            connections: vec![Connection {
                from_node: 0,
                to_node: 1,
            }],
            selected_node: None,
            next_id: 2,
            workspace: workspace,
        }
    }
}

impl eframe::App for HyNodeEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let mut action = editor::menu::draw_default_context(
                    ui,
                    &self.workspace.groups,
                    &self.workspace.nodes,
                );

                for node in &mut self.nodes {
                    action = action
                        .or(node.draw(ui, node.id == self.selected_node.unwrap_or(usize::MAX)));
                }

                let spawn_pos = ctx.input(|i| i.pointer.interact_pos().unwrap_or(egui::Pos2::ZERO));

                match action {
                    Some(Action::AddNode(node_index)) => {
                        let node = &self.workspace.nodes[node_index];
                        self.nodes.push(Node {
                            id: self.next_id,
                            pos: spawn_pos,
                            label: node.title.clone(),
                            inputs: node
                                .inputs
                                .iter()
                                .map(|conn| Connector {
                                    name: conn.label.clone(),
                                    color: conn.color.to_egui_color(),
                                    ..Default::default()
                                })
                                .collect::<Vec<_>>(),
                            outputs: node
                                .outputs
                                .iter()
                                .map(|conn| Connector {
                                    name: conn.label.clone(),
                                    color: conn.color.to_egui_color(),
                                    ..Default::default()
                                })
                                .collect::<Vec<_>>(),
                        });
                        self.selected_node = Some(self.next_id);
                        self.next_id += 1;
                    }
                    Some(Action::RemoveNode(node_index)) => {
                        if let Some(index) = self
                            .nodes
                            .iter()
                            .enumerate()
                            .find(|node| node.1.id == node_index)
                        {
                            self.nodes.remove(index.0);
                        }
                    }
                    Some(Action::SelectNode(node_index)) => {
                        self.selected_node = Some(node_index);
                    }
                    Some(Action::EmptyClick) => {
                        self.selected_node = None;
                    }
                    None => {}
                }
            });
        });
    }
}
