use serde::{Deserialize, Serialize};

use crate::accessibility::accessibility_live_setting;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::NativeBackendKind;
use crate::renderer::MountedNodeSnapshot;

mod accessibility_features;
mod accessibility_structure;

pub const NATIVE_IR_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum NativeCapabilityFeature {
    Press,
    PressLifecycle,
    LongPress,
    Move,
    InputModality,
    Hover,
    FocusEvents,
    FocusWithin,
    AutoFocus,
    ProgrammaticFocus,
    Selection,
    MultipleSelectionSnapshot,
    Locale,
    Direction,
    AnchoredOverlayPosition,
    AccessibilityRole,
    AccessibilityName,
    AccessibilityDescription,
    AccessibilityRoleDescription,
    AccessibilityKeyShortcuts,
    AccessibilityValueText,
    AccessibilityLabelledBy,
    AccessibilityDescribedBy,
    AccessibilityDetails,
    AccessibilityControls,
    AccessibilityOwns,
    AccessibilityFlowTo,
    AccessibilityErrorMessage,
    AccessibilityActiveDescendant,
    AccessibilityLevel,
    AccessibilityPositionInSet,
    AccessibilitySetSize,
    AccessibilityRowCount,
    AccessibilityRowIndex,
    AccessibilityRowSpan,
    AccessibilityColumnCount,
    AccessibilityColumnIndex,
    AccessibilityColumnSpan,
    AccessibilityRowIndexText,
    AccessibilityColumnIndexText,
    AccessibilitySort,
    AccessibilityHidden,
    AccessibilityAutocomplete,
    AccessibilityMultiline,
    AccessibilityCurrent,
    AccessibilityHasPopup,
    AccessibilityPressed,
    AccessibilityLiveRegion,
    AccessibilityBusy,
    AccessibilityModal,
    AccessibilityRelationships,
    AccessibilityStructure,
    AccessibilityState,
    AccessibilityAnnouncements,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CapabilitySupport {
    Unsupported,
    Portable,
    Native,
}

impl CapabilitySupport {
    pub fn is_available(self) -> bool {
        self != Self::Unsupported
    }

    pub fn is_native(self) -> bool {
        self == Self::Native
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeFeatureCapability {
    pub feature: NativeCapabilityFeature,
    pub support: CapabilitySupport,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl NativeFeatureCapability {
    fn new(
        feature: NativeCapabilityFeature,
        support: CapabilitySupport,
        note: impl Into<Option<&'static str>>,
    ) -> Self {
        Self {
            feature,
            support,
            note: note.into().map(str::to_string),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeRoleCapabilities {
    pub role: NativeRole,
    pub features: Vec<NativeFeatureCapability>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCapabilities {
    pub ir_version: u16,
    pub backend: NativeBackendKind,
    pub features: Vec<NativeFeatureCapability>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_overrides: Vec<NativeRoleCapabilities>,
}

impl Default for NativeCapabilities {
    fn default() -> Self {
        Self::for_backend(NativeBackendKind::Headless)
    }
}

const PORTABLE_FEATURES: &[NativeCapabilityFeature] = &[
    NativeCapabilityFeature::Press,
    NativeCapabilityFeature::PressLifecycle,
    NativeCapabilityFeature::LongPress,
    NativeCapabilityFeature::Move,
    NativeCapabilityFeature::InputModality,
    NativeCapabilityFeature::Hover,
    NativeCapabilityFeature::FocusEvents,
    NativeCapabilityFeature::FocusWithin,
    NativeCapabilityFeature::AutoFocus,
    NativeCapabilityFeature::ProgrammaticFocus,
    NativeCapabilityFeature::Selection,
    NativeCapabilityFeature::MultipleSelectionSnapshot,
    NativeCapabilityFeature::Locale,
    NativeCapabilityFeature::Direction,
    NativeCapabilityFeature::AnchoredOverlayPosition,
    NativeCapabilityFeature::AccessibilityRole,
    NativeCapabilityFeature::AccessibilityName,
    NativeCapabilityFeature::AccessibilityDescription,
    NativeCapabilityFeature::AccessibilityRoleDescription,
    NativeCapabilityFeature::AccessibilityKeyShortcuts,
    NativeCapabilityFeature::AccessibilityValueText,
    NativeCapabilityFeature::AccessibilityLabelledBy,
    NativeCapabilityFeature::AccessibilityDescribedBy,
    NativeCapabilityFeature::AccessibilityDetails,
    NativeCapabilityFeature::AccessibilityControls,
    NativeCapabilityFeature::AccessibilityOwns,
    NativeCapabilityFeature::AccessibilityFlowTo,
    NativeCapabilityFeature::AccessibilityErrorMessage,
    NativeCapabilityFeature::AccessibilityActiveDescendant,
    NativeCapabilityFeature::AccessibilityLevel,
    NativeCapabilityFeature::AccessibilityPositionInSet,
    NativeCapabilityFeature::AccessibilitySetSize,
    NativeCapabilityFeature::AccessibilityRowCount,
    NativeCapabilityFeature::AccessibilityRowIndex,
    NativeCapabilityFeature::AccessibilityRowSpan,
    NativeCapabilityFeature::AccessibilityColumnCount,
    NativeCapabilityFeature::AccessibilityColumnIndex,
    NativeCapabilityFeature::AccessibilityColumnSpan,
    NativeCapabilityFeature::AccessibilityRowIndexText,
    NativeCapabilityFeature::AccessibilityColumnIndexText,
    NativeCapabilityFeature::AccessibilitySort,
    NativeCapabilityFeature::AccessibilityHidden,
    NativeCapabilityFeature::AccessibilityAutocomplete,
    NativeCapabilityFeature::AccessibilityMultiline,
    NativeCapabilityFeature::AccessibilityCurrent,
    NativeCapabilityFeature::AccessibilityHasPopup,
    NativeCapabilityFeature::AccessibilityPressed,
    NativeCapabilityFeature::AccessibilityLiveRegion,
    NativeCapabilityFeature::AccessibilityBusy,
    NativeCapabilityFeature::AccessibilityModal,
    NativeCapabilityFeature::AccessibilityRelationships,
    NativeCapabilityFeature::AccessibilityStructure,
    NativeCapabilityFeature::AccessibilityState,
    NativeCapabilityFeature::AccessibilityAnnouncements,
];

impl NativeCapabilities {
    pub fn for_backend(backend: NativeBackendKind) -> Self {
        let features = PORTABLE_FEATURES
            .iter()
            .copied()
            .map(|feature| {
                NativeFeatureCapability::new(
                    feature,
                    CapabilitySupport::Portable,
                    Some(
                        "the self-drawn runtime owns this feature; no OS accessibility or input bridge is claimed",
                    ),
                )
            })
            .collect();

        Self {
            ir_version: NATIVE_IR_VERSION,
            backend,
            features,
            role_overrides: Vec::new(),
        }
    }

    pub fn capability(
        &self,
        feature: NativeCapabilityFeature,
        role: Option<NativeRole>,
    ) -> Option<&NativeFeatureCapability> {
        role.and_then(|role| {
            self.role_overrides
                .iter()
                .find(|capabilities| capabilities.role == role)
                .and_then(|capabilities| {
                    capabilities
                        .features
                        .iter()
                        .find(|capability| capability.feature == feature)
                })
        })
        .or_else(|| {
            self.features
                .iter()
                .find(|capability| capability.feature == feature)
        })
    }

    pub fn support(
        &self,
        feature: NativeCapabilityFeature,
        role: Option<NativeRole>,
    ) -> CapabilitySupport {
        self.capability(feature, role)
            .map(|capability| capability.support)
            .unwrap_or(CapabilitySupport::Unsupported)
    }

    pub fn audit_tree(&self, root: &NativeElement) -> Vec<NativeCapabilityIssue> {
        let mut issues = Vec::new();
        let mut path = vec![root.key.as_str().to_string()];
        audit_element(self, root, &mut path, &mut issues);
        issues
    }

    pub fn audit_mounted(&self, snapshot: &[MountedNodeSnapshot]) -> Vec<NativeCapabilityIssue> {
        snapshot
            .iter()
            .flat_map(|node| {
                requested_features(node.role, &node.props)
                    .into_iter()
                    .filter_map(|feature| self.issue(node.key.as_str(), node.role, feature))
            })
            .collect()
    }

    fn issue(
        &self,
        path: &str,
        role: NativeRole,
        feature: NativeCapabilityFeature,
    ) -> Option<NativeCapabilityIssue> {
        let capability = self.capability(feature, Some(role))?;
        (!capability.support.is_native()).then(|| NativeCapabilityIssue {
            path: path.to_string(),
            role,
            feature,
            support: capability.support,
            message: capability
                .note
                .clone()
                .unwrap_or_else(|| "native behavior is not fully implemented".to_string()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NativeCapabilityIssue {
    pub path: String,
    pub role: NativeRole,
    pub feature: NativeCapabilityFeature,
    pub support: CapabilitySupport,
    pub message: String,
}

pub trait CapabilityHost {
    fn native_capabilities(&self) -> NativeCapabilities;
}

fn audit_element(
    capabilities: &NativeCapabilities,
    element: &NativeElement,
    path: &mut Vec<String>,
    issues: &mut Vec<NativeCapabilityIssue>,
) {
    let display_path = path.join("/");
    for feature in requested_features(element.role, &element.props) {
        if let Some(issue) = capabilities.issue(&display_path, element.role, feature) {
            issues.push(issue);
        }
    }
    for child in &element.children {
        path.push(child.key.as_str().to_string());
        audit_element(capabilities, child, path, issues);
        path.pop();
    }
}

fn requested_features(role: NativeRole, props: &NativeProps) -> Vec<NativeCapabilityFeature> {
    use NativeCapabilityFeature as Feature;

    let mut features = Vec::new();
    if has_event(props, &["onPress", "onClick"]) || props.action.is_some() {
        features.push(Feature::Press);
    }
    if props
        .metadata
        .get(crate::selection::COLLECTION_ACTION_METADATA_KEY)
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    {
        features.push(Feature::Press);
        features.push(Feature::PressLifecycle);
        features.push(Feature::LongPress);
        features.push(Feature::InputModality);
    }
    if has_event(
        props,
        &["onPressStart", "onPressUp", "onPressEnd", "onPressChange"],
    ) {
        features.push(Feature::PressLifecycle);
        features.push(Feature::InputModality);
    }
    if has_event(
        props,
        &["onLongPressStart", "onLongPressEnd", "onLongPress"],
    ) {
        features.push(Feature::LongPress);
        features.push(Feature::InputModality);
    }
    if has_event(props, &["onMoveStart", "onMove", "onMoveEnd"]) {
        features.push(Feature::Move);
        features.push(Feature::InputModality);
    }
    if has_event(props, &["onHoverStart", "onHoverEnd", "onHoverChange"]) {
        features.push(Feature::Hover);
        features.push(Feature::InputModality);
    }
    if has_event(props, &["onFocus", "onBlur", "onFocusChange"]) {
        features.push(Feature::FocusEvents);
    }
    if has_event(
        props,
        &["onFocusWithin", "onBlurWithin", "onFocusWithinChange"],
    ) {
        features.push(Feature::FocusWithin);
    }
    if props.auto_focus {
        features.push(Feature::AutoFocus);
    }
    if has_event(props, &["onSelectionChange"]) {
        features.push(Feature::Selection);
    }
    if props.multiple
        || props
            .web
            .attributes
            .get("data-selection-mode")
            .is_some_and(|mode| mode.eq_ignore_ascii_case("multiple"))
    {
        features.push(Feature::MultipleSelectionSnapshot);
    }
    if props.lang.is_some() {
        features.push(Feature::Locale);
    }
    if props.dir.is_some() {
        features.push(Feature::Direction);
    }
    if props
        .metadata
        .get(crate::overlay_position::OVERLAY_POSITION_MARKER)
        .or_else(|| {
            props
                .web
                .attributes
                .get(crate::overlay_position::OVERLAY_POSITION_MARKER)
        })
        .is_some_and(|value| value.is_empty() || value.eq_ignore_ascii_case("true"))
    {
        features.push(Feature::AnchoredOverlayPosition);
    }
    if props.explicit_role.is_some() {
        features.push(Feature::AccessibilityRole);
    }
    if props.accessibility_label.is_some() && props.accessibility_label != props.label {
        features.push(Feature::AccessibilityName);
    }
    if props.accessibility_description.description.is_some() {
        features.push(Feature::AccessibilityDescription);
    }
    if props.accessibility_description.role_description.is_some() {
        features.push(Feature::AccessibilityRoleDescription);
    }
    if props.accessibility_description.key_shortcuts.is_some() {
        features.push(Feature::AccessibilityKeyShortcuts);
    }
    if props.accessibility_description.value_text.is_some() {
        features.push(Feature::AccessibilityValueText);
    }
    features.extend(accessibility_features::requested_features(
        &props.accessibility_relationships,
    ));
    features.extend(accessibility_structure::requested_features(
        &props.accessibility_structure,
    ));
    if props.accessibility_state.hidden.is_some() {
        features.push(Feature::AccessibilityHidden);
    }
    if props.accessibility_state.autocomplete.is_some() {
        features.push(Feature::AccessibilityAutocomplete);
    }
    if props.accessibility_state.multiline.is_some() {
        features.push(Feature::AccessibilityMultiline);
    }
    if props.accessibility_state.current.is_some() {
        features.push(Feature::AccessibilityCurrent);
    }
    if props.accessibility_state.has_popup.is_some() {
        features.push(Feature::AccessibilityHasPopup);
    }
    if props.accessibility_state.pressed.is_some() {
        features.push(Feature::AccessibilityPressed);
    }
    if props.accessibility_state.live.is_some()
        || props.accessibility_state.atomic.is_some()
        || props.accessibility_state.relevant.is_some()
    {
        features.push(Feature::AccessibilityLiveRegion);
    }
    if props.accessibility_state.busy.is_some() {
        features.push(Feature::AccessibilityBusy);
    }
    if props.accessibility_state.modal.is_some() {
        features.push(Feature::AccessibilityModal);
    }
    if props
        .metadata
        .get(crate::native::NUMBER_FIELD_ANNOUNCE_METADATA_KEY)
        .or_else(|| {
            props
                .web
                .attributes
                .get(crate::native::NUMBER_FIELD_ANNOUNCE_METADATA_KEY)
        })
        .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    {
        features.push(Feature::AccessibilityAnnouncements);
    }
    if accessibility_live_setting(role, props).priority().is_some() {
        features.push(Feature::AccessibilityAnnouncements);
    }
    features.sort_unstable();
    features.dedup();
    features
}

fn has_event(props: &NativeProps, names: &[&str]) -> bool {
    names.iter().any(|name| {
        props
            .web
            .events
            .get(*name)
            .is_some_and(|action| !action.is_empty())
    })
}

#[cfg(test)]
mod tests;
