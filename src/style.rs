use std::collections::BTreeMap;

use crate::geometry::Orientation;
use crate::web::WebProps;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PortableStyle {
    pub declarations: BTreeMap<String, String>,
    pub custom_properties: BTreeMap<String, String>,
    pub variant_declarations: BTreeMap<String, BTreeMap<String, String>>,
    pub display: Option<DisplayMode>,
    pub position: Option<PositionMode>,
    pub flex_direction: Option<Orientation>,
    pub flex_wrap: Option<FlexWrap>,
    pub align_items: Option<AlignItems>,
    pub justify_content: Option<JustifyContent>,
    pub width: Option<StyleLength>,
    pub height: Option<StyleLength>,
    pub min_width: Option<StyleLength>,
    pub min_height: Option<StyleLength>,
    pub max_width: Option<StyleLength>,
    pub max_height: Option<StyleLength>,
    pub gap: Option<StyleLength>,
    pub row_gap: Option<StyleLength>,
    pub column_gap: Option<StyleLength>,
    pub inset: EdgeInsets,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub border_width: EdgeInsets,
    pub border_color: Option<StyleColor>,
    pub border_style: Option<BorderStyle>,
    pub color: Option<StyleColor>,
    pub background_color: Option<StyleColor>,
    pub border_radius: Option<StyleLength>,
    pub font_size: Option<StyleLength>,
    pub font_weight: Option<FontWeight>,
    pub line_height: Option<StyleLength>,
    pub text_align: Option<TextAlign>,
    pub overflow_x: Option<OverflowMode>,
    pub overflow_y: Option<OverflowMode>,
    pub visibility: Option<VisibilityMode>,
    pub z_index: Option<i32>,
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
        let property = normalize_css_property_name(property);
        let value = normalize_css_value(value);
        let value_ref = value.as_str();
        self.record_declaration(&property, value_ref);
        if property.starts_with("--") {
            return;
        }
        match property.as_str() {
            "display" => self.display = parse_display(value_ref),
            "position" => self.position = parse_position(value_ref),
            "flex-direction" => self.flex_direction = parse_flex_direction(value_ref),
            "flex-wrap" => self.flex_wrap = parse_flex_wrap(value_ref),
            "align-items" => self.align_items = parse_align_items(value_ref),
            "justify-content" => self.justify_content = parse_justify_content(value_ref),
            "width" => self.width = parse_length(value_ref),
            "height" => self.height = parse_length(value_ref),
            "min-width" => self.min_width = parse_length(value_ref),
            "min-height" => self.min_height = parse_length(value_ref),
            "max-width" => self.max_width = parse_length(value_ref),
            "max-height" => self.max_height = parse_length(value_ref),
            "gap" => self.gap = parse_length(value_ref),
            "row-gap" => self.row_gap = parse_length(value_ref),
            "column-gap" => self.column_gap = parse_length(value_ref),
            "inset" => self.inset = parse_edge_insets(value_ref),
            "top" => self.inset.top = parse_length(value_ref),
            "right" => self.inset.right = parse_length(value_ref),
            "bottom" => self.inset.bottom = parse_length(value_ref),
            "left" => self.inset.left = parse_length(value_ref),
            "padding" => self.padding = parse_edge_insets(value_ref),
            "padding-block" => self
                .padding
                .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref)),
            "padding-inline" => {
                self.padding
                    .apply_edges_opt(EdgeSelection::X, parse_length(value_ref));
            }
            "padding-top" => self.padding.top = parse_length(value_ref),
            "padding-right" => self.padding.right = parse_length(value_ref),
            "padding-bottom" => self.padding.bottom = parse_length(value_ref),
            "padding-left" => self.padding.left = parse_length(value_ref),
            "margin" => self.margin = parse_edge_insets(value_ref),
            "margin-block" => self
                .margin
                .apply_edges_opt(EdgeSelection::Y, parse_length(value_ref)),
            "margin-inline" => self
                .margin
                .apply_edges_opt(EdgeSelection::X, parse_length(value_ref)),
            "margin-top" => self.margin.top = parse_length(value_ref),
            "margin-right" => self.margin.right = parse_length(value_ref),
            "margin-bottom" => self.margin.bottom = parse_length(value_ref),
            "margin-left" => self.margin.left = parse_length(value_ref),
            "border" => self.apply_border_shorthand(value_ref),
            "border-width" => self.border_width = parse_edge_insets(value_ref),
            "border-top-width" => self.border_width.top = parse_length(value_ref),
            "border-right-width" => self.border_width.right = parse_length(value_ref),
            "border-bottom-width" => self.border_width.bottom = parse_length(value_ref),
            "border-left-width" => self.border_width.left = parse_length(value_ref),
            "border-color" => self.border_color = parse_color(value_ref),
            "border-style" => self.border_style = parse_border_style(value_ref),
            "color" => self.color = parse_color(value_ref),
            "background" | "background-color" => self.background_color = parse_color(value_ref),
            "border-radius" => self.border_radius = parse_length(value_ref),
            "font-size" => self.font_size = parse_length(value_ref),
            "font-weight" => self.font_weight = parse_font_weight(value_ref),
            "line-height" => self.line_height = parse_length(value_ref),
            "text-align" => self.text_align = parse_text_align(value_ref),
            "overflow" => {
                let overflow = parse_overflow(value_ref);
                self.overflow_x = overflow;
                self.overflow_y = overflow;
            }
            "overflow-x" => self.overflow_x = parse_overflow(value_ref),
            "overflow-y" => self.overflow_y = parse_overflow(value_ref),
            "visibility" => self.visibility = parse_visibility(value_ref),
            "z-index" => self.z_index = parse_z_index(value_ref),
            "opacity" => self.opacity = value_ref.trim().parse::<f64>().ok(),
            other => {
                self.unsupported.insert(other.to_string(), value);
            }
        }
    }

    fn record_declaration(&mut self, property: &str, value: &str) {
        if property.starts_with("--") {
            self.custom_properties
                .insert(property.to_string(), value.to_string());
        } else {
            self.declarations
                .insert(property.to_string(), value.to_string());
        }
    }

    fn record_variant_declaration(&mut self, variant: &str, property: String, value: String) {
        self.variant_declarations
            .entry(variant.to_string())
            .or_default()
            .insert(property, value);
    }

    fn apply_border_shorthand(&mut self, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.border_width.set_all(Some(width));
            } else if let Some(style) = parse_border_style(part) {
                self.border_style = Some(style);
            } else if let Some(color) = parse_color(part) {
                self.border_color = Some(color);
            }
        }
    }

    fn apply_tailwind_utility(&mut self, class: &str) {
        let Some((variants, class)) = split_tailwind_class(class.trim()) else {
            return;
        };
        let class = class.strip_prefix('!').unwrap_or(class);
        if class.is_empty() {
            return;
        }
        let declarations = tailwind_utility_declarations(class);
        if !variants.is_empty() {
            let variant_key = variants.join(":");
            for (property, value) in declarations {
                self.record_variant_declaration(&variant_key, property, value);
            }
            return;
        }
        for (property, value) in declarations {
            self.apply(&property, &value);
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
            "inline" => self.display = Some(DisplayMode::Inline),
            "grid" => self.display = Some(DisplayMode::Grid),
            "inline-grid" => self.display = Some(DisplayMode::InlineGrid),
            "hidden" => self.display = Some(DisplayMode::None),
            "static" => self.position = Some(PositionMode::Static),
            "fixed" => self.position = Some(PositionMode::Fixed),
            "absolute" => self.position = Some(PositionMode::Absolute),
            "relative" => self.position = Some(PositionMode::Relative),
            "sticky" => self.position = Some(PositionMode::Sticky),
            "flex-row" | "flex-row-reverse" => self.flex_direction = Some(Orientation::Horizontal),
            "flex-col" | "flex-col-reverse" => self.flex_direction = Some(Orientation::Vertical),
            "flex-wrap" => self.flex_wrap = Some(FlexWrap::Wrap),
            "flex-nowrap" => self.flex_wrap = Some(FlexWrap::NoWrap),
            "flex-wrap-reverse" => self.flex_wrap = Some(FlexWrap::WrapReverse),
            "items-start" => self.align_items = Some(AlignItems::Start),
            "items-center" => self.align_items = Some(AlignItems::Center),
            "items-end" => self.align_items = Some(AlignItems::End),
            "items-stretch" => self.align_items = Some(AlignItems::Stretch),
            "items-baseline" => self.align_items = Some(AlignItems::Baseline),
            "justify-start" => self.justify_content = Some(JustifyContent::Start),
            "justify-center" => self.justify_content = Some(JustifyContent::Center),
            "justify-end" => self.justify_content = Some(JustifyContent::End),
            "justify-between" => self.justify_content = Some(JustifyContent::SpaceBetween),
            "justify-around" => self.justify_content = Some(JustifyContent::SpaceAround),
            "justify-evenly" => self.justify_content = Some(JustifyContent::SpaceEvenly),
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
        } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
            self.column_gap = Some(value);
        } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
            self.row_gap = Some(value);
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
    Inline,
    Flex,
    Block,
    Grid,
    InlineGrid,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PositionMode {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlignItems {
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JustifyContent {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderStyle {
    None,
    Hidden,
    Solid,
    Dashed,
    Dotted,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum FontWeight {
    Number(u16),
    Keyword(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextAlign {
    Start,
    Center,
    End,
    Justify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverflowMode {
    Visible,
    Hidden,
    Scroll,
    Auto,
    Clip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VisibilityMode {
    Visible,
    Hidden,
    Collapse,
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

    fn apply_edges_opt(&mut self, edges: EdgeSelection, value: Option<StyleLength>) {
        if let Some(value) = value {
            self.apply_edges(edges, value);
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

pub fn normalize_css_property_name(property: &str) -> String {
    let property = property.trim();
    if property.starts_with("--") {
        return property.to_string();
    }
    let mut normalized = String::with_capacity(property.len());
    for (index, ch) in property.chars().enumerate() {
        if ch == '_' {
            normalized.push('-');
        } else if ch.is_ascii_uppercase() {
            if index > 0 {
                normalized.push('-');
            }
            normalized.push(ch.to_ascii_lowercase());
        } else {
            normalized.push(ch.to_ascii_lowercase());
        }
    }
    normalized
}

fn normalize_css_value(value: &str) -> String {
    value.trim().to_string()
}

fn parse_display(value: &str) -> Option<DisplayMode> {
    match value.trim() {
        "inline" => Some(DisplayMode::Inline),
        "flex" | "inline-flex" => Some(DisplayMode::Flex),
        "block" | "inline-block" => Some(DisplayMode::Block),
        "grid" => Some(DisplayMode::Grid),
        "inline-grid" => Some(DisplayMode::InlineGrid),
        "none" => Some(DisplayMode::None),
        _ => None,
    }
}

fn parse_position(value: &str) -> Option<PositionMode> {
    match value.trim() {
        "static" => Some(PositionMode::Static),
        "relative" => Some(PositionMode::Relative),
        "absolute" => Some(PositionMode::Absolute),
        "fixed" => Some(PositionMode::Fixed),
        "sticky" => Some(PositionMode::Sticky),
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

fn parse_flex_wrap(value: &str) -> Option<FlexWrap> {
    match value.trim() {
        "nowrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        "wrap-reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value.trim() {
        "flex-start" | "start" => Some(AlignItems::Start),
        "center" => Some(AlignItems::Center),
        "flex-end" | "end" => Some(AlignItems::End),
        "stretch" => Some(AlignItems::Stretch),
        "baseline" => Some(AlignItems::Baseline),
        _ => None,
    }
}

fn parse_justify_content(value: &str) -> Option<JustifyContent> {
    match value.trim() {
        "flex-start" | "start" => Some(JustifyContent::Start),
        "center" => Some(JustifyContent::Center),
        "flex-end" | "end" => Some(JustifyContent::End),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        "space-evenly" => Some(JustifyContent::SpaceEvenly),
        _ => None,
    }
}

fn parse_border_style(value: &str) -> Option<BorderStyle> {
    match value.trim() {
        "none" => Some(BorderStyle::None),
        "hidden" => Some(BorderStyle::Hidden),
        "solid" => Some(BorderStyle::Solid),
        "dashed" => Some(BorderStyle::Dashed),
        "dotted" => Some(BorderStyle::Dotted),
        "double" => Some(BorderStyle::Double),
        "groove" => Some(BorderStyle::Groove),
        "ridge" => Some(BorderStyle::Ridge),
        "inset" => Some(BorderStyle::Inset),
        "outset" => Some(BorderStyle::Outset),
        _ => None,
    }
}

fn parse_font_weight(value: &str) -> Option<FontWeight> {
    let value = value.trim();
    if let Ok(number) = value.parse::<u16>() {
        return Some(FontWeight::Number(number));
    }
    if matches!(
        value,
        "normal" | "bold" | "bolder" | "lighter" | "inherit" | "initial" | "unset"
    ) {
        Some(FontWeight::Keyword(value.to_string()))
    } else {
        None
    }
}

fn parse_text_align(value: &str) -> Option<TextAlign> {
    match value.trim() {
        "left" | "start" => Some(TextAlign::Start),
        "center" => Some(TextAlign::Center),
        "right" | "end" => Some(TextAlign::End),
        "justify" => Some(TextAlign::Justify),
        _ => None,
    }
}

fn parse_overflow(value: &str) -> Option<OverflowMode> {
    match value.trim() {
        "visible" => Some(OverflowMode::Visible),
        "hidden" => Some(OverflowMode::Hidden),
        "scroll" => Some(OverflowMode::Scroll),
        "auto" => Some(OverflowMode::Auto),
        "clip" => Some(OverflowMode::Clip),
        _ => None,
    }
}

fn parse_visibility(value: &str) -> Option<VisibilityMode> {
    match value.trim() {
        "visible" => Some(VisibilityMode::Visible),
        "hidden" => Some(VisibilityMode::Hidden),
        "collapse" => Some(VisibilityMode::Collapse),
        _ => None,
    }
}

fn parse_z_index(value: &str) -> Option<i32> {
    value.trim().parse::<i32>().ok()
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

fn split_tailwind_class(class: &str) -> Option<(Vec<String>, &str)> {
    if class.is_empty() {
        return None;
    }
    let mut bracket_depth = 0usize;
    let mut start = 0usize;
    let mut variants = Vec::new();
    for (index, ch) in class.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            ':' if bracket_depth == 0 => {
                variants.push(class[start..index].to_string());
                start = index + 1;
            }
            _ => {}
        }
    }
    Some((variants, &class[start..]))
}

fn tailwind_utility_declarations(class: &str) -> BTreeMap<String, String> {
    let mut declarations = BTreeMap::new();
    if let Some(arbitrary) = class
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        if let Some((property, value)) = arbitrary.split_once(':') {
            declarations.insert(
                normalize_css_property_name(property),
                tailwind_arbitrary_value(value.trim()),
            );
        }
        return declarations;
    }
    let declaration = match class {
        "inline" => Some(("display", "inline".to_string())),
        "flex" | "inline-flex" => Some(("display", "flex".to_string())),
        "block" | "inline-block" => Some(("display", "block".to_string())),
        "grid" => Some(("display", "grid".to_string())),
        "inline-grid" => Some(("display", "inline-grid".to_string())),
        "hidden" => Some(("display", "none".to_string())),
        "static" => Some(("position", "static".to_string())),
        "fixed" => Some(("position", "fixed".to_string())),
        "absolute" => Some(("position", "absolute".to_string())),
        "relative" => Some(("position", "relative".to_string())),
        "sticky" => Some(("position", "sticky".to_string())),
        "flex-row" => Some(("flex-direction", "row".to_string())),
        "flex-row-reverse" => Some(("flex-direction", "row-reverse".to_string())),
        "flex-col" => Some(("flex-direction", "column".to_string())),
        "flex-col-reverse" => Some(("flex-direction", "column-reverse".to_string())),
        "flex-wrap" => Some(("flex-wrap", "wrap".to_string())),
        "flex-nowrap" => Some(("flex-wrap", "nowrap".to_string())),
        "flex-wrap-reverse" => Some(("flex-wrap", "wrap-reverse".to_string())),
        "items-start" => Some(("align-items", "flex-start".to_string())),
        "items-center" => Some(("align-items", "center".to_string())),
        "items-end" => Some(("align-items", "flex-end".to_string())),
        "items-stretch" => Some(("align-items", "stretch".to_string())),
        "items-baseline" => Some(("align-items", "baseline".to_string())),
        "justify-start" => Some(("justify-content", "flex-start".to_string())),
        "justify-center" => Some(("justify-content", "center".to_string())),
        "justify-end" => Some(("justify-content", "flex-end".to_string())),
        "justify-between" => Some(("justify-content", "space-between".to_string())),
        "justify-around" => Some(("justify-content", "space-around".to_string())),
        "justify-evenly" => Some(("justify-content", "space-evenly".to_string())),
        "overflow-visible" => Some(("overflow", "visible".to_string())),
        "overflow-hidden" => Some(("overflow", "hidden".to_string())),
        "overflow-scroll" => Some(("overflow", "scroll".to_string())),
        "overflow-auto" => Some(("overflow", "auto".to_string())),
        "overflow-clip" => Some(("overflow", "clip".to_string())),
        "overflow-x-visible" => Some(("overflow-x", "visible".to_string())),
        "overflow-x-hidden" => Some(("overflow-x", "hidden".to_string())),
        "overflow-x-scroll" => Some(("overflow-x", "scroll".to_string())),
        "overflow-x-auto" => Some(("overflow-x", "auto".to_string())),
        "overflow-x-clip" => Some(("overflow-x", "clip".to_string())),
        "overflow-y-visible" => Some(("overflow-y", "visible".to_string())),
        "overflow-y-hidden" => Some(("overflow-y", "hidden".to_string())),
        "overflow-y-scroll" => Some(("overflow-y", "scroll".to_string())),
        "overflow-y-auto" => Some(("overflow-y", "auto".to_string())),
        "overflow-y-clip" => Some(("overflow-y", "clip".to_string())),
        "visible" => Some(("visibility", "visible".to_string())),
        "invisible" => Some(("visibility", "hidden".to_string())),
        "collapse" => Some(("visibility", "collapse".to_string())),
        "font-thin" => Some(("font-weight", "100".to_string())),
        "font-extralight" => Some(("font-weight", "200".to_string())),
        "font-light" => Some(("font-weight", "300".to_string())),
        "font-normal" => Some(("font-weight", "400".to_string())),
        "font-medium" => Some(("font-weight", "500".to_string())),
        "font-semibold" => Some(("font-weight", "600".to_string())),
        "font-bold" => Some(("font-weight", "700".to_string())),
        "font-extrabold" => Some(("font-weight", "800".to_string())),
        "font-black" => Some(("font-weight", "900".to_string())),
        "text-left" => Some(("text-align", "left".to_string())),
        "text-center" => Some(("text-align", "center".to_string())),
        "text-right" => Some(("text-align", "right".to_string())),
        "text-justify" => Some(("text-align", "justify".to_string())),
        "text-start" => Some(("text-align", "start".to_string())),
        "text-end" => Some(("text-align", "end".to_string())),
        "border" => Some(("border-width", "1px".to_string())),
        "border-solid" => Some(("border-style", "solid".to_string())),
        "border-dashed" => Some(("border-style", "dashed".to_string())),
        "border-dotted" => Some(("border-style", "dotted".to_string())),
        "border-double" => Some(("border-style", "double".to_string())),
        "border-hidden" => Some(("border-style", "hidden".to_string())),
        "border-none" => Some(("border-style", "none".to_string())),
        "rounded" => Some(("border-radius", "4px".to_string())),
        "rounded-none" => Some(("border-radius", "0px".to_string())),
        "rounded-sm" => Some(("border-radius", "2px".to_string())),
        "rounded-md" => Some(("border-radius", "6px".to_string())),
        "rounded-lg" => Some(("border-radius", "8px".to_string())),
        "rounded-xl" => Some(("border-radius", "12px".to_string())),
        "rounded-2xl" => Some(("border-radius", "16px".to_string())),
        "rounded-3xl" => Some(("border-radius", "24px".to_string())),
        "rounded-full" => Some(("border-radius", "9999px".to_string())),
        _ => None,
    };
    if let Some((property, value)) = declaration {
        declarations.insert(property.to_string(), value);
        return declarations;
    }
    if let Some(text_size) = tailwind_text_size_declarations(class) {
        declarations.extend(text_size);
        return declarations;
    }
    if let Some((properties, value)) = tailwind_inset_utility(class) {
        insert_position_declarations(&mut declarations, properties, value);
        return declarations;
    }
    if let Some((edges, value)) = tailwind_border_width_utility(class) {
        insert_border_width_declarations(&mut declarations, edges, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_prefixed_declaration(class) {
        declarations.insert(property, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "p") {
        insert_edge_declarations(&mut declarations, "padding", edges, value);
    } else if let Some((edges, value)) = tailwind_edge_utility(class, "m") {
        insert_edge_declarations(&mut declarations, "margin", edges, value);
    }
    declarations
}

fn tailwind_prefixed_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class.strip_prefix("w-").and_then(tailwind_length) {
        Some(("width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("h-").and_then(tailwind_length) {
        Some(("height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-w-").and_then(tailwind_length) {
        Some(("min-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("min-h-").and_then(tailwind_length) {
        Some(("min-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-w-").and_then(tailwind_length) {
        Some(("max-width".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("max-h-").and_then(tailwind_length) {
        Some(("max-height".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-").and_then(tailwind_length) {
        Some(("gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-x-").and_then(tailwind_length) {
        Some(("column-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("gap-y-").and_then(tailwind_length) {
        Some(("row-gap".to_string(), style_length_css(value)))
    } else if let Some(value) = class.strip_prefix("opacity-").and_then(tailwind_opacity) {
        Some(("opacity".to_string(), trim_float(value)))
    } else if let Some(value) = tailwind_z_index(class) {
        Some(("z-index".to_string(), value))
    } else if let Some(value) = class.strip_prefix("bg-").and_then(tailwind_color_css) {
        Some(("background-color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("border-").and_then(tailwind_color_css) {
        Some(("border-color".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("leading-")
        .and_then(tailwind_line_height)
    {
        Some(("line-height".to_string(), value))
    } else if let Some(value) = class.strip_prefix("text-").and_then(tailwind_color_css) {
        Some(("color".to_string(), value))
    } else if let Some(value) = class.strip_prefix("rounded-").and_then(tailwind_length) {
        Some(("border-radius".to_string(), style_length_css(value)))
    } else {
        None
    }
}

fn insert_edge_declarations(
    declarations: &mut BTreeMap<String, String>,
    prefix: &str,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert(prefix.to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert(format!("{prefix}-left"), value.clone());
            declarations.insert(format!("{prefix}-right"), value);
        }
        EdgeSelection::Y => {
            declarations.insert(format!("{prefix}-top"), value.clone());
            declarations.insert(format!("{prefix}-bottom"), value);
        }
        EdgeSelection::Top => {
            declarations.insert(format!("{prefix}-top"), value);
        }
        EdgeSelection::Right => {
            declarations.insert(format!("{prefix}-right"), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert(format!("{prefix}-bottom"), value);
        }
        EdgeSelection::Left => {
            declarations.insert(format!("{prefix}-left"), value);
        }
    }
}

fn insert_position_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("inset".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("left".to_string(), value.clone());
            declarations.insert("right".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("top".to_string(), value.clone());
            declarations.insert("bottom".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("top".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("right".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("bottom".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("left".to_string(), value);
        }
    }
}

fn insert_border_width_declarations(
    declarations: &mut BTreeMap<String, String>,
    edges: EdgeSelection,
    value: StyleLength,
) {
    let value = style_length_css(value);
    match edges {
        EdgeSelection::All => {
            declarations.insert("border-width".to_string(), value);
        }
        EdgeSelection::X => {
            declarations.insert("border-left-width".to_string(), value.clone());
            declarations.insert("border-right-width".to_string(), value);
        }
        EdgeSelection::Y => {
            declarations.insert("border-top-width".to_string(), value.clone());
            declarations.insert("border-bottom-width".to_string(), value);
        }
        EdgeSelection::Top => {
            declarations.insert("border-top-width".to_string(), value);
        }
        EdgeSelection::Right => {
            declarations.insert("border-right-width".to_string(), value);
        }
        EdgeSelection::Bottom => {
            declarations.insert("border-bottom-width".to_string(), value);
        }
        EdgeSelection::Left => {
            declarations.insert("border-left-width".to_string(), value);
        }
    }
}

fn style_length_css(value: StyleLength) -> String {
    match value {
        StyleLength::Points(value) => format!("{}px", trim_float(value)),
        StyleLength::Percent(value) => format!("{}%", trim_float(value)),
        StyleLength::Auto => "auto".to_string(),
    }
}

fn trim_float(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}

fn tailwind_arbitrary_value(value: &str) -> String {
    value.replace('_', " ")
}

fn tailwind_text_size_declarations(class: &str) -> Option<BTreeMap<String, String>> {
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

fn tailwind_line_height(value: &str) -> Option<String> {
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

fn tailwind_color_css(value: &str) -> Option<String> {
    let value = value.split_once('/').map_or(value, |(value, _)| value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    match value {
        "black" => Some("#000".to_string()),
        "white" => Some("#fff".to_string()),
        "transparent" => Some("transparent".to_string()),
        "current" => Some("currentColor".to_string()),
        other if is_tailwind_palette_color(other) => Some(other.to_string()),
        _ => None,
    }
}

fn tailwind_z_index(class: &str) -> Option<String> {
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

fn tailwind_inset_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
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

fn tailwind_border_width_utility(class: &str) -> Option<(EdgeSelection, StyleLength)> {
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

fn tailwind_border_width(value: &str) -> Option<StyleLength> {
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

fn negate_style_length(value: StyleLength) -> Option<StyleLength> {
    match value {
        StyleLength::Points(value) => Some(StyleLength::Points(-value)),
        StyleLength::Percent(value) => Some(StyleLength::Percent(-value)),
        StyleLength::Auto => None,
    }
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
            .style("position", "absolute")
            .style("inset", "1px 2px 3px 4px")
            .style("paddingTop", "12")
            .style("margin", "1px 2px 3px 4px")
            .style("border", "2px solid #000")
            .style("fontWeight", "700")
            .style("lineHeight", "1.5rem")
            .style("textAlign", "center")
            .style("overflow", "hidden")
            .style("--brand-accent", "#663399")
            .style("backgroundColor", "#663399")
            .style("boxShadow", "0 1px 3px black");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Flex));
        assert_eq!(style.flex_direction, Some(Orientation::Horizontal));
        assert_eq!(style.min_width, Some(StyleLength::Points(280.0)));
        assert_eq!(style.gap, Some(StyleLength::Points(8.0)));
        assert_eq!(style.position, Some(PositionMode::Absolute));
        assert_eq!(style.inset.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.inset.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.inset.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.inset.left, Some(StyleLength::Points(4.0)));
        assert_eq!(style.padding.top, Some(StyleLength::Points(12.0)));
        assert_eq!(style.margin.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.margin.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.margin.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.margin.left, Some(StyleLength::Points(4.0)));
        assert_eq!(style.border_width.top, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_style, Some(BorderStyle::Solid));
        assert_eq!(
            style.border_color,
            Some(StyleColor::Rgba {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 255,
            })
        );
        assert_eq!(style.font_weight, Some(FontWeight::Number(700)));
        assert_eq!(style.line_height, Some(StyleLength::Points(24.0)));
        assert_eq!(style.text_align, Some(TextAlign::Center));
        assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
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
            style
                .declarations
                .get("background-color")
                .map(String::as_str),
            Some("#663399")
        );
        assert_eq!(
            style
                .custom_properties
                .get("--brand-accent")
                .map(String::as_str),
            Some("#663399")
        );
        assert_eq!(
            style.unsupported.get("box-shadow").map(String::as_str),
            Some("0 1px 3px black")
        );
    }

    #[test]
    fn parses_tailwind_utilities_before_inline_style_overrides() {
        let web = WebProps::new()
            .class_name(
                "flex flex-col items-center justify-between min-w-[280px] gap-4 p-2 \
                 mx-auto bg-[#663399] text-white rounded-lg opacity-50 \
                 hover:bg-blue-600 md:flex-row focus:[outline:2px_solid_blue]",
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
            style.declarations.get("min-width").map(String::as_str),
            Some("280px")
        );
        assert_eq!(
            style.declarations.get("gap").map(String::as_str),
            Some("10px")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("background-color"))
                .map(String::as_str),
            Some("blue-600")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("flex-direction"))
                .map(String::as_str),
            Some("row")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("outline"))
                .map(String::as_str),
            Some("2px solid blue")
        );
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

    #[test]
    fn parses_common_tailwind_layout_text_and_border_utilities() {
        let web = WebProps::new().class_name(
            "grid relative inset-x-4 -top-2 z-10 visible flex-wrap gap-x-3 gap-y-5 \
             overflow-x-auto overflow-y-hidden border border-x-2 border-b-[3px] \
             border-dashed border-red-500 text-sm text-center font-semibold leading-tight",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(style.position, Some(PositionMode::Relative));
        assert_eq!(style.inset.left, Some(StyleLength::Points(16.0)));
        assert_eq!(style.inset.right, Some(StyleLength::Points(16.0)));
        assert_eq!(style.inset.top, Some(StyleLength::Points(-8.0)));
        assert_eq!(style.z_index, Some(10));
        assert_eq!(style.visibility, Some(VisibilityMode::Visible));
        assert_eq!(style.flex_wrap, Some(FlexWrap::Wrap));
        assert_eq!(style.column_gap, Some(StyleLength::Points(12.0)));
        assert_eq!(style.row_gap, Some(StyleLength::Points(20.0)));
        assert_eq!(style.overflow_x, Some(OverflowMode::Auto));
        assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
        assert_eq!(style.border_width.top, Some(StyleLength::Points(1.0)));
        assert_eq!(style.border_width.left, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
        assert_eq!(style.border_width.bottom, Some(StyleLength::Points(3.0)));
        assert_eq!(style.border_style, Some(BorderStyle::Dashed));
        assert_eq!(
            style.border_color,
            Some(StyleColor::Keyword("red-500".to_string()))
        );
        assert_eq!(style.font_size, Some(StyleLength::Points(14.0)));
        assert_eq!(style.line_height, Some(StyleLength::Points(1.25)));
        assert_eq!(style.text_align, Some(TextAlign::Center));
        assert_eq!(style.font_weight, Some(FontWeight::Number(600)));
        assert_eq!(
            style
                .declarations
                .get("border-right-width")
                .map(String::as_str),
            Some("2px")
        );
        assert_eq!(
            style
                .declarations
                .get("border-bottom-width")
                .map(String::as_str),
            Some("3px")
        );
        assert_eq!(
            style.declarations.get("top").map(String::as_str),
            Some("-8px")
        );
        assert_eq!(
            style.declarations.get("font-size").map(String::as_str),
            Some("0.875rem")
        );
        assert_eq!(
            style.declarations.get("line-height").map(String::as_str),
            Some("1.25")
        );
    }
}
