use std::collections::BTreeMap;

use crate::error::{GuiError, GuiResult};

use super::CompiledProps;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ComponentClassVariants {
    axes: Vec<ComponentClassVariantAxis>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ComponentClassVariantAxis {
    prop: String,
    default_value: Option<String>,
    classes: BTreeMap<String, String>,
}

impl ComponentClassVariants {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn axis<I, K, V>(
        self,
        prop: impl Into<String>,
        default_value: impl Into<String>,
        classes: I,
    ) -> GuiResult<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.optional_axis(prop, Some(default_value.into()), classes)
    }

    pub fn optional_axis<I, K, V>(
        mut self,
        prop: impl Into<String>,
        default_value: Option<String>,
        classes: I,
    ) -> GuiResult<Self>
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        let prop = prop.into();
        validate_variant_prop_name(&prop)?;
        if self.axes.iter().any(|axis| axis.prop == prop) {
            return Err(GuiError::invalid_tree(format!(
                "RSX component variant prop {prop:?} was registered more than once"
            )));
        }
        let classes = classes
            .into_iter()
            .map(|(value, class_name)| {
                let value = value.into();
                validate_variant_value(&prop, &value)?;
                Ok((value, class_name.into()))
            })
            .collect::<GuiResult<BTreeMap<_, _>>>()?;
        if classes.is_empty() {
            return Err(GuiError::invalid_tree(format!(
                "RSX component variant prop {prop:?} needs at least one value"
            )));
        }
        if let Some(default_value) = default_value.as_deref() {
            validate_variant_value(&prop, default_value)?;
            if !classes.contains_key(default_value) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX component variant prop {prop:?} default value {default_value:?} is not registered"
                )));
            }
        }
        self.axes.push(ComponentClassVariantAxis {
            prop,
            default_value,
            classes,
        });
        Ok(self)
    }

    pub(crate) fn apply_to_props(
        &self,
        component: &str,
        props: &mut CompiledProps,
    ) -> GuiResult<()> {
        let caller_class_name = props.class_name.take();
        let mut variant_class_name = None;
        for axis in &self.axes {
            let selected =
                component_variant_prop_value(props, &axis.prop).or(axis.default_value.as_deref());
            let Some(selected) = selected.filter(|value| !value.trim().is_empty()) else {
                continue;
            };
            let Some(class_name) = axis.classes.get(selected) else {
                let expected = axis.classes.keys().cloned().collect::<Vec<_>>().join(", ");
                return Err(GuiError::invalid_tree(format!(
                    "RSX component {component:?} prop {:?} has unsupported variant value {selected:?}; expected one of {expected}",
                    axis.prop
                )));
            };
            variant_class_name = merge_class_names(variant_class_name, class_name.clone());
        }
        props.class_name = merge_optional_class_names(variant_class_name, caller_class_name);
        Ok(())
    }
}

pub(super) fn merge_class_names(existing: Option<String>, incoming: String) -> Option<String> {
    let incoming = incoming.trim();
    match (existing, incoming.is_empty()) {
        (None, true) => Some(String::new()),
        (None, false) => Some(incoming.to_string()),
        (Some(existing), true) => {
            let existing = existing.trim();
            (!existing.is_empty()).then(|| existing.to_string())
        }
        (Some(existing), false) => {
            let existing = existing.trim();
            if existing.is_empty() {
                Some(incoming.to_string())
            } else {
                Some(format!("{existing} {incoming}"))
            }
        }
    }
}

fn merge_optional_class_names(
    existing: Option<String>,
    incoming: Option<String>,
) -> Option<String> {
    match incoming {
        Some(incoming) => merge_class_names(existing, incoming),
        None => existing,
    }
}

fn component_variant_prop_value<'a>(props: &'a CompiledProps, prop: &str) -> Option<&'a str> {
    match prop {
        "class" | "className" => props.class_name.as_deref(),
        "label" => props.label.as_deref(),
        "textValue" => props.text_value.as_deref(),
        "value" => props.value.as_deref(),
        "placeholder" => props.placeholder.as_deref(),
        "action" => props.action.as_deref(),
        "aria-label" | "ariaLabel" => props.aria_label.as_deref(),
        "id" => props.id.as_deref(),
        "name" => props.name.as_deref(),
        "form" => props.form.as_deref(),
        "type" | "inputType" => props.input_type.as_deref(),
        other => props.attributes.get(other).map(String::as_str),
    }
}

fn validate_variant_prop_name(prop: &str) -> GuiResult<()> {
    if prop.trim().is_empty() || prop.chars().any(char::is_whitespace) {
        return Err(GuiError::invalid_tree(format!(
            "RSX component variant prop {prop:?} must be non-empty and contain no whitespace"
        )));
    }
    Ok(())
}

fn validate_variant_value(prop: &str, value: &str) -> GuiResult<()> {
    if value.trim().is_empty() || value.chars().any(char::is_whitespace) {
        return Err(GuiError::invalid_tree(format!(
            "RSX component variant prop {prop:?} value {value:?} must be non-empty and contain no whitespace"
        )));
    }
    Ok(())
}
