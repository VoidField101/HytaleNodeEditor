pub mod menu;
pub mod node;
pub mod striped_button;

pub mod viewer;
pub mod values;


#[derive(thiserror::Error, Debug)]
pub enum EditorError {
    #[error("Node variant not resolved {0} @ '{1}'")]
    NodeVariantIndexResolve(usize, String),
}

