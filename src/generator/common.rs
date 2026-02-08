use std::collections::HashMap;

use egui::pos2;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::editor::{self, value::NodeEditorValueTypes};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
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
pub struct NodeId(pub String);



impl Default for Position {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl NodeId {
    pub fn try_parse(&self) -> Option<(&str, Uuid)> {
        self.0.chars().position(|c| c == '-').and_then(|pos| {
            let (name, uid_str) = self.0.split_at(pos);
            Uuid::try_parse(uid_str).map(|uuid| (name, uuid)).ok()
        })
    }

    pub fn from_parts(name: &str, uuid: &Uuid) -> Self {
        Self(format!("{}-{}", name, uuid.as_hyphenated().to_string()))
    }

     pub fn new_rand(name: &str) -> Self {
        Self::from_parts(name, &Uuid::new_v4())
    }
}

