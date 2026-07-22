use serde::{Deserialize, Serialize};

/// The input modality that produced a semantic native event.
///
/// This mirrors the interaction methods exposed by React Aria press events,
/// while retaining `Unknown` for legacy platform adapters and protocol peers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeInputModality {
    #[default]
    Unknown,
    Keyboard,
    Mouse,
    Touch,
    Pen,
    Virtual,
}

impl NativeInputModality {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Keyboard => "keyboard",
            Self::Mouse => "mouse",
            Self::Touch => "touch",
            Self::Pen => "pen",
            Self::Virtual => "virtual",
        }
    }

    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    pub fn supports_hover(self) -> bool {
        matches!(self, Self::Mouse | Self::Pen)
    }

    pub fn shows_focus_ring(self) -> bool {
        matches!(self, Self::Keyboard | Self::Virtual)
    }
}

/// Keyboard modifiers captured when an event was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeKeyModifiers {
    #[serde(default, skip_serializing_if = "is_false")]
    pub alt: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub control: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub meta: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub shift: bool,
}

impl NativeKeyModifiers {
    pub const fn new() -> Self {
        Self {
            alt: false,
            control: false,
            meta: false,
            shift: false,
        }
    }

    pub const fn alt(mut self, pressed: bool) -> Self {
        self.alt = pressed;
        self
    }

    pub const fn control(mut self, pressed: bool) -> Self {
        self.control = pressed;
        self
    }

    pub const fn meta(mut self, pressed: bool) -> Self {
        self.meta = pressed;
        self
    }

    pub const fn shift(mut self, pressed: bool) -> Self {
        self.shift = pressed;
        self
    }

    pub fn is_empty(&self) -> bool {
        !self.alt && !self.control && !self.meta && !self.shift
    }
}

/// A position in the local coordinate space of the event target.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeEventPosition {
    pub x: f64,
    pub y: f64,
}

impl NativeEventPosition {
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// Optional details shared by semantic pointer, keyboard, and virtual events.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeEventContext {
    #[serde(default, skip_serializing_if = "NativeInputModality::is_unknown")]
    pub modality: NativeInputModality,
    #[serde(default, skip_serializing_if = "NativeKeyModifiers::is_empty")]
    pub modifiers: NativeKeyModifiers,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<NativeEventPosition>,
    /// Distance travelled since the previous move event. Move start/end
    /// events omit this value, matching React Aria's move event contract.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delta: Option<NativeEventPosition>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub repeat: bool,
    /// Consecutive pointer press count reported by the native toolkit or the
    /// shared press state machine. Zero means the source did not provide one.
    #[serde(default, skip_serializing_if = "is_zero")]
    pub click_count: u8,
    /// The native source emitted a separate semantic activation lifecycle for
    /// this raw key event, so routing must not synthesize a second press.
    #[serde(default, skip_serializing_if = "is_false")]
    pub handled_activation: bool,
}

impl NativeEventContext {
    pub const fn new() -> Self {
        Self {
            modality: NativeInputModality::Unknown,
            modifiers: NativeKeyModifiers::new(),
            position: None,
            delta: None,
            repeat: false,
            click_count: 0,
            handled_activation: false,
        }
    }

    pub const fn modality(mut self, modality: NativeInputModality) -> Self {
        self.modality = modality;
        self
    }

    pub const fn modifiers(mut self, modifiers: NativeKeyModifiers) -> Self {
        self.modifiers = modifiers;
        self
    }

    pub const fn position(mut self, x: f64, y: f64) -> Self {
        self.position = Some(NativeEventPosition::new(x, y));
        self
    }

    pub const fn delta(mut self, x: f64, y: f64) -> Self {
        self.delta = Some(NativeEventPosition::new(x, y));
        self
    }

    pub const fn repeat(mut self, repeat: bool) -> Self {
        self.repeat = repeat;
        self
    }

    pub const fn click_count(mut self, count: u8) -> Self {
        self.click_count = count;
        self
    }

    pub const fn handled_activation(mut self, handled: bool) -> Self {
        self.handled_activation = handled;
        self
    }

    pub fn is_empty(&self) -> bool {
        self.modality == NativeInputModality::Unknown
            && self.modifiers.is_empty()
            && self.position.is_none()
            && self.delta.is_none()
            && !self.repeat
            && self.click_count == 0
            && !self.handled_activation
    }
}

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero(value: &u8) -> bool {
    *value == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_context_serializes_without_details() {
        assert_eq!(
            serde_json::to_value(NativeEventContext::new()).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn event_context_round_trips_interaction_details() {
        let context = NativeEventContext::new()
            .modality(NativeInputModality::Pen)
            .modifiers(NativeKeyModifiers::new().shift(true))
            .position(12.5, 8.0)
            .delta(-1.5, 2.0)
            .repeat(true)
            .click_count(2)
            .handled_activation(true);

        let json = serde_json::to_value(context).unwrap();
        assert_eq!(json["modality"], "pen");
        assert_eq!(json["modifiers"]["shift"], true);
        assert_eq!(json["position"]["x"], 12.5);
        assert_eq!(json["delta"]["x"], -1.5);
        assert_eq!(json["delta"]["y"], 2.0);
        assert_eq!(json["repeat"], true);
        assert_eq!(json["clickCount"], 2);
        assert_eq!(json["handledActivation"], true);
        assert_eq!(
            serde_json::from_value::<NativeEventContext>(json).unwrap(),
            context
        );
    }

    #[test]
    fn modality_capabilities_match_native_interaction_semantics() {
        assert!(NativeInputModality::Mouse.supports_hover());
        assert!(NativeInputModality::Pen.supports_hover());
        assert!(!NativeInputModality::Touch.supports_hover());
        assert!(NativeInputModality::Keyboard.shows_focus_ring());
        assert!(NativeInputModality::Virtual.shows_focus_ring());
        assert!(!NativeInputModality::Mouse.shows_focus_ring());
    }
}
