use super::*;

pub(in crate::style) fn tailwind_filter_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if class == "filter-none" {
        declarations.insert("filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if class == "filter" {
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("filter".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("filter-").and_then(tailwind_custom_var) {
        declarations.insert("filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "") {
        declarations.insert(property, value);
        declarations.insert("filter".to_string(), tailwind_filter_pipeline());
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_backdrop_filter_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if matches!(class, "backdrop-filter-none" | "backdrop-none") {
        declarations.insert("backdrop-filter".to_string(), "none".to_string());
        return Some(declarations);
    }
    if matches!(class, "backdrop-filter" | "backdrop") {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .or_else(|| class.strip_prefix("backdrop-["))
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-")
        .or_else(|| class.strip_prefix("backdrop-"))
        .and_then(tailwind_custom_var)
    {
        declarations.insert("backdrop-filter".to_string(), value);
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_filter_component_declaration(class, "backdrop-") {
        declarations.insert(property, value);
        declarations.insert(
            "backdrop-filter".to_string(),
            tailwind_backdrop_filter_pipeline(),
        );
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_blend_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("mix-blend-") {
        return tailwind_blend_mode_value(value, true)
            .map(|value| ("mix-blend-mode".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("bg-blend-") {
        return tailwind_blend_mode_value(value, false)
            .map(|value| ("background-blend-mode".to_string(), value));
    }
    None
}

pub(in crate::style) fn tailwind_blend_mode_value(
    value: &str,
    include_plus_modes: bool,
) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    let known = matches!(
        value,
        "normal"
            | "multiply"
            | "screen"
            | "overlay"
            | "darken"
            | "lighten"
            | "color-dodge"
            | "color-burn"
            | "hard-light"
            | "soft-light"
            | "difference"
            | "exclusion"
            | "hue"
            | "saturation"
            | "color"
            | "luminosity"
    ) || (include_plus_modes && matches!(value, "plus-darker" | "plus-lighter"));
    known.then(|| value.to_string())
}

pub(in crate::style) fn tailwind_fragmentation_declaration(
    class: &str,
) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("break-before-") {
        return tailwind_break_value(value).map(|value| ("break-before".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("break-after-") {
        return tailwind_break_value(value).map(|value| ("break-after".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("break-inside-") {
        return tailwind_break_inside_value(value).map(|value| ("break-inside".to_string(), value));
    }
    None
}

pub(in crate::style) fn tailwind_break_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    matches!(
        value,
        "auto"
            | "avoid"
            | "all"
            | "avoid-page"
            | "avoid-column"
            | "page"
            | "left"
            | "right"
            | "recto"
            | "verso"
            | "column"
    )
    .then(|| value.to_string())
}

pub(in crate::style) fn tailwind_break_inside_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    matches!(value, "auto" | "avoid" | "avoid-page" | "avoid-column").then(|| value.to_string())
}

pub(in crate::style) fn tailwind_mask_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("mask-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = tailwind_mask_arbitrary_property(value) {
            return Some((property, value));
        }
        return Some(("mask-image".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class.strip_prefix("mask-").and_then(tailwind_custom_var) {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-image-") {
        return Some(("mask-image".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-size-") {
        return Some(("mask-size".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-position-") {
        return Some(("mask-position".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-repeat-") {
        return Some(("mask-repeat".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-origin-") {
        return Some(("mask-origin".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-clip-") {
        return Some(("mask-clip".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-composite-") {
        return Some(("mask-composite".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-mode-") {
        return Some(("mask-mode".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-type-") {
        return Some(("mask-type".to_string(), value));
    }
    if let Some(value) = tailwind_mask_prefixed_value(class, "mask-border-") {
        return Some(("mask-border".to_string(), value));
    }

    let declaration = match class {
        "mask-none" => Some(("mask-image", "none")),
        "mask-alpha" => Some(("mask-mode", "alpha")),
        "mask-luminance" => Some(("mask-mode", "luminance")),
        "mask-match" => Some(("mask-mode", "match-source")),
        "mask-type-alpha" => Some(("mask-type", "alpha")),
        "mask-type-luminance" => Some(("mask-type", "luminance")),
        "mask-auto" => Some(("mask-size", "auto")),
        "mask-cover" => Some(("mask-size", "cover")),
        "mask-contain" => Some(("mask-size", "contain")),
        "mask-repeat" => Some(("mask-repeat", "repeat")),
        "mask-no-repeat" => Some(("mask-repeat", "no-repeat")),
        "mask-repeat-x" => Some(("mask-repeat", "repeat-x")),
        "mask-repeat-y" => Some(("mask-repeat", "repeat-y")),
        "mask-repeat-space" => Some(("mask-repeat", "space")),
        "mask-repeat-round" => Some(("mask-repeat", "round")),
        "mask-center" => Some(("mask-position", "center")),
        "mask-top" => Some(("mask-position", "top")),
        "mask-right" => Some(("mask-position", "right")),
        "mask-bottom" => Some(("mask-position", "bottom")),
        "mask-left" => Some(("mask-position", "left")),
        "mask-top-left" | "mask-left-top" => Some(("mask-position", "top left")),
        "mask-top-right" | "mask-right-top" => Some(("mask-position", "top right")),
        "mask-bottom-right" | "mask-right-bottom" => Some(("mask-position", "bottom right")),
        "mask-bottom-left" | "mask-left-bottom" => Some(("mask-position", "bottom left")),
        "mask-origin-border" => Some(("mask-origin", "border-box")),
        "mask-origin-padding" => Some(("mask-origin", "padding-box")),
        "mask-origin-content" => Some(("mask-origin", "content-box")),
        "mask-origin-fill" => Some(("mask-origin", "fill-box")),
        "mask-origin-stroke" => Some(("mask-origin", "stroke-box")),
        "mask-origin-view" => Some(("mask-origin", "view-box")),
        "mask-clip-border" => Some(("mask-clip", "border-box")),
        "mask-clip-padding" => Some(("mask-clip", "padding-box")),
        "mask-clip-content" => Some(("mask-clip", "content-box")),
        "mask-clip-fill" => Some(("mask-clip", "fill-box")),
        "mask-clip-stroke" => Some(("mask-clip", "stroke-box")),
        "mask-clip-view" => Some(("mask-clip", "view-box")),
        "mask-no-clip" => Some(("mask-clip", "no-clip")),
        "mask-add" => Some(("mask-composite", "add")),
        "mask-subtract" => Some(("mask-composite", "subtract")),
        "mask-intersect" => Some(("mask-composite", "intersect")),
        "mask-exclude" => Some(("mask-composite", "exclude")),
        _ => None,
    }?;
    Some((declaration.0.to_string(), declaration.1.to_string()))
}

pub(in crate::style) fn tailwind_mask_arbitrary_property(value: &str) -> Option<(String, String)> {
    let (name, value) = value.split_once(':')?;
    let property = match name {
        "image" => "mask-image",
        "mode" => "mask-mode",
        "repeat" => "mask-repeat",
        "position" => "mask-position",
        "size" => "mask-size",
        "origin" => "mask-origin",
        "clip" => "mask-clip",
        "composite" => "mask-composite",
        "type" => "mask-type",
        "border" => "mask-border",
        "border-source" => "mask-border-source",
        "border-mode" => "mask-border-mode",
        "border-slice" => "mask-border-slice",
        "border-width" => "mask-border-width",
        "border-outset" => "mask-border-outset",
        "border-repeat" => "mask-border-repeat",
        _ => return None,
    };
    Some((property.to_string(), tailwind_arbitrary_value(value)))
}

pub(in crate::style) fn tailwind_mask_prefixed_value(class: &str, prefix: &str) -> Option<String> {
    class
        .strip_prefix(prefix)
        .and_then(tailwind_arbitrary_or_custom_var)
}

pub(in crate::style) fn tailwind_filter_component_declaration(
    class: &str,
    prefix: &str,
) -> Option<(String, String)> {
    let class = class.strip_prefix(prefix)?;
    let variable_prefix = if prefix.is_empty() {
        "--tw"
    } else {
        "--tw-backdrop"
    };
    if let Some(value) = class.strip_prefix("blur") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_blur_value(value)?;
        return Some((format!("{variable_prefix}-blur"), value));
    }
    if let Some(value) = class.strip_prefix("brightness-") {
        let value = tailwind_percent_filter_value(value, "brightness")?;
        return Some((format!("{variable_prefix}-brightness"), value));
    }
    if let Some(value) = class.strip_prefix("contrast-") {
        let value = tailwind_percent_filter_value(value, "contrast")?;
        return Some((format!("{variable_prefix}-contrast"), value));
    }
    if prefix.is_empty() {
        if let Some(value) = class.strip_prefix("drop-shadow") {
            let value = tailwind_optional_suffix(value)?;
            let value = tailwind_drop_shadow_value(value)?;
            return Some(("--tw-drop-shadow".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("grayscale") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "grayscale")?;
        return Some((format!("{variable_prefix}-grayscale"), value));
    }
    if let Some(value) = class.strip_prefix("hue-rotate-") {
        let (negative, value) = strip_negative_prefix(value);
        let value = tailwind_signed_angle_value(value, negative)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("-hue-rotate-") {
        let value = tailwind_signed_angle_value(value, true)?;
        return Some((
            format!("{variable_prefix}-hue-rotate"),
            format!("hue-rotate({value})"),
        ));
    }
    if let Some(value) = class.strip_prefix("invert") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "invert")?;
        return Some((format!("{variable_prefix}-invert"), value));
    }
    if prefix == "backdrop-" {
        if let Some(value) = class.strip_prefix("opacity-") {
            let value = tailwind_percent_filter_value(value, "opacity")?;
            return Some(("--tw-backdrop-opacity".to_string(), value));
        }
    }
    if let Some(value) = class.strip_prefix("saturate-") {
        let value = tailwind_percent_filter_value(value, "saturate")?;
        return Some((format!("{variable_prefix}-saturate"), value));
    }
    if let Some(value) = class.strip_prefix("sepia") {
        let value = tailwind_optional_suffix(value)?;
        let value = tailwind_binary_filter_value(value, "sepia")?;
        return Some((format!("{variable_prefix}-sepia"), value));
    }
    None
}

pub(in crate::style) fn tailwind_optional_suffix(value: &str) -> Option<&str> {
    if value.is_empty() {
        Some("DEFAULT")
    } else {
        value.strip_prefix('-')
    }
}

pub(in crate::style) fn tailwind_blur_value(value: &str) -> Option<String> {
    match value {
        "DEFAULT" => Some("blur(8px)".to_string()),
        "none" => Some(String::new()),
        "xs" => Some("blur(4px)".to_string()),
        "sm" => Some("blur(8px)".to_string()),
        "md" => Some("blur(12px)".to_string()),
        "lg" => Some("blur(16px)".to_string()),
        "xl" => Some("blur(24px)".to_string()),
        "2xl" => Some("blur(40px)".to_string()),
        "3xl" => Some("blur(64px)".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value).map(|value| format!("blur({value})")),
    }
}

pub(in crate::style) fn tailwind_percent_filter_value(
    value: &str,
    function: &str,
) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(format!("{function}({value})"))
}

pub(in crate::style) fn tailwind_binary_filter_value(
    value: &str,
    function: &str,
) -> Option<String> {
    let value = match value {
        "DEFAULT" => "100%".to_string(),
        "0" => "0%".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("{function}({value})"))
}

pub(in crate::style) fn tailwind_drop_shadow_value(value: &str) -> Option<String> {
    let shadow = match value {
        "DEFAULT" => "0 1px 2px rgb(0 0 0 / 0.1), 0 1px 1px rgb(0 0 0 / 0.06)".to_string(),
        "xs" => "0 1px 1px rgb(0 0 0 / 0.05)".to_string(),
        "sm" => "0 1px 2px rgb(0 0 0 / 0.15)".to_string(),
        "md" => "0 3px 3px rgb(0 0 0 / 0.12)".to_string(),
        "lg" => "0 4px 4px rgb(0 0 0 / 0.15)".to_string(),
        "xl" => "0 9px 7px rgb(0 0 0 / 0.1)".to_string(),
        "2xl" => "0 25px 25px rgb(0 0 0 / 0.15)".to_string(),
        "none" => "0 0 #0000".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)?,
    };
    Some(format!("drop-shadow({shadow})"))
}

pub(in crate::style) fn tailwind_filter_pipeline() -> String {
    "var(--tw-blur) var(--tw-brightness) var(--tw-contrast) var(--tw-drop-shadow) var(--tw-grayscale) var(--tw-hue-rotate) var(--tw-invert) var(--tw-saturate) var(--tw-sepia)"
        .to_string()
}

pub(in crate::style) fn tailwind_backdrop_filter_pipeline() -> String {
    "var(--tw-backdrop-blur) var(--tw-backdrop-brightness) var(--tw-backdrop-contrast) var(--tw-backdrop-grayscale) var(--tw-backdrop-hue-rotate) var(--tw-backdrop-invert) var(--tw-backdrop-opacity) var(--tw-backdrop-saturate) var(--tw-backdrop-sepia)"
        .to_string()
}
