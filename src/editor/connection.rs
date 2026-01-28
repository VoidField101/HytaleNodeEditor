use egui::Ui;

#[derive(Default)]
pub struct Connection {
    pub from_node: usize,
    pub to_node: usize,
}


impl Connection {
    pub fn draw(&mut self, _ui: &mut Ui) {}
}
