use std::{
    collections::{HashMap, btree_map::ValuesMut},
    i64, num,
};

use derive_where::derive_where;
use eframe::Frame;
use egui::{Color32, InnerResponse, Pos2, RichText, TextEdit, Ui};
use egui_typed_input::ValText;
use serde_json::{Number, Value};

use crate::{
    editor::EditorError,
    workspace::{
        self,
        content::{Content, ContentType},
        nodes::{Connector, NodeDescription},
        workspace::Workspace,
    },
};

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNode<'a> {
    pub title: String,
    #[derive_where(skip)]
    pub description: &'a NodeDescription,
    pub values: HashMap<String, (&'a Content, Value)>,
}

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNodeProto<'a> {
    pub pos: Pos2,
    pub variant_index: usize,
    #[derive_where(skip)]
    pub workspace: &'a Workspace,
    pub values: HashMap<String, Value>,
}

#[derive(Default, Debug)]
pub struct HyConnection {
    pub from_node: usize,
    pub from_connector: usize,
    pub to_node: usize,
    pub to_connector: usize,
}

impl<'a> HyNode<'a> {
    pub fn new(description: &'a NodeDescription) -> Self {
        Self {
            title: description.title.clone(),
            description,
            values: description
                .content
                .iter()
                .map(|v| (v.id.clone(), (v, v.options.get_default())))
                .collect(),
        }
    }

    pub fn draw_content(&mut self, ui: &mut Ui) {
        egui::containers::Frame::group(ui.style()).show(ui, |ui| {
            ui.vertical(|ui| {
                for content in self.values.iter_mut() {
                    let (content_id, (content_ref, value)) = content;
                    let common = content_ref.options.get_common();
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    if let Some(width) = common.1 {
                        ui.set_min_width(width as f32);
                    }

                    if !matches!(
                        content_ref.options,
                        ContentType::Checkbox { .. } | ContentType::Bool { .. }
                    ) {
                        ui.strong(common.0);
                    }

                    match &content_ref.options {
                        workspace::content::ContentType::SmallString {
                            label,
                            default,
                            width,
                        } => {
                            if let Some(val) = value.as_str() {
                                let mut cloned = val.to_string();
                                ui.text_edit_singleline(&mut cloned);
                            }
                        }
                        workspace::content::ContentType::Enum {
                            label,
                            width,
                            values,
                            default,
                        } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                        workspace::content::ContentType::List {
                            label,
                            width,
                            array_element_type,
                        } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                        workspace::content::ContentType::IntSlider {
                            label,
                            width,
                            default,
                            tick_frequency,
                            min,
                            max,
                        } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                        workspace::content::ContentType::String {
                            label,
                            width,
                            height,
                        } => {
                            ui.set_min_height(*height as f32);
                            if let Some(val) = value.as_str() {
                                let mut cloned = val.to_string();
                                ui.text_edit_multiline(&mut cloned);
                            }
                        }
                        workspace::content::ContentType::Checkbox { label, .. }
                        | workspace::content::ContentType::Bool { label, .. } => {
                            if let Some(val) = value.as_bool() {
                                let mut val2 = val;
                                ui.checkbox(&mut val2, label);

                                if val2 != val {
                                    *value = Value::Bool(val2)
                                }
                            }
                        }
                        workspace::content::ContentType::Int {
                            label,
                            width,
                            default,
                            min,
                            max,
                        } => {
                            if let Some(val) = value.as_f64().map(|n| n as i64) {
                                let mut valin = ValText::<i64, _>::number_int();
                                valin.set_val(val);

                                let mut edit = TextEdit::singleline(&mut valin);

                                if min.unwrap_or(i64::MIN) > val || max.unwrap_or(i64::MAX) < val {
                                    edit = edit.text_color(Color32::RED);
                                }

                                edit.show(ui);

                                if let Some(Ok(new_val)) = valin.get_val() {
                                    if *new_val != val {
                                        *value = Value::Number(Number::from(*new_val))
                                    }
                                }
                            }
                        }
                        workspace::content::ContentType::Float {
                            label,
                            width,
                            default,
                            min,
                            max,
                        } => {
                            if let Some(val) = value.as_f64() {
                                let mut valin = ValText::<f64, _>::number();
                                valin.set_val(val);

                                let mut edit = TextEdit::singleline(&mut valin);

                                if min.unwrap_or(f64::MIN) > val || max.unwrap_or(f64::MAX) < val {
                                    edit = edit.text_color(Color32::RED);
                                }

                                edit.show(ui);

                                if let Some(Ok(new_val)) = valin.get_val() {
                                    if *new_val != val {
                                        if let Some(new_val) = Number::from_f64(*new_val) {
                                            *value = Value::Number(new_val);
                                        }
                                        else {
                                    println!("HI2")
                                }
                                    }
                                }
                                else {
                                    println!("HI")
                                }
                                
                            }
                        }
                        workspace::content::ContentType::Object { label, fields } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                    }
                }
            });
        });
    }
}

impl<'a> TryFrom<HyNodeProto<'a>> for HyNode<'a> {
    type Error = EditorError;

    fn try_from(mut value: HyNodeProto<'a>) -> Result<Self, Self::Error> {
        let desc = value
            .workspace
            .nodes
            .get(value.variant_index)
            .ok_or_else(|| {
                EditorError::NodeVariantIndexResolve(
                    value.variant_index,
                    value.workspace.workspace.workspace_name.clone(),
                )
            })?;

        let values = desc
            .content
            .iter()
            .map(|content| {
                (
                    content.id.clone(),
                    value
                        .values
                        .remove(&content.id)
                        .or_else(|| {
                            let dv = content.options.get_default();
                            if dv.is_null() { None } else { Some(dv) }
                        })
                        .map(|v| (content, v))
                        .unwrap_or((content, Value::Null)),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Self {
            title: desc.title.to_string(),
            description: desc,
            values,
        })
    }
}
