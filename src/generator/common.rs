use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "$x")]
    pub x: i32,
    #[serde(rename = "$y")]
    pub y: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupPosition {
    #[serde(rename = "$x")]
    pub x: f32,
    #[serde(rename = "$y")]
    pub y: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    #[serde(rename = "$Position")]
    pub position: GroupPosition,
    #[serde(rename = "$width")]
    pub width: f32,
    #[serde(rename = "$height")]
    pub height: f32,
    #[serde(rename = "$name")]
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct WorksheetInfo {
    pub title: String,
    pub workspace_id: String,
    pub groups: Vec<Group>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NodeId(String);

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}
