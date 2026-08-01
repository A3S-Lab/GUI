use std::collections::BTreeMap;

use crate::input::NativeInputModality;
#[cfg(feature = "platform-runtime")]
use crate::native::NativeProps;
use crate::native::NativeRole;

use crate::event::{non_empty_action, NativeEventKind};

#[cfg(any(
    test,
    feature = "platform-runtime",
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
pub(crate) const DEFAULT_LONG_PRESS_THRESHOLD_MICROS: u64 = 500_000;
#[cfg(any(
    test,
    feature = "platform-runtime",
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
pub(crate) const MAX_LONG_PRESS_THRESHOLD_MICROS: u64 = 60_000_000;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SemanticEventData<'a> {
    pub(crate) kind: NativeEventKind,
    pub(crate) modality: NativeInputModality,
    pub(crate) value: Option<&'a str>,
    pub(crate) handled_activation: bool,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct SemanticActionSource<'a> {
    pub(crate) role: NativeRole,
    action: Option<&'a str>,
    events: &'a BTreeMap<String, String>,
    metadata: &'a BTreeMap<String, String>,
    attributes: Option<&'a BTreeMap<String, String>>,
    expanded: Option<bool>,
}

impl<'a> SemanticActionSource<'a> {
    pub(crate) fn new(
        role: NativeRole,
        action: Option<&'a str>,
        events: &'a BTreeMap<String, String>,
        metadata: &'a BTreeMap<String, String>,
        attributes: Option<&'a BTreeMap<String, String>>,
        expanded: Option<bool>,
    ) -> Self {
        Self {
            role,
            action: action.filter(|action| !action.is_empty()),
            events,
            metadata,
            attributes,
            expanded,
        }
    }

    #[cfg(feature = "platform-runtime")]
    pub(crate) fn from_props(role: NativeRole, props: &'a NativeProps) -> Self {
        Self::new(
            role,
            props.action.as_deref(),
            &props.web.events,
            &props.metadata,
            Some(&props.web.attributes),
            props.expanded,
        )
    }

    #[cfg(feature = "platform-runtime")]
    pub(crate) fn has_interaction_binding(self) -> bool {
        self.action.is_some() || self.events.values().any(|action| !action.is_empty())
    }

    #[cfg(feature = "platform-runtime")]
    pub(crate) fn tracks_press(self) -> bool {
        self.action.is_some()
            || [
                "onClick",
                "onPress",
                "onPressStart",
                "onPressEnd",
                "onPressUp",
                "onPressChange",
            ]
            .into_iter()
            .any(|name| self.event(name).is_some())
            || role_is_pressable(self.role)
    }

    pub(crate) fn static_action_value(self) -> Option<&'a str> {
        const KEYS: [&str; 8] = [
            "actionValue",
            "action-value",
            "actionPayload",
            "action-payload",
            "data-action-value",
            "data-action-payload",
            "data-a3s-action-value",
            "data-a3s-action-payload",
        ];
        KEYS.into_iter().find_map(|name| {
            self.metadata
                .get(name)
                .or_else(|| self.attributes.and_then(|attributes| attributes.get(name)))
                .map(String::as_str)
                .filter(|value| !value.is_empty())
        })
    }

    fn event(self, name: &str) -> Option<&'a str> {
        non_empty_action(self.events.get(name))
    }

    fn press_action(self) -> Option<&'a str> {
        self.event("onPress")
            .or_else(|| self.event("onClick"))
            .or(self.action)
    }

    fn is_expansion_toggle(self) -> bool {
        matches!(
            self.role,
            NativeRole::Disclosure | NativeRole::DisclosureSummary | NativeRole::Popover
        ) || self.expanded.is_some()
    }
}

pub(crate) fn actions_for_event<'a>(
    source: SemanticActionSource<'a>,
    event: SemanticEventData<'_>,
) -> Vec<&'a str> {
    match event.kind {
        NativeEventKind::PressStart => {
            [source.event("onPressStart"), source.event("onPressChange")]
                .into_iter()
                .flatten()
                .collect()
        }
        NativeEventKind::PressEnd | NativeEventKind::PressCancel => {
            [source.event("onPressEnd"), source.event("onPressChange")]
                .into_iter()
                .flatten()
                .collect()
        }
        NativeEventKind::HoverStart if event.modality.supports_hover() => {
            [source.event("onHoverStart"), source.event("onHoverChange")]
                .into_iter()
                .flatten()
                .collect()
        }
        NativeEventKind::HoverEnd if event.modality.supports_hover() => {
            [source.event("onHoverEnd"), source.event("onHoverChange")]
                .into_iter()
                .flatten()
                .collect()
        }
        NativeEventKind::Focus => [source.event("onFocus"), source.event("onFocusChange")]
            .into_iter()
            .flatten()
            .collect(),
        NativeEventKind::Blur => [source.event("onBlur"), source.event("onFocusChange")]
            .into_iter()
            .flatten()
            .collect(),
        _ => action_for_event(source, event).into_iter().collect(),
    }
}

pub(crate) fn focus_within_actions_for_event(
    source: SemanticActionSource<'_>,
    kind: NativeEventKind,
) -> Vec<&str> {
    match kind {
        NativeEventKind::Focus => [
            source.event("onFocusWithin"),
            source.event("onFocusWithinChange"),
        ]
        .into_iter()
        .flatten()
        .collect(),
        NativeEventKind::Blur => [
            source.event("onBlurWithin"),
            source.event("onFocusWithinChange"),
        ]
        .into_iter()
        .flatten()
        .collect(),
        _ => Vec::new(),
    }
}

fn action_for_event<'a>(
    source: SemanticActionSource<'a>,
    event: SemanticEventData<'_>,
) -> Option<&'a str> {
    match event.kind {
        NativeEventKind::PressStart => source
            .event("onPressStart")
            .or_else(|| source.event("onPressChange")),
        NativeEventKind::PressEnd => source
            .event("onPressEnd")
            .or_else(|| source.event("onPressChange")),
        NativeEventKind::PressUp => source.event("onPressUp"),
        NativeEventKind::PressCancel => source
            .event("onPressEnd")
            .or_else(|| source.event("onPressChange")),
        NativeEventKind::Press => source.press_action(),
        NativeEventKind::LongPressStart => source.event("onLongPressStart"),
        NativeEventKind::LongPressEnd => source.event("onLongPressEnd"),
        NativeEventKind::LongPress => source.event("onLongPress"),
        NativeEventKind::MoveStart => source.event("onMoveStart"),
        NativeEventKind::Move => source.event("onMove"),
        NativeEventKind::MoveEnd => source.event("onMoveEnd"),
        NativeEventKind::DragStart => source.event("onDragStart"),
        NativeEventKind::DragMove => source.event("onDragMove"),
        NativeEventKind::DragEnd => source.event("onDragEnd"),
        NativeEventKind::DropEnter => source
            .event("onDropEnter")
            .or_else(|| source.event("onDragEnter")),
        NativeEventKind::DropMove => source.event("onDropMove"),
        NativeEventKind::DropExit => source
            .event("onDropExit")
            .or_else(|| source.event("onDragLeave")),
        NativeEventKind::Drop => source.event("onDrop"),
        NativeEventKind::Action => source.event("onAction"),
        NativeEventKind::HoverStart if event.modality.supports_hover() => source
            .event("onHoverStart")
            .or_else(|| source.event("onHoverChange")),
        NativeEventKind::HoverEnd if event.modality.supports_hover() => source
            .event("onHoverEnd")
            .or_else(|| source.event("onHoverChange")),
        NativeEventKind::HoverStart | NativeEventKind::HoverEnd => None,
        NativeEventKind::Change => source
            .event("onChange")
            .or_else(|| source.event("onInput"))
            .or(source.action),
        NativeEventKind::SelectionChange => source
            .event("onSelectionChange")
            .or_else(|| source.event("onChange"))
            .or_else(|| source.event("onInput"))
            .or(source.action),
        NativeEventKind::Toggle
            if source.role == NativeRole::Tree || source.is_expansion_toggle() =>
        {
            source
                .event("onExpandedChange")
                .or_else(|| source.event("onToggle"))
                .or_else(|| source.event("onChange"))
                .or(source.action)
        }
        NativeEventKind::Toggle => source
            .event("onChange")
            .or_else(|| source.event("onInput"))
            .or_else(|| source.event("onToggle"))
            .or_else(|| source.event("onClick"))
            .or(source.action),
        NativeEventKind::Focus => source
            .event("onFocus")
            .or_else(|| source.event("onFocusChange")),
        NativeEventKind::Blur => source
            .event("onBlur")
            .or_else(|| source.event("onFocusChange")),
        NativeEventKind::KeyDown => source.event("onKeyDown").or_else(|| {
            (!event.handled_activation && is_press_activation_key(source.role, event.value))
                .then(|| source.press_action())
                .flatten()
        }),
        NativeEventKind::KeyUp => source.event("onKeyUp"),
        NativeEventKind::Wheel => source.event("onWheel"),
        NativeEventKind::Copy => source.event("onCopy"),
        NativeEventKind::Cut => source.event("onCut"),
        NativeEventKind::Paste => source.event("onPaste"),
        NativeEventKind::Close => source
            .event("onClose")
            .or_else(|| source.event("onCloseRequest")),
    }
}

pub(crate) fn native_key_value(raw: &str) -> String {
    if raw == " " {
        return " ".to_string();
    }
    let trimmed = raw.trim();
    match trimmed {
        "Return" | "KP_Enter" | "ISO_Enter" => "Enter".to_string(),
        "space" | "Space" | "Spacebar" => " ".to_string(),
        "BackSpace" => "Backspace".to_string(),
        "Esc" => "Escape".to_string(),
        "ISO_Left_Tab" => "Tab".to_string(),
        "Left" => "ArrowLeft".to_string(),
        "Right" => "ArrowRight".to_string(),
        "Up" => "ArrowUp".to_string(),
        "Down" => "ArrowDown".to_string(),
        "Page_Up" => "PageUp".to_string(),
        "Page_Down" => "PageDown".to_string(),
        "" => String::new(),
        value => value.to_string(),
    }
}

#[cfg(any(
    test,
    feature = "platform-runtime",
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
pub(crate) fn long_press_threshold_micros(metadata: &BTreeMap<String, String>) -> u64 {
    ["threshold", "data-long-press-threshold"]
        .into_iter()
        .find_map(|name| metadata.get(name))
        .and_then(|value| value.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .map(|millis| millis.saturating_mul(1_000))
        .map(|micros| micros.min(MAX_LONG_PRESS_THRESHOLD_MICROS))
        .unwrap_or(DEFAULT_LONG_PRESS_THRESHOLD_MICROS)
}

pub(crate) fn is_press_activation_key(role: NativeRole, value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = native_key_value(value);
    match role {
        NativeRole::Link | NativeRole::ImageMapArea => normalized.eq_ignore_ascii_case("enter"),
        NativeRole::Button | NativeRole::DisclosureSummary | NativeRole::MenuItem => {
            is_activation_key(Some(&normalized))
        }
        NativeRole::ListBoxItem | NativeRole::TreeItem => normalized.eq_ignore_ascii_case("enter"),
        _ => false,
    }
}

pub(crate) fn is_activation_key(value: Option<&str>) -> bool {
    let Some(value) = value else {
        return false;
    };
    let normalized = native_key_value(value);
    normalized.eq_ignore_ascii_case("enter")
        || normalized == " "
        || normalized.eq_ignore_ascii_case("space")
        || normalized.eq_ignore_ascii_case("spacebar")
}

#[cfg(feature = "platform-runtime")]
fn role_is_pressable(role: NativeRole) -> bool {
    matches!(
        role,
        NativeRole::Button
            | NativeRole::Link
            | NativeRole::ImageMapArea
            | NativeRole::Checkbox
            | NativeRole::Switch
            | NativeRole::Radio
            | NativeRole::ListBoxItem
            | NativeRole::TreeItem
            | NativeRole::DisclosureSummary
            | NativeRole::Tab
            | NativeRole::MenuItem
    )
}
