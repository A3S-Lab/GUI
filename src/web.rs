use std::collections::BTreeMap;

use crate::css_text::parse_style_declarations;
use crate::event::non_empty_action;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WebProps {
    pub id: Option<String>,
    pub class_name: Option<String>,
    pub style: BTreeMap<String, String>,
    pub attributes: BTreeMap<String, String>,
    pub events: BTreeMap<String, String>,
}

impl WebProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.class_name = Some(class_name.into());
        self
    }

    pub fn style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.style.insert(property.into(), value.into());
        self
    }

    pub fn attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = name.into();
        let value = value.into();
        match name.as_str() {
            "id" => self.id = Some(value),
            "class" | "className" => self.class_name = Some(value),
            "style" => {
                let declarations = parse_style_declarations(&value);
                for declaration in declarations
                    .iter()
                    .filter(|declaration| !declaration.important)
                {
                    self.style
                        .insert(declaration.property.clone(), declaration.value.clone());
                }
                for declaration in declarations
                    .iter()
                    .filter(|declaration| declaration.important)
                {
                    self.style
                        .insert(declaration.property.clone(), declaration.value.clone());
                }
            }
            _ => {
                self.attributes.insert(name, value);
            }
        }
        self
    }

    pub fn event(mut self, name: impl Into<String>, action: impl Into<String>) -> Self {
        self.events.insert(name.into(), action.into());
        self
    }

    pub fn on_click(self, action: impl Into<String>) -> Self {
        self.event("onClick", action)
    }

    pub fn on_press(self, action: impl Into<String>) -> Self {
        self.event("onPress", action)
    }

    pub fn on_press_start(self, action: impl Into<String>) -> Self {
        self.event("onPressStart", action)
    }

    pub fn on_press_end(self, action: impl Into<String>) -> Self {
        self.event("onPressEnd", action)
    }

    pub fn on_change(self, action: impl Into<String>) -> Self {
        self.event("onChange", action)
    }

    pub fn on_input(self, action: impl Into<String>) -> Self {
        self.event("onInput", action)
    }

    pub fn on_selection_change(self, action: impl Into<String>) -> Self {
        self.event("onSelectionChange", action)
    }

    pub fn on_focus(self, action: impl Into<String>) -> Self {
        self.event("onFocus", action)
    }

    pub fn on_blur(self, action: impl Into<String>) -> Self {
        self.event("onBlur", action)
    }

    pub fn on_focus_change(self, action: impl Into<String>) -> Self {
        self.event("onFocusChange", action)
    }

    pub fn on_toggle(self, action: impl Into<String>) -> Self {
        self.event("onToggle", action)
    }

    pub fn on_expanded_change(self, action: impl Into<String>) -> Self {
        self.event("onExpandedChange", action)
    }

    pub fn on_key_down(self, action: impl Into<String>) -> Self {
        self.event("onKeyDown", action)
    }

    pub fn on_key_up(self, action: impl Into<String>) -> Self {
        self.event("onKeyUp", action)
    }

    pub fn on_copy(self, action: impl Into<String>) -> Self {
        self.event("onCopy", action)
    }

    pub fn on_cut(self, action: impl Into<String>) -> Self {
        self.event("onCut", action)
    }

    pub fn on_paste(self, action: impl Into<String>) -> Self {
        self.event("onPaste", action)
    }

    pub fn primary_action(&self) -> Option<&str> {
        non_empty_action(self.events.get("onPress"))
            .or_else(|| non_empty_action(self.events.get("onClick")))
            .or_else(|| non_empty_action(self.events.get("onChange")))
            .or_else(|| non_empty_action(self.events.get("onInput")))
    }

    pub fn metadata(&self) -> BTreeMap<String, String> {
        let mut metadata = BTreeMap::new();
        if let Some(id) = &self.id {
            metadata.insert("id".to_string(), id.clone());
        }
        if let Some(class_name) = &self.class_name {
            metadata.insert("className".to_string(), class_name.clone());
        }
        for (key, value) in &self.attributes {
            metadata.insert(key.clone(), value.clone());
        }
        metadata
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn semantic_ui_on_press_is_primary_action() {
        let props = WebProps::new()
            .on_click("fallbackClick")
            .on_press("primaryPress");

        assert_eq!(props.primary_action(), Some("primaryPress"));
    }

    #[test]
    fn primary_action_ignores_empty_action_ids() {
        let props = WebProps::new().on_press("").on_click("fallbackClick");

        assert_eq!(props.primary_action(), Some("fallbackClick"));
    }

    #[test]
    fn primary_action_uses_input_event_as_value_fallback() {
        let props = WebProps::new().on_input("setQuery");

        assert_eq!(props.primary_action(), Some("setQuery"));
    }

    #[test]
    fn style_attribute_preserves_css_text_delimiters_inside_values() {
        let props = WebProps::new().attribute(
            "style",
            r#"
            color: rgb(10 20 30 / 50%);
            border-color: color-mix(in srgb, red 40%, blue) !important;
            border-color: #fff;
            background-image: url("https://example.com/a:b;c.svg");
            content: "label: value; still text";
            --accent: color-mix(in srgb, rebeccapurple 40%, white);
            /* ignored comment: with delimiter; */
            padding-inline: 1rem 2rem;
            "#,
        );

        assert_eq!(
            props.style.get("color").map(String::as_str),
            Some("rgb(10 20 30 / 50%)")
        );
        assert_eq!(
            props.style.get("border-color").map(String::as_str),
            Some("color-mix(in srgb, red 40%, blue)")
        );
        assert_eq!(
            props.style.get("background-image").map(String::as_str),
            Some(r#"url("https://example.com/a:b;c.svg")"#)
        );
        assert_eq!(
            props.style.get("content").map(String::as_str),
            Some(r#""label: value; still text""#)
        );
        assert_eq!(
            props.style.get("--accent").map(String::as_str),
            Some("color-mix(in srgb, rebeccapurple 40%, white)")
        );
        assert_eq!(
            props.style.get("padding-inline").map(String::as_str),
            Some("1rem 2rem")
        );
        assert!(!props.style.contains_key("ignored comment"));
    }
}
