use egui::{Color32, Pos2, Shape, Ui, vec2};

use crate::editor::{Action, menu::draw_node_context};

#[derive(Default, Clone)]
pub struct Connector {
    pub name: String,
    pub color: Color32,
    pub pos: Pos2,
    pub port_index: usize,
    pub is_input: bool,
}

#[derive(Default, Clone)]
pub struct Node {
    pub id: usize,
    pub pos: Pos2,
    pub label: String,
    pub inputs: Vec<Connector>,
    pub outputs: Vec<Connector>,
}

impl Connector {
    pub fn draw(&mut self, ui: &mut Ui, center: Pos2, _input: bool) {
        const SIZE: f32 = 7.0;
        let painter = ui.painter();
        let color = self.color;

        self.pos = center;

        painter.add(Shape::circle_filled(center, SIZE, color));
    }
}

impl Node {
    pub fn draw(&mut self, ui: &mut Ui, selected: bool) -> Option<Action> {
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
                        ui.vertical(|ui| {
                            self.inputs.iter_mut().for_each(|connector| {
                                let (rect, _) = ui.allocate_at_least(
                                    egui::vec2(10.0, 20.0),
                                    egui::Sense::hover(),
                                );
                                connector.draw(ui, rect.center() - vec2(10.0, 0.0), true)
                            });
                        });

                        ui.label(&self.label);
                        // Draw the port shape

                        ui.vertical(|ui| {
                            self.outputs.iter_mut().for_each(|connector| {
                                let (rect, _) = ui.allocate_at_least(
                                    egui::vec2(10.0, 20.0),
                                    egui::Sense::hover(),
                                );
                                connector.draw(ui, rect.center() + vec2(10.0, 0.0), false)
                            });
                        });
                    });
                });
            });

        let drag_resp = res.response;
        if drag_resp.dragged() {
            self.pos += drag_resp.drag_delta();
            action = Some(Action::SelectNode(self.id));
        } else if drag_resp.clicked() {
            action = Some(Action::SelectNode(self.id));
        }
        drag_resp.context_menu(|ui| {
            action = draw_node_context(ui, &self);
        });

        action
    }
}
