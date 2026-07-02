use serde::{Deserialize, Serialize};

use super::{parse_border_style, parse_color, parse_length};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DisplayMode {
    Inline,
    InlineBlock,
    Flex,
    InlineFlex,
    Block,
    Grid,
    InlineGrid,
    FlowRoot,
    Contents,
    ListItem,
    Table,
    InlineTable,
    TableCaption,
    TableCell,
    TableColumn,
    TableColumnGroup,
    TableFooterGroup,
    TableHeaderGroup,
    TableRowGroup,
    TableRow,
    Ruby,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
    WebkitBox,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BoxSizing {
    BorderBox,
    ContentBox,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BoxDecorationBreak {
    Slice,
    Clone,
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
pub enum ContainerType {
    Normal,
    Size,
    InlineSize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ContentVisibility {
    Visible,
    Auto,
    Hidden,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundAttachment {
    Fixed,
    Local,
    Scroll,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackgroundBox {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ObjectFit {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ListStylePosition {
    Inside,
    Outside,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FontStyle {
    Normal,
    Italic,
    Oblique,
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
pub enum TextDirection {
    Ltr,
    Rtl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnicodeBidi {
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WritingMode {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
    SidewaysRl,
    SidewaysLr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextOrientation {
    Mixed,
    Upright,
    Sideways,
    SidewaysRight,
    UseGlyphOrientation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextTransform {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FillRule {
    Nonzero,
    Evenodd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StrokeLineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StrokeLineJoin {
    Arcs,
    Bevel,
    Miter,
    MiterClip,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextOverflow {
    Clip,
    Ellipsis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WhiteSpaceMode {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
    BreakSpaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TextWrapMode {
    Wrap,
    NoWrap,
    Balance,
    Pretty,
    Stable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WordBreakMode {
    Normal,
    BreakAll,
    KeepAll,
    BreakWord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverflowWrapMode {
    Normal,
    BreakWord,
    Anywhere,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HyphensMode {
    None,
    Manual,
    Auto,
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
pub enum IsolationMode {
    Auto,
    Isolate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
    PlusDarker,
    PlusLighter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FloatMode {
    Left,
    Right,
    InlineStart,
    InlineEnd,
    Footnote,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ClearMode {
    Left,
    Right,
    Both,
    InlineStart,
    InlineEnd,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TableLayout {
    Auto,
    Fixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BorderCollapse {
    Collapse,
    Separate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CaptionSide {
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PointerEvents {
    Auto,
    None,
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    BoundingBox,
    All,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BackfaceVisibility {
    Visible,
    Hidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ResizeMode {
    None,
    Both,
    Horizontal,
    Vertical,
    Block,
    Inline,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ScrollBehavior {
    Auto,
    Smooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum OverscrollBehavior {
    Auto,
    Contain,
    None,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "camelCase")]
pub enum StyleTime {
    Milliseconds(f64),
    Css(String),
}

impl StyleTime {
    pub fn milliseconds(&self) -> Option<f64> {
        match self {
            StyleTime::Milliseconds(value) => Some(*value),
            StyleTime::Css(_) => None,
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
    Function(String),
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
    pub(super) fn set_all(&mut self, value: Option<StyleLength>) {
        self.top = value.clone();
        self.right = value.clone();
        self.bottom = value.clone();
        self.left = value;
    }

    pub(super) fn apply_edges(&mut self, edges: EdgeSelection, value: StyleLength) {
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

    pub(super) fn apply_edges_opt(&mut self, edges: EdgeSelection, value: Option<StyleLength>) {
        if let Some(value) = value {
            self.apply_edges(edges, value);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum EdgeSelection {
    All,
    X,
    Y,
    Top,
    Right,
    Bottom,
    Left,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalEdgeInsets {
    pub block_start: Option<StyleLength>,
    pub block_end: Option<StyleLength>,
    pub inline_start: Option<StyleLength>,
    pub inline_end: Option<StyleLength>,
}

impl LogicalEdgeInsets {
    pub(super) fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: StyleLength) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value.clone());
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value.clone());
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    pub(super) fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_length(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_length)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(both.clone());
                self.block_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(start.clone());
                self.block_end = Some(end.clone());
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(both.clone());
                self.inline_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(start.clone());
                self.inline_end = Some(end.clone());
            }
            (_, [single, ..]) => self.apply_edges(axis, single.clone()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) enum LogicalEdgeSelection {
    Block,
    Inline,
    BlockStart,
    BlockEnd,
    InlineStart,
    InlineEnd,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeColors {
    pub top: Option<StyleColor>,
    pub right: Option<StyleColor>,
    pub bottom: Option<StyleColor>,
    pub left: Option<StyleColor>,
}

impl EdgeColors {
    pub(super) fn set_all(&mut self, value: Option<StyleColor>) {
        self.top = value.clone();
        self.right = value.clone();
        self.bottom = value.clone();
        self.left = value;
    }

    pub(super) fn apply_edges(&mut self, edges: EdgeSelection, value: StyleColor) {
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
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalEdgeColors {
    pub block_start: Option<StyleColor>,
    pub block_end: Option<StyleColor>,
    pub inline_start: Option<StyleColor>,
    pub inline_end: Option<StyleColor>,
}

impl LogicalEdgeColors {
    pub(super) fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: StyleColor) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value.clone());
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value.clone());
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    pub(super) fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_color(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_color)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(both.clone());
                self.block_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(start.clone());
                self.block_end = Some(end.clone());
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(both.clone());
                self.inline_end = Some(both.clone());
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(start.clone());
                self.inline_end = Some(end.clone());
            }
            (_, [single, ..]) => self.apply_edges(axis, single.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EdgeBorderStyles {
    pub top: Option<BorderStyle>,
    pub right: Option<BorderStyle>,
    pub bottom: Option<BorderStyle>,
    pub left: Option<BorderStyle>,
}

impl EdgeBorderStyles {
    pub(super) fn set_all(&mut self, value: Option<BorderStyle>) {
        self.top = value;
        self.right = value;
        self.bottom = value;
        self.left = value;
    }

    pub(super) fn apply_edges(&mut self, edges: EdgeSelection, value: BorderStyle) {
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

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalBorderStyles {
    pub block_start: Option<BorderStyle>,
    pub block_end: Option<BorderStyle>,
    pub inline_start: Option<BorderStyle>,
    pub inline_end: Option<BorderStyle>,
}

impl LogicalBorderStyles {
    pub(super) fn apply_edges(&mut self, edges: LogicalEdgeSelection, value: BorderStyle) {
        match edges {
            LogicalEdgeSelection::Block => {
                self.block_start = Some(value);
                self.block_end = Some(value);
            }
            LogicalEdgeSelection::Inline => {
                self.inline_start = Some(value);
                self.inline_end = Some(value);
            }
            LogicalEdgeSelection::BlockStart => self.block_start = Some(value),
            LogicalEdgeSelection::BlockEnd => self.block_end = Some(value),
            LogicalEdgeSelection::InlineStart => self.inline_start = Some(value),
            LogicalEdgeSelection::InlineEnd => self.inline_end = Some(value),
        }
    }

    pub(super) fn apply_axis_values(&mut self, axis: LogicalEdgeSelection, value: &str) {
        if let Some(value) = parse_border_style(value) {
            self.apply_edges(axis, value);
            return;
        }
        let values = value
            .split_whitespace()
            .filter_map(parse_border_style)
            .collect::<Vec<_>>();
        match (axis, values.as_slice()) {
            (_, []) => {}
            (LogicalEdgeSelection::Block, [both]) => {
                self.block_start = Some(*both);
                self.block_end = Some(*both);
            }
            (LogicalEdgeSelection::Block, [start, end, ..]) => {
                self.block_start = Some(*start);
                self.block_end = Some(*end);
            }
            (LogicalEdgeSelection::Inline, [both]) => {
                self.inline_start = Some(*both);
                self.inline_end = Some(*both);
            }
            (LogicalEdgeSelection::Inline, [start, end, ..]) => {
                self.inline_start = Some(*start);
                self.inline_end = Some(*end);
            }
            (_, [single, ..]) => self.apply_edges(axis, *single),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerRadius {
    pub horizontal: StyleLength,
    pub vertical: Option<StyleLength>,
}

impl CornerRadius {
    pub(super) fn circular(value: StyleLength) -> Self {
        Self {
            horizontal: value,
            vertical: None,
        }
    }

    pub(super) fn elliptical(horizontal: StyleLength, vertical: StyleLength) -> Self {
        Self {
            horizontal,
            vertical: Some(vertical),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CornerRadii {
    pub top_left: Option<CornerRadius>,
    pub top_right: Option<CornerRadius>,
    pub bottom_right: Option<CornerRadius>,
    pub bottom_left: Option<CornerRadius>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogicalCornerRadii {
    pub start_start: Option<CornerRadius>,
    pub start_end: Option<CornerRadius>,
    pub end_end: Option<CornerRadius>,
    pub end_start: Option<CornerRadius>,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum CornerSelection {
    All,
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

#[derive(Debug, Clone, Copy)]
pub(super) enum LogicalCornerSelection {
    Start,
    End,
    StartStart,
    StartEnd,
    EndEnd,
    EndStart,
}
