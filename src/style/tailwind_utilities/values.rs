use super::*;

pub(in crate::style) fn style_length_css(value: StyleLength) -> String {
    match value {
        StyleLength::Points(value) => format!("{}px", trim_float(value)),
        StyleLength::Percent(value) => format!("{}%", trim_float(value)),
        StyleLength::Auto => "auto".to_string(),
        StyleLength::Css(value) => value,
    }
}

pub(in crate::style) fn corner_radius_css(radius: CornerRadius) -> String {
    let horizontal = style_length_css(radius.horizontal);
    if let Some(vertical) = radius.vertical {
        format!("{horizontal} {}", style_length_css(vertical))
    } else {
        horizontal
    }
}

pub(in crate::style) fn trim_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

pub(in crate::style) fn is_tailwind_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_'))
}

pub(in crate::style) fn is_tailwind_cursor(value: &str) -> bool {
    matches!(
        value,
        "auto"
            | "default"
            | "pointer"
            | "wait"
            | "text"
            | "move"
            | "help"
            | "not-allowed"
            | "none"
            | "context-menu"
            | "progress"
            | "cell"
            | "crosshair"
            | "vertical-text"
            | "alias"
            | "copy"
            | "no-drop"
            | "grab"
            | "grabbing"
            | "all-scroll"
            | "col-resize"
            | "row-resize"
            | "n-resize"
            | "e-resize"
            | "s-resize"
            | "w-resize"
            | "ne-resize"
            | "nw-resize"
            | "se-resize"
            | "sw-resize"
            | "ew-resize"
            | "ns-resize"
            | "nesw-resize"
            | "nwse-resize"
            | "zoom-in"
            | "zoom-out"
    )
}

pub(in crate::style) fn tailwind_transform_declaration(class: &str) -> Option<String> {
    if let Some(suffix) = class.strip_prefix("rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate(-{value})"));
        }
    }
    if let Some(value) = class.strip_prefix("scale-").and_then(tailwind_scale_value) {
        return Some(format!("scale({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-x-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleX({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-y-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleY({value})"));
    }
    if let Some(suffix) = class.strip_prefix("translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX(-{value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY(-{value})"));
        }
    }
    None
}

pub(in crate::style) fn tailwind_rotate_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(format!("{}deg", trim_float(value.parse::<f64>().ok()?)))
}

pub(in crate::style) fn tailwind_scale_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(trim_float(value.parse::<f64>().ok()? / 100.0))
}

pub(in crate::style) fn tailwind_translate_value(value: &str) -> Option<String> {
    tailwind_length(value).map(style_length_css)
}

pub(in crate::style) fn tailwind_text_size_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix("text-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if parse_length(&value).is_some() {
            declarations.insert("font-size".to_string(), value);
            return Some(declarations);
        }
        return None;
    }
    let (font_size, line_height) = match class {
        "text-xs" => ("0.75rem", "1rem"),
        "text-sm" => ("0.875rem", "1.25rem"),
        "text-base" => ("1rem", "1.5rem"),
        "text-lg" => ("1.125rem", "1.75rem"),
        "text-xl" => ("1.25rem", "1.75rem"),
        "text-2xl" => ("1.5rem", "2rem"),
        "text-3xl" => ("1.875rem", "2.25rem"),
        "text-4xl" => ("2.25rem", "2.5rem"),
        "text-5xl" => ("3rem", "1"),
        "text-6xl" => ("3.75rem", "1"),
        "text-7xl" => ("4.5rem", "1"),
        "text-8xl" => ("6rem", "1"),
        "text-9xl" => ("8rem", "1"),
        _ => return None,
    };
    declarations.insert("font-size".to_string(), font_size.to_string());
    declarations.insert("line-height".to_string(), line_height.to_string());
    Some(declarations)
}

pub(in crate::style) fn tailwind_font_feature_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some((property, value)) = tailwind_font_variant_numeric_declaration(class) {
        declarations.insert(property, value);
        if class != "normal-nums" {
            declarations.insert(
                "font-variant-numeric".to_string(),
                tailwind_font_variant_numeric_pipeline(),
            );
        }
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("font-stretch-")
        .and_then(tailwind_font_stretch_value)
    {
        declarations.insert("font-stretch".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("font-features-")
        .and_then(tailwind_arbitrary_or_custom_var)
    {
        declarations.insert("font-feature-settings".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("tab-").and_then(tailwind_tab_size_value) {
        declarations.insert("tab-size".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = tailwind_text_shadow_value(class) {
        declarations.insert("text-shadow".to_string(), value);
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_font_variant_numeric_declaration(
    class: &str,
) -> Option<(String, String)> {
    let declaration = match class {
        "normal-nums" => ("font-variant-numeric", "normal"),
        "ordinal" => ("--tw-ordinal", "ordinal"),
        "slashed-zero" => ("--tw-slashed-zero", "slashed-zero"),
        "lining-nums" => ("--tw-numeric-figure", "lining-nums"),
        "oldstyle-nums" => ("--tw-numeric-figure", "oldstyle-nums"),
        "proportional-nums" => ("--tw-numeric-spacing", "proportional-nums"),
        "tabular-nums" => ("--tw-numeric-spacing", "tabular-nums"),
        "diagonal-fractions" => ("--tw-numeric-fraction", "diagonal-fractions"),
        "stacked-fractions" => ("--tw-numeric-fraction", "stacked-fractions"),
        _ => return None,
    };
    Some((declaration.0.to_string(), declaration.1.to_string()))
}

pub(in crate::style) fn tailwind_font_variant_numeric_pipeline() -> String {
    "var(--tw-ordinal) var(--tw-slashed-zero) var(--tw-numeric-figure) var(--tw-numeric-spacing) var(--tw-numeric-fraction)".to_string()
}

pub(in crate::style) fn tailwind_font_stretch_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    match value {
        "ultra-condensed" | "extra-condensed" | "condensed" | "semi-condensed" | "normal"
        | "semi-expanded" | "expanded" | "extra-expanded" | "ultra-expanded" => {
            Some(value.to_string())
        }
        _ => value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value))),
    }
}

pub(in crate::style) fn tailwind_tab_size_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| value.parse::<u32>().ok().map(|value| value.to_string()))
}

pub(in crate::style) fn tailwind_text_shadow_value(class: &str) -> Option<String> {
    if class == "text-shadow-none" {
        return Some("none".to_string());
    }
    let value = class.strip_prefix("text-shadow-")?;
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    if is_tailwind_identifier(value) {
        Some(format!("var(--text-shadow-{value})"))
    } else {
        None
    }
}

pub(in crate::style) fn tailwind_line_clamp_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let value = class.strip_prefix("line-clamp-")?;
    let mut declarations = BTreeMap::new();
    if value == "none" {
        declarations.insert("overflow".to_string(), "visible".to_string());
        declarations.insert("display".to_string(), "block".to_string());
        declarations.insert("-webkit-box-orient".to_string(), "horizontal".to_string());
        declarations.insert("-webkit-line-clamp".to_string(), "unset".to_string());
        return Some(declarations);
    }
    let value = tailwind_line_clamp_value(value)?;
    declarations.insert("overflow".to_string(), "hidden".to_string());
    declarations.insert("display".to_string(), "-webkit-box".to_string());
    declarations.insert("-webkit-box-orient".to_string(), "vertical".to_string());
    declarations.insert("-webkit-line-clamp".to_string(), value);
    Some(declarations)
}

pub(in crate::style) fn tailwind_line_clamp_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "number") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    value.parse::<u32>().ok().map(|value| value.to_string())
}

pub(in crate::style) fn tailwind_length(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if let Some(variable) = tailwind_custom_var(value) {
        return Some(StyleLength::Css(variable));
    }
    if value == "full" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "screen" {
        return Some(StyleLength::Percent(100.0));
    }
    if value == "auto" {
        return Some(StyleLength::Auto);
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    if let Some((numerator, denominator)) = value.split_once('/') {
        let numerator = numerator.parse::<f64>().ok()?;
        let denominator = denominator.parse::<f64>().ok()?;
        if denominator != 0.0 {
            return Some(StyleLength::Percent((numerator / denominator) * 100.0));
        }
    }
    let value = value.parse::<f64>().ok()?;
    Some(StyleLength::Points(value * 4.0))
}

pub(in crate::style) fn tailwind_line_height(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "none" => Some("1".to_string()),
        "tight" => Some("1.25".to_string()),
        "snug" => Some("1.375".to_string()),
        "normal" => Some("1.5".to_string()),
        "relaxed" => Some("1.625".to_string()),
        "loose" => Some("2".to_string()),
        _ => tailwind_length(value).map(style_length_css),
    }
}

pub(in crate::style) fn tailwind_text_indent(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("indent-")?;
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some(style_length_css(length))
}

pub(in crate::style) fn tailwind_opacity(value: &str) -> Option<f64> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return arbitrary.parse::<f64>().ok();
    }
    value.parse::<f64>().ok().map(|value| value / 100.0)
}

pub(in crate::style) fn tailwind_color(value: &str) -> Option<StyleColor> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let color = parse_color(&tailwind_arbitrary_value(arbitrary))?;
        return Some(apply_tailwind_color_opacity(color, opacity));
    }
    let color = match value {
        "black" => parse_color("#000"),
        "white" => parse_color("#fff"),
        "transparent" => Some(StyleColor::Keyword("transparent".to_string())),
        "current" => Some(StyleColor::Keyword("currentColor".to_string())),
        "inherit" => Some(StyleColor::Keyword("inherit".to_string())),
        other if is_tailwind_palette_color(other) => Some(StyleColor::Keyword(other.to_string())),
        other => tailwind_semantic_color(other).and_then(parse_color),
    }?;
    Some(apply_tailwind_color_opacity(color, opacity))
}

pub(in crate::style) fn tailwind_accent_color_css(value: &str) -> Option<String> {
    if value == "auto" {
        Some("auto".to_string())
    } else {
        tailwind_color_css(value)
    }
}

pub(in crate::style) fn tailwind_color_css(value: &str) -> Option<String> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if let Some(color) = parse_color(&value) {
            return Some(style_color_css(&apply_tailwind_color_opacity(
                color, opacity,
            )));
        }
        return Some(apply_tailwind_keyword_opacity(value, opacity));
    }
    match value {
        "black" => parse_color("#000")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "white" => parse_color("#fff")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "transparent" => Some("transparent".to_string()),
        "current" => Some(apply_tailwind_keyword_opacity(
            "currentColor".to_string(),
            opacity,
        )),
        "inherit" => Some(apply_tailwind_keyword_opacity(
            "inherit".to_string(),
            opacity,
        )),
        other if is_tailwind_palette_color(other) => {
            Some(apply_tailwind_keyword_opacity(other.to_string(), opacity))
        }
        other => tailwind_semantic_color(other)
            .and_then(parse_color)
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
    }
}

pub(in crate::style) fn tailwind_semantic_color(value: &str) -> Option<&'static str> {
    match value {
        // shadcn-compatible semantic aliases, backed by DESIGN.md's Vercel/Geist palette.
        "background" | "canvas" => Some("#fafafa"),
        "foreground" | "ink" => Some("#171717"),
        "card" | "popover" | "elevated" | "canvas-elevated" => Some("#ffffff"),
        "card-foreground" | "popover-foreground" => Some("#171717"),
        "primary" => Some("#171717"),
        "primary-foreground" => Some("#ffffff"),
        "secondary" | "muted" | "accent" | "hairline-soft" => Some("#f2f2f2"),
        "secondary-foreground" | "accent-foreground" => Some("#171717"),
        "muted-foreground" | "mute" => Some("#8f8f8f"),
        "body" => Some("#4d4d4d"),
        "faint" => Some("#a1a1a1"),
        "destructive" | "error" => Some("#ee0000"),
        "destructive-foreground" | "error-foreground" => Some("#ffffff"),
        "border" | "input" | "hairline" => Some("#ebebeb"),
        "ring" | "link" | "success" => Some("#0070f3"),
        "link-deep" => Some("#0761d1"),
        "link-soft" => Some("#d3e5ff"),
        "warning" => Some("#f5a623"),
        "violet" => Some("#7928ca"),
        "cyan" => Some("#50e3c2"),
        "pink" => Some("#ff0080"),
        "magenta" => Some("#eb367f"),
        "gradient-develop-start" | "chart-1" => Some("#007cf0"),
        "gradient-develop-end" | "chart-2" => Some("#00dfd8"),
        "gradient-preview-start" | "chart-3" => Some("#7928ca"),
        "gradient-preview-end" | "chart-4" => Some("#ff0080"),
        "gradient-ship-start" => Some("#ff4d4d"),
        "gradient-ship-end" | "chart-5" => Some("#f9cb28"),
        "sidebar" => Some("#fafafa"),
        "sidebar-foreground" => Some("#171717"),
        "sidebar-primary" => Some("#171717"),
        "sidebar-primary-foreground" => Some("#ffffff"),
        "sidebar-accent" => Some("#f2f2f2"),
        "sidebar-accent-foreground" => Some("#171717"),
        "sidebar-border" => Some("#ebebeb"),
        "sidebar-ring" => Some("#0070f3"),
        _ => None,
    }
}

pub(in crate::style) fn split_tailwind_color_opacity(value: &str) -> (&str, Option<&str>) {
    let mut bracket_depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '/' if bracket_depth == 0 => return (&value[..index], Some(&value[index + 1..])),
            _ => {}
        }
    }
    (value, None)
}

pub(in crate::style) fn apply_tailwind_color_opacity(
    color: StyleColor,
    opacity: Option<&str>,
) -> StyleColor {
    let Some(alpha) = opacity.and_then(tailwind_opacity_alpha) else {
        return color;
    };
    match color {
        StyleColor::Rgba {
            red, green, blue, ..
        } => StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        },
        StyleColor::Function(value) => {
            StyleColor::Function(apply_tailwind_function_opacity(value, opacity))
        }
        StyleColor::Keyword(value) => {
            StyleColor::Keyword(apply_tailwind_keyword_opacity(value, opacity))
        }
    }
}

pub(in crate::style) fn apply_tailwind_function_opacity(
    value: String,
    opacity: Option<&str>,
) -> String {
    let Some(percent) = opacity.and_then(tailwind_opacity_percent) else {
        return value;
    };
    format!("color-mix(in srgb, {value} {percent}, transparent)")
}

pub(in crate::style) fn apply_tailwind_keyword_opacity(
    value: String,
    opacity: Option<&str>,
) -> String {
    let Some(opacity) = opacity else {
        return value;
    };
    let Some(percent) = tailwind_opacity_percent(opacity) else {
        return value;
    };
    if value == "transparent" {
        value
    } else {
        format!("{value} / {percent}")
    }
}

pub(in crate::style) fn tailwind_opacity_alpha(value: &str) -> Option<u8> {
    let opacity = tailwind_opacity(value)?;
    Some((opacity.clamp(0.0, 1.0) * 255.0).round() as u8)
}

pub(in crate::style) fn tailwind_opacity_percent(value: &str) -> Option<String> {
    let opacity = tailwind_opacity(value)?;
    Some(format!("{}%", trim_float(opacity.clamp(0.0, 1.0) * 100.0)))
}

pub(in crate::style) fn style_color_css(color: &StyleColor) -> String {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } if *alpha < 255 => {
            let alpha = trim_float((*alpha as f64 / 255.0 * 100.0).round() / 100.0);
            format!("rgba({red}, {green}, {blue}, {alpha})")
        }
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => format!("rgb({red}, {green}, {blue})"),
        StyleColor::Function(value) => value.clone(),
        StyleColor::Keyword(value) => value.clone(),
    }
}

pub(in crate::style) fn tailwind_z_index(class: &str) -> Option<String> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let value = class.strip_prefix("z-")?;
    if value == "auto" {
        return Some("auto".to_string());
    }
    let value = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(tailwind_arbitrary_value)
        .unwrap_or_else(|| value.to_string());
    if negative {
        Some(format!("-{value}"))
    } else {
        Some(value)
    }
}

pub(in crate::style) fn is_tailwind_palette_color(value: &str) -> bool {
    let Some((name, shade)) = value.rsplit_once('-') else {
        return false;
    };
    matches!(
        name,
        "slate"
            | "gray"
            | "zinc"
            | "neutral"
            | "stone"
            | "red"
            | "orange"
            | "amber"
            | "yellow"
            | "lime"
            | "green"
            | "emerald"
            | "teal"
            | "cyan"
            | "sky"
            | "blue"
            | "indigo"
            | "violet"
            | "purple"
            | "fuchsia"
            | "pink"
            | "rose"
    ) && matches!(
        shade,
        "50" | "100" | "200" | "300" | "400" | "500" | "600" | "700" | "800" | "900" | "950"
    )
}

pub(in crate::style) fn tailwind_inset_utility(
    class: &str,
) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("inset-x-") {
        (EdgeSelection::X, value)
    } else if let Some(value) = class.strip_prefix("inset-y-") {
        (EdgeSelection::Y, value)
    } else if let Some(value) = class.strip_prefix("inset-") {
        (EdgeSelection::All, value)
    } else if let Some(value) = class.strip_prefix("top-") {
        (EdgeSelection::Top, value)
    } else if let Some(value) = class.strip_prefix("right-") {
        (EdgeSelection::Right, value)
    } else if let Some(value) = class.strip_prefix("bottom-") {
        (EdgeSelection::Bottom, value)
    } else if let Some(value) = class.strip_prefix("left-") {
        (EdgeSelection::Left, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

pub(in crate::style) fn tailwind_logical_inset_utility(
    class: &str,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let (edges, value) = if let Some(value) = class.strip_prefix("start-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("end-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if let Some(value) = class.strip_prefix("inset-be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if let Some(value) = class.strip_prefix("inset-is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if let Some(value) = class.strip_prefix("inset-ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

pub(in crate::style) fn tailwind_border_width_utility(
    class: &str,
) -> Option<(EdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border")?;
    if suffix.is_empty() {
        return Some((EdgeSelection::All, StyleLength::Points(1.0)));
    }
    let suffix = suffix.strip_prefix('-')?;
    let (edges, value) = if suffix == "x" {
        (EdgeSelection::X, "1")
    } else if let Some(value) = suffix.strip_prefix("x-") {
        (EdgeSelection::X, value)
    } else if suffix == "y" {
        (EdgeSelection::Y, "1")
    } else if let Some(value) = suffix.strip_prefix("y-") {
        (EdgeSelection::Y, value)
    } else if suffix == "t" {
        (EdgeSelection::Top, "1")
    } else if let Some(value) = suffix.strip_prefix("t-") {
        (EdgeSelection::Top, value)
    } else if suffix == "r" {
        (EdgeSelection::Right, "1")
    } else if let Some(value) = suffix.strip_prefix("r-") {
        (EdgeSelection::Right, value)
    } else if suffix == "b" {
        (EdgeSelection::Bottom, "1")
    } else if let Some(value) = suffix.strip_prefix("b-") {
        (EdgeSelection::Bottom, value)
    } else if suffix == "l" {
        (EdgeSelection::Left, "1")
    } else if let Some(value) = suffix.strip_prefix("l-") {
        (EdgeSelection::Left, value)
    } else {
        (EdgeSelection::All, suffix)
    };
    Some((edges, tailwind_border_width(value)?))
}

pub(in crate::style) fn tailwind_logical_border_width_utility(
    class: &str,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let suffix = class.strip_prefix("border-")?;
    let (edges, value) = if suffix == "s" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("s-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "e" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("e-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else if suffix == "bs" {
        (LogicalEdgeSelection::BlockStart, "1")
    } else if let Some(value) = suffix.strip_prefix("bs-") {
        (LogicalEdgeSelection::BlockStart, value)
    } else if suffix == "be" {
        (LogicalEdgeSelection::BlockEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("be-") {
        (LogicalEdgeSelection::BlockEnd, value)
    } else if suffix == "is" {
        (LogicalEdgeSelection::InlineStart, "1")
    } else if let Some(value) = suffix.strip_prefix("is-") {
        (LogicalEdgeSelection::InlineStart, value)
    } else if suffix == "ie" {
        (LogicalEdgeSelection::InlineEnd, "1")
    } else if let Some(value) = suffix.strip_prefix("ie-") {
        (LogicalEdgeSelection::InlineEnd, value)
    } else {
        return None;
    };
    Some((edges, tailwind_border_width(value)?))
}

pub(in crate::style) fn tailwind_border_color_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let suffix = class.strip_prefix("border-")?;
    let mut declarations = BTreeMap::new();
    if let Some((edges, value)) = tailwind_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_border_color_declarations(&mut declarations, edges, color);
        return Some(declarations);
    }
    if let Some((edges, value)) = tailwind_logical_border_color_edge_value(suffix) {
        let color = tailwind_border_color_value(value)?;
        insert_logical_border_color_declaration(&mut declarations, edges, color);
        return Some(declarations);
    }
    let color = tailwind_border_color_value(suffix)?;
    declarations.insert("border-color".to_string(), color);
    Some(declarations)
}

pub(in crate::style) fn tailwind_border_color_edge_value(
    value: &str,
) -> Option<(EdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("x-") {
        Some((EdgeSelection::X, value))
    } else if let Some(value) = value.strip_prefix("y-") {
        Some((EdgeSelection::Y, value))
    } else if let Some(value) = value.strip_prefix("t-") {
        Some((EdgeSelection::Top, value))
    } else if let Some(value) = value.strip_prefix("r-") {
        Some((EdgeSelection::Right, value))
    } else if let Some(value) = value.strip_prefix("b-") {
        Some((EdgeSelection::Bottom, value))
    } else if let Some(value) = value.strip_prefix("l-") {
        Some((EdgeSelection::Left, value))
    } else {
        None
    }
}

pub(in crate::style) fn tailwind_logical_border_color_edge_value(
    value: &str,
) -> Option<(LogicalEdgeSelection, &str)> {
    if let Some(value) = value.strip_prefix("s-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("e-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else if let Some(value) = value.strip_prefix("bs-") {
        Some((LogicalEdgeSelection::BlockStart, value))
    } else if let Some(value) = value.strip_prefix("be-") {
        Some((LogicalEdgeSelection::BlockEnd, value))
    } else if let Some(value) = value.strip_prefix("is-") {
        Some((LogicalEdgeSelection::InlineStart, value))
    } else if let Some(value) = value.strip_prefix("ie-") {
        Some((LogicalEdgeSelection::InlineEnd, value))
    } else {
        None
    }
}

pub(in crate::style) fn tailwind_border_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    if let Some(value) = tailwind_custom_var(value) {
        return Some(value);
    }
    tailwind_color_css(value)
}

pub(in crate::style) fn tailwind_border_width(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
    }
    if value == "px" {
        return Some(StyleLength::Points(1.0));
    }
    value.parse::<f64>().ok().map(StyleLength::Points)
}

pub(in crate::style) fn negate_style_length(value: StyleLength) -> Option<StyleLength> {
    match value {
        StyleLength::Points(value) => Some(StyleLength::Points(-value)),
        StyleLength::Percent(value) => Some(StyleLength::Percent(-value)),
        StyleLength::Css(value) => Some(StyleLength::Css(format!("calc({value} * -1)"))),
        StyleLength::Auto => None,
    }
}

pub(in crate::style) fn tailwind_edge_utility(
    class: &str,
    prefix: &str,
) -> Option<(EdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b'-', ..] => (EdgeSelection::All, &suffix[1..]),
        [b'x', b'-', ..] => (EdgeSelection::X, &suffix[2..]),
        [b'y', b'-', ..] => (EdgeSelection::Y, &suffix[2..]),
        [b't', b'-', ..] => (EdgeSelection::Top, &suffix[2..]),
        [b'r', b'-', ..] => (EdgeSelection::Right, &suffix[2..]),
        [b'b', b'-', ..] => (EdgeSelection::Bottom, &suffix[2..]),
        [b'l', b'-', ..] => (EdgeSelection::Left, &suffix[2..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}

pub(in crate::style) fn tailwind_logical_edge_utility(
    class: &str,
    prefix: &str,
    allow_negative: bool,
) -> Option<(LogicalEdgeSelection, StyleLength)> {
    let negative = class.starts_with('-');
    if negative && !allow_negative {
        return None;
    }
    let class = class.strip_prefix('-').unwrap_or(class);
    let suffix = class.strip_prefix(prefix)?;
    let (edges, value) = match suffix.as_bytes() {
        [b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[2..]),
        [b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[2..]),
        [b'b', b's', b'-', ..] => (LogicalEdgeSelection::BlockStart, &suffix[3..]),
        [b'b', b'e', b'-', ..] => (LogicalEdgeSelection::BlockEnd, &suffix[3..]),
        [b'i', b's', b'-', ..] => (LogicalEdgeSelection::InlineStart, &suffix[3..]),
        [b'i', b'e', b'-', ..] => (LogicalEdgeSelection::InlineEnd, &suffix[3..]),
        _ => return None,
    };
    let mut length = tailwind_length(value)?;
    if negative {
        length = negate_style_length(length)?;
    }
    Some((edges, length))
}
