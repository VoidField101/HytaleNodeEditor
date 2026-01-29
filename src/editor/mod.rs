pub mod connection;
pub mod menu;
pub mod node;
pub mod striped_button;

pub enum Action {
    AddNode(usize),
    RemoveNode(usize),
    SelectNode(usize),
    EmptyClick,
}
