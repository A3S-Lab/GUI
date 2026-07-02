use super::*;

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
