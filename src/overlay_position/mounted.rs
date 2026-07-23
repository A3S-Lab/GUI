use std::collections::{BTreeMap, BTreeSet};

use crate::error::{GuiError, GuiResult};
use crate::host::HostNodeId;
use crate::native::{NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::style::TextDirection;

use super::{
    OverlayPlacement, OverlayPositionOptions, OverlayPositionRequest,
    OVERLAY_ARROW_BOUNDARY_OFFSET_ATTRIBUTE, OVERLAY_ARROW_SIZE_ATTRIBUTE,
    OVERLAY_CONTAINER_PADDING_ATTRIBUTE, OVERLAY_CROSS_OFFSET_ATTRIBUTE,
    OVERLAY_MAX_HEIGHT_ATTRIBUTE, OVERLAY_OFFSET_ATTRIBUTE, OVERLAY_PLACEMENT_ATTRIBUTE,
    OVERLAY_POSITION_MARKER, OVERLAY_SHOULD_FLIP_ATTRIBUTE,
    OVERLAY_SHOULD_UPDATE_POSITION_ATTRIBUTE,
};

const OVERLAY_OPEN_ATTRIBUTE: &str = "data-open";
const OVERLAY_TRIGGER_MARKER: &str = "data-overlay-trigger";

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MountedOverlayPosition {
    pub overlay: HostNodeId,
    pub anchor: HostNodeId,
    pub request: OverlayPositionRequest,
}

pub fn mounted_overlay_positions(
    snapshot: &[MountedNodeSnapshot],
) -> GuiResult<Vec<MountedOverlayPosition>> {
    let records = snapshot
        .iter()
        .map(|record| (record.node, record))
        .collect::<BTreeMap<_, _>>();
    let parents = snapshot
        .iter()
        .map(|record| (record.node, record.parent))
        .collect::<BTreeMap<_, _>>();
    let mut positions = Vec::new();

    for (index, record) in snapshot.iter().enumerate() {
        let Some(options) = options_from_props(&record.props)? else {
            continue;
        };
        if record.role != NativeRole::Popover || !is_open(&record.props) {
            continue;
        }
        let anchor = resolve_anchor(snapshot, index, record, &records, &parents)?;
        if anchor == record.node || is_descendant(anchor, record.node, &parents) {
            return Err(GuiError::invalid_tree(format!(
                "overlay {} cannot anchor to itself or its own content",
                record.key.as_str()
            )));
        }
        let direction = if record
            .props
            .dir
            .as_deref()
            .is_some_and(|direction| direction.eq_ignore_ascii_case("rtl"))
        {
            TextDirection::Rtl
        } else {
            TextDirection::Ltr
        };
        positions.push(MountedOverlayPosition {
            overlay: record.node,
            anchor,
            request: OverlayPositionRequest::new(options, direction)?,
        });
    }

    Ok(positions)
}

fn options_from_props(props: &NativeProps) -> GuiResult<Option<OverlayPositionOptions>> {
    if !bool_value(props, OVERLAY_POSITION_MARKER).unwrap_or(false) {
        return Ok(None);
    }
    let placement = attribute(props, OVERLAY_PLACEMENT_ATTRIBUTE)
        .unwrap_or("bottom")
        .parse::<OverlayPlacement>()?;
    let options = OverlayPositionOptions {
        placement,
        offset: number_value(props, OVERLAY_OFFSET_ATTRIBUTE)?.unwrap_or(0.0),
        cross_offset: number_value(props, OVERLAY_CROSS_OFFSET_ATTRIBUTE)?.unwrap_or(0.0),
        should_flip: bool_value(props, OVERLAY_SHOULD_FLIP_ATTRIBUTE).unwrap_or(true),
        should_update_position: bool_value(props, OVERLAY_SHOULD_UPDATE_POSITION_ATTRIBUTE)
            .unwrap_or(true),
        container_padding: number_value(props, OVERLAY_CONTAINER_PADDING_ATTRIBUTE)?
            .unwrap_or(12.0),
        arrow_size: number_value(props, OVERLAY_ARROW_SIZE_ATTRIBUTE)?.unwrap_or(0.0),
        arrow_boundary_offset: number_value(props, OVERLAY_ARROW_BOUNDARY_OFFSET_ATTRIBUTE)?
            .unwrap_or(0.0),
        max_height: number_value(props, OVERLAY_MAX_HEIGHT_ATTRIBUTE)?,
    };
    options.validate().map(Some)
}

fn resolve_anchor<'a>(
    snapshot: &'a [MountedNodeSnapshot],
    overlay_index: usize,
    overlay: &'a MountedNodeSnapshot,
    records: &BTreeMap<HostNodeId, &'a MountedNodeSnapshot>,
    parents: &BTreeMap<HostNodeId, Option<HostNodeId>>,
) -> GuiResult<HostNodeId> {
    if let Some(anchor) = overlay
        .props
        .anchor
        .as_deref()
        .map(str::trim)
        .filter(|anchor| !anchor.is_empty())
    {
        return resolve_explicit_anchor(snapshot, anchor);
    }

    let mut ancestor = overlay.parent;
    let mut visited = BTreeSet::new();
    while let Some(node) = ancestor {
        if !visited.insert(node) {
            break;
        }
        if records.get(&node).is_some_and(|record| is_trigger(record)) {
            return Ok(node);
        }
        ancestor = parents.get(&node).copied().flatten();
    }

    if let Some(parent) = overlay.parent {
        if let Some(sibling) = snapshot[..overlay_index]
            .iter()
            .rev()
            .find(|candidate| candidate.parent == Some(parent) && is_trigger(candidate))
        {
            return Ok(sibling.node);
        }
        if records
            .get(&parent)
            .is_some_and(|record| record.role != NativeRole::Window)
        {
            return Ok(parent);
        }
    }

    Err(GuiError::invalid_tree(format!(
        "open positioned overlay {} needs an anchor, a trigger context, or a non-window parent",
        overlay.key.as_str()
    )))
}

fn resolve_explicit_anchor(
    snapshot: &[MountedNodeSnapshot],
    requested: &str,
) -> GuiResult<HostNodeId> {
    let requested = requested.trim_start_matches('#');
    let id_matches = snapshot
        .iter()
        .filter(|record| attribute(&record.props, "id") == Some(requested))
        .map(|record| record.node)
        .collect::<Vec<_>>();
    if id_matches.len() == 1 {
        return Ok(id_matches[0]);
    }
    if id_matches.len() > 1 {
        return Err(GuiError::invalid_tree(format!(
            "overlay anchor id {requested:?} is ambiguous"
        )));
    }

    let key_matches = snapshot
        .iter()
        .filter(|record| record.key.as_str() == requested)
        .map(|record| record.node)
        .collect::<Vec<_>>();
    match key_matches.as_slice() {
        [node] => Ok(*node),
        [] => Err(GuiError::invalid_tree(format!(
            "overlay anchor {requested:?} does not match a mounted id or unique key"
        ))),
        _ => Err(GuiError::invalid_tree(format!(
            "overlay anchor key {requested:?} is ambiguous; use a unique id"
        ))),
    }
}

fn is_trigger(record: &MountedNodeSnapshot) -> bool {
    bool_value(&record.props, OVERLAY_TRIGGER_MARKER).unwrap_or(false)
        || attribute(&record.props, "aria-haspopup").is_some()
}

fn is_open(props: &NativeProps) -> bool {
    !props.hidden
        && !props.inert
        && props.html_dialog.open.unwrap_or(true)
        && bool_value(props, OVERLAY_OPEN_ATTRIBUTE).unwrap_or(true)
}

fn is_descendant(
    node: HostNodeId,
    ancestor: HostNodeId,
    parents: &BTreeMap<HostNodeId, Option<HostNodeId>>,
) -> bool {
    let mut current = parents.get(&node).copied().flatten();
    let mut visited = BTreeSet::new();
    while let Some(parent) = current {
        if parent == ancestor {
            return true;
        }
        if !visited.insert(parent) {
            break;
        }
        current = parents.get(&parent).copied().flatten();
    }
    false
}

fn attribute<'a>(props: &'a NativeProps, name: &str) -> Option<&'a str> {
    if name == "id" {
        return props
            .web
            .id
            .as_deref()
            .or_else(|| props.metadata.get(name).map(String::as_str));
    }
    props
        .metadata
        .get(name)
        .or_else(|| props.web.attributes.get(name))
        .map(String::as_str)
}

fn number_value(props: &NativeProps, name: &str) -> GuiResult<Option<f64>> {
    attribute(props, name)
        .map(|value| {
            value.parse::<f64>().map_err(|_| {
                GuiError::invalid_tree(format!(
                    "overlay position attribute {name} must be a number, got {value:?}"
                ))
            })
        })
        .transpose()
}

fn bool_value(props: &NativeProps, name: &str) -> Option<bool> {
    attribute(props, name).and_then(|value| match value.trim().to_ascii_lowercase().as_str() {
        "" | "1" | "true" => Some(true),
        "0" | "false" => Some(false),
        value if value.eq_ignore_ascii_case(name) => Some(true),
        _ => None,
    })
}
