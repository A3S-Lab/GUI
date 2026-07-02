use super::*;

pub(in crate::style) fn tailwind_ring_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "ring" => {
            insert_tailwind_ring_width_declarations(&mut declarations, false, "1px".to_string());
            return Some(declarations);
        }
        "ring-inset" => {
            declarations.insert("--tw-ring-inset".to_string(), "inset".to_string());
            declarations.insert(
                "box-shadow".to_string(),
                tailwind_box_shadow_pipeline().to_string(),
            );
            return Some(declarations);
        }
        "inset-ring" => {
            insert_tailwind_ring_width_declarations(&mut declarations, true, "1px".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    if let Some(value) = class.strip_prefix("ring-") {
        if let Some(width) = tailwind_ring_width(value) {
            insert_tailwind_ring_width_declarations(&mut declarations, false, width);
            return Some(declarations);
        }
        declarations.insert(
            "--tw-ring-color".to_string(),
            tailwind_ring_color_value(value)?,
        );
        declarations.insert(
            "box-shadow".to_string(),
            tailwind_box_shadow_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("inset-ring-") {
        if let Some(width) = tailwind_ring_width(value) {
            insert_tailwind_ring_width_declarations(&mut declarations, true, width);
            return Some(declarations);
        }
        declarations.insert(
            "--tw-inset-ring-color".to_string(),
            tailwind_ring_color_value(value)?,
        );
        declarations.insert(
            "box-shadow".to_string(),
            tailwind_box_shadow_pipeline().to_string(),
        );
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn insert_tailwind_ring_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    inset: bool,
    width: String,
) {
    let property = if inset {
        "--tw-inset-ring-shadow"
    } else {
        "--tw-ring-shadow"
    };
    let prefix = if inset { "inset " } else { "" };
    declarations.insert(property.to_string(), format!("{prefix}0 0 0 {width}"));
    declarations.insert(
        "box-shadow".to_string(),
        tailwind_box_shadow_pipeline().to_string(),
    );
}

pub(in crate::style) fn tailwind_ring_width(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value
        .parse::<f64>()
        .ok()
        .map(|value| format!("{}px", trim_float(value)))
}

pub(in crate::style) fn tailwind_ring_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_border_color_value(value)
}

pub(in crate::style) fn tailwind_box_shadow_pipeline() -> &'static str {
    "var(--tw-inset-ring-shadow), var(--tw-ring-shadow)"
}

pub(in crate::style) fn compose_tailwind_ring_shadow(
    shadow: &str,
    color: Option<&str>,
    force_inset: bool,
) -> String {
    let mut shadow = shadow.trim().to_string();
    if force_inset && !shadow.starts_with("inset ") {
        shadow = format!("inset {shadow}");
    }
    let Some(color) = color.map(str::trim).filter(|color| !color.is_empty()) else {
        return shadow;
    };
    if shadow.contains(color) {
        shadow
    } else {
        format!("{shadow} {color}")
    }
}

pub(in crate::style) fn tailwind_grid_declaration(class: &str) -> Option<(String, String)> {
    let declaration = match class {
        "grid-flow-row" => Some(("grid-auto-flow", "row".to_string())),
        "grid-flow-col" => Some(("grid-auto-flow", "column".to_string())),
        "grid-flow-dense" => Some(("grid-auto-flow", "dense".to_string())),
        "grid-flow-row-dense" => Some(("grid-auto-flow", "row dense".to_string())),
        "grid-flow-col-dense" => Some(("grid-auto-flow", "column dense".to_string())),
        "auto-cols-auto" => Some(("grid-auto-columns", "auto".to_string())),
        "auto-cols-min" => Some(("grid-auto-columns", "min-content".to_string())),
        "auto-cols-max" => Some(("grid-auto-columns", "max-content".to_string())),
        "auto-cols-fr" => Some(("grid-auto-columns", "minmax(0, 1fr)".to_string())),
        "auto-rows-auto" => Some(("grid-auto-rows", "auto".to_string())),
        "auto-rows-min" => Some(("grid-auto-rows", "min-content".to_string())),
        "auto-rows-max" => Some(("grid-auto-rows", "max-content".to_string())),
        "auto-rows-fr" => Some(("grid-auto-rows", "minmax(0, 1fr)".to_string())),
        "col-auto" => Some(("grid-column", "auto".to_string())),
        "col-span-full" => Some(("grid-column", "1 / -1".to_string())),
        "row-auto" => Some(("grid-row", "auto".to_string())),
        "row-span-full" => Some(("grid-row", "1 / -1".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        return Some((property.to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-cols-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("grid-rows-")
        .and_then(tailwind_grid_track_list)
    {
        return Some(("grid-template-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-cols-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-columns".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("auto-rows-")
        .and_then(tailwind_grid_auto_track)
    {
        return Some(("grid-auto-rows".to_string(), value));
    }
    if let Some(value) = class
        .strip_prefix("col-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-column".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("row-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("grid-row".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("col-").and_then(tailwind_custom_var) {
        return Some(("grid-column".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("row-").and_then(tailwind_custom_var) {
        return Some(("grid-row".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("col-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-column".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = class.strip_prefix("row-span-").and_then(tailwind_grid_line) {
        return Some((
            "grid-row".to_string(),
            format!("span {value} / span {value}"),
        ));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-start-") {
        return Some(("grid-column-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "col-end-") {
        return Some(("grid-column-end".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-start-") {
        return Some(("grid-row-start".to_string(), value));
    }
    if let Some(value) = tailwind_grid_line_utility(class, "row-end-") {
        return Some(("grid-row-end".to_string(), value));
    }
    None
}

pub(in crate::style) fn tailwind_grid_track_list(value: &str) -> Option<String> {
    if matches!(value, "none" | "subgrid") {
        return Some(value.to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let count = value.parse::<u16>().ok()?;
    if count == 0 {
        return None;
    }
    Some(format!("repeat({count}, minmax(0, 1fr))"))
}

pub(in crate::style) fn tailwind_grid_auto_track(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    match value {
        "auto" => Some("auto".to_string()),
        "min" => Some("min-content".to_string()),
        "max" => Some("max-content".to_string()),
        "fr" => Some("minmax(0, 1fr)".to_string()),
        _ => None,
    }
}

pub(in crate::style) fn tailwind_grid_line_utility(class: &str, prefix: &str) -> Option<String> {
    if let Some(value) = class.strip_prefix(prefix).and_then(tailwind_grid_line) {
        return Some(value);
    }
    let negative_prefix = format!("-{prefix}");
    let value = class
        .strip_prefix(&negative_prefix)
        .and_then(tailwind_grid_line)?;
    Some(format!("calc({value} * -1)"))
}

pub(in crate::style) fn tailwind_grid_line(value: &str) -> Option<String> {
    if value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<u16>().ok().map(|value| value.to_string())
}

pub(in crate::style) fn insert_edge_declarations(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert(prefix.to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert(format!("{prefix}-inline"), value);
        }
        EdgeSelection::Y => {
            declarations.insert(format!("{prefix}-block"), value);
        }
        EdgeSelection::Top => {
            declarations.insert(format!("{prefix}-top"), value);
        }
        EdgeSelection::Right => {
            declarations.insert(format!("{prefix}-right"), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert(format!("{prefix}-bottom"), value);
        }
        EdgeSelection::Left => {
            declarations.insert(format!("{prefix}-left"), value);
        }
    }
}

pub(in crate::style) fn insert_position_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("inset".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("inset-inline".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("inset-block".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("top".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("right".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("bottom".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("left".to_string(), value);
        }
    }
}

pub(in crate::style) fn insert_logical_edge_declaration(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => format!("{prefix}-block"),
        LogicalEdgeSelection::Inline => format!("{prefix}-inline"),
        LogicalEdgeSelection::BlockStart => format!("{prefix}-block-start"),
        LogicalEdgeSelection::BlockEnd => format!("{prefix}-block-end"),
        LogicalEdgeSelection::InlineStart => format!("{prefix}-inline-start"),
        LogicalEdgeSelection::InlineEnd => format!("{prefix}-inline-end"),
    };
    declarations.insert(property, value);
}

pub(in crate::style) fn insert_logical_position_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "inset-block",
        LogicalEdgeSelection::Inline => "inset-inline",
        LogicalEdgeSelection::BlockStart => "inset-block-start",
        LogicalEdgeSelection::BlockEnd => "inset-block-end",
        LogicalEdgeSelection::InlineStart => "inset-inline-start",
        LogicalEdgeSelection::InlineEnd => "inset-inline-end",
    };
    declarations.insert(property.to_string(), value);
}

pub(in crate::style) fn insert_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: CornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        CornerSelection::All => {
            declarations.insert("border-radius".to_string(), value);
        }
        CornerSelection::Top => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::Right => {
            declarations.insert("border-top-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::Bottom => {
            declarations.insert("border-bottom-right-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::Left => {
            declarations.insert("border-top-left-radius".to_string(), value.clone());
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
        CornerSelection::TopLeft => {
            declarations.insert("border-top-left-radius".to_string(), value);
        }
        CornerSelection::TopRight => {
            declarations.insert("border-top-right-radius".to_string(), value);
        }
        CornerSelection::BottomRight => {
            declarations.insert("border-bottom-right-radius".to_string(), value);
        }
        CornerSelection::BottomLeft => {
            declarations.insert("border-bottom-left-radius".to_string(), value);
        }
    }
}

pub(in crate::style) fn insert_logical_corner_radius_declarations(
    declarations: &mut BTreeMap<String, String>,
    corners: LogicalCornerSelection,
    radius: CornerRadius,
) {
    let value = corner_radius_css(radius);
    match corners {
        LogicalCornerSelection::Start => {
            declarations.insert("border-start-start-radius".to_string(), value.clone());
            declarations.insert("border-end-start-radius".to_string(), value);
        }
        LogicalCornerSelection::End => {
            declarations.insert("border-start-end-radius".to_string(), value.clone());
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::StartStart => {
            declarations.insert("border-start-start-radius".to_string(), value);
        }
        LogicalCornerSelection::StartEnd => {
            declarations.insert("border-start-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndEnd => {
            declarations.insert("border-end-end-radius".to_string(), value);
        }
        LogicalCornerSelection::EndStart => {
            declarations.insert("border-end-start-radius".to_string(), value);
        }
    }
}

pub(in crate::style) fn insert_border_color_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: String,
) {
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-color".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-color".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-color".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-color".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-color".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-color".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-color".to_string(), value);
        }
    }
}

pub(in crate::style) fn insert_logical_border_color_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: String,
) {
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-color",
        LogicalEdgeSelection::Inline => "border-inline-color",
        LogicalEdgeSelection::BlockStart => "border-block-start-color",
        LogicalEdgeSelection::BlockEnd => "border-block-end-color",
        LogicalEdgeSelection::InlineStart => "border-inline-start-color",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-color",
    };
    declarations.insert(property.to_string(), value);
}

pub(in crate::style) fn insert_logical_border_width_declaration(
    declarations: &mut BTreeMap<String, String>,
    edges: LogicalEdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    let property = match edges {
        LogicalEdgeSelection::Block => "border-block-width",
        LogicalEdgeSelection::Inline => "border-inline-width",
        LogicalEdgeSelection::BlockStart => "border-block-start-width",
        LogicalEdgeSelection::BlockEnd => "border-block-end-width",
        LogicalEdgeSelection::InlineStart => "border-inline-start-width",
        LogicalEdgeSelection::InlineEnd => "border-inline-end-width",
    };
    declarations.insert(property.to_string(), value);
}

pub(in crate::style) fn insert_border_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-width".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-inline-width".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-block-width".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-width".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-width".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-width".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-width".to_string(), value);
        }
    }
}
