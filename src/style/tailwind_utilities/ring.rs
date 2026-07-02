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
