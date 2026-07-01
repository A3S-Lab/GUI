use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyle {
    pub display: Option<DisplayMode>,
    pub flex_direction: Option<Orientation>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub width: Option<StyleLength>,
    pub height: Option<StyleLength>,
    pub min_width: Option<StyleLength>,
    pub min_height: Option<StyleLength>,
    pub max_width: Option<StyleLength>,
    pub max_height: Option<StyleLength>,
    pub gap: Option<StyleLength>,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub color: Option<StyleColor>,
    pub background_color: Option<StyleColor>,
    pub border_radius: Option<StyleLength>,
    pub opacity: Option<f64>,
    pub unsupported: BTreeMap<String, String>,
}

impl PortableStyle {
    pub fn from_web(web: &WebProps) -> Self {
        let mut style = PortableStyle::default();
        if let Some(class_name) = &web.class_name {
            for class in class_name.split_whitespace() {
                style.apply_tailwind_utility(class);
            }
        }
        for (property, value) in &web.style {
            style.apply(property, value);
        }
        style
    }

    fn apply(&mut self, property: &str, value: &str) {
        match property {
            "display" => self.display = parse_display(value),
            "flexDirection" | "flex-direction" => self.flex_direction = parse_flex_direction(value),
            "alignItems" | "align-items" => self.align_items = parse_align_items(value),
            "justifyContent" | "justify-content" => {
                self.justify_content = parse_justify_content(value);
            }
            "width" => self.width = parse_length(value),
            "height" => self.height = parse_length(value),
            "minWidth" | "min-width" => self.min_width = parse_length(value),
            "minHeight" | "min-height" => self.min_height = parse_length(value),
            "maxWidth" | "max-width" => self.max_width = parse_length(value),
            "maxHeight" | "max-height" => self.max_height = parse_length(value),
            "gap" => self.gap = parse_length(value),
            "padding" => self.padding = parse_edge_insets(value),
            "paddingTop" | "padding-top" => self.padding.top = parse_length(value),
            "paddingRight" | "padding-right" => self.padding.right = parse_length(value),
            "paddingBottom" | "padding-bottom" => self.padding.bottom = parse_length(value),
            "paddingLeft" | "padding-left" => self.padding.left = parse_length(value),
            "margin" => self.margin = parse_edge_insets(value),
            "marginTop" | "margin-top" => self.margin.top = parse_length(value),
            "marginRight" | "margin-right" => self.margin.right = parse_length(value),
            "marginBottom" | "margin-bottom" => self.margin.bottom = parse_length(value),
            "marginLeft" | "margin-left" => self.margin.left = parse_length(value),
            "color" => self.color = parse_color(value),
            "backgroundColor" | "background-color" => self.background_color = parse_color(value),
            "borderRadius" | "border-radius" => self.border_radius = parse_length(value),
            "opacity" => self.opacity = value.trim().parse::<f64>().ok(),
            other => {
                self.unsupported
                    .insert(other.to_string(), value.to_string());
            }
        }
    }

    fn apply_tailwind_utility(&mut self, class: &str) {
        let Some(class) = base_tailwind_utility(class.trim()) else {
            return;
        };
        let class = class.strip_prefix('!').unwrap_or(class);
        if class.is_empty() {
            return;
        }
        if let Some(arbitrary) = class
            .strip_prefix('[')
            .and_then(|value| value.strip_suffix(']'))
        {
            if let Some((property, value)) = arbitrary.split_once(':') {
                self.apply(property.trim(), &tailwind_arbitrary_value(value.trim()));
            }
            return;
        }
        match class {
            "flex" | "inline-flex" => self.display = Some(DisplayMode::Flex),
            "block" | "inline-block" => self.display = Some(DisplayMode::Block),
            "hidden" => self.display = Some(DisplayMode::None),
            "flex-row" | "flex-row-reverse" => self.flex_direction = Some(Orientation::Horizontal),
            "flex-col" | "flex-col-reverse" => self.flex_direction = Some(Orientation::Vertical),
            "items-start" => self.align_items = Some(AlignItems::Start),
            "items-center" => self.align_items = Some(AlignItems::Center),
            "items-end" => self.align_items = Some(AlignItems::End),
            "items-stretch" => self.align_items = Some(AlignItems::Stretch),
            "justify-start" => self.justify_content = Some(JustifyContent::Start),
            "justify-center" => self.justify_content = Some(JustifyContent::Center),
            "justify-end" => self.justify_content = Some(JustifyContent::End),
            "justify-between" => self.justify_content = Some(JustifyContent::SpaceBetween),
            "rounded" => self.border_radius = Some(StyleLength::Points(4.0)),
            "rounded-none" => self.border_radius = Some(StyleLength::Points(0.0)),
            "rounded-sm" => self.border_radius = Some(StyleLength::Points(2.0)),
            "rounded-md" => self.border_radius = Some(StyleLength::Points(6.0)),
            "rounded-lg" => self.border_radius = Some(StyleLength::Points(8.0)),
            "rounded-xl" => self.border_radius = Some(StyleLength::Points(12.0)),
            "rounded-2xl" => self.border_radius = Some(StyleLength::Points(16.0)),
            "rounded-3xl" => self.border_radius = Some(StyleLength::Points(24.0)),
            "rounded-full" => self.border_radius = Some(StyleLength::Points(9999.0)),
            _ => self.apply_tailwind_prefixed_utility(class),
        }
    }

    fn apply_tailwind_prefixed_utility(&mut self, class: &str) {
        if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
            self.width = Some(value);
        } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
            self.height = Some(value);
        } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
            self.min_width = Some(value);
        } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
            self.min_height = Some(value);
        } else if let Some(value) = class.strip_prefix("max-w-").and_then(tailwind_length) {
            self.max_width = Some(value);
        } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
            self.max_height = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
            self.gap = Some(value);
        } else if let Some(opacity) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
            self.opacity = Some(opacity);
        } else if let Some(color) = class.strip_prefix("bg-").and_then(tailwind_color) {
            self.background_color = Some(color);
        } else if let Some(color) = class.strip_prefix("text-").and_then(tailwind_color) {
            self.color = Some(color);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
            self.padding.apply_edges(edges, value);
        } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
            self.margin.apply_edges(edges, value);
        } else if let Some(value) = class.strip_prefix("rounded-").and_then(tailwind_length) {
            self.border_radius = Some(value);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayMode {
    Flex,
    Block,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleLength {
    Points(f64),
    Percent(f64),
    Auto,
}

impl StyleLength {
    pub fn points(self) -> Option<f64> {
        match self {
            StyleLength::Points(value) => Some(value),
            StyleLength::Percent(_) | StyleLength::Auto => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleColor {
    Rgba {
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    },
    Keyword(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeInsets {
    pub top: Option<StyleLength>,
    pub right: Option<StyleLength>,
    pub bottom: Option<StyleLength>,
    pub left: Option<StyleLength>,
}

impl EdgeInsets {
    fn set_all(&mut self, value: Option<StyleLength>) {
        self.top = value;
        self.right = value;
        self.bottom = value;
        self.left = value;
    }

    fn apply_edges(&mut self, edges: EdgeSelection, value: StyleLength) {
        match edges {
            EdgeSelection::All => self.set_all(Some(value)),
            EdgeSelection::X => {
                self.left = Some(value);
                self.right = Some(value);
            }
            EdgeSelection::Y => {
                self.top = Some(value);
                self.bottom = Some(value);
            }
            EdgeSelection::Top => self.top = Some(value),
            EdgeSelection::Right => self.right = Some(value),
            EdgeSelection::Bottom => self.bottom = Some(value),
            EdgeSelection::Left => self.left = Some(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum EdgeSelection {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

fn parse_display(value: &str) -> Option<DisplayMode> {
    match value.trim() {
        "flex" | "inline-flex" => Some(DisplayMode::Flex),
        "block" | "inline-block" => Some(DisplayMode::Block),
        "none" => Some(DisplayMode::None),
        _ => None,
    }
}

fn parse_flex_direction(value: &str) -> Option<Orientation> {
    match value.trim() {
        "row" | "row-reverse" => Some(Orientation::Horizontal),
        "column" | "column-reverse" => Some(Orientation::Vertical),
        _ => None,
    }
}

fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value.trim() {
        "flex-start" | "start" => Some(AlignItems::Start),
        "center" => Some(AlignItems::Center),
        "flex-end" | "end" => Some(AlignItems::End),
        "stretch" => Some(AlignItems::Stretch),
        _ => None,
    }
}

fn parse_justify_content(value: &str) -> Option<JustifyContent> {
    match value.trim() {
        "flex-start" | "start" => Some(JustifyContent::Start),
        "center" => Some(JustifyContent::Center),
        "flex-end" | "end" => Some(JustifyContent::End),
        "space-between" => Some(JustifyContent::SpaceBetween),
        _ => None,
    }
}

fn parse_edge_insets(value: &str) -> EdgeInsets {
    let values = value
        .split_whitespace()
        .filter_map(parse_length)
        .collect::<Vec<_>>();
    let mut edges = EdgeInsets::default();
    match values.as_slice() {
        [] => {}
        [all] => edges.set_all(Some(*all)),
        [vertical, horizontal] => {
            edges.top = Some(*vertical);
            edges.bottom = Some(*vertical);
            edges.left = Some(*horizontal);
            edges.right = Some(*horizontal);
        }
        [top, horizontal, bottom] => {
            edges.top = Some(*top);
            edges.left = Some(*horizontal);
            edges.right = Some(*horizontal);
            edges.bottom = Some(*bottom);
        }
        [top, right, bottom, left, ..] => {
            edges.top = Some(*top);
            edges.right = Some(*right);
            edges.bottom = Some(*bottom);
            edges.left = Some(*left);
        }
    }
    edges
}

fn parse_length(value: &str) -> Option<StyleLength> {
    let value = value.trim();
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
    value.parse::<f64>().ok().map(StyleLength::Points)
}

fn parse_color(value: &str) -> Option<StyleColor> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    if let Some(color) = parse_rgb_function(value) {
        return Some(color);
    }
    if value.is_empty() {
        None
    } else {
        Some(StyleColor::Keyword(value.to_string()))
    }
}

fn parse_hex_color(hex: &str) -> Option<StyleColor> {
    match hex.len() {
        3 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: 255,
        }),
        4 => Some(StyleColor::Rgba {
            red: expand_hex_digit(&hex[0..1])?,
            green: expand_hex_digit(&hex[1..2])?,
            blue: expand_hex_digit(&hex[2..3])?,
            alpha: expand_hex_digit(&hex[3..4])?,
        }),
        6 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: 255,
        }),
        8 => Some(StyleColor::Rgba {
            red: u8::from_str_radix(&hex[0..2], 16).ok()?,
            green: u8::from_str_radix(&hex[2..4], 16).ok()?,
            blue: u8::from_str_radix(&hex[4..6], 16).ok()?,
            alpha: u8::from_str_radix(&hex[6..8], 16).ok()?,
        }),
        _ => None,
    }
}

fn expand_hex_digit(hex: &str) -> Option<u8> {
    let value = u8::from_str_radix(hex, 16).ok()?;
    Some((value << 4) | value)
}

fn parse_rgb_function(value: &str) -> Option<StyleColor> {
    let content = value
        .strip_prefix("rgb(")
        .or_else(|| value.strip_prefix("rgba("))?
        .strip_suffix(')')?;
    let content = content.replace(',', " ");
    let mut parts = content.split_whitespace();
    let red = parse_rgb_channel(parts.next()?)?;
    let green = parse_rgb_channel(parts.next()?)?;
    let blue = parse_rgb_channel(parts.next()?)?;
    let alpha = parts.next().and_then(parse_alpha_channel).unwrap_or(255);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_rgb_channel(value: &str) -> Option<u8> {
    value.trim().parse::<u8>().ok()
}

fn parse_alpha_channel(value: &str) -> Option<u8> {
    let value = value.trim().trim_start_matches('/');
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some((percent.clamp(0.0, 100.0) * 2.55).round() as u8);
    }
    let alpha = value.parse::<f64>().ok()?;
    Some((alpha.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn base_tailwind_utility(class: &str) -> Option<&str> {
    let mut bracket_depth = 0usize;
    for ch in class.chars() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => return None,
            _ => {}
        }
    }
    Some(class)
}

fn tailwind_arbitrary_value(value: &str) -> String {
    value.replace('_', " ")
}

fn tailwind_length(value: &str) -> Option<StyleLength> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_length(&tailwind_arbitrary_value(arbitrary));
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

fn tailwind_opacity(value: &str) -> Option<f64> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return arbitrary.parse::<f64>().ok();
    }
    value.parse::<f64>().ok().map(|value| value / 100.0)
}

fn tailwind_color(value: &str) -> Option<StyleColor> {
    let value = value.split_once('/').map_or(value, |(value, _)| value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return parse_color(&tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "black" => parse_color("#000"),
        "white" => parse_color("#fff"),
        "transparent" => Some(StyleColor::Keyword("transparent".to_string())),
        "current" => Some(StyleColor::Keyword("currentColor".to_string())),
        other if is_tailwind_palette_color(other) => Some(StyleColor::Keyword(other.to_string())),
        _ => None,
    }
}

fn is_tailwind_palette_color(value: &str) -> bool {
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

fn tailwind_edge_utility(class: &str, prefix: &str) -> Option<(EdgeSelection, StyleLength)> {
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
        if let StyleLength::Points(points) = length {
            length = StyleLength::Points(-points);
        }
    }
    Some((edges, length))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_web_style_into_portable_tokens() {
        let web = WebProps::new()
            .style("display", "flex")
            .style("flexDirection", "row")
            .style("minWidth", "280")
            .style("gap", "8px")
            .style("paddingTop", "12")
            .style("margin", "1px 2px 3px 4px")
            .style("backgroundColor", "#663399")
            .style("boxShadow", "0 1px 3px black");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Horizontal));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(8.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(12.0)));
        assert_eq!(style.margin.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.margin.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.margin.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.margin.left, Some(StyleLength::Points(4.0)));
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(
            style.unsupported.get("boxShadow").map(String::as_str),
            Some("0 1px 3px black")
        );
    }

    #[test]
    fn parses_tailwind_utilities_before_inline_style_overrides() {
        let web = WebProps::new()
            .class_name(
                "flex flex-col items-center justify-between min-w-[280px] gap-4 p-2 \
                 mx-auto bg-[#663399] text-white rounded-lg opacity-50",
            )
            .style("gap", "10px");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Vertical));
        assert_eq!(style.align_items, Some(AlignItems::Center));
        assert_eq!(style.justify_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(10.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(8.0)));
        assert_eq!(style.padding.right, Some(StyleLength::Points(8.0)));
        assert_eq!(style.margin.left, Some(StyleLength::Auto));
        assert_eq!(style.margin.right, Some(StyleLength::Auto));
        assert_eq!(style.border_radius, Some(StyleLength::Points(8.0)));
        assert_eq!(style.opacity, Some(0.5));
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 255,
            })
        );
        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 255,
            })
        );
    }
}
