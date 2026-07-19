use super::*;

pub(in crate::style) fn tailwind_prefixed_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
        Some(("width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
        Some(("height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
        Some(("min-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
        Some(("min-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class
        .strip_prefix("max-w-")
        .and_then(tailwind_max_width_css)
    {
        Some(("max-width".to_string(), value))
    } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
        Some(("max-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
        Some(("gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
        Some(("column-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
        Some(("row-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
        Some(("opacity".to_string(), trim_float(value)))
    } else if let Some(value) = tailwind_z_index(class) {
        Some(("z-index".to_string(), value))
    } else if let Some(value) = class.strip_prefix("flex-").and_then(tailwind_flex_value) {
        Some(("flex".to_string(), value))
    } else if let Some(value) = class.strip_prefix("basis-").and_then(tailwind_basis_value) {
        Some(("flex-basis".to_string(), value))
    } else if let Some(value) = class.strip_prefix("grow-").and_then(tailwind_number_token) {
        Some(("flex-grow".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("shrink-")
        .and_then(tailwind_number_token)
    {
        Some(("flex-shrink".to_string(), value))
    } else if let Some(value) = tailwind_order_value(class) {
        Some(("order".to_string(), value))
    } else if let Some(value) = class.strip_prefix("bg-").and_then(tailwind_color_css) {
        Some(("background-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("border-").and_then(tailwind_color_css) {
        Some(("border-color".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("accent-")
        .and_then(tailwind_accent_color_css)
    {
        Some(("accent-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("caret-").and_then(tailwind_color_css) {
        Some(("caret-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("font-").and_then(tailwind_font_family) {
        Some(("font-family".to_string(), value))
    } else if let Some(value) = tailwind_letter_spacing(class) {
        Some(("letter-spacing".to_string(), value))
    } else if let Some((property, value)) = tailwind_decoration_declaration(class) {
        Some((property, value))
    } else if let Some(value) = class
        .strip_prefix("underline-offset-")
        .and_then(tailwind_underline_offset)
    {
        Some(("text-underline-offset".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("leading-")
        .and_then(tailwind_line_height)
    {
        Some(("line-height".to_string(), value))
    } else if let Some(value) = tailwind_text_indent(class) {
        Some(("text-indent".to_string(), value))
    } else if let Some(value) = class.strip_prefix("text-").and_then(tailwind_color_css) {
        Some(("color".to_string(), value))
    } else {
        None
    }
}

pub(in crate::style) fn tailwind_size_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let value = class.strip_prefix("size-").and_then(tailwind_length)?;
    let value = style_length_css(value);
    let mut declarations = BTreeMap::new();
    declarations.insert("width".to_string(), value.clone());
    declarations.insert("height".to_string(), value);
    Some(declarations)
}

pub(in crate::style) fn tailwind_content_declaration(class: &str) -> Option<(String, String)> {
    if class == "content-none" {
        return Some(("content".to_string(), "none".to_string()));
    }
    if let Some(value) = class
        .strip_prefix("content-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "content".to_string(),
            tailwind_arbitrary_content_value(value),
        ));
    }
    class
        .strip_prefix("content-")
        .and_then(tailwind_custom_var)
        .map(|value| ("content".to_string(), value))
}

pub(in crate::style) fn tailwind_screen_reader_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "sr-only" => {
            declarations.insert("position".to_string(), "absolute".to_string());
            declarations.insert("width".to_string(), "1px".to_string());
            declarations.insert("height".to_string(), "1px".to_string());
            declarations.insert("padding".to_string(), "0".to_string());
            declarations.insert("margin".to_string(), "-1px".to_string());
            declarations.insert("overflow".to_string(), "hidden".to_string());
            declarations.insert("clip".to_string(), "rect(0, 0, 0, 0)".to_string());
            declarations.insert("white-space".to_string(), "nowrap".to_string());
            declarations.insert("border-width".to_string(), "0".to_string());
        }
        "not-sr-only" => {
            declarations.insert("position".to_string(), "static".to_string());
            declarations.insert("width".to_string(), "auto".to_string());
            declarations.insert("height".to_string(), "auto".to_string());
            declarations.insert("padding".to_string(), "0".to_string());
            declarations.insert("margin".to_string(), "0".to_string());
            declarations.insert("overflow".to_string(), "visible".to_string());
            declarations.insert("clip".to_string(), "auto".to_string());
            declarations.insert("white-space".to_string(), "normal".to_string());
            declarations.insert("border-width".to_string(), "0".to_string());
        }
        _ => return None,
    }
    Some(declarations)
}

pub(in crate::style) fn tailwind_svg_presentation_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(value) = class
        .strip_prefix("fill-")
        .and_then(tailwind_svg_paint_value)
    {
        declarations.insert("fill".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("stroke-") {
        let (property, value) = tailwind_stroke_declaration(value)?;
        declarations.insert(property, value);
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_svg_paint_value(value: &str) -> Option<String> {
    if value == "none" {
        return Some("none".to_string());
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_color_css(value).or_else(|| tailwind_custom_var(value))
}

pub(in crate::style) fn tailwind_stroke_declaration(value: &str) -> Option<(String, String)> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(("stroke-width".to_string(), value));
    }
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(("stroke".to_string(), value));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if is_likely_stroke_width_value(&value) {
            return Some(("stroke-width".to_string(), value));
        }
        return Some(("stroke".to_string(), value));
    }
    if let Ok(width) = value.parse::<f64>() {
        return Some(("stroke-width".to_string(), trim_float(width)));
    }
    tailwind_svg_paint_value(value).map(|value| ("stroke".to_string(), value))
}

pub(in crate::style) fn is_likely_stroke_width_value(value: &str) -> bool {
    !value.trim().starts_with("var(") && parse_length(value).is_some()
}

pub(in crate::style) fn tailwind_radius_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let suffix = class.strip_prefix("rounded")?;
    let (physical, logical, value) = if suffix.is_empty() {
        (Some(CornerSelection::All), None, "xs")
    } else {
        let suffix = suffix.strip_prefix('-')?;
        if let Some(value) = suffix.strip_prefix("ss-") {
            (None, Some(LogicalCornerSelection::StartStart), value)
        } else if let Some(value) = suffix.strip_prefix("se-") {
            (None, Some(LogicalCornerSelection::StartEnd), value)
        } else if let Some(value) = suffix.strip_prefix("ee-") {
            (None, Some(LogicalCornerSelection::EndEnd), value)
        } else if let Some(value) = suffix.strip_prefix("es-") {
            (None, Some(LogicalCornerSelection::EndStart), value)
        } else if let Some(value) = suffix.strip_prefix("s-") {
            (None, Some(LogicalCornerSelection::Start), value)
        } else if let Some(value) = suffix.strip_prefix("e-") {
            (None, Some(LogicalCornerSelection::End), value)
        } else if let Some(value) = suffix.strip_prefix("tl-") {
            (Some(CornerSelection::TopLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("tr-") {
            (Some(CornerSelection::TopRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("br-") {
            (Some(CornerSelection::BottomRight), None, value)
        } else if let Some(value) = suffix.strip_prefix("bl-") {
            (Some(CornerSelection::BottomLeft), None, value)
        } else if let Some(value) = suffix.strip_prefix("t-") {
            (Some(CornerSelection::Top), None, value)
        } else if let Some(value) = suffix.strip_prefix("r-") {
            (Some(CornerSelection::Right), None, value)
        } else if let Some(value) = suffix.strip_prefix("b-") {
            (Some(CornerSelection::Bottom), None, value)
        } else if let Some(value) = suffix.strip_prefix("l-") {
            (Some(CornerSelection::Left), None, value)
        } else {
            (Some(CornerSelection::All), None, suffix)
        }
    };
    let radius = CornerRadius::circular(tailwind_radius_value(value)?);
    if let Some(selection) = physical {
        insert_corner_radius_declarations(&mut declarations, selection, radius);
    } else if let Some(selection) = logical {
        insert_logical_corner_radius_declarations(&mut declarations, selection, radius);
    }
    Some(declarations)
}

pub(in crate::style) fn tailwind_radius_value(value: &str) -> Option<StyleLength> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(StyleLength::Css(value));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some(StyleLength::Points(0.0)),
        "xs" => Some(StyleLength::Points(4.0)),
        "sm" => Some(StyleLength::Points(6.0)),
        "md" => Some(StyleLength::Points(8.0)),
        "lg" => Some(StyleLength::Points(12.0)),
        "xl" => Some(StyleLength::Points(16.0)),
        "xxl" => Some(StyleLength::Points(24.0)),
        "pill" | "full" => Some(StyleLength::Css("calc(infinity * 1px)".to_string())),
        _ => None,
    }
}

pub(in crate::style) fn tailwind_formatting_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "box-border" => Some(("box-sizing", "border-box".to_string())),
        "box-content" => Some(("box-sizing", "content-box".to_string())),
        "box-decoration-slice" => Some(("box-decoration-break", "slice".to_string())),
        "box-decoration-clone" => Some(("box-decoration-break", "clone".to_string())),
        "isolate" => Some(("isolation", "isolate".to_string())),
        "isolation-auto" => Some(("isolation", "auto".to_string())),
        "float-right" => Some(("float", "right".to_string())),
        "float-left" => Some(("float", "left".to_string())),
        "float-start" => Some(("float", "inline-start".to_string())),
        "float-end" => Some(("float", "inline-end".to_string())),
        "float-none" => Some(("float", "none".to_string())),
        "clear-right" => Some(("clear", "right".to_string())),
        "clear-left" => Some(("clear", "left".to_string())),
        "clear-both" => Some(("clear", "both".to_string())),
        "clear-start" => Some(("clear", "inline-start".to_string())),
        "clear-end" => Some(("clear", "inline-end".to_string())),
        "clear-none" => Some(("clear", "none".to_string())),
        "align-baseline" => Some(("vertical-align", "baseline".to_string())),
        "align-top" => Some(("vertical-align", "top".to_string())),
        "align-middle" => Some(("vertical-align", "middle".to_string())),
        "align-bottom" => Some(("vertical-align", "bottom".to_string())),
        "align-text-top" => Some(("vertical-align", "text-top".to_string())),
        "align-text-bottom" => Some(("vertical-align", "text-bottom".to_string())),
        "align-sub" => Some(("vertical-align", "sub".to_string())),
        "align-super" => Some(("vertical-align", "super".to_string())),
        "table-auto" => Some(("table-layout", "auto".to_string())),
        "table-fixed" => Some(("table-layout", "fixed".to_string())),
        "border-collapse" => Some(("border-collapse", "collapse".to_string())),
        "border-separate" => Some(("border-collapse", "separate".to_string())),
        "caption-top" => Some(("caption-side", "top".to_string())),
        "caption-bottom" => Some(("caption-side", "bottom".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("align-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "vertical-align".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("align-").and_then(tailwind_custom_var) {
        declarations.insert("vertical-align".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_border_spacing_declaration(class) {
        insert_tailwind_border_spacing_declarations(&mut declarations, axis, value);
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_space_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "space-x-reverse" => {
            declarations.insert("--tw-space-x-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "space-y-reverse" => {
            declarations.insert("--tw-space-y-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    let (axis, value) = if let Some(value) = class.strip_prefix("space-x-") {
        ("x", value)
    } else if let Some(value) = class.strip_prefix("space-y-") {
        ("y", value)
    } else if let Some(value) = class.strip_prefix("-space-x-") {
        ("x", value)
    } else if let Some(value) = class.strip_prefix("-space-y-") {
        ("y", value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if class.starts_with("-space-") {
        length = negate_style_length(length)?;
    }
    declarations.insert(format!("--tw-space-{axis}-reverse"), "0".to_string());
    declarations.insert(format!("space-{axis}"), style_length_css(length));
    Some(declarations)
}

pub(in crate::style) fn tailwind_divide_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "divide-x" => {
            declarations.insert("--tw-divide-x-reverse".to_string(), "0".to_string());
            declarations.insert("divide-x-width".to_string(), "1px".to_string());
            return Some(declarations);
        }
        "divide-y" => {
            declarations.insert("--tw-divide-y-reverse".to_string(), "0".to_string());
            declarations.insert("divide-y-width".to_string(), "1px".to_string());
            return Some(declarations);
        }
        "divide-x-reverse" => {
            declarations.insert("--tw-divide-x-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "divide-y-reverse" => {
            declarations.insert("--tw-divide-y-reverse".to_string(), "1".to_string());
            return Some(declarations);
        }
        "divide-solid" => {
            declarations.insert("divide-style".to_string(), "solid".to_string());
            return Some(declarations);
        }
        "divide-dashed" => {
            declarations.insert("divide-style".to_string(), "dashed".to_string());
            return Some(declarations);
        }
        "divide-dotted" => {
            declarations.insert("divide-style".to_string(), "dotted".to_string());
            return Some(declarations);
        }
        "divide-double" => {
            declarations.insert("divide-style".to_string(), "double".to_string());
            return Some(declarations);
        }
        "divide-hidden" => {
            declarations.insert("divide-style".to_string(), "hidden".to_string());
            return Some(declarations);
        }
        "divide-none" => {
            declarations.insert("divide-style".to_string(), "none".to_string());
            return Some(declarations);
        }
        _ => {}
    }
    if let Some(value) = class.strip_prefix("divide-x-") {
        declarations.insert("--tw-divide-x-reverse".to_string(), "0".to_string());
        declarations.insert(
            "divide-x-width".to_string(),
            style_length_css(tailwind_divide_width(value)?),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("divide-y-") {
        declarations.insert("--tw-divide-y-reverse".to_string(), "0".to_string());
        declarations.insert(
            "divide-y-width".to_string(),
            style_length_css(tailwind_divide_width(value)?),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("divide-") {
        declarations.insert(
            "divide-color".to_string(),
            tailwind_divide_color_value(value)?,
        );
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_divide_width(value: &str) -> Option<StyleLength> {
    if let Some(value) = tailwind_typed_custom_var(value, "length") {
        return Some(StyleLength::Css(value));
    }
    tailwind_border_width(value)
}

pub(in crate::style) fn tailwind_divide_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_border_color_value(value)
}

pub(in crate::style) fn tailwind_container_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let (base, name) = class.split_once('/').unwrap_or((class, ""));
    let container_type = match base {
        "@container" => "inline-size",
        "@container-size" => "size",
        "@container-normal" => "normal",
        _ => return None,
    };
    let mut declarations = BTreeMap::new();
    declarations.insert("container-type".to_string(), container_type.to_string());
    if !name.is_empty() {
        declarations.insert("container-name".to_string(), tailwind_container_name(name));
    }
    Some(declarations)
}

pub(in crate::style) fn tailwind_container_name(value: &str) -> String {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        tailwind_arbitrary_value(arbitrary)
    } else {
        value.to_string()
    }
}

pub(in crate::style) fn tailwind_border_spacing_declaration(
    class: &str,
) -> Option<(SpacingAxis, String)> {
    let (axis, value) = if let Some(value) = class.strip_prefix("border-spacing-x-") {
        (SpacingAxis::X, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-y-") {
        (SpacingAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("border-spacing-") {
        (SpacingAxis::Both, value)
    } else {
        return None;
    };
    Some((axis, tailwind_spacing_value(value)?))
}

#[derive(Debug, Clone, Copy)]
pub(in crate::style) enum SpacingAxis {
    Both,
    X,
    Y,
}

pub(in crate::style) fn insert_tailwind_border_spacing_declarations(
    declarations: &mut BTreeMap<String, String>,
    axis: SpacingAxis,
    value: String,
) {
    match axis {
        SpacingAxis::Both => {
            declarations.insert("--tw-border-spacing-x".to_string(), value.clone());
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
        SpacingAxis::X => {
            declarations.insert("--tw-border-spacing-x".to_string(), value);
        }
        SpacingAxis::Y => {
            declarations.insert("--tw-border-spacing-y".to_string(), value);
        }
    }
    declarations.insert(
        "border-spacing".to_string(),
        "var(--tw-border-spacing-x) var(--tw-border-spacing-y)".to_string(),
    );
}

pub(in crate::style) fn tailwind_spacing_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| tailwind_length(value).map(style_length_css))
}
