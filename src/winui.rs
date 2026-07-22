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
#[cfg(any(test, all(feature = "winui-native", target_os = "windows")))]
use crate::platform::NativeTextInputPurpose;
use crate::platform::{
    apply_widget_setters, NativeBackendKind, NativeControlState, NativeWidgetBlueprint,
    NativeWidgetConfig, NativeWidgetConfigPatch, NativeWidgetSetter,
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

#[derive(Debug, Clone, PartialEq)]
pub struct WinUiNativeObject {
    pub id: HostNodeId,
    pub kind: WinUiWidgetKind,
    pub label: Option<String>,
    pub value: Option<String>,
    pub action: Option<String>,
    pub control_state: NativeControlState,
    pub children: Vec<HostNodeId>,
}

#[derive(Debug, Default)]
pub struct WinUiWidgetDriver {
    root: Option<HostNodeId>,
    focused: Option<HostNodeId>,
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
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Default)]
pub struct WinUiHandleAdapter {
    focused: Rc<Cell<Option<HostNodeId>>>,
}

impl WinUiHandleAdapter {
    pub fn focused(&self) -> Option<HostNodeId> {
        self.focused.get()
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
        Ok(WinUiNativeHandle {
            state: Rc::new(RefCell::new(WinUiNativeHandleState {
                id,
                kind: WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
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
        state.kind = WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
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
        state.kind = WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
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

    fn remove_handle(&mut self, id: HostNodeId, handle: Self::Handle) -> GuiResult<()> {
        handle.state.borrow_mut().children.clear();
        if self.focused.get() == Some(id) {
            self.focused.set(None);
        }
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
                kind: WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?,
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
        object.kind = WinUiWidgetKind::from_widget_class(blueprint.widget_class.as_str())?;
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::CommandExecutingHost;
    use crate::compiler::CompiledRsxNode;
    use crate::geometry::Orientation;
    use crate::native::{NativeElement, NativeProps, NativeRole};
    use crate::platform::{PlatformAdapter, WinUiAdapter};
    use crate::runtime::GuiRuntime;
    use crate::style::{OverflowMode, StyleLength};

    #[test]
    fn winui_tree_blueprints_mount_through_flat_list_primitives() {
        let tree = WinUiAdapter.blueprint(&NativeElement::new("tree", NativeRole::Tree));
        let item = WinUiAdapter.blueprint(&NativeElement::new("item", NativeRole::TreeItem));

        assert_eq!(
            WinUiWidgetKind::from_widget_class(tree.widget_class.as_str()).unwrap(),
            WinUiWidgetKind::ListView
        );
        assert_eq!(
            WinUiWidgetKind::from_widget_class(item.widget_class.as_str()).unwrap(),
            WinUiWidgetKind::ListViewItem
        );
    }

    #[test]
    fn winui_max_length_value_maps_protocol_limits_to_winui_contract() {
        assert_eq!(winui_max_length_value(None), 0);
        assert_eq!(winui_max_length_value(Some(64)), 64);
        assert_eq!(winui_max_length_value(Some(i32::MAX as u32)), i32::MAX);
        assert_eq!(winui_max_length_value(Some(u32::MAX)), i32::MAX);
    }

    #[test]
    fn winui_truncate_to_max_length_limits_unicode_scalar_values() {
        assert_eq!(winui_truncate_to_max_length("abcdef", Some(3)), "abc");
        assert_eq!(winui_truncate_to_max_length("aé日b", Some(3)), "aé日");
        assert_eq!(winui_truncate_to_max_length("abc", None), "abc");
        assert_eq!(winui_truncate_to_max_length("abc", Some(0)), "");
    }

    #[test]
    fn winui_text_input_hints_disable_prediction_for_structured_fields() {
        let config = WinUiAdapter
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
            winui_text_input_hints(&config),
            WinUiTextInputHints {
                spellcheck_enabled: Some(true),
                text_prediction_enabled: Some(false),
                prevent_keyboard_display_on_programmatic_focus: false,
                color_font_enabled: true,
            }
        );
    }

    #[test]
    fn winui_text_input_hints_track_web_completion_and_keyboard_hints() {
        let config = WinUiAdapter
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
            winui_text_input_hints(&config),
            WinUiTextInputHints {
                spellcheck_enabled: Some(false),
                text_prediction_enabled: Some(false),
                prevent_keyboard_display_on_programmatic_focus: true,
                color_font_enabled: false,
            }
        );

        let config = WinUiAdapter
            .blueprint(
                &NativeElement::new("field", NativeRole::TextField)
                    .with_props(NativeProps::new().autocomplete("on")),
            )
            .config();

        assert_eq!(
            winui_text_input_hints(&config).text_prediction_enabled,
            Some(true)
        );
    }

    #[test]
    fn winui_widget_driver_reparents_children_and_removes_subtrees() {
        let mut driver = WinUiWidgetDriver::default();
        let root = HostNodeId::new(1);
        let child = HostNodeId::new(2);
        let grandchild = HostNodeId::new(3);
        let second = HostNodeId::new(4);
        let container = WinUiAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = WinUiAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

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
    fn winui_handle_adapter_clears_previous_parent_on_reparent() {
        let mut driver = WinUiHandleDriver::default();
        let first = HostNodeId::new(1);
        let second = HostNodeId::new(2);
        let child = HostNodeId::new(3);
        let container = WinUiAdapter.blueprint(&NativeElement::new("container", NativeRole::View));
        let button = WinUiAdapter.blueprint(&NativeElement::new("button", NativeRole::Button));

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
    fn winui_executor_consumes_compiled_semantic_ui_commands() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "props": {"isRequired": true, "isInvalid": true},
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "placeholder": "you@example.com",
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();

        assert_eq!(object.kind, WinUiWidgetKind::TextBox);
        assert_eq!(object.label.as_deref(), Some("Email"));
        assert_eq!(object.action.as_deref(), Some("setEmail"));
        assert_eq!(
            object.control_state.placeholder.as_deref(),
            Some("you@example.com")
        );
        assert!(object.control_state.required);
        assert!(object.control_state.invalid);
    }

    #[test]
    fn winui_executor_consumes_compiled_semantic_ui_toolbar_commands() {
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
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, WinUiWidgetKind::CommandBar);
        assert_eq!(
            object.control_state.orientation,
            Some(crate::geometry::Orientation::Horizontal)
        );
        assert_eq!(child.kind, WinUiWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("saveDocument"));
    }

    #[test]
    fn winui_executor_consumes_compiled_semantic_ui_dialog_commands() {
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
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, WinUiWidgetKind::ContentDialog);
        assert_eq!(object.label.as_deref(), Some("Preferences"));
        assert_eq!(child.kind, WinUiWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("closePreferences"));
    }

    #[test]
    fn winui_executor_consumes_compiled_semantic_ui_popover_commands() {
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
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let child = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, WinUiWidgetKind::ToolTip);
        assert_eq!(object.label.as_deref(), Some("Actions"));
        assert_eq!(child.kind, WinUiWidgetKind::Button);
        assert_eq!(child.action.as_deref(), Some("archiveItem"));
    }

    #[test]
    fn winui_executor_consumes_compiled_semantic_ui_menu_commands() {
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
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let object = runtime.host().executor().driver().object(root_id).unwrap();
        let item = runtime
            .host()
            .executor()
            .driver()
            .object(object.children[0])
            .unwrap();

        assert_eq!(object.kind, WinUiWidgetKind::MenuPanel);
        assert_eq!(item.kind, WinUiWidgetKind::MenuItemButton);
        assert_eq!(item.label.as_deref(), Some("Open"));
        assert_eq!(item.value.as_deref(), Some("open"));
        assert_eq!(item.action.as_deref(), Some("openFile"));
    }

    #[test]
    fn winui_handle_adapter_stores_thread_bound_native_handles() {
        let compiled: CompiledRsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "props": {"isRequired": true},
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "placeholder": "you@example.com",
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
        )
        .unwrap();
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiHandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();

        assert_eq!(state.kind, WinUiWidgetKind::TextBox);
        assert_eq!(state.label.as_deref(), Some("Email"));
        assert_eq!(state.action.as_deref(), Some("setEmail"));
        assert_eq!(
            state.control_state.placeholder.as_deref(),
            Some("you@example.com")
        );
        assert!(state.control_state.required);
        assert!(state.config.required);
        assert_eq!(state.config.placeholder.as_deref(), Some("you@example.com"));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetRequired(true)));
        assert!(state
            .applied_setters
            .contains(&NativeWidgetSetter::SetPlaceholder(Some(
                "you@example.com".to_string()
            ))));
    }

    #[test]
    fn winui_handle_adapter_clears_removed_text_max_length_on_rerender() {
        let mut driver = WinUiHandleDriver::default();
        let id = HostNodeId::new(1);
        let limited = WinUiAdapter.blueprint(
            &NativeElement::new("notes", NativeRole::TextField)
                .with_props(NativeProps::new().label("Notes").max_length(Some(8))),
        );
        let unlimited = WinUiAdapter.blueprint(
            &NativeElement::new("notes", NativeRole::TextField)
                .with_props(NativeProps::new().label("Notes")),
        );

        driver.create_widget(id, &limited).unwrap();
        let initial_setter_count = {
            let handle = driver.handle(id).unwrap();
            let state = handle.state();
            assert_eq!(state.config.max_length, Some(8));
            assert!(state
                .applied_setters
                .contains(&NativeWidgetSetter::SetMaxLength(Some(8))));
            state.applied_setters.len()
        };

        driver.update_widget(id, &unlimited).unwrap();

        let handle = driver.handle(id).unwrap();
        let state = handle.state();
        let update_setters = &state.applied_setters[initial_setter_count..];

        assert_eq!(state.config.max_length, None);
        assert_eq!(update_setters, [NativeWidgetSetter::SetMaxLength(None)]);
    }

    #[test]
    fn winui_scroll_handle_adapter_applies_rerender_style_setters() {
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
        let host = CommandExecutingHost::new(WinUiAdapter, WinUiHandleCommandExecutor::default());
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&first).unwrap();
        let initial_setter_count = {
            let handle = runtime.host().executor().driver().handle(root_id).unwrap();
            let state = handle.state();
            assert_eq!(state.kind, WinUiWidgetKind::ScrollViewer);
            state.applied_setters.len()
        };

        runtime.render_compiled(&second).unwrap();
        let handle = runtime.host().executor().driver().handle(root_id).unwrap();
        let state = handle.state();
        let update_setters = &state.applied_setters[initial_setter_count..];

        assert_eq!(state.kind, WinUiWidgetKind::ScrollViewer);
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
