use egui::{Color32, Pos2, Shape, Stroke, Ui};

#[derive(Default)]
pub struct Connection {
    pub from_node: usize,
    pub to_node: usize,
}

#[derive(Default)]
pub struct Connector {
    pub name: String,
    pub color: Color32,
}

#[derive(Default)]
pub struct Node {
    pub id: usize,
    pub pos: Pos2,
    pub label: String,
    pub inputs: Vec<Connector>,
    pub outputs: Vec<Connector>,
}

impl Connector {
    pub fn draw(&self, ui: &mut Ui, center: Pos2, _input: bool) {
        const SIZE: f32 = 5.0;
        let painter = ui.painter();
        let color = self.color;

        let points = vec![
            center + egui::vec2(-SIZE, -SIZE),
            center + egui::vec2(-SIZE, SIZE),
            center + egui::vec2(SIZE, 0.0),
        ];
        painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
    }
}

impl Connection {
    pub fn draw(&mut self, _ui: &mut Ui) {}
}

impl Node {
    pub fn draw(&mut self, ui: &mut Ui) {
        egui::Area::new(egui::Id::new(self.id))
            .fixed_pos(self.pos)
            .show(ui.ctx(), |ui| {
                egui::Frame::window(ui.style()).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.vertical(|ui| {
                            self.inputs.iter().for_each(|connector| {
                                let (rect, _) = ui.allocate_at_least(
                                    egui::vec2(20.0, 20.0),
                                    egui::Sense::hover(),
                                );
                                connector.draw(ui, rect.center(), true)
                            });
                        });

                        ui.label(&self.label);
                        // Draw the port shape

                        ui.vertical(|ui| {
                            self.outputs.iter().for_each(|connector| {
                                let (rect, _) = ui.allocate_at_least(
                                    egui::vec2(20.0, 20.0),
                                    egui::Sense::hover(),
                                );
                                connector.draw(ui, rect.center(), false)
                            });
                        });
                    });

                    let drag_resp = ui.allocate_rect(ui.min_rect(), egui::Sense::drag());
                    if drag_resp.dragged() {
                        self.pos += drag_resp.drag_delta();
                    }
                });
            });
    }
}
