use egui::{Color32, Ui};

use crate::editor::node::Node;

#[derive(Default)]
pub struct Connection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

#[derive(Default)]
pub struct ConnectionPartial {
    pub from_node: usize,
    pub from_connector: usize,
    pub reverse: bool,
}

impl Connection {
    pub fn draw(&mut self, ui: &mut Ui, nodes: &[Node]) {
        let n1 = nodes.get(self.from_node).expect("");
        let n2 = nodes.get(self.to_node).expect("");

        let c1 = n1.outputs.get(self.from_connector).expect("");
        let c2 = n2.inputs.get(self.to_connector).expect("");

        let control_offset = (c1.pos.x - c2.pos.x).abs() / 2.0;
        let cp1 = c1.pos + egui::vec2(control_offset, 0.0);
        let cp2 = c2.pos - egui::vec2(control_offset, 0.0);

        let painter = ui.painter();
        painter.add(egui::Shape::CubicBezier(
            egui::epaint::CubicBezierShape::from_points_stroke(
                [c1.pos, cp1, cp2, c2.pos],
                false,
                Color32::TRANSPARENT,
                egui::Stroke::new(2.0, c1.color),
            ),
        ));
    }
}

impl ConnectionPartial {
    pub fn draw(&mut self, ui: &mut Ui, nodes: &[Node]) {
        if let Some(cursor) = ui.ctx().input(|i| i.pointer.interact_pos()) {
            let pos1;
            let pos2;
            let c;
            let n1 = nodes.get(self.from_node).expect("");
            if self.reverse {
                c = n1.inputs.get(self.from_connector).expect("");
                pos1 = cursor;
                pos2 = c.pos;
            } else {
                c = n1.outputs.get(self.from_connector).expect("");
                pos1 = c.pos;
                pos2 = cursor;
            }

            let control_offset = (pos1.x - pos2.x).abs() / 2.0;
            let cp1 = pos1 + egui::vec2(control_offset, 0.0);
            let cp2 = pos2 - egui::vec2(control_offset, 0.0);

            let painter = ui.painter();
            painter.add(egui::Shape::CubicBezier(
                egui::epaint::CubicBezierShape::from_points_stroke(
                    [pos1, cp1, cp2, pos2],
                    false,
                    Color32::TRANSPARENT,
                    egui::Stroke::new(2.0, c.color),
                ),
            ));
        }
    }
}
