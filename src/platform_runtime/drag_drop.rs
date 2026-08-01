use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::native::NativeProps;
use crate::platform_host::{PlatformElementId, PlatformPoint, PlatformPointerId};

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
    pub allowed_operations: Vec<SelfDrawnDropOperation>,
    pub drop_operation: SelfDrawnDropOperation,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDragSource {
    pub(super) types: Vec<String>,
    pub(super) value: Option<String>,
    pub(super) items: Vec<SelfDrawnDropItem>,
    pub(super) allowed_operations: Vec<SelfDrawnDropOperation>,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDropTarget {
    accepted_types: Vec<String>,
    requested_operation: Option<SelfDrawnDropOperation>,
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
    pub(super) current_target: Option<PlatformElementId>,
    pub(super) current_item_indices: Vec<usize>,
    pub(super) current_operation: SelfDrawnDropOperation,
    pub(super) last_position: Option<PlatformPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SelfDrawnMatchedDropTarget {
    pub(super) id: PlatformElementId,
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
        let mut allowed_operations = attribute(
            props,
            &["allowedDropOperations", "data-allowed-drop-operations"],
        )
        .map(parse_operations)
        .unwrap_or_default();
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
        })
    }
}

impl SelfDrawnDropTarget {
    pub(super) fn from_props(props: &NativeProps, style_requires_target: bool) -> Option<Self> {
        let has_event = [
            "onDrop",
            "onDropEnter",
            "onDropMove",
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
        if !has_event && !style_requires_target && accepted.is_none() && operation.is_none() {
            return None;
        }
        Some(Self {
            accepted_types: accepted.map(parse_tokens).unwrap_or_default(),
            requested_operation: operation.map(|operation| {
                parse_operation(operation).unwrap_or(SelfDrawnDropOperation::Cancel)
            }),
        })
    }

    pub(super) fn operation_for(
        &self,
        types: &[String],
        allowed_operations: &[SelfDrawnDropOperation],
    ) -> SelfDrawnDropOperation {
        if !accepts_types(&self.accepted_types, types) {
            return SelfDrawnDropOperation::Cancel;
        }
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
}

impl SelfDrawnDragSession {
    pub(super) fn context(&self, operation: SelfDrawnDropOperation) -> SelfDrawnDragContext {
        self.context_with_items(operation, self.items.clone())
    }

    pub(super) fn target_context(&self, operation: SelfDrawnDropOperation) -> SelfDrawnDragContext {
        let items = self
            .current_item_indices
            .iter()
            .filter_map(|index| self.items.get(*index).cloned())
            .collect();
        self.context_with_items(operation, items)
    }

    fn context_with_items(
        &self,
        operation: SelfDrawnDropOperation,
        items: Vec<SelfDrawnDropItem>,
    ) -> SelfDrawnDragContext {
        SelfDrawnDragContext {
            types: self.types.clone(),
            value: self.value.clone(),
            items,
            allowed_operations: self.allowed_operations.clone(),
            drop_operation: operation,
        }
    }
}

fn parse_drag_items(raw: &str) -> Option<Vec<SelfDrawnDropItem>> {
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

fn accepts_types(patterns: &[String], types: &[String]) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_matching_supports_multiple_queries_and_mime_wildcards() {
        assert!(accepts_types(
            &["image/*".to_string(), "application/json".to_string()],
            &["text/plain".to_string(), "image/png".to_string()]
        ));
        assert!(!accepts_types(
            &["image/*".to_string(), "application/json".to_string()],
            &["text/plain".to_string(), "application/pdf".to_string()]
        ));
        assert!(accepts_types(
            &["all".to_string()],
            &["custom-type".to_string()]
        ));
    }

    #[test]
    fn invalid_requested_operation_rejects_instead_of_falling_back() {
        let props = NativeProps::new().web(
            crate::web::WebProps::new()
                .attribute("data-accepted-drag-types", "text/plain")
                .attribute("data-drop-operation", "archive"),
        );
        let target = SelfDrawnDropTarget::from_props(&props, false).unwrap();

        assert_eq!(
            target.operation_for(
                &["text/plain".to_string()],
                &[SelfDrawnDropOperation::Copy, SelfDrawnDropOperation::Move,],
            ),
            SelfDrawnDropOperation::Cancel
        );
    }

    #[test]
    fn drag_item_wire_shape_is_typed_and_old_contexts_default_items() {
        let item = SelfDrawnDropItem::text(BTreeMap::from([
            ("text/plain".to_string(), "alpha".to_string()),
            ("text/html".to_string(), "<b>alpha</b>".to_string()),
        ]));
        let wire = serde_json::to_value(&item).unwrap();
        assert_eq!(wire["kind"], "text");
        assert_eq!(
            wire["types"],
            serde_json::json!(["text/html", "text/plain"])
        );
        assert_eq!(wire["formats"]["text/plain"], "alpha");

        let old = serde_json::from_value::<SelfDrawnDragContext>(serde_json::json!({
            "types": ["text/plain"],
            "value": "alpha",
            "allowedOperations": ["copy"],
            "dropOperation": "copy"
        }))
        .unwrap();
        assert!(old.items.is_empty());
    }

    #[test]
    fn encoded_drag_items_reject_non_text_or_empty_representations() {
        assert!(parse_drag_items(r#"[{"text/plain":1}]"#).is_none());
        assert!(parse_drag_items(r#"[{}]"#).is_none());
        assert!(parse_drag_items("[]").is_none());
    }
}
