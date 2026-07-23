use std::cell::{Cell, Ref, RefCell};
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::backend::{
    DriverCommandExecutor, HandleWidgetDriver, NativeEventSource, NativeHandleAdapter,
    NativeWidgetDriver,
};
use crate::error::{GuiError, GuiResult};
use crate::event::NativeEvent;
use crate::host::HostNodeId;
use crate::overlay_position::OverlayPositionRequest;
#[cfg(any(test, feature = "appkit-native"))]
use crate::platform::NativeTextInputPurpose;
use crate::platform::{
    apply_widget_setters, push_widget_setter_history, NativeBackendKind, NativeControlState,
    NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetKind,
    NativeWidgetSetter, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
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
    pub fn from_widget_kind(kind: NativeWidgetKind) -> Self {
        match kind {
            NativeWidgetKind::Window => Self::Window,
            NativeWidgetKind::Container(_) | NativeWidgetKind::Image | NativeWidgetKind::Media => {
                Self::View
            }
            NativeWidgetKind::ScrollContainer => Self::ScrollView,
            NativeWidgetKind::Label => Self::Label,
            NativeWidgetKind::Button => Self::Button,
            NativeWidgetKind::TextInput(_) => Self::TextField,
            NativeWidgetKind::Checkbox => Self::Checkbox,
            NativeWidgetKind::Switch => Self::Switch,
            NativeWidgetKind::RadioGroup => Self::RadioGroup,
            NativeWidgetKind::Radio => Self::Radio,
            NativeWidgetKind::ComboBox => Self::ComboBox,
            NativeWidgetKind::List | NativeWidgetKind::Tree | NativeWidgetKind::Table => {
                Self::ListView
            }
            NativeWidgetKind::ListItem | NativeWidgetKind::TreeItem => Self::ListItem,
            NativeWidgetKind::Dialog => Self::Panel,
            NativeWidgetKind::Popover => Self::Popover,
            NativeWidgetKind::Tabs => Self::Tabs,
            NativeWidgetKind::Tab => Self::Tab,
            NativeWidgetKind::Menu => Self::Menu,
            NativeWidgetKind::MenuItem => Self::MenuItem,
            NativeWidgetKind::Separator => Self::Separator,
            NativeWidgetKind::Slider => Self::Slider,
            NativeWidgetKind::Progress => Self::ProgressIndicator,
            NativeWidgetKind::Toolbar => Self::Toolbar,
        }
    }

    /// Legacy class-name compatibility. Runtime drivers use `from_widget_kind`.
    #[deprecated(note = "use AppKitWidgetKind::from_widget_kind with typed NativeWidgetKind")]
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

#[derive(Clone, PartialEq)]
pub struct AppKitNativeObject {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub label: Option<String>,
    pub accessibility_label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

impl std::fmt::Debug for AppKitNativeObject {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AppKitNativeObject")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("label", &self.label)
            .field("accessibility_label", &self.accessibility_label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct AppKitWidgetDriver {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
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

    pub fn take_applied_setters(&self) -> Vec<NativeWidgetSetter> {
        std::mem::take(&mut self.state.borrow_mut().applied_setters)
    }
}

#[derive(Clone, PartialEq)]
pub struct AppKitNativeHandleState {
    pub id: HostNodeId,
    pub kind: AppKitWidgetKind,
    pub config: NativeWidgetConfig,
    pub label: Option<String>,
    pub accessibility_label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub applied_setters: Vec<NativeWidgetSetter>,
    pub children: Vec<HostNodeId>,
}

impl std::fmt::Debug for AppKitNativeHandleState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AppKitNativeHandleState")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("config", &self.config)
            .field("label", &self.label)
            .field("accessibility_label", &self.accessibility_label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("applied_setters", &self.applied_setters)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct AppKitHandleAdapter {
    focused: Rc<Cell<Option<HostNodeId>>>,
    overlay_positions: Rc<RefCell<BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>>>,
}

impl AppKitHandleAdapter {
    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused.get()
    }

    pub fn overlay_positions(
        &self,
    ) -> Ref<'_, BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>> {
        self.overlay_positions.borrow()
    }
}

pub type AppKitHandleDriver = HandleWidgetDriver<AppKitHandleAdapter>;
pub type AppKitHandleCommandExecutor = DriverCommandExecutor<AppKitHandleDriver>;

impl AppKitWidgetDriver {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
    }

    pub fn overlay_positions(&self) -> &BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)> {
        &self.overlay_positions
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
        let setters = config.create_setters();
        let mut applied_setters = Vec::new();
        push_widget_setter_history(
            &mut applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
        Ok(AppKitNativeHandle {
            state: Rc::new(RefCell::new(AppKitNativeHandleState {
                id,
                kind: AppKitWidgetKind::from_widget_kind(blueprint.widget_kind),
                label: config.label.clone(),
                accessibility_label: config.accessibility_label.clone(),
                value: config.value.clone(),
                action: config.action.clone(),
                applied_setters,
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
        state.kind = AppKitWidgetKind::from_widget_kind(blueprint.widget_kind);
        state.config = blueprint.config();
        state.label = state.config.label.clone();
        state.accessibility_label = state.config.accessibility_label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        let setters = state.config.create_setters();
        push_widget_setter_history(
            &mut state.applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
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
        state.kind = AppKitWidgetKind::from_widget_kind(blueprint.widget_kind);
        let setters = patch.setters();
        apply_widget_setters(&mut state.config, &setters);
        state.label = state.config.label.clone();
        state.accessibility_label = state.config.accessibility_label.clone();
        state.value = state.config.value.clone();
        state.action = state.config.action.clone();
        state.control_state = blueprint.control_state.clone();
        push_widget_setter_history(
            &mut state.applied_setters,
            &setters,
            blueprint.effective_value_sensitivity(),
            DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
        );
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

    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        handle.state.borrow_mut().children.clear();
        if self.focused.get() == Some(id) {
            self.focused.set(None);
        }
        self.overlay_positions
            .borrow_mut()
            .retain(|overlay, (anchor, _)| *overlay != id && *anchor != id);
        Ok(())
    }

    fn set_root_handle(&mut self, _id: HostNodeId, _handle: &Self::Handle) -> GuiResult<()> {
        Ok(())
    }

    fn request_focus_handle(&mut self, id: HostNodeId, handle: &Self::Handle) -> GuiResult<()> {
        if handle.state.borrow().id != id {
            return Err(GuiError::host(format!(
                "AppKit handle id does not match focus target {}",
                id.get()
            )));
        }
        self.focused.set(Some(id));
        Ok(())
    }

    fn position_overlay_handle(
        &mut self,
        overlay: HostNodeId,
        overlay_handle: &Self::Handle,
        anchor: HostNodeId,
        anchor_handle: &Self::Handle,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        if overlay_handle.state.borrow().id != overlay || anchor_handle.state.borrow().id != anchor
        {
            return Err(GuiError::host(
                "AppKit overlay or anchor handle id mismatch",
            ));
        }
        if overlay_handle.state.borrow().kind != AppKitWidgetKind::Popover {
            return Err(GuiError::host(format!(
                "AppKit object {} is not an NSPopover",
                overlay.get()
            )));
        }
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        self.overlay_positions
            .borrow_mut()
            .insert(overlay, (anchor, request));
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
                kind: AppKitWidgetKind::from_widget_kind(blueprint.widget_kind),
                label: blueprint.label.clone(),
                accessibility_label: blueprint.accessibility_label.clone(),
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
        object.kind = AppKitWidgetKind::from_widget_kind(blueprint.widget_kind);
        object.label = blueprint.label.clone();
        object.accessibility_label = blueprint.accessibility_label.clone();
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
        self.overlay_positions.retain(|overlay, (anchor, _)| {
            !removed_ids.contains(overlay) && !removed_ids.contains(anchor)
        });
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
        if self
            .focused
            .is_some_and(|focused| removed_ids.contains(&focused))
        {
            self.focused = None;
        }
        Ok(())
    }

    fn set_root_widget(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.root = Some(id);
        Ok(())
    }

    fn request_focus(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.ensure_object(id)?;
        self.focused = Some(id);
        Ok(())
    }

    fn position_overlay(
        &mut self,
        overlay: HostNodeId,
        anchor: HostNodeId,
        request: OverlayPositionRequest,
    ) -> GuiResult<()> {
        self.ensure_object(overlay)?;
        self.ensure_object(anchor)?;
        if self.objects.get(&overlay).map(|object| object.kind) != Some(AppKitWidgetKind::Popover) {
            return Err(GuiError::host(format!(
                "AppKit object {} is not an NSPopover",
                overlay.get()
            )));
        }
        let request = OverlayPositionRequest::new(request.options, request.direction)?;
        self.overlay_positions.insert(overlay, (anchor, request));
        Ok(())
    }
}

#[cfg(test)]
mod tests;
