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

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase")]
/// Typed transfer details attached to self-drawn drag and drop callbacks.
pub struct SelfDrawnDragContext {
    pub types: Vec<String>,
    pub value: Option<String>,
    pub allowed_operations: Vec<SelfDrawnDropOperation>,
    pub drop_operation: SelfDrawnDropOperation,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnDragSource {
    pub(super) types: Vec<String>,
    pub(super) value: Option<String>,
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
    pub(super) allowed_operations: Vec<SelfDrawnDropOperation>,
    pub(super) current_target: Option<PlatformElementId>,
    pub(super) current_operation: SelfDrawnDropOperation,
    pub(super) last_position: Option<PlatformPoint>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SelfDrawnMatchedDropTarget {
    pub(super) id: PlatformElementId,
    pub(super) operation: SelfDrawnDropOperation,
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
        if !draggable
            && !has_event
            && !style_requires_drag
            && type_value.is_none()
            && value.is_none()
        {
            return None;
        }
        let mut types = type_value.map(parse_tokens).unwrap_or_default();
        if types.is_empty() && value.is_some() {
            types.push("text/plain".to_string());
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
}

impl SelfDrawnDragSession {
    pub(super) fn context(&self, operation: SelfDrawnDropOperation) -> SelfDrawnDragContext {
        SelfDrawnDragContext {
            types: self.types.clone(),
            value: self.value.clone(),
            allowed_operations: self.allowed_operations.clone(),
            drop_operation: operation,
        }
    }
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
}
