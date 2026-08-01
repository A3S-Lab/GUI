use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::accessibility::{
    AccessibilityDescriptionProps, AccessibilityRelationshipProps, AccessibilityRole,
    AccessibilityStateProps, AccessibilityStructureProps,
};
use crate::error::{GuiError, GuiResult};
use crate::geometry::Rect;
use crate::native::ValueSensitivity;

use super::text_input::PlatformTextRange;
use super::validation::{validate_non_negative_rect, validate_text};
use super::PlatformWindowId;

pub const MAX_PLATFORM_ACCESSIBILITY_NODES: usize = 65_536;
pub const MAX_PLATFORM_ACCESSIBILITY_DEPTH: usize = 256;
pub const MAX_PLATFORM_ACCESSIBILITY_BYTES: usize = 8 * 1024 * 1024;
pub const MAX_PLATFORM_ELEMENT_ID_BYTES: usize = 1024;
const MAX_PLATFORM_ACCESSIBILITY_VALUE_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PlatformElementId(String);

impl PlatformElementId {
    pub fn new(value: impl Into<String>) -> GuiResult<Self> {
        let value = value.into();
        validate_text("element id", &value, MAX_PLATFORM_ELEMENT_ID_BYTES, false)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn validate(&self) -> GuiResult<()> {
        validate_text("element id", &self.0, MAX_PLATFORM_ELEMENT_ID_BYTES, false)
    }
}

#[derive(Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformAccessibilityNode {
    pub id: PlatformElementId,
    pub role: AccessibilityRole,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default)]
    pub value_sensitivity: ValueSensitivity,
    #[serde(default)]
    pub relationships: AccessibilityRelationshipProps,
    #[serde(default)]
    pub description: AccessibilityDescriptionProps,
    #[serde(default)]
    pub structure: AccessibilityStructureProps,
    #[serde(default)]
    pub state: AccessibilityStateProps,
    #[serde(default)]
    pub disabled: bool,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub invalid: bool,
    #[serde(default)]
    pub read_only: bool,
    #[serde(default)]
    pub multiple: bool,
    #[serde(default)]
    pub focused: bool,
    #[serde(default)]
    pub selected: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checked: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expanded: Option<bool>,
    pub logical_bounds: Rect,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<PlatformAccessibilityNode>,
}

impl PlatformAccessibilityNode {
    pub fn new(id: PlatformElementId, role: AccessibilityRole, logical_bounds: Rect) -> Self {
        Self {
            id,
            role,
            label: None,
            value: None,
            value_sensitivity: ValueSensitivity::Public,
            relationships: AccessibilityRelationshipProps::default(),
            description: AccessibilityDescriptionProps::default(),
            structure: AccessibilityStructureProps::default(),
            state: AccessibilityStateProps::default(),
            disabled: false,
            required: false,
            invalid: false,
            read_only: false,
            multiple: false,
            focused: false,
            selected: false,
            checked: None,
            expanded: None,
            logical_bounds,
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: Self) -> Self {
        self.children.push(child);
        self
    }
}

impl std::fmt::Debug for PlatformAccessibilityNode {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut description = self.description.clone();
        if self.value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        formatter
            .debug_struct("PlatformAccessibilityNode")
            .field("id", &self.id)
            .field("role", &self.role)
            .field("label", &self.label)
            .field(
                "value",
                &self.value_sensitivity.redact(self.value.as_deref()),
            )
            .field("value_sensitivity", &self.value_sensitivity)
            .field("relationships", &self.relationships)
            .field("description", &description)
            .field("structure", &self.structure)
            .field("state", &self.state)
            .field("disabled", &self.disabled)
            .field("required", &self.required)
            .field("invalid", &self.invalid)
            .field("read_only", &self.read_only)
            .field("multiple", &self.multiple)
            .field("focused", &self.focused)
            .field("selected", &self.selected)
            .field("checked", &self.checked)
            .field("expanded", &self.expanded)
            .field("logical_bounds", &self.logical_bounds)
            .field("children", &self.children)
            .finish()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PlatformAccessibilityNodeWire<'a> {
    id: &'a PlatformElementId,
    role: AccessibilityRole,
    #[serde(skip_serializing_if = "Option::is_none")]
    label: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<&'a str>,
    #[serde(skip_serializing_if = "ValueSensitivity::is_public")]
    value_sensitivity: ValueSensitivity,
    relationships: &'a AccessibilityRelationshipProps,
    description: AccessibilityDescriptionProps,
    structure: &'a AccessibilityStructureProps,
    state: &'a AccessibilityStateProps,
    disabled: bool,
    required: bool,
    invalid: bool,
    read_only: bool,
    multiple: bool,
    focused: bool,
    selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    checked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expanded: Option<bool>,
    logical_bounds: Rect,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    children: &'a Vec<PlatformAccessibilityNode>,
}

impl Serialize for PlatformAccessibilityNode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut description = self.description.clone();
        if self.value_sensitivity.is_sensitive() {
            description.value_text = None;
        }
        PlatformAccessibilityNodeWire {
            id: &self.id,
            role: self.role,
            label: self.label.as_deref(),
            value: self.value_sensitivity.redact(self.value.as_deref()),
            value_sensitivity: self.value_sensitivity,
            relationships: &self.relationships,
            description,
            structure: &self.structure,
            state: &self.state,
            disabled: self.disabled,
            required: self.required,
            invalid: self.invalid,
            read_only: self.read_only,
            multiple: self.multiple,
            focused: self.focused,
            selected: self.selected,
            checked: self.checked,
            expanded: self.expanded,
            logical_bounds: self.logical_bounds,
            children: &self.children,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformAccessibilitySnapshot {
    pub window: PlatformWindowId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<PlatformAccessibilityNode>,
}

impl std::fmt::Debug for PlatformAccessibilitySnapshot {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PlatformAccessibilitySnapshot")
            .field("window", &self.window)
            .field("root", &self.root)
            .finish()
    }
}

impl PlatformAccessibilitySnapshot {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        let Some(root) = self.root.as_ref() else {
            return Ok(());
        };

        let mut node_ids = BTreeSet::new();
        let mut node_count = 0usize;
        validate_node(root, 1, &mut node_count, &mut node_ids)?;

        let serialized = serde_json::to_vec(root).map_err(|error| {
            GuiError::host(format!(
                "platform host accessibility snapshot could not be encoded: {error}"
            ))
        })?;
        if serialized.len() > MAX_PLATFORM_ACCESSIBILITY_BYTES {
            return Err(GuiError::host(format!(
                "platform host accessibility snapshot exceeds the {}-byte limit",
                MAX_PLATFORM_ACCESSIBILITY_BYTES
            )));
        }

        Ok(())
    }
}

fn validate_node(
    node: &PlatformAccessibilityNode,
    depth: usize,
    count: &mut usize,
    ids: &mut BTreeSet<PlatformElementId>,
) -> GuiResult<()> {
    if depth > MAX_PLATFORM_ACCESSIBILITY_DEPTH {
        return Err(GuiError::host(format!(
            "platform host accessibility tree exceeds the {}-level depth limit",
            MAX_PLATFORM_ACCESSIBILITY_DEPTH
        )));
    }
    *count += 1;
    if *count > MAX_PLATFORM_ACCESSIBILITY_NODES {
        return Err(GuiError::host(format!(
            "platform host accessibility tree exceeds the {}-node limit",
            MAX_PLATFORM_ACCESSIBILITY_NODES
        )));
    }
    node.id.validate()?;
    if !ids.insert(node.id.clone()) {
        return Err(GuiError::host(format!(
            "platform host accessibility tree contains duplicate element id {:?}",
            node.id.as_str()
        )));
    }
    validate_non_negative_rect("accessibility bounds", node.logical_bounds)?;
    if node.value_sensitivity == ValueSensitivity::Sensitive
        && (node.value.is_some() || node.description.value_text.is_some())
    {
        return Err(GuiError::host(
            "platform host accessibility snapshots cannot retain sensitive values",
        ));
    }
    for child in &node.children {
        validate_node(child, depth + 1, count, ids)?;
    }
    Ok(())
}

#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PlatformAccessibilityActionKind {
    Focus,
    Press,
    Increment,
    Decrement,
    Dismiss,
    ScrollIntoView,
    SetValue {
        value: String,
        #[serde(default)]
        sensitivity: ValueSensitivity,
    },
    SetTextSelection {
        range: PlatformTextRange,
    },
}

impl std::fmt::Debug for PlatformAccessibilityActionKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Focus => formatter.write_str("Focus"),
            Self::Press => formatter.write_str("Press"),
            Self::Increment => formatter.write_str("Increment"),
            Self::Decrement => formatter.write_str("Decrement"),
            Self::Dismiss => formatter.write_str("Dismiss"),
            Self::ScrollIntoView => formatter.write_str("ScrollIntoView"),
            Self::SetValue { value, sensitivity } => formatter
                .debug_struct("SetValue")
                .field("has_value", &true)
                .field("value_bytes", &value.len())
                .field("sensitivity", sensitivity)
                .finish(),
            Self::SetTextSelection { range } => formatter
                .debug_struct("SetTextSelection")
                .field("range", range)
                .finish(),
        }
    }
}

impl PlatformAccessibilityActionKind {
    fn validate(&self) -> GuiResult<()> {
        match self {
            Self::SetValue { value, .. } => validate_text(
                "accessibility action value",
                value,
                MAX_PLATFORM_ACCESSIBILITY_VALUE_BYTES,
                true,
            ),
            Self::SetTextSelection { range } => range.validate("accessibility text selection"),
            _ => Ok(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PlatformAccessibilityAction {
    pub window: PlatformWindowId,
    pub node: PlatformElementId,
    pub action: PlatformAccessibilityActionKind,
}

impl PlatformAccessibilityAction {
    pub fn validate(&self) -> GuiResult<()> {
        self.window.validate()?;
        self.node.validate()?;
        self.action.validate()
    }
}
