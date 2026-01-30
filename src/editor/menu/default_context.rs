use egui::Ui;

use crate::{
    editor::{self, menu::MenuAction},
    workspace::{nodes::NodeDescription, workspace::NodeGroup},
};

pub fn draw_default_context<'a>(
    ui: &mut Ui,
    groups: &[NodeGroup],
    descriptors: &'a [NodeDescription],
) -> Option<MenuAction<'a>> {
    //let response = ui.allocate_rect(ui.max_rect(), egui::Sense::click_and_drag());
    let mut action = Option::None;
    //response.context_menu(|ui| {
    egui::ScrollArea::vertical()
        .max_height(800.0) // Limits the menu height so it doesn't go off-screen
        .show(ui, |ui| {
            for group in groups.iter() {
                if let Some(node) = draw_group_submenu(ui, group, descriptors) {
                    action = Some(&descriptors[node]);
                }
            }
        });
    //});

    action.map(|desc| MenuAction::AddNode(desc))
}

fn draw_group_submenu(
    ui: &mut Ui,
    group: &NodeGroup,
    descriptors: &[NodeDescription],
) -> Option<usize> {
    let mut button = ui.add(editor::striped_button::StripedButton::new(
        group.name.clone(),
        group.color.to_egui_color(),
    ));
    let mut action = Option::None;

    egui::containers::menu::SubMenu::default().show(ui, &mut button, |ui| {
        egui::ScrollArea::vertical()
            .max_height(400.0) // Limits the menu height so it doesn't go off-screen
            .show(ui, |ui| {
                for node_index in group.nodes.iter() {
                    let descriptor = descriptors
                        .get(*node_index)
                        .expect("Fatal error trying to resolve non existing node_index");

                    let create_button = ui.add(editor::striped_button::StripedButton::new(
                        descriptor.title.clone(),
                        descriptor.color.to_egui_color(),
                    ));

                    if create_button.clicked() {
                        action = Some(*node_index);
                    }
                }
            });
    });

    return action;
}
