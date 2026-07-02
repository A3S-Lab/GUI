use super::*;

pub(in crate::style) fn tailwind_transform_declarations(
    class: &str,
) -> Option<BTreeMap<String, String>> {
    let mut declarations = BTreeMap::new();
    let declaration = match class {
        "transform-none" => Some(("transform", "none".to_string())),
        "transform-gpu" => Some(("transform", tailwind_transform_pipeline(true))),
        "transform-cpu" | "transform" => Some(("transform", tailwind_transform_pipeline(false))),
        "transform-flat" => Some(("transform-style", "flat".to_string())),
        "transform-3d" => Some(("transform-style", "preserve-3d".to_string())),
        "backface-visible" => Some(("backface-visibility", "visible".to_string())),
        "backface-hidden" => Some(("backface-visibility", "hidden".to_string())),
        "perspective-none" => Some(("perspective", "none".to_string())),
        "perspective-dramatic" => Some(("perspective", "100px".to_string())),
        "perspective-near" => Some(("perspective", "300px".to_string())),
        "perspective-normal" => Some(("perspective", "500px".to_string())),
        "perspective-midrange" => Some(("perspective", "800px".to_string())),
        "perspective-distant" => Some(("perspective", "1200px".to_string())),
        "origin-center" => Some(("transform-origin", "center".to_string())),
        "origin-top" => Some(("transform-origin", "top".to_string())),
        "origin-top-right" => Some(("transform-origin", "top right".to_string())),
        "origin-right" => Some(("transform-origin", "right".to_string())),
        "origin-bottom-right" => Some(("transform-origin", "bottom right".to_string())),
        "origin-bottom" => Some(("transform-origin", "bottom".to_string())),
        "origin-bottom-left" => Some(("transform-origin", "bottom left".to_string())),
        "origin-left" => Some(("transform-origin", "left".to_string())),
        "origin-top-left" => Some(("transform-origin", "top left".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return Some(declarations);
    }

    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("transform".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("transform-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("transform".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("origin-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert(
            "transform-origin".to_string(),
            tailwind_arbitrary_value(value),
        );
        return Some(declarations);
    }
    if let Some(value) = class.strip_prefix("origin-").and_then(tailwind_custom_var) {
        declarations.insert("transform-origin".to_string(), value);
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        declarations.insert("perspective".to_string(), tailwind_arbitrary_value(value));
        return Some(declarations);
    }
    if let Some(value) = class
        .strip_prefix("perspective-")
        .and_then(tailwind_custom_var)
    {
        declarations.insert("perspective".to_string(), value);
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_translate_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "translate",
            "--tw-translate",
            axis,
            value,
            "0",
        );
        return Some(declarations);
    }
    if let Some((axis, value)) = tailwind_scale_declaration(class) {
        insert_tailwind_axis_declarations(
            &mut declarations,
            "scale",
            "--tw-scale",
            axis,
            value,
            "100%",
        );
        return Some(declarations);
    }
    if let Some(value) = tailwind_rotate_declaration(class) {
        declarations.insert("--tw-rotate".to_string(), value);
        declarations.insert("rotate".to_string(), "var(--tw-rotate)".to_string());
        return Some(declarations);
    }
    if let Some((property, value)) = tailwind_transform_function_declaration(class) {
        declarations.insert(property, value);
        declarations.insert("transform".to_string(), tailwind_transform_pipeline(false));
        return Some(declarations);
    }
    None
}

pub(in crate::style) fn insert_tailwind_axis_declarations(
    declarations: &mut BTreeMap<String, String>,
    property: &str,
    variable_prefix: &str,
    axis: TransformAxis,
    value: String,
    default_value: &str,
) {
    match axis {
        TransformAxis::All => {
            declarations.insert(format!("{variable_prefix}-x"), value.clone());
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::X => {
            declarations.insert(format!("{variable_prefix}-x"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x) var({variable_prefix}-y, {default_value})"),
            );
        }
        TransformAxis::Y => {
            declarations.insert(format!("{variable_prefix}-y"), value);
            declarations.insert(
                property.to_string(),
                format!("var({variable_prefix}-x, {default_value}) var({variable_prefix}-y)"),
            );
        }
        TransformAxis::Z => {
            declarations.insert(format!("{variable_prefix}-z"), value);
            declarations.insert(
                property.to_string(),
                format!(
                    "var({variable_prefix}-x, {default_value}) var({variable_prefix}-y, {default_value}) var({variable_prefix}-z)"
                ),
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(in crate::style) enum TransformAxis {
    All,
    X,
    Y,
    Z,
}

pub(in crate::style) fn tailwind_translate_declaration(
    class: &str,
) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("translate-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("translate-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("translate-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("translate-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_length_value(value, negative)?))
}

pub(in crate::style) fn tailwind_scale_declaration(class: &str) -> Option<(TransformAxis, String)> {
    let (negative, class) = strip_negative_prefix(class);
    let (axis, value) = if let Some(value) = class.strip_prefix("scale-x-") {
        (TransformAxis::X, value)
    } else if let Some(value) = class.strip_prefix("scale-y-") {
        (TransformAxis::Y, value)
    } else if let Some(value) = class.strip_prefix("scale-z-") {
        (TransformAxis::Z, value)
    } else if let Some(value) = class.strip_prefix("scale-") {
        (TransformAxis::All, value)
    } else {
        return None;
    };
    Some((axis, tailwind_signed_scale_value(value, negative)?))
}

pub(in crate::style) fn tailwind_rotate_declaration(class: &str) -> Option<String> {
    let (negative, class) = strip_negative_prefix(class);
    let value = class.strip_prefix("rotate-")?;
    tailwind_signed_angle_value(value, negative)
}

pub(in crate::style) fn tailwind_transform_function_declaration(
    class: &str,
) -> Option<(String, String)> {
    let (negative, class) = strip_negative_prefix(class);
    if let Some(value) = class.strip_prefix("rotate-x-") {
        return Some((
            "--tw-rotate-x".to_string(),
            format!("rotateX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-y-") {
        return Some((
            "--tw-rotate-y".to_string(),
            format!("rotateY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("rotate-z-") {
        return Some((
            "--tw-rotate-z".to_string(),
            format!("rotateZ({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-x-") {
        return Some((
            "--tw-skew-x".to_string(),
            format!("skewX({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    if let Some(value) = class.strip_prefix("skew-y-") {
        return Some((
            "--tw-skew-y".to_string(),
            format!("skewY({})", tailwind_signed_angle_value(value, negative)?),
        ));
    }
    None
}

pub(in crate::style) fn strip_negative_prefix(value: &str) -> (bool, &str) {
    if let Some(value) = value.strip_prefix('-') {
        (true, value)
    } else {
        (false, value)
    }
}

pub(in crate::style) fn tailwind_signed_length_value(
    value: &str,
    negative: bool,
) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        if value == "full" {
            Some("100%".to_string())
        } else {
            tailwind_length(value).map(style_length_css)
        }
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

pub(in crate::style) fn tailwind_signed_scale_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}%", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

pub(in crate::style) fn tailwind_signed_angle_value(value: &str, negative: bool) -> Option<String> {
    let value = tailwind_arbitrary_or_custom_var(value).or_else(|| {
        value
            .parse::<f64>()
            .ok()
            .map(|value| format!("{}deg", trim_float(value)))
    })?;
    Some(if negative {
        negate_css_value(&value)
    } else {
        value
    })
}

pub(in crate::style) fn negate_css_value(value: &str) -> String {
    if value.starts_with("var(") || value.starts_with("calc(") {
        format!("calc({value} * -1)")
    } else if let Some(number) = value.strip_prefix('-') {
        number.to_string()
    } else {
        format!("-{value}")
    }
}

pub(in crate::style) fn tailwind_transform_pipeline(gpu: bool) -> String {
    let pipeline = "var(--tw-rotate-x) var(--tw-rotate-y) var(--tw-rotate-z) var(--tw-skew-x) var(--tw-skew-y)";
    if gpu {
        format!("translateZ(0) {pipeline}")
    } else {
        pipeline.to_string()
    }
}
