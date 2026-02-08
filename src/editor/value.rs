use std::{fmt::Debug, str::FromStr};

use crate::{
    editor::EditorError,
    generator::nodes_v1::NodeValue,
    workspace::content::{ContentType, ValueType},
};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum NodeEditorValueTypes {
    #[default]
    Null,
    Other(serde_json::Value),
    String(String),
    Integer(i64),
    IntegerText(NodeNumericEditing<i64>),
    Float(i64),
    FloatText(NodeNumericEditing<f64>),
    Boolean(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub struct NodeNumericEditing<T>
where
    T: FromStr + ToString + PartialOrd + Debug + Copy + Sized,
{
    current_value: T,
    text_field: String,
    matching: bool,
    min: Option<T>,
    max: Option<T>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValueFilterAction {
    InvalidIgnore,
    InvalidReset,
    Valid,
}

impl NodeEditorValueTypes {
    pub fn from_value(value: serde_json::Value, typ: &ContentType) -> Result<Self, EditorError> {
        let (default, real_type) = typ.get_default();
        Ok(match real_type {
            ValueType::Boolean => NodeEditorValueTypes::Boolean(
                value
                    .as_bool()
                    .or_else(|| default.as_bool())
                    .unwrap_or(!value.is_null()),
            ),
            ValueType::Float => {
                if let ContentType::Float { min, max, .. } = typ {
                    NodeEditorValueTypes::FloatText(NodeNumericEditing::new(
                        value.as_f64().or_else(|| default.as_f64()).unwrap_or(0.0),
                        *min,
                        *max,
                    ))
                } else {
                    unreachable!()
                }
            }
            ValueType::Int => {
                if let ContentType::Int { min, max, .. } = typ {
                    NodeEditorValueTypes::IntegerText(NodeNumericEditing::new(
                        value
                            .as_i64()
                            .or_else(|| value.as_f64().map(|v| v as i64))
                            .or_else(|| default.as_i64())
                            .or_else(|| default.as_f64().map(|v| v as i64))
                            .unwrap_or(0),
                        *min,
                        *max,
                    ))
                } else if let ContentType::IntSlider { .. } = typ {
                    NodeEditorValueTypes::Integer(
                        value
                            .as_i64()
                            .or_else(|| value.as_f64().map(|v| v as i64))
                            .or_else(|| default.as_i64())
                            .or_else(|| default.as_f64().map(|v| v as i64))
                            .unwrap_or(0),
                    )
                } else {
                    unreachable!()
                }
            }
            ValueType::Object => NodeEditorValueTypes::Other(value),
            ValueType::String => NodeEditorValueTypes::String(
                value
                    .as_str()
                    .map(|v| v.to_string())
                    .unwrap_or_else(String::new),
            ),
            ValueType::List => NodeEditorValueTypes::Other(value),
            ValueType::Enum => NodeEditorValueTypes::Other(value),
        })
    }

    // FIXME: Due to issues with NodeValue, this one should be replaces with the raw serde_json Value
    pub fn from_nodevalue(
        value: Option<NodeValue>,
        typ: &ContentType,
    ) -> Result<Self, EditorError> {
        let (default, real_type) = typ.get_default();
        Ok(match real_type {
            ValueType::Boolean => {
                NodeEditorValueTypes::Boolean(if let Some(NodeValue::Bool(v)) = value {
                    v
                } else {
                    default.as_bool().unwrap_or(value.is_none())
                })
            }
            ValueType::Float => {
                if let ContentType::Float { min, max, .. } = typ {
                    NodeEditorValueTypes::FloatText(NodeNumericEditing::new(
                        if let Some(NodeValue::Number(v)) = value {
                            v as f64
                        } else {
                            default.as_f64().unwrap_or(0.0)
                        },
                        *min,
                        *max,
                    ))
                } else {
                    unreachable!()
                }
            }
            ValueType::Int => {
                if let ContentType::Int { min, max, .. } = typ {
                    NodeEditorValueTypes::IntegerText(NodeNumericEditing::new(
                        if let Some(NodeValue::Number(v)) = value {
                            v as i64
                        } else {
                            default.as_i64().unwrap_or(0)
                        },
                        *min,
                        *max,
                    ))
                } else if let ContentType::IntSlider { .. } = typ {
                    NodeEditorValueTypes::Integer(if let Some(NodeValue::Number(v)) = value {
                        v as i64
                    } else {
                        default.as_i64().unwrap_or(0)
                    })
                } else {
                    unreachable!()
                }
            }
            ValueType::Object => NodeEditorValueTypes::Other(serde_json::Value::Null),
            ValueType::String => {
                NodeEditorValueTypes::String(if let Some(NodeValue::String(v)) = value {
                    v
                } else {
                    default
                        .as_str()
                        .map(|s| s.to_string())
                        .unwrap_or_else(String::new)
                })
            }
            ValueType::List => NodeEditorValueTypes::Other(serde_json::Value::Null),
            ValueType::Enum => NodeEditorValueTypes::Other(serde_json::Value::Null),
        })
    }
}

impl<T> NodeNumericEditing<T>
where
    T: FromStr + ToString + PartialOrd + Debug + Copy + Sized,
{
    pub fn new(value: T, min: Option<T>, max: Option<T>) -> Self {
        Self {
            current_value: value,
            text_field: value.to_string(),
            matching: true,
            min,
            max,
        }
    }

    pub fn set_range(&mut self, min: Option<T>, max: Option<T>) {
        self.min = min;
        self.max = max;
    }

    pub fn with_content_mut<F, C, R>(&mut self, cl: F, check: C) -> R
    where
        F: FnOnce(&mut String) -> R,
        C: FnOnce(&String, &mut String, &Option<T>) -> ValueFilterAction,
    {
        let previous = self.text_field.clone();
        let result = cl(&mut self.text_field);

        let parse_result = T::from_str(&self.text_field).ok();

        match check(&previous, &mut self.text_field, &parse_result) {
            ValueFilterAction::InvalidIgnore => {}
            ValueFilterAction::InvalidReset => self.text_field = previous,
            ValueFilterAction::Valid => {
                if let Some(value) = parse_result {
                    if self.is_valid_impl(&value) {
                        self.current_value = value;
                        self.matching = true;
                    } else {
                        self.text_field = previous;
                    }
                } else {
                    self.matching = false;
                }
            }
        }

        result
    }

    pub fn set_value(&mut self, value: T) -> bool {
        if self.is_valid_impl(&value) {
            self.current_value = value;
            self.text_field = self.current_value.to_string();
            true
        } else {
            false
        }
    }

    pub fn set_value_force(&mut self, value: T) -> bool {
        self.current_value = value;
        self.text_field = self.current_value.to_string();
        self.is_valid()
    }

    pub fn is_matching(&self) -> bool {
        self.matching
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_impl(&self.current_value)
    }

    fn is_valid_impl(&self, value: &T) -> bool {
        if let Some(min) = self.min
            && min > *value
        {
            false
        } else if let Some(max) = self.max
            && max < *value
        {
            false
        } else {
            true
        }
    }
}
