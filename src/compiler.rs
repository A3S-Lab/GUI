use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

mod components;
mod props;

use components::component_from_jsx_tag;

use crate::error::GuiResult;
use crate::native::NativeElement;
use crate::react_aria::{AriaElement, ReactAriaMapper};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CompiledJsxNode {
    Element {
        key: String,
        tag: String,
        #[serde(default)]
        import_source: Option<String>,
        #[serde(default)]
        props: CompiledProps,
        #[serde(default)]
        children: Vec<CompiledJsxNode>,
    },
    Text {
        key: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledProps {
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub text_value: Option<String>,
    #[serde(default)]
    pub value: Option<String>,
    #[serde(default)]
    pub placeholder: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default, alias = "aria-label")]
    pub aria_label: Option<String>,
    #[serde(default, alias = "disabled", alias = "aria-disabled")]
    pub is_disabled: bool,
    #[serde(default, alias = "required", alias = "aria-required")]
    pub is_required: bool,
    #[serde(default, alias = "invalid", alias = "aria-invalid")]
    pub is_invalid: bool,
    #[serde(
        default,
        alias = "readOnly",
        alias = "readonly",
        alias = "aria-readonly"
    )]
    pub is_read_only: bool,
    #[serde(default, alias = "selected", alias = "aria-selected")]
    pub is_selected: bool,
    #[serde(default, alias = "checked", alias = "aria-checked")]
    pub is_checked: Option<bool>,
    #[serde(default, alias = "expanded", alias = "aria-expanded")]
    pub is_expanded: Option<bool>,
    #[serde(default, alias = "aria-orientation")]
    pub orientation: Option<CompiledOrientation>,
    #[serde(default, alias = "min", alias = "aria-valuemin")]
    pub min_value: Option<f64>,
    #[serde(default, alias = "max", alias = "aria-valuemax")]
    pub max_value: Option<f64>,
    #[serde(default, alias = "current", alias = "aria-valuenow")]
    pub value_number: Option<f64>,
    #[serde(default, alias = "step")]
    pub step_value: Option<f64>,
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub class_name: Option<String>,
    #[serde(default)]
    pub style: BTreeMap<String, CompiledStyleValue>,
    #[serde(default)]
    pub attributes: BTreeMap<String, String>,
    #[serde(default)]
    pub events: BTreeMap<String, String>,
}

impl Default for CompiledProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            value: None,
            placeholder: None,
            action: None,
            aria_label: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            step_value: None,
            id: None,
            class_name: None,
            style: BTreeMap::new(),
            attributes: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompiledOrientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompiledStyleValue {
    String(String),
    Number(f64),
    Bool(bool),
}

impl CompiledStyleValue {
    pub fn to_portable_value(&self) -> String {
        match self {
            CompiledStyleValue::String(value) => value.clone(),
            CompiledStyleValue::Number(value) => {
                if value.fract() == 0.0 {
                    format!("{value:.0}")
                } else {
                    value.to_string()
                }
            }
            CompiledStyleValue::Bool(value) => value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ReactCompilerBridge {
    mapper: ReactAriaMapper,
}

impl ReactCompilerBridge {
    pub fn new() -> Self {
        Self {
            mapper: ReactAriaMapper::new(),
        }
    }

    pub fn lower_to_aria(&self, node: &CompiledJsxNode) -> GuiResult<AriaElement> {
        lower_node(node)
    }

    pub fn lower_to_native(&self, node: &CompiledJsxNode) -> GuiResult<NativeElement> {
        let aria = self.lower_to_aria(node)?;
        self.mapper.map(&aria)
    }
}

fn lower_node(node: &CompiledJsxNode) -> GuiResult<AriaElement> {
    match node {
        CompiledJsxNode::Text { key, value } => Ok(AriaElement::text(key.clone(), value.clone())),
        CompiledJsxNode::Element {
            key,
            tag,
            props,
            children,
            ..
        } => {
            let component = component_from_jsx_tag(tag, props)?;
            let mut element = AriaElement::new(key.clone(), component)
                .with_props(props.clone().into_aria_props_for_tag(tag, children));
            element.children = children
                .iter()
                .map(lower_node)
                .collect::<GuiResult<Vec<_>>>()?;
            Ok(element)
        }
    }
}

#[cfg(test)]
mod tests;
