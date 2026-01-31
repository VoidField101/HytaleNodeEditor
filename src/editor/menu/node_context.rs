use egui::Ui;

use crate::editor::{menu::MenuAction, node::HyNode};

pub fn draw_node_context<'a>(ui: &mut Ui, node: &HyNode) -> Option<MenuAction<'a>> {
    let mut action = Option::None;
    egui::ScrollArea::vertical()
        .max_height(800.0) // Limits the menu height so it doesn't go off-screen
        .show(ui, |ui| {
            if ui.button("Delete").clicked() {
                action = Some(MenuAction::RemoveNode);
            }
        });

    action
}
