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
            "padding" => self.padding.set_all(parse_length(value)),
            "paddingTop" | "padding-top" => self.padding.top = parse_length(value),
            "paddingRight" | "padding-right" => self.padding.right = parse_length(value),
            "paddingBottom" | "padding-bottom" => self.padding.bottom = parse_length(value),
            "paddingLeft" | "padding-left" => self.padding.left = parse_length(value),
            "margin" => self.margin.set_all(parse_length(value)),
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
    value.parse::<f64>().ok().map(StyleLength::Points)
}

fn parse_color(value: &str) -> Option<StyleColor> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    if value.is_empty() {
        None
    } else {
        Some(StyleColor::Keyword(value.to_string()))
    }
}

fn parse_hex_color(hex: &str) -> Option<StyleColor> {
    match hex.len() {
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
            .style("backgroundColor", "#663399")
            .style("boxShadow", "0 1px 3px black");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Horizontal));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(8.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(12.0)));
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
}
