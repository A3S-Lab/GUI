use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::native::NativeProps;
use crate::platform_host::{PlatformElementId, PlatformPoint, PlatformPointerId};

use super::drag_drop_collection::SelfDrawnCollectionDropTarget;
use super::interaction::SelfDrawnEventContext;

pub(super) const DEFAULT_DROP_ACTIVATE_THRESHOLD_MICROS: u64 = 800_000;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Portable outcome negotiated between a drag source and drop target.
pub enum SelfDrawnDropOperation {
    Copy,
    Move,
    Link,
    #[default]
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
#[non_exhaustive]
/// One portable item transferred through a self-drawn drag session.
///
/// Text items retain every representation supplied by the source. File and
/// directory variants can be added without changing the surrounding event
/// context once external platform transfer lands.
pub enum SelfDrawnDropItem {
    Text {
        types: Vec<String>,
        formats: BTreeMap<String, String>,
    },
}

impl SelfDrawnDropItem {
    /// Creates a text item and derives its stable type list from the formats.
    pub fn text(formats: BTreeMap<String, String>) -> Self {
        let formats = formats
            .into_iter()
            .filter_map(|(format, value)| {
                let format = format.trim();
                (!format.is_empty()).then(|| (format.to_string(), value))
            })
            .collect::<BTreeMap<_, _>>();
        let types = formats.keys().cloned().collect();
        Self::Text { types, formats }
    }

    pub fn kind(&self) -> &'static str {
        match self {
            Self::Text { .. } => "text",
        }
    }

    pub fn types(&self) -> &[String] {
        match self {
            Self::Text { types, .. } => types,
        }
    }

    pub fn formats(&self) -> &BTreeMap<String, String> {
        match self {
            Self::Text { formats, .. } => formats,
        }
    }

    pub fn get_text(&self, format: &str) -> Option<&str> {
        match self {
            Self::Text { formats, .. } => formats.get(format).map(String::as_str),
        }
    }

    fn matches(&self, patterns: &[String]) -> bool {
        accepts_types(patterns, self.types())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
/// Typed transfer details attached to self-drawn drag and drop callbacks.
pub struct SelfDrawnDragContext {
    pub types: Vec<String>,
    pub value: Option<String>,
    pub items: Vec<SelfDrawnDropItem>,
    pub dragging_keys: Vec<String>,
    pub allowed_operations: Vec<SelfDrawnDropOperation>,
    pub drop_operation: SelfDrawnDropOperation,
    pub target: Option<SelfDrawnCollectionDropTarget>,
    pub is_internal: bool,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDragSource {
    pub(super) types: Vec<String>,
    pub(super) value: Option<String>,
    pub(super) items: Vec<SelfDrawnDropItem>,
    pub(super) allowed_operations: Vec<SelfDrawnDropOperation>,
    pub(super) collection: Option<PlatformElementId>,
    pub(super) dragging_keys: Vec<String>,
    pub(super) dragging_nodes: Vec<PlatformElementId>,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDropTarget {
    accepted_types: Vec<String>,
    requested_operation: Option<SelfDrawnDropOperation>,
    get_drop_operation_policy: Option<String>,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDragCandidate {
    pub(super) source: SelfDrawnDragSource,
    pub(super) start_position: PlatformPoint,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDragSession {
    pub(super) source: PlatformElementId,
    pub(super) pointer: Option<PlatformPointerId>,
    pub(super) types: Vec<String>,
    pub(super) value: Option<String>,
    pub(super) items: Vec<SelfDrawnDropItem>,
    pub(super) allowed_operations: Vec<SelfDrawnDropOperation>,
    pub(super) source_collection: Option<PlatformElementId>,
    pub(super) dragging_keys: Vec<String>,
    pub(super) dragging_nodes: Vec<PlatformElementId>,
    pub(super) current_target: Option<PlatformElementId>,
    pub(super) current_collection: Option<PlatformElementId>,
    pub(super) current_collection_target: Option<SelfDrawnCollectionDropTarget>,
    pub(super) current_item_indices: Vec<usize>,
    pub(super) current_operation: SelfDrawnDropOperation,
    pub(super) last_position: Option<PlatformPoint>,
    pub(super) drop_activation: Option<SelfDrawnDropActivationTracking>,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDropActivationTracking {
    pub(super) deadline_micros: u64,
    pub(super) target: PlatformElementId,
    pub(super) collection: Option<PlatformElementId>,
    pub(super) collection_target: Option<SelfDrawnCollectionDropTarget>,
    pub(super) context: SelfDrawnEventContext,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SelfDrawnMatchedDropTarget {
    pub(super) id: PlatformElementId,
    pub(super) collection: Option<PlatformElementId>,
    pub(super) collection_target: Option<SelfDrawnCollectionDropTarget>,
    pub(super) operation: SelfDrawnDropOperation,
    pub(super) item_indices: Vec<usize>,
}

impl SelfDrawnDragSource {
    pub(super) fn from_props(props: &NativeProps, style_requires_drag: bool) -> Option<Self> {
        let has_event = ["onDragStart", "onDragMove", "onDragEnd"]
            .into_iter()
            .any(|name| event(props, name).is_some());
        let draggable = props.draggable.as_deref().is_some_and(truthy)
            || attribute(props, &["draggable"]).is_some_and(truthy);
        let type_value = attribute(props, &["dragType", "data-drag-type"]);
        let value = attribute(props, &["dragValue", "data-drag-value"]).map(str::to_string);
        let item_value = attribute(props, &["dragItems", "data-drag-items"]);
        if !draggable
            && !has_event
            && !style_requires_drag
            && type_value.is_none()
            && value.is_none()
            && item_value.is_none()
        {
            return None;
        }
        let mut types = type_value.map(parse_tokens).unwrap_or_default();
        if types.is_empty() && value.is_some() {
            types.push("text/plain".to_string());
        }
        let items = item_value
            .and_then(parse_drag_items)
            .unwrap_or_else(|| legacy_drag_items(&types, value.as_deref()));
        if !items.is_empty() {
            types = item_types(&items);
        }
        let mut allowed_operations = Self::allowed_operations_from_props(props).unwrap_or_default();
        if allowed_operations.is_empty() {
            allowed_operations = vec![
                SelfDrawnDropOperation::Copy,
                SelfDrawnDropOperation::Move,
                SelfDrawnDropOperation::Link,
            ];
        }
        Some(Self {
            types,
            value,
            items,
            allowed_operations,
            collection: None,
            dragging_keys: Vec::new(),
            dragging_nodes: Vec::new(),
        })
    }

    pub(super) fn allowed_operations_from_props(
        props: &NativeProps,
    ) -> Option<Vec<SelfDrawnDropOperation>> {
        attribute(
            props,
            &["allowedDropOperations", "data-allowed-drop-operations"],
        )
        .map(parse_operations)
        .filter(|operations| !operations.is_empty())
    }
}

impl SelfDrawnDropTarget {
    pub(super) fn from_props(props: &NativeProps, style_requires_target: bool) -> Option<Self> {
        let has_event = [
            "onDrop",
            "onRootDrop",
            "onItemDrop",
            "onInsert",
            "onReorder",
            "onCollectionMove",
            "onDropEnter",
            "onDropMove",
            "onDropActivate",
            "onDropExit",
            "onDragEnter",
            "onDragLeave",
        ]
        .into_iter()
        .any(|name| event(props, name).is_some());
        let accepted = attribute(props, &["acceptedDragTypes", "data-accepted-drag-types"]);
        let operation = attribute(
            props,
            &["dropOperation", "dropEffect", "data-drop-operation"],
        );
        let get_drop_operation_policy = attribute(
            props,
            &["getDropOperation", "data-get-drop-operation-policy"],
        );
        if !has_event
            && !style_requires_target
            && accepted.is_none()
            && operation.is_none()
            && get_drop_operation_policy.is_none()
        {
            return None;
        }
        Some(Self {
            accepted_types: accepted.map(parse_tokens).unwrap_or_default(),
            requested_operation: operation.map(|operation| {
                parse_operation(operation).unwrap_or(SelfDrawnDropOperation::Cancel)
            }),
            get_drop_operation_policy: get_drop_operation_policy.map(str::to_string),
        })
    }

    #[cfg(test)]
    pub(super) fn operation_for(
        &self,
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
    ) -> SelfDrawnDropOperation {
        if !self.accepts(types) {
            return SelfDrawnDropOperation::Cancel;
        }
        self.default_operation_for(allowed_operations)
    }

    pub(super) fn accepts(&self, types: &[String]) -> bool {
        accepts_types(&self.accepted_types, types)
    }

    pub(super) fn default_operation_for(
        &self,
        allowed_operations: &[SelfDrawnDropOperation],
    ) -> SelfDrawnDropOperation {
        match self.requested_operation {
            Some(operation) if allowed_operations.contains(&operation) => operation,
            Some(_) => SelfDrawnDropOperation::Cancel,
            None => allowed_operations
                .iter()
                .copied()
                .find(|operation| *operation != SelfDrawnDropOperation::Cancel)
                .unwrap_or(SelfDrawnDropOperation::Cancel),
        }
    }

    pub(super) fn matching_item_indices(&self, items: &[SelfDrawnDropItem]) -> Vec<usize> {
        items
            .iter()
            .enumerate()
            .filter_map(|(index, item)| item.matches(&self.accepted_types).then_some(index))
            .collect()
    }

    pub(super) fn get_drop_operation_policy(&self) -> Option<&str> {
        self.get_drop_operation_policy.as_deref()
    }
}

impl SelfDrawnDragSession {
    pub(super) fn context(&self, operation: SelfDrawnDropOperation) -> SelfDrawnDragContext {
        self.context_with_items(operation, self.items.clone(), None, false)
    }

    pub(super) fn target_context(&self, operation: SelfDrawnDropOperation) -> SelfDrawnDragContext {
        let items = self
            .current_item_indices
            .iter()
            .filter_map(|index| self.items.get(*index).cloned())
            .collect();
        let is_internal =
            self.source_collection.is_some() && self.source_collection == self.current_collection;
        self.context_with_items(
            operation,
            items,
            self.current_collection_target.clone(),
            is_internal,
        )
    }

    fn context_with_items(
        &self,
        operation: SelfDrawnDropOperation,
        items: Vec<SelfDrawnDropItem>,
        target: Option<SelfDrawnCollectionDropTarget>,
        is_internal: bool,
    ) -> SelfDrawnDragContext {
        SelfDrawnDragContext {
            types: self.types.clone(),
            value: self.value.clone(),
            items,
            dragging_keys: self.dragging_keys.clone(),
            allowed_operations: self.allowed_operations.clone(),
            drop_operation: operation,
            target,
            is_internal,
        }
    }
}

pub(super) fn parse_drag_items(raw: &str) -> Option<Vec<SelfDrawnDropItem>> {
    let formats = serde_json::from_str::<Vec<BTreeMap<String, String>>>(raw).ok()?;
    if formats.is_empty() {
        return None;
    }
    let items = formats
        .into_iter()
        .map(SelfDrawnDropItem::text)
        .collect::<Vec<_>>();
    items
        .iter()
        .all(|item| !item.types().is_empty())
        .then_some(items)
}

fn legacy_drag_items(types: &[String], value: Option<&str>) -> Vec<SelfDrawnDropItem> {
    let Some(value) = value else {
        return Vec::new();
    };
    let formats = types
        .iter()
        .cloned()
        .map(|format| (format, value.to_string()))
        .collect();
    vec![SelfDrawnDropItem::text(formats)]
}

fn item_types(items: &[SelfDrawnDropItem]) -> Vec<String> {
    items
        .iter()
        .flat_map(SelfDrawnDropItem::types)
        .fold(Vec::new(), |mut types, drag_type| {
            if !types.contains(drag_type) {
                types.push(drag_type.clone());
            }
            types
        })
}

fn attribute<'a>(props: &'a NativeProps, names: &[&str]) -> Option<&'a str> {
    names.iter().find_map(|name| {
        props
            .web
            .attributes
            .get(*name)
            .or_else(|| props.metadata.get(*name))
            .map(String::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

fn event<'a>(props: &'a NativeProps, name: &str) -> Option<&'a str> {
    props
        .web
        .events
        .get(name)
        .map(String::as_str)
        .filter(|action| !action.is_empty())
}

fn truthy(value: &str) -> bool {
    value.is_empty()
        || value == "1"
        || value.eq_ignore_ascii_case("true")
        || value.eq_ignore_ascii_case("draggable")
}

fn parse_tokens(raw: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    for token in raw
        .split(',')
        .flat_map(|part| part.split_ascii_whitespace())
        .map(str::trim)
        .filter(|token| !token.is_empty())
    {
        if !tokens.iter().any(|existing| existing == token) {
            tokens.push(token.to_string());
        }
    }
    tokens
}

fn parse_operations(raw: &str) -> Vec<SelfDrawnDropOperation> {
    parse_tokens(raw)
        .into_iter()
        .filter_map(|token| parse_operation(&token))
        .filter(|operation| *operation != SelfDrawnDropOperation::Cancel)
        .fold(Vec::new(), |mut operations, operation| {
            if !operations.contains(&operation) {
                operations.push(operation);
            }
            operations
        })
}

fn parse_operation(raw: &str) -> Option<SelfDrawnDropOperation> {
    match raw.trim().to_ascii_lowercase().as_str() {
        "copy" => Some(SelfDrawnDropOperation::Copy),
        "move" => Some(SelfDrawnDropOperation::Move),
        "link" => Some(SelfDrawnDropOperation::Link),
        "cancel" | "none" => Some(SelfDrawnDropOperation::Cancel),
        _ => None,
    }
}

pub(super) fn accepts_types(patterns: &[String], types: &[String]) -> bool {
    if patterns.is_empty()
        || patterns
            .iter()
            .any(|pattern| matches!(pattern.as_str(), "all" | "*" | "*/*"))
    {
        return true;
    }
    patterns.iter().any(|pattern| {
        types
            .iter()
            .any(|drag_type| type_matches(pattern, drag_type))
    })
}

fn type_matches(pattern: &str, drag_type: &str) -> bool {
    if let Some(prefix) = pattern.strip_suffix("/*") {
        return drag_type
            .split_once('/')
            .is_some_and(|(kind, _)| kind.eq_ignore_ascii_case(prefix));
    }
    if pattern.contains('/') || drag_type.contains('/') {
        pattern.eq_ignore_ascii_case(drag_type)
    } else {
        pattern == drag_type
    }
}
