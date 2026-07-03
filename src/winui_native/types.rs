use super::*;

#[derive(Debug, Clone)]
pub struct WinUiOsHandle {
    pub id: HostNodeId,
    pub kind: WinUiWidgetKind,
    pub widget: WinUiOsWidget,
}

#[derive(Debug, Clone)]
pub enum WinUiOsWidget {
    Window(xaml::Window),
    StackPanel(Controls::StackPanel),
    TextBlock(Controls::TextBlock),
    Separator(xaml::FrameworkElement),
    Button(Controls::Button),
    TextBox(Controls::TextBox),
    PasswordBox(Controls::PasswordBox),
    CheckBox(Controls::CheckBox),
    ToggleSwitch(Controls::CheckBox),
    RadioButton(Controls::RadioButton),
    ComboBox(Controls::ComboBox),
    ComboBoxItem(Controls::ComboBoxItem),
    ListBox(Controls::ListBox),
    ListBoxItem(Controls::ListBoxItem),
    ContentDialog(Controls::ContentDialog),
    ToolTip(Controls::ToolTip),
    TabView(Controls::TabView),
    TabViewItem(Controls::TabViewItem),
    Grid(Controls::Grid),
    Slider(Controls::Slider),
    ProgressBar(Controls::ProgressBar),
}

impl WinUiOsWidget {
    pub(super) fn ui_element(&self) -> Option<xaml::UIElement> {
        match self {
            WinUiOsWidget::Window(_) => None,
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => widget.cast().ok(),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::PasswordBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    pub(super) fn inspectable(&self) -> Option<windows_core::IInspectable> {
        match self {
            WinUiOsWidget::Window(widget) => widget.cast().ok(),
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => widget.cast().ok(),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::PasswordBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    pub(super) fn framework_element(&self) -> Option<xaml::FrameworkElement> {
        match self {
            WinUiOsWidget::Window(_) => None,
            WinUiOsWidget::StackPanel(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBlock(widget) => widget.cast().ok(),
            WinUiOsWidget::Separator(widget) => Some(widget.clone()),
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::PasswordBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Grid(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    pub(super) fn control(&self) -> Option<Controls::Control> {
        match self {
            WinUiOsWidget::Window(_)
            | WinUiOsWidget::StackPanel(_)
            | WinUiOsWidget::TextBlock(_)
            | WinUiOsWidget::Separator(_)
            | WinUiOsWidget::Grid(_) => None,
            WinUiOsWidget::Button(widget) => widget.cast().ok(),
            WinUiOsWidget::TextBox(widget) => widget.cast().ok(),
            WinUiOsWidget::PasswordBox(widget) => widget.cast().ok(),
            WinUiOsWidget::CheckBox(widget) | WinUiOsWidget::ToggleSwitch(widget) => {
                widget.cast().ok()
            }
            WinUiOsWidget::RadioButton(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ComboBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBox(widget) => widget.cast().ok(),
            WinUiOsWidget::ListBoxItem(widget) => widget.cast().ok(),
            WinUiOsWidget::ContentDialog(widget) => widget.cast().ok(),
            WinUiOsWidget::ToolTip(widget) => widget.cast().ok(),
            WinUiOsWidget::TabView(widget) => widget.cast().ok(),
            WinUiOsWidget::TabViewItem(widget) => widget.cast().ok(),
            WinUiOsWidget::Slider(widget) => widget.cast().ok(),
            WinUiOsWidget::ProgressBar(widget) => widget.cast().ok(),
        }
    }

    pub(super) fn children_collection(&self) -> GuiResult<Option<Controls::UIElementCollection>> {
        match self {
            WinUiOsWidget::StackPanel(widget) => Ok(Some(map_winui(
                "failed to read WinUI stack panel children",
                widget.Children(),
            )?)),
            WinUiOsWidget::Grid(widget) => Ok(Some(map_winui(
                "failed to read WinUI grid children",
                widget.Children(),
            )?)),
            _ => Ok(None),
        }
    }

    pub(super) fn items_collection(&self) -> GuiResult<Option<Controls::ItemCollection>> {
        match self {
            WinUiOsWidget::ComboBox(widget) => Ok(Some(map_winui(
                "failed to read WinUI combo box items",
                widget.Items(),
            )?)),
            WinUiOsWidget::ListBox(widget) => Ok(Some(map_winui(
                "failed to read WinUI list box items",
                widget.Items(),
            )?)),
            WinUiOsWidget::TabView(_) => Ok(None),
            _ => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WinUiComboBoxItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl WinUiComboBoxItem {
    pub(super) fn from_config(config: &NativeWidgetConfig) -> Self {
        let label = config
            .label
            .clone()
            .or_else(|| config.value.clone())
            .unwrap_or_default();
        let value = config.value.clone().unwrap_or_else(|| label.clone());
        Self {
            label,
            value,
            selected: config.selected,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WinUiTabItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl WinUiTabItem {
    pub(super) fn from_config(id: HostNodeId, config: &NativeWidgetConfig) -> Self {
        let label = config
            .label
            .clone()
            .or_else(|| config.value.clone())
            .unwrap_or_else(|| id.get().to_string());
        let value = config.value.clone().unwrap_or_else(|| label.clone());
        Self {
            label,
            value,
            selected: config.selected,
        }
    }

    pub(super) fn fallback(id: HostNodeId) -> Self {
        let label = id.get().to_string();
        Self {
            label: label.clone(),
            value: label,
            selected: false,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct WinUiRangeState {
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) current: Option<f64>,
    pub(super) step: Option<f64>,
}

impl WinUiRangeState {
    pub(super) fn from_config(config: &NativeWidgetConfig) -> Self {
        Self {
            min: config.min,
            max: config.max,
            current: config.current,
            step: config.step,
        }
    }

    pub(super) fn lower(self) -> f64 {
        self.min.unwrap_or(0.0)
    }

    pub(super) fn upper(self) -> f64 {
        self.max.unwrap_or(100.0)
    }

    pub(super) fn current(self) -> f64 {
        self.current.unwrap_or_else(|| self.lower())
    }

    pub(super) fn step(self) -> f64 {
        self.step.filter(|value| *value > 0.0).unwrap_or(1.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct WinUiTextInputSizing {
    pub(super) rows: Option<u32>,
    pub(super) cols: Option<u32>,
    pub(super) size: Option<u32>,
    pub(super) explicit_width: Option<f64>,
    pub(super) explicit_height: Option<f64>,
}

impl WinUiTextInputSizing {
    pub(super) fn from_config(config: &NativeWidgetConfig) -> Self {
        let size = config.portable_style.native_size_constraints();
        Self {
            rows: config.rows,
            cols: config.cols,
            size: config.size,
            explicit_width: size.width,
            explicit_height: size.height,
        }
    }

    pub(super) fn hinted_width(self) -> Option<f64> {
        if self.explicit_width.is_some() {
            return None;
        }
        self.size
            .or(self.cols)
            .filter(|value| *value > 0)
            .map(|columns| WINUI_TEXT_INPUT_MIN_WIDTH.max(columns as f64 * 8.0 + 28.0))
    }

    pub(super) fn hinted_height(self) -> Option<f64> {
        if self.explicit_height.is_some() {
            return None;
        }
        self.rows
            .filter(|value| *value > 0)
            .map(|rows| WINUI_TEXT_INPUT_MIN_HEIGHT.max(rows as f64 * 20.0 + 18.0))
    }
}

pub(super) fn config_is_textarea(config: &NativeWidgetConfig) -> bool {
    config
        .metadata
        .get(HTML_TAG_METADATA_KEY)
        .is_some_and(|tag| tag == "textarea")
}

pub(super) fn config_is_password(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("password"))
}
