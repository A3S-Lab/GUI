use super::{StyleLength, StyleTime};

pub(super) fn parse_time(value: &str) -> Option<StyleTime> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if value == "0" {
        return Some(StyleTime::Milliseconds(0.0));
    }
    if let Some(milliseconds) = value.strip_suffix("ms") {
        return milliseconds
            .trim()
            .parse::<f64>()
            .ok()
            .map(StyleTime::Milliseconds);
    }
    if let Some(seconds) = value.strip_suffix('s') {
        return seconds
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleTime::Milliseconds(value * 1000.0));
    }
    if is_css_time_expression(value) {
        return Some(StyleTime::Css(value.to_string()));
    }
    None
}

fn is_css_time_expression(value: &str) -> bool {
    if matches!(
        value,
        "inherit" | "initial" | "unset" | "revert" | "revert-layer"
    ) {
        return true;
    }
    if let Some((name, _)) = value.split_once('(') {
        let name = name.trim();
        return (matches!(name, "calc" | "min" | "max" | "clamp" | "var")
            || is_css_math_function_name(name))
            && value.ends_with(')');
    }
    false
}

pub(super) fn parse_length(value: &str) -> Option<StyleLength> {
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    if value == "auto" {
        return Some(StyleLength::Auto);
    }
    if let Some(percent) = value.strip_suffix('%') {
        return percent.trim().parse::<f64>().ok().map(StyleLength::Percent);
    }
    if let Some(points) = value.strip_suffix("px") {
        return points.trim().parse::<f64>().ok().map(StyleLength::Points);
    }
    if let Some(rem) = value.strip_suffix("rem") {
        return rem
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleLength::Points(value * 16.0));
    }
    if let Some(em) = value.strip_suffix("em") {
        return em
            .trim()
            .parse::<f64>()
            .ok()
            .map(|value| StyleLength::Points(value * 16.0));
    }
    if let Some(points) = value.strip_suffix("pt") {
        return points.trim().parse::<f64>().ok().map(StyleLength::Points);
    }
    if let Ok(points) = value.parse::<f64>() {
        return Some(StyleLength::Points(points));
    }
    if is_css_length_expression(value) {
        return Some(StyleLength::Css(value.to_string()));
    }
    None
}

fn is_css_length_expression(value: &str) -> bool {
    if matches!(
        value,
        "inherit"
            | "initial"
            | "unset"
            | "revert"
            | "revert-layer"
            | "normal"
            | "none"
            | "from-font"
            | "min-content"
            | "max-content"
            | "fit-content"
            | "stretch"
            | "contain"
    ) {
        return true;
    }
    if let Some((name, _)) = value.split_once('(') {
        let name = name.trim();
        if matches!(
            name,
            "anchor"
                | "anchor-size"
                | "calc"
                | "calc-size"
                | "min"
                | "max"
                | "clamp"
                | "var"
                | "env"
                | "fit-content",
        ) || is_css_math_function_name(name)
        {
            return value.ends_with(')');
        }
    }
    let Some((number, unit)) = split_number_and_unit(value) else {
        return false;
    };
    number.parse::<f64>().is_ok() && is_css_length_unit(unit)
}

fn is_css_math_function_name(name: &str) -> bool {
    matches!(
        name,
        "round"
            | "mod"
            | "rem"
            | "sin"
            | "cos"
            | "tan"
            | "asin"
            | "acos"
            | "atan"
            | "atan2"
            | "pow"
            | "sqrt"
            | "hypot"
            | "log"
            | "exp"
            | "abs"
            | "sign"
    )
}

fn split_number_and_unit(value: &str) -> Option<(&str, &str)> {
    let mut split = value.len();
    for (index, ch) in value.char_indices().rev() {
        if ch.is_ascii_alphabetic() || ch == '%' {
            split = index;
        } else {
            break;
        }
    }
    if split == value.len() || split == 0 {
        return None;
    }
    Some((&value[..split], &value[split..]))
}

fn is_css_length_unit(unit: &str) -> bool {
    matches!(
        unit,
        "cap"
            | "ch"
            | "em"
            | "ex"
            | "ic"
            | "lh"
            | "rlh"
            | "rem"
            | "vw"
            | "svw"
            | "lvw"
            | "dvw"
            | "vh"
            | "svh"
            | "lvh"
            | "dvh"
            | "vi"
            | "svi"
            | "lvi"
            | "dvi"
            | "vb"
            | "svb"
            | "lvb"
            | "dvb"
            | "vmin"
            | "svmin"
            | "lvmin"
            | "dvmin"
            | "vmax"
            | "svmax"
            | "lvmax"
            | "dvmax"
            | "cm"
            | "mm"
            | "q"
            | "Q"
            | "in"
            | "pc"
            | "pt"
            | "px"
    )
}
