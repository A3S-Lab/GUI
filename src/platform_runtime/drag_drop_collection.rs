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
    low_level_drop: Option<String>,
    root_drop: Option<String>,
    item_drop: Option<String>,
    insert: Option<String>,
    reorder: Option<String>,
    move_within: Option<String>,
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
        let low_level_drop = event_action(props, "onDrop");
        let root_drop = event_action(props, "onRootDrop");
        let item_drop = event_action(props, "onItemDrop");
        let insert = event_action(props, "onInsert");
        let reorder = event_action(props, "onReorder");
        let move_within = event_action(props, "onCollectionMove");
        if low_level_drop.is_none()
            && root_drop.is_none()
            && item_drop.is_none()
            && insert.is_none()
            && reorder.is_none()
            && move_within.is_none()
        {
            return None;
        }
        Some(Self {
            target,
            orientation: drop_orientation(props),
            low_level_drop,
            root_drop,
            item_drop,
            insert,
            reorder,
            move_within,
        })
    }

    pub(super) fn has_low_level_drop(&self) -> bool {
        self.low_level_drop.is_some()
    }

    pub(super) fn has_root_drop(&self) -> bool {
        self.root_drop.is_some()
    }

    pub(super) fn has_item_drop(&self) -> bool {
        self.item_drop.is_some()
    }

    pub(super) fn has_insert(&self) -> bool {
        self.insert.is_some()
    }

    pub(super) fn has_reorder(&self) -> bool {
        self.reorder.is_some()
    }

    pub(super) fn has_move(&self) -> bool {
        self.move_within.is_some()
    }

    pub(super) fn allows_root(&self, is_internal: bool) -> bool {
        self.has_low_level_drop() || (!is_internal && self.has_root_drop())
    }

    pub(super) fn allows_item(&self, is_internal: bool) -> bool {
        self.has_low_level_drop() || self.has_item_drop() || (is_internal && self.has_move())
    }

    pub(super) fn allows_insert(&self, is_internal: bool) -> bool {
        self.has_low_level_drop()
            || (!is_internal && self.has_insert())
            || (is_internal && (self.has_reorder() || self.has_move()))
    }

    pub(super) fn low_level_action(&self) -> Option<&str> {
        self.low_level_drop.as_deref()
    }

    pub(super) fn root_action(&self) -> Option<&str> {
        self.root_drop.as_deref()
    }

    pub(super) fn item_action(&self) -> Option<&str> {
        self.item_drop.as_deref()
    }

    pub(super) fn insert_action(&self) -> Option<&str> {
        self.insert.as_deref()
    }

    pub(super) fn reorder_action(&self) -> Option<&str> {
        self.reorder.as_deref()
    }

    pub(super) fn move_action(&self) -> Option<&str> {
        self.move_within.as_deref()
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

fn event_action(props: &NativeProps, name: &str) -> Option<String> {
    props
        .web
        .events
        .get(name)
        .map(|action| action.trim())
        .filter(|action| !action.is_empty())
        .map(str::to_string)
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
