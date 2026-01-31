use serde::{self, Deserialize, Deserializer, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase", tag = "Type", content = "Options")]
pub enum ContentType {
    #[serde(rename_all = "PascalCase")]
    SmallString {
        label: String,
        default: Option<String>,
        width: Option<u32>,
    },
    #[serde(rename_all = "PascalCase")]
    Enum {
        label: String,
        width: Option<u32>,
        values: Vec<String>,
        default: Option<String>,
    },
    #[serde(rename_all = "PascalCase")]
    List {
        label: String,
        width: Option<u32>,
        #[serde(alias = "Type")]
        array_element_type: String,
    },
    #[serde(rename_all = "PascalCase")]
    IntSlider {
        label: String,
        width: Option<u32>,
        default: Option<i64>,
        tick_frequency: i32,
        min: i64,
        max: i64,
    },
    #[serde(rename_all = "PascalCase")]
    Bool {
        label: String,
        width: Option<u32>,
        default_value: Option<bool>,
    },
    #[serde(rename_all = "PascalCase")]
    String {
        label: String,
        width: u32,
        height: u32,
    },
    #[serde(rename_all = "PascalCase")]
    Checkbox {
        label: String,
        width: Option<u32>,
        default: Option<bool>,
    },
    #[serde(rename_all = "PascalCase", alias = "Integer")]
    Int {
        label: String,
        width: Option<u32>,
        #[serde(deserialize_with = "string_quirk_deserializer", default)]
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>,
    },
    #[serde(rename_all = "PascalCase")]
    Float {
        label: String,
        width: Option<u32>,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>,
    },
    #[serde(rename_all = "PascalCase")]
    Object {
        label: String,
        fields: Vec<ContentObjectFields>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ContentObjectFields {
    pub id: String,
    #[serde(flatten)]
    pub options: ContentType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Content {
    pub id: String,
    #[serde(flatten)]
    pub options: ContentType,
}

/// Deserializes to a i64 even if the JSON represents a String
/// This is requires since some workspace files are apparently misconfigured
fn string_quirk_deserializer<'de, D>(data: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v = Option::<Value>::deserialize(data)?;

    match v {
        Some(Value::Number(n)) => Ok(n.as_i64()),
        Some(Value::String(s)) => s.parse::<i64>().map(Some).map_err(serde::de::Error::custom),
        Some(Value::Null) | None => Ok(None),
        _ => Err(serde::de::Error::custom("Expected a number or a string")),
    }
}

impl ContentType {
    pub fn get_default(&self) -> Value {
        let val = match self {
            ContentType::SmallString {
                label: _,
                default,
                width: _,
            } => default.clone().map(Value::from),
            ContentType::Enum {
                label: _,
                width: _,
                values: _,
                default,
            } => default.clone().map(Value::from),
            ContentType::List {
                label: _,
                width: _,
                array_element_type: _,
            } => None,
            ContentType::IntSlider {
                label: _,
                width: _,
                default,
                tick_frequency: _,
                min: _,
                max: _,
            } => default.clone().map(Value::from),
            ContentType::Bool {
                label: _,
                width: _,
                default_value,
            } => default_value.clone().map(Value::from),
            ContentType::String {
                label: _,
                width: _,
                height: _,
            } => None,
            ContentType::Checkbox {
                label: _,
                width: _,
                default,
            } => default.clone().map(Value::from),
            ContentType::Int {
                label: _,
                width: _,
                default,
                min: _,
                max: _,
            } => default.clone().map(Value::from),
            ContentType::Float {
                label: _,
                width: _,
                default,
                min: _,
                max: _,
            } => default.clone().map(Value::from),
            ContentType::Object {
                label: _,
                fields: _,
            } => None,
        };

        val.unwrap_or(Value::Null)
    }

    pub fn get_common(&self) -> (&str, Option<u32>) {
        let (label, width) = match self {
            ContentType::SmallString {
                label,
                default: _,
                width,
            } => (label, *width),
            ContentType::Enum {
                label,
                width,
                values: _,
                default: _,
            } => (label, *width),
            ContentType::List {
                label,
                width,
                array_element_type: _,
            } => (label, *width),
            ContentType::IntSlider {
                label,
                width,
                default: _,
                tick_frequency: _,
                min: _,
                max: _,
            } => (label, *width),
            ContentType::Bool {
                label,
                width,
                default_value: _,
            } => (label, *width),
            ContentType::String {
                label,
                width,
                height: _,
            } => (label, Some(*width)),
            ContentType::Checkbox {
                label,
                width,
                default: _,
            } => (label, *width),
            ContentType::Int {
                label,
                width,
                default: _,
                min: _,
                max: _,
            } => (label, *width),
            ContentType::Float {
                label,
                width,
                default: _,
                min: _,
                max: _,
            } => (label, *width),
            ContentType::Object { label, fields: _ } => (label, None),
        };

        (label, width)
    }
}
