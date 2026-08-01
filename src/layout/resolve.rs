use crate::geometry::{Orientation, Size};
use crate::native::NativeProps;
use crate::style::{
    AlignItems, BorderStyle, BoxSizing, ContentVisibility, DisplayMode, JustifyContent,
    OverflowMode, PointerEvents, PortableStyle, PositionMode, SelfAlignment, StyleColor,
    StyleLength, TextDirection, VisibilityMode, WritingMode,
};

use super::{
    LayoutColor, LayoutCornerRadii, LayoutDiagnostic, LayoutDiagnosticCode, LayoutEdgeColors,
    LayoutEdgeWidths, LayoutElementId, LayoutPaint,
};

mod diagnostics;
use diagnostics::push_error;
pub(super) use diagnostics::{diagnose_style_inventory, push_warning};

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Edges {
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
    pub left: f64,
}

impl Edges {
    pub fn horizontal(self) -> f64 {
        self.left + self.right
    }

    pub fn vertical(self) -> f64 {
        self.top + self.bottom
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Insets {
    pub top: Option<f64>,
    pub right: Option<f64>,
    pub bottom: Option<f64>,
    pub left: Option<f64>,
}

#[derive(Debug, Clone)]
pub(super) struct ResolvedStyle {
    pub margin: Edges,
    pub padding: Edges,
    pub border_widths: LayoutEdgeWidths,
    pub declared_width: Option<f64>,
    pub declared_height: Option<f64>,
    pub min_width: Option<f64>,
    pub min_height: Option<f64>,
    pub max_width: Option<f64>,
    pub max_height: Option<f64>,
    pub box_sizing: BoxSizing,
    pub orientation: Orientation,
    pub row_gap: f64,
    pub column_gap: f64,
    pub align_items: AlignItems,
    pub align_self: SelfAlignment,
    pub justify_content: JustifyContent,
    pub position: PositionMode,
    pub insets: Insets,
    pub order: i32,
    pub z_index: i32,
    pub clip_x: bool,
    pub clip_y: bool,
    pub opacity: f64,
    pub hit_testable: bool,
    pub hidden: bool,
    pub explicit_width: bool,
    pub explicit_height: bool,
}

pub(super) fn resolve_style(
    style: &PortableStyle,
    props: &NativeProps,
    id: &LayoutElementId,
    available: Size,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> ResolvedStyle {
    diagnose_layout_modes(style, id, diagnostics);
    let right_to_left = matches!(style.direction, Some(TextDirection::Rtl))
        || props
            .dir
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case("rtl"));
    if matches!(
        style.writing_mode,
        Some(mode) if mode != WritingMode::HorizontalTb
    ) {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedLayoutMode,
            Some("writingMode"),
            "vertical and sideways writing modes are not implemented by M3 layout",
        );
    }

    let margin = resolve_edges(
        &style.margin,
        &style.logical_margin,
        available.width,
        true,
        right_to_left,
        "margin",
        id,
        diagnostics,
    );
    let padding = resolve_edges(
        &style.padding,
        &style.logical_padding,
        available.width,
        false,
        right_to_left,
        "padding",
        id,
        diagnostics,
    );
    let border = resolve_edges(
        &style.border_width,
        &style.logical_border_width,
        available.width,
        false,
        right_to_left,
        "borderWidth",
        id,
        diagnostics,
    );

    let mut border_widths = LayoutEdgeWidths {
        top: border.top,
        right: border.right,
        bottom: border.bottom,
        left: border.left,
    };
    apply_border_styles(style, id, right_to_left, &mut border_widths, diagnostics);

    let width_value = style.width.as_ref().or(style.inline_size.as_ref());
    let height_value = style.height.as_ref().or(style.block_size.as_ref());
    let min_width_value = style.min_width.as_ref().or(style.min_inline_size.as_ref());
    let min_height_value = style.min_height.as_ref().or(style.min_block_size.as_ref());
    let max_width_value = style.max_width.as_ref().or(style.max_inline_size.as_ref());
    let max_height_value = style.max_height.as_ref().or(style.max_block_size.as_ref());

    let declared_width = resolve_dimension(width_value, available.width, "width", id, diagnostics);
    let declared_height =
        resolve_dimension(height_value, available.height, "height", id, diagnostics);
    let min_width = resolve_dimension(
        min_width_value,
        available.width,
        "minWidth",
        id,
        diagnostics,
    );
    let min_height = resolve_dimension(
        min_height_value,
        available.height,
        "minHeight",
        id,
        diagnostics,
    );
    let max_width = resolve_dimension(
        max_width_value,
        available.width,
        "maxWidth",
        id,
        diagnostics,
    );
    let max_height = resolve_dimension(
        max_height_value,
        available.height,
        "maxHeight",
        id,
        diagnostics,
    );

    let shared_gap =
        resolve_non_negative(style.gap.as_ref(), available.width, "gap", id, diagnostics)
            .unwrap_or(0.0);
    let row_gap = resolve_non_negative(
        style.row_gap.as_ref().or(style.space_y.as_ref()),
        available.width,
        "rowGap",
        id,
        diagnostics,
    )
    .unwrap_or(shared_gap);
    let column_gap = resolve_non_negative(
        style.column_gap.as_ref().or(style.space_x.as_ref()),
        available.width,
        "columnGap",
        id,
        diagnostics,
    )
    .unwrap_or(shared_gap);

    let insets = resolve_insets(style, available, right_to_left, id, diagnostics);
    let order = parse_integer(style.order.as_deref(), "order", id, diagnostics).unwrap_or(0);
    let opacity = match style.opacity {
        Some(value) if value.is_finite() && (0.0..=1.0).contains(&value) => value,
        Some(_) => {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedM3StyleField,
                Some("opacity"),
                "opacity must be a finite number between zero and one",
            );
            1.0
        }
        None => 1.0,
    };

    let overflow_x = style
        .overflow_x
        .or(style.overflow_inline)
        .unwrap_or(OverflowMode::Visible);
    let overflow_y = style
        .overflow_y
        .or(style.overflow_block)
        .unwrap_or(OverflowMode::Visible);
    let hidden = props.hidden
        || style.display == Some(DisplayMode::None)
        || style.content_visibility == Some(ContentVisibility::Hidden)
        || matches!(
            style.visibility,
            Some(VisibilityMode::Hidden | VisibilityMode::Collapse)
        );

    ResolvedStyle {
        margin,
        padding,
        border_widths,
        declared_width,
        declared_height,
        min_width,
        min_height,
        max_width,
        max_height,
        box_sizing: style.box_sizing.unwrap_or(BoxSizing::BorderBox),
        orientation: style
            .flex_direction
            .or(props.orientation)
            .unwrap_or_else(|| match style.display {
                Some(DisplayMode::Flex | DisplayMode::InlineFlex) => Orientation::Horizontal,
                _ => Orientation::Vertical,
            }),
        row_gap,
        column_gap,
        align_items: style.align_items.unwrap_or(AlignItems::Stretch),
        align_self: style.align_self.unwrap_or(SelfAlignment::Auto),
        justify_content: style.justify_content.unwrap_or(JustifyContent::Start),
        position: style.position.unwrap_or(PositionMode::Static),
        insets,
        order,
        z_index: style.z_index.unwrap_or(0),
        clip_x: overflow_x != OverflowMode::Visible,
        clip_y: overflow_y != OverflowMode::Visible,
        opacity,
        hit_testable: !props.inert
            && !style.makes_native_widget_inert()
            && style.pointer_events != Some(PointerEvents::None),
        hidden,
        explicit_width: width_value.is_some_and(|value| !matches!(value, StyleLength::Auto)),
        explicit_height: height_value.is_some_and(|value| !matches!(value, StyleLength::Auto)),
    }
}

pub(super) fn resolve_paint(
    style: &PortableStyle,
    id: &LayoutElementId,
    size: Size,
    right_to_left: bool,
    border_widths: LayoutEdgeWidths,
    opacity: f64,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> LayoutPaint {
    let current_color = style
        .color
        .as_ref()
        .and_then(|color| resolve_color(color, None));
    let background = style.background_color.as_ref().and_then(|color| {
        resolve_color_checked(color, current_color, "backgroundColor", id, diagnostics)
    });
    if style.background.is_some() && style.background_color.is_none() {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedM3StyleField,
            Some("background"),
            "the M3 scene slice supports color-only backgrounds",
        );
    }

    let border_colors = resolve_border_colors(
        style,
        current_color,
        right_to_left,
        border_widths,
        id,
        diagnostics,
    );
    let corner_radii = resolve_corner_radii(style, size, right_to_left, id, diagnostics);
    if !corner_radii.is_zero() && !border_widths.is_zero() {
        let background_can_inset = background.is_some_and(|color| color.alpha == 255);
        if border_colors.uniform().is_none() || !background_can_inset {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedRoundedBorder,
                Some("borderRadius"),
                "rounded borders currently require one border color and an opaque background",
            );
        }
    }

    LayoutPaint {
        background,
        border_widths,
        border_colors,
        corner_radii,
        opacity,
    }
}

fn diagnose_layout_modes(
    style: &PortableStyle,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) {
    if style.display.is_some_and(|display| {
        !matches!(
            display,
            DisplayMode::None
                | DisplayMode::Inline
                | DisplayMode::InlineBlock
                | DisplayMode::Flex
                | DisplayMode::InlineFlex
                | DisplayMode::Block
                | DisplayMode::FlowRoot
        )
    }) {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedLayoutMode,
            Some("display"),
            "this display mode is not implemented by the row/column M3 slice",
        );
    }
    if style
        .flex_wrap
        .is_some_and(|wrap| wrap != crate::style::FlexWrap::NoWrap)
    {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedLayoutMode,
            Some("flexWrap"),
            "wrapped flex lines are not implemented by the M3 slice",
        );
    }
    if style.position == Some(PositionMode::Sticky) {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedLayoutMode,
            Some("position"),
            "sticky positioning is not implemented by the M3 slice",
        );
    }
    if style.align_items == Some(AlignItems::Baseline)
        || style.align_self == Some(SelfAlignment::Baseline)
        || style.justify_content == Some(JustifyContent::Baseline)
    {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedLayoutMode,
            Some("alignment"),
            "baseline alignment requires the M4 text metrics path",
        );
    }
}

fn resolve_edges(
    physical: &crate::style::EdgeInsets,
    logical: &crate::style::LogicalEdgeInsets,
    basis: f64,
    allow_negative: bool,
    right_to_left: bool,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Edges {
    let mut edges = Edges {
        top: resolve_edge_value(
            physical.top.as_ref(),
            basis,
            allow_negative,
            field,
            id,
            diagnostics,
        ),
        right: resolve_edge_value(
            physical.right.as_ref(),
            basis,
            allow_negative,
            field,
            id,
            diagnostics,
        ),
        bottom: resolve_edge_value(
            physical.bottom.as_ref(),
            basis,
            allow_negative,
            field,
            id,
            diagnostics,
        ),
        left: resolve_edge_value(
            physical.left.as_ref(),
            basis,
            allow_negative,
            field,
            id,
            diagnostics,
        ),
    };
    if let Some(value) = logical.block_start.as_ref() {
        edges.top = resolve_edge_value(Some(value), basis, allow_negative, field, id, diagnostics);
    }
    if let Some(value) = logical.block_end.as_ref() {
        edges.bottom =
            resolve_edge_value(Some(value), basis, allow_negative, field, id, diagnostics);
    }
    if let Some(value) = logical.inline_start.as_ref() {
        let value = resolve_edge_value(Some(value), basis, allow_negative, field, id, diagnostics);
        if right_to_left {
            edges.right = value;
        } else {
            edges.left = value;
        }
    }
    if let Some(value) = logical.inline_end.as_ref() {
        let value = resolve_edge_value(Some(value), basis, allow_negative, field, id, diagnostics);
        if right_to_left {
            edges.left = value;
        } else {
            edges.right = value;
        }
    }
    edges
}

fn resolve_edge_value(
    value: Option<&StyleLength>,
    basis: f64,
    allow_negative: bool,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> f64 {
    match value {
        None => 0.0,
        Some(StyleLength::Auto) => {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedM3StyleField,
                Some(field),
                format!("automatic {field} resolution is not implemented"),
            );
            0.0
        }
        Some(value) => {
            let resolved = resolve_length(value, basis, field, id, diagnostics).unwrap_or(0.0);
            if !allow_negative && resolved < 0.0 {
                push_error(
                    diagnostics,
                    id,
                    LayoutDiagnosticCode::UnresolvedLength,
                    Some(field),
                    format!("{field} cannot be negative"),
                );
                0.0
            } else {
                resolved
            }
        }
    }
}

fn resolve_dimension(
    value: Option<&StyleLength>,
    basis: f64,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<f64> {
    let value = value?;
    if matches!(value, StyleLength::Auto) {
        return None;
    }
    let resolved = resolve_length(value, basis, field, id, diagnostics)?;
    if resolved < 0.0 {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnresolvedLength,
            Some(field),
            format!("{field} cannot be negative"),
        );
        None
    } else {
        Some(resolved)
    }
}

fn resolve_non_negative(
    value: Option<&StyleLength>,
    basis: f64,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<f64> {
    resolve_dimension(value, basis, field, id, diagnostics)
}

fn resolve_length(
    value: &StyleLength,
    basis: f64,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<f64> {
    let resolved = match value {
        StyleLength::Points(value) => *value,
        StyleLength::Percent(value) => basis * *value / 100.0,
        StyleLength::Auto => return None,
        StyleLength::Css(value) => {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnresolvedLength,
                Some(field),
                format!("{field} value {value:?} requires a CSS expression resolver"),
            );
            return None;
        }
    };
    if resolved.is_finite() {
        Some(resolved)
    } else {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnresolvedLength,
            Some(field),
            format!("{field} resolved to a non-finite value"),
        );
        None
    }
}

fn resolve_insets(
    style: &PortableStyle,
    available: Size,
    right_to_left: bool,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Insets {
    let mut insets = Insets {
        top: resolve_inset(
            style.inset.top.as_ref(),
            available.height,
            "top",
            id,
            diagnostics,
        ),
        right: resolve_inset(
            style.inset.right.as_ref(),
            available.width,
            "right",
            id,
            diagnostics,
        ),
        bottom: resolve_inset(
            style.inset.bottom.as_ref(),
            available.height,
            "bottom",
            id,
            diagnostics,
        ),
        left: resolve_inset(
            style.inset.left.as_ref(),
            available.width,
            "left",
            id,
            diagnostics,
        ),
    };
    if let Some(value) = style.logical_inset.block_start.as_ref() {
        insets.top = resolve_inset(
            Some(value),
            available.height,
            "insetBlockStart",
            id,
            diagnostics,
        );
    }
    if let Some(value) = style.logical_inset.block_end.as_ref() {
        insets.bottom = resolve_inset(
            Some(value),
            available.height,
            "insetBlockEnd",
            id,
            diagnostics,
        );
    }
    if let Some(value) = style.logical_inset.inline_start.as_ref() {
        let value = resolve_inset(
            Some(value),
            available.width,
            "insetInlineStart",
            id,
            diagnostics,
        );
        if right_to_left {
            insets.right = value;
        } else {
            insets.left = value;
        }
    }
    if let Some(value) = style.logical_inset.inline_end.as_ref() {
        let value = resolve_inset(
            Some(value),
            available.width,
            "insetInlineEnd",
            id,
            diagnostics,
        );
        if right_to_left {
            insets.left = value;
        } else {
            insets.right = value;
        }
    }
    insets
}

fn resolve_inset(
    value: Option<&StyleLength>,
    basis: f64,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<f64> {
    let value = value?;
    if matches!(value, StyleLength::Auto) {
        return None;
    }
    resolve_length(value, basis, field, id, diagnostics)
}

fn parse_integer(
    value: Option<&str>,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<i32> {
    let value = value?;
    match value.trim().parse::<i32>() {
        Ok(value) => Some(value),
        Err(_) => {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedM3StyleField,
                Some(field),
                format!("{field} must be an integer in the M3 layout slice"),
            );
            None
        }
    }
}

fn apply_border_styles(
    style: &PortableStyle,
    id: &LayoutElementId,
    right_to_left: bool,
    widths: &mut LayoutEdgeWidths,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) {
    let base = style.border_style;
    let mut top = style.border_styles.top.or(base);
    let mut right = style.border_styles.right.or(base);
    let mut bottom = style.border_styles.bottom.or(base);
    let mut left = style.border_styles.left.or(base);
    top = style.logical_border_styles.block_start.or(top);
    bottom = style.logical_border_styles.block_end.or(bottom);
    if right_to_left {
        right = style.logical_border_styles.inline_start.or(right);
        left = style.logical_border_styles.inline_end.or(left);
    } else {
        left = style.logical_border_styles.inline_start.or(left);
        right = style.logical_border_styles.inline_end.or(right);
    }
    widths.top = painted_border_width(widths.top, top, "borderTopStyle", id, diagnostics);
    widths.right = painted_border_width(widths.right, right, "borderRightStyle", id, diagnostics);
    widths.bottom =
        painted_border_width(widths.bottom, bottom, "borderBottomStyle", id, diagnostics);
    widths.left = painted_border_width(widths.left, left, "borderLeftStyle", id, diagnostics);
}

fn painted_border_width(
    width: f64,
    style: Option<BorderStyle>,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> f64 {
    if width <= 0.0 {
        return 0.0;
    }
    match style {
        None | Some(BorderStyle::Solid) => width,
        Some(BorderStyle::None | BorderStyle::Hidden) => 0.0,
        Some(_) => {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedBorderStyle,
                Some(field),
                "only solid borders are implemented by the Graphics rectangle slice",
            );
            width
        }
    }
}

fn resolve_border_colors(
    style: &PortableStyle,
    current_color: Option<LayoutColor>,
    right_to_left: bool,
    widths: LayoutEdgeWidths,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> LayoutEdgeColors {
    let base = style.border_color.as_ref();
    let mut top = style.border_colors.top.as_ref().or(base);
    let mut right = style.border_colors.right.as_ref().or(base);
    let mut bottom = style.border_colors.bottom.as_ref().or(base);
    let mut left = style.border_colors.left.as_ref().or(base);
    top = style.logical_border_colors.block_start.as_ref().or(top);
    bottom = style.logical_border_colors.block_end.as_ref().or(bottom);
    if right_to_left {
        right = style.logical_border_colors.inline_start.as_ref().or(right);
        left = style.logical_border_colors.inline_end.as_ref().or(left);
    } else {
        left = style.logical_border_colors.inline_start.as_ref().or(left);
        right = style.logical_border_colors.inline_end.as_ref().or(right);
    }
    LayoutEdgeColors {
        top: resolve_optional_border_color(
            top,
            current_color,
            widths.top,
            "borderTopColor",
            id,
            diagnostics,
        ),
        right: resolve_optional_border_color(
            right,
            current_color,
            widths.right,
            "borderRightColor",
            id,
            diagnostics,
        ),
        bottom: resolve_optional_border_color(
            bottom,
            current_color,
            widths.bottom,
            "borderBottomColor",
            id,
            diagnostics,
        ),
        left: resolve_optional_border_color(
            left,
            current_color,
            widths.left,
            "borderLeftColor",
            id,
            diagnostics,
        ),
    }
}

fn resolve_optional_border_color(
    color: Option<&StyleColor>,
    current_color: Option<LayoutColor>,
    width: f64,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<LayoutColor> {
    if width == 0.0 {
        return None;
    }
    match color {
        Some(color) => resolve_color_checked(color, current_color, field, id, diagnostics),
        None => current_color.or_else(|| {
            push_error(
                diagnostics,
                id,
                LayoutDiagnosticCode::UnsupportedColor,
                Some(field),
                "a painted border needs an explicit or resolvable current color",
            );
            None
        }),
    }
}

fn resolve_color_checked(
    color: &StyleColor,
    current_color: Option<LayoutColor>,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> Option<LayoutColor> {
    resolve_color(color, current_color).or_else(|| {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedColor,
            Some(field),
            format!("{field} requires a color function, keyword, or inheritance resolver"),
        );
        None
    })
}

fn resolve_color(color: &StyleColor, current_color: Option<LayoutColor>) -> Option<LayoutColor> {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } => Some(LayoutColor::rgba(*red, *green, *blue, *alpha)),
        StyleColor::Function(_) => None,
        StyleColor::Keyword(keyword) => match keyword.trim().to_ascii_lowercase().as_str() {
            "transparent" => Some(LayoutColor::TRANSPARENT),
            "black" => Some(LayoutColor::BLACK),
            "white" => Some(LayoutColor::WHITE),
            "red" => Some(LayoutColor::rgba(255, 0, 0, 255)),
            "green" => Some(LayoutColor::rgba(0, 128, 0, 255)),
            "blue" => Some(LayoutColor::rgba(0, 0, 255, 255)),
            "yellow" => Some(LayoutColor::rgba(255, 255, 0, 255)),
            "gray" | "grey" => Some(LayoutColor::rgba(128, 128, 128, 255)),
            "currentcolor" => current_color,
            _ => None,
        },
    }
}

fn resolve_corner_radii(
    style: &PortableStyle,
    size: Size,
    right_to_left: bool,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> LayoutCornerRadii {
    let base = style.border_radius.as_ref();
    let mut top_left = style.border_radii.top_left.as_ref();
    let mut top_right = style.border_radii.top_right.as_ref();
    let mut bottom_right = style.border_radii.bottom_right.as_ref();
    let mut bottom_left = style.border_radii.bottom_left.as_ref();
    if right_to_left {
        top_right = style
            .logical_border_radii
            .start_start
            .as_ref()
            .or(top_right);
        top_left = style.logical_border_radii.start_end.as_ref().or(top_left);
        bottom_left = style.logical_border_radii.end_end.as_ref().or(bottom_left);
        bottom_right = style
            .logical_border_radii
            .end_start
            .as_ref()
            .or(bottom_right);
    } else {
        top_left = style.logical_border_radii.start_start.as_ref().or(top_left);
        top_right = style.logical_border_radii.start_end.as_ref().or(top_right);
        bottom_right = style.logical_border_radii.end_end.as_ref().or(bottom_right);
        bottom_left = style
            .logical_border_radii
            .end_start
            .as_ref()
            .or(bottom_left);
    }
    normalize_corner_radii(
        LayoutCornerRadii {
            top_left: resolve_radius(top_left, base, size, "borderTopLeftRadius", id, diagnostics),
            top_right: resolve_radius(
                top_right,
                base,
                size,
                "borderTopRightRadius",
                id,
                diagnostics,
            ),
            bottom_right: resolve_radius(
                bottom_right,
                base,
                size,
                "borderBottomRightRadius",
                id,
                diagnostics,
            ),
            bottom_left: resolve_radius(
                bottom_left,
                base,
                size,
                "borderBottomLeftRadius",
                id,
                diagnostics,
            ),
        },
        size,
    )
}

fn resolve_radius(
    corner: Option<&crate::style::CornerRadius>,
    fallback: Option<&StyleLength>,
    size: Size,
    field: &str,
    id: &LayoutElementId,
    diagnostics: &mut Vec<LayoutDiagnostic>,
) -> f64 {
    let Some(horizontal) = corner.map(|corner| &corner.horizontal).or(fallback) else {
        return 0.0;
    };
    let vertical = corner
        .and_then(|corner| corner.vertical.as_ref())
        .unwrap_or(horizontal);
    let horizontal =
        resolve_non_negative(Some(horizontal), size.width, field, id, diagnostics).unwrap_or(0.0);
    let vertical =
        resolve_non_negative(Some(vertical), size.height, field, id, diagnostics).unwrap_or(0.0);
    if (vertical - horizontal).abs() > f64::EPSILON {
        push_error(
            diagnostics,
            id,
            LayoutDiagnosticCode::UnsupportedM3StyleField,
            Some(field),
            "elliptical corner radii are not implemented by the Graphics scene schema",
        );
    }
    horizontal
}

fn normalize_corner_radii(mut radii: LayoutCornerRadii, size: Size) -> LayoutCornerRadii {
    let mut scale = 1.0_f64;
    for (limit, sum) in [
        (size.width, radii.top_left + radii.top_right),
        (size.width, radii.bottom_left + radii.bottom_right),
        (size.height, radii.top_left + radii.bottom_left),
        (size.height, radii.top_right + radii.bottom_right),
    ] {
        if sum > limit && sum > 0.0 {
            scale = scale.min(limit.max(0.0) / sum);
        }
    }
    radii.top_left *= scale;
    radii.top_right *= scale;
    radii.bottom_right *= scale;
    radii.bottom_left *= scale;
    radii
}
