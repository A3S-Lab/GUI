use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Orientation, Rect, Size};

use super::CollectionKey;

/// A collection's measured layout in content coordinates.
///
/// Native hosts provide this snapshot immediately before PageUp or PageDown
/// navigation. Custom hosts may provide the same information through
/// [`crate::host::NativeHost::measure_collection_layout`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionLayoutSnapshot {
    visible_rect: Rect,
    content_size: Size,
    #[serde(default)]
    item_rects: BTreeMap<CollectionKey, Rect>,
}

impl CollectionLayoutSnapshot {
    pub fn new(visible_rect: Rect, content_size: Size) -> Self {
        Self {
            visible_rect,
            content_size,
            item_rects: BTreeMap::new(),
        }
    }

    pub fn with_item_rect(mut self, key: impl Into<CollectionKey>, rect: Rect) -> Self {
        self.insert_item_rect(key, rect);
        self
    }

    pub fn insert_item_rect(&mut self, key: impl Into<CollectionKey>, rect: Rect) -> Option<Rect> {
        self.item_rects.insert(key.into(), rect)
    }

    pub const fn visible_rect(&self) -> Rect {
        self.visible_rect
    }

    pub const fn content_size(&self) -> Size {
        self.content_size
    }

    pub fn item_rect(&self, key: &CollectionKey) -> Option<Rect> {
        self.item_rects.get(key).copied()
    }

    pub fn item_rects(&self) -> &BTreeMap<CollectionKey, Rect> {
        &self.item_rects
    }

    pub fn validate(&self) -> GuiResult<()> {
        validate_rect("visible rectangle", self.visible_rect)?;
        validate_size("content size", self.content_size)?;
        for (key, rect) in &self.item_rects {
            validate_rect(&format!("item {key:?} rectangle"), *rect)?;
        }
        Ok(())
    }

    pub(crate) fn page_target(
        &self,
        ordered_keys: &[&CollectionKey],
        current_key: Option<&CollectionKey>,
        orientation: Orientation,
        direction: CollectionPageDirection,
    ) -> Option<CollectionKey> {
        let boundary = match direction {
            CollectionPageDirection::Above => ordered_keys.first(),
            CollectionPageDirection::Below => ordered_keys.last(),
        }
        .copied()
        .cloned();
        let current_key = current_key?;
        let current_index = ordered_keys
            .iter()
            .position(|candidate| *candidate == current_key)?;

        let (viewport_extent, content_extent) = match orientation {
            Orientation::Horizontal => (self.visible_rect.width, self.content_size.width),
            Orientation::Vertical => (self.visible_rect.height, self.content_size.height),
        };
        if content_extent <= viewport_extent {
            return boundary;
        }

        let current_rect = self.item_rect(current_key)?;
        let (current_start, current_extent) = rect_axis(current_rect, orientation);
        let target = match direction {
            CollectionPageDirection::Above => {
                (current_start + current_extent - viewport_extent).max(0.0)
            }
            CollectionPageDirection::Below => {
                (current_start - current_extent + viewport_extent).min(content_extent)
            }
        };

        match direction {
            CollectionPageDirection::Above => {
                for candidate in ordered_keys[..current_index].iter().rev() {
                    let Some(rect) = self.item_rect(candidate) else {
                        continue;
                    };
                    if rect_axis(rect, orientation).0 <= target {
                        return Some((*candidate).clone());
                    }
                }
            }
            CollectionPageDirection::Below => {
                for candidate in &ordered_keys[current_index.saturating_add(1)..] {
                    let Some(rect) = self.item_rect(candidate) else {
                        continue;
                    };
                    if rect_axis(rect, orientation).0 >= target {
                        return Some((*candidate).clone());
                    }
                }
            }
        }
        boundary
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CollectionPageDirection {
    Above,
    Below,
}

fn rect_axis(rect: Rect, orientation: Orientation) -> (f64, f64) {
    match orientation {
        Orientation::Horizontal => (rect.x, rect.width),
        Orientation::Vertical => (rect.y, rect.height),
    }
}

fn validate_rect(name: &str, rect: Rect) -> GuiResult<()> {
    if !rect.x.is_finite() || !rect.y.is_finite() {
        return Err(GuiError::invalid_tree(format!(
            "collection layout {name} coordinates must be finite"
        )));
    }
    validate_extent(name, "width", rect.width)?;
    validate_extent(name, "height", rect.height)
}

fn validate_size(name: &str, size: Size) -> GuiResult<()> {
    validate_extent(name, "width", size.width)?;
    validate_extent(name, "height", size.height)
}

fn validate_extent(name: &str, axis: &str, value: f64) -> GuiResult<()> {
    if !value.is_finite() || value < 0.0 {
        return Err(GuiError::invalid_tree(format!(
            "collection layout {name} {axis} must be finite and non-negative"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(value: &'static str) -> CollectionKey {
        CollectionKey::from(value)
    }

    #[test]
    fn vertical_page_navigation_uses_variable_item_geometry() {
        let keys = [key("a"), key("b"), key("c"), key("d"), key("e")];
        let snapshot = CollectionLayoutSnapshot::new(
            Rect::new(0.0, 35.0, 200.0, 100.0),
            Size::new(200.0, 230.0),
        )
        .with_item_rect("a", Rect::new(0.0, 0.0, 200.0, 30.0))
        .with_item_rect("b", Rect::new(0.0, 30.0, 200.0, 50.0))
        .with_item_rect("c", Rect::new(0.0, 80.0, 200.0, 20.0))
        .with_item_rect("d", Rect::new(0.0, 100.0, 200.0, 70.0))
        .with_item_rect("e", Rect::new(0.0, 170.0, 200.0, 60.0));
        let ordered = keys.iter().collect::<Vec<_>>();

        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[1]),
                Orientation::Vertical,
                CollectionPageDirection::Below,
            ),
            Some(key("c"))
        );
        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[4]),
                Orientation::Vertical,
                CollectionPageDirection::Above,
            ),
            Some(key("d"))
        );
    }

    #[test]
    fn horizontal_page_navigation_uses_width() {
        let keys = [key("a"), key("b"), key("c"), key("d")];
        let snapshot =
            CollectionLayoutSnapshot::new(Rect::new(20.0, 0.0, 90.0, 30.0), Size::new(210.0, 30.0))
                .with_item_rect("a", Rect::new(0.0, 0.0, 40.0, 30.0))
                .with_item_rect("b", Rect::new(40.0, 0.0, 60.0, 30.0))
                .with_item_rect("c", Rect::new(100.0, 0.0, 30.0, 30.0))
                .with_item_rect("d", Rect::new(130.0, 0.0, 80.0, 30.0));
        let ordered = keys.iter().collect::<Vec<_>>();

        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[0]),
                Orientation::Horizontal,
                CollectionPageDirection::Below,
            ),
            Some(key("c"))
        );
        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[3]),
                Orientation::Horizontal,
                CollectionPageDirection::Above,
            ),
            Some(key("c"))
        );
    }

    #[test]
    fn non_scrollable_layout_uses_boundaries() {
        let keys = [key("a"), key("b"), key("c")];
        let snapshot = CollectionLayoutSnapshot::new(
            Rect::new(0.0, 0.0, 100.0, 100.0),
            Size::new(100.0, 90.0),
        )
        .with_item_rect("b", Rect::new(0.0, 30.0, 100.0, 30.0));
        let ordered = keys.iter().collect::<Vec<_>>();

        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[1]),
                Orientation::Vertical,
                CollectionPageDirection::Above,
            ),
            Some(key("a"))
        );
        assert_eq!(
            snapshot.page_target(
                &ordered,
                Some(&keys[1]),
                Orientation::Vertical,
                CollectionPageDirection::Below,
            ),
            Some(key("c"))
        );
    }

    #[test]
    fn invalid_geometry_is_rejected() {
        let snapshot = CollectionLayoutSnapshot::new(
            Rect::new(0.0, 0.0, f64::NAN, 100.0),
            Size::new(100.0, 100.0),
        );

        assert!(snapshot
            .validate()
            .unwrap_err()
            .to_string()
            .contains("must be finite and non-negative"));
    }
}
