use super::*;

#[derive(Debug, Clone)]
pub(super) struct Gtk4DropDownState {
    pub(super) drop_down: gtk::DropDown,
    pub(super) model: gtk::StringList,
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Gtk4RangeState {
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) current: Option<f64>,
    pub(super) step: Option<f64>,
}

impl Gtk4RangeState {
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

    pub(super) fn spin_button_digits(self) -> u32 {
        [self.min, self.max, self.current, self.step]
            .into_iter()
            .flatten()
            .map(gtk_number_digits)
            .max()
            .unwrap_or(0)
    }
}

pub(super) fn gtk_number_digits(value: f64) -> u32 {
    if !value.is_finite() || value.fract().abs() < f64::EPSILON {
        return 0;
    }

    let mut scaled = value.abs();
    for digits in 1..=6 {
        scaled *= 10.0;
        if (scaled - scaled.round()).abs() < 1e-9 {
            return digits;
        }
    }
    6
}

pub(super) fn parse_gtk_number_value(value: Option<&str>) -> Option<f64> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<f64>().ok())
        .filter(|value| value.is_finite())
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Gtk4TextInputSizing {
    pub(super) rows: Option<u32>,
    pub(super) cols: Option<u32>,
    pub(super) size: Option<u32>,
    pub(super) has_explicit_width: bool,
    pub(super) has_explicit_height: bool,
}

impl Gtk4TextInputSizing {
    pub(super) fn from_config(config: &NativeWidgetConfig) -> Self {
        Self {
            rows: config.rows,
            cols: config.cols,
            size: config.size,
            has_explicit_width: style_sets_gtk_width(&config.portable_style),
            has_explicit_height: style_sets_gtk_height(&config.portable_style),
        }
    }

    pub(super) fn hinted_width_chars(self) -> Option<i32> {
        self.size
            .or(self.cols)
            .filter(|value| *value > 0)
            .map(u32_to_i32)
    }

    pub(super) fn hinted_width_points(self) -> Option<i32> {
        self.cols
            .filter(|value| *value > 0)
            .map(|columns| (columns as f64 * 8.0 + 28.0).max(80.0))
            .map(points_to_i32)
    }

    pub(super) fn hinted_height_points(self) -> Option<i32> {
        self.rows
            .filter(|value| *value > 0)
            .map(|rows| (rows as f64 * 20.0 + 18.0).max(64.0))
            .map(points_to_i32)
    }

    pub(super) fn text_view_size_request(
        self,
        current_width: i32,
        current_height: i32,
    ) -> (i32, i32) {
        let width = if self.has_explicit_width {
            current_width
        } else {
            self.hinted_width_points().unwrap_or(-1)
        };
        let height = if self.has_explicit_height {
            current_height
        } else {
            self.hinted_height_points().unwrap_or(-1)
        };
        (width, height)
    }
}

#[derive(Debug, Clone)]
pub struct Gtk4OsHandle {
    pub id: HostNodeId,
    pub kind: Gtk4WidgetKind,
    pub widget: Gtk4OsWidget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gtk4DropDownItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl Gtk4DropDownItem {
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
pub struct Gtk4NotebookTab {
    pub label: String,
    pub value: String,
    pub selected: bool,
    pub panel: Option<HostNodeId>,
}

impl Gtk4NotebookTab {
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
            panel: None,
        }
    }

    pub(super) fn fallback(id: HostNodeId) -> Self {
        let label = id.get().to_string();
        Self {
            label: label.clone(),
            value: label,
            selected: false,
            panel: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Gtk4OsWidget {
    ApplicationWindow(gtk::ApplicationWindow),
    Box(gtk::Box),
    Label(gtk::Label),
    Button(gtk::Button),
    Entry(gtk::Entry),
    SearchEntry(gtk::SearchEntry),
    PasswordEntry(gtk::PasswordEntry),
    SpinButton(gtk::SpinButton),
    TextView(gtk::TextView),
    CheckButton(gtk::CheckButton),
    Switch(gtk::Switch),
    DropDown(gtk::DropDown),
    ListBox(gtk::ListBox),
    ScrolledWindow {
        scrolled_window: gtk::ScrolledWindow,
        content: gtk::Box,
    },
    ListBoxRow {
        row: gtk::ListBoxRow,
        label: gtk::Label,
        item: Gtk4DropDownItem,
    },
    Dialog(gtk::Dialog),
    Popover(gtk::Popover),
    Menu(Gtk4Menu),
    MenuItem(Gtk4MenuItem),
    Notebook(gtk::Notebook),
    Separator(gtk::Separator),
    Scale(gtk::Scale),
    ProgressBar(gtk::ProgressBar),
}

impl Gtk4OsWidget {
    pub(super) fn as_widget(&self) -> Option<gtk::Widget> {
        match self {
            Gtk4OsWidget::ApplicationWindow(window) => Some(window.clone().upcast()),
            Gtk4OsWidget::Box(box_) => Some(box_.clone().upcast()),
            Gtk4OsWidget::Label(label) => Some(label.clone().upcast()),
            Gtk4OsWidget::Button(button) => Some(button.clone().upcast()),
            Gtk4OsWidget::Entry(entry) => Some(entry.clone().upcast()),
            Gtk4OsWidget::SearchEntry(entry) => Some(entry.clone().upcast()),
            Gtk4OsWidget::PasswordEntry(entry) => Some(entry.clone().upcast()),
            Gtk4OsWidget::SpinButton(spin_button) => Some(spin_button.clone().upcast()),
            Gtk4OsWidget::TextView(text_view) => Some(text_view.clone().upcast()),
            Gtk4OsWidget::CheckButton(check_button) => Some(check_button.clone().upcast()),
            Gtk4OsWidget::Switch(switch) => Some(switch.clone().upcast()),
            Gtk4OsWidget::DropDown(drop_down) => Some(drop_down.clone().upcast()),
            Gtk4OsWidget::ListBox(list_box) => Some(list_box.clone().upcast()),
            Gtk4OsWidget::ScrolledWindow {
                scrolled_window, ..
            } => Some(scrolled_window.clone().upcast()),
            Gtk4OsWidget::ListBoxRow { row, .. } => Some(row.clone().upcast()),
            Gtk4OsWidget::Dialog(dialog) => Some(dialog.clone().upcast()),
            Gtk4OsWidget::Popover(popover) => Some(popover.clone().upcast()),
            Gtk4OsWidget::Menu(menu) => Some(menu.bar.clone().upcast()),
            Gtk4OsWidget::Notebook(notebook) => Some(notebook.clone().upcast()),
            Gtk4OsWidget::Separator(separator) => Some(separator.clone().upcast()),
            Gtk4OsWidget::Scale(scale) => Some(scale.clone().upcast()),
            Gtk4OsWidget::ProgressBar(progress_bar) => Some(progress_bar.clone().upcast()),
            Gtk4OsWidget::MenuItem(_) => None,
        }
    }
}
pub(super) fn push_event(
    events: &Rc<RefCell<Vec<NativeEvent>>>,
    events_suppressed: &Rc<RefCell<bool>>,
    event: NativeEvent,
) {
    if !*events_suppressed.borrow() {
        events.borrow_mut().push(event);
    }
}

pub(super) fn gtk_key_value(key: gtk::gdk::Key, keycode: u32) -> String {
    key.to_unicode()
        .filter(|value| !value.is_control())
        .map(|value| value.to_string())
        .or_else(|| {
            key.name()
                .map(|name| native_key_value(name.as_str()))
                .filter(|value| !value.is_empty())
        })
        .unwrap_or_else(|| keycode.to_string())
}

pub(super) fn config_orientation(config: &NativeWidgetConfig) -> Option<gtk::Orientation> {
    config
        .orientation
        .or(config.portable_style.flex_direction)
        .map(gtk_orientation)
}

pub(super) fn gtk_orientation(orientation: Orientation) -> gtk::Orientation {
    match orientation {
        Orientation::Horizontal => gtk::Orientation::Horizontal,
        Orientation::Vertical => gtk::Orientation::Vertical,
    }
}

pub(super) fn gtk_input_purpose(purpose: NativeTextInputPurpose) -> gtk::InputPurpose {
    match purpose {
        NativeTextInputPurpose::FreeForm => gtk::InputPurpose::FreeForm,
        NativeTextInputPurpose::Alpha => gtk::InputPurpose::Alpha,
        NativeTextInputPurpose::Digits => gtk::InputPurpose::Digits,
        NativeTextInputPurpose::Number => gtk::InputPurpose::Number,
        NativeTextInputPurpose::Phone => gtk::InputPurpose::Phone,
        NativeTextInputPurpose::Url => gtk::InputPurpose::Url,
        NativeTextInputPurpose::Email => gtk::InputPurpose::Email,
        NativeTextInputPurpose::Name => gtk::InputPurpose::Name,
        NativeTextInputPurpose::Password => gtk::InputPurpose::Password,
        NativeTextInputPurpose::Pin => gtk::InputPurpose::Pin,
        NativeTextInputPurpose::Terminal => gtk::InputPurpose::Terminal,
    }
}

pub(super) fn gtk_input_hints(hints: NativeTextInputHints) -> gtk::InputHints {
    let mut gtk_hints = gtk::InputHints::NONE;
    match hints.spellcheck {
        Some(true) => gtk_hints.insert(gtk::InputHints::SPELLCHECK),
        Some(false) => gtk_hints.insert(gtk::InputHints::NO_SPELLCHECK),
        None => {}
    }
    if hints.word_completion {
        gtk_hints.insert(gtk::InputHints::WORD_COMPLETION);
    }
    if hints.lowercase {
        gtk_hints.insert(gtk::InputHints::LOWERCASE);
    }
    if hints.uppercase_chars {
        gtk_hints.insert(gtk::InputHints::UPPERCASE_CHARS);
    }
    if hints.uppercase_words {
        gtk_hints.insert(gtk::InputHints::UPPERCASE_WORDS);
    }
    if hints.uppercase_sentences {
        gtk_hints.insert(gtk::InputHints::UPPERCASE_SENTENCES);
    }
    if hints.inhibit_osk {
        gtk_hints.insert(gtk::InputHints::INHIBIT_OSK);
    }
    match hints.emoji {
        Some(true) => gtk_hints.insert(gtk::InputHints::EMOJI),
        Some(false) => gtk_hints.insert(gtk::InputHints::NO_EMOJI),
        None => {}
    }
    if hints.private {
        gtk_hints.insert(gtk::InputHints::PRIVATE);
    }
    gtk_hints
}

pub(super) fn config_dimension(value: Option<f64>, default: i32) -> i32 {
    value.map(points_to_i32).unwrap_or(default)
}

pub(super) fn gtk4_scroll_policy(value: Option<OverflowMode>) -> gtk::PolicyType {
    match value {
        Some(OverflowMode::Scroll) => gtk::PolicyType::Always,
        Some(OverflowMode::Hidden | OverflowMode::Clip) => gtk::PolicyType::Never,
        Some(OverflowMode::Visible | OverflowMode::Auto) | None => gtk::PolicyType::Automatic,
    }
}

pub(super) fn apply_widget_size(widget: &gtk::Widget, style: &crate::style::PortableStyle) {
    let size = style.native_size_constraints();
    let width = size
        .width
        .or(size.min_width)
        .map(points_to_i32)
        .unwrap_or(-1);
    let height = size
        .height
        .or(size.min_height)
        .map(points_to_i32)
        .unwrap_or(-1);
    if width >= 0 || height >= 0 {
        widget.set_size_request(width, height);
    }
}

pub(super) fn style_sets_gtk_width(style: &crate::style::PortableStyle) -> bool {
    let size = style.native_size_constraints();
    size.width.or(size.min_width).is_some()
}

pub(super) fn style_sets_gtk_height(style: &crate::style::PortableStyle) -> bool {
    let size = style.native_size_constraints();
    size.height.or(size.min_height).is_some()
}

pub(super) fn config_is_password(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("password"))
}

pub(super) fn config_is_search(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("search"))
}

pub(super) fn u32_to_i32(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

pub(super) fn text_buffer_text(buffer: &gtk::TextBuffer) -> String {
    let (start, end) = buffer.bounds();
    buffer.text(&start, &end, true).to_string()
}

pub(super) fn truncate_to_max_length(value: &str, max_length: Option<u32>) -> String {
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

pub(super) fn set_text_buffer_text(buffer: &gtk::TextBuffer, value: &str, max_length: Option<u32>) {
    buffer.set_text(&truncate_to_max_length(value, max_length));
}

pub(super) fn set_progress_bar_fraction(progress_bar: &gtk::ProgressBar, range: Gtk4RangeState) {
    let min = range.lower();
    let max = range.upper();
    let current = range.current();
    let range = max - min;
    let fraction = if range.abs() < f64::EPSILON {
        0.0
    } else {
        ((current - min) / range).clamp(0.0, 1.0)
    };
    progress_bar.set_fraction(fraction);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_to_max_length_limits_unicode_scalar_values() {
        assert_eq!(truncate_to_max_length("abcdef", Some(3)), "abc");
        assert_eq!(truncate_to_max_length("aé日b", Some(3)), "aé日");
        assert_eq!(truncate_to_max_length("abc", None), "abc");
        assert_eq!(truncate_to_max_length("abc", Some(0)), "");
    }

    #[test]
    fn gtk4_text_input_sizing_resets_removed_text_view_hints() {
        let sizing = Gtk4TextInputSizing {
            rows: None,
            cols: None,
            size: None,
            has_explicit_width: false,
            has_explicit_height: false,
        };

        assert_eq!(sizing.hinted_width_points(), None);
        assert_eq!(sizing.hinted_height_points(), None);
        assert_eq!(sizing.text_view_size_request(412, 138), (-1, -1));
    }

    #[test]
    fn gtk4_text_view_sizing_preserves_explicit_axis_requests() {
        let sizing = Gtk4TextInputSizing {
            rows: Some(6),
            cols: None,
            size: None,
            has_explicit_width: true,
            has_explicit_height: false,
        };

        assert_eq!(sizing.text_view_size_request(320, -1), (320, 138));
    }
}

pub(super) fn points_to_i32(value: f64) -> i32 {
    if value.is_finite() {
        value.round().clamp(i32::MIN as f64, i32::MAX as f64) as i32
    } else {
        -1
    }
}

pub(super) fn index_to_i32(index: usize) -> GuiResult<i32> {
    index
        .try_into()
        .map_err(|_| GuiError::host("GTK4 child index overflow"))
}
