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
#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
use crate::platform::NativeTextInputPurpose;
use crate::platform::{
    apply_widget_setters, push_widget_setter_history, NativeBackendKind, NativeContainerKind,
    NativeControlState, NativeWidgetBlueprint, NativeWidgetConfig, NativeWidgetConfigPatch,
    NativeWidgetKind, NativeWidgetSetter, DEFAULT_NATIVE_SETTER_HISTORY_LIMIT,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WinUiWidgetKind {
    Window,
    StackPanel,
    TextBlock,
    Button,
    TextBox,
    CheckBox,
    ToggleSwitch,
    RadioButtons,
    RadioButton,
    ComboBox,
    ListView,
    ListViewItem,
    ScrollViewer,
    ContentDialog,
    ToolTip,
    TabView,
    TabViewItem,
    Grid,
    MenuPanel,
    MenuItemButton,
    SelectorItem,
    Separator,
    Slider,
    ProgressBar,
    CommandBar,
}

impl WinUiWidgetKind {
    pub fn from_widget_kind(kind: NativeWidgetKind) -> Self {
        match kind {
            NativeWidgetKind::Window => Self::Window,
            NativeWidgetKind::Container(NativeContainerKind::Linear) => Self::StackPanel,
            NativeWidgetKind::Container(_) | NativeWidgetKind::Media | NativeWidgetKind::Table => {
                Self::Grid
            }
            NativeWidgetKind::ScrollContainer => Self::ScrollViewer,
            NativeWidgetKind::Label | NativeWidgetKind::Image => Self::TextBlock,
            NativeWidgetKind::Button => Self::Button,
            NativeWidgetKind::TextInput(_) => Self::TextBox,
            NativeWidgetKind::Checkbox => Self::CheckBox,
            NativeWidgetKind::Switch => Self::ToggleSwitch,
            NativeWidgetKind::RadioGroup => Self::RadioButtons,
            NativeWidgetKind::Radio => Self::RadioButton,
            NativeWidgetKind::ComboBox => Self::ComboBox,
            NativeWidgetKind::List => Self::ListView,
            NativeWidgetKind::ListItem => Self::ListViewItem,
            NativeWidgetKind::Tree => Self::ListView,
            NativeWidgetKind::TreeItem => Self::SelectorItem,
            NativeWidgetKind::Dialog => Self::ContentDialog,
            NativeWidgetKind::Popover => Self::ToolTip,
            NativeWidgetKind::Tabs => Self::TabView,
            NativeWidgetKind::Tab => Self::TabViewItem,
            NativeWidgetKind::Menu => Self::MenuPanel,
            NativeWidgetKind::MenuItem => Self::MenuItemButton,
            NativeWidgetKind::Separator => Self::Separator,
            NativeWidgetKind::Slider => Self::Slider,
            NativeWidgetKind::Progress => Self::ProgressBar,
            NativeWidgetKind::Toolbar => Self::CommandBar,
        }
    }

    /// Legacy class-name compatibility. Runtime drivers use `from_widget_kind`.
    #[deprecated(note = "use WinUiWidgetKind::from_widget_kind with typed NativeWidgetKind")]
    pub fn from_widget_class(widget_class: &str) -> GuiResult<Self> {
        match widget_class {
            "Microsoft.UI.Xaml.Window" => Ok(WinUiWidgetKind::Window),
            "Microsoft.UI.Xaml.Controls.StackPanel"
            | "Microsoft.UI.Xaml.Controls.StackPanel(document)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(document-head)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(document-body)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(metadata)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(resource-link)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(style-sheet)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(script)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(template)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(slot)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(paragraph)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(preformatted-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(block-quote)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(contact-address)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(no-break-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(centered-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(font-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(big-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(teletype-text)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(applet)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(background-sound)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(frame)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(frameset)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(noembed-fallback)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(noframes-fallback)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(marquee)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(math)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(nextid)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(selected-content)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(heading-group)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(ruby)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(ruby-text-container)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(main)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(navigation)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(header)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(footer)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(article)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(section)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(aside)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(search)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(disclosure)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(figure)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(description-list)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(description-details)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(form)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(fieldset)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(option-group)"
            | "Microsoft.UI.Xaml.Controls.StackPanel(table-section)" => {
                Ok(WinUiWidgetKind::StackPanel)
            }
            "Microsoft.UI.Xaml.Controls.TextBlock"
            | "Microsoft.UI.Xaml.Controls.TextBlock(abbreviation)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(citation)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(definition)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(data-value)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(inserted-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(deleted-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(marked-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(time)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(emphasis)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(strong-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(code)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(keyboard-input)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(sample-output)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(variable)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(inline-quote)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(subscript)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(superscript)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(small-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(bold-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(italic-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(struck-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(underlined-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(bidi-isolate)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(bidi-override)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(line-break)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(word-break-opportunity)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(document-title)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(heading)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(ruby-base)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(ruby-text)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(ruby-parenthesis)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(figure-caption)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(description-term)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(legend)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(output)"
            | "Microsoft.UI.Xaml.Controls.TextBlock(table-caption)"
            | "Microsoft.UI.Xaml.Controls.Image" => Ok(WinUiWidgetKind::TextBlock),
            "Microsoft.UI.Xaml.Controls.Button"
            | "Microsoft.UI.Xaml.Controls.HyperlinkButton"
            | "Microsoft.UI.Xaml.Controls.HyperlinkButton(image-map-area)"
            | "Microsoft.UI.Xaml.Controls.Button(disclosure-summary)" => {
                Ok(WinUiWidgetKind::Button)
            }
            "Microsoft.UI.Xaml.Controls.TextBox"
            | "Microsoft.UI.Xaml.Controls.TextBox(search)"
            | "Microsoft.UI.Xaml.Controls.TextBox(textarea)"
            | "Microsoft.UI.Xaml.Controls.PasswordBox" => Ok(WinUiWidgetKind::TextBox),
            "Microsoft.UI.Xaml.Controls.CheckBox" => Ok(WinUiWidgetKind::CheckBox),
            "Microsoft.UI.Xaml.Controls.ToggleSwitch" => Ok(WinUiWidgetKind::ToggleSwitch),
            "Microsoft.UI.Xaml.Controls.RadioButtons" => Ok(WinUiWidgetKind::RadioButtons),
            "Microsoft.UI.Xaml.Controls.RadioButton" => Ok(WinUiWidgetKind::RadioButton),
            "Microsoft.UI.Xaml.Controls.ComboBox" => Ok(WinUiWidgetKind::ComboBox),
            "Microsoft.UI.Xaml.Controls.ListView" | "Microsoft.UI.Xaml.Controls.TreeView" => {
                Ok(WinUiWidgetKind::ListView)
            }
            "Microsoft.UI.Xaml.Controls.ListViewItem"
            | "Microsoft.UI.Xaml.Controls.TreeViewItem" => Ok(WinUiWidgetKind::ListViewItem),
            "Microsoft.UI.Xaml.Controls.ScrollViewer+StackPanel" => {
                Ok(WinUiWidgetKind::ScrollViewer)
            }
            "Microsoft.UI.Xaml.Controls.ContentDialog" => Ok(WinUiWidgetKind::ContentDialog),
            "Microsoft.UI.Xaml.Controls.ToolTip" => Ok(WinUiWidgetKind::ToolTip),
            "Microsoft.UI.Xaml.Controls.TabView" => Ok(WinUiWidgetKind::TabView),
            "Microsoft.UI.Xaml.Controls.TabViewItem" => Ok(WinUiWidgetKind::TabViewItem),
            "Microsoft.UI.Xaml.Controls.Grid"
            | "Microsoft.UI.Xaml.Controls.MediaPlayerElement"
            | "Microsoft.UI.Xaml.Controls.Canvas"
            | "Microsoft.UI.Xaml.Controls.Canvas(image-map)"
            | "Microsoft.UI.Xaml.Controls.ContentControl(embedded-content)"
            | "Microsoft.UI.Xaml.Controls.Grid(table)"
            | "Microsoft.UI.Xaml.Controls.Grid(row)"
            | "Microsoft.UI.Xaml.Controls.Grid(cell)"
            | "Microsoft.UI.Xaml.Controls.Grid(column)" => Ok(WinUiWidgetKind::Grid),
            "Microsoft.UI.Xaml.Controls.StackPanel(menu)" => Ok(WinUiWidgetKind::MenuPanel),
            "Microsoft.UI.Xaml.Controls.Button(menu-item)" => Ok(WinUiWidgetKind::MenuItemButton),
            "Microsoft.UI.Xaml.Controls.Primitives.SelectorItem" => {
                Ok(WinUiWidgetKind::SelectorItem)
            }
            "Microsoft.UI.Xaml.Controls.Border(separator)" => Ok(WinUiWidgetKind::Separator),
            "Microsoft.UI.Xaml.Controls.Slider" => Ok(WinUiWidgetKind::Slider),
            "Microsoft.UI.Xaml.Controls.ProgressBar"
            | "Microsoft.UI.Xaml.Controls.ProgressBar(meter)" => Ok(WinUiWidgetKind::ProgressBar),
            "Microsoft.UI.Xaml.Controls.StackPanel(toolbar)"
            | "Microsoft.UI.Xaml.Controls.CommandBar" => Ok(WinUiWidgetKind::CommandBar),
            other => Err(GuiError::host(format!(
                "unsupported WinUI widget class {other}"
            ))),
        }
    }
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
pub(crate) fn winui_max_length_value(max_length: Option<u32>) -> i32 {
    max_length
        .map(|value| value.min(i32::MAX as u32) as i32)
        .unwrap_or(0)
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
pub(crate) fn winui_truncate_to_max_length(value: &str, max_length: Option<u32>) -> String {
    let Some(max_length) = max_length else {
        return value.to_string();
    };
    let max_length = max_length as usize;
    if value.chars().count() <= max_length {
        value.to_string()
    } else {
        value.chars().take(max_length).collect()
    }
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct WinUiTextInputHints {
    pub spellcheck_enabled: Option<bool>,
    pub text_prediction_enabled: Option<bool>,
    pub prevent_keyboard_display_on_programmatic_focus: bool,
    pub color_font_enabled: bool,
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
pub(crate) fn winui_text_input_hints(config: &NativeWidgetConfig) -> WinUiTextInputHints {
    let hints = config.text_input_hints();
    WinUiTextInputHints {
        spellcheck_enabled: winui_spellcheck_enabled(config, hints.spellcheck),
        text_prediction_enabled: winui_text_prediction_enabled(config),
        prevent_keyboard_display_on_programmatic_focus: hints.inhibit_osk,
        color_font_enabled: hints.emoji.unwrap_or(true),
    }
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
fn winui_spellcheck_enabled(config: &NativeWidgetConfig, spellcheck: Option<bool>) -> Option<bool> {
    if spellcheck.is_some() {
        return spellcheck;
    }
    match config.text_input_purpose() {
        NativeTextInputPurpose::Digits
        | NativeTextInputPurpose::Number
        | NativeTextInputPurpose::Phone
        | NativeTextInputPurpose::Url
        | NativeTextInputPurpose::Email
        | NativeTextInputPurpose::Password
        | NativeTextInputPurpose::Pin
        | NativeTextInputPurpose::Terminal => Some(false),
        NativeTextInputPurpose::FreeForm
        | NativeTextInputPurpose::Alpha
        | NativeTextInputPurpose::Name => None,
    }
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
fn winui_text_prediction_enabled(config: &NativeWidgetConfig) -> Option<bool> {
    if normalized_token(config.input_mode.as_deref()).as_deref() == Some("none") {
        return Some(false);
    }

    match config.text_input_purpose() {
        NativeTextInputPurpose::Digits
        | NativeTextInputPurpose::Number
        | NativeTextInputPurpose::Phone
        | NativeTextInputPurpose::Url
        | NativeTextInputPurpose::Email
        | NativeTextInputPurpose::Password
        | NativeTextInputPurpose::Pin
        | NativeTextInputPurpose::Terminal => return Some(false),
        NativeTextInputPurpose::FreeForm
        | NativeTextInputPurpose::Alpha
        | NativeTextInputPurpose::Name => {}
    }

    match normalized_token(config.auto_correct.as_deref()).as_deref() {
        Some("on" | "true") => return Some(true),
        Some("off" | "false") => return Some(false),
        _ => {}
    }
    match normalized_token(config.autocomplete.as_deref()).as_deref() {
        Some("on") => Some(true),
        Some("off") => Some(false),
        _ => None,
    }
}

#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
fn normalized_token(value: Option<&str>) -> Option<String> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase)
}

#[derive(Clone, PartialEq)]
pub struct WinUiNativeObject {
    pub id: HostNodeId,
    pub kind: WinUiWidgetKind,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

impl std::fmt::Debug for WinUiNativeObject {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WinUiNativeObject")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("label", &self.label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct WinUiWidgetDriver {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
    overlay_positions: BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>,
    objects: BTreeMap<HostNodeId, WinUiNativeObject>,
    events: Vec<NativeEvent>,
}

pub type WinUiCommandExecutor = DriverCommandExecutor<WinUiWidgetDriver>;

#[derive(Debug, Clone)]
pub struct WinUiNativeHandle {
    state: Rc<RefCell<WinUiNativeHandleState>>,
}

impl WinUiNativeHandle {
    pub fn state(&self) -> Ref<'_, WinUiNativeHandleState> {
        self.state.borrow()
    }

    pub fn take_applied_setters(&self) -> Vec<NativeWidgetSetter> {
        std::mem::take(&mut self.state.borrow_mut().applied_setters)
    }
}

#[derive(Clone, PartialEq)]
pub struct WinUiNativeHandleState {
    pub id: HostNodeId,
    pub kind: WinUiWidgetKind,
    pub config: NativeWidgetConfig,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub applied_setters: Vec<NativeWidgetSetter>,
    pub children: Vec<HostNodeId>,
}

impl std::fmt::Debug for WinUiNativeHandleState {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WinUiNativeHandleState")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("config", &self.config)
            .field("label", &self.label)
            .field("has_value", &self.value.is_some())
            .field("action", &self.action)
            .field("control_state", &self.control_state)
            .field("applied_setters", &self.applied_setters)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Debug, Default)]
pub struct WinUiHandleAdapter {
    focused: Rc<Cell<Option<HostNodeId>>>,
    overlay_positions: Rc<RefCell<BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>>>,
}

impl WinUiHandleAdapter {
    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused.get()
    }

    pub fn overlay_positions(
        &self,
    ) -> Ref<'_, BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)>> {
        self.overlay_positions.borrow()
    }
}

pub type WinUiHandleDriver = HandleWidgetDriver<WinUiHandleAdapter>;
pub type WinUiHandleCommandExecutor = DriverCommandExecutor<WinUiHandleDriver>;

impl WinUiWidgetDriver {
    pub fn root(&self) -> Option<HostNodeId> {
        self.root
    }

    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused
    }

    pub fn overlay_positions(&self) -> &BTreeMap<HostNodeId, (HostNodeId, OverlayPositionRequest)> {
        &self.overlay_positions
    }

    pub fn object(&self, id: HostNodeId) -> Option<&WinUiNativeObject> {
        self.objects.get(&id)
    }

    pub fn objects(&self) -> &BTreeMap<HostNodeId, WinUiNativeObject> {
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
                "WinUI object {} does not exist",
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

impl NativeEventSource for WinUiWidgetDriver {
    fn take_native_events(&mut self) -> Vec<NativeEvent> {
        std::mem::take(&mut self.events)
    }
}

impl NativeHandleAdapter for WinUiHandleAdapter {
    type Handle = WinUiNativeHandle;

    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
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
        Ok(WinUiNativeHandle {
            state: Rc::new(RefCell::new(WinUiNativeHandleState {
                id,
                kind: WinUiWidgetKind::from_widget_kind(blueprint.widget_kind),
                label: config.label.clone(),
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
        state.kind = WinUiWidgetKind::from_widget_kind(blueprint.widget_kind);
        state.config = blueprint.config();
        state.label = state.config.label.clone();
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
        state.kind = WinUiWidgetKind::from_widget_kind(blueprint.widget_kind);
        let setters = patch.setters();
        apply_widget_setters(&mut state.config, &setters);
        state.label = state.config.label.clone();
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
                "WinUI handle id does not match focus target {}",
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
            return Err(GuiError::host("WinUI overlay or anchor handle id mismatch"));
        }
        if overlay_handle.state.borrow().kind != WinUiWidgetKind::ToolTip {
            return Err(GuiError::host(format!(
                "WinUI object {} is not a ToolTip",
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

impl NativeWidgetDriver for WinUiWidgetDriver {
    fn backend(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
    }

    fn create_widget(
        &mut self,
        id: HostNodeId,
        blueprint: &NativeWidgetBlueprint,
    ) -> GuiResult<()> {
        if self.objects.contains_key(&id) {
            return Err(GuiError::host(format!(
                "WinUI object {} already exists",
                id.get()
            )));
        }
        self.objects.insert(
            id,
            WinUiNativeObject {
                id,
                kind: WinUiWidgetKind::from_widget_kind(blueprint.widget_kind),
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
            .ok_or_else(|| GuiError::host(format!("WinUI object {} missing", id.get())))?;
        object.kind = WinUiWidgetKind::from_widget_kind(blueprint.widget_kind);
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
                "cannot insert WinUI object {} into itself",
                child.get()
            )));
        }
        if self.subtree_contains(child, parent) {
            return Err(GuiError::host(format!(
                "inserting WinUI object {} under {} would create a cycle",
                child.get(),
                parent.get()
            )));
        }

        for object in self.objects.values_mut() {
            object.children.retain(|existing| *existing != child);
        }
        let parent_object = self.objects.get_mut(&parent).ok_or_else(|| {
            GuiError::host(format!("WinUI parent object {} missing", parent.get()))
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
        if self.objects.get(&overlay).map(|object| object.kind) != Some(WinUiWidgetKind::ToolTip) {
            return Err(GuiError::host(format!(
                "WinUI object {} is not a ToolTip",
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
