use super::*;

pub(in crate::style) fn tailwind_motion_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    match class {
        "transition" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-all" => {
            declarations.insert("transition-property".to_string(), "all".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-colors" => {
            declarations.insert(
                "transition-property".to_string(),
                "color, background-color, border-color, outline-color, text-decoration-color, fill, stroke".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-opacity" => {
            declarations.insert("transition-property".to_string(), "opacity".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-shadow" => {
            declarations.insert("transition-property".to_string(), "box-shadow".to_string());
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-transform" => {
            declarations.insert(
                "transition-property".to_string(),
                "transform, translate, scale, rotate".to_string(),
            );
            insert_tailwind_default_transition(&mut declarations);
            return Some(declarations);
        }
        "transition-none" => {
            declarations.insert("transition-property".to_string(), "none".to_string());
            return Some(declarations);
        }
        "transition-discrete" => {
            declarations.insert(
                "transition-behavior".to_string(),
                "allow-discrete".to_string(),
            );
            return Some(declarations);
        }
        "transition-normal" => {
            declarations.insert("transition-behavior".to_string(), "normal".to_string());
            return Some(declarations);
        }
        "animate-spin" => {
            declarations.insert(
                "animation".to_string(),
                "spin 1s linear infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-ping" => {
            declarations.insert(
                "animation".to_string(),
                "ping 1s cubic-bezier(0, 0, 0.2, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-pulse" => {
            declarations.insert(
                "animation".to_string(),
                "pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite".to_string(),
            );
            return Some(declarations);
        }
        "animate-bounce" => {
            declarations.insert("animation".to_string(), "bounce 1s infinite".to_string());
            return Some(declarations);
        }
        "animate-none" => {
            declarations.insert("animation".to_string(), "none".to_string());
            return Some(declarations);
        }
        _ => {}
    }

    if let Some(value) = class
        .strip_prefix("transition-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transition-property".to_string(),
            tailwind_arbitrary_value(value),
        );
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transition-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transition-property".to_string(), value);
        insert_tailwind_default_transition(&mut declarations);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("duration-")
        .and_then(tailwind_time_value)
    {
        declarations.insert("transition-duration".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("delay-").and_then(tailwind_time_value) {
        declarations.insert("transition-delay".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("ease-").and_then(tailwind_easing_value) {
        declarations.insert("transition-timing-function".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("animate-")
        .and_then(tailwind_animation_value)
    {
        declarations.insert("animation".to_string(), value);
        return Some(declarations);
    }

    None
}

pub(in crate::style) fn insert_tailwind_default_transition(
    declarations: &mut BTreeMap<String, String>,
) {
    declarations.insert(
        "transition-timing-function".to_string(),
        "cubic-bezier(0.4, 0, 0.2, 1)".to_string(),
    );
    declarations.insert("transition-duration".to_string(), "150ms".to_string());
}

pub(in crate::style) fn tailwind_time_value(value: &str) -> Option<String> {
    if value == "initial" {
        return Some("initial".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<f64>().ok().map(|value| {
        if value == 0.0 {
            "0ms".to_string()
        } else {
            format!("{}ms", trim_float(value))
        }
    })
}

pub(in crate::style) fn tailwind_easing_value(value: &str) -> Option<String> {
    match value {
        "linear" => Some("linear".to_string()),
        "in" => Some("cubic-bezier(0.4, 0, 1, 1)".to_string()),
        "out" => Some("cubic-bezier(0, 0, 0.2, 1)".to_string()),
        "in-out" => Some("cubic-bezier(0.4, 0, 0.2, 1)".to_string()),
        "initial" => Some("initial".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

pub(in crate::style) fn tailwind_animation_value(value: &str) -> Option<String> {
    match value {
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value),
    }
}

pub(in crate::style) fn tailwind_interaction_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "scheme-normal" => Some(("color-scheme", "normal".to_string())),
        "scheme-dark" => Some(("color-scheme", "dark".to_string())),
        "scheme-light" => Some(("color-scheme", "light".to_string())),
        "scheme-light-dark" => Some(("color-scheme", "light dark".to_string())),
        "scheme-only-dark" => Some(("color-scheme", "only dark".to_string())),
        "scheme-only-light" => Some(("color-scheme", "only light".to_string())),
        "forced-color-adjust-auto" => Some(("forced-color-adjust", "auto".to_string())),
        "forced-color-adjust-none" => Some(("forced-color-adjust", "none".to_string())),
        "field-sizing-fixed" => Some(("field-sizing", "fixed".to_string())),
        "field-sizing-content" => Some(("field-sizing", "content".to_string())),
        "appearance-none" => Some(("appearance", "none".to_string())),
        "appearance-auto" => Some(("appearance", "auto".to_string())),
        "will-change-auto" => Some(("will-change", "auto".to_string())),
        "will-change-scroll" => Some(("will-change", "scroll-position".to_string())),
        "will-change-contents" => Some(("will-change", "contents".to_string())),
        "will-change-transform" => Some(("will-change", "transform".to_string())),
        "scrollbar-gutter-auto" => Some(("scrollbar-gutter", "auto".to_string())),
        "scrollbar-gutter-stable" => Some(("scrollbar-gutter", "stable".to_string())),
        "scrollbar-gutter-both" => Some(("scrollbar-gutter", "stable both-edges".to_string())),
        "scrollbar-auto" => Some(("scrollbar-width", "auto".to_string())),
        "scrollbar-thin" => Some(("scrollbar-width", "thin".to_string())),
        "scrollbar-none" => Some(("scrollbar-width", "none".to_string())),
        "scroll-auto" => Some(("scroll-behavior", "auto".to_string())),
        "scroll-smooth" => Some(("scroll-behavior", "smooth".to_string())),
        "snap-none" => Some(("scroll-snap-type", "none".to_string())),
        "snap-x" => Some((
            "scroll-snap-type",
            "x var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-y" => Some((
            "scroll-snap-type",
            "y var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-both" => Some((
            "scroll-snap-type",
            "both var(--tw-scroll-snap-strictness)".to_string(),
        )),
        "snap-mandatory" => Some(("--tw-scroll-snap-strictness", "mandatory".to_string())),
        "snap-proximity" => Some(("--tw-scroll-snap-strictness", "proximity".to_string())),
        "snap-start" => Some(("scroll-snap-align", "start".to_string())),
        "snap-end" => Some(("scroll-snap-align", "end".to_string())),
        "snap-center" => Some(("scroll-snap-align", "center".to_string())),
        "snap-align-none" => Some(("scroll-snap-align", "none".to_string())),
        "snap-normal" => Some(("scroll-snap-stop", "normal".to_string())),
        "snap-always" => Some(("scroll-snap-stop", "always".to_string())),
        "overscroll-auto" => Some(("overscroll-behavior", "auto".to_string())),
        "overscroll-contain" => Some(("overscroll-behavior", "contain".to_string())),
        "overscroll-none" => Some(("overscroll-behavior", "none".to_string())),
        "overscroll-x-auto" => Some(("overscroll-behavior-x", "auto".to_string())),
        "overscroll-x-contain" => Some(("overscroll-behavior-x", "contain".to_string())),
        "overscroll-x-none" => Some(("overscroll-behavior-x", "none".to_string())),
        "overscroll-y-auto" => Some(("overscroll-behavior-y", "auto".to_string())),
        "overscroll-y-contain" => Some(("overscroll-behavior-y", "contain".to_string())),
        "overscroll-y-none" => Some(("overscroll-behavior-y", "none".to_string())),
        "touch-auto" => Some(("touch-action", "auto".to_string())),
        "touch-none" => Some(("touch-action", "none".to_string())),
        "touch-pan-x" => Some(("touch-action", "pan-x".to_string())),
        "touch-pan-left" => Some(("touch-action", "pan-left".to_string())),
        "touch-pan-right" => Some(("touch-action", "pan-right".to_string())),
        "touch-pan-y" => Some(("touch-action", "pan-y".to_string())),
        "touch-pan-up" => Some(("touch-action", "pan-up".to_string())),
        "touch-pan-down" => Some(("touch-action", "pan-down".to_string())),
        "touch-pinch-zoom" => Some(("touch-action", "pinch-zoom".to_string())),
        "touch-manipulation" => Some(("touch-action", "manipulation".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("appearance-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("appearance".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("will-change".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("will-change-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("will-change".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("touch-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("touch-action".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scheme-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("color-scheme".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("scheme-").and_then(tailwind_custom_var) {
        declarations.insert("color-scheme".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("forced-color-adjust-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "forced-color-adjust".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("forced-color-adjust-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("forced-color-adjust".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("field-sizing-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("field-sizing".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("field-sizing-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("field-sizing".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-thumb-")
        .and_then(tailwind_scrollbar_color_value)
    {
        declarations.insert("--tw-scrollbar-thumb".to_string(), value);
        declarations.insert(
            "scrollbar-color".to_string(),
            tailwind_scrollbar_color_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-track-")
        .and_then(tailwind_scrollbar_color_value)
    {
        declarations.insert("--tw-scrollbar-track".to_string(), value);
        declarations.insert(
            "scrollbar-color".to_string(),
            tailwind_scrollbar_color_pipeline().to_string(),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-gutter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "scrollbar-gutter".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-gutter-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("scrollbar-gutter".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "scrollbar-width".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("scrollbar-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("scrollbar-width".to_string(), value);
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn tailwind_scrollbar_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_color_css(value).or_else(|| tailwind_custom_var(value))
}

pub(in crate::style) fn tailwind_scrollbar_color_pipeline() -> &'static str {
    "var(--tw-scrollbar-thumb) var(--tw-scrollbar-track)"
}
