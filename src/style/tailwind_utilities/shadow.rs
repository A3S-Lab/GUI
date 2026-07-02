use super::*;

pub(in crate::style) fn tailwind_shadow_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    if let Some(shadow) = tailwind_shadow_value(class) {
        insert_tailwind_shadow_declarations(&mut declarations, "--tw-shadow", shadow);
        return Some(declarations);
    }
    if let Some(shadow) = tailwind_inset_shadow_value(class) {
        insert_tailwind_shadow_declarations(&mut declarations, "--tw-inset-shadow", shadow);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("shadow-") {
        let color = tailwind_shadow_color_value(value)?;
        insert_tailwind_shadow_color_declarations(&mut declarations, "--tw-shadow-color", color);
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("inset-shadow-") {
        let color = tailwind_shadow_color_value(value)?;
        insert_tailwind_shadow_color_declarations(
            &mut declarations,
            "--tw-inset-shadow-color",
            color,
        );
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn insert_tailwind_shadow_declarations(
    declarations: &mut BTreeMap<String, String>,
    property: &str,
    shadow: String,
) {
    declarations.insert(property.to_string(), shadow);
    declarations.insert(
        "box-shadow".to_string(),
        tailwind_box_shadow_pipeline().to_string(),
    );
}

pub(in crate::style) fn insert_tailwind_shadow_color_declarations(
    declarations: &mut BTreeMap<String, String>,
    property: &str,
    color: String,
) {
    declarations.insert(property.to_string(), color);
    declarations.insert(
        "box-shadow".to_string(),
        tailwind_box_shadow_pipeline().to_string(),
    );
}

pub(in crate::style) fn tailwind_shadow_value(class: &str) -> Option<String> {
    if let Some(value) = class
        .strip_prefix("shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(value));
    }
    if let Some(value) = class.strip_prefix("shadow-") {
        if tailwind_typed_custom_var(value, "color").is_none() {
            if let Some(value) = tailwind_custom_var(value) {
                return Some(value);
            }
        }
    }
    let value = match class {
        "shadow-2xs" => "0 1px rgb(0 0 0 / 0.05)",
        "shadow-xs" => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
        "shadow-sm" | "shadow" => "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)",
        "shadow-md" => "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)",
        "shadow-lg" => "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)",
        "shadow-xl" => "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)",
        "shadow-2xl" => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
        "shadow-none" => "0 0 #0000",
        _ => return None,
    };
    Some(value.to_string())
}

pub(in crate::style) fn tailwind_inset_shadow_value(class: &str) -> Option<String> {
    if let Some(value) = class
        .strip_prefix("inset-shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(value));
    }
    if let Some(value) = class.strip_prefix("inset-shadow-") {
        if tailwind_typed_custom_var(value, "color").is_none() {
            if let Some(value) = tailwind_custom_var(value) {
                return Some(value);
            }
        }
    }
    let value = match class {
        "inset-shadow-2xs" => "inset 0 1px rgb(0 0 0 / 0.05)",
        "inset-shadow-xs" => "inset 0 1px 1px rgb(0 0 0 / 0.05)",
        "inset-shadow-sm" | "inset-shadow" => "inset 0 2px 4px rgb(0 0 0 / 0.05)",
        "inset-shadow-none" => "inset 0 0 #0000",
        _ => return None,
    };
    Some(value.to_string())
}

pub(in crate::style) fn tailwind_shadow_color_value(value: &str) -> Option<String> {
    if let Some(value) = tailwind_typed_custom_var(value, "color") {
        return Some(value);
    }
    tailwind_border_color_value(value)
}

pub(in crate::style) fn tailwind_box_shadow_pipeline() -> &'static str {
    "var(--tw-inset-shadow), var(--tw-inset-ring-shadow), var(--tw-ring-shadow), var(--tw-shadow)"
}

pub(in crate::style) fn compose_tailwind_shadow(shadow: &str, color: Option<&str>) -> String {
    let shadow = shadow.trim();
    let Some(color) = color.map(str::trim).filter(|color| !color.is_empty()) else {
        return shadow.to_string();
    };
    let colored = shadow
        .replace("rgb(0 0 0 / 0.05)", color)
        .replace("rgb(0 0 0 / 0.1)", color)
        .replace("rgb(0 0 0 / 0.12)", color)
        .replace("rgb(0 0 0 / 0.25)", color);
    if colored != shadow || shadow.contains(color) {
        colored
    } else {
        format!("{shadow} {color}")
    }
}
