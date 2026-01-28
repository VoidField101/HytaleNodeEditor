use egui::Ui;

use crate::editor::{Action, node::Node};

pub fn draw_node_context(ui: &mut Ui, node: &Node) -> Option<Action> {
    let mut action = Option::None;
    egui::ScrollArea::vertical()
        .max_height(800.0) // Limits the menu height so it doesn't go off-screen
        .show(ui, |ui| {
            if ui.button("Delete").clicked() {
                action = Some(node.id);
            }
        });

    action.map(|desc| Action::RemoveNode(desc))
}
