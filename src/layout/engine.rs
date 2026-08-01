use std::collections::BTreeSet;

use crate::error::{GuiError, GuiResult};
use crate::geometry::{Orientation, Rect, Size};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::render_contract::{native_role_render_milestone, RenderFieldMilestone};
use crate::style::{
    AlignItems, BoxSizing, DisplayMode, JustifyContent, PortableStyle, PositionMode, SelfAlignment,
    TextDirection,
};

use super::resolve::{
    diagnose_style_inventory, push_warning, resolve_paint, resolve_style, Edges, Insets,
    ResolvedStyle,
};
use super::{
    LayoutDiagnostic, LayoutDiagnosticCode, LayoutElementId, LayoutHitRegion, LayoutNodeRecord,
    LayoutPaint, LayoutSnapshot, LAYOUT_QUANTIZATION, LAYOUT_SCHEMA_VERSION,
};

#[derive(Debug, Clone)]
struct PreparedNode {
    id: LayoutElementId,
    parent_id: Option<LayoutElementId>,
    role: NativeRole,
    props: NativeProps,
    style: PortableStyle,
    children: Vec<PreparedNode>,
}

#[derive(Debug, Clone)]
struct MeasuredNode {
    id: LayoutElementId,
    parent_id: Option<LayoutElementId>,
    role: NativeRole,
    disabled: bool,
    margin: Edges,
    padding: Edges,
    border_size: Size,
    orientation: Orientation,
    row_gap: f64,
    column_gap: f64,
    align_items: AlignItems,
    align_self: SelfAlignment,
    justify_content: JustifyContent,
    position: PositionMode,
    insets: Insets,
    order: i32,
    z_index: i32,
    clip_x: bool,
    clip_y: bool,
    hit_testable: bool,
    explicit_width: bool,
    explicit_height: bool,
    paint: LayoutPaint,
    children: Vec<MeasuredNode>,
}

struct Placement<'a> {
    viewport: Rect,
    nodes: &'a mut Vec<LayoutNodeRecord>,
    hit_regions: &'a mut Vec<LayoutHitRegion>,
    next_paint_order: u32,
}

pub fn layout_native_tree(root: &NativeElement, logical_size: Size) -> GuiResult<LayoutSnapshot> {
    validate_logical_size(logical_size)?;
    let logical_size = Size::new(quantize(logical_size.width), quantize(logical_size.height));
    validate_logical_size(logical_size)?;
    let mut diagnostics = Vec::new();
    let root_id = LayoutElementId::root(root.key.as_str());
    let prepared = prepare_node(root, root_id, None, &mut diagnostics)?;
    let measured = measure_node(prepared, logical_size, true, &mut diagnostics);
    let mut nodes = Vec::new();
    let mut hit_regions = Vec::new();

    if let Some(measured) = measured {
        let mut placement = Placement {
            viewport: Rect::new(0.0, 0.0, logical_size.width, logical_size.height),
            nodes: &mut nodes,
            hit_regions: &mut hit_regions,
            next_paint_order: 0,
        };
        place_node(
            &measured,
            0.0,
            0.0,
            measured.border_size,
            None,
            1.0,
            &mut placement,
        )?;
    }

    Ok(LayoutSnapshot {
        schema_version: LAYOUT_SCHEMA_VERSION,
        logical_size,
        nodes,
        hit_regions,
        diagnostics,
    })
}

fn validate_logical_size(size: Size) -> GuiResult<()> {
    if !size.width.is_finite()
        || !size.height.is_finite()
        || size.width <= 0.0
        || size.height <= 0.0
    {
        return Err(GuiError::invalid_tree(
            "layout logical size must be finite and greater than zero",
        ));
    }
    Ok(())
}

fn prepare_node(
    element: &NativeElement,
    id: LayoutElementId,
    parent_id: Option<LayoutElementId>,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> GuiResult<PreparedNode> {
    if element.key.as_str().is_empty() {
        return Err(GuiError::invalid_tree(
            "layout native elements need non-empty keys",
        ));
    }
    let style = PortableStyle::from_web(&element.props.web);
    diagnose_style_inventory(&style, &id, diagnostics)?;
    let role_milestone = native_role_render_milestone(element.role);
    if role_milestone > RenderFieldMilestone::M3LayoutScene {
        push_warning(
            diagnostics,
            &id,
            LayoutDiagnosticCode::DeferredRole,
            None,
            format!(
                "role {:?} receives only its generic M3 box; visible content is deferred to {role_milestone:?}",
                element.role
            ),
        );
    }

    let mut sibling_keys = BTreeSet::new();
    let mut children = Vec::with_capacity(element.children.len());
    for child in &element.children {
        let key = child.key.as_str();
        if key.is_empty() {
            return Err(GuiError::invalid_tree(
                "layout native elements need non-empty keys",
            ));
        }
        if !sibling_keys.insert(key) {
            return Err(GuiError::invalid_tree(format!(
                "layout native siblings need unique keys; duplicate key {key:?} under {id}"
            )));
        }
        children.push(prepare_node(
            child,
            id.child(key),
            Some(id.clone()),
            diagnostics,
        )?);
    }

    Ok(PreparedNode {
        id,
        parent_id,
        role: element.role,
        props: element.props.clone(),
        style,
        children,
    })
}

fn measure_node(
    mut node: PreparedNode,
    available: Size,
    is_root: bool,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<MeasuredNode> {
    let resolved = resolve_style(&node.style, &node.props, &node.id, available, diagnostics);
    if resolved.hidden {
        return None;
    }

    let horizontal_chrome =
        resolved.padding.horizontal() + resolved.border_widths.left + resolved.border_widths.right;
    let vertical_chrome =
        resolved.padding.vertical() + resolved.border_widths.top + resolved.border_widths.bottom;
    let available_border_width = (available.width - resolved.margin.horizontal()).max(0.0);
    let available_border_height = (available.height - resolved.margin.vertical()).max(0.0);
    let declared_border_width = resolved
        .declared_width
        .map(|value| declared_to_border(value, resolved.box_sizing, horizontal_chrome));
    let declared_border_height = resolved
        .declared_height
        .map(|value| declared_to_border(value, resolved.box_sizing, vertical_chrome));
    let provisional_width = constrain_dimension(
        declared_border_width.unwrap_or(available_border_width),
        resolved.min_width,
        resolved.max_width,
        resolved.box_sizing,
        horizontal_chrome,
    );
    let provisional_height = constrain_dimension(
        declared_border_height.unwrap_or(available_border_height),
        resolved.min_height,
        resolved.max_height,
        resolved.box_sizing,
        vertical_chrome,
    );
    let child_available = Size::new(
        (provisional_width - horizontal_chrome).max(0.0),
        (provisional_height - vertical_chrome).max(0.0),
    );

    let children = std::mem::take(&mut node.children)
        .into_iter()
        .filter_map(|child| measure_node(child, child_available, false, diagnostics))
        .collect::<Vec<_>>();
    let (desired_content_width, desired_content_height) =
        desired_content_size(&children, &resolved);
    let fills_available_width = is_root
        || !matches!(
            node.style.display,
            Some(DisplayMode::Inline | DisplayMode::InlineBlock | DisplayMode::InlineFlex)
        );
    let auto_width = if fills_available_width && available_border_width > 0.0 {
        available_border_width
    } else {
        desired_content_width + horizontal_chrome
    };
    let auto_height = if is_root {
        available_border_height
    } else {
        desired_content_height + vertical_chrome
    };
    let width = constrain_dimension(
        declared_border_width.unwrap_or(auto_width),
        resolved.min_width,
        resolved.max_width,
        resolved.box_sizing,
        horizontal_chrome,
    );
    let height = constrain_dimension(
        declared_border_height.unwrap_or(auto_height),
        resolved.min_height,
        resolved.max_height,
        resolved.box_sizing,
        vertical_chrome,
    );
    let border_size = Size::new(width.max(0.0), height.max(0.0));
    let right_to_left = matches!(node.style.direction, Some(TextDirection::Rtl))
        || node
            .props
            .dir
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case("rtl"));
    let paint = resolve_paint(
        &node.style,
        &node.id,
        border_size,
        right_to_left,
        resolved.border_widths,
        resolved.opacity,
        diagnostics,
    );

    Some(measured_node(node, resolved, border_size, paint, children))
}

fn measured_node(
    node: PreparedNode,
    resolved: ResolvedStyle,
    border_size: Size,
    paint: LayoutPaint,
    children: Vec<MeasuredNode>,
) -> MeasuredNode {
    MeasuredNode {
        id: node.id,
        parent_id: node.parent_id,
        role: node.role,
        disabled: node.props.disabled,
        margin: resolved.margin,
        padding: resolved.padding,
        border_size,
        orientation: resolved.orientation,
        row_gap: resolved.row_gap,
        column_gap: resolved.column_gap,
        align_items: resolved.align_items,
        align_self: resolved.align_self,
        justify_content: resolved.justify_content,
        position: resolved.position,
        insets: resolved.insets,
        order: resolved.order,
        z_index: resolved.z_index,
        clip_x: resolved.clip_x,
        clip_y: resolved.clip_y,
        hit_testable: resolved.hit_testable,
        explicit_width: resolved.explicit_width,
        explicit_height: resolved.explicit_height,
        paint,
        children,
    }
}

fn desired_content_size(children: &[MeasuredNode], style: &ResolvedStyle) -> (f64, f64) {
    let flow = children
        .iter()
        .filter(|child| !is_out_of_flow(child.position))
        .collect::<Vec<_>>();
    if flow.is_empty() {
        return (0.0, 0.0);
    }
    match style.orientation {
        Orientation::Vertical => {
            let width = flow
                .iter()
                .map(|child| child.margin.horizontal() + child.border_size.width)
                .fold(0.0, f64::max);
            let height = flow
                .iter()
                .map(|child| child.margin.vertical() + child.border_size.height)
                .sum::<f64>()
                + style.row_gap * (flow.len().saturating_sub(1) as f64);
            (width, height)
        }
        Orientation::Horizontal => {
            let width = flow
                .iter()
                .map(|child| child.margin.horizontal() + child.border_size.width)
                .sum::<f64>()
                + style.column_gap * (flow.len().saturating_sub(1) as f64);
            let height = flow
                .iter()
                .map(|child| child.margin.vertical() + child.border_size.height)
                .fold(0.0, f64::max);
            (width, height)
        }
    }
}

fn declared_to_border(value: f64, box_sizing: BoxSizing, chrome: f64) -> f64 {
    match box_sizing {
        BoxSizing::BorderBox => value,
        BoxSizing::ContentBox => value + chrome,
    }
}

fn constrain_dimension(
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    box_sizing: BoxSizing,
    chrome: f64,
) -> f64 {
    let min = min.map(|value| declared_to_border(value, box_sizing, chrome));
    let max = max.map(|value| declared_to_border(value, box_sizing, chrome));
    let value = max.map_or(value, |maximum| value.min(maximum));
    min.map_or(value, |minimum| value.max(minimum))
}

fn place_node(
    node: &MeasuredNode,
    x: f64,
    y: f64,
    actual_size: Size,
    inherited_clip: Option<Rect>,
    inherited_opacity: f64,
    placement: &mut Placement<'_>,
) -> GuiResult<()> {
    let border_box = Rect::new(
        x,
        y,
        actual_size.width.max(0.0),
        actual_size.height.max(0.0),
    );
    let padding_box = inset_rect(
        border_box,
        Edges {
            top: node.paint.border_widths.top,
            right: node.paint.border_widths.right,
            bottom: node.paint.border_widths.bottom,
            left: node.paint.border_widths.left,
        },
    );
    let content_box = inset_rect(padding_box, node.padding);
    let cumulative_opacity = (inherited_opacity * node.paint.opacity).clamp(0.0, 1.0);
    let mut paint = node.paint.clone();
    paint.opacity = quantize(cumulative_opacity);
    let paint_order = placement.next_paint_order;
    placement.next_paint_order = placement
        .next_paint_order
        .checked_add(1)
        .ok_or_else(|| GuiError::invalid_tree("layout paint order exceeds u32 capacity"))?;
    let border_box = quantize_rect(border_box);
    let content_box = quantize_rect(content_box);
    let clip = inherited_clip.map(quantize_rect);
    validate_rect(border_box, "layout border box")?;
    validate_rect(content_box, "layout content box")?;
    if let Some(clip) = clip {
        validate_rect(clip, "layout clip")?;
    }
    placement.nodes.push(LayoutNodeRecord {
        id: node.id.clone(),
        parent_id: node.parent_id.clone(),
        role: node.role,
        border_box,
        content_box,
        clip,
        z_index: node.z_index,
        paint_order,
        hit_testable: node.hit_testable,
        paint,
    });

    if node.hit_testable {
        let bounds = match inherited_clip {
            Some(clip) => intersect_rect(border_box, clip),
            None => Some(border_box),
        };
        if let Some(bounds) = bounds {
            placement.hit_regions.push(LayoutHitRegion {
                id: node.id.clone(),
                role: node.role,
                bounds: quantize_rect(bounds),
                clip,
                disabled: node.disabled,
            });
        }
    }

    let child_clip = descendant_clip(
        inherited_clip,
        padding_box,
        placement.viewport,
        node.clip_x,
        node.clip_y,
    );
    place_children(node, content_box, child_clip, cumulative_opacity, placement)
}

fn place_children(
    node: &MeasuredNode,
    content: Rect,
    child_clip: Option<Rect>,
    opacity: f64,
    placement: &mut Placement<'_>,
) -> GuiResult<()> {
    let mut flow = node
        .children
        .iter()
        .enumerate()
        .filter(|(_, child)| !is_out_of_flow(child.position))
        .collect::<Vec<_>>();
    flow.sort_by_key(|(index, child)| (child.order, *index));
    let gap = match node.orientation {
        Orientation::Vertical => node.row_gap,
        Orientation::Horizontal => node.column_gap,
    };
    let main_available = match node.orientation {
        Orientation::Vertical => content.height,
        Orientation::Horizontal => content.width,
    };
    let occupied = flow
        .iter()
        .map(|(_, child)| outer_main_size(child, node.orientation))
        .sum::<f64>()
        + gap * (flow.len().saturating_sub(1) as f64);
    let remaining = (main_available - occupied).max(0.0);
    let (mut cursor, extra_gap) = justify_offsets(node.justify_content, flow.len(), remaining);
    let mut flow_geometry_by_source = vec![None; node.children.len()];

    for (index, child) in flow {
        let (x, y, size) = flow_geometry(child, node, content, cursor);
        let (x, y) = relative_offset(child, x, y);
        flow_geometry_by_source[index] = Some((x, y, size));
        cursor += outer_main_size(child, node.orientation) + gap + extra_gap;
    }

    let mut children = node.children.iter().enumerate().collect::<Vec<_>>();
    children.sort_by_key(|(index, child)| (child.z_index, child.order, *index));

    for (index, child) in children {
        if is_out_of_flow(child.position) {
            let containing = if child.position == PositionMode::Fixed {
                placement.viewport
            } else {
                content
            };
            let inherited_clip = if child.position == PositionMode::Fixed {
                None
            } else {
                child_clip
            };
            let (x, y) = absolute_origin(child, containing);
            place_node(
                child,
                x,
                y,
                child.border_size,
                inherited_clip,
                opacity,
                placement,
            )?;
            continue;
        }

        let (x, y, size) = flow_geometry_by_source[index].ok_or_else(|| {
            GuiError::invalid_tree(format!(
                "layout flow geometry is missing for element {}",
                child.id
            ))
        })?;
        place_node(child, x, y, size, child_clip, opacity, placement)?;
    }
    Ok(())
}

fn flow_geometry(
    child: &MeasuredNode,
    parent: &MeasuredNode,
    content: Rect,
    cursor: f64,
) -> (f64, f64, Size) {
    match parent.orientation {
        Orientation::Vertical => {
            let alignment = cross_alignment(child.align_self, parent.align_items);
            let available = (content.width - child.margin.horizontal()).max(0.0);
            let width = if alignment == CrossAlignment::Stretch && !child.explicit_width {
                available
            } else {
                child.border_size.width
            };
            let x = cross_origin(
                content.x,
                content.width,
                width,
                child.margin.left,
                child.margin.right,
                alignment,
            );
            (
                x,
                content.y + cursor + child.margin.top,
                Size::new(width, child.border_size.height),
            )
        }
        Orientation::Horizontal => {
            let alignment = cross_alignment(child.align_self, parent.align_items);
            let available = (content.height - child.margin.vertical()).max(0.0);
            let height = if alignment == CrossAlignment::Stretch && !child.explicit_height {
                available
            } else {
                child.border_size.height
            };
            let y = cross_origin(
                content.y,
                content.height,
                height,
                child.margin.top,
                child.margin.bottom,
                alignment,
            );
            (
                content.x + cursor + child.margin.left,
                y,
                Size::new(child.border_size.width, height),
            )
        }
    }
}

fn outer_main_size(child: &MeasuredNode, orientation: Orientation) -> f64 {
    match orientation {
        Orientation::Vertical => child.margin.vertical() + child.border_size.height,
        Orientation::Horizontal => child.margin.horizontal() + child.border_size.width,
    }
}

fn justify_offsets(justify: JustifyContent, count: usize, remaining: f64) -> (f64, f64) {
    match justify {
        JustifyContent::Center => (remaining / 2.0, 0.0),
        JustifyContent::End => (remaining, 0.0),
        JustifyContent::SpaceBetween if count > 1 => (0.0, remaining / (count - 1) as f64),
        JustifyContent::SpaceAround if count > 0 => {
            let gap = remaining / count as f64;
            (gap / 2.0, gap)
        }
        JustifyContent::SpaceEvenly if count > 0 => {
            let gap = remaining / (count + 1) as f64;
            (gap, gap)
        }
        _ => (0.0, 0.0),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrossAlignment {
    Start,
    Center,
    End,
    Stretch,
}

fn cross_alignment(alignment: SelfAlignment, parent: AlignItems) -> CrossAlignment {
    match alignment {
        SelfAlignment::Start => CrossAlignment::Start,
        SelfAlignment::Center => CrossAlignment::Center,
        SelfAlignment::End => CrossAlignment::End,
        SelfAlignment::Stretch => CrossAlignment::Stretch,
        SelfAlignment::Auto | SelfAlignment::Baseline => match parent {
            AlignItems::Center => CrossAlignment::Center,
            AlignItems::End => CrossAlignment::End,
            AlignItems::Stretch | AlignItems::Normal => CrossAlignment::Stretch,
            AlignItems::Start | AlignItems::Baseline => CrossAlignment::Start,
        },
    }
}

fn cross_origin(
    start: f64,
    available: f64,
    size: f64,
    leading_margin: f64,
    trailing_margin: f64,
    alignment: CrossAlignment,
) -> f64 {
    let outer = size + leading_margin + trailing_margin;
    match alignment {
        CrossAlignment::Start | CrossAlignment::Stretch => start + leading_margin,
        CrossAlignment::Center => start + (available - outer) / 2.0 + leading_margin,
        CrossAlignment::End => start + available - outer + leading_margin,
    }
}

fn absolute_origin(child: &MeasuredNode, containing: Rect) -> (f64, f64) {
    let x = if let Some(left) = child.insets.left {
        containing.x + left + child.margin.left
    } else if let Some(right) = child.insets.right {
        containing.x + containing.width - right - child.border_size.width - child.margin.right
    } else {
        containing.x + child.margin.left
    };
    let y = if let Some(top) = child.insets.top {
        containing.y + top + child.margin.top
    } else if let Some(bottom) = child.insets.bottom {
        containing.y + containing.height - bottom - child.border_size.height - child.margin.bottom
    } else {
        containing.y + child.margin.top
    };
    (x, y)
}

fn relative_offset(child: &MeasuredNode, x: f64, y: f64) -> (f64, f64) {
    if child.position != PositionMode::Relative {
        return (x, y);
    }
    let x = x + child.insets.left.unwrap_or(0.0) - child.insets.right.unwrap_or(0.0);
    let y = y + child.insets.top.unwrap_or(0.0) - child.insets.bottom.unwrap_or(0.0);
    (x, y)
}

fn is_out_of_flow(position: PositionMode) -> bool {
    matches!(position, PositionMode::Absolute | PositionMode::Fixed)
}

fn descendant_clip(
    inherited: Option<Rect>,
    padding_box: Rect,
    viewport: Rect,
    clip_x: bool,
    clip_y: bool,
) -> Option<Rect> {
    if !clip_x && !clip_y {
        return inherited;
    }
    let own = Rect::new(
        if clip_x { padding_box.x } else { viewport.x },
        if clip_y { padding_box.y } else { viewport.y },
        if clip_x {
            padding_box.width
        } else {
            viewport.width
        },
        if clip_y {
            padding_box.height
        } else {
            viewport.height
        },
    );
    Some(match inherited {
        Some(inherited) => intersect_or_empty(inherited, own),
        None => own,
    })
}

fn inset_rect(rect: Rect, edges: Edges) -> Rect {
    Rect::new(
        rect.x + edges.left,
        rect.y + edges.top,
        (rect.width - edges.horizontal()).max(0.0),
        (rect.height - edges.vertical()).max(0.0),
    )
}

fn intersect_rect(left: Rect, right: Rect) -> Option<Rect> {
    let intersection = intersect_or_empty(left, right);
    (intersection.width > 0.0 && intersection.height > 0.0).then_some(intersection)
}

fn intersect_or_empty(left: Rect, right: Rect) -> Rect {
    let x = left.x.max(right.x);
    let y = left.y.max(right.y);
    let right_edge = (left.x + left.width).min(right.x + right.width);
    let bottom_edge = (left.y + left.height).min(right.y + right.height);
    Rect::new(x, y, (right_edge - x).max(0.0), (bottom_edge - y).max(0.0))
}

fn quantize_rect(rect: Rect) -> Rect {
    Rect::new(
        quantize(rect.x),
        quantize(rect.y),
        quantize(rect.width.max(0.0)),
        quantize(rect.height.max(0.0)),
    )
}

fn validate_rect(rect: Rect, field: &str) -> GuiResult<()> {
    if !rect.x.is_finite()
        || !rect.y.is_finite()
        || !rect.width.is_finite()
        || !rect.height.is_finite()
        || rect.width < 0.0
        || rect.height < 0.0
    {
        return Err(GuiError::invalid_tree(format!(
            "{field} must contain finite coordinates and non-negative dimensions"
        )));
    }
    Ok(())
}

fn quantize(value: f64) -> f64 {
    let value = (value / LAYOUT_QUANTIZATION).round() * LAYOUT_QUANTIZATION;
    if value == -0.0 {
        0.0
    } else {
        value
    }
}
