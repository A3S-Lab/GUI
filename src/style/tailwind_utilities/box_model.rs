use super::*;

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
