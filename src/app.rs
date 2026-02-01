use std::{env, fs};

use eframe::CreationContext;
use egui::{CornerRadius, Frame, Id, Margin};
use egui_snarl::{
    InPinId, NodeId, OutPinId, Snarl,
    ui::{NodeLayout, PinPlacement, SnarlStyle, SnarlWidget},
};
use ouroboros::self_referencing;

use crate::{
    editor::{node::HyNode, viewer::HyNodeViewer},
    generator::nodes_v1,
    workspace::{load_descriptions, load_workspace, workspace::Workspace},
};

#[self_referencing]
pub struct HyNodeEditor {
    workspace: Workspace,
    #[borrows(workspace)]
    #[covariant]
    snarl: Snarl<HyNode<'this>>,
}

impl HyNodeEditor {
    pub fn build(cc: &CreationContext) -> Box<Self> {
        egui_extras::install_image_loaders(&cc.egui_ctx);

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

        Box::new(HyNodeEditor::new(workspace, |workspace: &Workspace| {
            let mut snarl = Snarl::new();

            let (conn, nodes) = norm.0.to_editor(&workspace);

            for node in nodes.into_iter() {
                snarl.insert_node(
                    node.pos,
                    node.try_into()
                        .expect("A error finializeing the node prototypes occured"),
                );
            }

            for connection in conn.iter() {
                snarl.connect(
                    OutPinId {
                        node: NodeId(connection.from_node),
                        output: connection.from_connector,
                    },
                    InPinId {
                        node: NodeId(connection.to_node),
                        input: connection.to_connector,
                    },
                );
            }

            snarl
        }))
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

        let snarl_style = SnarlStyle {
            node_layout: Some(NodeLayout::coil()),
            wire_width: Some(4.0),
            pin_placement: Some(PinPlacement::Edge),
            ..Default::default()
        };

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {});

        egui::CentralPanel::default()
            .frame(Frame {
                inner_margin: Margin::ZERO,
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.with_mut(|mut_self| {
                    SnarlWidget::new()
                        .id(Id::new("snarl-workspace"))
                        .style(snarl_style)
                        .show(
                            mut_self.snarl,
                            &mut HyNodeViewer {
                                workspace: &mut_self.workspace,
                            },
                            ui,
                        );
                })
            });
    }
}
