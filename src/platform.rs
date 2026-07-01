use std::collections::BTreeMap;

use crate::accessibility::{accessibility_role, AccessibilityRole};
use crate::error::{GuiError, GuiResult};
use crate::geometry::Orientation;
use crate::host::{HostNodeId, NativeHost};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::style::{DisplayMode, PortableStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeBackendKind {
    AppKit,
    WinUI,
    Gtk4,
    Headless,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetBlueprint {
    pub backend: NativeBackendKind,
    pub widget_class: String,
    pub role: NativeRole,
    pub accessibility_role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub class_name: Option<String>,
    pub control_state: NativeControlState,
    pub style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

impl NativeWidgetBlueprint {
    pub fn config(&self) -> NativeWidgetConfig {
        NativeWidgetConfig::from_blueprint(self)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeControlState {
    pub placeholder: Option<String>,
    pub disabled: bool,
    pub required: bool,
    pub invalid: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
}

impl NativeControlState {
    pub fn from_props(props: &NativeProps) -> Self {
        Self {
            placeholder: props.placeholder.clone(),
            disabled: props.disabled,
            required: props.required,
            invalid: props.invalid,
            selected: props.selected,
            checked: props.checked,
            expanded: props.expanded,
            orientation: props.orientation,
            min: props.min,
            max: props.max,
            current: props.current,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetConfig {
    pub backend: NativeBackendKind,
    pub widget_class: String,
    pub role: NativeRole,
    pub accessibility_role: AccessibilityRole,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub class_name: Option<String>,
    pub placeholder: Option<String>,
    pub enabled: bool,
    pub visible: bool,
    pub required: bool,
    pub invalid: bool,
    pub selected: bool,
    pub checked: Option<bool>,
    pub expanded: Option<bool>,
    pub orientation: Option<Orientation>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub current: Option<f64>,
    pub web_style: BTreeMap<String, String>,
    pub portable_style: PortableStyle,
    pub events: BTreeMap<String, String>,
    pub metadata: BTreeMap<String, String>,
}

impl NativeWidgetConfig {
    pub fn from_blueprint(blueprint: &NativeWidgetBlueprint) -> Self {
        let state = &blueprint.control_state;
        Self {
            backend: blueprint.backend,
            widget_class: blueprint.widget_class.clone(),
            role: blueprint.role,
            accessibility_role: blueprint.accessibility_role,
            label: blueprint.label.clone(),
            value: blueprint.value.clone(),
            action: blueprint.action.clone(),
            class_name: blueprint.class_name.clone(),
            placeholder: state.placeholder.clone(),
            enabled: !state.disabled,
            visible: blueprint.portable_style.display != Some(DisplayMode::None),
            required: state.required,
            invalid: state.invalid,
            selected: state.selected,
            checked: state.checked,
            expanded: state.expanded,
            orientation: state.orientation,
            min: state.min,
            max: state.max,
            current: state.current,
            web_style: blueprint.style.clone(),
            portable_style: blueprint.portable_style.clone(),
            events: blueprint.events.clone(),
            metadata: blueprint.metadata.clone(),
        }
    }

    pub fn diff(&self, after: &Self) -> NativeWidgetConfigPatch {
        NativeWidgetConfigPatch::between(self, after)
    }

    pub fn create_setters(&self) -> Vec<NativeWidgetSetter> {
        vec![
            NativeWidgetSetter::SetAccessibilityRole(self.accessibility_role),
            NativeWidgetSetter::SetLabel(self.label.clone()),
            NativeWidgetSetter::SetValue(self.value.clone()),
            NativeWidgetSetter::SetAction(self.action.clone()),
            NativeWidgetSetter::SetClassName(self.class_name.clone()),
            NativeWidgetSetter::SetPlaceholder(self.placeholder.clone()),
            NativeWidgetSetter::SetEnabled(self.enabled),
            NativeWidgetSetter::SetVisible(self.visible),
            NativeWidgetSetter::SetRequired(self.required),
            NativeWidgetSetter::SetInvalid(self.invalid),
            NativeWidgetSetter::SetSelected(self.selected),
            NativeWidgetSetter::SetChecked(self.checked),
            NativeWidgetSetter::SetExpanded(self.expanded),
            NativeWidgetSetter::SetOrientation(self.orientation),
            NativeWidgetSetter::SetMinimum(self.min),
            NativeWidgetSetter::SetMaximum(self.max),
            NativeWidgetSetter::SetCurrent(self.current),
            NativeWidgetSetter::SetWebStyle(self.web_style.clone()),
            NativeWidgetSetter::SetPortableStyle(self.portable_style.clone()),
            NativeWidgetSetter::SetEvents(self.events.clone()),
            NativeWidgetSetter::SetMetadata(self.metadata.clone()),
        ]
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeConfigValueChange<T> {
    pub before: T,
    pub after: T,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NativeWidgetConfigPatch {
    pub backend: Option<NativeConfigValueChange<NativeBackendKind>>,
    pub widget_class: Option<NativeConfigValueChange<String>>,
    pub role: Option<NativeConfigValueChange<NativeRole>>,
    pub accessibility_role: Option<NativeConfigValueChange<AccessibilityRole>>,
    pub label: Option<NativeConfigValueChange<Option<String>>>,
    pub value: Option<NativeConfigValueChange<Option<String>>>,
    pub action: Option<NativeConfigValueChange<Option<String>>>,
    pub class_name: Option<NativeConfigValueChange<Option<String>>>,
    pub placeholder: Option<NativeConfigValueChange<Option<String>>>,
    pub enabled: Option<NativeConfigValueChange<bool>>,
    pub visible: Option<NativeConfigValueChange<bool>>,
    pub required: Option<NativeConfigValueChange<bool>>,
    pub invalid: Option<NativeConfigValueChange<bool>>,
    pub selected: Option<NativeConfigValueChange<bool>>,
    pub checked: Option<NativeConfigValueChange<Option<bool>>>,
    pub expanded: Option<NativeConfigValueChange<Option<bool>>>,
    pub orientation: Option<NativeConfigValueChange<Option<Orientation>>>,
    pub min: Option<NativeConfigValueChange<Option<f64>>>,
    pub max: Option<NativeConfigValueChange<Option<f64>>>,
    pub current: Option<NativeConfigValueChange<Option<f64>>>,
    pub web_style: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
    pub portable_style: Option<NativeConfigValueChange<PortableStyle>>,
    pub events: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
    pub metadata: Option<NativeConfigValueChange<BTreeMap<String, String>>>,
}

impl NativeWidgetConfigPatch {
    pub fn between(before: &NativeWidgetConfig, after: &NativeWidgetConfig) -> Self {
        Self {
            backend: diff_value(&before.backend, &after.backend),
            widget_class: diff_value(&before.widget_class, &after.widget_class),
            role: diff_value(&before.role, &after.role),
            accessibility_role: diff_value(&before.accessibility_role, &after.accessibility_role),
            label: diff_value(&before.label, &after.label),
            value: diff_value(&before.value, &after.value),
            action: diff_value(&before.action, &after.action),
            class_name: diff_value(&before.class_name, &after.class_name),
            placeholder: diff_value(&before.placeholder, &after.placeholder),
            enabled: diff_value(&before.enabled, &after.enabled),
            visible: diff_value(&before.visible, &after.visible),
            required: diff_value(&before.required, &after.required),
            invalid: diff_value(&before.invalid, &after.invalid),
            selected: diff_value(&before.selected, &after.selected),
            checked: diff_value(&before.checked, &after.checked),
            expanded: diff_value(&before.expanded, &after.expanded),
            orientation: diff_value(&before.orientation, &after.orientation),
            min: diff_value(&before.min, &after.min),
            max: diff_value(&before.max, &after.max),
            current: diff_value(&before.current, &after.current),
            web_style: diff_value(&before.web_style, &after.web_style),
            portable_style: diff_value(&before.portable_style, &after.portable_style),
            events: diff_value(&before.events, &after.events),
            metadata: diff_value(&before.metadata, &after.metadata),
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    pub fn setters(&self) -> Vec<NativeWidgetSetter> {
        let mut setters = Vec::new();
        push_setter(
            &mut setters,
            &self.accessibility_role,
            NativeWidgetSetter::SetAccessibilityRole,
        );
        push_setter(&mut setters, &self.label, NativeWidgetSetter::SetLabel);
        push_setter(&mut setters, &self.value, NativeWidgetSetter::SetValue);
        push_setter(&mut setters, &self.action, NativeWidgetSetter::SetAction);
        push_setter(
            &mut setters,
            &self.class_name,
            NativeWidgetSetter::SetClassName,
        );
        push_setter(
            &mut setters,
            &self.placeholder,
            NativeWidgetSetter::SetPlaceholder,
        );
        push_setter(&mut setters, &self.enabled, NativeWidgetSetter::SetEnabled);
        push_setter(&mut setters, &self.visible, NativeWidgetSetter::SetVisible);
        push_setter(
            &mut setters,
            &self.required,
            NativeWidgetSetter::SetRequired,
        );
        push_setter(&mut setters, &self.invalid, NativeWidgetSetter::SetInvalid);
        push_setter(
            &mut setters,
            &self.selected,
            NativeWidgetSetter::SetSelected,
        );
        push_setter(&mut setters, &self.checked, NativeWidgetSetter::SetChecked);
        push_setter(
            &mut setters,
            &self.expanded,
            NativeWidgetSetter::SetExpanded,
        );
        push_setter(
            &mut setters,
            &self.orientation,
            NativeWidgetSetter::SetOrientation,
        );
        push_setter(&mut setters, &self.min, NativeWidgetSetter::SetMinimum);
        push_setter(&mut setters, &self.max, NativeWidgetSetter::SetMaximum);
        push_setter(&mut setters, &self.current, NativeWidgetSetter::SetCurrent);
        push_setter(
            &mut setters,
            &self.web_style,
            NativeWidgetSetter::SetWebStyle,
        );
        push_setter(
            &mut setters,
            &self.portable_style,
            NativeWidgetSetter::SetPortableStyle,
        );
        push_setter(&mut setters, &self.events, NativeWidgetSetter::SetEvents);
        push_setter(
            &mut setters,
            &self.metadata,
            NativeWidgetSetter::SetMetadata,
        );
        setters
    }
}

fn diff_value<T: Clone + PartialEq>(before: &T, after: &T) -> Option<NativeConfigValueChange<T>> {
    (before != after).then(|| NativeConfigValueChange {
        before: before.clone(),
        after: after.clone(),
    })
}

fn push_setter<T: Clone>(
    setters: &mut Vec<NativeWidgetSetter>,
    change: &Option<NativeConfigValueChange<T>>,
    setter: impl FnOnce(T) -> NativeWidgetSetter,
) {
    if let Some(change) = change {
        setters.push(setter(change.after.clone()));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "camelCase")]
pub enum NativeWidgetSetter {
    SetAccessibilityRole(AccessibilityRole),
    SetLabel(Option<String>),
    SetValue(Option<String>),
    SetAction(Option<String>),
    SetClassName(Option<String>),
    SetPlaceholder(Option<String>),
    SetEnabled(bool),
    SetVisible(bool),
    SetRequired(bool),
    SetInvalid(bool),
    SetSelected(bool),
    SetChecked(Option<bool>),
    SetExpanded(Option<bool>),
    SetOrientation(Option<Orientation>),
    SetMinimum(Option<f64>),
    SetMaximum(Option<f64>),
    SetCurrent(Option<f64>),
    SetWebStyle(BTreeMap<String, String>),
    SetPortableStyle(PortableStyle),
    SetEvents(BTreeMap<String, String>),
    SetMetadata(BTreeMap<String, String>),
}

pub fn apply_widget_setter(config: &mut NativeWidgetConfig, setter: &NativeWidgetSetter) {
    match setter {
        NativeWidgetSetter::SetAccessibilityRole(value) => config.accessibility_role = *value,
        NativeWidgetSetter::SetLabel(value) => config.label = value.clone(),
        NativeWidgetSetter::SetValue(value) => config.value = value.clone(),
        NativeWidgetSetter::SetAction(value) => config.action = value.clone(),
        NativeWidgetSetter::SetClassName(value) => config.class_name = value.clone(),
        NativeWidgetSetter::SetPlaceholder(value) => config.placeholder = value.clone(),
        NativeWidgetSetter::SetEnabled(value) => config.enabled = *value,
        NativeWidgetSetter::SetVisible(value) => config.visible = *value,
        NativeWidgetSetter::SetRequired(value) => config.required = *value,
        NativeWidgetSetter::SetInvalid(value) => config.invalid = *value,
        NativeWidgetSetter::SetSelected(value) => config.selected = *value,
        NativeWidgetSetter::SetChecked(value) => config.checked = *value,
        NativeWidgetSetter::SetExpanded(value) => config.expanded = *value,
        NativeWidgetSetter::SetOrientation(value) => config.orientation = *value,
        NativeWidgetSetter::SetMinimum(value) => config.min = *value,
        NativeWidgetSetter::SetMaximum(value) => config.max = *value,
        NativeWidgetSetter::SetCurrent(value) => config.current = *value,
        NativeWidgetSetter::SetWebStyle(value) => config.web_style = value.clone(),
        NativeWidgetSetter::SetPortableStyle(value) => config.portable_style = value.clone(),
        NativeWidgetSetter::SetEvents(value) => config.events = value.clone(),
        NativeWidgetSetter::SetMetadata(value) => config.metadata = value.clone(),
    }
}

pub fn apply_widget_setters(config: &mut NativeWidgetConfig, setters: &[NativeWidgetSetter]) {
    for setter in setters {
        apply_widget_setter(config, setter);
    }
}

pub trait PlatformAdapter: Send + Sync {
    fn kind(&self) -> NativeBackendKind;

    fn blueprint(&self, element: &NativeElement) -> NativeWidgetBlueprint {
        widget_blueprint(self.kind(), element)
    }
}

pub trait BlueprintHost {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppKitAdapter;

#[derive(Debug, Default, Clone, Copy)]
pub struct WinUiAdapter;

#[derive(Debug, Default, Clone, Copy)]
pub struct Gtk4Adapter;

impl PlatformAdapter for AppKitAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }
}

impl PlatformAdapter for WinUiAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
    }
}

impl PlatformAdapter for Gtk4Adapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlatformPlannedNode {
    pub id: HostNodeId,
    pub blueprint: NativeWidgetBlueprint,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum PlatformCommand {
    Create {
        id: HostNodeId,
        blueprint: NativeWidgetBlueprint,
    },
    Update {
        id: HostNodeId,
        blueprint: NativeWidgetBlueprint,
    },
    InsertChild {
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    },
    Remove {
        id: HostNodeId,
    },
    SetRoot {
        id: HostNodeId,
    },
}

#[derive(Debug)]
pub struct PlatformPlanningHost<A: PlatformAdapter> {
    adapter: A,
    next_id: u64,
    root: Option<HostNodeId>,
    nodes: BTreeMap<HostNodeId, PlatformPlannedNode>,
    commands: Vec<PlatformCommand>,
}

impl<A: PlatformAdapter> PlatformPlanningHost<A> {
    pub fn new(adapter: A) -> Self {
        Self {
            adapter,
            next_id: 0,
            root: None,
            nodes: BTreeMap::new(),
            commands: Vec::new(),
        }
    }

    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn node(&self, id: HostNodeId) -> Option<&PlatformPlannedNode> {
        self.nodes.get(&id)
    }

    pub fn nodes(&self) -> &BTreeMap<HostNodeId, PlatformPlannedNode> {
        &self.nodes
    }

    pub fn commands(&self) -> &[PlatformCommand] {
        &self.commands
    }

    pub fn clear_commands(&mut self) {
        self.commands.clear();
    }

    fn allocate_id(&mut self) -> HostNodeId {
        self.next_id += 1;
        HostNodeId::new(self.next_id)
    }

    fn ensure_node(&self, id: HostNodeId) -> GuiResult<()> {
        if self.nodes.contains_key(&id) {
            Ok(())
        } else {
            Err(GuiError::host(format!("unknown host node id {}", id.get())))
        }
    }
}

impl<A: PlatformAdapter> BlueprintHost for PlatformPlanningHost<A> {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint> {
        self.node(id).map(|node| &node.blueprint)
    }
}

impl<A: PlatformAdapter> NativeHost for PlatformPlanningHost<A> {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        let id = self.allocate_id();
        let blueprint = self.adapter.blueprint(element);
        self.nodes.insert(
            id,
            PlatformPlannedNode {
                id,
                blueprint: blueprint.clone(),
                children: Vec::new(),
            },
        );
        self.commands
            .push(PlatformCommand::Create { id, blueprint });
        Ok(id)
    }

    fn update(&mut self, id: HostNodeId, props: &crate::native::NativeProps) -> GuiResult<()> {
        let node = self
            .nodes
            .get_mut(&id)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", id.get())))?;
        let element = NativeElement::new(format!("host-{}", id.get()), node.blueprint.role)
            .with_props(props.clone());
        let blueprint = self.adapter.blueprint(&element);
        node.blueprint = blueprint.clone();
        self.commands
            .push(PlatformCommand::Update { id, blueprint });
        Ok(())
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.ensure_node(child)?;
        let parent_node = self
            .nodes
            .get_mut(&parent)
            .ok_or_else(|| GuiError::host(format!("unknown host node id {}", parent.get())))?;
        parent_node.children.retain(|existing| *existing != child);
        let index = index.min(parent_node.children.len());
        parent_node.children.insert(index, child);
        self.commands.push(PlatformCommand::InsertChild {
            parent,
            child,
            index,
        });
        Ok(())
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        for node in self.nodes.values_mut() {
            node.children.retain(|child| *child != id);
        }
        self.nodes.remove(&id);
        if self.root == Some(id) {
            self.root = None;
        }
        self.commands.push(PlatformCommand::Remove { id });
        Ok(())
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_node(id)?;
        self.root = Some(id);
        self.commands.push(PlatformCommand::SetRoot { id });
        Ok(())
    }
}

pub fn native_widget_name(backend: NativeBackendKind, role: NativeRole) -> &'static str {
    match backend {
        NativeBackendKind::AppKit => appkit_widget_name(role),
        NativeBackendKind::WinUI => winui_widget_name(role),
        NativeBackendKind::Gtk4 => gtk4_widget_name(role),
        NativeBackendKind::Headless => "a3s_gui::HeadlessNode",
    }
}

pub fn widget_blueprint(
    backend: NativeBackendKind,
    element: &NativeElement,
) -> NativeWidgetBlueprint {
    NativeWidgetBlueprint {
        backend,
        widget_class: native_widget_name(backend, element.role).to_string(),
        role: element.role,
        accessibility_role: accessibility_role(element.role),
        label: element.props.label.clone(),
        value: element.props.value.clone(),
        action: element
            .props
            .action
            .clone()
            .or_else(|| element.props.web.primary_action().map(str::to_string)),
        class_name: element.props.web.class_name.clone(),
        control_state: NativeControlState::from_props(&element.props),
        style: element.props.web.style.clone(),
        portable_style: PortableStyle::from_web(&element.props.web),
        events: element.props.web.events.clone(),
        metadata: element.props.metadata.clone(),
    }
}

fn appkit_widget_name(role: NativeRole) -> &'static str {
    match role {
        NativeRole::Window => "NSWindow",
        NativeRole::View => "NSView",
        NativeRole::Text => "NSTextField(label)",
        NativeRole::Button => "NSButton",
        NativeRole::TextField => "NSTextField(input)",
        NativeRole::Checkbox => "NSButton(checkbox)",
        NativeRole::Switch => "NSSwitch",
        NativeRole::RadioGroup => "NSStackView(radio-group)",
        NativeRole::Radio => "NSButton(radio)",
        NativeRole::Select | NativeRole::ComboBox => "NSComboBox",
        NativeRole::ListBox => "NSScrollView+NSStackView",
        NativeRole::ListBoxItem => "NSButton(list-row)",
        NativeRole::Dialog => "NSPanel",
        NativeRole::Popover => "NSPopover",
        NativeRole::Tabs => "NSTabView",
        NativeRole::TabList => "NSTabView",
        NativeRole::Tab => "NSTabViewItem",
        NativeRole::TabPanel => "NSView",
        NativeRole::Menu => "NSMenu",
        NativeRole::MenuItem => "NSMenuItem",
        NativeRole::Separator => "NSBox(separator)",
        NativeRole::Slider => "NSSlider",
        NativeRole::ProgressBar => "NSProgressIndicator",
        NativeRole::Toolbar => "NSStackView(toolbar)",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::renderer::Renderer;
    use crate::web::WebProps;

    #[test]
    fn appkit_blueprint_targets_native_button_not_webview() {
        let element = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new().label("Save").web(
                WebProps::new()
                    .class_name("primary")
                    .style("backgroundColor", "#663399")
                    .on_click("saveDocument"),
            ),
        );

        let blueprint = AppKitAdapter.blueprint(&element);

        assert_eq!(blueprint.widget_class, "NSButton");
        assert_eq!(blueprint.accessibility_role, AccessibilityRole::Button);
        assert_eq!(blueprint.action.as_deref(), Some("saveDocument"));
        assert_eq!(blueprint.class_name.as_deref(), Some("primary"));
        assert_eq!(
            blueprint.style.get("backgroundColor").map(String::as_str),
            Some("#663399")
        );
        assert!(blueprint.portable_style.background_color.is_some());
    }

    #[test]
    fn appkit_blueprint_targets_native_listbox_not_webview() {
        let list_box = NativeElement::new("projects", NativeRole::ListBox)
            .child(NativeElement::new("a3s", NativeRole::ListBoxItem));
        let item = &list_box.children[0];

        assert_eq!(
            AppKitAdapter.blueprint(&list_box).widget_class,
            "NSScrollView+NSStackView"
        );
        assert_eq!(
            AppKitAdapter.blueprint(item).widget_class,
            "NSButton(list-row)"
        );
    }

    #[test]
    fn toolbar_blueprint_targets_native_container_controls_not_webview() {
        let element = NativeElement::new("tools", NativeRole::Toolbar)
            .with_props(NativeProps::new().orientation(Orientation::Horizontal));

        assert_eq!(
            AppKitAdapter.blueprint(&element).widget_class,
            "NSStackView(toolbar)"
        );
        assert_eq!(
            WinUiAdapter.blueprint(&element).widget_class,
            "Microsoft.UI.Xaml.Controls.StackPanel(toolbar)"
        );
        assert_eq!(
            Gtk4Adapter.blueprint(&element).widget_class,
            "gtk::Box(toolbar)"
        );
    }

    #[test]
    fn dialog_blueprint_targets_native_dialog_controls_not_webview() {
        let element =
            NativeElement::new("preferences", NativeRole::Dialog).with_props(NativeProps::new());

        assert_eq!(AppKitAdapter.blueprint(&element).widget_class, "NSPanel");
        assert_eq!(
            WinUiAdapter.blueprint(&element).widget_class,
            "Microsoft.UI.Xaml.Controls.ContentDialog"
        );
        assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Dialog");
    }

    #[test]
    fn popover_blueprint_targets_native_overlay_controls_not_webview() {
        let element = NativeElement::new("actions-popover", NativeRole::Popover)
            .with_props(NativeProps::new());

        assert_eq!(AppKitAdapter.blueprint(&element).widget_class, "NSPopover");
        assert_eq!(
            WinUiAdapter.blueprint(&element).widget_class,
            "Microsoft.UI.Xaml.Controls.ToolTip"
        );
        assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Popover");
    }

    #[test]
    fn menu_blueprint_targets_native_menu_controls_not_webview() {
        let menu = NativeElement::new("file-menu", NativeRole::Menu)
            .child(NativeElement::new("open", NativeRole::MenuItem));
        let item = &menu.children[0];

        assert_eq!(AppKitAdapter.blueprint(&menu).widget_class, "NSMenu");
        assert_eq!(AppKitAdapter.blueprint(item).widget_class, "NSMenuItem");
        assert_eq!(
            WinUiAdapter.blueprint(&menu).widget_class,
            "Microsoft.UI.Xaml.Controls.StackPanel(menu)"
        );
        assert_eq!(
            WinUiAdapter.blueprint(item).widget_class,
            "Microsoft.UI.Xaml.Controls.Button(menu-item)"
        );
        assert_eq!(Gtk4Adapter.blueprint(&menu).widget_class, "gio::Menu");
        assert_eq!(Gtk4Adapter.blueprint(item).widget_class, "gio::MenuItem");
    }

    #[test]
    fn same_ir_targets_winui_and_gtk_native_controls() {
        let element = NativeElement::new("email", NativeRole::TextField)
            .with_props(NativeProps::new().label("Email"));

        assert_eq!(
            WinUiAdapter.blueprint(&element).widget_class,
            "Microsoft.UI.Xaml.Controls.TextBox"
        );
        assert_eq!(Gtk4Adapter.blueprint(&element).widget_class, "gtk::Entry");
    }

    #[test]
    fn blueprint_preserves_react_aria_control_state_for_native_adapters() {
        let element = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .placeholder("0-100")
                .disabled(true)
                .required(true)
                .invalid(true)
                .selected(true)
                .checked(false)
                .expanded(true)
                .orientation(Orientation::Horizontal)
                .range(Some(0.0), Some(100.0), Some(50.0)),
        );

        let blueprint = Gtk4Adapter.blueprint(&element);

        assert_eq!(
            blueprint.control_state.placeholder.as_deref(),
            Some("0-100")
        );
        assert!(blueprint.control_state.disabled);
        assert!(blueprint.control_state.required);
        assert!(blueprint.control_state.invalid);
        assert!(blueprint.control_state.selected);
        assert_eq!(blueprint.control_state.checked, Some(false));
        assert_eq!(blueprint.control_state.expanded, Some(true));
        assert_eq!(
            blueprint.control_state.orientation,
            Some(Orientation::Horizontal)
        );
        assert_eq!(blueprint.control_state.min, Some(0.0));
        assert_eq!(blueprint.control_state.max, Some(100.0));
        assert_eq!(blueprint.control_state.current, Some(50.0));
    }

    #[test]
    fn widget_config_normalizes_blueprint_for_native_setters() {
        let element = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .value("50")
                .placeholder("0-100")
                .disabled(true)
                .required(true)
                .invalid(true)
                .orientation(Orientation::Horizontal)
                .range(Some(0.0), Some(100.0), Some(50.0))
                .metadata("data-testid", "volume-slider")
                .web(
                    WebProps::new()
                        .class_name("range")
                        .style("display", "none")
                        .style("minWidth", "240")
                        .on_change("setVolume"),
                ),
        );

        let blueprint = WinUiAdapter.blueprint(&element);
        let config = blueprint.config();

        assert_eq!(config.widget_class, "Microsoft.UI.Xaml.Controls.Slider");
        assert_eq!(config.accessibility_role, AccessibilityRole::Slider);
        assert_eq!(config.label.as_deref(), Some("Volume"));
        assert_eq!(config.value.as_deref(), Some("50"));
        assert_eq!(config.placeholder.as_deref(), Some("0-100"));
        assert!(!config.enabled);
        assert!(!config.visible);
        assert!(config.required);
        assert!(config.invalid);
        assert_eq!(config.orientation, Some(Orientation::Horizontal));
        assert_eq!(config.min, Some(0.0));
        assert_eq!(config.max, Some(100.0));
        assert_eq!(config.current, Some(50.0));
        assert_eq!(config.class_name.as_deref(), Some("range"));
        assert_eq!(
            config
                .portable_style
                .min_width
                .and_then(|value| value.points()),
            Some(240.0)
        );
        assert_eq!(
            config.events.get("onChange").map(String::as_str),
            Some("setVolume")
        );
        assert_eq!(
            config.metadata.get("data-testid").map(String::as_str),
            Some("volume-slider")
        );

        let setters = config.create_setters();
        assert!(setters.contains(&NativeWidgetSetter::SetAccessibilityRole(
            AccessibilityRole::Slider
        )));
        assert!(setters.contains(&NativeWidgetSetter::SetLabel(Some("Volume".to_string()))));
        assert!(setters.contains(&NativeWidgetSetter::SetEnabled(false)));
        assert!(setters.contains(&NativeWidgetSetter::SetVisible(false)));
        assert!(setters.contains(&NativeWidgetSetter::SetPlaceholder(Some(
            "0-100".to_string()
        ))));
        assert!(setters.contains(&NativeWidgetSetter::SetMinimum(Some(0.0))));
        assert!(setters.contains(&NativeWidgetSetter::SetMaximum(Some(100.0))));
        assert!(setters.contains(&NativeWidgetSetter::SetCurrent(Some(50.0))));
    }

    #[test]
    fn widget_config_diff_reports_changed_native_setters() {
        let first = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Volume")
                .value("50")
                .range(Some(0.0), Some(100.0), Some(50.0))
                .web(
                    WebProps::new()
                        .style("display", "flex")
                        .on_change("setVolume"),
                ),
        );
        let second = NativeElement::new("volume", NativeRole::Slider).with_props(
            NativeProps::new()
                .label("Muted")
                .value("0")
                .disabled(true)
                .range(Some(0.0), Some(100.0), Some(0.0))
                .web(
                    WebProps::new()
                        .style("display", "none")
                        .on_change("setVolume"),
                ),
        );

        let before = Gtk4Adapter.blueprint(&first).config();
        let after = Gtk4Adapter.blueprint(&second).config();
        let unchanged = before.diff(&before);
        let patch = before.diff(&after);

        assert!(unchanged.is_empty());
        assert_eq!(
            patch.label.as_ref().map(|change| change.after.as_deref()),
            Some(Some("Muted"))
        );
        assert_eq!(
            patch.value.as_ref().map(|change| change.after.as_deref()),
            Some(Some("0"))
        );
        assert_eq!(
            patch.enabled.as_ref().map(|change| change.after),
            Some(false)
        );
        assert_eq!(
            patch.visible.as_ref().map(|change| change.after),
            Some(false)
        );
        assert_eq!(
            patch.current.as_ref().map(|change| change.after),
            Some(Some(0.0))
        );
        assert!(patch.min.is_none());
        assert!(patch.max.is_none());
        assert!(patch.events.is_none());

        let setters = patch.setters();
        assert!(setters.contains(&NativeWidgetSetter::SetLabel(Some("Muted".to_string()))));
        assert!(setters.contains(&NativeWidgetSetter::SetValue(Some("0".to_string()))));
        assert!(setters.contains(&NativeWidgetSetter::SetEnabled(false)));
        assert!(setters.contains(&NativeWidgetSetter::SetVisible(false)));
        assert!(setters.contains(&NativeWidgetSetter::SetCurrent(Some(0.0))));
        assert!(!setters.contains(&NativeWidgetSetter::SetMinimum(Some(0.0))));
        assert!(!setters.contains(&NativeWidgetSetter::SetMaximum(Some(100.0))));
        assert!(!setters
            .iter()
            .any(|setter| matches!(setter, NativeWidgetSetter::SetEvents(_))));
    }

    #[test]
    fn native_widget_setters_round_trip_as_json() {
        let setters = vec![
            NativeWidgetSetter::SetLabel(Some("Save".to_string())),
            NativeWidgetSetter::SetEnabled(false),
            NativeWidgetSetter::SetCurrent(Some(50.0)),
            NativeWidgetSetter::SetEvents(BTreeMap::from([(
                "onPress".to_string(),
                "saveProfile".to_string(),
            )])),
        ];

        let json = serde_json::to_string(&setters).unwrap();
        let decoded: Vec<NativeWidgetSetter> = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded, setters);
        assert!(json.contains(r#""type":"setLabel""#));
        assert!(json.contains(r#""type":"setEnabled""#));
        assert!(json.contains(r#""type":"setCurrent""#));
        assert!(json.contains(r#""onPress":"saveProfile""#));
    }

    #[test]
    fn widget_setters_replay_into_native_config() {
        let before = Gtk4Adapter
            .blueprint(
                &NativeElement::new("volume", NativeRole::Slider).with_props(
                    NativeProps::new()
                        .label("Volume")
                        .range(Some(0.0), Some(100.0), Some(50.0)),
                ),
            )
            .config();
        let after = Gtk4Adapter
            .blueprint(
                &NativeElement::new("volume", NativeRole::Slider).with_props(
                    NativeProps::new().label("Muted").disabled(true).range(
                        Some(0.0),
                        Some(100.0),
                        Some(0.0),
                    ),
                ),
            )
            .config();
        let mut replayed = before.clone();

        apply_widget_setters(&mut replayed, &before.diff(&after).setters());

        assert_eq!(replayed.label.as_deref(), Some("Muted"));
        assert!(!replayed.enabled);
        assert_eq!(replayed.current, Some(0.0));
        assert_eq!(replayed, after);
    }

    #[test]
    fn renderer_can_drive_platform_planning_host_directly() {
        let tree = NativeElement::new("root", NativeRole::View)
            .child(
                NativeElement::new("save", NativeRole::Button)
                    .with_props(NativeProps::new().label("Save").action("saveDocument")),
            )
            .child(
                NativeElement::new("email", NativeRole::TextField)
                    .with_props(NativeProps::new().label("Email")),
            );
        let mut renderer = Renderer::new();
        let mut host = PlatformPlanningHost::new(WinUiAdapter);

        let root_id = renderer.render(&tree, &mut host).unwrap();
        let root = host.node(root_id).unwrap();
        let child_widgets: Vec<_> = root
            .children
            .iter()
            .map(|id| host.node(*id).unwrap().blueprint.widget_class.as_str())
            .collect();

        assert_eq!(
            root.blueprint.widget_class,
            "Microsoft.UI.Xaml.Controls.StackPanel"
        );
        assert_eq!(
            child_widgets,
            vec![
                "Microsoft.UI.Xaml.Controls.Button",
                "Microsoft.UI.Xaml.Controls.TextBox"
            ]
        );
        assert!(host.commands().iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.Button"
        )));
    }

    #[test]
    fn platform_planning_host_updates_blueprint_on_rerender() {
        let first = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new()
                .label("Save")
                .web(WebProps::new().style("minWidth", "120").on_click("save")),
        );
        let second = NativeElement::new("save", NativeRole::Button).with_props(
            NativeProps::new().label("Saved").web(
                WebProps::new()
                    .style("minWidth", "160")
                    .on_press("saveAgain"),
            ),
        );
        let mut renderer = Renderer::new();
        let mut host = PlatformPlanningHost::new(AppKitAdapter);

        let first_id = renderer.render(&first, &mut host).unwrap();
        let second_id = renderer.render(&second, &mut host).unwrap();

        assert_eq!(first_id, second_id);
        let blueprint = &host.node(second_id).unwrap().blueprint;
        assert_eq!(blueprint.label.as_deref(), Some("Saved"));
        assert_eq!(blueprint.action.as_deref(), Some("saveAgain"));
        assert_eq!(
            blueprint
                .portable_style
                .min_width
                .and_then(|value| value.points()),
            Some(160.0)
        );
        assert!(host.commands().iter().any(|command| matches!(
            command,
            PlatformCommand::Update {
                id,
                blueprint,
            } if *id == second_id && blueprint.label.as_deref() == Some("Saved")
        )));
    }

    #[test]
    fn command_stream_records_native_remove_and_reorder() {
        let first = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("a", NativeRole::Button))
            .child(NativeElement::new("b", NativeRole::Button));
        let second = NativeElement::new("root", NativeRole::View)
            .child(NativeElement::new("b", NativeRole::Button))
            .child(NativeElement::new("c", NativeRole::Button));
        let mut renderer = Renderer::new();
        let mut host = PlatformPlanningHost::new(Gtk4Adapter);

        let root_id = renderer.render(&first, &mut host).unwrap();
        host.clear_commands();
        renderer.render(&second, &mut host).unwrap();

        assert!(host.commands().iter().any(|command| matches!(
            command,
            PlatformCommand::InsertChild {
                parent,
                index: 0,
                ..
            } if *parent == root_id
        )));
        assert!(host
            .commands()
            .iter()
            .any(|command| matches!(command, PlatformCommand::Remove { .. })));
        assert!(host.commands().iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "gtk::Button"
        )));
    }

    #[test]
    fn platform_commands_round_trip_as_native_backend_json() {
        let element = NativeElement::new("email", NativeRole::TextField).with_props(
            NativeProps::new()
                .label("Email")
                .value("a@b.c")
                .placeholder("you@example.com")
                .disabled(true)
                .required(true)
                .invalid(true)
                .range(Some(0.0), Some(100.0), Some(50.0))
                .web(
                    WebProps::new()
                        .style("minWidth", "280")
                        .attribute("data-testid", "email-input")
                        .on_change("setEmail"),
                ),
        );
        let command = PlatformCommand::Create {
            id: HostNodeId::new(42),
            blueprint: Gtk4Adapter.blueprint(&element),
        };

        let json = serde_json::to_string(&command).unwrap();
        let decoded: PlatformCommand = serde_json::from_str(&json).unwrap();

        assert_eq!(decoded, command);
        assert!(json.contains(r#""type":"create""#));
        assert!(json.contains(r#""backend":"gtk4""#));
        assert!(json.contains(r#""widgetClass":"gtk::Entry""#));
        assert!(json.contains(r#""role":"textField""#));
        assert!(json.contains(r#""accessibilityRole":"textField""#));
        assert!(json.contains(r#""controlState""#));
        assert!(json.contains(r#""placeholder":"you@example.com""#));
        assert!(json.contains(r#""disabled":true"#));
        assert!(json.contains(r#""onChange":"setEmail""#));
        let PlatformCommand::Create { blueprint, .. } = decoded else {
            unreachable!("decoded command should remain a create command");
        };
        assert_eq!(
            blueprint.control_state.placeholder.as_deref(),
            Some("you@example.com")
        );
        assert!(blueprint.control_state.disabled);
        assert!(blueprint.control_state.required);
        assert!(blueprint.control_state.invalid);
        assert_eq!(blueprint.control_state.min, Some(0.0));
        assert_eq!(blueprint.control_state.max, Some(100.0));
        assert_eq!(blueprint.control_state.current, Some(50.0));
    }
}

fn winui_widget_name(role: NativeRole) -> &'static str {
    match role {
        NativeRole::Window => "Microsoft.UI.Xaml.Window",
        NativeRole::View => "Microsoft.UI.Xaml.Controls.StackPanel",
        NativeRole::Text => "Microsoft.UI.Xaml.Controls.TextBlock",
        NativeRole::Button => "Microsoft.UI.Xaml.Controls.Button",
        NativeRole::TextField => "Microsoft.UI.Xaml.Controls.TextBox",
        NativeRole::Checkbox => "Microsoft.UI.Xaml.Controls.CheckBox",
        NativeRole::Switch => "Microsoft.UI.Xaml.Controls.ToggleSwitch",
        NativeRole::RadioGroup => "Microsoft.UI.Xaml.Controls.RadioButtons",
        NativeRole::Radio => "Microsoft.UI.Xaml.Controls.RadioButton",
        NativeRole::Select | NativeRole::ComboBox => "Microsoft.UI.Xaml.Controls.ComboBox",
        NativeRole::ListBox => "Microsoft.UI.Xaml.Controls.ListView",
        NativeRole::ListBoxItem => "Microsoft.UI.Xaml.Controls.ListViewItem",
        NativeRole::Dialog => "Microsoft.UI.Xaml.Controls.ContentDialog",
        NativeRole::Popover => "Microsoft.UI.Xaml.Controls.ToolTip",
        NativeRole::Tabs => "Microsoft.UI.Xaml.Controls.TabView",
        NativeRole::TabList => "Microsoft.UI.Xaml.Controls.TabView",
        NativeRole::Tab => "Microsoft.UI.Xaml.Controls.TabViewItem",
        NativeRole::TabPanel => "Microsoft.UI.Xaml.Controls.Grid",
        NativeRole::Menu => "Microsoft.UI.Xaml.Controls.StackPanel(menu)",
        NativeRole::MenuItem => "Microsoft.UI.Xaml.Controls.Button(menu-item)",
        NativeRole::Separator => "Microsoft.UI.Xaml.Controls.Border(separator)",
        NativeRole::Slider => "Microsoft.UI.Xaml.Controls.Slider",
        NativeRole::ProgressBar => "Microsoft.UI.Xaml.Controls.ProgressBar",
        NativeRole::Toolbar => "Microsoft.UI.Xaml.Controls.StackPanel(toolbar)",
    }
}

fn gtk4_widget_name(role: NativeRole) -> &'static str {
    match role {
        NativeRole::Window => "gtk::ApplicationWindow",
        NativeRole::View => "gtk::Box",
        NativeRole::Text => "gtk::Label",
        NativeRole::Button => "gtk::Button",
        NativeRole::TextField => "gtk::Entry",
        NativeRole::Checkbox => "gtk::CheckButton",
        NativeRole::Switch => "gtk::Switch",
        NativeRole::RadioGroup => "gtk::Box(radio-group)",
        NativeRole::Radio => "gtk::CheckButton(radio)",
        NativeRole::Select | NativeRole::ComboBox => "gtk::DropDown",
        NativeRole::ListBox => "gtk::ListBox",
        NativeRole::ListBoxItem => "gtk::ListBoxRow",
        NativeRole::Dialog => "gtk::Dialog",
        NativeRole::Popover => "gtk::Popover",
        NativeRole::Tabs => "gtk::Notebook",
        NativeRole::TabList => "gtk::Notebook",
        NativeRole::Tab => "gtk::Label(tab)",
        NativeRole::TabPanel => "gtk::Box",
        NativeRole::Menu => "gio::Menu",
        NativeRole::MenuItem => "gio::MenuItem",
        NativeRole::Separator => "gtk::Separator",
        NativeRole::Slider => "gtk::Scale",
        NativeRole::ProgressBar => "gtk::ProgressBar",
        NativeRole::Toolbar => "gtk::Box(toolbar)",
    }
}
