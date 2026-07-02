use super::*;

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

pub(super) fn normalize_css_value(value: &str) -> String {
    value.trim().to_string()
}

pub(super) fn parse_display(value: &str) -> Option<DisplayMode> {
    let value = value.trim().to_ascii_lowercase();
    match value.as_str() {
        "inline" => Some(DisplayMode::Inline),
        "inline-block" => Some(DisplayMode::InlineBlock),
        "flow" => Some(DisplayMode::Block),
        "flex" => Some(DisplayMode::Flex),
        "inline-flex" => Some(DisplayMode::InlineFlex),
        "block" => Some(DisplayMode::Block),
        "grid" => Some(DisplayMode::Grid),
        "inline-grid" => Some(DisplayMode::InlineGrid),
        "flow-root" => Some(DisplayMode::FlowRoot),
        "contents" => Some(DisplayMode::Contents),
        "list-item" => Some(DisplayMode::ListItem),
        "table" => Some(DisplayMode::Table),
        "inline-table" => Some(DisplayMode::InlineTable),
        "table-caption" => Some(DisplayMode::TableCaption),
        "table-cell" => Some(DisplayMode::TableCell),
        "table-column" => Some(DisplayMode::TableColumn),
        "table-column-group" => Some(DisplayMode::TableColumnGroup),
        "table-footer-group" => Some(DisplayMode::TableFooterGroup),
        "table-header-group" => Some(DisplayMode::TableHeaderGroup),
        "table-row-group" => Some(DisplayMode::TableRowGroup),
        "table-row" => Some(DisplayMode::TableRow),
        "ruby" => Some(DisplayMode::Ruby),
        "ruby-base" => Some(DisplayMode::RubyBase),
        "ruby-text" => Some(DisplayMode::RubyText),
        "ruby-base-container" => Some(DisplayMode::RubyBaseContainer),
        "ruby-text-container" => Some(DisplayMode::RubyTextContainer),
        "-webkit-box" => Some(DisplayMode::WebkitBox),
        "none" => Some(DisplayMode::None),
        _ => parse_multi_keyword_display(&value),
    }
}

pub(super) fn parse_multi_keyword_display(value: &str) -> Option<DisplayMode> {
    let tokens = value.split_ascii_whitespace();
    let mut outside = None;
    let mut inside = None;
    let mut list_item = false;
    let mut count = 0usize;

    for token in tokens {
        count += 1;
        match token {
            "block" | "inline" => {
                if outside.is_some() {
                    return None;
                }
                outside = Some(token);
            }
            "flow" | "flow-root" | "table" | "flex" | "grid" | "ruby" => {
                if inside.is_some() {
                    return None;
                }
                inside = Some(token);
            }
            "list-item" => {
                if list_item {
                    return None;
                }
                list_item = true;
            }
            _ => return None,
        }
    }

    if !(2..=3).contains(&count) {
        return None;
    }

    if list_item {
        return match (outside.unwrap_or("block"), inside.unwrap_or("flow")) {
            ("block", "flow") => Some(DisplayMode::ListItem),
            _ => None,
        };
    }

    let outside = outside?;
    let inside = inside.unwrap_or("flow");
    match (outside, inside) {
        ("block", "flow") => Some(DisplayMode::Block),
        ("inline", "flow") => Some(DisplayMode::Inline),
        ("block", "flow-root") => Some(DisplayMode::FlowRoot),
        ("inline", "flow-root") => Some(DisplayMode::InlineBlock),
        ("block", "table") => Some(DisplayMode::Table),
        ("inline", "table") => Some(DisplayMode::InlineTable),
        ("block", "flex") => Some(DisplayMode::Flex),
        ("inline", "flex") => Some(DisplayMode::InlineFlex),
        ("block", "grid") => Some(DisplayMode::Grid),
        ("inline", "grid") => Some(DisplayMode::InlineGrid),
        ("inline", "ruby") => Some(DisplayMode::Ruby),
        _ => None,
    }
}

pub(super) fn parse_box_sizing(value: &str) -> Option<BoxSizing> {
    match value.trim() {
        "border-box" => Some(BoxSizing::BorderBox),
        "content-box" => Some(BoxSizing::ContentBox),
        _ => None,
    }
}

pub(super) fn parse_box_decoration_break(value: &str) -> Option<BoxDecorationBreak> {
    match value.trim() {
        "slice" => Some(BoxDecorationBreak::Slice),
        "clone" => Some(BoxDecorationBreak::Clone),
        _ => None,
    }
}

pub(super) fn parse_position(value: &str) -> Option<PositionMode> {
    match value.trim() {
        "static" => Some(PositionMode::Static),
        "relative" => Some(PositionMode::Relative),
        "absolute" => Some(PositionMode::Absolute),
        "fixed" => Some(PositionMode::Fixed),
        "sticky" => Some(PositionMode::Sticky),
        _ => None,
    }
}

pub(super) fn parse_flex_direction(value: &str) -> Option<Orientation> {
    match value.trim() {
        "row" | "row-reverse" => Some(Orientation::Horizontal),
        "column" | "column-reverse" => Some(Orientation::Vertical),
        _ => None,
    }
}

pub(super) fn parse_flex_wrap(value: &str) -> Option<FlexWrap> {
    match value.trim() {
        "nowrap" => Some(FlexWrap::NoWrap),
        "wrap" => Some(FlexWrap::Wrap),
        "wrap-reverse" => Some(FlexWrap::WrapReverse),
        _ => None,
    }
}

pub(super) fn parse_grid_auto_flow(value: &str) -> Option<GridAutoFlow> {
    match value.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["row"] => Some(GridAutoFlow::Row),
        ["column"] => Some(GridAutoFlow::Column),
        ["dense"] => Some(GridAutoFlow::Dense),
        ["row", "dense"] | ["dense", "row"] => Some(GridAutoFlow::RowDense),
        ["column", "dense"] | ["dense", "column"] => Some(GridAutoFlow::ColumnDense),
        _ => None,
    }
}

pub(super) fn parse_container_type(value: &str) -> Option<ContainerType> {
    match value.trim() {
        "normal" => Some(ContainerType::Normal),
        "size" => Some(ContainerType::Size),
        "inline-size" => Some(ContainerType::InlineSize),
        _ => None,
    }
}

pub(super) fn parse_content_visibility(value: &str) -> Option<ContentVisibility> {
    match value.trim() {
        "visible" => Some(ContentVisibility::Visible),
        "auto" => Some(ContentVisibility::Auto),
        "hidden" => Some(ContentVisibility::Hidden),
        _ => None,
    }
}

pub(super) fn parse_align_items(value: &str) -> Option<AlignItems> {
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

pub(super) fn parse_justify_content(value: &str) -> Option<JustifyContent> {
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

pub(super) fn parse_self_alignment(value: &str) -> Option<SelfAlignment> {
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

pub(super) fn parse_border_style(value: &str) -> Option<BorderStyle> {
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

pub(super) fn parse_background_attachment(value: &str) -> Option<BackgroundAttachment> {
    match value.trim() {
        "fixed" => Some(BackgroundAttachment::Fixed),
        "local" => Some(BackgroundAttachment::Local),
        "scroll" => Some(BackgroundAttachment::Scroll),
        _ => None,
    }
}

pub(super) fn parse_background_box(value: &str) -> Option<BackgroundBox> {
    match value.trim() {
        "border-box" => Some(BackgroundBox::BorderBox),
        "padding-box" => Some(BackgroundBox::PaddingBox),
        "content-box" => Some(BackgroundBox::ContentBox),
        "text" => Some(BackgroundBox::Text),
        _ => None,
    }
}

pub(super) fn parse_object_fit(value: &str) -> Option<ObjectFit> {
    match value.trim() {
        "fill" => Some(ObjectFit::Fill),
        "contain" => Some(ObjectFit::Contain),
        "cover" => Some(ObjectFit::Cover),
        "none" => Some(ObjectFit::None),
        "scale-down" => Some(ObjectFit::ScaleDown),
        _ => None,
    }
}

pub(super) fn parse_list_style_position(value: &str) -> Option<ListStylePosition> {
    match value.trim() {
        "inside" => Some(ListStylePosition::Inside),
        "outside" => Some(ListStylePosition::Outside),
        _ => None,
    }
}

pub(super) fn parse_font_style(value: &str) -> Option<FontStyle> {
    match value.trim() {
        "normal" => Some(FontStyle::Normal),
        "italic" => Some(FontStyle::Italic),
        "oblique" => Some(FontStyle::Oblique),
        _ => None,
    }
}

pub(super) fn parse_font_weight(value: &str) -> Option<FontWeight> {
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

pub(super) fn parse_text_align(value: &str) -> Option<TextAlign> {
    match value.trim() {
        "left" | "start" => Some(TextAlign::Start),
        "center" => Some(TextAlign::Center),
        "right" | "end" => Some(TextAlign::End),
        "justify" => Some(TextAlign::Justify),
        _ => None,
    }
}

pub(super) fn parse_text_direction(value: &str) -> Option<TextDirection> {
    match value.trim() {
        "ltr" => Some(TextDirection::Ltr),
        "rtl" => Some(TextDirection::Rtl),
        _ => None,
    }
}

pub(super) fn parse_unicode_bidi(value: &str) -> Option<UnicodeBidi> {
    match value.trim() {
        "normal" => Some(UnicodeBidi::Normal),
        "embed" => Some(UnicodeBidi::Embed),
        "isolate" => Some(UnicodeBidi::Isolate),
        "bidi-override" => Some(UnicodeBidi::BidiOverride),
        "isolate-override" => Some(UnicodeBidi::IsolateOverride),
        "plaintext" => Some(UnicodeBidi::Plaintext),
        _ => None,
    }
}

pub(super) fn parse_writing_mode(value: &str) -> Option<WritingMode> {
    match value.trim() {
        "horizontal-tb" => Some(WritingMode::HorizontalTb),
        "vertical-rl" => Some(WritingMode::VerticalRl),
        "vertical-lr" => Some(WritingMode::VerticalLr),
        "sideways-rl" => Some(WritingMode::SidewaysRl),
        "sideways-lr" => Some(WritingMode::SidewaysLr),
        _ => None,
    }
}

pub(super) fn parse_text_orientation(value: &str) -> Option<TextOrientation> {
    match value.trim() {
        "mixed" => Some(TextOrientation::Mixed),
        "upright" => Some(TextOrientation::Upright),
        "sideways" => Some(TextOrientation::Sideways),
        "sideways-right" => Some(TextOrientation::SidewaysRight),
        "use-glyph-orientation" => Some(TextOrientation::UseGlyphOrientation),
        _ => None,
    }
}

pub(super) fn parse_text_transform(value: &str) -> Option<TextTransform> {
    match value.trim() {
        "none" => Some(TextTransform::None),
        "uppercase" => Some(TextTransform::Uppercase),
        "lowercase" => Some(TextTransform::Lowercase),
        "capitalize" => Some(TextTransform::Capitalize),
        _ => None,
    }
}

pub(super) fn parse_fill_rule(value: &str) -> Option<FillRule> {
    match value.trim() {
        "nonzero" => Some(FillRule::Nonzero),
        "evenodd" => Some(FillRule::Evenodd),
        _ => None,
    }
}

pub(super) fn parse_stroke_linecap(value: &str) -> Option<StrokeLineCap> {
    match value.trim() {
        "butt" => Some(StrokeLineCap::Butt),
        "round" => Some(StrokeLineCap::Round),
        "square" => Some(StrokeLineCap::Square),
        _ => None,
    }
}

pub(super) fn parse_stroke_linejoin(value: &str) -> Option<StrokeLineJoin> {
    match value.trim() {
        "arcs" => Some(StrokeLineJoin::Arcs),
        "bevel" => Some(StrokeLineJoin::Bevel),
        "miter" => Some(StrokeLineJoin::Miter),
        "miter-clip" => Some(StrokeLineJoin::MiterClip),
        "round" => Some(StrokeLineJoin::Round),
        _ => None,
    }
}

pub(super) fn parse_text_decoration_line(value: &str) -> Option<&str> {
    match value.trim() {
        "none" | "underline" | "overline" | "line-through" | "blink" => Some(value.trim()),
        _ => None,
    }
}

pub(super) fn parse_text_decoration_style(value: &str) -> Option<TextDecorationStyle> {
    match value.trim() {
        "solid" => Some(TextDecorationStyle::Solid),
        "double" => Some(TextDecorationStyle::Double),
        "dotted" => Some(TextDecorationStyle::Dotted),
        "dashed" => Some(TextDecorationStyle::Dashed),
        "wavy" => Some(TextDecorationStyle::Wavy),
        _ => None,
    }
}

pub(super) fn is_text_emphasis_style_token(value: &str) -> bool {
    matches!(
        value.trim(),
        "none" | "filled" | "open" | "dot" | "circle" | "double-circle" | "triangle" | "sesame"
    ) || value.trim().starts_with('"')
        || value.trim().starts_with('\'')
}

pub(super) fn parse_text_overflow(value: &str) -> Option<TextOverflow> {
    match value.trim() {
        "clip" => Some(TextOverflow::Clip),
        "ellipsis" => Some(TextOverflow::Ellipsis),
        _ => None,
    }
}

pub(super) fn parse_white_space(value: &str) -> Option<WhiteSpaceMode> {
    match value.trim() {
        "normal" => Some(WhiteSpaceMode::Normal),
        "nowrap" => Some(WhiteSpaceMode::NoWrap),
        "pre" => Some(WhiteSpaceMode::Pre),
        "pre-line" => Some(WhiteSpaceMode::PreLine),
        "pre-wrap" => Some(WhiteSpaceMode::PreWrap),
        "break-spaces" => Some(WhiteSpaceMode::BreakSpaces),
        _ => None,
    }
}

pub(super) fn parse_text_wrap(value: &str) -> Option<TextWrapMode> {
    match value.trim() {
        "wrap" => Some(TextWrapMode::Wrap),
        "nowrap" => Some(TextWrapMode::NoWrap),
        "balance" => Some(TextWrapMode::Balance),
        "pretty" => Some(TextWrapMode::Pretty),
        "stable" => Some(TextWrapMode::Stable),
        _ => None,
    }
}

pub(super) fn parse_word_break(value: &str) -> Option<WordBreakMode> {
    match value.trim() {
        "normal" => Some(WordBreakMode::Normal),
        "break-all" => Some(WordBreakMode::BreakAll),
        "keep-all" => Some(WordBreakMode::KeepAll),
        "break-word" => Some(WordBreakMode::BreakWord),
        _ => None,
    }
}

pub(super) fn parse_overflow_wrap(value: &str) -> Option<OverflowWrapMode> {
    match value.trim() {
        "normal" => Some(OverflowWrapMode::Normal),
        "break-word" => Some(OverflowWrapMode::BreakWord),
        "anywhere" => Some(OverflowWrapMode::Anywhere),
        _ => None,
    }
}

pub(super) fn parse_hyphens(value: &str) -> Option<HyphensMode> {
    match value.trim() {
        "none" => Some(HyphensMode::None),
        "manual" => Some(HyphensMode::Manual),
        "auto" => Some(HyphensMode::Auto),
        _ => None,
    }
}

pub(super) fn parse_overflow(value: &str) -> Option<OverflowMode> {
    match value.trim() {
        "visible" => Some(OverflowMode::Visible),
        "hidden" => Some(OverflowMode::Hidden),
        "scroll" => Some(OverflowMode::Scroll),
        "auto" => Some(OverflowMode::Auto),
        "clip" => Some(OverflowMode::Clip),
        _ => None,
    }
}

pub(super) fn parse_visibility(value: &str) -> Option<VisibilityMode> {
    match value.trim() {
        "visible" => Some(VisibilityMode::Visible),
        "hidden" => Some(VisibilityMode::Hidden),
        "collapse" => Some(VisibilityMode::Collapse),
        _ => None,
    }
}

pub(super) fn parse_isolation(value: &str) -> Option<IsolationMode> {
    match value.trim() {
        "auto" => Some(IsolationMode::Auto),
        "isolate" => Some(IsolationMode::Isolate),
        _ => None,
    }
}

pub(super) fn parse_blend_mode(value: &str) -> Option<BlendMode> {
    match value.trim() {
        "normal" => Some(BlendMode::Normal),
        "multiply" => Some(BlendMode::Multiply),
        "screen" => Some(BlendMode::Screen),
        "overlay" => Some(BlendMode::Overlay),
        "darken" => Some(BlendMode::Darken),
        "lighten" => Some(BlendMode::Lighten),
        "color-dodge" => Some(BlendMode::ColorDodge),
        "color-burn" => Some(BlendMode::ColorBurn),
        "hard-light" => Some(BlendMode::HardLight),
        "soft-light" => Some(BlendMode::SoftLight),
        "difference" => Some(BlendMode::Difference),
        "exclusion" => Some(BlendMode::Exclusion),
        "hue" => Some(BlendMode::Hue),
        "saturation" => Some(BlendMode::Saturation),
        "color" => Some(BlendMode::Color),
        "luminosity" => Some(BlendMode::Luminosity),
        "plus-darker" => Some(BlendMode::PlusDarker),
        "plus-lighter" => Some(BlendMode::PlusLighter),
        _ => None,
    }
}

pub(super) fn parse_float(value: &str) -> Option<FloatMode> {
    match value.trim() {
        "left" => Some(FloatMode::Left),
        "right" => Some(FloatMode::Right),
        "inline-start" => Some(FloatMode::InlineStart),
        "inline-end" => Some(FloatMode::InlineEnd),
        "footnote" => Some(FloatMode::Footnote),
        "none" => Some(FloatMode::None),
        _ => None,
    }
}

pub(super) fn parse_clear(value: &str) -> Option<ClearMode> {
    match value.trim() {
        "left" => Some(ClearMode::Left),
        "right" => Some(ClearMode::Right),
        "both" => Some(ClearMode::Both),
        "inline-start" => Some(ClearMode::InlineStart),
        "inline-end" => Some(ClearMode::InlineEnd),
        "none" => Some(ClearMode::None),
        _ => None,
    }
}

pub(super) fn parse_table_layout(value: &str) -> Option<TableLayout> {
    match value.trim() {
        "auto" => Some(TableLayout::Auto),
        "fixed" => Some(TableLayout::Fixed),
        _ => None,
    }
}

pub(super) fn parse_border_collapse(value: &str) -> Option<BorderCollapse> {
    match value.trim() {
        "collapse" => Some(BorderCollapse::Collapse),
        "separate" => Some(BorderCollapse::Separate),
        _ => None,
    }
}

pub(super) fn parse_caption_side(value: &str) -> Option<CaptionSide> {
    match value.trim() {
        "top" => Some(CaptionSide::Top),
        "bottom" => Some(CaptionSide::Bottom),
        _ => None,
    }
}

pub(super) fn parse_z_index(value: &str) -> Option<i32> {
    value.trim().parse::<i32>().ok()
}

pub(super) fn parse_opacity(value: &str) -> Option<f64> {
    let value = value.trim();
    if let Some(percent) = value.strip_suffix('%') {
        return Some((percent.trim().parse::<f64>().ok()? / 100.0).clamp(0.0, 1.0));
    }
    value.parse::<f64>().ok().map(|value| value.clamp(0.0, 1.0))
}

pub(super) fn parse_css_string_token(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub(super) fn parse_non_empty_css_string(value: &str) -> Option<String> {
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_string())
    }
}

pub(super) fn join_css_functions<const N: usize>(values: [Option<&str>; N]) -> Option<String> {
    let parts = values
        .into_iter()
        .flatten()
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" "))
    }
}

pub(super) fn is_tailwind_translate_pipeline(value: &str) -> bool {
    value.contains("var(--tw-translate-x)") || value.contains("var(--tw-translate-y)")
}

pub(super) fn is_tailwind_scale_pipeline(value: &str) -> bool {
    value.contains("var(--tw-scale-x)") || value.contains("var(--tw-scale-y)")
}

pub(super) fn is_tailwind_border_spacing_pipeline(value: &str) -> bool {
    value.contains("var(--tw-border-spacing-x)") || value.contains("var(--tw-border-spacing-y)")
}

pub(super) fn is_tailwind_transform_pipeline(value: &str) -> bool {
    value.contains("var(--tw-rotate-x)")
        || value.contains("var(--tw-rotate-y)")
        || value.contains("var(--tw-rotate-z)")
        || value.contains("var(--tw-skew-x)")
        || value.contains("var(--tw-skew-y)")
}

pub(super) fn is_tailwind_filter_pipeline(value: &str) -> bool {
    value.contains("var(--tw-blur)")
        || value.contains("var(--tw-brightness)")
        || value.contains("var(--tw-contrast)")
        || value.contains("var(--tw-drop-shadow)")
        || value.contains("var(--tw-grayscale)")
        || value.contains("var(--tw-hue-rotate)")
        || value.contains("var(--tw-invert)")
        || value.contains("var(--tw-saturate)")
        || value.contains("var(--tw-sepia)")
}

pub(super) fn is_tailwind_backdrop_filter_pipeline(value: &str) -> bool {
    value.contains("var(--tw-backdrop-blur)")
        || value.contains("var(--tw-backdrop-brightness)")
        || value.contains("var(--tw-backdrop-contrast)")
        || value.contains("var(--tw-backdrop-grayscale)")
        || value.contains("var(--tw-backdrop-hue-rotate)")
        || value.contains("var(--tw-backdrop-invert)")
        || value.contains("var(--tw-backdrop-opacity)")
        || value.contains("var(--tw-backdrop-saturate)")
        || value.contains("var(--tw-backdrop-sepia)")
}

pub(super) fn is_tailwind_font_variant_numeric_pipeline(value: &str) -> bool {
    value.contains("var(--tw-ordinal)")
        || value.contains("var(--tw-slashed-zero)")
        || value.contains("var(--tw-numeric-figure)")
        || value.contains("var(--tw-numeric-spacing)")
        || value.contains("var(--tw-numeric-fraction)")
}

pub(super) fn parse_pointer_events(value: &str) -> Option<PointerEvents> {
    match value.trim() {
        "auto" => Some(PointerEvents::Auto),
        "none" => Some(PointerEvents::None),
        "visiblePainted" | "visible-painted" => Some(PointerEvents::VisiblePainted),
        "visibleFill" | "visible-fill" => Some(PointerEvents::VisibleFill),
        "visibleStroke" | "visible-stroke" => Some(PointerEvents::VisibleStroke),
        "visible" => Some(PointerEvents::Visible),
        "painted" => Some(PointerEvents::Painted),
        "fill" => Some(PointerEvents::Fill),
        "stroke" => Some(PointerEvents::Stroke),
        "bounding-box" | "boundingBox" => Some(PointerEvents::BoundingBox),
        "all" => Some(PointerEvents::All),
        _ => None,
    }
}

pub(super) fn parse_user_select(value: &str) -> Option<UserSelect> {
    match value.trim() {
        "auto" => Some(UserSelect::Auto),
        "text" => Some(UserSelect::Text),
        "none" => Some(UserSelect::None),
        "all" => Some(UserSelect::All),
        "contain" => Some(UserSelect::Contain),
        _ => None,
    }
}

pub(super) fn parse_backface_visibility(value: &str) -> Option<BackfaceVisibility> {
    match value.trim() {
        "visible" => Some(BackfaceVisibility::Visible),
        "hidden" => Some(BackfaceVisibility::Hidden),
        _ => None,
    }
}

pub(super) fn parse_resize(value: &str) -> Option<ResizeMode> {
    match value.trim() {
        "none" => Some(ResizeMode::None),
        "both" => Some(ResizeMode::Both),
        "horizontal" => Some(ResizeMode::Horizontal),
        "vertical" => Some(ResizeMode::Vertical),
        "block" => Some(ResizeMode::Block),
        "inline" => Some(ResizeMode::Inline),
        _ => None,
    }
}

pub(super) fn parse_scroll_behavior(value: &str) -> Option<ScrollBehavior> {
    match value.trim() {
        "auto" => Some(ScrollBehavior::Auto),
        "smooth" => Some(ScrollBehavior::Smooth),
        _ => None,
    }
}

pub(super) fn parse_overscroll_behavior(value: &str) -> Option<OverscrollBehavior> {
    match value.trim() {
        "auto" => Some(OverscrollBehavior::Auto),
        "contain" => Some(OverscrollBehavior::Contain),
        "none" => Some(OverscrollBehavior::None),
        _ => None,
    }
}

pub(super) fn parse_edge_insets(value: &str) -> EdgeInsets {
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

pub(super) fn parse_edge_colors(value: &str) -> EdgeColors {
    let values = parse_edge_values(value, parse_color);
    let mut edges = EdgeColors::default();
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

pub(super) fn parse_edge_border_styles(value: &str) -> EdgeBorderStyles {
    let values = parse_edge_values(value, parse_border_style);
    let mut edges = EdgeBorderStyles::default();
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

pub(super) fn parse_edge_values<T>(value: &str, parser: impl Fn(&str) -> Option<T>) -> Vec<T> {
    if let Some(value) = parser(value) {
        return vec![value];
    }
    value.split_whitespace().filter_map(parser).collect()
}

pub(super) fn parse_corner_radius(value: &str) -> Option<CornerRadius> {
    if let Some(length) = parse_length(value) {
        return Some(CornerRadius::circular(length));
    }
    let values = value
        .split_whitespace()
        .filter_map(parse_length)
        .collect::<Vec<_>>();
    match values.as_slice() {
        [horizontal] => Some(CornerRadius::circular(horizontal.clone())),
        [horizontal, vertical, ..] => Some(CornerRadius::elliptical(
            horizontal.clone(),
            vertical.clone(),
        )),
        [] => None,
    }
}

pub(super) fn parse_corner_radii(value: &str) -> CornerRadii {
    let (horizontal, vertical) = value
        .split_once('/')
        .map_or((value, None), |(horizontal, vertical)| {
            (horizontal.trim(), Some(vertical.trim()))
        });
    let horizontal = parse_corner_radius_values(horizontal);
    let vertical = vertical.map(parse_corner_radius_values);
    let mut radii = CornerRadii::default();
    let Some(horizontal) = expand_corner_values(&horizontal) else {
        return radii;
    };
    let vertical = vertical
        .as_ref()
        .and_then(|values| expand_corner_values(values));
    radii.top_left = Some(make_corner_radius(
        &horizontal[0],
        vertical.as_ref().map(|r| &r[0]),
    ));
    radii.top_right = Some(make_corner_radius(
        &horizontal[1],
        vertical.as_ref().map(|r| &r[1]),
    ));
    radii.bottom_right = Some(make_corner_radius(
        &horizontal[2],
        vertical.as_ref().map(|r| &r[2]),
    ));
    radii.bottom_left = Some(make_corner_radius(
        &horizontal[3],
        vertical.as_ref().map(|r| &r[3]),
    ));
    radii
}

pub(super) fn parse_corner_radius_values(value: &str) -> Vec<StyleLength> {
    if let Some(length) = parse_length(value) {
        return vec![length];
    }
    value.split_whitespace().filter_map(parse_length).collect()
}

pub(super) fn expand_corner_values(values: &[StyleLength]) -> Option<[StyleLength; 4]> {
    match values {
        [all] => Some([all.clone(), all.clone(), all.clone(), all.clone()]),
        [top_left_bottom_right, top_right_bottom_left] => Some([
            top_left_bottom_right.clone(),
            top_right_bottom_left.clone(),
            top_left_bottom_right.clone(),
            top_right_bottom_left.clone(),
        ]),
        [top_left, top_right_bottom_left, bottom_right] => Some([
            top_left.clone(),
            top_right_bottom_left.clone(),
            bottom_right.clone(),
            top_right_bottom_left.clone(),
        ]),
        [top_left, top_right, bottom_right, bottom_left, ..] => Some([
            top_left.clone(),
            top_right.clone(),
            bottom_right.clone(),
            bottom_left.clone(),
        ]),
        [] => None,
    }
}

pub(super) fn make_corner_radius(
    horizontal: &StyleLength,
    vertical: Option<&StyleLength>,
) -> CornerRadius {
    CornerRadius {
        horizontal: horizontal.clone(),
        vertical: vertical.cloned(),
    }
}
