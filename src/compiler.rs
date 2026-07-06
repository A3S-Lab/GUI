use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{Map as JsonMap, Value as JsonValue};

mod component_variants;
mod components;
mod props;

use component_variants::merge_class_names;
pub use component_variants::ComponentClassVariants;
use components::component_from_rsx_tag;

use crate::error::{GuiError, GuiResult};
use crate::native::NativeElement;
use crate::semantic_ui::{use_press_value, SemanticElement, SemanticMapper, UsePressProps};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum CompiledRsxNode {
    Element {
        key: String,
        tag: String,
        #[serde(default)]
        import_source: Option<String>,
        #[serde(default)]
        props: CompiledProps,
        #[serde(default)]
        children: Vec<CompiledRsxNode>,
    },
    Text {
        key: String,
        value: String,
    },
}

impl CompiledRsxNode {
    pub fn validate(&self) -> GuiResult<()> {
        match self {
            CompiledRsxNode::Element {
                key, tag, children, ..
            } => {
                if key.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled elements need non-empty keys",
                    ));
                }
                if tag.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled elements need non-empty tags",
                    ));
                }
                validate_compiled_children(children)
            }
            CompiledRsxNode::Text { key, .. } => {
                if key.is_empty() {
                    return Err(GuiError::invalid_tree(
                        "a3s-gui compiled text nodes need non-empty keys",
                    ));
                }
                Ok(())
            }
        }
    }

    fn key(&self) -> &str {
        match self {
            CompiledRsxNode::Element { key, .. } | CompiledRsxNode::Text { key, .. } => key,
        }
    }

    pub fn has_bindings(&self) -> bool {
        match self {
            CompiledRsxNode::Text { .. } => false,
            CompiledRsxNode::Element {
                props, children, ..
            } => {
                !props.bindings.is_empty()
                    || !props.spreads.is_empty()
                    || children.iter().any(CompiledRsxNode::has_bindings)
            }
        }
    }
}

fn validate_compiled_children(children: &[CompiledRsxNode]) -> GuiResult<()> {
    let mut sibling_keys = BTreeSet::new();
    for child in children {
        child.validate()?;
        let key = child.key();
        if !sibling_keys.insert(key) {
            return Err(GuiError::invalid_tree(format!(
                "a3s-gui compiled sibling nodes need unique keys; duplicate key {key:?}"
            )));
        }
    }
    Ok(())
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
    #[serde(default)]
    pub action_labels: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub bindings: BTreeMap<String, CompiledBinding>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub spreads: Vec<CompiledBinding>,
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub explicit_props: BTreeSet<String>,
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
            action_labels: BTreeMap::new(),
            bindings: BTreeMap::new(),
            spreads: Vec::new(),
            explicit_props: BTreeSet::new(),
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompiledBinding {
    pub source: CompiledBindingSource,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CompiledBindingSource {
    State,
    Props,
    Derived,
    Context,
    Resource,
    Local,
}

impl CompiledBinding {
    pub fn state(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::State,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn props(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Props,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn derived(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Derived,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn context(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Context,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn resource(path: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            source: CompiledBindingSource::Resource,
            path: path.into_iter().map(Into::into).collect(),
        }
    }

    pub fn display_path(&self) -> String {
        let root = match self.source {
            CompiledBindingSource::State => "state",
            CompiledBindingSource::Props => "props",
            CompiledBindingSource::Derived => "derived",
            CompiledBindingSource::Context => "context",
            CompiledBindingSource::Resource => "resource",
            CompiledBindingSource::Local => {
                return if self.path.is_empty() {
                    "local".to_string()
                } else {
                    self.path.join(".")
                };
            }
        };
        if self.path.is_empty() {
            root.to_string()
        } else {
            format!("{root}.{}", self.path.join("."))
        }
    }

    fn resolve<'a>(&self, scope: &'a JsonValue) -> GuiResult<&'a JsonValue> {
        let root = match self.source {
            CompiledBindingSource::State => "state",
            CompiledBindingSource::Props => "props",
            CompiledBindingSource::Derived => "derived",
            CompiledBindingSource::Context => "context",
            CompiledBindingSource::Resource => "resource",
            CompiledBindingSource::Local => {
                let Some(root) = self.path.first() else {
                    return Err(GuiError::invalid_tree(
                        "RSX local binding cannot resolve an empty local path",
                    ));
                };
                root
            }
        };
        let mut value = scope.get(root).ok_or_else(|| {
            GuiError::invalid_tree(format!(
                "RSX binding {} cannot resolve missing scope root {root:?}",
                self.display_path()
            ))
        })?;
        let path = match self.source {
            CompiledBindingSource::State
            | CompiledBindingSource::Props
            | CompiledBindingSource::Derived
            | CompiledBindingSource::Context
            | CompiledBindingSource::Resource => self.path.as_slice(),
            CompiledBindingSource::Local => &self.path[1..],
        };
        let display_path = self.display_path();
        for segment in path {
            value = json_path_get(value, segment).ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX binding {display_path} cannot resolve missing path segment {segment:?}",
                ))
            })?;
        }
        Ok(value)
    }
}

fn json_path_get<'a>(value: &'a JsonValue, segment: &str) -> Option<&'a JsonValue> {
    match value {
        JsonValue::Object(object) => object.get(segment),
        JsonValue::Array(items) => segment
            .parse::<usize>()
            .ok()
            .and_then(|index| items.get(index)),
        JsonValue::Null | JsonValue::Bool(_) | JsonValue::Number(_) | JsonValue::String(_) => None,
    }
}

impl CompiledRsxNode {
    pub fn resolve_bindings(&self, scope: &JsonValue) -> GuiResult<Self> {
        self.resolve_bindings_with_components(scope, &BTreeMap::new())
    }

    pub fn resolve_bindings_with_components(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
    ) -> GuiResult<Self> {
        self.resolve_bindings_with_component_defaults(scope, components, &BTreeMap::new())
    }

    pub fn resolve_bindings_with_component_defaults(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    ) -> GuiResult<Self> {
        self.resolve_bindings_with_component_options(
            scope,
            components,
            component_defaults,
            &BTreeMap::new(),
        )
    }

    pub fn resolve_bindings_with_component_options(
        &self,
        scope: &JsonValue,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
        component_variants: &BTreeMap<String, ComponentClassVariants>,
    ) -> GuiResult<Self> {
        let mut resolved = resolve_node_bindings_with_components(
            self,
            scope,
            components,
            component_defaults,
            component_variants,
            &mut Vec::new(),
            None,
        )?;
        match resolved.len() {
            0 => Ok(CompiledRsxNode::Element {
                key: self.key().to_string(),
                tag: "Fragment".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: Vec::new(),
            }),
            1 => Ok(resolved.remove(0)),
            _ => Ok(CompiledRsxNode::Element {
                key: self.key().to_string(),
                tag: "Fragment".to_string(),
                import_source: None,
                props: CompiledProps::default(),
                children: resolved,
            }),
        }
    }
}

fn resolve_node_bindings_with_components(
    node: &CompiledRsxNode,
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    match node {
        CompiledRsxNode::Text { .. } => Ok(vec![node.clone()]),
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => {
            if is_for_control(tag) {
                return resolve_for_control(
                    key,
                    props,
                    children,
                    scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    slots,
                );
            }

            let mut props = props.clone();
            if let Some(defaults) = component_defaults.get(tag) {
                props.apply_default_props(defaults)?;
            }
            props.resolve_bindings(scope)?;
            if let Some(variants) = component_variants.get(tag) {
                variants.apply_to_props(tag, &mut props)?;
            }

            if is_slot_control(tag) {
                if let Some(slots) = slots {
                    return Ok(slots
                        .children(props.name.as_deref())
                        .iter()
                        .cloned()
                        .map(|child| prefix_node_keys(child, key))
                        .collect());
                }
            }

            if let Some(component) = components.get(tag) {
                if component_stack.iter().any(|name| name == tag) {
                    let mut cycle = component_stack.clone();
                    cycle.push(tag.clone());
                    return Err(GuiError::invalid_tree(format!(
                        "RSX component cycle detected: {}",
                        cycle.join(" -> ")
                    )));
                }

                let resolved_slot_children = resolve_children_bindings_with_components(
                    children,
                    scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    slots,
                )?;
                let resolved_slots = ResolvedSlots::from_children(resolved_slot_children);
                let component_scope = extend_component_scope(scope, &props)?;
                component_stack.push(tag.clone());
                let resolved = resolve_node_bindings_with_components(
                    component,
                    &component_scope,
                    components,
                    component_defaults,
                    component_variants,
                    component_stack,
                    Some(&resolved_slots),
                )?
                .into_iter()
                .map(|child| prefix_node_keys(child, key))
                .collect::<Vec<_>>();
                component_stack.pop();
                return Ok(resolved);
            }

            if is_show_control(tag) {
                return if show_condition(tag, &props)? {
                    resolve_children_bindings_with_components(
                        children,
                        scope,
                        components,
                        component_defaults,
                        component_variants,
                        component_stack,
                        slots,
                    )
                } else {
                    Ok(Vec::new())
                };
            }

            let children = resolve_children_bindings_with_components(
                children,
                scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )?;

            Ok(vec![CompiledRsxNode::Element {
                key: key.clone(),
                tag: tag.clone(),
                import_source: import_source.clone(),
                props,
                children,
            }])
        }
    }
}

fn resolve_children_bindings_with_components(
    children: &[CompiledRsxNode],
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let children = children
        .iter()
        .map(|child| {
            resolve_node_bindings_with_components(
                child,
                scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )
        })
        .collect::<GuiResult<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    Ok(children)
}

fn is_show_control(tag: &str) -> bool {
    matches!(tag, "Show" | "When")
}

fn is_for_control(tag: &str) -> bool {
    matches!(tag, "For" | "Each")
}

fn is_slot_control(tag: &str) -> bool {
    matches!(tag, "Slot" | "slot")
}

#[derive(Debug, Default)]
struct ResolvedSlots {
    default: Vec<CompiledRsxNode>,
    named: BTreeMap<String, Vec<CompiledRsxNode>>,
}

impl ResolvedSlots {
    fn from_children(children: Vec<CompiledRsxNode>) -> Self {
        let mut slots = Self::default();
        for mut child in children {
            match take_structural_slot_name(&mut child) {
                Some(name) if !name.is_empty() => {
                    slots.named.entry(name).or_default().push(child);
                }
                _ => slots.default.push(child),
            }
        }
        slots
    }

    fn children(&self, name: Option<&str>) -> &[CompiledRsxNode] {
        match name {
            Some(name) if !name.is_empty() => {
                self.named.get(name).map(Vec::as_slice).unwrap_or(&[])
            }
            _ => self.default.as_slice(),
        }
    }
}

fn take_structural_slot_name(node: &mut CompiledRsxNode) -> Option<String> {
    let CompiledRsxNode::Element { props, .. } = node else {
        return None;
    };
    props.attributes.remove("slot")
}

fn resolve_for_control(
    key: &str,
    props: &CompiledProps,
    children: &[CompiledRsxNode],
    scope: &JsonValue,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    component_variants: &BTreeMap<String, ComponentClassVariants>,
    component_stack: &mut Vec<String>,
    slots: Option<&ResolvedSlots>,
) -> GuiResult<Vec<CompiledRsxNode>> {
    let item_name = control_identifier_attribute(props, "as")?.unwrap_or("item");
    let index_name = control_identifier_attribute(props, "indexAs")?;
    if index_name == Some(item_name) {
        return Err(GuiError::invalid_tree(
            "RSX <For> indexAs cannot reuse the item variable name",
        ));
    }
    let key_by = props.attributes.get("keyBy").map(String::as_str);
    let each = for_each_binding(props)?;
    let items = each.resolve(scope)?.as_array().ok_or_else(|| {
        GuiError::invalid_tree(format!(
            "RSX <For> each binding {} must resolve to an array",
            each.display_path()
        ))
    })?;

    let mut rendered = Vec::new();
    let mut item_keys = BTreeSet::new();
    for (index, item) in items.iter().enumerate() {
        let item_key = for_item_key(item, key_by, index)?;
        if !item_keys.insert(item_key.clone()) {
            return Err(GuiError::invalid_tree(format!(
                "RSX <For> produced duplicate item key {item_key:?}"
            )));
        }
        let item_scope = extend_local_scope(scope, item_name, item, index_name, index)?;
        let item_prefix = format!("{key}-{item_key}");
        rendered.extend(
            resolve_children_bindings_with_components(
                children,
                &item_scope,
                components,
                component_defaults,
                component_variants,
                component_stack,
                slots,
            )?
            .into_iter()
            .map(|child| prefix_node_keys(child, &item_prefix)),
        );
    }

    Ok(rendered)
}

fn extend_component_scope(scope: &JsonValue, props: &CompiledProps) -> GuiResult<JsonValue> {
    let JsonValue::Object(scope) = scope else {
        return Err(GuiError::invalid_tree(
            "RSX binding scope must be a JSON object",
        ));
    };
    let mut scope = scope.clone();
    scope.insert("props".to_string(), props_scope_value(props));
    Ok(JsonValue::Object(scope))
}

fn props_scope_value(props: &CompiledProps) -> JsonValue {
    let mut scope = JsonMap::new();

    if let Ok(press) = press_scope_value(props) {
        if let JsonValue::Object(press_scope) = &press {
            if let Some(press_props) = press_scope.get("pressProps") {
                scope.insert("pressProps".to_string(), press_props.clone());
            }
            if let Some(is_pressed) = press_scope.get("isPressed") {
                scope.insert("isPressed".to_string(), is_pressed.clone());
            }
        }
        scope.insert("press".to_string(), press);
    }

    insert_optional_string(&mut scope, "label", props.label.as_ref());
    insert_optional_string(&mut scope, "textValue", props.text_value.as_ref());
    insert_optional_string(&mut scope, "value", props.value.as_ref());
    insert_optional_string(&mut scope, "placeholder", props.placeholder.as_ref());
    insert_optional_string(&mut scope, "action", props.action.as_ref());
    insert_optional_string(&mut scope, "aria-label", props.aria_label.as_ref());
    insert_optional_string(&mut scope, "ariaLabel", props.aria_label.as_ref());
    insert_optional_string(&mut scope, "id", props.id.as_ref());
    insert_optional_string(&mut scope, "className", props.class_name.as_ref());
    insert_optional_string(&mut scope, "name", props.name.as_ref());
    insert_optional_string(&mut scope, "form", props.form.as_ref());
    insert_optional_string(&mut scope, "type", props.input_type.as_ref());
    insert_optional_string(&mut scope, "inputType", props.input_type.as_ref());
    insert_optional_string(&mut scope, "href", props.href.as_ref());
    insert_optional_string(&mut scope, "src", props.src.as_ref());
    insert_optional_string(&mut scope, "alt", props.alt.as_ref());

    scope.insert("isDisabled".to_string(), JsonValue::Bool(props.is_disabled));
    scope.insert("disabled".to_string(), JsonValue::Bool(props.is_disabled));
    scope.insert("isRequired".to_string(), JsonValue::Bool(props.is_required));
    scope.insert("required".to_string(), JsonValue::Bool(props.is_required));
    scope.insert("isInvalid".to_string(), JsonValue::Bool(props.is_invalid));
    scope.insert("invalid".to_string(), JsonValue::Bool(props.is_invalid));
    scope.insert(
        "isReadOnly".to_string(),
        JsonValue::Bool(props.is_read_only),
    );
    scope.insert("readOnly".to_string(), JsonValue::Bool(props.is_read_only));
    scope.insert("isSelected".to_string(), JsonValue::Bool(props.is_selected));
    scope.insert("selected".to_string(), JsonValue::Bool(props.is_selected));

    insert_optional_bool(&mut scope, "isChecked", props.is_checked);
    insert_optional_bool(&mut scope, "checked", props.is_checked);
    insert_optional_bool(&mut scope, "isExpanded", props.is_expanded);
    insert_optional_bool(&mut scope, "expanded", props.is_expanded);
    insert_optional_number(&mut scope, "min", props.min_value);
    insert_optional_number(&mut scope, "minValue", props.min_value);
    insert_optional_number(&mut scope, "max", props.max_value);
    insert_optional_number(&mut scope, "maxValue", props.max_value);
    insert_optional_number(&mut scope, "step", props.step_value);
    insert_optional_number(&mut scope, "stepValue", props.step_value);
    insert_optional_number(&mut scope, "valueNumber", props.value_number);
    if let Some(orientation) = props.orientation {
        let value = match orientation {
            CompiledOrientation::Horizontal => "horizontal",
            CompiledOrientation::Vertical => "vertical",
        };
        scope.insert(
            "orientation".to_string(),
            JsonValue::String(value.to_string()),
        );
    }

    for (name, value) in &props.attributes {
        scope
            .entry(name.clone())
            .or_insert_with(|| JsonValue::String(value.clone()));
    }
    for (name, value) in &props.events {
        scope.insert(name.clone(), JsonValue::String(value.clone()));
    }

    JsonValue::Object(scope)
}

fn press_scope_value(props: &CompiledProps) -> GuiResult<JsonValue> {
    use_press_value(
        UsePressProps::new()
            .on_press(non_empty_prop_action(
                props.events.get("onPress").or(props.action.as_ref()),
            ))
            .on_press_start(non_empty_prop_action(props.events.get("onPressStart")))
            .on_press_end(non_empty_prop_action(props.events.get("onPressEnd")))
            .disabled(props.is_disabled)
            .pressed(bool_attribute_value(props, &["isPressed", "pressed"]).unwrap_or(false)),
    )
}

fn non_empty_prop_action(action: Option<&String>) -> Option<String> {
    action.filter(|action| !action.is_empty()).cloned()
}

fn bool_attribute_value(props: &CompiledProps, names: &[&str]) -> Option<bool> {
    names.iter().find_map(|name| {
        props.attributes.get(*name).and_then(|value| {
            match value.trim().to_ascii_lowercase().as_str() {
                "" | "true" => Some(true),
                "false" => Some(false),
                _ => None,
            }
        })
    })
}

fn insert_optional_string(
    scope: &mut JsonMap<String, JsonValue>,
    name: &str,
    value: Option<&String>,
) {
    if let Some(value) = value {
        scope.insert(name.to_string(), JsonValue::String(value.clone()));
    }
}

fn insert_optional_bool(scope: &mut JsonMap<String, JsonValue>, name: &str, value: Option<bool>) {
    if let Some(value) = value {
        scope.insert(name.to_string(), JsonValue::Bool(value));
    }
}

fn insert_optional_number(scope: &mut JsonMap<String, JsonValue>, name: &str, value: Option<f64>) {
    if let Some(value) = value {
        if let Some(number) = serde_json::Number::from_f64(value) {
            scope.insert(name.to_string(), JsonValue::Number(number));
        }
    }
}

fn for_each_binding(props: &CompiledProps) -> GuiResult<&CompiledBinding> {
    let each = props.bindings.get("each");
    let of = props.bindings.get("of");
    match (each, of) {
        (Some(_), Some(_)) => Err(GuiError::invalid_tree(
            "RSX <For> cannot use both each={...} and of={...}",
        )),
        (Some(binding), None) | (None, Some(binding)) => {
            for property in props.bindings.keys() {
                if property != "each" && property != "of" {
                    return Err(GuiError::invalid_tree(format!(
                        "RSX <For> only supports dynamic each/of bindings; property {property:?} must be static"
                    )));
                }
            }
            Ok(binding)
        }
        (None, None) => Err(GuiError::invalid_tree(
            "RSX <For> needs an each={state.items} binding",
        )),
    }
}

fn extend_local_scope(
    scope: &JsonValue,
    item_name: &str,
    item: &JsonValue,
    index_name: Option<&str>,
    index: usize,
) -> GuiResult<JsonValue> {
    let JsonValue::Object(scope) = scope else {
        return Err(GuiError::invalid_tree(
            "RSX binding scope must be a JSON object",
        ));
    };
    let mut scope = scope.clone();
    scope.insert(item_name.to_string(), item.clone());
    if let Some(index_name) = index_name {
        scope.insert(
            index_name.to_string(),
            JsonValue::Number(serde_json::Number::from(index as u64)),
        );
    }
    Ok(JsonValue::Object(scope))
}

fn for_item_key(item: &JsonValue, key_by: Option<&str>, index: usize) -> GuiResult<String> {
    let Some(key_by) = key_by else {
        return Ok(index.to_string());
    };
    let key = if key_by == "." {
        item
    } else {
        let mut value = item;
        for segment in key_by.split('.') {
            if segment.trim().is_empty() {
                return Err(GuiError::invalid_tree(
                    "RSX <For> keyBy cannot contain empty path segments",
                ));
            }
            value = json_path_get(value, segment).ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX <For> keyBy path segment {segment:?} is missing"
                ))
            })?;
        }
        value
    };
    let key = binding_string("keyBy", key)?;
    if key.is_empty() {
        Err(GuiError::invalid_tree(
            "RSX <For> keyBy resolved to an empty item key",
        ))
    } else {
        Ok(key)
    }
}

fn prefix_node_keys(node: CompiledRsxNode, prefix: &str) -> CompiledRsxNode {
    match node {
        CompiledRsxNode::Text { key, value } => CompiledRsxNode::Text {
            key: format!("{prefix}-{key}"),
            value,
        },
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => CompiledRsxNode::Element {
            key: format!("{prefix}-{key}"),
            tag,
            import_source,
            props,
            children: children
                .into_iter()
                .map(|child| prefix_node_keys(child, prefix))
                .collect(),
        },
    }
}

fn control_identifier_attribute<'a>(
    props: &'a CompiledProps,
    name: &str,
) -> GuiResult<Option<&'a str>> {
    if props.bindings.contains_key(name) {
        return Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a static identifier"
        )));
    }
    let Some(value) = props.attributes.get(name) else {
        return Ok(None);
    };
    if is_valid_local_identifier(value) {
        Ok(Some(value.as_str()))
    } else {
        Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must be a valid identifier"
        )))
    }
}

fn is_valid_local_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if matches!(
        value,
        "state" | "props" | "derived" | "context" | "resource"
    ) {
        return false;
    }
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

fn show_condition(tag: &str, props: &CompiledProps) -> GuiResult<bool> {
    let when = control_bool_attribute(props, "when")?;
    let unless = control_bool_attribute(props, "unless")?;

    match (when, unless) {
        (Some(when), Some(unless)) => Ok(when && !unless),
        (Some(when), None) => Ok(when),
        (None, Some(unless)) => Ok(!unless),
        (None, None) => Err(GuiError::invalid_tree(format!(
            "RSX <{tag}> needs a boolean when={{...}} or unless={{...}} binding"
        ))),
    }
}

fn control_bool_attribute(props: &CompiledProps, name: &str) -> GuiResult<Option<bool>> {
    let Some(value) = props.attributes.get(name) else {
        return Ok(None);
    };
    match value.as_str() {
        "true" => Ok(Some(true)),
        "false" => Ok(Some(false)),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX control attribute {name:?} must resolve to a boolean"
        ))),
    }
}

impl CompiledProps {
    pub(crate) fn apply_default_props(
        &mut self,
        defaults: &BTreeMap<String, JsonValue>,
    ) -> GuiResult<()> {
        let explicit_props = self.explicit_props.clone();
        for (property, value) in defaults {
            if explicit_props.contains(&canonical_prop_name(property)) {
                continue;
            }
            self.apply_resolved_binding(property, value)?;
        }
        Ok(())
    }

    pub fn resolve_bindings(&mut self, scope: &JsonValue) -> GuiResult<()> {
        let explicit_props = self.explicit_props.clone();
        for binding in self.spreads.clone() {
            let value = binding.resolve(scope)?;
            self.apply_resolved_spread(&binding, value, &explicit_props)?;
        }
        for (property, binding) in self.bindings.clone() {
            let value = binding.resolve(scope)?;
            self.apply_resolved_binding(&property, value)?;
        }
        self.bindings.clear();
        self.spreads.clear();
        self.explicit_props.clear();
        Ok(())
    }

    fn apply_resolved_spread(
        &mut self,
        binding: &CompiledBinding,
        value: &JsonValue,
        explicit_props: &BTreeSet<String>,
    ) -> GuiResult<()> {
        let JsonValue::Object(object) = value else {
            return Err(GuiError::invalid_tree(format!(
                "RSX spread {} must resolve to an object",
                binding.display_path()
            )));
        };

        for (property, value) in object {
            if property == "key" {
                return Err(GuiError::invalid_tree(
                    "RSX spread props cannot provide key; keyed identity must be explicit",
                ));
            }
            if explicit_props.contains(&canonical_prop_name(property)) {
                continue;
            }
            self.apply_resolved_binding(property, value)?;
        }
        Ok(())
    }

    fn apply_resolved_binding(&mut self, property: &str, value: &JsonValue) -> GuiResult<()> {
        match property {
            "label" => self.label = Some(binding_string(property, value)?),
            "textValue" => self.text_value = Some(binding_string(property, value)?),
            "value" => self.value = Some(binding_string(property, value)?),
            "placeholder" => self.placeholder = Some(binding_string(property, value)?),
            "action" => self.action = Some(binding_string(property, value)?),
            "aria-label" | "ariaLabel" => self.aria_label = Some(binding_string(property, value)?),
            "id" => self.id = Some(binding_string(property, value)?),
            "name" => self.name = Some(binding_string(property, value)?),
            "form" => self.form = Some(binding_string(property, value)?),
            "type" | "inputType" => self.input_type = Some(binding_string(property, value)?),
            "class" | "className" => {
                self.class_name =
                    merge_class_names(self.class_name.take(), binding_string(property, value)?)
            }
            "style" => self
                .style
                .extend(parse_style_text(&binding_string(property, value)?)),
            "orientation" => {
                self.orientation = match binding_string(property, value)?.as_str() {
                    "horizontal" => Some(CompiledOrientation::Horizontal),
                    "vertical" => Some(CompiledOrientation::Vertical),
                    other => {
                        return Err(GuiError::invalid_tree(format!(
                            "RSX binding for property {property:?} resolved to unsupported orientation {other:?}"
                        )))
                    }
                };
            }
            "isDisabled" | "disabled" => self.is_disabled = binding_bool(property, value)?,
            "isRequired" | "required" => self.is_required = binding_bool(property, value)?,
            "isInvalid" | "invalid" => self.is_invalid = binding_bool(property, value)?,
            "isReadOnly" | "readOnly" => self.is_read_only = binding_bool(property, value)?,
            "isSelected" | "selected" => self.is_selected = binding_bool(property, value)?,
            "isChecked" | "checked" => self.is_checked = Some(binding_bool(property, value)?),
            "isExpanded" | "expanded" => self.is_expanded = Some(binding_bool(property, value)?),
            "min" | "minValue" => self.min_value = Some(binding_number(property, value)?),
            "max" | "maxValue" => self.max_value = Some(binding_number(property, value)?),
            "step" | "stepValue" => self.step_value = Some(binding_number(property, value)?),
            "valueNumber" => self.value_number = Some(binding_number(property, value)?),
            other if other.starts_with("on") => {
                self.events.insert(
                    normalize_event_name(other),
                    binding_string(property, value)?,
                );
            }
            other if is_action_payload_property(other) => {
                self.attributes
                    .insert(other.to_string(), binding_payload_string(property, value)?);
            }
            other if other.starts_with("aria-") || other.starts_with("data-") => {
                self.attributes
                    .insert(other.to_string(), binding_string(property, value)?);
            }
            other => {
                self.attributes
                    .insert(other.to_string(), binding_string(property, value)?);
            }
        }
        Ok(())
    }
}

fn canonical_prop_name(name: &str) -> String {
    match name {
        "class" | "className" => "className".to_string(),
        "aria-label" | "ariaLabel" => "aria-label".to_string(),
        "disabled" | "isDisabled" => "isDisabled".to_string(),
        "required" | "isRequired" => "isRequired".to_string(),
        "invalid" | "isInvalid" => "isInvalid".to_string(),
        "readOnly" | "readonly" | "isReadOnly" => "isReadOnly".to_string(),
        "selected" | "isSelected" => "isSelected".to_string(),
        "checked" | "isChecked" => "isChecked".to_string(),
        "expanded" | "isExpanded" => "isExpanded".to_string(),
        "min" | "minValue" => "minValue".to_string(),
        "max" | "maxValue" => "maxValue".to_string(),
        "step" | "stepValue" => "stepValue".to_string(),
        "type" | "inputType" => "inputType".to_string(),
        other if other.starts_with("on") => normalize_event_name(other),
        other => other.to_string(),
    }
}

fn normalize_event_name(name: &str) -> String {
    match name {
        "onclick" => "onClick",
        "onpress" => "onPress",
        "onchange" => "onChange",
        "oninput" => "onInput",
        "onselectionchange" => "onSelectionChange",
        "onfocus" => "onFocus",
        "onblur" => "onBlur",
        "onfocuschange" => "onFocusChange",
        "ontoggle" => "onToggle",
        "onexpandedchange" => "onExpandedChange",
        "onkeydown" => "onKeyDown",
        "onkeyup" => "onKeyUp",
        _ => name,
    }
    .to_string()
}

fn parse_style_text(style: &str) -> BTreeMap<String, CompiledStyleValue> {
    style
        .split(';')
        .filter_map(|declaration| {
            let (property, value) = declaration.split_once(':')?;
            let property = property.trim();
            let value = value.trim();
            if property.is_empty() || value.is_empty() {
                return None;
            }
            Some((
                property.to_string(),
                value
                    .parse::<f64>()
                    .map(CompiledStyleValue::Number)
                    .unwrap_or_else(|_| CompiledStyleValue::String(value.to_string())),
            ))
        })
        .collect()
}

fn is_action_payload_property(property: &str) -> bool {
    matches!(
        property,
        "actionPayload" | "action-payload" | "data-action-payload" | "data-a3s-action-payload"
    )
}

fn binding_payload_string(property: &str, value: &JsonValue) -> GuiResult<String> {
    match value {
        JsonValue::String(value) => Ok(value.clone()),
        JsonValue::Number(value) => Ok(value.to_string()),
        JsonValue::Bool(value) => Ok(value.to_string()),
        JsonValue::Null => Ok(String::new()),
        JsonValue::Array(_) | JsonValue::Object(_) => {
            serde_json::to_string(value).map_err(|error| {
                GuiError::invalid_tree(format!(
                    "RSX binding for property {property:?} could not serialize action payload: {error}"
                ))
            })
        }
    }
}

fn binding_string(property: &str, value: &JsonValue) -> GuiResult<String> {
    match value {
        JsonValue::String(value) => Ok(value.clone()),
        JsonValue::Number(value) => Ok(value.to_string()),
        JsonValue::Bool(value) => Ok(value.to_string()),
        JsonValue::Null => Ok(String::new()),
        JsonValue::Array(_) | JsonValue::Object(_) => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a scalar value"
        ))),
    }
}

fn binding_bool(property: &str, value: &JsonValue) -> GuiResult<bool> {
    match value {
        JsonValue::Bool(value) => Ok(*value),
        JsonValue::String(value) if value == "true" => Ok(true),
        JsonValue::String(value) if value == "false" => Ok(false),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a boolean"
        ))),
    }
}

fn binding_number(property: &str, value: &JsonValue) -> GuiResult<f64> {
    match value {
        JsonValue::Number(value) => {
            value
                .as_f64()
                .filter(|value| value.is_finite())
                .ok_or_else(|| {
                    GuiError::invalid_tree(format!(
                        "RSX binding for property {property:?} must resolve to a finite number"
                    ))
                })
        }
        JsonValue::String(value) => value
            .parse::<f64>()
            .ok()
            .filter(|value| value.is_finite())
            .ok_or_else(|| {
                GuiError::invalid_tree(format!(
                    "RSX binding for property {property:?} must resolve to a finite number"
                ))
            }),
        _ => Err(GuiError::invalid_tree(format!(
            "RSX binding for property {property:?} must resolve to a number"
        ))),
    }
}

#[derive(Debug, Clone, Default)]
pub struct RsxCompilerBridge {
    mapper: SemanticMapper,
}

impl RsxCompilerBridge {
    pub fn new() -> Self {
        Self {
            mapper: SemanticMapper::new(),
        }
    }

    pub fn lower_to_semantic(&self, node: &CompiledRsxNode) -> GuiResult<SemanticElement> {
        node.validate()?;
        lower_node(node)
    }

    pub fn lower_to_native(&self, node: &CompiledRsxNode) -> GuiResult<NativeElement> {
        let semantic = self.lower_to_semantic(node)?;
        self.mapper.map(&semantic)
    }
}

fn lower_node(node: &CompiledRsxNode) -> GuiResult<SemanticElement> {
    match node {
        CompiledRsxNode::Text { key, value } => {
            Ok(SemanticElement::text(key.clone(), value.clone()))
        }
        CompiledRsxNode::Element {
            key,
            tag,
            props,
            children,
            ..
        } => {
            let component = component_from_rsx_tag(tag, props)?;
            let mut element = SemanticElement::new(key.clone(), component)
                .with_props(props.clone().into_semantic_props_for_tag(tag, children));
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
