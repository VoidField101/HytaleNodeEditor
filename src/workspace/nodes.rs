use std::{collections::HashMap, fs, io, path::Path};

use serde::{self, Deserialize, Serialize};

use crate::workspace::{color::ColorValue, workspace::Workspace};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NodeDescription {
    pub id: String,
    pub title: String,
    pub color: ColorValue,

    pub content: Vec<Content>,
    pub outputs: Vec<Connector>,
    pub inputs: Vec<Connector>,
    pub schema: HashMap<String, SchemaObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", untagged)]
pub enum SchemaObject {
    ConstString(String),
    Pin(Pin),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Pin {
    pub node: String,
    pub pin: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    pub id: String,
    #[serde(rename = "Type")]
    pub typ: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", deny_unknown_fields)]
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
    pub fn load_from_file(path: &Path) -> io::Result<Self> {
        let content = fs::read_to_string(&path)?;
        Ok(serde_json::from_str::<NodeDescription>(&content)?)
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
                        workspace.nodes.iter().find(|desc| desc.id == *vaiant_name)
                    })
            } else {
                workspace.nodes.iter().find(|desc| desc.id == pin.node)
            }
        })
    }
}
