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
    pub flex: Option<String>,
    pub flex_basis: Option<StyleLength>,
    pub flex_grow: Option<String>,
    pub flex_shrink: Option<String>,
    pub order: Option<String>,
    pub align_items: Option<AlignItems>,
    pub align_content: Option<JustifyContent>,
    pub align_self: Option<SelfAlignment>,
    pub justify_content: Option<JustifyContent>,
    pub justify_items: Option<AlignItems>,
    pub justify_self: Option<SelfAlignment>,
    pub place_content: Option<String>,
    pub place_items: Option<String>,
    pub place_self: Option<String>,
    pub width: Option<StyleLength>,
    pub height: Option<StyleLength>,
    pub min_width: Option<StyleLength>,
    pub min_height: Option<StyleLength>,
    pub max_width: Option<StyleLength>,
    pub max_height: Option<StyleLength>,
    pub gap: Option<StyleLength>,
    pub row_gap: Option<StyleLength>,
    pub column_gap: Option<StyleLength>,
    pub grid: Option<String>,
    pub grid_template: Option<String>,
    pub grid_template_columns: Option<String>,
    pub grid_template_rows: Option<String>,
    pub grid_template_areas: Option<String>,
    pub grid_auto_columns: Option<String>,
    pub grid_auto_rows: Option<String>,
    pub grid_auto_flow: Option<GridAutoFlow>,
    pub grid_column: Option<String>,
    pub grid_column_start: Option<String>,
    pub grid_column_end: Option<String>,
    pub grid_row: Option<String>,
    pub grid_row_start: Option<String>,
    pub grid_row_end: Option<String>,
    pub grid_area: Option<String>,
    pub inset: EdgeInsets,
    pub padding: EdgeInsets,
    pub margin: EdgeInsets,
    pub border_width: EdgeInsets,
    pub border_color: Option<StyleColor>,
    pub border_style: Option<BorderStyle>,
    pub box_shadow: Option<String>,
    pub outline_width: Option<StyleLength>,
    pub outline_color: Option<StyleColor>,
    pub outline_style: Option<BorderStyle>,
    pub outline_offset: Option<StyleLength>,
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
    pub aspect_ratio: Option<String>,
    pub transform: Option<String>,
    pub filter: Option<String>,
    pub backdrop_filter: Option<String>,
    pub cursor: Option<String>,
    pub pointer_events: Option<PointerEvents>,
    pub user_select: Option<UserSelect>,
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
            "flex" => self.flex = parse_css_string_token(value_ref),
            "flex-basis" => self.flex_basis = parse_length(value_ref),
            "flex-grow" => self.flex_grow = parse_css_string_token(value_ref),
            "flex-shrink" => self.flex_shrink = parse_css_string_token(value_ref),
            "order" => self.order = parse_css_string_token(value_ref),
            "align-items" => self.align_items = parse_align_items(value_ref),
            "align-content" => self.align_content = parse_justify_content(value_ref),
            "align-self" => self.align_self = parse_self_alignment(value_ref),
            "justify-content" => self.justify_content = parse_justify_content(value_ref),
            "justify-items" => self.justify_items = parse_align_items(value_ref),
            "justify-self" => self.justify_self = parse_self_alignment(value_ref),
            "place-content" => self.place_content = parse_css_string_token(value_ref),
            "place-items" => self.place_items = parse_css_string_token(value_ref),
            "place-self" => self.place_self = parse_css_string_token(value_ref),
            "width" => self.width = parse_length(value_ref),
            "height" => self.height = parse_length(value_ref),
            "min-width" => self.min_width = parse_length(value_ref),
            "min-height" => self.min_height = parse_length(value_ref),
            "max-width" => self.max_width = parse_length(value_ref),
            "max-height" => self.max_height = parse_length(value_ref),
            "gap" => self.gap = parse_length(value_ref),
            "row-gap" => self.row_gap = parse_length(value_ref),
            "column-gap" => self.column_gap = parse_length(value_ref),
            "grid" => self.grid = parse_css_string_token(value_ref),
            "grid-template" => self.grid_template = parse_css_string_token(value_ref),
            "grid-template-columns" => {
                self.grid_template_columns = parse_css_string_token(value_ref);
            }
            "grid-template-rows" => self.grid_template_rows = parse_css_string_token(value_ref),
            "grid-template-areas" => self.grid_template_areas = parse_css_string_token(value_ref),
            "grid-auto-columns" => self.grid_auto_columns = parse_css_string_token(value_ref),
            "grid-auto-rows" => self.grid_auto_rows = parse_css_string_token(value_ref),
            "grid-auto-flow" => self.grid_auto_flow = parse_grid_auto_flow(value_ref),
            "grid-column" => self.grid_column = parse_css_string_token(value_ref),
            "grid-column-start" => self.grid_column_start = parse_css_string_token(value_ref),
            "grid-column-end" => self.grid_column_end = parse_css_string_token(value_ref),
            "grid-row" => self.grid_row = parse_css_string_token(value_ref),
            "grid-row-start" => self.grid_row_start = parse_css_string_token(value_ref),
            "grid-row-end" => self.grid_row_end = parse_css_string_token(value_ref),
            "grid-area" => self.grid_area = parse_css_string_token(value_ref),
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
            "box-shadow" => self.box_shadow = parse_css_string_token(value_ref),
            "outline" => self.apply_outline_shorthand(value_ref),
            "outline-width" => self.outline_width = parse_length(value_ref),
            "outline-color" => self.outline_color = parse_color(value_ref),
            "outline-style" => self.outline_style = parse_border_style(value_ref),
            "outline-offset" => self.outline_offset = parse_length(value_ref),
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
            "aspect-ratio" => self.aspect_ratio = parse_css_string_token(value_ref),
            "transform" => self.transform = parse_css_string_token(value_ref),
            "filter" => self.filter = parse_css_string_token(value_ref),
            "backdrop-filter" => self.backdrop_filter = parse_css_string_token(value_ref),
            "cursor" => self.cursor = parse_css_string_token(value_ref),
            "pointer-events" => self.pointer_events = parse_pointer_events(value_ref),
            "user-select" => self.user_select = parse_user_select(value_ref),
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

    fn apply_outline_shorthand(&mut self, value: &str) {
        for part in value.split_whitespace() {
            if let Some(width) = parse_length(part) {
                self.outline_width = Some(width);
            } else if let Some(style) = parse_border_style(part) {
                self.outline_style = Some(style);
            } else if let Some(color) = parse_color(part) {
                self.outline_color = Some(color);
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
pub enum GridAutoFlow {
    Row,
    Column,
    Dense,
    RowDense,
    ColumnDense,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AlignItems {
    Normal,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JustifyContent {
    Normal,
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
    Stretch,
    Baseline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SelfAlignment {
    Auto,
    Start,
    Center,
    End,
    Stretch,
    Baseline,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointerEvents {
    Auto,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserSelect {
    Auto,
    Text,
    None,
    All,
    Contain,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleLength {
    Points(f64),
    Percent(f64),
    Auto,
    Css(String),
}

impl StyleLength {
    pub fn points(&self) -> Option<f64> {
        match self {
            StyleLength::Points(value) => Some(*value),
            StyleLength::Percent(_) | StyleLength::Auto | StyleLength::Css(_) => None,
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeInsets {
    pub top: Option<StyleLength>,
    pub right: Option<StyleLength>,
    pub bottom: Option<StyleLength>,
    pub left: Option<StyleLength>,
}

impl EdgeInsets {
    fn set_all(&mut self, value: Option<StyleLength>) {
        self.top = value.clone();
        self.right = value.clone();
        self.bottom = value.clone();
        self.left = value;
    }

    fn apply_edges(&mut self, edges: EdgeSelection, value: StyleLength) {
        match edges {
            EdgeSelection::All => self.set_all(Some(value)),
            EdgeSelection::X => {
                self.left = Some(value.clone());
                self.right = Some(value);
            }
            EdgeSelection::Y => {
                self.top = Some(value.clone());
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

fn parse_grid_auto_flow(value: &str) -> Option<GridAutoFlow> {
    match value.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["row"] => Some(GridAutoFlow::Row),
        ["column"] => Some(GridAutoFlow::Column),
        ["dense"] => Some(GridAutoFlow::Dense),
        ["row", "dense"] | ["dense", "row"] => Some(GridAutoFlow::RowDense),
        ["column", "dense"] | ["dense", "column"] => Some(GridAutoFlow::ColumnDense),
        _ => None,
    }
}

fn parse_align_items(value: &str) -> Option<AlignItems> {
    match value.trim() {
        "normal" => Some(AlignItems::Normal),
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
        "normal" => Some(JustifyContent::Normal),
        "flex-start" | "start" => Some(JustifyContent::Start),
        "center" => Some(JustifyContent::Center),
        "flex-end" | "end" => Some(JustifyContent::End),
        "space-between" => Some(JustifyContent::SpaceBetween),
        "space-around" => Some(JustifyContent::SpaceAround),
        "space-evenly" => Some(JustifyContent::SpaceEvenly),
        "stretch" => Some(JustifyContent::Stretch),
        "baseline" => Some(JustifyContent::Baseline),
        _ => None,
    }
}

fn parse_self_alignment(value: &str) -> Option<SelfAlignment> {
    match value.trim() {
        "auto" => Some(SelfAlignment::Auto),
        "flex-start" | "start" => Some(SelfAlignment::Start),
        "center" => Some(SelfAlignment::Center),
        "flex-end" | "end" => Some(SelfAlignment::End),
        "stretch" => Some(SelfAlignment::Stretch),
        "baseline" => Some(SelfAlignment::Baseline),
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

fn parse_css_string_token(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

fn parse_pointer_events(value: &str) -> Option<PointerEvents> {
    match value.trim() {
        "auto" => Some(PointerEvents::Auto),
        "none" => Some(PointerEvents::None),
        _ => None,
    }
}

fn parse_user_select(value: &str) -> Option<UserSelect> {
    match value.trim() {
        "auto" => Some(UserSelect::Auto),
        "text" => Some(UserSelect::Text),
        "none" => Some(UserSelect::None),
        "all" => Some(UserSelect::All),
        "contain" => Some(UserSelect::Contain),
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
        [all] => edges.set_all(Some(all.clone())),
        [vertical, horizontal] => {
            edges.top = Some(vertical.clone());
            edges.bottom = Some(vertical.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
        }
        [top, horizontal, bottom] => {
            edges.top = Some(top.clone());
            edges.left = Some(horizontal.clone());
            edges.right = Some(horizontal.clone());
            edges.bottom = Some(bottom.clone());
        }
        [top, right, bottom, left, ..] => {
            edges.top = Some(top.clone());
            edges.right = Some(right.clone());
            edges.bottom = Some(bottom.clone());
            edges.left = Some(left.clone());
        }
    }
    edges
}

fn parse_length(value: &str) -> Option<StyleLength> {
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
            | "min-content"
            | "max-content"
            | "fit-content"
            | "stretch"
            | "contain"
    ) {
        return true;
    }
    if matches!(
        value.split_once('(').map(|(name, _)| name.trim()),
        Some("calc" | "min" | "max" | "clamp" | "var" | "env" | "fit-content")
    ) && value.ends_with(')')
    {
        return true;
    }
    let Some((number, unit)) = split_number_and_unit(value) else {
        return false;
    };
    number.parse::<f64>().is_ok() && is_css_length_unit(unit)
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

fn parse_color(value: &str) -> Option<StyleColor> {
    let value = value.trim();
    if let Some(hex) = value.strip_prefix('#') {
        return parse_hex_color(hex);
    }
    if let Some(color) = parse_rgb_function(value) {
        return Some(color);
    }
    if let Some(color) = parse_hsl_function(value) {
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
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let red = parse_rgb_channel(&channels[0])?;
    let green = parse_rgb_channel(&channels[1])?;
    let blue = parse_rgb_channel(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_rgb_channel(value: &str) -> Option<u8> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
    }
    value.trim().parse::<u8>().ok()
}

fn parse_hsl_function(value: &str) -> Option<StyleColor> {
    let content = value
        .strip_prefix("hsl(")
        .or_else(|| value.strip_prefix("hsla("))?
        .strip_suffix(')')?;
    let (channels, alpha) = parse_color_function_parts(content);
    if channels.len() < 3 {
        return None;
    }
    let hue = parse_hue_degrees(&channels[0])?;
    let saturation = parse_percent_fraction(&channels[1])?;
    let lightness = parse_percent_fraction(&channels[2])?;
    let alpha = alpha
        .as_deref()
        .and_then(parse_alpha_channel)
        .unwrap_or(255);
    let (red, green, blue) = hsl_to_rgb(hue, saturation, lightness);
    Some(StyleColor::Rgba {
        red,
        green,
        blue,
        alpha,
    })
}

fn parse_color_function_parts(content: &str) -> (Vec<String>, Option<String>) {
    let content = content.replace(',', " ");
    let mut channels = Vec::new();
    let mut alpha = None;
    let mut alpha_next = false;
    for part in content.split_whitespace() {
        if part == "/" {
            alpha_next = true;
        } else if let Some((before, after)) = part.split_once('/') {
            if !before.is_empty() {
                channels.push(before.to_string());
            }
            if !after.is_empty() {
                alpha = Some(after.to_string());
            }
            alpha_next = false;
        } else if alpha_next {
            alpha = Some(part.to_string());
            alpha_next = false;
        } else {
            channels.push(part.to_string());
        }
    }
    if alpha.is_none() && channels.len() > 3 {
        alpha = channels.pop();
    }
    (channels, alpha)
}

fn parse_hue_degrees(value: &str) -> Option<f64> {
    let value = value.trim();
    let degrees = if let Some(degrees) = value.strip_suffix("deg") {
        degrees.trim().parse::<f64>().ok()?
    } else if let Some(turns) = value.strip_suffix("turn") {
        turns.trim().parse::<f64>().ok()? * 360.0
    } else if let Some(radians) = value.strip_suffix("rad") {
        radians.trim().parse::<f64>().ok()?.to_degrees()
    } else if let Some(gradians) = value.strip_suffix("grad") {
        gradians.trim().parse::<f64>().ok()? * 0.9
    } else {
        value.parse::<f64>().ok()?
    };
    Some(degrees.rem_euclid(360.0))
}

fn parse_percent_fraction(value: &str) -> Option<f64> {
    let value = value.trim().strip_suffix('%')?.trim();
    Some((value.parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0))
}

fn hsl_to_rgb(hue: f64, saturation: f64, lightness: f64) -> (u8, u8, u8) {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let hue_prime = hue / 60.0;
    let x = chroma * (1.0 - (hue_prime % 2.0 - 1.0).abs());
    let (red1, green1, blue1) = if (0.0..1.0).contains(&hue_prime) {
        (chroma, x, 0.0)
    } else if (1.0..2.0).contains(&hue_prime) {
        (x, chroma, 0.0)
    } else if (2.0..3.0).contains(&hue_prime) {
        (0.0, chroma, x)
    } else if (3.0..4.0).contains(&hue_prime) {
        (0.0, x, chroma)
    } else if (4.0..5.0).contains(&hue_prime) {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };
    let m = lightness - chroma / 2.0;
    (
        ((red1 + m) * 255.0).round() as u8,
        ((green1 + m) * 255.0).round() as u8,
        ((blue1 + m) * 255.0).round() as u8,
    )
}

fn parse_alpha_channel(value: &str) -> Option<u8> {
    let value = value.trim().trim_start_matches('/');
    if let Some(percent) = value.strip_suffix('%') {
        let percent = percent.trim().parse::<f64>().ok()?;
        return Some(((percent.clamp(0.0, 100.0) / 100.0) * 255.0).round() as u8);
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
        "items-normal" => Some(("align-items", "normal".to_string())),
        "content-normal" => Some(("align-content", "normal".to_string())),
        "content-center" => Some(("align-content", "center".to_string())),
        "content-start" => Some(("align-content", "flex-start".to_string())),
        "content-end" => Some(("align-content", "flex-end".to_string())),
        "content-between" => Some(("align-content", "space-between".to_string())),
        "content-around" => Some(("align-content", "space-around".to_string())),
        "content-evenly" => Some(("align-content", "space-evenly".to_string())),
        "content-baseline" => Some(("align-content", "baseline".to_string())),
        "content-stretch" => Some(("align-content", "stretch".to_string())),
        "self-auto" => Some(("align-self", "auto".to_string())),
        "self-start" => Some(("align-self", "flex-start".to_string())),
        "self-center" => Some(("align-self", "center".to_string())),
        "self-end" => Some(("align-self", "flex-end".to_string())),
        "self-stretch" => Some(("align-self", "stretch".to_string())),
        "self-baseline" => Some(("align-self", "baseline".to_string())),
        "justify-normal" => Some(("justify-content", "normal".to_string())),
        "justify-start" => Some(("justify-content", "flex-start".to_string())),
        "justify-center" => Some(("justify-content", "center".to_string())),
        "justify-end" => Some(("justify-content", "flex-end".to_string())),
        "justify-between" => Some(("justify-content", "space-between".to_string())),
        "justify-around" => Some(("justify-content", "space-around".to_string())),
        "justify-evenly" => Some(("justify-content", "space-evenly".to_string())),
        "justify-stretch" => Some(("justify-content", "stretch".to_string())),
        "justify-items-normal" => Some(("justify-items", "normal".to_string())),
        "justify-items-start" => Some(("justify-items", "flex-start".to_string())),
        "justify-items-center" => Some(("justify-items", "center".to_string())),
        "justify-items-end" => Some(("justify-items", "flex-end".to_string())),
        "justify-items-stretch" => Some(("justify-items", "stretch".to_string())),
        "justify-self-auto" => Some(("justify-self", "auto".to_string())),
        "justify-self-start" => Some(("justify-self", "flex-start".to_string())),
        "justify-self-center" => Some(("justify-self", "center".to_string())),
        "justify-self-end" => Some(("justify-self", "flex-end".to_string())),
        "justify-self-stretch" => Some(("justify-self", "stretch".to_string())),
        "place-content-center" => Some(("place-content", "center".to_string())),
        "place-content-start" => Some(("place-content", "start".to_string())),
        "place-content-end" => Some(("place-content", "end".to_string())),
        "place-content-between" => Some(("place-content", "space-between".to_string())),
        "place-content-around" => Some(("place-content", "space-around".to_string())),
        "place-content-evenly" => Some(("place-content", "space-evenly".to_string())),
        "place-content-baseline" => Some(("place-content", "baseline".to_string())),
        "place-content-stretch" => Some(("place-content", "stretch".to_string())),
        "place-items-start" => Some(("place-items", "start".to_string())),
        "place-items-center" => Some(("place-items", "center".to_string())),
        "place-items-end" => Some(("place-items", "end".to_string())),
        "place-items-baseline" => Some(("place-items", "baseline".to_string())),
        "place-items-stretch" => Some(("place-items", "stretch".to_string())),
        "place-self-auto" => Some(("place-self", "auto".to_string())),
        "place-self-start" => Some(("place-self", "start".to_string())),
        "place-self-center" => Some(("place-self", "center".to_string())),
        "place-self-end" => Some(("place-self", "end".to_string())),
        "place-self-stretch" => Some(("place-self", "stretch".to_string())),
        "flex-1" => Some(("flex", "1".to_string())),
        "flex-auto" => Some(("flex", "auto".to_string())),
        "flex-initial" => Some(("flex", "0 auto".to_string())),
        "flex-none" => Some(("flex", "none".to_string())),
        "basis-auto" => Some(("flex-basis", "auto".to_string())),
        "basis-full" => Some(("flex-basis", "100%".to_string())),
        "grow" => Some(("flex-grow", "1".to_string())),
        "grow-0" => Some(("flex-grow", "0".to_string())),
        "shrink" => Some(("flex-shrink", "1".to_string())),
        "shrink-0" => Some(("flex-shrink", "0".to_string())),
        "order-first" => Some(("order", "-9999".to_string())),
        "order-last" => Some(("order", "9999".to_string())),
        "order-none" => Some(("order", "0".to_string())),
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
        "outline" => Some(("outline-width", "1px".to_string())),
        "outline-none" => Some(("outline", "2px solid transparent".to_string())),
        "outline-hidden" => Some(("outline-style", "none".to_string())),
        "outline-solid" => Some(("outline-style", "solid".to_string())),
        "outline-dashed" => Some(("outline-style", "dashed".to_string())),
        "outline-dotted" => Some(("outline-style", "dotted".to_string())),
        "outline-double" => Some(("outline-style", "double".to_string())),
        "shadow" => Some((
            "box-shadow",
            "0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xs" => Some(("box-shadow", "0 1px rgb(0 0 0 / 0.05)".to_string())),
        "shadow-sm" => Some(("box-shadow", "0 1px 2px 0 rgb(0 0 0 / 0.05)".to_string())),
        "shadow-md" => Some((
            "box-shadow",
            "0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-lg" => Some((
            "box-shadow",
            "0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-xl" => Some((
            "box-shadow",
            "0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1)".to_string(),
        )),
        "shadow-2xl" => Some((
            "box-shadow",
            "0 25px 50px -12px rgb(0 0 0 / 0.25)".to_string(),
        )),
        "shadow-inner" => Some((
            "box-shadow",
            "inset 0 2px 4px 0 rgb(0 0 0 / 0.05)".to_string(),
        )),
        "shadow-none" => Some(("box-shadow", "none".to_string())),
        "transform" => Some(("transform", "translateZ(0)".to_string())),
        "transform-none" => Some(("transform", "none".to_string())),
        "filter" => Some(("filter", "var(--tw-filter)".to_string())),
        "filter-none" => Some(("filter", "none".to_string())),
        "backdrop-filter" => Some(("backdrop-filter", "var(--tw-backdrop-filter)".to_string())),
        "backdrop-filter-none" => Some(("backdrop-filter", "none".to_string())),
        "pointer-events-auto" => Some(("pointer-events", "auto".to_string())),
        "pointer-events-none" => Some(("pointer-events", "none".to_string())),
        "select-auto" => Some(("user-select", "auto".to_string())),
        "select-text" => Some(("user-select", "text".to_string())),
        "select-none" => Some(("user-select", "none".to_string())),
        "select-all" => Some(("user-select", "all".to_string())),
        "resize-none" => Some(("resize", "none".to_string())),
        "resize" => Some(("resize", "both".to_string())),
        "resize-x" => Some(("resize", "horizontal".to_string())),
        "resize-y" => Some(("resize", "vertical".to_string())),
        "aspect-auto" => Some(("aspect-ratio", "auto".to_string())),
        "aspect-square" => Some(("aspect-ratio", "1 / 1".to_string())),
        "aspect-video" => Some(("aspect-ratio", "16 / 9".to_string())),
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
    if let Some((property, value)) = tailwind_visual_effect_declaration(class) {
        declarations.insert(property, value);
        return declarations;
    }
    if let Some((property, value)) = tailwind_grid_declaration(class) {
        declarations.insert(property, value);
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
    } else if let Some(value) = class.strip_prefix("flex-").and_then(tailwind_flex_value) {
        Some(("flex".to_string(), value))
    } else if let Some(value) = class.strip_prefix("basis-").and_then(tailwind_basis_value) {
        Some(("flex-basis".to_string(), value))
    } else if let Some(value) = class.strip_prefix("grow-").and_then(tailwind_number_token) {
        Some(("flex-grow".to_string(), value))
    } else if let Some(value) = class
        .strip_prefix("shrink-")
        .and_then(tailwind_number_token)
    {
        Some(("flex-shrink".to_string(), value))
    } else if let Some(value) = tailwind_order_value(class) {
        Some(("order".to_string(), value))
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

fn tailwind_flex_value(value: &str) -> Option<String> {
    match value {
        "auto" => Some("auto".to_string()),
        "initial" => Some("0 auto".to_string()),
        "none" => Some("none".to_string()),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| tailwind_fraction(value).map(|value| format!("calc({value} * 100%)")))
            .or_else(|| value.parse::<f64>().ok().map(trim_float)),
    }
}

fn tailwind_basis_value(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value)
        .or_else(|| tailwind_container_width(value).map(ToString::to_string))
        .or_else(|| tailwind_length(value).map(style_length_css))
}

fn tailwind_number_token(value: &str) -> Option<String> {
    tailwind_arbitrary_or_custom_var(value).or_else(|| value.parse::<f64>().ok().map(trim_float))
}

fn tailwind_order_value(class: &str) -> Option<String> {
    let negative = class.starts_with("-order-");
    let value = if negative {
        class.strip_prefix("-order-")?
    } else {
        class.strip_prefix("order-")?
    };
    let value = match value {
        "first" if !negative => "-9999".to_string(),
        "last" if !negative => "9999".to_string(),
        "none" if !negative => "0".to_string(),
        _ => tailwind_arbitrary_or_custom_var(value)
            .or_else(|| value.parse::<i32>().ok().map(|value| value.to_string()))?,
    };
    Some(if negative {
        format!("calc({value} * -1)")
    } else {
        value
    })
}

fn tailwind_fraction(value: &str) -> Option<String> {
    let (numerator, denominator) = value.split_once('/')?;
    let numerator = numerator.parse::<f64>().ok()?;
    let denominator = denominator.parse::<f64>().ok()?;
    if denominator == 0.0 {
        None
    } else {
        Some(trim_float(numerator / denominator))
    }
}

fn tailwind_container_width(value: &str) -> Option<&'static str> {
    match value {
        "3xs" => Some("16rem"),
        "2xs" => Some("18rem"),
        "xs" => Some("20rem"),
        "sm" => Some("24rem"),
        "md" => Some("28rem"),
        "lg" => Some("32rem"),
        "xl" => Some("36rem"),
        "2xl" => Some("42rem"),
        "3xl" => Some("48rem"),
        "4xl" => Some("56rem"),
        "5xl" => Some("64rem"),
        "6xl" => Some("72rem"),
        "7xl" => Some("80rem"),
        _ => None,
    }
}

fn tailwind_visual_effect_declaration(class: &str) -> Option<(String, String)> {
    if let Some(value) = class
        .strip_prefix("shadow-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("box-shadow".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("outline".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "outline-offset".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("cursor-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("cursor".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("aspect-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("aspect-ratio".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("transform-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("transform".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(("filter".to_string(), tailwind_arbitrary_value(value)));
    }
    if let Some(value) = class
        .strip_prefix("backdrop-filter-[")
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some((
            "backdrop-filter".to_string(),
            tailwind_arbitrary_value(value),
        ));
    }
    if let Some(value) = class
        .strip_prefix("outline-offset-")
        .and_then(tailwind_length)
    {
        return Some(("outline-offset".to_string(), style_length_css(value)));
    }
    if let Some(value) = class
        .strip_prefix("outline-")
        .and_then(tailwind_border_width)
    {
        return Some(("outline-width".to_string(), style_length_css(value)));
    }
    if let Some(value) = class.strip_prefix("outline-").and_then(tailwind_color_css) {
        return Some(("outline-color".to_string(), value));
    }
    if let Some(value) = class.strip_prefix("cursor-") {
        if is_tailwind_cursor(value) {
            return Some(("cursor".to_string(), value.to_string()));
        }
    }
    if let Some(value) = class.strip_prefix("aspect-") {
        if let Some((width, height)) = value.split_once('/') {
            if width.parse::<f64>().is_ok() && height.parse::<f64>().is_ok() {
                return Some(("aspect-ratio".to_string(), format!("{width} / {height}")));
            }
        }
    }
    if let Some(value) = tailwind_transform_declaration(class) {
        return Some(("transform".to_string(), value));
    }
    None
}

fn tailwind_grid_declaration(class: &str) -> Option<(String, String)> {
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

fn tailwind_grid_track_list(value: &str) -> Option<String> {
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

fn tailwind_grid_auto_track(value: &str) -> Option<String> {
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

fn tailwind_grid_line_utility(class: &str, prefix: &str) -> Option<String> {
    if let Some(value) = class.strip_prefix(prefix).and_then(tailwind_grid_line) {
        return Some(value);
    }
    let negative_prefix = format!("-{prefix}");
    let value = class
        .strip_prefix(&negative_prefix)
        .and_then(tailwind_grid_line)?;
    Some(format!("calc({value} * -1)"))
}

fn tailwind_grid_line(value: &str) -> Option<String> {
    if value == "auto" {
        return Some("auto".to_string());
    }
    if let Some(value) = tailwind_arbitrary_or_custom_var(value) {
        return Some(value);
    }
    value.parse::<u16>().ok().map(|value| value.to_string())
}

fn tailwind_arbitrary_or_custom_var(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    tailwind_custom_var(value)
}

fn tailwind_custom_var(value: &str) -> Option<String> {
    let variable = value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))?
        .trim();
    if variable.is_empty() {
        None
    } else {
        Some(format!("var({variable})"))
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
        StyleLength::Css(value) => value,
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

fn is_tailwind_cursor(value: &str) -> bool {
    matches!(
        value,
        "auto"
            | "default"
            | "pointer"
            | "wait"
            | "text"
            | "move"
            | "help"
            | "not-allowed"
            | "none"
            | "context-menu"
            | "progress"
            | "cell"
            | "crosshair"
            | "vertical-text"
            | "alias"
            | "copy"
            | "no-drop"
            | "grab"
            | "grabbing"
            | "all-scroll"
            | "col-resize"
            | "row-resize"
            | "n-resize"
            | "e-resize"
            | "s-resize"
            | "w-resize"
            | "ne-resize"
            | "nw-resize"
            | "se-resize"
            | "sw-resize"
            | "ew-resize"
            | "ns-resize"
            | "nesw-resize"
            | "nwse-resize"
            | "zoom-in"
            | "zoom-out"
    )
}

fn tailwind_transform_declaration(class: &str) -> Option<String> {
    if let Some(suffix) = class.strip_prefix("rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-rotate-") {
        if let Some(value) = tailwind_rotate_value(suffix) {
            return Some(format!("rotate(-{value})"));
        }
    }
    if let Some(value) = class.strip_prefix("scale-").and_then(tailwind_scale_value) {
        return Some(format!("scale({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-x-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleX({value})"));
    }
    if let Some(value) = class
        .strip_prefix("scale-y-")
        .and_then(tailwind_scale_value)
    {
        return Some(format!("scaleY({value})"));
    }
    if let Some(suffix) = class.strip_prefix("translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-x-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateX(-{value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY({value})"));
        }
    }
    if let Some(suffix) = class.strip_prefix("-translate-y-") {
        if let Some(value) = tailwind_translate_value(suffix) {
            return Some(format!("translateY(-{value})"));
        }
    }
    None
}

fn tailwind_rotate_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(format!("{}deg", trim_float(value.parse::<f64>().ok()?)))
}

fn tailwind_scale_value(value: &str) -> Option<String> {
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        return Some(tailwind_arbitrary_value(arbitrary));
    }
    Some(trim_float(value.parse::<f64>().ok()? / 100.0))
}

fn tailwind_translate_value(value: &str) -> Option<String> {
    tailwind_length(value).map(style_length_css)
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
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let color = parse_color(&tailwind_arbitrary_value(arbitrary))?;
        return Some(apply_tailwind_color_opacity(color, opacity));
    }
    let color = match value {
        "black" => parse_color("#000"),
        "white" => parse_color("#fff"),
        "transparent" => Some(StyleColor::Keyword("transparent".to_string())),
        "current" => Some(StyleColor::Keyword("currentColor".to_string())),
        other if is_tailwind_palette_color(other) => Some(StyleColor::Keyword(other.to_string())),
        _ => None,
    }?;
    Some(apply_tailwind_color_opacity(color, opacity))
}

fn tailwind_color_css(value: &str) -> Option<String> {
    let (value, opacity) = split_tailwind_color_opacity(value);
    if let Some(arbitrary) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    {
        let value = tailwind_arbitrary_value(arbitrary);
        if let Some(color) = parse_color(&value) {
            return Some(style_color_css(&apply_tailwind_color_opacity(
                color, opacity,
            )));
        }
        return Some(apply_tailwind_keyword_opacity(value, opacity));
    }
    let color = match value {
        "black" => parse_color("#000")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "white" => parse_color("#fff")
            .map(|color| style_color_css(&apply_tailwind_color_opacity(color, opacity))),
        "transparent" => Some("transparent".to_string()),
        "current" => Some("currentColor".to_string()),
        other if is_tailwind_palette_color(other) => Some(other.to_string()),
        _ => None,
    }?;
    Some(match value {
        "black" | "white" => color,
        _ => apply_tailwind_keyword_opacity(color, opacity),
    })
}

fn split_tailwind_color_opacity(value: &str) -> (&str, Option<&str>) {
    let mut bracket_depth = 0usize;
    for (index, ch) in value.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '/' if bracket_depth == 0 => return (&value[..index], Some(&value[index + 1..])),
            _ => {}
        }
    }
    (value, None)
}

fn apply_tailwind_color_opacity(color: StyleColor, opacity: Option<&str>) -> StyleColor {
    let Some(alpha) = opacity.and_then(tailwind_opacity_alpha) else {
        return color;
    };
    match color {
        StyleColor::Rgba {
            red, green, blue, ..
        } => StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        },
        StyleColor::Keyword(value) => {
            StyleColor::Keyword(apply_tailwind_keyword_opacity(value, opacity))
        }
    }
}

fn apply_tailwind_keyword_opacity(value: String, opacity: Option<&str>) -> String {
    let Some(opacity) = opacity else {
        return value;
    };
    let Some(percent) = tailwind_opacity_percent(opacity) else {
        return value;
    };
    if value == "transparent" {
        value
    } else {
        format!("{value} / {percent}")
    }
}

fn tailwind_opacity_alpha(value: &str) -> Option<u8> {
    let opacity = tailwind_opacity(value)?;
    Some((opacity.clamp(0.0, 1.0) * 255.0).round() as u8)
}

fn tailwind_opacity_percent(value: &str) -> Option<String> {
    let opacity = tailwind_opacity(value)?;
    Some(format!("{}%", trim_float(opacity.clamp(0.0, 1.0) * 100.0)))
}

fn style_color_css(color: &StyleColor) -> String {
    match color {
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha,
        } if *alpha < 255 => {
            let alpha = trim_float((*alpha as f64 / 255.0 * 100.0).round() / 100.0);
            format!("rgba({red}, {green}, {blue}, {alpha})")
        }
        StyleColor::Rgba {
            red,
            green,
            blue,
            alpha: _,
        } => format!("rgb({red}, {green}, {blue})"),
        StyleColor::Keyword(value) => value.clone(),
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
        StyleLength::Auto | StyleLength::Css(_) => None,
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
        assert_eq!(style.box_shadow.as_deref(), Some("0 1px 3px black"));
        assert!(!style.unsupported.contains_key("box-shadow"));
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

    #[test]
    fn parses_css_grid_properties_into_portable_tokens() {
        let web = WebProps::new()
            .style("display", "grid")
            .style("grid", "auto-flow 1fr / 100px")
            .style("gridTemplateColumns", "repeat(3, minmax(0, 1fr))")
            .style("gridTemplateRows", "auto 1fr")
            .style("gridTemplateAreas", "\"header header\" \"nav main\"")
            .style("gridAutoColumns", "minmax(0, 1fr)")
            .style("gridAutoRows", "min-content")
            .style("gridAutoFlow", "column dense")
            .style("gridColumn", "span 2 / span 2")
            .style("gridColumnStart", "1")
            .style("gridColumnEnd", "-1")
            .style("gridRow", "1 / -1")
            .style("gridRowStart", "2")
            .style("gridRowEnd", "4")
            .style("gridArea", "main");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(style.grid.as_deref(), Some("auto-flow 1fr / 100px"));
        assert_eq!(
            style.grid_template_columns.as_deref(),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(style.grid_template_rows.as_deref(), Some("auto 1fr"));
        assert_eq!(
            style.grid_template_areas.as_deref(),
            Some("\"header header\" \"nav main\"")
        );
        assert_eq!(style.grid_auto_columns.as_deref(), Some("minmax(0, 1fr)"));
        assert_eq!(style.grid_auto_rows.as_deref(), Some("min-content"));
        assert_eq!(style.grid_auto_flow, Some(GridAutoFlow::ColumnDense));
        assert_eq!(style.grid_column.as_deref(), Some("span 2 / span 2"));
        assert_eq!(style.grid_column_start.as_deref(), Some("1"));
        assert_eq!(style.grid_column_end.as_deref(), Some("-1"));
        assert_eq!(style.grid_row.as_deref(), Some("1 / -1"));
        assert_eq!(style.grid_row_start.as_deref(), Some("2"));
        assert_eq!(style.grid_row_end.as_deref(), Some("4"));
        assert_eq!(style.grid_area.as_deref(), Some("main"));
        assert!(!style.unsupported.contains_key("grid-template-columns"));
        assert!(!style.unsupported.contains_key("grid-auto-flow"));
    }

    #[test]
    fn parses_tailwind_grid_utilities_into_portable_tokens() {
        let web = WebProps::new().class_name(
            "grid grid-cols-3 grid-rows-[auto_1fr] auto-cols-fr auto-rows-min \
             grid-flow-col-dense col-span-2 -col-start-2 col-end-[-1] \
             row-span-full row-start-2 row-end-4 \
             md:grid-cols-6 hover:col-span-[3]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.display, Some(DisplayMode::Grid));
        assert_eq!(
            style.grid_template_columns.as_deref(),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(style.grid_template_rows.as_deref(), Some("auto 1fr"));
        assert_eq!(style.grid_auto_columns.as_deref(), Some("minmax(0, 1fr)"));
        assert_eq!(style.grid_auto_rows.as_deref(), Some("min-content"));
        assert_eq!(style.grid_auto_flow, Some(GridAutoFlow::ColumnDense));
        assert_eq!(style.grid_column.as_deref(), Some("span 2 / span 2"));
        assert_eq!(style.grid_column_start.as_deref(), Some("calc(2 * -1)"));
        assert_eq!(style.grid_column_end.as_deref(), Some("-1"));
        assert_eq!(style.grid_row.as_deref(), Some("1 / -1"));
        assert_eq!(style.grid_row_start.as_deref(), Some("2"));
        assert_eq!(style.grid_row_end.as_deref(), Some("4"));
        assert_eq!(
            style
                .declarations
                .get("grid-template-columns")
                .map(String::as_str),
            Some("repeat(3, minmax(0, 1fr))")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("grid-template-columns"))
                .map(String::as_str),
            Some("repeat(6, minmax(0, 1fr))")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("grid-column"))
                .map(String::as_str),
            Some("span 3 / span 3")
        );
    }

    #[test]
    fn parses_css_flex_item_and_box_alignment_properties() {
        let web = WebProps::new()
            .style("flex", "1")
            .style("flexBasis", "25%")
            .style("flexGrow", "2")
            .style("flexShrink", "0")
            .style("order", "3")
            .style("alignContent", "space-between")
            .style("alignSelf", "stretch")
            .style("justifyItems", "center")
            .style("justifySelf", "end")
            .style("placeContent", "center stretch")
            .style("placeItems", "start")
            .style("placeSelf", "end");

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.flex.as_deref(), Some("1"));
        assert_eq!(style.flex_basis, Some(StyleLength::Percent(25.0)));
        assert_eq!(style.flex_grow.as_deref(), Some("2"));
        assert_eq!(style.flex_shrink.as_deref(), Some("0"));
        assert_eq!(style.order.as_deref(), Some("3"));
        assert_eq!(style.align_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.align_self, Some(SelfAlignment::Stretch));
        assert_eq!(style.justify_items, Some(AlignItems::Center));
        assert_eq!(style.justify_self, Some(SelfAlignment::End));
        assert_eq!(style.place_content.as_deref(), Some("center stretch"));
        assert_eq!(style.place_items.as_deref(), Some("start"));
        assert_eq!(style.place_self.as_deref(), Some("end"));
        assert!(!style.unsupported.contains_key("flex-basis"));
        assert!(!style.unsupported.contains_key("align-self"));
    }

    #[test]
    fn parses_tailwind_flex_item_and_box_alignment_utilities() {
        let web = WebProps::new().class_name(
            "flex-1 basis-1/2 grow-2 shrink-0 order-first -order-2 \
             content-between self-end justify-items-center justify-self-stretch \
             place-content-evenly place-items-baseline place-self-start \
             md:basis-[calc(50%_-_1rem)] hover:order-[7]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(style.flex.as_deref(), Some("1"));
        assert_eq!(style.flex_basis, Some(StyleLength::Percent(50.0)));
        assert_eq!(style.flex_grow.as_deref(), Some("2"));
        assert_eq!(style.flex_shrink.as_deref(), Some("0"));
        assert_eq!(style.order.as_deref(), Some("calc(2 * -1)"));
        assert_eq!(style.align_content, Some(JustifyContent::SpaceBetween));
        assert_eq!(style.align_self, Some(SelfAlignment::End));
        assert_eq!(style.justify_items, Some(AlignItems::Center));
        assert_eq!(style.justify_self, Some(SelfAlignment::Stretch));
        assert_eq!(style.place_content.as_deref(), Some("space-evenly"));
        assert_eq!(style.place_items.as_deref(), Some("baseline"));
        assert_eq!(style.place_self.as_deref(), Some("start"));
        assert_eq!(
            style.declarations.get("flex-basis").map(String::as_str),
            Some("50%")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("md")
                .and_then(|styles| styles.get("flex-basis"))
                .map(String::as_str),
            Some("calc(50% - 1rem)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("order"))
                .map(String::as_str),
            Some("7")
        );
    }

    #[test]
    fn preserves_css_length_expressions_as_portable_tokens() {
        let web = WebProps::new()
            .style("width", "calc(100% - 2rem)")
            .style("height", "50dvh")
            .style("minWidth", "min-content")
            .style("maxHeight", "clamp(240px, 50vh, 640px)")
            .style("gap", "var(--space)")
            .style("borderWidth", "fit-content");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.width,
            Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
        );
        assert_eq!(style.height, Some(StyleLength::Css("50dvh".to_string())));
        assert_eq!(
            style.min_width,
            Some(StyleLength::Css("min-content".to_string()))
        );
        assert_eq!(
            style.max_height,
            Some(StyleLength::Css("clamp(240px, 50vh, 640px)".to_string()))
        );
        assert_eq!(
            style.gap,
            Some(StyleLength::Css("var(--space)".to_string()))
        );
        assert_eq!(
            style.border_width.top,
            Some(StyleLength::Css("fit-content".to_string()))
        );
    }

    #[test]
    fn preserves_tailwind_arbitrary_css_length_expressions() {
        let web = WebProps::new().class_name(
            "w-[calc(100%_-_2rem)] h-[50dvh] min-w-[min-content] \
             max-h-[clamp(240px,_50vh,_640px)] gap-[var(--space)]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.width,
            Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
        );
        assert_eq!(style.height, Some(StyleLength::Css("50dvh".to_string())));
        assert_eq!(
            style.min_width,
            Some(StyleLength::Css("min-content".to_string()))
        );
        assert_eq!(
            style.max_height,
            Some(StyleLength::Css("clamp(240px, 50vh, 640px)".to_string()))
        );
        assert_eq!(
            style.gap,
            Some(StyleLength::Css("var(--space)".to_string()))
        );
        assert_eq!(
            style.declarations.get("width").map(String::as_str),
            Some("calc(100% - 2rem)")
        );
    }

    #[test]
    fn parses_css_color_functions_and_alpha_syntax() {
        let web = WebProps::new()
            .style("color", "hsl(210 50% 40% / 50%)")
            .style("backgroundColor", "rgb(10 20 30 / 25%)")
            .style("borderColor", "hsla(120, 100%, 25%, 0.75)");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 51,
                green: 102,
                blue: 153,
                alpha: 128,
            })
        );
        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 10,
                green: 20,
                blue: 30,
                alpha: 64,
            })
        );
        assert_eq!(
            style.border_color,
            Some(StyleColor::Rgba {
                red: 0,
                green: 128,
                blue: 0,
                alpha: 191,
            })
        );
    }

    #[test]
    fn preserves_tailwind_color_opacity_modifiers() {
        let web = WebProps::new()
            .class_name("bg-[#663399]/50 text-white/75 border-blue-600/25 hover:bg-black/40");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.background_color,
            Some(StyleColor::Rgba {
                red: 0x66,
                green: 0x33,
                blue: 0x99,
                alpha: 128,
            })
        );
        assert_eq!(
            style.color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 255,
                blue: 255,
                alpha: 191,
            })
        );
        assert_eq!(
            style.border_color,
            Some(StyleColor::Keyword("blue-600 / 25%".to_string()))
        );
        assert_eq!(
            style
                .declarations
                .get("background-color")
                .map(String::as_str),
            Some("rgba(102, 51, 153, 0.5)")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("background-color"))
                .map(String::as_str),
            Some("rgba(0, 0, 0, 0.4)")
        );
    }

    #[test]
    fn parses_css_visual_effect_and_interaction_properties() {
        let web = WebProps::new()
            .style("boxShadow", "0 2px 8px rgb(0 0 0 / 25%)")
            .style("outline", "2px dashed #ff0000")
            .style("outlineOffset", "4px")
            .style("transform", "translateX(4px) rotate(15deg)")
            .style("filter", "blur(4px)")
            .style("backdropFilter", "saturate(150%)")
            .style("aspectRatio", "4 / 3")
            .style("cursor", "pointer")
            .style("pointerEvents", "none")
            .style("userSelect", "text");

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.box_shadow.as_deref(),
            Some("0 2px 8px rgb(0 0 0 / 25%)")
        );
        assert_eq!(style.outline_width, Some(StyleLength::Points(2.0)));
        assert_eq!(style.outline_style, Some(BorderStyle::Dashed));
        assert_eq!(
            style.outline_color,
            Some(StyleColor::Rgba {
                red: 255,
                green: 0,
                blue: 0,
                alpha: 255,
            })
        );
        assert_eq!(style.outline_offset, Some(StyleLength::Points(4.0)));
        assert_eq!(
            style.transform.as_deref(),
            Some("translateX(4px) rotate(15deg)")
        );
        assert_eq!(style.filter.as_deref(), Some("blur(4px)"));
        assert_eq!(style.backdrop_filter.as_deref(), Some("saturate(150%)"));
        assert_eq!(style.aspect_ratio.as_deref(), Some("4 / 3"));
        assert_eq!(style.cursor.as_deref(), Some("pointer"));
        assert_eq!(style.pointer_events, Some(PointerEvents::None));
        assert_eq!(style.user_select, Some(UserSelect::Text));
        assert!(!style.unsupported.contains_key("box-shadow"));
    }

    #[test]
    fn parses_tailwind_visual_effect_and_interaction_utilities() {
        let web = WebProps::new().class_name(
            "shadow-lg outline-2 outline-offset-4 outline-blue-600 cursor-pointer \
             pointer-events-none select-none aspect-video filter-none backdrop-filter-none \
             rotate-45 hover:shadow-[0_0_4px_black] focus:outline-[3px_solid_red]",
        );

        let style = PortableStyle::from_web(&web);

        assert_eq!(
            style.box_shadow.as_deref(),
            Some("0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)")
        );
        assert_eq!(style.outline_width, Some(StyleLength::Points(2.0)));
        assert_eq!(style.outline_offset, Some(StyleLength::Points(16.0)));
        assert_eq!(
            style.outline_color,
            Some(StyleColor::Keyword("blue-600".to_string()))
        );
        assert_eq!(style.cursor.as_deref(), Some("pointer"));
        assert_eq!(style.pointer_events, Some(PointerEvents::None));
        assert_eq!(style.user_select, Some(UserSelect::None));
        assert_eq!(style.aspect_ratio.as_deref(), Some("16 / 9"));
        assert_eq!(style.filter.as_deref(), Some("none"));
        assert_eq!(style.backdrop_filter.as_deref(), Some("none"));
        assert_eq!(style.transform.as_deref(), Some("rotate(45deg)"));
        assert_eq!(
            style
                .variant_declarations
                .get("hover")
                .and_then(|styles| styles.get("box-shadow"))
                .map(String::as_str),
            Some("0 0 4px black")
        );
        assert_eq!(
            style
                .variant_declarations
                .get("focus")
                .and_then(|styles| styles.get("outline"))
                .map(String::as_str),
            Some("3px solid red")
        );
    }
}
