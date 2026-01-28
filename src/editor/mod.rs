pub mod node;
pub mod striped_button;
pub mod connection;
pub mod menu;


pub enum Action {
    AddNode(usize),
    RemoveNode(usize),
    SelectNode(usize),
    EmptyClick
}