use egui::{Color32, Pos2, Ui};

use crate::editor::{menu::MenuAction, menu::draw_node_context};

#[derive(Default, Clone)]
pub struct HyNodePin {
    pub name: String,
    pub color: Color32,
    pub allow_multiple: bool,
}

#[derive(Default, Clone)]
pub struct HyNode {
    pub id: usize,
    pub pos: Pos2,
    pub label: String,
    pub inputs: Vec<HyNodePin>,
    pub outputs: Vec<HyNodePin>,
}

#[derive(Default)]
pub struct HyConnection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

impl HyNode {
    pub fn draw(&mut self, ui: &mut Ui, selected: bool) -> Option<MenuAction<'_>> {
        let mut action = None;
        let res = egui::Area::new(egui::Id::new(self.id))
            .fixed_pos(self.pos)
            .sense(egui::Sense::click_and_drag())
            .show(ui.ctx(), |ui| {
                let mut frame = egui::Frame::window(ui.style());
                if selected {
                    frame = frame
                        .stroke(ui.style().visuals.selection.stroke)
                        .fill(ui.style().visuals.selection.bg_fill);
                }

                frame.show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.style_mut().interaction.selectable_labels = false;

                        ui.label(&self.label);
                        // Draw the port shape
                    });
                });
            });

        let drag_resp = res.response;
        if drag_resp.dragged() {
            self.pos += drag_resp.drag_delta();
            // action = Some(Action::SelectNode(self.id));
        } else if drag_resp.clicked() {
            // action = Some(Action::SelectNode(self.id));
        }
        drag_resp.context_menu(|ui| {
            action = draw_node_context(ui, &self);
        });

        action
    }
}
