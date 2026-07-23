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
        let mut features = vec![
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
                Feature::FocusWithin,
                Portable,
                Some("the runtime derives subtree boundaries from linked native focus transitions"),
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
                Feature::AnchoredOverlayPosition,
                Portable,
                Some(match backend {
                    NativeBackendKind::AppKit => {
                        "AppKit projects placement and offsets to NSPopover; collision flipping remains runtime-dependent"
                    }
                    NativeBackendKind::Gtk4 => {
                        "GTK4 projects placement and offsets to gtk::Popover; collision flipping remains runtime-dependent"
                    }
                    NativeBackendKind::WinUI => {
                        "WinUI ToolTip projects the placement target and signed offsets; exact side placement and collision flipping depend on WinUI"
                    }
                    NativeBackendKind::Headless => {
                        "headless mode records the typed anchor relationship without native geometry"
                    }
                }),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityRole,
                Portable,
                Some("semantic roles are projected, but native role overrides are incomplete"),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityName,
                if headless { Portable } else { Native },
                Some(if headless {
                    "headless mode retains the computed name without an OS accessibility object"
                } else {
                    "computed names use the backend's native accessibility-name property"
                }),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityDescription,
                if headless { Portable } else { Native },
                Some(match backend {
                    NativeBackendKind::AppKit => {
                        "descriptions use the native NSAccessibility help property"
                    }
                    NativeBackendKind::Gtk4 => {
                        "descriptions use the native GtkAccessible description property"
                    }
                    NativeBackendKind::WinUI => {
                        "descriptions use the native UI Automation help-text property"
                    }
                    NativeBackendKind::Headless => {
                        "headless mode retains descriptions without an OS accessibility object"
                    }
                }),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityRoleDescription,
                match backend {
                    NativeBackendKind::AppKit | NativeBackendKind::Gtk4 => Native,
                    NativeBackendKind::WinUI | NativeBackendKind::Headless => Portable,
                },
                Some(match backend {
                    NativeBackendKind::AppKit => {
                        "role descriptions use the native NSAccessibility role-description property"
                    }
                    NativeBackendKind::Gtk4 => {
                        "role descriptions use the native GtkAccessible role-description property"
                    }
                    NativeBackendKind::WinUI => {
                        "role descriptions remain in portable accessibility output because the current WinUI binding has no exact attached-property setter"
                    }
                    NativeBackendKind::Headless => {
                        "headless mode retains role descriptions without an OS accessibility object"
                    }
                }),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityKeyShortcuts,
                match backend {
                    NativeBackendKind::Gtk4 | NativeBackendKind::WinUI => Native,
                    NativeBackendKind::AppKit | NativeBackendKind::Headless => Portable,
                },
                Some(match backend {
                    NativeBackendKind::AppKit => {
                        "key shortcuts remain in portable accessibility output because NSAccessibility has no equivalent setter"
                    }
                    NativeBackendKind::Gtk4 => {
                        "key shortcuts use the native GtkAccessible key-shortcuts property"
                    }
                    NativeBackendKind::WinUI => {
                        "key shortcuts use the native UI Automation accelerator-key property"
                    }
                    NativeBackendKind::Headless => {
                        "headless mode retains key shortcuts without an OS accessibility object"
                    }
                }),
            ),
            NativeFeatureCapability::new(
                Feature::AccessibilityValueText,
                match backend {
                    NativeBackendKind::AppKit | NativeBackendKind::Gtk4 => Native,
                    NativeBackendKind::WinUI | NativeBackendKind::Headless => Portable,
                },
                Some(match backend {
                    NativeBackendKind::AppKit => {
                        "value text uses the native NSAccessibility value-description property"
                    }
                    NativeBackendKind::Gtk4 => {
                        "value text uses the native GtkAccessible value-text property"
                    }
                    NativeBackendKind::WinUI => {
                        "value text remains in portable accessibility output because WinUI has no generic attached-property override for a control pattern value"
                    }
                    NativeBackendKind::Headless => {
                        "headless mode retains value text without an OS accessibility object"
                    }
                }),
            ),
        ];
        features.extend(accessibility_features::capabilities(backend));
        features.extend(accessibility_structure::capabilities(backend));
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

            match backend {
                NativeBackendKind::AppKit => {
                    for role in [
                        NativeRole::ListBoxItem,
                        NativeRole::TreeItem,
                        NativeRole::Tab,
                    ] {
                        set_role_capability(
                            &mut role_overrides,
                            role,
                            Feature::AccessibilityName,
                            Portable,
                            Some(
                                "AppKit logical combo-box/list and tab items retain the computed name in portable accessibility output but do not expose an independent native accessibility-label setter",
                            ),
                        );
                        for feature in [
                            Feature::AccessibilityDescription,
                            Feature::AccessibilityRoleDescription,
                            Feature::AccessibilityKeyShortcuts,
                            Feature::AccessibilityValueText,
                        ] {
                            set_role_capability(
                                &mut role_overrides,
                                role,
                                feature,
                                Portable,
                                Some(
                                    "AppKit logical combo-box/list and tab items retain descriptive accessibility metadata in portable output but do not expose an independent native accessibility-property setter",
                                ),
                            );
                        }
                        for feature in [
                            Feature::AccessibilityHidden,
                            Feature::AccessibilityAutocomplete,
                            Feature::AccessibilityMultiline,
                            Feature::AccessibilityCurrent,
                            Feature::AccessibilityHasPopup,
                            Feature::AccessibilityPressed,
                            Feature::AccessibilityLiveRegion,
                            Feature::AccessibilityBusy,
                            Feature::AccessibilityModal,
                        ] {
                            set_role_capability(
                                &mut role_overrides,
                                role,
                                feature,
                                Portable,
                                Some(
                                    "AppKit logical combo-box/list and tab items retain accessibility state in portable output but do not expose an independent native state setter",
                                ),
                            );
                        }
                    }
                }
                NativeBackendKind::Gtk4 => {
                    set_role_capability(
                        &mut role_overrides,
                        NativeRole::MenuItem,
                        Feature::AccessibilityName,
                        Portable,
                        Some(
                            "GTK4 gio::MenuItem retains the computed name in portable accessibility output but has no independent GtkAccessible label property",
                        ),
                    );
                    for feature in [
                        Feature::AccessibilityDescription,
                        Feature::AccessibilityRoleDescription,
                        Feature::AccessibilityKeyShortcuts,
                        Feature::AccessibilityValueText,
                    ] {
                        set_role_capability(
                            &mut role_overrides,
                            NativeRole::MenuItem,
                            feature,
                            Portable,
                            Some(
                                "GTK4 gio::MenuItem retains descriptive accessibility metadata in portable output but has no independent GtkAccessible property",
                            ),
                        );
                    }
                    for feature in [
                        Feature::AccessibilityHidden,
                        Feature::AccessibilityAutocomplete,
                        Feature::AccessibilityMultiline,
                        Feature::AccessibilityCurrent,
                        Feature::AccessibilityHasPopup,
                        Feature::AccessibilityPressed,
                        Feature::AccessibilityLiveRegion,
                        Feature::AccessibilityBusy,
                        Feature::AccessibilityModal,
                    ] {
                        set_role_capability(
                            &mut role_overrides,
                            NativeRole::MenuItem,
                            feature,
                            Portable,
                            Some(
                                "GTK4 gio::MenuItem retains accessibility state in portable output but has no independent GtkAccessible state property",
                            ),
                        );
                    }
                    set_role_capability(
                        &mut role_overrides,
                        NativeRole::MenuItem,
                        Feature::AccessibilityAnnouncements,
                        Portable,
                        Some(
                            "GTK4 gio::MenuItem has no mounted GtkAccessible target for announcements",
                        ),
                    );
                }
                NativeBackendKind::WinUI => {
                    set_role_capability(
                        &mut role_overrides,
                        NativeRole::Window,
                        Feature::AccessibilityName,
                        Portable,
                        Some(
                            "the WinUI Window wrapper retains the computed name in portable accessibility output but is not a UIElement AutomationProperties target",
                        ),
                    );
                    for feature in [
                        Feature::AccessibilityDescription,
                        Feature::AccessibilityRoleDescription,
                        Feature::AccessibilityKeyShortcuts,
                        Feature::AccessibilityValueText,
                    ] {
                        set_role_capability(
                            &mut role_overrides,
                            NativeRole::Window,
                            feature,
                            Portable,
                            Some(
                                "the WinUI Window wrapper retains descriptive accessibility metadata in portable output but is not a UIElement AutomationProperties target",
                            ),
                        );
                    }
                    set_role_capability(
                        &mut role_overrides,
                        NativeRole::Dialog,
                        Feature::AccessibilityModal,
                        Native,
                        Some(
                            "WinUI ContentDialog exposes modality through its native automation peer",
                        ),
                    );
                }
                NativeBackendKind::Headless => {}
            }

            if backend == NativeBackendKind::AppKit {
                for role in [
                    NativeRole::Window,
                    NativeRole::Dialog,
                    NativeRole::Popover,
                    NativeRole::Menu,
                    NativeRole::MenuItem,
                    NativeRole::ListBoxItem,
                    NativeRole::TreeItem,
                    NativeRole::Tab,
                ] {
                    set_role_capability(
                        &mut role_overrides,
                        role,
                        Feature::AccessibilityLiveRegion,
                        Portable,
                        Some(
                            "this AppKit wrapper has no mounted NSView target for accessibility announcements",
                        ),
                    );
                    set_role_capability(
                        &mut role_overrides,
                        role,
                        Feature::AccessibilityAnnouncements,
                        Portable,
                        Some(
                            "this AppKit wrapper has no mounted NSView target for accessibility announcements",
                        ),
                    );
                }
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
        accessibility_features::add_wrapper_overrides(backend, &mut role_overrides);
        accessibility_structure::add_wrapper_overrides(backend, &mut role_overrides);

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
