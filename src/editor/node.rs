use std::collections::HashMap;

use derive_where::derive_where;
use egui::emath::*;
use egui::{Color32, Pos2, RichText, TextEdit, Ui};
use serde_json::Value;

use crate::{
    editor::{
        EditorError,
        value::{NodeEditorValueTypes, ValueFilterAction},
    },
    workspace::{
        self,
        content::{Content, ContentType},
        nodes::NodeDescription,
        workspace::Workspace,
    },
};

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNode<'a> {
    pub title: String,
    #[derive_where(skip)]
    pub description: &'a NodeDescription,
    pub values: HashMap<String, (&'a Content, NodeEditorValueTypes)>,
}

#[derive(Clone)]
#[derive_where(Debug)]
pub struct HyNodeProto<'a> {
    pub pos: Pos2,
    pub variant_index: usize,
    #[derive_where(skip)]
    pub workspace: &'a Workspace,
    pub values: HashMap<String, NodeEditorValueTypes>,
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
                .map(|v| {
                    (
                        v.id.clone(),
                        (
                            v,
                            NodeEditorValueTypes::from_value(Value::Null, &v.options).ok(),
                        ),
                    )
                })
                .filter_map(|v| {
                    if let Some(vt) = v.1.1 {
                        Some((v.0, (v.1.0, vt)))
                    } else {
                        None
                    }
                })
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
                        workspace::content::ContentType::SmallString { .. } => {
                            if let NodeEditorValueTypes::String(val) = value {
                                ui.text_edit_singleline(val);
                            }
                        }
                        workspace::content::ContentType::Enum { .. } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                        workspace::content::ContentType::List { .. } => {
                            ui.label(RichText::new("JSON only").underline());
                        }
                        workspace::content::ContentType::IntSlider {
                            min,
                            max,
                            tick_frequency,
                            ..
                        } => {
                            if let NodeEditorValueTypes::Integer(val) = value {
                                ui.add(
                                    egui::Slider::new(val, *min..=*max)
                                        .step_by(*tick_frequency as f64),
                                );
                            }
                        }
                        workspace::content::ContentType::String { height, .. } => {
                            ui.set_min_height(*height as f32);
                            if let NodeEditorValueTypes::String(val) = value {
                                ui.text_edit_multiline(val);
                            }
                        }
                        workspace::content::ContentType::Checkbox { label, .. }
                        | workspace::content::ContentType::Bool { label, .. } => {
                            if let NodeEditorValueTypes::Boolean(val) = value {
                                ui.checkbox(val, label);
                            }
                        }
                        workspace::content::ContentType::Int { .. } => {
                            if let NodeEditorValueTypes::IntegerText(val) = value {
                                let valid = val.is_valid() && val.is_matching();
                                val.with_content_mut(
                                    |txt| {
                                        let mut edit = TextEdit::singleline(txt);
                                        if !valid {
                                            edit = edit.text_color(Color32::RED);
                                        }
                                        edit.show(ui);
                                    },
                                    |_prev, next, res| {
                                        if next.contains('.') || !res.is_some() {
                                            ValueFilterAction::InvalidReset
                                        } else {
                                            ValueFilterAction::Valid
                                        }
                                    },
                                );
                            }
                        }
                        workspace::content::ContentType::Float { .. } => {
                            if let NodeEditorValueTypes::FloatText(val) = value {
                                let valid = val.is_valid() && val.is_matching();
                                val.with_content_mut(
                                    |txt| {
                                        let mut edit = TextEdit::singleline(txt);
                                        if !valid {
                                            edit = edit.text_color(Color32::RED);
                                        }
                                        edit.show(ui);
                                    },
                                    |_prev, _next, res| {
                                        if !res.is_some() {
                                            ValueFilterAction::InvalidReset
                                        } else {
                                            ValueFilterAction::Valid
                                        }
                                    },
                                );
                            }
                        }
                        workspace::content::ContentType::Object { .. } => {
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
                        .map(|v| (content, v))
                        .unwrap_or_else(|| {
                            (
                                content,
                                NodeEditorValueTypes::from_value(Value::Null, &content.options)
                                    .unwrap(),
                            )
                        }),
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
