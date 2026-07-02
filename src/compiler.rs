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
    pub name: Option<String>,
    #[serde(default)]
    pub form: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
    #[serde(default)]
    pub accept: Option<String>,
    #[serde(default)]
    pub capture: Option<String>,
    #[serde(default)]
    pub alt: Option<String>,
    #[serde(default)]
    pub href: Option<String>,
    #[serde(default)]
    pub src: Option<String>,
    #[serde(default, alias = "srcSet")]
    pub srcset: Option<String>,
    #[serde(default)]
    pub sizes: Option<String>,
    #[serde(default)]
    pub media: Option<String>,
    #[serde(default)]
    pub resource_type: Option<String>,
    #[serde(default)]
    pub intrinsic_width: Option<u32>,
    #[serde(default)]
    pub intrinsic_height: Option<u32>,
    #[serde(default)]
    pub loading: Option<String>,
    #[serde(default)]
    pub decoding: Option<String>,
    #[serde(default)]
    pub fetch_priority: Option<String>,
    #[serde(default)]
    pub cross_origin: Option<String>,
    #[serde(default)]
    pub referrer_policy: Option<String>,
    #[serde(default)]
    pub poster: Option<String>,
    #[serde(default)]
    pub controls: Option<bool>,
    #[serde(default, alias = "autoPlay")]
    pub autoplay: Option<bool>,
    #[serde(default)]
    pub loop_playback: Option<bool>,
    #[serde(default)]
    pub muted: Option<bool>,
    #[serde(default)]
    pub plays_inline: Option<bool>,
    #[serde(default)]
    pub preload: Option<String>,
    #[serde(default)]
    pub track_kind: Option<String>,
    #[serde(default, alias = "srcLang")]
    pub srclang: Option<String>,
    #[serde(default)]
    pub track_label: Option<String>,
    #[serde(default)]
    pub default_track: Option<bool>,
    #[serde(default)]
    pub list: Option<String>,
    #[serde(default)]
    pub dirname: Option<String>,
    #[serde(default)]
    pub form_action: Option<String>,
    #[serde(default, alias = "formEncType")]
    pub form_enctype: Option<String>,
    #[serde(default)]
    pub form_method: Option<String>,
    #[serde(default)]
    pub form_target: Option<String>,
    #[serde(default)]
    pub form_no_validate: Option<bool>,
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
            name: None,
            form: None,
            input_type: None,
            accept: None,
            capture: None,
            alt: None,
            href: None,
            src: None,
            srcset: None,
            sizes: None,
            media: None,
            resource_type: None,
            intrinsic_width: None,
            intrinsic_height: None,
            loading: None,
            decoding: None,
            fetch_priority: None,
            cross_origin: None,
            referrer_policy: None,
            poster: None,
            controls: None,
            autoplay: None,
            loop_playback: None,
            muted: None,
            plays_inline: None,
            preload: None,
            track_kind: None,
            srclang: None,
            track_label: None,
            default_track: None,
            list: None,
            dirname: None,
            form_action: None,
            form_enctype: None,
            form_method: None,
            form_target: None,
            form_no_validate: None,
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
