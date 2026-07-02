use crate::geometry::Orientation;
use crate::web::WebProps;

#[derive(Debug, Clone, PartialEq)]
pub struct AriaProps {
    pub label: Option<String>,
    pub text_value: Option<String>,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub action: Option<String>,
    pub is_disabled: bool,
    pub is_required: bool,
    pub is_invalid: bool,
    pub is_read_only: bool,
    pub is_multiple: bool,
    pub auto_focus: bool,
    pub is_selected: bool,
    pub is_checked: Option<bool>,
    pub is_expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value_number: Option<f64>,
    pub step_value: Option<f64>,
    pub autocomplete: Option<String>,
    pub input_mode: Option<String>,
    pub pattern: Option<String>,
    pub min_length: Option<u32>,
    pub max_length: Option<u32>,
    pub rows: Option<u32>,
    pub cols: Option<u32>,
    pub size: Option<u32>,
    pub web: WebProps,
}

impl Default for AriaProps {
    fn default() -> Self {
        Self {
            label: None,
            text_value: None,
            value: None,
            placeholder: None,
            action: None,
            is_disabled: false,
            is_required: false,
            is_invalid: false,
            is_read_only: false,
            is_multiple: false,
            auto_focus: false,
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
            step_value: None,
            autocomplete: None,
            input_mode: None,
            pattern: None,
            min_length: None,
            max_length: None,
            rows: None,
            cols: None,
            size: None,
            web: WebProps::default(),
        }
    }
}

impl AriaProps {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn text_value(mut self, text_value: impl Into<String>) -> Self {
        self.text_value = Some(text_value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = Some(action.into());
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.is_disabled = disabled;
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.is_required = required;
        self
    }

    pub fn invalid(mut self, invalid: bool) -> Self {
        self.is_invalid = invalid;
        self
    }

    pub fn read_only(mut self, read_only: bool) -> Self {
        self.is_read_only = read_only;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.is_multiple = multiple;
        self
    }

    pub fn auto_focus(mut self, auto_focus: bool) -> Self {
        self.auto_focus = auto_focus;
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.is_selected = selected;
        self
    }

    pub fn checked(mut self, checked: bool) -> Self {
        self.is_checked = Some(checked);
        self
    }

    pub fn expanded(mut self, expanded: bool) -> Self {
        self.is_expanded = Some(expanded);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }

    pub fn range(mut self, min: Option<f64>, max: Option<f64>, current: Option<f64>) -> Self {
        self.min_value = min;
        self.max_value = max;
        self.value_number = current;
        self
    }

    pub fn step(mut self, step: Option<f64>) -> Self {
        self.step_value = step;
        self
    }

    pub fn autocomplete(mut self, autocomplete: impl Into<String>) -> Self {
        self.autocomplete = Some(autocomplete.into());
        self
    }

    pub fn input_mode(mut self, input_mode: impl Into<String>) -> Self {
        self.input_mode = Some(input_mode.into());
        self
    }

    pub fn pattern(mut self, pattern: impl Into<String>) -> Self {
        self.pattern = Some(pattern.into());
        self
    }

    pub fn min_length(mut self, min_length: Option<u32>) -> Self {
        self.min_length = min_length;
        self
    }

    pub fn max_length(mut self, max_length: Option<u32>) -> Self {
        self.max_length = max_length;
        self
    }

    pub fn rows(mut self, rows: Option<u32>) -> Self {
        self.rows = rows;
        self
    }

    pub fn cols(mut self, cols: Option<u32>) -> Self {
        self.cols = cols;
        self
    }

    pub fn size(mut self, size: Option<u32>) -> Self {
        self.size = size;
        self
    }

    pub fn dom_attribute(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.web = self.web.attribute(name, value);
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.web = self.web.id(id);
        self
    }

    pub fn class_name(mut self, class_name: impl Into<String>) -> Self {
        self.web = self.web.class_name(class_name);
        self
    }

    pub fn style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
        self.web = self.web.style(property, value);
        self
    }

    pub fn event(mut self, name: impl Into<String>, action: impl Into<String>) -> Self {
        self.web = self.web.event(name, action);
        self
    }

    pub fn on_click(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_click(action);
        self
    }

    pub fn on_press(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_press(action);
        self
    }

    pub fn on_change(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_change(action);
        self
    }

    pub fn on_selection_change(mut self, action: impl Into<String>) -> Self {
        self.web = self.web.on_selection_change(action);
        self
    }

    pub fn web(mut self, web: WebProps) -> Self {
        self.web = web;
        self
    }
}
