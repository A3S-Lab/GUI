use std::collections::BTreeMap;

use crate::error::GuiResult;
use crate::geometry::Rect;
use crate::input::NativeInputModality;
use crate::layout::{LayoutElementId, LayoutSnapshot};
use crate::native::{normalize_props_for_native_role, NativeElement, NativeProps, NativeRole};
use crate::platform_host::{PlatformElementId, PlatformPoint};
use crate::semantic_event::{long_press_threshold_micros, SemanticActionSource};
use crate::style::interaction_requirements_from_web;

use super::drag_drop::{SelfDrawnDragSource, SelfDrawnDropTarget};
use super::drag_drop_collection::{
    collection_item_key, drop_indicator_target, is_collection_container, is_draggable_collection,
    SelfDrawnCollectionDropConfig, SelfDrawnCollectionDropTarget,
};

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnInteractionTree {
    pub(super) nodes: BTreeMap<PlatformElementId, SelfDrawnInteractionNode>,
    pub(super) tree_order: Vec<PlatformElementId>,
    pub(super) hit_regions: Vec<SelfDrawnHitRegion>,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnInteractionNode {
    pub(super) id: PlatformElementId,
    pub(super) parent: Option<PlatformElementId>,
    pub(super) role: NativeRole,
    pub(super) props: NativeProps,
    pub(super) available: bool,
    pub(super) focusable: bool,
    pub(super) tab_index: i32,
    pub(super) tree_order: usize,
    pub(super) bounds: Rect,
    pub(super) collection_container: bool,
    pub(super) draggable_collection: bool,
    pub(super) collection_item_key: Option<String>,
    pub(super) drop_indicator_target: Option<SelfDrawnCollectionDropTarget>,
    tracks_pointer_interaction: bool,
    tracks_movement: bool,
    pub(super) drag_source: Option<SelfDrawnDragSource>,
    pub(super) drop_target: Option<SelfDrawnDropTarget>,
    pub(super) collection_drop: Option<SelfDrawnCollectionDropConfig>,
    long_press_mode: SelfDrawnLongPressMode,
    long_press_threshold_micros: u64,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum SelfDrawnLongPressMode {
    #[default]
    Disabled,
    AnyPointer,
    TouchOrPen,
}

#[derive(Debug, Clone)]
pub(super) struct SelfDrawnHitRegion {
    pub(super) id: PlatformElementId,
    pub(super) bounds: Rect,
}

impl SelfDrawnInteractionTree {
    pub(super) fn build(root: &NativeElement, layout: &LayoutSnapshot) -> GuiResult<Self> {
        let mut tree = Self {
            nodes: BTreeMap::new(),
            tree_order: Vec::new(),
            hit_regions: Vec::with_capacity(layout.hit_regions.len()),
        };
        let root_id = LayoutElementId::root(root.key.as_str());
        tree.push_node(root, &root_id, None, true, layout)?;
        tree.hit_regions = layout
            .hit_regions
            .iter()
            .map(|region| {
                Ok(SelfDrawnHitRegion {
                    id: PlatformElementId::new(region.id.as_str())?,
                    bounds: region.bounds,
                })
            })
            .collect::<GuiResult<Vec<_>>>()?;
        Ok(tree)
    }

    pub(super) fn len(&self) -> usize {
        self.nodes.len()
    }

    pub(super) fn node(&self, id: &PlatformElementId) -> Option<&SelfDrawnInteractionNode> {
        self.nodes.get(id)
    }

    pub(super) fn contains(&self, id: &PlatformElementId) -> bool {
        self.nodes.contains_key(id)
    }

    pub(super) fn is_available(&self, id: &PlatformElementId) -> bool {
        self.node(id).is_some_and(|node| node.available)
    }

    pub(super) fn ids(&self) -> impl Iterator<Item = &PlatformElementId> {
        self.tree_order.iter()
    }

    pub(super) fn root_id(&self) -> Option<&PlatformElementId> {
        self.tree_order.first()
    }

    pub(super) fn source(&self, id: &PlatformElementId) -> Option<SemanticActionSource<'_>> {
        let node = self.node(id)?;
        Some(SemanticActionSource::from_props(node.role, &node.props))
    }

    pub(super) fn hit_test(&self, point: PlatformPoint) -> Option<PlatformElementId> {
        for region in self.hit_regions.iter().rev() {
            if !contains(region.bounds, point) {
                continue;
            }
            let node = self.node(&region.id)?;
            if !node.available {
                return None;
            }
            return self.interaction_target(&region.id);
        }
        None
    }

    pub(super) fn focus_target(&self, target: &PlatformElementId) -> Option<PlatformElementId> {
        self.ancestors_inclusive(target)
            .into_iter()
            .find(|id| self.node(id).is_some_and(|node| node.focusable))
    }

    pub(super) fn ancestors_inclusive(&self, target: &PlatformElementId) -> Vec<PlatformElementId> {
        let mut path = Vec::new();
        let mut next = Some(target.clone());
        while let Some(current) = next {
            let Some(node) = self.node(&current) else {
                break;
            };
            next = node.parent.clone();
            path.push(current);
        }
        path
    }

    pub(super) fn tab_target(
        &self,
        current: Option<&PlatformElementId>,
        reverse: bool,
    ) -> Option<PlatformElementId> {
        let mut entries = self
            .nodes
            .values()
            .filter(|node| node.focusable && node.tab_index >= 0)
            .collect::<Vec<_>>();
        entries.sort_by_key(|node| {
            if node.tab_index > 0 {
                (0_u8, node.tab_index, node.tree_order)
            } else {
                (1_u8, 0, node.tree_order)
            }
        });
        if entries.is_empty() {
            return None;
        }
        let current_index = current.and_then(|current| {
            entries
                .iter()
                .position(|candidate| candidate.id == *current)
        });
        let index = match (current_index, reverse) {
            (Some(0), true) | (None, true) => entries.len() - 1,
            (Some(index), true) => index - 1,
            (Some(index), false) => (index + 1) % entries.len(),
            (None, false) => 0,
        };
        Some(entries[index].id.clone())
    }

    pub(super) fn auto_focus_target(&self) -> Option<PlatformElementId> {
        self.tree_order.iter().find_map(|id| {
            self.node(id)
                .filter(|node| node.focusable && node.props.auto_focus)
                .map(|node| node.id.clone())
        })
    }

    pub(super) fn long_press_threshold_micros(
        &self,
        target: &PlatformElementId,
        modality: NativeInputModality,
    ) -> Option<u64> {
        let node = self.node(target)?;
        let accepted = match node.long_press_mode {
            SelfDrawnLongPressMode::Disabled => false,
            SelfDrawnLongPressMode::AnyPointer => matches!(
                modality,
                NativeInputModality::Mouse | NativeInputModality::Touch | NativeInputModality::Pen
            ),
            SelfDrawnLongPressMode::TouchOrPen => {
                matches!(
                    modality,
                    NativeInputModality::Touch | NativeInputModality::Pen
                )
            }
        };
        accepted.then_some(node.long_press_threshold_micros)
    }

    pub(super) fn tracks_movement(&self, target: &PlatformElementId) -> bool {
        self.node(target)
            .is_some_and(|node| node.available && node.tracks_movement)
    }

    pub(super) fn local_position(
        &self,
        target: &PlatformElementId,
        point: PlatformPoint,
    ) -> Option<PlatformPoint> {
        let bounds = self.node(target)?.bounds;
        Some(PlatformPoint::new(point.x - bounds.x, point.y - bounds.y))
    }

    pub(super) fn is_focusable(&self, target: &PlatformElementId) -> bool {
        self.node(target).is_some_and(|node| node.focusable)
    }

    fn interaction_target(&self, target: &PlatformElementId) -> Option<PlatformElementId> {
        self.ancestors_inclusive(target).into_iter().find(|id| {
            self.node(id).is_some_and(|node| {
                node.available
                    && (node.focusable
                        || node.tracks_pointer_interaction
                        || self
                            .source(id)
                            .is_some_and(|source| source.has_interaction_binding()))
            })
        })
    }

    fn push_node(
        &mut self,
        element: &NativeElement,
        layout_id: &LayoutElementId,
        parent: Option<PlatformElementId>,
        parent_available: bool,
        layout: &LayoutSnapshot,
    ) -> GuiResult<()> {
        let props = normalize_props_for_native_role(element.role, &element.props);
        let id = PlatformElementId::new(layout_id.as_str())?;
        let has_layout = layout.node(layout_id).is_some();
        let bounds = layout
            .node(layout_id)
            .map(|node| node.border_box)
            .unwrap_or_else(|| Rect::new(0.0, 0.0, 0.0, 0.0));
        let available = parent_available
            && has_layout
            && !props.disabled
            && !props.hidden
            && !props.inert
            && props.html_dialog.open.unwrap_or(true);
        let tree_order = self.tree_order.len();
        let focusable =
            available && !is_focus_scope(&props) && role_is_focusable(element.role, &props);
        let interaction_requirements = interaction_requirements_from_web(&props.web);
        let style_requires_long_press = interaction_requirements.long_press;
        let has_move_event = ["onMoveStart", "onMove", "onMoveEnd"]
            .into_iter()
            .any(|name| {
                props
                    .web
                    .events
                    .get(name)
                    .is_some_and(|action| !action.is_empty())
            });
        let tracks_movement = interaction_requirements.movement || has_move_event;
        let draggable_collection = is_draggable_collection(&props);
        let drag_source = (!draggable_collection)
            .then(|| SelfDrawnDragSource::from_props(&props, interaction_requirements.dragging))
            .flatten();
        let drop_target =
            SelfDrawnDropTarget::from_props(&props, interaction_requirements.drop_target);
        let collection_drop =
            SelfDrawnCollectionDropConfig::from_props(&props, drop_target.clone());
        let collection_container = is_collection_container(element.role, &props);
        let collection_item_key = collection_item_key(element.role, element.key.as_str(), &props);
        let drop_indicator_target = drop_indicator_target(&props);
        let tracks_pointer_interaction = interaction_requirements.press
            || interaction_requirements.long_press
            || tracks_movement
            || drag_source.is_some()
            || interaction_requirements.hover;
        let has_long_press_event = ["onLongPressStart", "onLongPressEnd", "onLongPress"]
            .into_iter()
            .any(|name| {
                props
                    .web
                    .events
                    .get(name)
                    .is_some_and(|action| !action.is_empty())
            });
        let has_collection_action = props
            .metadata
            .get(crate::selection::COLLECTION_ACTION_METADATA_KEY)
            .is_some_and(|value| value.eq_ignore_ascii_case("true"));
        let long_press_mode = if has_long_press_event || style_requires_long_press {
            SelfDrawnLongPressMode::AnyPointer
        } else if has_collection_action {
            SelfDrawnLongPressMode::TouchOrPen
        } else {
            SelfDrawnLongPressMode::Disabled
        };
        let long_press_threshold_micros = long_press_threshold_micros(&props.metadata);
        self.tree_order.push(id.clone());
        self.nodes.insert(
            id.clone(),
            SelfDrawnInteractionNode {
                id: id.clone(),
                parent,
                role: element.role,
                tab_index: props.tab_index.unwrap_or(0),
                props,
                available,
                focusable,
                tree_order,
                bounds,
                collection_container,
                draggable_collection,
                collection_item_key,
                drop_indicator_target,
                tracks_pointer_interaction,
                tracks_movement,
                drag_source,
                drop_target,
                collection_drop,
                long_press_mode,
                long_press_threshold_micros,
            },
        );
        for child in &element.children {
            let child_layout_id = layout_id.child(child.key.as_str());
            self.push_node(child, &child_layout_id, Some(id.clone()), available, layout)?;
        }
        Ok(())
    }
}

pub(super) fn contains(bounds: Rect, point: PlatformPoint) -> bool {
    bounds.width > 0.0
        && bounds.height > 0.0
        && point.x >= bounds.x
        && point.y >= bounds.y
        && point.x < bounds.x + bounds.width
        && point.y < bounds.y + bounds.height
}

fn is_focus_scope(props: &NativeProps) -> bool {
    bool_marker(props, "data-focus-scope")
}

fn bool_marker(props: &NativeProps, name: &str) -> bool {
    props
        .metadata
        .get(name)
        .or_else(|| props.web.attributes.get(name))
        .is_some_and(|value| {
            value.is_empty()
                || value == "1"
                || value.eq_ignore_ascii_case("true")
                || value.eq_ignore_ascii_case(name)
        })
}

fn role_is_focusable(role: NativeRole, props: &NativeProps) -> bool {
    if props.tab_index.is_some() {
        return true;
    }
    if props
        .content_editable
        .as_deref()
        .is_some_and(|value| !value.eq_ignore_ascii_case("false"))
    {
        return true;
    }
    matches!(
        role,
        NativeRole::Button
            | NativeRole::Link
            | NativeRole::ImageMapArea
            | NativeRole::TextField
            | NativeRole::Checkbox
            | NativeRole::Switch
            | NativeRole::Radio
            | NativeRole::Select
            | NativeRole::ComboBox
            | NativeRole::ListBox
            | NativeRole::ListBoxItem
            | NativeRole::TreeItem
            | NativeRole::DisclosureSummary
            | NativeRole::Tab
            | NativeRole::MenuItem
            | NativeRole::Slider
    )
}
