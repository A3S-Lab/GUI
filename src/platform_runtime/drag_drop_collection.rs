use serde::{Deserialize, Serialize};

use crate::geometry::Orientation;
use crate::native::{NativeProps, NativeRole};

use super::drag_drop::SelfDrawnDropTarget;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Position of a collection drop relative to a stable item key.
pub enum SelfDrawnDropPosition {
    Before,
    #[default]
    On,
    After,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
/// React Aria-compatible target descriptor for a self-drawn collection drop.
pub enum SelfDrawnCollectionDropTarget {
    Root,
    Item {
        key: String,
        #[serde(rename = "dropPosition")]
        drop_position: SelfDrawnDropPosition,
    },
}

impl SelfDrawnCollectionDropTarget {
    pub fn key(&self) -> Option<&str> {
        match self {
            Self::Root => None,
            Self::Item { key, .. } => Some(key),
        }
    }

    pub fn drop_position(&self) -> Option<SelfDrawnDropPosition> {
        match self {
            Self::Root => None,
            Self::Item { drop_position, .. } => Some(*drop_position),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnCollectionDropConfig {
    pub(super) target: SelfDrawnDropTarget,
    pub(super) orientation: Orientation,
    low_level_drop: bool,
    allows_root: bool,
    allows_item: bool,
    allows_insert: bool,
}

impl SelfDrawnCollectionDropConfig {
    pub(super) fn from_props(
        props: &NativeProps,
        target: Option<SelfDrawnDropTarget>,
    ) -> Option<Self> {
        if !bool_attribute(props, "data-droppable-collection") {
            return None;
        }
        let target = target?;
        let low_level_drop = has_event(props, "onDrop");
        let allows_root = low_level_drop || has_event(props, "onRootDrop");
        let allows_item = low_level_drop || has_event(props, "onItemDrop");
        let allows_insert = low_level_drop || has_event(props, "onInsert");
        if !allows_root && !allows_item && !allows_insert {
            return None;
        }
        Some(Self {
            target,
            orientation: drop_orientation(props),
            low_level_drop,
            allows_root,
            allows_item,
            allows_insert,
        })
    }

    pub(super) fn allows_root(&self) -> bool {
        self.allows_root
    }

    pub(super) fn allows_item(&self) -> bool {
        self.allows_item
    }

    pub(super) fn allows_insert(&self) -> bool {
        self.allows_insert
    }

    pub(super) fn drop_event_name(&self, target: &SelfDrawnCollectionDropTarget) -> &'static str {
        if self.low_level_drop {
            return "onDrop";
        }
        match target {
            SelfDrawnCollectionDropTarget::Root => "onRootDrop",
            SelfDrawnCollectionDropTarget::Item {
                drop_position: SelfDrawnDropPosition::On,
                ..
            } => "onItemDrop",
            SelfDrawnCollectionDropTarget::Item { .. } => "onInsert",
        }
    }
}

pub(super) fn is_draggable_collection(props: &NativeProps) -> bool {
    bool_attribute(props, "data-draggable-collection")
}

pub(super) fn is_collection_container(role: NativeRole, props: &NativeProps) -> bool {
    is_draggable_collection(props)
        || bool_attribute(props, "data-droppable-collection")
        || matches!(
            role,
            NativeRole::ListBox | NativeRole::Tree | NativeRole::Table
        )
}

pub(super) fn collection_item_key(
    role: NativeRole,
    element_key: &str,
    props: &NativeProps,
) -> Option<String> {
    let marked = bool_attribute(props, "data-collection-drop-item");
    if !marked
        && !matches!(
            role,
            NativeRole::ListBoxItem | NativeRole::TreeItem | NativeRole::TableRow
        )
    {
        return None;
    }
    Some(
        attribute(props, "data-collection-key")
            .filter(|key| !key.is_empty())
            .unwrap_or(element_key)
            .to_string(),
    )
}

pub(super) fn drop_indicator_target(props: &NativeProps) -> Option<SelfDrawnCollectionDropTarget> {
    let marked = bool_attribute(props, "data-collection-drop-indicator")
        || attribute(props, "data-slot") == Some("drop-indicator");
    if !marked {
        return None;
    }
    let key = attribute(props, "data-drop-target-key")?.to_string();
    let drop_position = match attribute(props, "data-drop-position")? {
        value if value.eq_ignore_ascii_case("before") => SelfDrawnDropPosition::Before,
        value if value.eq_ignore_ascii_case("after") => SelfDrawnDropPosition::After,
        _ => return None,
    };
    Some(SelfDrawnCollectionDropTarget::Item { key, drop_position })
}

fn drop_orientation(props: &NativeProps) -> Orientation {
    match attribute(props, "data-drop-orientation") {
        Some(value) if value.eq_ignore_ascii_case("horizontal") => Orientation::Horizontal,
        _ => Orientation::Vertical,
    }
}

fn has_event(props: &NativeProps, name: &str) -> bool {
    props
        .web
        .events
        .get(name)
        .is_some_and(|action| !action.trim().is_empty())
}

fn bool_attribute(props: &NativeProps, name: &str) -> bool {
    attribute(props, name).is_some_and(|value| {
        value.is_empty()
            || value == "1"
            || value.eq_ignore_ascii_case("true")
            || value.eq_ignore_ascii_case(name)
    })
}

fn attribute<'a>(props: &'a NativeProps, name: &str) -> Option<&'a str> {
    props
        .web
        .attributes
        .get(name)
        .or_else(|| props.metadata.get(name))
        .map(String::as_str)
        .map(str::trim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collection_target_uses_the_react_aria_wire_shape() {
        let target = SelfDrawnCollectionDropTarget::Item {
            key: "alpha".to_string(),
            drop_position: SelfDrawnDropPosition::Before,
        };
        assert_eq!(
            serde_json::to_value(target).unwrap(),
            serde_json::json!({
                "type": "item",
                "key": "alpha",
                "dropPosition": "before"
            })
        );
        assert_eq!(
            serde_json::to_value(SelfDrawnCollectionDropTarget::Root).unwrap(),
            serde_json::json!({"type": "root"})
        );
    }
}
