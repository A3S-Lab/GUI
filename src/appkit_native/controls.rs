use super::*;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AppKitRangeState {
    pub(super) min: Option<f64>,
    pub(super) max: Option<f64>,
    pub(super) current: Option<f64>,
    pub(super) step: Option<f64>,
}

impl AppKitRangeState {
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

    pub(super) fn step(self) -> Option<f64> {
        self.step.filter(|value| value.is_finite() && *value > 0.0)
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct AppKitTextInputSizing {
    pub(super) rows: Option<u32>,
    pub(super) cols: Option<u32>,
    pub(super) size: Option<u32>,
    pub(super) explicit_width: Option<f64>,
    pub(super) explicit_height: Option<f64>,
}

impl AppKitTextInputSizing {
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
            .map(|columns| APPKIT_TEXT_INPUT_MIN_WIDTH.max(columns as f64 * 8.0 + 28.0))
    }

    pub(super) fn hinted_height(self) -> Option<f64> {
        if self.explicit_height.is_some() {
            return None;
        }
        self.rows
            .filter(|value| *value > 0)
            .map(|rows| (rows as f64 * 20.0 + 18.0).max(64.0))
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

pub(super) fn config_is_search(config: &NativeWidgetConfig) -> bool {
    config
        .input_type
        .as_deref()
        .is_some_and(|input_type| input_type.trim().eq_ignore_ascii_case("search"))
}

pub(super) fn apply_progress_range(progress: &NSProgressIndicator, range: AppKitRangeState) {
    progress.setMinValue(range.lower());
    progress.setMaxValue(range.upper());
    progress.setDoubleValue(range.current());
}

pub(super) fn apply_slider_step(slider: &NSSlider, range: AppKitRangeState) {
    let Some(step) = range.step() else {
        slider.setAllowsTickMarkValuesOnly(false);
        slider.setNumberOfTickMarks(0);
        slider.setAltIncrementValue(0.0);
        return;
    };

    slider.setAltIncrementValue(step);
    let span = range.upper() - range.lower();
    let ticks = (span / step).round() + 1.0;
    if span.is_finite()
        && span > 0.0
        && ticks >= 2.0
        && ticks <= MAX_APPKIT_SLIDER_TICK_MARKS as f64
    {
        slider.setNumberOfTickMarks(ticks as NSInteger);
        slider.setAllowsTickMarkValuesOnly(true);
    } else {
        slider.setNumberOfTickMarks(0);
        slider.setAllowsTickMarkValuesOnly(false);
    }
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

pub(super) fn set_control_string_value(control: &NSControl, value: &str, max_length: Option<u32>) {
    control.setStringValue(&ns_string(&truncate_to_max_length(value, max_length)));
}

pub(super) fn apply_control_max_length(control: &NSControl, max_length: Option<u32>) {
    let value = control.stringValue().to_string();
    set_control_string_value(control, &value, max_length);
}

pub(super) fn parse_f64(value: &str) -> Option<f64> {
    value.trim().parse::<f64>().ok()
}

pub(super) fn ns_string(value: &str) -> Retained<NSString> {
    NSString::from_str(value)
}

pub(super) fn ns_string_as_any(value: &NSString) -> &AnyObject {
    value.as_super().as_super()
}

pub(super) fn responder_key(responder: &NSResponder) -> usize {
    responder as *const NSResponder as usize
}

pub(super) fn appkit_key_value(event: &NSEvent) -> String {
    let characters = event
        .characters()
        .or_else(|| event.charactersIgnoringModifiers())
        .map(|characters| characters.to_string());
    appkit_key_value_from_parts(event.keyCode(), characters.as_deref())
}

pub(super) fn appkit_key_value_from_parts(key_code: u16, characters: Option<&str>) -> String {
    match key_code {
        36 | 76 => return "Enter".to_string(),
        48 => return "Tab".to_string(),
        49 => return " ".to_string(),
        51 => return "Backspace".to_string(),
        53 => return "Escape".to_string(),
        115 => return "Home".to_string(),
        116 => return "PageUp".to_string(),
        117 => return "Delete".to_string(),
        119 => return "End".to_string(),
        121 => return "PageDown".to_string(),
        123 => return "ArrowLeft".to_string(),
        124 => return "ArrowRight".to_string(),
        125 => return "ArrowDown".to_string(),
        126 => return "ArrowUp".to_string(),
        _ => {}
    }

    let raw = characters.unwrap_or_default();
    match raw {
        "\r" | "\n" => "Enter".to_string(),
        "\t" | "\u{19}" => "Tab".to_string(),
        "\u{1b}" => "Escape".to_string(),
        "\u{7f}" | "\u{8}" => "Backspace".to_string(),
        " " => " ".to_string(),
        value => crate::event::native_key_value(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::PlatformAdapter;

    #[test]
    fn truncate_to_max_length_limits_unicode_scalar_values() {
        let unicode_value = format!("a{}{}b", '\u{e9}', '\u{4e2d}');
        let unicode_prefix = format!("a{}{}", '\u{e9}', '\u{4e2d}');
        assert_eq!(truncate_to_max_length("abcdef", Some(3)), "abc");
        assert_eq!(
            truncate_to_max_length(&unicode_value, Some(3)),
            unicode_prefix
        );
        assert_eq!(truncate_to_max_length("abc", None), "abc");
        assert_eq!(truncate_to_max_length("abc", Some(0)), "");
    }

    #[test]
    fn appkit_key_value_normalizes_special_keys() {
        assert_eq!(appkit_key_value_from_parts(36, Some("\r")), "Enter");
        assert_eq!(appkit_key_value_from_parts(48, Some("\t")), "Tab");
        assert_eq!(appkit_key_value_from_parts(49, Some(" ")), " ");
        assert_eq!(appkit_key_value_from_parts(51, Some("\u{7f}")), "Backspace");
        assert_eq!(appkit_key_value_from_parts(53, Some("\u{1b}")), "Escape");
        assert_eq!(appkit_key_value_from_parts(123, None), "ArrowLeft");
        assert_eq!(appkit_key_value_from_parts(124, None), "ArrowRight");
        assert_eq!(appkit_key_value_from_parts(125, None), "ArrowDown");
        assert_eq!(appkit_key_value_from_parts(126, None), "ArrowUp");
        assert_eq!(appkit_key_value_from_parts(0, Some("a")), "a");
        assert_eq!(appkit_key_value_from_parts(14, Some("é")), "é");
    }

    #[test]
    fn appkit_window_close_events_are_queued_once_per_window() {
        let events = Rc::new(RefCell::new(Vec::new()));
        let closed_windows = Rc::new(RefCell::new(BTreeSet::new()));
        let first = HostNodeId::new(7);
        let second = HostNodeId::new(9);

        push_window_close_event_once(first, &events, &closed_windows);
        push_window_close_event_once(first, &events, &closed_windows);
        push_window_close_event_once(second, &events, &closed_windows);

        assert_eq!(
            *events.borrow(),
            vec![
                NativeEvent::new(first, NativeEventKind::Close),
                NativeEvent::new(second, NativeEventKind::Close)
            ]
        );
        assert!(closed_windows.borrow().contains(&first));
        assert!(closed_windows.borrow().contains(&second));
    }

    #[test]
    fn appkit_stack_insert_index_preserves_protocol_order() {
        assert_eq!(stack_insert_index(0, 0), 0);
        assert_eq!(stack_insert_index(1, 1), 1);
        assert_eq!(stack_insert_index(2, 2), 2);
        assert_eq!(stack_insert_index(2, 99), 2);
    }

    #[test]
    fn appkit_text_input_sizing_resets_removed_rows_to_default_height() {
        let sizing = AppKitTextInputSizing {
            rows: None,
            cols: Some(48),
            size: None,
            explicit_width: None,
            explicit_height: None,
        };

        assert_eq!(sizing.hinted_width(), Some(412.0));
        assert_eq!(sizing.hinted_height(), None);

        let config = NativeWidgetConfig {
            rows: None,
            cols: Some(48),
            ..AppKitAdapter
                .blueprint(&crate::native::NativeElement::new(
                    "notes",
                    crate::native::NativeRole::TextField,
                ))
                .config()
        };
        let size = config_text_input_size(&config);

        assert_eq!(size.width, 412.0);
        assert_eq!(size.height, APPKIT_TEXT_INPUT_DEFAULT_HEIGHT);
    }
}

pub(super) fn set_combo_box_value(combo_box: &NSComboBox, value: Option<&str>) {
    let value = ns_string(value.unwrap_or(""));
    unsafe {
        let object = ns_string_as_any(&value);
        if combo_box.indexOfItemWithObjectValue(object) >= 0 {
            combo_box.selectItemWithObjectValue(Some(object));
        }
    }
    combo_box.as_super().as_super().setStringValue(&value);
}

pub(super) fn combo_box_selected_value(combo_box: &NSComboBox) -> String {
    combo_box
        .objectValueOfSelectedItem()
        .and_then(|value| value.downcast::<NSString>().ok())
        .map(|value| value.to_string())
        .unwrap_or_else(|| combo_box.as_super().as_super().stringValue().to_string())
}

pub(super) fn list_view_selected_value(state: &AppKitListViewState) -> String {
    state
        .rows
        .borrow()
        .iter()
        .find(|row| row.button.state() == NSControlStateValueOn)
        .map(|row| row.value.clone())
        .unwrap_or_default()
}

pub(super) fn appkit_state(value: bool) -> NSControlStateValue {
    if value {
        NSControlStateValueOn
    } else {
        NSControlStateValueOff
    }
}

pub(super) fn set_button_checked(button: &NSButton, value: bool) {
    button.setState(appkit_state(value));
}

pub(super) fn set_switch_checked(switch: &NSSwitch, value: bool) {
    switch.setState(appkit_state(value));
}

pub(super) fn control_checked_value(sender: &AnyObject) -> bool {
    sender
        .downcast_ref::<NSButton>()
        .map(|button| button.state() == NSControlStateValueOn)
        .or_else(|| {
            sender
                .downcast_ref::<NSSwitch>()
                .map(|switch| switch.state() == NSControlStateValueOn)
        })
        .unwrap_or(false)
}

pub(super) fn control_double_value(sender: &AnyObject) -> f64 {
    sender
        .downcast_ref::<NSControl>()
        .map(NSControl::doubleValue)
        .unwrap_or_default()
}
