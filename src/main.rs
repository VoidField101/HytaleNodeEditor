use std::env;

use egui::{Color32, Shape, Stroke};

mod editor;
mod workspace;
mod errors;

use crate::{
    editor::node::{Connection, Connector, Node},
    workspace::{groups::NodeGroup},
};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1500.0, 900.0]),

        ..Default::default()
    };
    eframe::run_native(
        "Open Hytale Nodeeditor",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<HyNodeEditor>::default())
        }),
    )
}

struct HyNodeEditor {
    groups: Vec<NodeGroup>,
    nodes: Vec<Node>,
    connections: Vec<Connection>,
    next_id: usize,
}

impl Default for HyNodeEditor {
    fn default() -> Self {
        let mut schemas = Vec::new();
        let mut path = env::current_dir().unwrap();
        path.push("hytale_workspaces");
        path.push("HytaleGenerator Java");
        // Read the directory
        //schemas = load_groups(&path).expect("Failed to load groups");

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
            next_id: 2,
            groups: schemas,
        }
    }
}

impl eframe::App for HyNodeEditor {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::CentralPanel::default().show(ctx, |ui| {
                let response = ui.allocate_rect(ui.max_rect(), egui::Sense::click_and_drag());
                response.context_menu(|ui| {
                    self.groups.iter().for_each(|group| {
                        let mut button = ui.add(editor::striped_button::StripedButton::new(
                            group.name.clone(),
                            group
                                .color
                                .to_egui_color()
                        ));

                        egui::containers::menu::SubMenu::default().show(ui, &mut button, |ui| {
                            group.nodes.iter().for_each(|node_description| {
                                if ui.button(node_description.title.clone()).clicked() {
                                    // Get the current mouse position to spawn the node there
                                    let spawn_pos = ctx.input(|i| {
                                        i.pointer.interact_pos().unwrap_or(egui::Pos2::ZERO)
                                    });

                                    self.nodes.push(Node {
                                        id: self.next_id,
                                        pos: spawn_pos,
                                        label: node_description.title.clone(),
                                        inputs: node_description
                                            .inputs
                                            .iter()
                                            .map(|conn| Connector {
                                                name: conn.label.clone(),
                                                color: conn.color.to_egui_color(),
                                            })
                                            .collect::<Vec<_>>(),
                                        outputs: node_description
                                            .outputs
                                            .iter()
                                            .map(|conn| Connector {
                                                name: conn.label.clone(),
                                                color: conn.color.to_egui_color(),
                                            })
                                            .collect::<Vec<_>>(),
                                    });
                                    self.next_id += 1;
                                    ui.close();
                                }
                            });
                        });
                    });
                });

                let painter = ui.painter();
                for conn in &self.connections {
                    let n1 = &self.nodes[conn.from_node];
                    let n2 = &self.nodes[conn.to_node];

                    let start = n1.pos + egui::vec2(80.0, 20.0);
                    let end = n2.pos + egui::vec2(0.0, 20.0);

                    let cp1 = start + egui::vec2(50.0, 0.0);
                    let cp2 = end - egui::vec2(50.0, 0.0);
                    painter.add(Shape::CubicBezier(
                        egui::epaint::CubicBezierShape::from_points_stroke(
                            [start, cp1, cp2, end],
                            false,
                            Color32::from_black_alpha(0),
                            Stroke::new(2.0, Color32::WHITE),
                        ),
                    ));
                }

                for node in &mut self.nodes {
                    node.draw(ui);
                }
            });
        });
    }
}
