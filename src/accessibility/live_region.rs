use super::AccessibilityAnnouncementPriority;
use crate::native::{NativeProps, NativeRole};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AccessibilityLiveSetting {
    Inherit,
    Off,
    Polite,
    Assertive,
}

impl AccessibilityLiveSetting {
    pub(crate) fn priority(self) -> Option<AccessibilityAnnouncementPriority> {
        match self {
            Self::Polite => Some(AccessibilityAnnouncementPriority::Polite),
            Self::Assertive => Some(AccessibilityAnnouncementPriority::Assertive),
            Self::Inherit | Self::Off => None,
        }
    }
}

pub(crate) fn accessibility_live_setting(
    role: NativeRole,
    props: &NativeProps,
) -> AccessibilityLiveSetting {
    if let Some(value) = props.accessibility_state.live.as_deref() {
        return match value.trim().to_ascii_lowercase().as_str() {
            "assertive" => AccessibilityLiveSetting::Assertive,
            "polite" => AccessibilityLiveSetting::Polite,
            "off" => AccessibilityLiveSetting::Off,
            _ => AccessibilityLiveSetting::Off,
        };
    }

    match normalized_explicit_role(props).as_deref() {
        Some("alert") => AccessibilityLiveSetting::Assertive,
        Some("log" | "status") => AccessibilityLiveSetting::Polite,
        Some("marquee" | "timer") => AccessibilityLiveSetting::Off,
        _ if role == NativeRole::Output => AccessibilityLiveSetting::Polite,
        _ if role == NativeRole::Marquee => AccessibilityLiveSetting::Off,
        _ => AccessibilityLiveSetting::Inherit,
    }
}

pub(crate) fn live_region_implicit_atomic(role: NativeRole, props: &NativeProps) -> bool {
    matches!(
        normalized_explicit_role(props).as_deref(),
        Some("alert" | "status")
    ) || role == NativeRole::Output
}

pub(crate) fn live_region_announces_on_initial_render(props: &NativeProps) -> bool {
    normalized_explicit_role(props).as_deref() == Some("alert")
}

fn normalized_explicit_role(props: &NativeProps) -> Option<String> {
    props
        .explicit_role
        .as_deref()
        .map(str::trim)
        .filter(|role| !role.is_empty())
        .map(str::to_ascii_lowercase)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_live_values_override_implicit_roles() {
        let timer = NativeProps::new().explicit_role("timer").live("assertive");
        assert_eq!(
            accessibility_live_setting(NativeRole::View, &timer),
            AccessibilityLiveSetting::Assertive
        );

        let alert = NativeProps::new().explicit_role("alert").live("off");
        assert_eq!(
            accessibility_live_setting(NativeRole::View, &alert),
            AccessibilityLiveSetting::Off
        );
    }

    #[test]
    fn live_roles_supply_wai_aria_implicit_values() {
        let status = NativeProps::new().explicit_role("status");
        assert_eq!(
            accessibility_live_setting(NativeRole::View, &status),
            AccessibilityLiveSetting::Polite
        );
        assert!(live_region_implicit_atomic(NativeRole::View, &status));

        let output = NativeProps::new();
        assert_eq!(
            accessibility_live_setting(NativeRole::Output, &output),
            AccessibilityLiveSetting::Polite
        );
        assert!(live_region_implicit_atomic(NativeRole::Output, &output));
    }
}
