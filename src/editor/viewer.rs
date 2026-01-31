use std::usize;

use egui::{RichText, Ui};
use egui_snarl::{
    InPin, NodeId, OutPin, Snarl,
    ui::{PinInfo, SnarlViewer},
};

use crate::{
    editor::{
        menu::MenuAction,
        node::{HyNode},
    },
    workspace::workspace::Workspace,
};

pub struct HyNodeViewer<'a> {
    pub workspace: &'a Workspace,
}

impl<'a, 'b> SnarlViewer<HyNode<'b>> for HyNodeViewer<'a>
    where 'a: 'b
{
    fn title(&mut self, node: &HyNode) -> String {
        node.title.to_owned()
    }

    fn inputs(&mut self, node: &HyNode) -> usize {
        node.description.inputs.len()
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

    fn show_graph_menu(&mut self, pos: egui::Pos2, ui: &mut Ui, snarl: &mut Snarl<HyNode<'b>>) {
        match super::menu::draw_default_context(ui, &self.workspace.groups, &self.workspace.nodes) {
            Some(MenuAction::AddNode(descriptor)) => {
                snarl.insert_node(
                    pos,
                    HyNode::new(descriptor),
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
        let pin = &snarl[pin.id.node].description.inputs[pin.id.input];
        ui.label(&pin.label);
        return PinInfo::circle().with_fill(pin.color.into());
    }

    fn outputs(&mut self, node: &HyNode) -> usize {
        node.description.outputs.len()
    }

    #[allow(refining_impl_trait)]
    fn show_output(
        &mut self,
        pin: &egui_snarl::OutPin,
        ui: &mut egui::Ui,
        snarl: &mut egui_snarl::Snarl<HyNode>,
    ) -> PinInfo {
        let pin = &snarl[pin.id.node].description.outputs[pin.id.output];
        if !pin.multiple {
            ui.label(&pin.label);
        }
        else {
            ui.label(RichText::new(&pin.label).italics());
        }

        return PinInfo::circle().with_fill(pin.color.into());
    }

    fn connect(&mut self, from: &OutPin, to: &InPin, snarl: &mut Snarl<HyNode>) {
        let from_pin = &snarl[from.id.node].description.outputs[from.id.output];
        let to_pin = &snarl[to.id.node].description.inputs[to.id.input];

        if (to.remotes.is_empty() || to_pin.multiple)
            && (from.remotes.is_empty() || from_pin.multiple)
        {
            snarl.connect(from.id, to.id);
        }
    }
}
