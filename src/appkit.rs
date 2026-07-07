use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::backend::{
    DriverCommandExecutor, HandleWidgetDriver, NativeEventSource, NativeHandleAdapter,
    NativeWidgetDriver,
};
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
#[cfg(any(test, feature = "appkit-native"))]
use crate::platform::NativeTextInputPurpose;
use crate::platform::{
    apply_widget_setters, NativeBackendKind, NativeControlState, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetSetter,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppKitWidgetKind {
    Window,
    View,
    Label,
    Button,
    TextField,
    Checkbox,
    Switch,
    RadioGroup,
    Radio,
    ComboBox,
    ListView,
    ScrollView,
    ListItem,
    Panel,
    Popover,
    Tabs,
    Tab,
    Menu,
    MenuItem,
    Separator,
    Slider,
    ProgressIndicator,
    Toolbar,
}

impl AppKitWidgetKind {
    pub fn from_widget_class(widget_class: &str) -> GuiResult<Self> {
        match widget_class {
            "NSWindow" => Ok(AppKitWidgetKind::Window),
            "NSView"
            | "NSView(document)"
            | "NSView(document-head)"
            | "NSView(document-body)"
            | "NSView(metadata)"
            | "NSView(resource-link)"
            | "NSView(style-sheet)"
            | "NSView(script)"
            | "NSView(template)"
            | "NSView(slot)"
            | "NSView(paragraph)"
            | "NSView(preformatted-text)"
            | "NSView(block-quote)"
            | "NSView(contact-address)"
            | "NSView(no-break-text)"
            | "NSView(centered-text)"
            | "NSView(font-text)"
            | "NSView(big-text)"
            | "NSView(teletype-text)"
            | "NSView(applet)"
            | "NSView(background-sound)"
            | "NSView(frame)"
            | "NSView(frameset)"
            | "NSView(noembed-fallback)"
            | "NSView(noframes-fallback)"
            | "NSView(marquee)"
            | "NSView(math)"
            | "NSView(nextid)"
            | "NSView(selected-content)"
            | "NSView(heading-group)"
            | "NSView(ruby)"
            | "NSView(ruby-text-container)"
            | "NSView(main)"
            | "NSView(navigation)"
            | "NSView(header)"
            | "NSView(footer)"
            | "NSView(article)"
            | "NSView(section)"
            | "NSView(aside)"
            | "NSView(search)"
            | "NSView(disclosure)"
            | "NSView(figure)"
            | "NSView(description-list)"
            | "NSView(description-details)"
            | "NSView(form)"
            | "NSView(fieldset)"
            | "NSView(option-group)"
            | "NSView(image-map)"
            | "NSImageView"
            | "AVPlayerView"
            | "NSView(canvas)"
            | "NSView(embedded-content)"
            | "NSView(table-section)"
            | "NSTableRowView"
            | "NSTableCellView"
            | "NSTableColumn" => Ok(AppKitWidgetKind::View),
            "NSTextField(label)"
            | "NSTextField(abbreviation)"
            | "NSTextField(citation)"
            | "NSTextField(definition)"
            | "NSTextField(data-value)"
            | "NSTextField(inserted-text)"
            | "NSTextField(deleted-text)"
            | "NSTextField(marked-text)"
            | "NSTextField(time)"
            | "NSTextField(emphasis)"
            | "NSTextField(strong-text)"
            | "NSTextField(code)"
            | "NSTextField(keyboard-input)"
            | "NSTextField(sample-output)"
            | "NSTextField(variable)"
            | "NSTextField(inline-quote)"
            | "NSTextField(subscript)"
            | "NSTextField(superscript)"
            | "NSTextField(small-text)"
            | "NSTextField(bold-text)"
            | "NSTextField(italic-text)"
            | "NSTextField(struck-text)"
            | "NSTextField(underlined-text)"
            | "NSTextField(bidi-isolate)"
            | "NSTextField(bidi-override)"
            | "NSTextField(line-break)"
            | "NSTextField(word-break-opportunity)"
            | "NSTextField(document-title)"
            | "NSTextField(heading)"
            | "NSTextField(ruby-base)"
            | "NSTextField(ruby-text)"
            | "NSTextField(ruby-parenthesis)"
            | "NSTextField(figure-caption)"
            | "NSTextField(description-term)"
            | "NSTextField(legend)"
            | "NSTextField(output)"
            | "NSTextField(table-caption)" => Ok(AppKitWidgetKind::Label),
            "NSButton"
            | "NSButton(link)"
            | "NSButton(image-map-area)"
            | "NSButton(disclosure-summary)" => Ok(AppKitWidgetKind::Button),
            "NSTextField(input)"
            | "NSTextField(textarea)"
            | "NSSearchField"
            | "NSSecureTextField" => Ok(AppKitWidgetKind::TextField),
            "NSButton(checkbox)" => Ok(AppKitWidgetKind::Checkbox),
            "NSSwitch" => Ok(AppKitWidgetKind::Switch),
            "NSStackView(radio-group)" => Ok(AppKitWidgetKind::RadioGroup),
            "NSButton(radio)" => Ok(AppKitWidgetKind::Radio),
            "NSComboBox" => Ok(AppKitWidgetKind::ComboBox),
            "NSScrollView+NSStackView" | "NSTableView" | "NSOutlineView" => {
                Ok(AppKitWidgetKind::ListView)
            }
            "NSScrollView+NSStackView(scroll)" => Ok(AppKitWidgetKind::ScrollView),
            "NSButton(list-row)" | "NSButton(outline-row)" => Ok(AppKitWidgetKind::ListItem),
            "NSPanel" => Ok(AppKitWidgetKind::Panel),
            "NSPopover" => Ok(AppKitWidgetKind::Popover),
            "NSTabView" => Ok(AppKitWidgetKind::Tabs),
            "NSTabViewItem" => Ok(AppKitWidgetKind::Tab),
            "NSMenu" => Ok(AppKitWidgetKind::Menu),
            "NSMenuItem" => Ok(AppKitWidgetKind::MenuItem),
            "NSBox(separator)" => Ok(AppKitWidgetKind::Separator),
            "NSSlider" => Ok(AppKitWidgetKind::Slider),
            "NSProgressIndicator" | "NSProgressIndicator(meter)" => {
                Ok(AppKitWidgetKind::ProgressIndicator)
            }
            "NSStackView(toolbar)" | "NSToolbar" => Ok(AppKitWidgetKind::Toolbar),
            other => Err(GuiError::host(format!(
                "unsupported AppKit widget class {other}"
            ))),
        }
    }
}

#[cfg(any(test, feature = "appkit-native"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AppKitTextInputTrait {
    Default,
    No,
    Yes,
}

#[cfg(any(test, feature = "appkit-native"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AppKitTextInputHints {
    pub automatic_text_completion_enabled: Option<bool>,
    pub spell_checking: AppKitTextInputTrait,
    pub autocorrection: AppKitTextInputTrait,
    pub text_replacement: AppKitTextInputTrait,
    pub text_completion: AppKitTextInputTrait,
    pub inline_prediction: AppKitTextInputTrait,
    pub character_picker_enabled: bool,
}

#[cfg(any(test, feature = "appkit-native"))]
pub(crate) fn appkit_text_input_hints(config: &NativeWidgetConfig) -> AppKitTextInputHints {
    let hints = config.text_input_hints();
    let constrained = appkit_constrained_text_input(config);
    let completion = appkit_completion_trait(config, constrained);
    let correction = appkit_correction_trait(config, constrained);

    AppKitTextInputHints {
        automatic_text_completion_enabled: appkit_automatic_text_completion_enabled(
            config, completion,
        ),
        spell_checking: hints
            .spellcheck
            .map(appkit_bool_trait)
            .unwrap_or_else(|| appkit_default_spell_checking(config, constrained)),
        autocorrection: correction,
        text_replacement: correction,
        text_completion: completion,
        inline_prediction: completion,
        character_picker_enabled: hints.emoji.unwrap_or(true) && !hints.private,
    }
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_constrained_text_input(config: &NativeWidgetConfig) -> bool {
    normalized_token(config.input_mode.as_deref()).as_deref() == Some("none")
        || matches!(
            config.text_input_purpose(),
            NativeTextInputPurpose::Digits
                | NativeTextInputPurpose::Number
                | NativeTextInputPurpose::Phone
                | NativeTextInputPurpose::Url
                | NativeTextInputPurpose::Email
                | NativeTextInputPurpose::Password
                | NativeTextInputPurpose::Pin
                | NativeTextInputPurpose::Terminal
        )
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_default_spell_checking(
    config: &NativeWidgetConfig,
    constrained: bool,
) -> AppKitTextInputTrait {
    if constrained || config.text_input_hints().private {
        AppKitTextInputTrait::No
    } else {
        AppKitTextInputTrait::Default
    }
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_correction_trait(config: &NativeWidgetConfig, constrained: bool) -> AppKitTextInputTrait {
    if constrained || config.text_input_hints().private {
        return AppKitTextInputTrait::No;
    }
    match normalized_token(config.auto_correct.as_deref()).as_deref() {
        Some("on" | "true") => AppKitTextInputTrait::Yes,
        Some("off" | "false") => AppKitTextInputTrait::No,
        _ => AppKitTextInputTrait::Default,
    }
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_completion_trait(config: &NativeWidgetConfig, constrained: bool) -> AppKitTextInputTrait {
    if constrained || config.text_input_hints().private {
        return AppKitTextInputTrait::No;
    }
    match normalized_token(config.autocomplete.as_deref()).as_deref() {
        Some("on") => AppKitTextInputTrait::Yes,
        Some("off") => AppKitTextInputTrait::No,
        _ => AppKitTextInputTrait::Default,
    }
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_automatic_text_completion_enabled(
    config: &NativeWidgetConfig,
    completion: AppKitTextInputTrait,
) -> Option<bool> {
    match completion {
        AppKitTextInputTrait::Yes => Some(true),
        AppKitTextInputTrait::No => Some(false),
        AppKitTextInputTrait::Default => None,
    }
    .or_else(|| {
        (normalized_token(config.input_mode.as_deref()).as_deref() == Some("none")).then_some(false)
    })
}

#[cfg(any(test, feature = "appkit-native"))]
fn appkit_bool_trait(value: bool) -> AppKitTextInputTrait {
    if value {
        AppKitTextInputTrait::Yes
    } else {
        AppKitTextInputTrait::No
    }
}

#[cfg(any(test, feature = "appkit-native"))]
fn normalized_token(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppKitNativeObject {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct AppKitWidgetDriver {
    root: Option<HostNodeId>,
    objects: BTreeMap<HostNodeId, AppKitNativeObject>,
    events: Vec<NativeEvent>,
}

pub type AppKitCommandExecutor = DriverCommandExecutor<AppKitWidgetDriver>;

#[derive(Debug, Clone)]
pub struct AppKitNativeHandle {
    state: Rc<RefCell<AppKitNativeHandleState>>,
}

impl AppKitNativeHandle {
    pub fn state(&self) -> Ref<'_, AppKitNativeHandleState> {
        self.state.borrow()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppKitNativeHandleState {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub config: NativeWidgetConfig,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub applied_setters: Vec<NativeWidgetSetter>,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct AppKitHandleAdapter;

pub type AppKitHandleDriver = HandleWidgetDriver<AppKitHandleAdapter>;
pub type AppKitHandleCommandExecutor = DriverCommandExecutor<AppKitHandleDriver>;

impl AppKitWidgetDriver {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn object(&self, id: HostNodeId) -> Option<&AppKitNativeObject> {
        self.objects.get(&id)
    }

    pub fn objects(&self) -> &BTreeMap<HostNodeId, AppKitNativeObject> {
        &self.objects
    }

    pub fn push_native_event(&mut self, event: NativeEvent) {
        self.events.push(event);
    }

    pub fn queued_native_events(&self) -> &[NativeEvent] {
        &self.events
    }

    fn ensure_object(&self, id: HostNodeId) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            Ok(())
        } else {
            Err(GuiError::host(format!(
                "AppKit object {} does not exist",
                id.get()
            )))
        }
    }

    fn subtree_contains(&self, root: HostNodeId, target: HostNodeId) -> bool {
        let Some(root) = self.objects.get(&root) else {
            return false;
        };
        let mut stack = root.children.clone();
        let mut visited = BTreeSet::new();

        while let Some(id) = stack.pop() {
            if id == target {
                return true;
            }
            if !visited.insert(id) {
                continue;
            }
            if let Some(object) = self.objects.get(&id) {
                stack.extend(object.children.iter().copied());
            }
        }

        false
    }

    fn subtree_ids(&self, root: HostNodeId) -> BTreeSet<HostNodeId> {
        let mut ids = BTreeSet::new();
        let mut stack = vec![root];

        while let Some(id) = stack.pop() {
            if !ids.insert(id) {
                continue;
            }
            if let Some(object) = self.objects.get(&id) {
                stack.extend(object.children.iter().copied());
            }
        }

        ids
    }
}

impl NativeEventSource for AppKitWidgetDriver {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events)
    }
}

impl NativeHandleAdapter for AppKitHandleAdapter {
    type Handle = AppKitNativeHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }

    fn create_handle(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<Self::Handle> {
        let config = blueprint.config();
        Ok(AppKitNativeHandle {
            state: Rc::new(RefCell::new(AppKitNativeHandleState {
                id,
                kind: AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
                label: config.label.clone(),
                value: config.value.clone(),
                action: config.action.clone(),
                applied_setters: config.create_setters(),
                config,
                control_state: blueprint.control_state.clone(),
                children: Vec::new(),
            })),
        })
    }

    fn update_handle(
        &mut self,
        _id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let mut state = handle.state.borrow_mut();
        state.kind = AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        state.config = blueprint.config();
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        let setters = state.config.create_setters();
        state.applied_setters.extend(setters);
        state.control_state = blueprint.control_state.clone();
        Ok(())
    }

    fn update_handle_config(
        &mut self,
        _id: HostNodeId,
        handle: &Self::Handle,
        blueprint: &NativeWidgetBlueprint,
        patch: &NativeWidgetConfigPatch,
    ) -> GuiResult<()> {
        let mut state = handle.state.borrow_mut();
        state.kind = AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        let setters = patch.setters();
        apply_widget_setters(&mut state.config, &setters);
        state.label = state.config.label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        state.control_state = blueprint.control_state.clone();
        state.applied_setters.extend(setters);
        Ok(())
    }

    fn insert_child_handle(
        &mut self,
        _parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        _child_handle: &Self::Handle,
        index: usize,
    ) -> GuiResult<()> {
        let mut parent = parent_handle.state.borrow_mut();
        parent.children.retain(|existing| *existing != child);
        let index = index.min(parent.children.len());
        parent.children.insert(index, child);
        Ok(())
    }

    fn remove_child_handle(
        &mut self,
        _parent: HostNodeId,
        parent_handle: &Self::Handle,
        child: HostNodeId,
        _child_handle: &Self::Handle,
    ) -> GuiResult<()> {
        parent_handle
            .state
            .borrow_mut()
            .children
            .retain(|existing| *existing != child);
        Ok(())
    }

    fn remove_handle(&mut self, _id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        handle.state.borrow_mut().children.clear();
        Ok(())
    }

    fn set_root_handle(&mut self, _id: HostNodeId, _handle: &Self::Handle) -> GuiResult<()> {
        Ok(())
    }
}

impl NativeWidgetDriver for AppKitWidgetDriver {
    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }

    fn create_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            return Err(GuiError::host(format!(
                "AppKit object {} already exists",
                id.get()
            )));
        }
        self.objects.insert(
            id,
            AppKitNativeObject {
                id,
                kind: AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
                label: blueprint.label.clone(),
                value: blueprint.value.clone(),
                action: blueprint.action.clone(),
                control_state: blueprint.control_state.clone(),
                children: Vec::new(),
            },
        );
        Ok(())
    }

    fn update_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        let object = self
            .objects
            .get_mut(&id)
            .ok_or_else(|| GuiError::host(format!("AppKit object {} missing", id.get())))?;
        object.kind = AppKitWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
        object.label = blueprint.label.clone();
        object.value = blueprint.value.clone();
        object.action = blueprint.action.clone();
        object.control_state = blueprint.control_state.clone();
        Ok(())
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.ensure_object(parent)?;
        self.ensure_object(child)?;
        if parent == child {
            return Err(GuiError::host(format!(
                "cannot insert AppKit object {} into itself",
                child.get()
            )));
        }
        if self.subtree_contains(child, parent) {
            return Err(GuiError::host(format!(
                "inserting AppKit object {} under {} would create a cycle",
                child.get(),
                parent.get()
            )));
        }

        for object in self.objects.values_mut() {
            object.children.retain(|existing| *existing != child);
        }
        let parent_object = self.objects.get_mut(&parent).ok_or_else(|| {
            GuiError::host(format!("AppKit parent object {} missing", parent.get()))
        })?;
        let index = index.min(parent_object.children.len());
        parent_object.children.insert(index, child);
        Ok(())
    }

    fn remove_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        let removed_ids = self.subtree_ids(id);
        for object in self.objects.values_mut() {
            object.children.retain(|child| !removed_ids.contains(child));
        }
        for removed_id in &removed_ids {
            self.objects.remove(removed_id);
        }
        if self
            .root
            .map(|root| removed_ids.contains(&root))
            .unwrap_or(false)
        {
            self.root = None;
        }
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.root = Some(id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::CommandExecutingHost;
    use crate::compiler::CompiledRsxNode;
    use crate::geometry::Orientation;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{AppKitAdapter, PlatformAdapter};
    use crate::runtime::GuiRuntime;
    use crate::style::{OverflowMode, StyleLength};

    #[test]
    fn appkit_text_input_hints_disable_completion_for_structured_fields() {
        let config = AppKitAdapter
            .blueprint(
                &NativeElement::new("field", NativeRole::TextField).with_props(
                    NativeProps::new()
                        .input_type("email")
                        .autocomplete("on")
                        .spell_check(Some(true)),
                ),
            )
            .config();

        assert_eq!(
            appkit_text_input_hints(&config),
            AppKitTextInputHints {
                automatic_text_completion_enabled: Some(false),
                spell_checking: AppKitTextInputTrait::Yes,
                autocorrection: AppKitTextInputTrait::No,
                text_replacement: AppKitTextInputTrait::No,
                text_completion: AppKitTextInputTrait::No,
                inline_prediction: AppKitTextInputTrait::No,
                character_picker_enabled: true,
            }
        );
    }

    #[test]
    fn appkit_text_input_hints_track_web_completion_and_keyboard_hints() {
        let config = AppKitAdapter
            .blueprint(
                &NativeElement::new("field", NativeRole::TextField).with_props(
                    NativeProps::new()
                        .autocomplete("on")
                        .auto_correct("off")
                        .input_mode("none"),
                ),
            )
            .config();

        assert_eq!(
            appkit_text_input_hints(&config),
            AppKitTextInputHints {
                automatic_text_completion_enabled: Some(false),
                spell_checking: AppKitTextInputTrait::No,
                autocorrection: AppKitTextInputTrait::No,
                text_replacement: AppKitTextInputTrait::No,
                text_completion: AppKitTextInputTrait::No,
                inline_prediction: AppKitTextInputTrait::No,
                character_picker_enabled: false,
            }
        );

        let config = AppKitAdapter
            .blueprint(
                &NativeElement::new("field", NativeRole::TextField)
                    .with_props(NativeProps::new().autocomplete("on")),
            )
            .config();

        assert_eq!(
            appkit_text_input_hints(&config).text_completion,
            AppKitTextInputTrait::Yes
        );
    }

    #[test]
    fn appkit_widget_driver_reparents_children_and_removes_subtrees() {
        let mut driver = AppKitWidgetDriver::default();
        let root = HostNodeId::new(1);
        let child = HostNodeId::new(2);
        let grandchild = HostNodeId::new(3);
        let second = HostNodeId::new(4);
        let container = AppKitAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = AppKitAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

        driver.create_widget(root, &container).unwrap();
        driver.create_widget(child, &container).unwrap();
        driver.create_widget(grandchild, &button).unwrap();
        driver.create_widget(second, &container).unwrap();
        driver.insert_child(root, child, 0).unwrap();
        driver.insert_child(child, grandchild, 0).unwrap();
        driver.insert_child(second, child, 0).unwrap();

        assert!(driver.object(root).unwrap().children.is_empty());
        assert_eq!(driver.object(second).unwrap().children, vec![child]);
        let error = driver.insert_child(child, second, 0).unwrap_err();
        assert!(error.to_string().contains("would create a cycle"));

        driver.set_root_widget(second).unwrap();
        driver.remove_widget(second).unwrap();

        assert!(driver.root().is_none());
        assert!(driver.object(root).is_some());
        assert!(driver.object(second).is_none());
        assert!(driver.object(child).is_none());
        assert!(driver.object(grandchild).is_none());
        assert_eq!(driver.objects().len(), 1);
    }

    #[test]
    fn appkit_handle_adapter_clears_previous_parent_on_reparent() {
        let mut driver = AppKitHandleDriver::default();
        let first = HostNodeId::new(1);
        let second = HostNodeId::new(2);
        let child = HostNodeId::new(3);
        let container = AppKitAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = AppKitAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

        driver.create_widget(first, &container).unwrap();
        driver.create_widget(second, &container).unwrap();
        driver.create_widget(child, &button).unwrap();
        driver.insert_child(first, child, 0).unwrap();
        driver.insert_child(second, child, 0).unwrap();

        assert_eq!(driver.children(first), Some([].as_slice()));
        assert_eq!(driver.children(second), Some([child].as_slice()));
        assert!(driver.handle(first).unwrap().state().children.is_empty());
        assert_eq!(driver.handle(second).unwrap().state().children, vec![child]);
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveDocument"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::Button);
        assert_eq!(object.label.as_deref(), Some("Save"));
        assert_eq!(object.action.as_deref(), Some("saveDocument"));
        assert!(object.control_state.disabled);
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_listbox_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "projects",
              "tag": "ListBox",
              "props": {},
              "children": [
                {
                  "kind": "element",
                  "key": "a3s",
                  "tag": "ListBoxItem",
                  "props": {"value": "a3s", "isSelected": true},
                  "children": [{"kind": "text", "key": "a3s-label", "value": "A3S"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let item = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::ListView);
        assert_eq!(item.kind, AppKitWidgetKind::ListItem);
        assert_eq!(item.label.as_deref(), Some("A3S"));
        assert_eq!(item.value.as_deref(), Some("a3s"));
        assert!(item.control_state.selected);
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_toolbar_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "tools",
              "tag": "Toolbar",
              "props": {"aria-orientation": "horizontal"},
              "children": [
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveDocument"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::Toolbar);
        assert_eq!(
            object.control_state.orientation,
            Some(crate::geometry::Orientation::Horizontal)
        );
        assert_eq!(child.kind, AppKitWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("saveDocument"));
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_dialog_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "preferences",
              "tag": "Dialog",
              "props": {"aria-label": "Preferences"},
              "children": [
                {
                  "kind": "element",
                  "key": "close",
                  "tag": "Button",
                  "props": {"events": {"onPress": "closePreferences"}},
                  "children": [{"kind": "text", "key": "close-text", "value": "Close"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::Panel);
        assert_eq!(object.label.as_deref(), Some("Preferences"));
        assert_eq!(child.kind, AppKitWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("closePreferences"));
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_popover_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "actions-popover",
              "tag": "Popover",
              "props": {"aria-label": "Actions"},
              "children": [
                {
                  "kind": "element",
                  "key": "archive",
                  "tag": "Button",
                  "props": {"events": {"onPress": "archiveItem"}},
                  "children": [{"kind": "text", "key": "archive-text", "value": "Archive"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::Popover);
        assert_eq!(object.label.as_deref(), Some("Actions"));
        assert_eq!(child.kind, AppKitWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("archiveItem"));
    }

    #[test]
    fn appkit_executor_consumes_compiled_semantic_ui_menu_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "file-menu",
              "tag": "Menu",
              "children": [
                {
                  "kind": "element",
                  "key": "open",
                  "tag": "MenuItem",
                  "props": {"value": "open", "events": {"onPress": "openFile"}},
                  "children": [{"kind": "text", "key": "open-text", "value": "Open"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let item = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, AppKitWidgetKind::Menu);
        assert_eq!(item.kind, AppKitWidgetKind::MenuItem);
        assert_eq!(item.label.as_deref(), Some("Open"));
        assert_eq!(item.value.as_deref(), Some("open"));
        assert_eq!(item.action.as_deref(), Some("openFile"));
    }

    #[test]
    fn appkit_handle_adapter_stores_thread_bound_native_handles() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"isDisabled": true, "events": {"onPress": "saveDocument"}},
              "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitHandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();

        assert_eq!(state.kind, AppKitWidgetKind::Button);
        assert_eq!(state.label.as_deref(), Some("Save"));
        assert_eq!(state.action.as_deref(), Some("saveDocument"));
        assert!(state.control_state.disabled);
        assert!(!state.config.enabled);
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetEnabled(false)));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetLabel(Some("Save".to_string()))));
    }

    #[test]
    fn appkit_handle_adapter_clears_removed_textarea_sizing_on_rerender() {
        let mut driver = AppKitHandleDriver::default();
        let id = HostNodeId::new(1);
        let limited = AppKitAdapter.blueprint(
            &NativeElement::new("notes", NativeRole::TextField).with_props(
                NativeProps::new()
                    .metadata(crate::html::HTML_TAG_METADATA_KEY, "textarea")
                    .rows(Some(6))
                    .cols(Some(48)),
            ),
        );
        let unlimited = AppKitAdapter.blueprint(
            &NativeElement::new("notes", NativeRole::TextField).with_props(
                NativeProps::new().metadata(crate::html::HTML_TAG_METADATA_KEY, "textarea"),
            ),
        );

        driver.create_widget(id, &limited).unwrap();
        let initial_setter_count = {
            let handle = driver.handle(id).unwrap();
            let state = handle.state();
            assert_eq!(state.config.rows, Some(6));
            assert_eq!(state.config.cols, Some(48));
            state.applied_setters.len()
        };

        driver.update_widget(id, &unlimited).unwrap();

        let handle = driver.handle(id).unwrap();
        let state = handle.state();
        let update_setters = &state.applied_setters[initial_setter_count..];

        assert_eq!(state.config.rows, None);
        assert_eq!(state.config.cols, None);
        assert!(update_setters.contains(&NativeWidgetSetter::SetRows(None)));
        assert!(update_setters.contains(&NativeWidgetSetter::SetCols(None)));
    }

    #[test]
    fn appkit_scroll_handle_adapter_applies_rerender_style_setters() {
        let first: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "shell",
              "tag": "Toolbar",
              "props": {
                "orientation": "vertical",
                "style": {"overflowY": "auto", "gap": 8, "inlineSize": 320}
              },
              "children": [{"kind": "text", "key": "summary", "value": "Ready"}]
            }
            "#,
        )
        .unwrap();
        let second: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "shell",
              "tag": "Toolbar",
              "props": {
                "orientation": "horizontal",
                "style": {"overflowX": "scroll", "overflowY": "auto", "gap": 12, "inlineSize": 420}
              },
              "children": [{"kind": "text", "key": "summary", "value": "Ready"}]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(AppKitAdapter, AppKitHandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&first).unwrap();
        let initial_setter_count = {
            let handle = runtime.host().executor().driver().handle(root_id).unwrap();
            let state = handle.state();
            assert_eq!(state.kind, AppKitWidgetKind::ScrollView);
            state.applied_setters.len()
        };

        runtime.render_compiled(&second).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();
        let update_setters = &state.applied_setters[initial_setter_count..];

        assert_eq!(state.kind, AppKitWidgetKind::ScrollView);
        assert!(
            update_setters.contains(&NativeWidgetSetter::SetOrientation(Some(
                Orientation::Horizontal
            )))
        );
        assert!(update_setters.iter().any(|setter| matches!(
            setter,
            NativeWidgetSetter::SetPortableStyle(style)
                if style.overflow_x == Some(OverflowMode::Scroll)
                    && style.gap.as_ref().and_then(StyleLength::points) == Some(12.0)
        )));
    }
}
