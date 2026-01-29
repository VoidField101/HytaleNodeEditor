use std::{env, fs, usize};

mod editor;
mod errors;
mod generator;
mod workspace;

use egui::{CornerRadius, vec2};

use crate::{
    editor::{
        Action,
        connection::{Connection, ConnectionPartial},
        node::{Connector, Node},
    },
    generator::biome,
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
    pan_offset: egui::Vec2,
    workspace: Workspace,
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    next_id: usize,
    selected_node: Option<usize>,
    partial: Option<ConnectionPartial>,
}

impl Default for HyNodeEditor {
    // FIXME: This really shouldn't do any file operations.
    fn default() -> Self {
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
        let node = serde_json::from_str::<biome::RootNode>(&content).unwrap();
        let norm = node.normalize(&workspace, "Biome").expect("Faile");

        let (conn, nodes) = norm.0.to_editor(&workspace);

        Self {
            nodes: nodes,
            connections: conn,
            selected_node: None,
            next_id: 2,
            workspace: workspace,
            pan_offset: vec2(0.0, 0.0),
            partial: None,
        }
    }
}

impl eframe::App for HyNodeEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.style_mut(|style| {
            style.visuals.menu_corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.active.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.inactive.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.noninteractive.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.open.corner_radius = CornerRadius::ZERO;
            style.visuals.widgets.hovered.corner_radius = CornerRadius::ZERO;
        });

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

                for conn in &mut self.connections {
                    conn.draw(ui, &self.nodes);
                }

                if let Some(partial) = &mut self.partial {
                    partial.draw(ui, &self.nodes);
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
                            if let Some(partial) = &self.partial
                                && partial.from_node == index.0
                            {
                                self.partial = None
                            }

                            self.connections
                                .retain(|con| con.from_node != index.0 && con.to_node != index.0);

                            self.nodes.remove(index.0);
                        }
                    }
                    Some(Action::SelectNode(node_index)) => {
                        self.selected_node = Some(node_index);
                    }
                    Some(Action::EmptyClick) => {
                        self.selected_node = None;
                        self.partial = None;
                    }
                    None => {}
                }
            });
        });

        if self.partial.is_some() {
            ctx.request_repaint_after_secs(1.0 / 60.0);
        }
    }
}
