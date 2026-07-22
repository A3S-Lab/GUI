use serde::{Deserialize, Serialize};

use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::NativeBackendKind;
use crate::renderer::MountedNodeSnapshot;

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
    AutoFocus,
    ProgrammaticFocus,
    Selection,
    MultipleSelectionSnapshot,
    Locale,
    Direction,
    AccessibilityRole,
    AccessibilityRelationships,
    AccessibilityState,
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

fn set_role_capability(
    role_overrides: &mut Vec<NativeRoleCapabilities>,
    role: NativeRole,
    feature: NativeCapabilityFeature,
    support: CapabilitySupport,
    note: Option<&'static str>,
) {
    let role_capabilities = if let Some(index) = role_overrides
        .iter()
        .position(|capabilities| capabilities.role == role)
    {
        &mut role_overrides[index]
    } else {
        let index = role_overrides.len();
        role_overrides.push(NativeRoleCapabilities {
            role,
            features: Vec::new(),
        });
        &mut role_overrides[index]
    };
    let capability = NativeFeatureCapability::new(feature, support, note);
    if let Some(existing) = role_capabilities
        .features
        .iter_mut()
        .find(|existing| existing.feature == feature)
    {
        *existing = capability;
    } else {
        role_capabilities.features.push(capability);
    }
}

fn roles_without_generic_event_source(backend: NativeBackendKind) -> Vec<NativeRole> {
    match backend {
        NativeBackendKind::AppKit => vec![
            NativeRole::Window,
            NativeRole::Dialog,
            NativeRole::Popover,
            NativeRole::Menu,
            NativeRole::MenuItem,
            NativeRole::Tab,
        ],
        NativeBackendKind::Gtk4 => vec![NativeRole::MenuItem],
        NativeBackendKind::WinUI => vec![NativeRole::Window],
        NativeBackendKind::Headless => Vec::new(),
    }
}

impl NativeCapabilities {
    pub fn for_backend(backend: NativeBackendKind) -> Self {
        use CapabilitySupport::{Native, Portable, Unsupported};
        use NativeCapabilityFeature as Feature;

        let headless = backend == NativeBackendKind::Headless;
        let features = vec![
            NativeFeatureCapability::new(
                Feature::Press,
                if headless { Portable } else { Unsupported },
                Some(if headless {
                    "headless dispatch models press without an OS event source"
                } else {
                    "full pointer and keyboard activation is role-specific"
                }),
            ),
            NativeFeatureCapability::new(
                Feature::PressLifecycle,
                Unsupported,
                Some("full pointer and keyboard press lifecycle is role-specific"),
            ),
            NativeFeatureCapability::new(
                Feature::LongPress,
                if headless { Portable } else { Native },
                headless.then_some("headless dispatch models long press without an OS timer"),
            ),
            NativeFeatureCapability::new(
                Feature::Move,
                if headless { Portable } else { Native },
                headless.then_some("headless dispatch models movement without an OS input source"),
            ),
            NativeFeatureCapability::new(
                Feature::InputModality,
                if headless { Unsupported } else { Native },
                headless.then_some("headless events have no OS input source"),
            ),
            NativeFeatureCapability::new(
                Feature::Hover,
                if headless { Unsupported } else { Native },
                headless.then_some("headless dispatch has no pointing-device hover source"),
            ),
            NativeFeatureCapability::new(
                Feature::FocusEvents,
                if headless { Portable } else { Unsupported },
                (!headless).then_some("native focusability is role-specific"),
            ),
            NativeFeatureCapability::new(
                Feature::AutoFocus,
                if headless { Portable } else { Unsupported },
                (!headless).then_some("native focusability is role-specific"),
            ),
            NativeFeatureCapability::new(
                Feature::ProgrammaticFocus,
                if headless { Portable } else { Unsupported },
                (!headless).then_some("programmatic OS focus is role-specific"),
            ),
            NativeFeatureCapability::new(
                Feature::Selection,
                if headless { Portable } else { Unsupported },
                (!headless).then_some("native selection notifications are role-specific"),
            ),
            NativeFeatureCapability::new(
                Feature::MultipleSelectionSnapshot,
                Portable,
                Some("the runtime accumulates stable keys when adapters emit scalar selection"),
            ),
            NativeFeatureCapability::new(
                Feature::Locale,
                Portable,
                Some("locale inherits in the runtime but native locale setters are incomplete"),
            ),
            NativeFeatureCapability::new(
                Feature::Direction,
                Portable,
                Some("direction inherits in the runtime but native direction setters are incomplete"),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityRole,
                Portable,
                Some("semantic roles are projected, but native role overrides are incomplete"),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityRelationships,
                Portable,
                Some("relationships are present in the IR and headless tree but native setters are incomplete"),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityState,
                Portable,
                Some("state is present in the IR and headless tree but native setters are incomplete"),
            ),
        ];
        let mut role_overrides = Vec::new();
        if !headless {
            let mut activation_roles = vec![
                NativeRole::Button,
                NativeRole::DisclosureSummary,
                NativeRole::Link,
                NativeRole::ImageMapArea,
                NativeRole::ListBoxItem,
                NativeRole::TreeItem,
            ];
            if backend == NativeBackendKind::WinUI {
                activation_roles.push(NativeRole::MenuItem);
            }
            for role in activation_roles {
                set_role_capability(&mut role_overrides, role, Feature::Press, Native, None);
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::PressLifecycle,
                    Native,
                    None,
                );
            }

            for role in [
                NativeRole::Button,
                NativeRole::DisclosureSummary,
                NativeRole::Link,
                NativeRole::ImageMapArea,
                NativeRole::TextField,
                NativeRole::Checkbox,
                NativeRole::Switch,
                NativeRole::Radio,
                NativeRole::Select,
                NativeRole::ComboBox,
                NativeRole::ListBox,
                NativeRole::Tree,
                NativeRole::Slider,
            ] {
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::FocusEvents,
                    Native,
                    None,
                );
                set_role_capability(&mut role_overrides, role, Feature::AutoFocus, Native, None);
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::ProgrammaticFocus,
                    Native,
                    None,
                );
            }
            let item_focus_roles = match backend {
                NativeBackendKind::AppKit | NativeBackendKind::Gtk4 => {
                    vec![NativeRole::ListBoxItem, NativeRole::TreeItem]
                }
                NativeBackendKind::WinUI => vec![
                    NativeRole::MenuItem,
                    NativeRole::ListBoxItem,
                    NativeRole::TreeItem,
                    NativeRole::Tab,
                ],
                NativeBackendKind::Headless => Vec::new(),
            };
            for role in item_focus_roles {
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::FocusEvents,
                    Native,
                    None,
                );
                set_role_capability(&mut role_overrides, role, Feature::AutoFocus, Native, None);
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::ProgrammaticFocus,
                    Native,
                    None,
                );
            }

            for role in [
                NativeRole::Select,
                NativeRole::ComboBox,
                NativeRole::ListBox,
                NativeRole::Tree,
                NativeRole::Tabs,
                NativeRole::TabList,
            ] {
                set_role_capability(&mut role_overrides, role, Feature::Selection, Native, None);
            }

            if backend != NativeBackendKind::WinUI {
                set_role_capability(
                    &mut role_overrides,
                    NativeRole::MenuItem,
                    Feature::Press,
                    Native,
                    Some("native menu activation emits the terminal press only"),
                );
            }

            for role in roles_without_generic_event_source(backend) {
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::InputModality,
                    Unsupported,
                    Some("this role has no mounted generic native event source"),
                );
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::Hover,
                    Unsupported,
                    Some("this role has no mounted generic native event source"),
                );
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::LongPress,
                    Unsupported,
                    Some("this role has no mounted generic native event source"),
                );
                set_role_capability(
                    &mut role_overrides,
                    role,
                    Feature::Move,
                    Unsupported,
                    Some("this role has no mounted generic native event source"),
                );
            }
        }

        Self {
            ir_version: NATIVE_IR_VERSION,
            backend,
            features,
            role_overrides,
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
                requested_features(&node.props)
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
    for feature in requested_features(&element.props) {
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

fn requested_features(props: &NativeProps) -> Vec<NativeCapabilityFeature> {
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
    if props.explicit_role.is_some() {
        features.push(Feature::AccessibilityRole);
    }
    if props.accessibility_relationships != Default::default() {
        features.push(Feature::AccessibilityRelationships);
    }
    if props.accessibility_state != Default::default() {
        features.push(Feature::AccessibilityState);
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
