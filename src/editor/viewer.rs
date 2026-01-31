use std::usize;

use egui::{RichText, Ui};
use egui_snarl::{
    InPin, NodeId, OutPin, Snarl,
    ui::{PinInfo, SnarlViewer},
};

use crate::{
    editor::{
        menu::MenuAction,
        node::{HyNode, HyNodePin},
    },
    workspace::workspace::Workspace,
};

pub struct HyNodeViewer<'a> {
    pub workspace: &'a Workspace,
}

impl<'a> SnarlViewer<HyNode> for HyNodeViewer<'a> {
    fn title(&mut self, node: &HyNode) -> String {
        node.label.to_owned()
    }

    fn inputs(&mut self, node: &HyNode) -> usize {
        node.inputs.len()
    }

    fn show_node_menu(
        &mut self,
        node: NodeId,
        inputs: &[InPin],
        outputs: &[OutPin],
        ui: &mut Ui,
        snarl: &mut Snarl<HyNode>,
    ) {
        if let Some(node_ref) = snarl.get_node(node) {
            match super::menu::draw_node_context(ui, node_ref) {
                Some(MenuAction::RemoveNode) => {
                    snarl.remove_node(node);
                }
                _ => {}
            }
        }
    }

    fn has_node_menu(&mut self, node: &HyNode) -> bool {
        true
    }

    fn has_graph_menu(&mut self, pos: egui::Pos2, snarl: &mut Snarl<HyNode>) -> bool {
        true
    }

    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, snarl: &mut Snarl<HyNode>) {
        match super::menu::draw_default_context(ui, &self.workspace.groups, &self.workspace.nodes) {
            Some(MenuAction::AddNode(descriptor)) => {
                snarl.insert_node(
                    pos,
                    HyNode {
                        label: descriptor.title.clone(),
                        inputs: descriptor
                            .inputs
                            .iter()
                            .map(|conn| conn.clone().into())
                            .collect::<Vec<_>>(),
                        outputs: descriptor
                            .outputs
                            .iter()
                            .map(|conn| conn.clone().into())
                            .collect::<Vec<_>>(),
                    },
                );
            }
            _ => {}
        }
    }

    #[allow(refining_impl_trait)]
    fn show_input(
        &mut self,
        pin: &egui_snarl::InPin,
        ui: &mut egui::Ui,
        snarl: &mut egui_snarl::Snarl<HyNode>,
    ) -> PinInfo {
        let pin = &snarl[pin.id.node].inputs[pin.id.input];
        ui.label(&pin.name);
        return PinInfo::circle().with_fill(pin.color);
    }

    fn outputs(&mut self, node: &HyNode) -> usize {
        node.outputs.len()
    }

    #[allow(refining_impl_trait)]
    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        snarl: &mut egui_snarl::Snarl<HyNode>,
    ) -> PinInfo {
        let pin = &snarl[pin.id.node].outputs[pin.id.output];
        if !pin.allow_multiple {
            ui.label(&pin.name);
        }
        else {
            ui.label(RichText::new(&pin.name).italics());
        }

        return PinInfo::circle().with_fill(pin.color);
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<HyNode>) {
        let from_pin = &snarl[from.id.node].outputs[from.id.output];
        let to_pin = &snarl[to.id.node].inputs[to.id.input];

        if (to.remotes.is_empty() || to_pin.allow_multiple)
            && (from.remotes.is_empty() || from_pin.allow_multiple)
        {
            snarl.connect(from.id, to.id);
        }
    }
}
