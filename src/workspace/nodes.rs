use std::{collections::HashMap, fs, io, path::Path};

use serde::{self, Deserialize, Serialize};

use serde_aux::prelude::*;

use crate::workspace::{color::ColorValue, content::Content, workspace::Workspace};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub id: String,
    pub title: String,
    pub color: ColorValue,

    #[serde(default)]
    pub content: Vec<Content>,
    #[serde(default)]
    pub outputs: Vec<Connector>,
    #[serde(default)]
    pub inputs: Vec<Connector>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    pub schema: HashMap<String, SchemaObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum SchemaObject {
    ConstString(String),
    #[serde(deserialize_with = "deserialize_struct_case_insensitive")]
    Pin(Pin),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all(serialize = "PascalCase"))]
pub struct Pin {
    pub node: String,
    pub pin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Connector {
    pub id: String,
    #[serde(rename = "Type")]
    pub typ: String,
    pub color: ColorValue,

    #[serde(default)]
    pub label: String,
    #[serde(default = "default_connector_multiple")]
    pub multiple: bool,
}

#[allow(unused)]
fn default_connector_multiple() -> bool {
    true
}

impl NodeDescription {
    pub fn load_from_file(path: &Path) -> anyhow::Result<Self> {
        let content = fs::read_to_string(&path)?;
        let start = content
            .find(|c| c == '{')
            .ok_or(anyhow::Error::msg("No start of JSON found"))?;
        Ok(serde_json::from_str::<NodeDescription>(&content[start..])?)
    }

    pub fn get_connector<'a>(&'a self, key: &str) -> Option<(usize, &'a Connector)> {
        if let Some(SchemaObject::Pin(pin)) = self.schema.get(key) {
            self.outputs
                .iter()
                .enumerate()
                .find(|out| out.1.id == pin.pin)
        } else {
            None
        }
    }

    pub fn get_pin<'a>(&'a self, key: &str) -> Option<&'a Pin> {
        if let Some(SchemaObject::Pin(pin)) = self.schema.get(key) {
            Some(pin)
        } else {
            None
        }
    }

    /// Finds the NodeDescription (aka. Variant) matching a given pin (based on the schema key)
    /// As the Variant will require a variant descriptor the resolver needs to look up that key if required
    /// If the Bariant does not exist in the workspace it's assumed to be the actual node id.
    pub fn get_variant<'a, 'c, R>(
        &self,
        workspace: &'a Workspace,
        key: &str,
        resolver: R,
    ) -> Option<&'a NodeDescription>
    where
        R: FnOnce(&str) -> Option<&'c str>,
    {
        self.get_pin(key).and_then(|pin| {
            if let Some(variant) = workspace.workspace.variants.get(&pin.node) {
                resolver(&variant.variant_field_name)
                    .and_then(|res| variant.variants.get(res))
                    .and_then(|vaiant_name| {
                        workspace.nodes.iter().find(|desc| desc.id.eq(vaiant_name))
                    })
            } else {
                workspace.nodes.iter().find(|desc| desc.id == pin.node)
            }
        })
    }
}
