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
        default: Option<String>
    },
    #[serde(rename_all = "PascalCase")]
    List {
        label: String,
        width: Option<u32>,
        #[serde(alias="Type")]
        array_element_type: String
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
    Bool{
        label: String,
        width: Option<u32>,
        default_value: Option<bool>
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
    #[serde(rename_all = "PascalCase", alias="Integer")]
    Int {
        label: String,
        width: Option<u32>,
        #[serde(deserialize_with="string_quirk_deserializer")]
        default: Option<i64>,
        min: Option<i64>,
        max: Option<i64>
    },
    #[serde(rename_all = "PascalCase")]
    Float {
        label: String,
        width: Option<u32>,
        default: Option<f64>,
        min: Option<f64>,
        max: Option<f64>
    },
    #[serde(rename_all = "PascalCase")]
    Object{
        label: String,
        fields: Vec<ContentObjectFields>
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
