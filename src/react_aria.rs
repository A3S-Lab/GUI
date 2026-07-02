use crate::error::GuiResult;
use crate::geometry::Orientation;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::web::WebProps;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AriaComponent {
    Button,
    Label,
    Text,
    Image,
    Media,
    Canvas,
    EmbeddedContent,
    TextField,
    Input,
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    Select,
    SelectValue,
    ListBox,
    ListBoxItem,
    Dialog,
    Popover,
    Tabs,
    TabList,
    Tab,
    TabPanel,
    Group,
    Form,
    Menu,
    MenuItem,
    Separator,
    Slider,
    ProgressBar,
    Toolbar,
    Table,
    TableSection,
    TableRow,
    TableCell,
    TableColumn,
    TableCaption,
}

impl AriaComponent {
    pub const fn as_str(self) -> &'static str {
        match self {
            AriaComponent::Button => "Button",
            AriaComponent::Label => "Label",
            AriaComponent::Text => "Text",
            AriaComponent::Image => "Image",
            AriaComponent::Media => "Media",
            AriaComponent::Canvas => "Canvas",
            AriaComponent::EmbeddedContent => "EmbeddedContent",
            AriaComponent::TextField => "TextField",
            AriaComponent::Input => "Input",
            AriaComponent::Checkbox => "Checkbox",
            AriaComponent::Switch => "Switch",
            AriaComponent::RadioGroup => "RadioGroup",
            AriaComponent::Radio => "Radio",
            AriaComponent::Select => "Select",
            AriaComponent::SelectValue => "SelectValue",
            AriaComponent::ListBox => "ListBox",
            AriaComponent::ListBoxItem => "ListBoxItem",
            AriaComponent::Dialog => "Dialog",
            AriaComponent::Popover => "Popover",
            AriaComponent::Tabs => "Tabs",
            AriaComponent::TabList => "TabList",
            AriaComponent::Tab => "Tab",
            AriaComponent::TabPanel => "TabPanel",
            AriaComponent::Group => "Group",
            AriaComponent::Form => "Form",
            AriaComponent::Menu => "Menu",
            AriaComponent::MenuItem => "MenuItem",
            AriaComponent::Separator => "Separator",
            AriaComponent::Slider => "Slider",
            AriaComponent::ProgressBar => "ProgressBar",
            AriaComponent::Toolbar => "Toolbar",
            AriaComponent::Table => "Table",
            AriaComponent::TableSection => "TableSection",
            AriaComponent::TableRow => "TableRow",
            AriaComponent::TableCell => "TableCell",
            AriaComponent::TableColumn => "TableColumn",
            AriaComponent::TableCaption => "TableCaption",
        }
    }
}

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
    pub is_selected: bool,
    pub is_checked: Option<bool>,
    pub is_expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
    pub value_number: Option<f64>,
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
            is_selected: false,
            is_checked: None,
            is_expanded: None,
            orientation: None,
            min_value: None,
            max_value: None,
            value_number: None,
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

#[derive(Debug, Clone, PartialEq)]
pub struct AriaElement {
    pub key: String,
    pub component: AriaComponent,
    pub props: AriaProps,
    pub children: Vec<AriaElement>,
}

impl AriaElement {
    pub fn new(key: impl Into<String>, component: AriaComponent) -> Self {
        Self {
            key: key.into(),
            component,
            props: AriaProps::default(),
            children: Vec::new(),
        }
    }

    pub fn text(key: impl Into<String>, text: impl Into<String>) -> Self {
        Self::new(key, AriaComponent::Text).with_props(AriaProps::new().text_value(text))
    }

    pub fn with_props(mut self, props: AriaProps) -> Self {
        self.props = props;
        self
    }

    pub fn child(mut self, child: AriaElement) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = AriaElement>) -> Self {
        self.children.extend(children);
        self
    }
}

#[derive(Debug, Clone)]
pub struct ReactAriaMapper;

impl Default for ReactAriaMapper {
    fn default() -> Self {
        Self::new()
    }
}

impl ReactAriaMapper {
    pub fn new() -> Self {
        Self
    }

    pub fn map(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        self.map_element(element)
    }

    fn map_element(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        match element.component {
            AriaComponent::Button => Ok(simple_leaf(
                element,
                NativeRole::Button,
                self.best_label(element)?,
            )),
            AriaComponent::Label | AriaComponent::Text | AriaComponent::SelectValue => Ok(
                simple_leaf(element, NativeRole::Text, self.best_label(element)?),
            ),
            AriaComponent::Image => self.map_container_with_label(element, NativeRole::Image),
            AriaComponent::Media => self.map_container_with_label(element, NativeRole::Media),
            AriaComponent::Canvas => self.map_container(element, NativeRole::Canvas),
            AriaComponent::EmbeddedContent => {
                self.map_container(element, NativeRole::EmbeddedContent)
            }
            AriaComponent::TextField => self.map_text_field(element),
            AriaComponent::Input => Ok(simple_leaf(
                element,
                NativeRole::TextField,
                self.best_label(element)?,
            )),
            AriaComponent::Checkbox => Ok(simple_leaf(
                element,
                NativeRole::Checkbox,
                self.best_label(element)?,
            )),
            AriaComponent::Switch => Ok(simple_leaf(
                element,
                NativeRole::Switch,
                self.best_label(element)?,
            )),
            AriaComponent::RadioGroup => {
                self.map_container_with_label(element, NativeRole::RadioGroup)
            }
            AriaComponent::Radio => Ok(radio_leaf(element, self.best_label(element)?)),
            AriaComponent::Select => self.map_select(element),
            AriaComponent::ListBox => self.map_container(element, NativeRole::ListBox),
            AriaComponent::ListBoxItem => Ok(simple_leaf(
                element,
                NativeRole::ListBoxItem,
                self.best_label(element)?,
            )),
            AriaComponent::Dialog => self.map_container_with_label(element, NativeRole::Dialog),
            AriaComponent::Popover => self.map_container(element, NativeRole::Popover),
            AriaComponent::Tabs => self.map_tabs(element),
            AriaComponent::TabList => self.map_container(element, NativeRole::TabList),
            AriaComponent::Tab => Ok(simple_leaf(
                element,
                NativeRole::Tab,
                self.best_label(element)?,
            )),
            AriaComponent::TabPanel => self.map_container(element, NativeRole::TabPanel),
            AriaComponent::Group | AriaComponent::Form => {
                self.map_container(element, NativeRole::View)
            }
            AriaComponent::Menu => self.map_container(element, NativeRole::Menu),
            AriaComponent::MenuItem => Ok(simple_leaf(
                element,
                NativeRole::MenuItem,
                self.best_label(element)?,
            )),
            AriaComponent::Separator => Ok(simple_leaf(element, NativeRole::Separator, None)),
            AriaComponent::Slider => Ok(simple_leaf(
                element,
                NativeRole::Slider,
                self.best_label(element)?,
            )),
            AriaComponent::ProgressBar => Ok(simple_leaf(
                element,
                NativeRole::ProgressBar,
                self.best_label(element)?,
            )),
            AriaComponent::Toolbar => self.map_container(element, NativeRole::Toolbar),
            AriaComponent::Table => self.map_container(element, NativeRole::Table),
            AriaComponent::TableSection => self.map_container(element, NativeRole::TableSection),
            AriaComponent::TableRow => self.map_container(element, NativeRole::TableRow),
            AriaComponent::TableCell => {
                self.map_container_with_label(element, NativeRole::TableCell)
            }
            AriaComponent::TableColumn => self.map_container(element, NativeRole::TableColumn),
            AriaComponent::TableCaption => {
                self.map_container_with_label(element, NativeRole::TableCaption)
            }
        }
    }

    fn map_container(&self, element: &AriaElement, role: NativeRole) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), role).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );
        native.children = element
            .children
            .iter()
            .map(|child| self.map_element(child))
            .collect::<GuiResult<Vec<_>>>()?;
        Ok(native)
    }

    fn map_container_with_label(
        &self,
        element: &AriaElement,
        role: NativeRole,
    ) -> GuiResult<NativeElement> {
        self.map_container(element, role)
    }

    fn map_text_field(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| first_child_label(element, AriaComponent::Label))
            .or_else(|| first_text_child(element));
        let input = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::Input);
        let mut props = native_props_from_aria(&element.props, label);
        if props.value.is_none() {
            props.value = input.and_then(|child| child.props.value.clone());
        }
        if props.placeholder.is_none() {
            props.placeholder = input.and_then(|child| child.props.placeholder.clone());
        }
        if let Some(input) = input {
            if props.action.is_none() {
                props.action = input
                    .props
                    .action
                    .clone()
                    .or_else(|| input.props.web.primary_action().map(str::to_string));
            }
            for (name, action) in &input.props.web.events {
                props
                    .web
                    .events
                    .entry(name.clone())
                    .or_insert_with(|| action.clone());
            }
            for (name, value) in &input.props.web.style {
                props
                    .web
                    .style
                    .entry(name.clone())
                    .or_insert_with(|| value.clone());
            }
            for (name, value) in &input.props.web.attributes {
                props
                    .web
                    .attributes
                    .entry(name.clone())
                    .or_insert_with(|| value.clone());
            }
            props.metadata = props.web.metadata();
        }
        Ok(NativeElement::new(element.key.clone(), NativeRole::TextField).with_props(props))
    }

    fn map_select(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let label = element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| first_child_label(element, AriaComponent::Label));
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Select)
            .with_props(native_props_from_aria(&element.props, label));

        if let Some(list_box) = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::ListBox)
        {
            native.children = list_box
                .children
                .iter()
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        }

        Ok(native)
    }

    fn map_tabs(&self, element: &AriaElement) -> GuiResult<NativeElement> {
        let mut native = NativeElement::new(element.key.clone(), NativeRole::Tabs).with_props(
            native_props_from_aria(&element.props, self.best_label(element)?),
        );

        let tabs = element
            .children
            .iter()
            .find(|child| child.component == AriaComponent::TabList)
            .map(|tab_list| tab_list.children.as_slice())
            .unwrap_or(element.children.as_slice());
        let panels = element
            .children
            .iter()
            .filter(|child| child.component == AriaComponent::TabPanel)
            .collect::<Vec<_>>();

        for (index, tab) in tabs
            .iter()
            .filter(|child| child.component == AriaComponent::Tab)
            .enumerate()
        {
            let mut native_tab = simple_leaf(tab, NativeRole::Tab, self.best_label(tab)?);
            if let Some(panel) = panels.get(index) {
                native_tab.children.push(self.map_element(panel)?);
            }
            native.children.push(native_tab);
        }

        if native.children.is_empty() {
            native.children = element
                .children
                .iter()
                .map(|child| self.map_element(child))
                .collect::<GuiResult<Vec<_>>>()?;
        }

        Ok(native)
    }

    fn best_label(&self, element: &AriaElement) -> GuiResult<Option<String>> {
        Ok(element
            .props
            .label
            .clone()
            .or_else(|| aria_label(&element.props))
            .or_else(|| element.props.text_value.clone())
            .or_else(|| first_text_child(element)))
    }
}

fn simple_leaf(element: &AriaElement, role: NativeRole, label: Option<String>) -> NativeElement {
    NativeElement::new(element.key.clone(), role)
        .with_props(native_props_from_aria(&element.props, label))
}

fn radio_leaf(element: &AriaElement, label: Option<String>) -> NativeElement {
    let mut props = native_props_from_aria(&element.props, label);
    if props.checked.is_none() && props.selected {
        props.checked = Some(true);
    }
    NativeElement::new(element.key.clone(), NativeRole::Radio).with_props(props)
}

fn native_props_from_aria(props: &AriaProps, label: Option<String>) -> NativeProps {
    let mut native = NativeProps::new()
        .disabled(props.is_disabled)
        .required(props.is_required)
        .invalid(props.is_invalid)
        .selected(props.is_selected);
    native.label = label;
    native.value = props.value.clone();
    native.placeholder = props.placeholder.clone();
    native.action = props
        .action
        .clone()
        .or_else(|| props.web.primary_action().map(str::to_string));
    native.checked = props.is_checked;
    native.expanded = props.is_expanded;
    native.orientation = props.orientation;
    native.min = props.min_value;
    native.max = props.max_value;
    native.current = props.value_number;
    native.metadata = props.web.metadata();
    native.web = props.web.clone();
    native
}

fn first_child_label(element: &AriaElement, component: AriaComponent) -> Option<String> {
    element.children.iter().find_map(|child| {
        if child.component == component {
            child
                .props
                .label
                .clone()
                .or_else(|| aria_label(&child.props))
                .or_else(|| child.props.text_value.clone())
                .or_else(|| first_text_child(child))
        } else {
            None
        }
    })
}

fn first_text_child(element: &AriaElement) -> Option<String> {
    element.children.iter().find_map(|child| {
        if child.component == AriaComponent::Text || child.component == AriaComponent::Label {
            child
                .props
                .text_value
                .clone()
                .or_else(|| child.props.label.clone())
        } else {
            first_text_child(child)
        }
    })
}

fn aria_label(props: &AriaProps) -> Option<String> {
    props.web.attributes.get("aria-label").cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::accessibility::{AccessibilityNode, AccessibilityRole};
    use crate::host::{HeadlessHost, HostOperation};
    use crate::platform::{native_widget_name, NativeBackendKind};
    use crate::renderer::Renderer;

    #[test]
    fn maps_button_to_native_button_with_accessibility_label() {
        let aria = AriaElement::new("save", AriaComponent::Button)
            .with_props(AriaProps::new().label("Save").action("save"));

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::Button);
        assert_eq!(native.props.label.as_deref(), Some("Save"));
        assert_eq!(native.props.action.as_deref(), Some("save"));

        let accessibility = AccessibilityNode::from_native(&native);
        assert_eq!(accessibility.role, AccessibilityRole::Button);
        assert_eq!(accessibility.label.as_deref(), Some("Save"));
    }

    #[test]
    fn aria_label_becomes_native_accessibility_label_without_visible_text() {
        let aria = AriaElement::new("save", AriaComponent::Button).with_props(
            AriaProps::new()
                .dom_attribute("aria-label", "Save profile")
                .on_press("saveProfile"),
        );

        let native = ReactAriaMapper::new().map(&aria).unwrap();
        let accessibility = AccessibilityNode::from_native(&native);

        assert_eq!(native.role, NativeRole::Button);
        assert_eq!(native.props.label.as_deref(), Some("Save profile"));
        assert_eq!(native.props.action.as_deref(), Some("saveProfile"));
        assert_eq!(accessibility.label.as_deref(), Some("Save profile"));
    }

    #[test]
    fn aria_label_overrides_descendant_text_for_container_name() {
        let aria = AriaElement::new("preferences", AriaComponent::Dialog)
            .with_props(AriaProps::new().dom_attribute("aria-label", "Preferences"))
            .child(
                AriaElement::new("close", AriaComponent::Button)
                    .child(AriaElement::text("close-text", "Close")),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::Dialog);
        assert_eq!(native.props.label.as_deref(), Some("Preferences"));
        assert_eq!(native.children[0].props.label.as_deref(), Some("Close"));
    }

    #[test]
    fn folds_text_field_label_and_input_into_one_native_control() {
        let aria = AriaElement::new("email-field", AriaComponent::TextField)
            .child(AriaElement::text("email-label", "Email"))
            .child(
                AriaElement::new("email-input", AriaComponent::Input).with_props(
                    AriaProps::new()
                        .placeholder("you@example.com")
                        .value("a@b.c"),
                ),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::TextField);
        assert_eq!(native.props.label.as_deref(), Some("Email"));
        assert_eq!(native.props.placeholder.as_deref(), Some("you@example.com"));
        assert_eq!(native.props.value.as_deref(), Some("a@b.c"));
        assert!(native.children.is_empty());
    }

    #[test]
    fn folded_text_field_inherits_input_web_events_and_style() {
        let aria = AriaElement::new("email-field", AriaComponent::TextField)
            .child(AriaElement::text("email-label", "Email"))
            .child(
                AriaElement::new("email-input", AriaComponent::Input).with_props(
                    AriaProps::new()
                        .on_change("setEmail")
                        .style("minWidth", "280")
                        .dom_attribute("data-testid", "email-input"),
                ),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::TextField);
        assert_eq!(native.props.action.as_deref(), Some("setEmail"));
        assert_eq!(
            native.props.web.events.get("onChange").map(String::as_str),
            Some("setEmail")
        );
        assert_eq!(
            native.props.web.style.get("minWidth").map(String::as_str),
            Some("280")
        );
        assert_eq!(
            native.props.metadata.get("data-testid").map(String::as_str),
            Some("email-input")
        );
    }

    #[test]
    fn maps_select_listbox_items_to_native_options() {
        let aria = AriaElement::new("project", AriaComponent::Select)
            .child(
                AriaElement::new("project-label", AriaComponent::Label)
                    .with_props(AriaProps::new().text_value("Project")),
            )
            .child(
                AriaElement::new("project-options", AriaComponent::ListBox)
                    .child(
                        AriaElement::new("a3s", AriaComponent::ListBoxItem)
                            .with_props(AriaProps::new().text_value("A3S").selected(true)),
                    )
                    .child(
                        AriaElement::new("other", AriaComponent::ListBoxItem)
                            .with_props(AriaProps::new().text_value("Other")),
                    ),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::Select);
        assert_eq!(native.props.label.as_deref(), Some("Project"));
        assert_eq!(native.children.len(), 2);
        assert_eq!(native.children[0].role, NativeRole::ListBoxItem);
        assert_eq!(native.children[0].props.label.as_deref(), Some("A3S"));
        assert!(native.children[0].props.selected);
    }

    #[test]
    fn maps_checkbox_and_switch_to_native_toggle_controls() {
        let checkbox = AriaElement::new("accept", AriaComponent::Checkbox).with_props(
            AriaProps::new()
                .text_value("Accept terms")
                .checked(true)
                .on_change("setAccepted"),
        );
        let switch = AriaElement::new("notifications", AriaComponent::Switch).with_props(
            AriaProps::new()
                .text_value("Notifications")
                .checked(false)
                .on_change("setNotifications"),
        );

        let checkbox = ReactAriaMapper::new().map(&checkbox).unwrap();
        let switch = ReactAriaMapper::new().map(&switch).unwrap();

        assert_eq!(checkbox.role, NativeRole::Checkbox);
        assert_eq!(checkbox.props.label.as_deref(), Some("Accept terms"));
        assert_eq!(checkbox.props.checked, Some(true));
        assert_eq!(checkbox.props.action.as_deref(), Some("setAccepted"));
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, checkbox.role),
            "NSButton(checkbox)"
        );

        assert_eq!(switch.role, NativeRole::Switch);
        assert_eq!(switch.props.checked, Some(false));
        assert_eq!(switch.props.action.as_deref(), Some("setNotifications"));
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, switch.role),
            "NSSwitch"
        );
    }

    #[test]
    fn maps_radio_group_to_native_radio_controls() {
        let aria = AriaElement::new("theme", AriaComponent::RadioGroup)
            .with_props(
                AriaProps::new()
                    .label("Theme")
                    .orientation(Orientation::Vertical)
                    .on_change("setTheme"),
            )
            .child(
                AriaElement::new("light", AriaComponent::Radio)
                    .with_props(AriaProps::new().text_value("Light").value("light")),
            )
            .child(
                AriaElement::new("dark", AriaComponent::Radio).with_props(
                    AriaProps::new()
                        .text_value("Dark")
                        .value("dark")
                        .selected(true),
                ),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::RadioGroup);
        assert_eq!(native.props.label.as_deref(), Some("Theme"));
        assert_eq!(native.props.action.as_deref(), Some("setTheme"));
        assert_eq!(native.props.orientation, Some(Orientation::Vertical));
        assert_eq!(native.children.len(), 2);
        assert_eq!(native.children[1].role, NativeRole::Radio);
        assert_eq!(native.children[1].props.label.as_deref(), Some("Dark"));
        assert_eq!(native.children[1].props.value.as_deref(), Some("dark"));
        assert!(native.children[1].props.selected);
        assert_eq!(native.children[1].props.checked, Some(true));
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, native.role),
            "NSStackView(radio-group)"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, native.children[1].role),
            "NSButton(radio)"
        );
    }

    #[test]
    fn folds_tabs_tablist_and_panels_into_native_tab_items() {
        let aria = AriaElement::new("settings", AriaComponent::Tabs)
            .with_props(AriaProps::new().on_selection_change("setTab"))
            .child(
                AriaElement::new("settings-tabs", AriaComponent::TabList)
                    .child(
                        AriaElement::new("profile-tab", AriaComponent::Tab)
                            .with_props(AriaProps::new().text_value("Profile").selected(true)),
                    )
                    .child(
                        AriaElement::new("billing-tab", AriaComponent::Tab)
                            .with_props(AriaProps::new().text_value("Billing")),
                    ),
            )
            .child(
                AriaElement::new("profile-panel", AriaComponent::TabPanel)
                    .child(AriaElement::text("profile-title", "Profile settings")),
            )
            .child(
                AriaElement::new("billing-panel", AriaComponent::TabPanel)
                    .child(AriaElement::text("billing-title", "Billing settings")),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::Tabs);
        assert_eq!(
            native
                .props
                .web
                .events
                .get("onSelectionChange")
                .map(String::as_str),
            Some("setTab")
        );
        assert_eq!(native.children.len(), 2);
        assert_eq!(native.children[0].role, NativeRole::Tab);
        assert_eq!(native.children[0].props.label.as_deref(), Some("Profile"));
        assert!(native.children[0].props.selected);
        assert_eq!(native.children[0].children.len(), 1);
        assert_eq!(native.children[0].children[0].role, NativeRole::TabPanel);
        assert_eq!(
            native.children[0].children[0].children[0].role,
            NativeRole::Text
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, native.role),
            "Microsoft.UI.Xaml.Controls.TabView"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, native.children[0].role),
            "Microsoft.UI.Xaml.Controls.TabViewItem"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, native.role),
            "gtk::Notebook"
        );
    }

    #[test]
    fn maps_menu_and_menu_items_to_native_menu_roles() {
        let aria = AriaElement::new("file-menu", AriaComponent::Menu)
            .child(
                AriaElement::new("open", AriaComponent::MenuItem).with_props(
                    AriaProps::new()
                        .text_value("Open")
                        .value("open")
                        .on_press("openFile"),
                ),
            )
            .child(
                AriaElement::new("recent", AriaComponent::MenuItem)
                    .with_props(AriaProps::new().text_value("Recent")),
            );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.role, NativeRole::Menu);
        assert_eq!(native.children.len(), 2);
        assert_eq!(native.children[0].role, NativeRole::MenuItem);
        assert_eq!(native.children[0].props.label.as_deref(), Some("Open"));
        assert_eq!(native.children[0].props.value.as_deref(), Some("open"));
        assert_eq!(native.children[0].props.action.as_deref(), Some("openFile"));
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, native.role),
            "NSMenu"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, native.children[0].role),
            "Microsoft.UI.Xaml.Controls.Button(menu-item)"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, native.children[0].role),
            "gio::MenuItem"
        );
    }

    #[test]
    fn accepts_web_compatible_props_and_normalizes_primary_action() {
        let aria = AriaElement::new("save", AriaComponent::Button).with_props(
            AriaProps::new()
                .label("Save")
                .class_name("primary")
                .style("backgroundColor", "rebeccapurple")
                .dom_attribute("aria-label", "Save document")
                .dom_attribute("data-testid", "save-button")
                .on_click("saveDocument"),
        );

        let native = ReactAriaMapper::new().map(&aria).unwrap();

        assert_eq!(native.props.action.as_deref(), Some("saveDocument"));
        assert_eq!(native.props.web.class_name.as_deref(), Some("primary"));
        assert_eq!(
            native
                .props
                .web
                .style
                .get("backgroundColor")
                .map(String::as_str),
            Some("rebeccapurple")
        );
        assert_eq!(
            native.props.metadata.get("aria-label").map(String::as_str),
            Some("Save document")
        );
        assert_eq!(
            native.props.metadata.get("data-testid").map(String::as_str),
            Some("save-button")
        );
    }

    #[test]
    fn renderer_updates_native_node_without_remounting_same_key_and_role() {
        let first = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Save"));
        let second = NativeElement::new("save", NativeRole::Button)
            .with_props(NativeProps::new().label("Saved"));
        let mut renderer = Renderer::new();
        let mut host = HeadlessHost::default();

        let first_id = renderer.render(&first, &mut host).unwrap();
        host.clear_operations();
        let second_id = renderer.render(&second, &mut host).unwrap();

        assert_eq!(first_id, second_id);
        assert!(host.operations().iter().any(
            |operation| matches!(operation, HostOperation::Update { id, .. } if *id == first_id)
        ));
        assert!(!host
            .operations()
            .iter()
            .any(|operation| matches!(operation, HostOperation::Create { .. })));
    }

    #[test]
    fn platform_names_point_to_native_widget_families() {
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, NativeRole::Button),
            "NSButton"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, NativeRole::TextField),
            "Microsoft.UI.Xaml.Controls.TextBox"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, NativeRole::Select),
            "gtk::DropDown"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, NativeRole::Separator),
            "NSBox(separator)"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, NativeRole::Separator),
            "Microsoft.UI.Xaml.Controls.Border(separator)"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, NativeRole::Separator),
            "gtk::Separator"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, NativeRole::Image),
            "NSImageView"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, NativeRole::Media),
            "Microsoft.UI.Xaml.Controls.MediaPlayerElement"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, NativeRole::Canvas),
            "gtk::DrawingArea"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::AppKit, NativeRole::Table),
            "NSTableView"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::WinUI, NativeRole::TableCell),
            "Microsoft.UI.Xaml.Controls.Grid(cell)"
        );
        assert_eq!(
            native_widget_name(NativeBackendKind::Gtk4, NativeRole::TableCaption),
            "gtk::Label(table-caption)"
        );
    }
}
