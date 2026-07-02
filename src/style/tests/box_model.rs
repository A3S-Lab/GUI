use super::support::*;

#[test]
fn parses_css_border_side_color_style_and_logical_width_tokens() {
    let web = WebProps::new()
        .style("borderColor", "#111 #222 #333 #444")
        .style("borderStyle", "solid dashed dotted double")
        .style("borderTop", "2px groove #ff0000")
        .style("borderInlineWidth", "3px")
        .style("borderBlockWidth", "4px 5px")
        .style("borderInlineStartColor", "currentColor")
        .style("borderBlockEndStyle", "hidden");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.border_width.top, Some(StyleLength::Points(2.0)));
    assert_eq!(style.border_width.left, Some(StyleLength::Points(3.0)));
    assert_eq!(style.border_width.right, Some(StyleLength::Points(3.0)));
    assert_eq!(
        style.logical_border_width.inline_start,
        Some(StyleLength::Points(3.0))
    );
    assert_eq!(
        style.logical_border_width.inline_end,
        Some(StyleLength::Points(3.0))
    );
    assert_eq!(
        style.logical_border_width.block_start,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_border_width.block_end,
        Some(StyleLength::Points(5.0))
    );
    assert_eq!(
        style.border_colors.top,
        Some(StyleColor::Rgba {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        })
    );
    assert_eq!(
        style.border_colors.right,
        Some(StyleColor::Rgba {
            red: 0x22,
            green: 0x22,
            blue: 0x22,
            alpha: 255,
        })
    );
    assert_eq!(
        style.logical_border_colors.inline_start,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.border_styles.top, Some(BorderStyle::Groove));
    assert_eq!(style.border_styles.right, Some(BorderStyle::Dashed));
    assert_eq!(style.border_styles.bottom, Some(BorderStyle::Dotted));
    assert_eq!(style.border_styles.left, Some(BorderStyle::Double));
    assert_eq!(
        style.logical_border_styles.block_end,
        Some(BorderStyle::Hidden)
    );
    assert_eq!(
        style
            .declarations
            .get("border-inline-start-color")
            .map(String::as_str),
        Some("currentColor")
    );
    assert!(!style.unsupported.contains_key("border-inline-start-color"));
    assert!(!style.unsupported.contains_key("border-block-end-style"));
}

#[test]
fn parses_css_border_image_properties() {
    let web = WebProps::new()
        .style("borderImage", "url(border.svg) 30 fill / 10px / 2px round")
        .style("borderImageSource", "linear-gradient(red, blue)")
        .style("borderImageSlice", "30 fill")
        .style("borderImageWidth", "10px")
        .style("borderImageOutset", "2")
        .style("borderImageRepeat", "round space");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.border_image.as_deref(),
        Some("url(border.svg) 30 fill / 10px / 2px round")
    );
    assert_eq!(
        style.border_image_source.as_deref(),
        Some("linear-gradient(red, blue)")
    );
    assert_eq!(style.border_image_slice.as_deref(), Some("30 fill"));
    assert_eq!(style.border_image_width.as_deref(), Some("10px"));
    assert_eq!(style.border_image_outset.as_deref(), Some("2"));
    assert_eq!(style.border_image_repeat.as_deref(), Some("round space"));
    assert_eq!(
        style.declarations.get("border-image").map(String::as_str),
        Some("url(border.svg) 30 fill / 10px / 2px round")
    );
    assert!(!style.unsupported.contains_key("border-image"));
    assert!(!style.unsupported.contains_key("border-image-source"));
    assert!(!style.unsupported.contains_key("border-image-slice"));
    assert!(!style.unsupported.contains_key("border-image-width"));
    assert!(!style.unsupported.contains_key("border-image-outset"));
    assert!(!style.unsupported.contains_key("border-image-repeat"));
}

#[test]
fn parses_tailwind_arbitrary_border_image_properties() {
    let web = WebProps::new().class_name(
        "[border-image:url(/border.svg)_30_fill_/_10px_/_2px_round] \
             [border-image-source:linear-gradient(red,_blue)] \
             [border-image-slice:30_fill] [border-image-width:10px] \
             [border-image-outset:2] [border-image-repeat:round_space] \
             hover:[border-image-repeat:space] focus:[border-image-slice:10_20_fill]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.border_image.as_deref(),
        Some("url(/border.svg) 30 fill / 10px / 2px round")
    );
    assert_eq!(
        style.border_image_source.as_deref(),
        Some("linear-gradient(red, blue)")
    );
    assert_eq!(style.border_image_slice.as_deref(), Some("30 fill"));
    assert_eq!(style.border_image_width.as_deref(), Some("10px"));
    assert_eq!(style.border_image_outset.as_deref(), Some("2"));
    assert_eq!(style.border_image_repeat.as_deref(), Some("round space"));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("border-image-repeat"))
            .map(String::as_str),
        Some("space")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("border-image-slice"))
            .map(String::as_str),
        Some("10 20 fill")
    );
    assert!(!style.unsupported.contains_key("border-image"));
}

#[test]
fn parses_tailwind_border_side_color_and_logical_width_utilities() {
    let web = WebProps::new().class_name(
        "border-x-2 border-s-4 border-x-blue-600 border-t-red-500 \
             border-e-[#663399]/50 border-bs-current border-dashed \
             md:border-s-green-500 hover:border-b-(--accent-border)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.border_width.left, Some(StyleLength::Points(2.0)));
    assert_eq!(style.border_width.right, Some(StyleLength::Points(2.0)));
    assert_eq!(
        style.logical_border_width.inline_start,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_border_width.inline_end,
        Some(StyleLength::Points(2.0))
    );
    assert_eq!(
        style.border_colors.top,
        Some(StyleColor::Keyword("red-500".to_string()))
    );
    assert_eq!(
        style.border_colors.left,
        Some(StyleColor::Keyword("blue-600".to_string()))
    );
    assert_eq!(
        style.border_colors.right,
        Some(StyleColor::Keyword("blue-600".to_string()))
    );
    assert_eq!(
        style.logical_border_colors.inline_end,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(
        style.logical_border_colors.block_start,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.border_style, Some(BorderStyle::Dashed));
    assert_eq!(style.border_styles.top, Some(BorderStyle::Dashed));
    assert_eq!(
        style
            .declarations
            .get("border-inline-width")
            .map(String::as_str),
        Some("2px")
    );
    assert_eq!(
        style
            .declarations
            .get("border-inline-start-width")
            .map(String::as_str),
        Some("4px")
    );
    assert_eq!(
        style
            .declarations
            .get("border-inline-end-color")
            .map(String::as_str),
        Some("rgba(102, 51, 153, 0.5)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("border-inline-start-color"))
            .map(String::as_str),
        Some("green-500")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("border-bottom-color"))
            .map(String::as_str),
        Some("var(--accent-border)")
    );
}

#[test]
fn parses_css_logical_edge_properties_into_portable_tokens() {
    let web = WebProps::new()
        .style("anchorName", "--trigger")
        .style("anchorScope", "--trigger")
        .style("positionAnchor", "--trigger")
        .style("positionArea", "bottom center")
        .style("positionTry", "flip-block")
        .style("positionTryFallbacks", "--top, --bottom")
        .style("positionTryOrder", "most-width")
        .style("positionTryOptions", "flip-inline")
        .style("positionVisibility", "anchors-visible")
        .style("insetBlock", "3px 4px")
        .style("insetInlineStart", "1rem")
        .style("insetInlineEnd", "2rem")
        .style("top", "anchor(bottom)")
        .style("width", "anchor-size(width)")
        .style("paddingInline", "10px")
        .style("paddingBlockEnd", "4px")
        .style("marginBlock", "1px 2px")
        .style("marginInlineStart", "auto")
        .style("marginTrim", "block")
        .style("scrollMarginBlockStart", "5px")
        .style("scrollMarginInline", "6px")
        .style("scrollPaddingBlock", "8px 9px")
        .style("scrollPaddingInlineEnd", "7px");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.anchor_name.as_deref(), Some("--trigger"));
    assert_eq!(style.anchor_scope.as_deref(), Some("--trigger"));
    assert_eq!(style.position_anchor.as_deref(), Some("--trigger"));
    assert_eq!(style.position_area.as_deref(), Some("bottom center"));
    assert_eq!(style.position_try.as_deref(), Some("flip-block"));
    assert_eq!(
        style.position_try_fallbacks.as_deref(),
        Some("--top, --bottom")
    );
    assert_eq!(style.position_try_order.as_deref(), Some("most-width"));
    assert_eq!(style.position_try_options.as_deref(), Some("flip-inline"));
    assert_eq!(
        style.position_visibility.as_deref(),
        Some("anchors-visible")
    );
    assert_eq!(
        style.inset.top,
        Some(StyleLength::Css("anchor(bottom)".to_string()))
    );
    assert_eq!(
        style.logical_inset.block_start,
        Some(StyleLength::Points(3.0))
    );
    assert_eq!(
        style.logical_inset.block_end,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_inset.inline_start,
        Some(StyleLength::Points(16.0))
    );
    assert_eq!(
        style.logical_inset.inline_end,
        Some(StyleLength::Points(32.0))
    );
    assert_eq!(
        style.logical_padding.inline_start,
        Some(StyleLength::Points(10.0))
    );
    assert_eq!(
        style.logical_padding.inline_end,
        Some(StyleLength::Points(10.0))
    );
    assert_eq!(
        style.logical_padding.block_end,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_margin.block_start,
        Some(StyleLength::Points(1.0))
    );
    assert_eq!(
        style.logical_margin.block_end,
        Some(StyleLength::Points(2.0))
    );
    assert_eq!(style.logical_margin.inline_start, Some(StyleLength::Auto));
    assert_eq!(style.margin_trim.as_deref(), Some("block"));
    assert_eq!(
        style.logical_scroll_margin.block_start,
        Some(StyleLength::Points(5.0))
    );
    assert_eq!(
        style.logical_scroll_margin.inline_start,
        Some(StyleLength::Points(6.0))
    );
    assert_eq!(
        style.logical_scroll_margin.inline_end,
        Some(StyleLength::Points(6.0))
    );
    assert_eq!(
        style.logical_scroll_padding.block_start,
        Some(StyleLength::Points(8.0))
    );
    assert_eq!(
        style.logical_scroll_padding.block_end,
        Some(StyleLength::Points(9.0))
    );
    assert_eq!(
        style.logical_scroll_padding.inline_end,
        Some(StyleLength::Points(7.0))
    );
    assert_eq!(
        style.width,
        Some(StyleLength::Css("anchor-size(width)".to_string()))
    );
    assert_eq!(style.padding.left, Some(StyleLength::Points(10.0)));
    assert_eq!(style.padding.right, Some(StyleLength::Points(10.0)));
    assert_eq!(
        style
            .declarations
            .get("margin-inline-start")
            .map(String::as_str),
        Some("auto")
    );
    assert!(!style.unsupported.contains_key("anchor-name"));
    assert!(!style.unsupported.contains_key("anchor-scope"));
    assert!(!style.unsupported.contains_key("position-anchor"));
    assert!(!style.unsupported.contains_key("position-area"));
    assert!(!style.unsupported.contains_key("position-try"));
    assert!(!style.unsupported.contains_key("position-try-fallbacks"));
    assert!(!style.unsupported.contains_key("position-try-order"));
    assert!(!style.unsupported.contains_key("position-try-options"));
    assert!(!style.unsupported.contains_key("position-visibility"));
    assert!(!style.unsupported.contains_key("inset-inline-start"));
    assert!(!style.unsupported.contains_key("padding-block-end"));
    assert!(!style.unsupported.contains_key("margin-trim"));
}

#[test]
fn parses_css_logical_size_properties_into_portable_tokens() {
    let web = WebProps::new()
        .style("inlineSize", "min-content")
        .style("blockSize", "calc(100% - 2rem)")
        .style("minInlineSize", "12rem")
        .style("minBlockSize", "10px")
        .style("maxInlineSize", "80vw")
        .style("maxBlockSize", "clamp(10rem, 50vh, 40rem)");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.inline_size,
        Some(StyleLength::Css("min-content".to_string()))
    );
    assert_eq!(
        style.block_size,
        Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
    );
    assert_eq!(style.min_inline_size, Some(StyleLength::Points(192.0)));
    assert_eq!(style.min_block_size, Some(StyleLength::Points(10.0)));
    assert_eq!(
        style.max_inline_size,
        Some(StyleLength::Css("80vw".to_string()))
    );
    assert_eq!(
        style.max_block_size,
        Some(StyleLength::Css("clamp(10rem, 50vh, 40rem)".to_string()))
    );
    assert!(!style.unsupported.contains_key("inline-size"));
    assert!(!style.unsupported.contains_key("max-block-size"));
}

#[test]
fn parses_tailwind_logical_spacing_and_inset_utilities() {
    let web = WebProps::new().class_name(
        "[anchor-name:--trigger] [anchor-scope:--trigger] \
             [position-anchor:--trigger] [position-area:bottom_center] \
             [position-try:flip-block] [position-try-fallbacks:--top,--bottom] \
             [position-try-order:most-width] [position-try-options:flip-inline] \
             [position-visibility:anchors-visible] \
             top-[anchor(bottom)] w-[anchor-size(width)] \
             start-4 end-[2rem] inset-bs-1 inset-be-(--footer) \
             ms-auto me-2 -mbs-1 pbs-3 pie-4 \
             [margin-trim:block] \
             scroll-ms-2 scroll-me-[10px] scroll-pbs-1 scroll-pe-(--snap) \
             md:start-8 hover:ms-[calc(1rem_+_2px)] \
             hover:[position-area:top_center] focus:[position-try:flip-inline] \
             active:[position-visibility:no-overflow] focus:[margin-trim:inline]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.anchor_name.as_deref(), Some("--trigger"));
    assert_eq!(style.anchor_scope.as_deref(), Some("--trigger"));
    assert_eq!(style.position_anchor.as_deref(), Some("--trigger"));
    assert_eq!(style.position_area.as_deref(), Some("bottom center"));
    assert_eq!(style.position_try.as_deref(), Some("flip-block"));
    assert_eq!(
        style.position_try_fallbacks.as_deref(),
        Some("--top,--bottom")
    );
    assert_eq!(style.position_try_order.as_deref(), Some("most-width"));
    assert_eq!(style.position_try_options.as_deref(), Some("flip-inline"));
    assert_eq!(
        style.position_visibility.as_deref(),
        Some("anchors-visible")
    );
    assert_eq!(
        style.inset.top,
        Some(StyleLength::Css("anchor(bottom)".to_string()))
    );
    assert_eq!(
        style.width,
        Some(StyleLength::Css("anchor-size(width)".to_string()))
    );
    assert_eq!(
        style.logical_inset.inline_start,
        Some(StyleLength::Points(16.0))
    );
    assert_eq!(
        style.logical_inset.inline_end,
        Some(StyleLength::Points(32.0))
    );
    assert_eq!(
        style.logical_inset.block_start,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_inset.block_end,
        Some(StyleLength::Css("var(--footer)".to_string()))
    );
    assert_eq!(style.logical_margin.inline_start, Some(StyleLength::Auto));
    assert_eq!(
        style.logical_margin.inline_end,
        Some(StyleLength::Points(8.0))
    );
    assert_eq!(
        style.logical_margin.block_start,
        Some(StyleLength::Points(-4.0))
    );
    assert_eq!(style.margin_trim.as_deref(), Some("block"));
    assert_eq!(
        style.logical_padding.block_start,
        Some(StyleLength::Points(12.0))
    );
    assert_eq!(
        style.logical_padding.inline_end,
        Some(StyleLength::Points(16.0))
    );
    assert_eq!(
        style.logical_scroll_margin.inline_start,
        Some(StyleLength::Points(8.0))
    );
    assert_eq!(
        style.logical_scroll_margin.inline_end,
        Some(StyleLength::Points(10.0))
    );
    assert_eq!(
        style.logical_scroll_padding.block_start,
        Some(StyleLength::Points(4.0))
    );
    assert_eq!(
        style.logical_scroll_padding.inline_end,
        Some(StyleLength::Css("var(--snap)".to_string()))
    );
    assert_eq!(
        style
            .declarations
            .get("inset-inline-start")
            .map(String::as_str),
        Some("16px")
    );
    assert_eq!(
        style
            .declarations
            .get("margin-block-start")
            .map(String::as_str),
        Some("-4px")
    );
    assert_eq!(
        style
            .declarations
            .get("scroll-padding-inline-end")
            .map(String::as_str),
        Some("var(--snap)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("inset-inline-start"))
            .map(String::as_str),
        Some("32px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("margin-inline-start"))
            .map(String::as_str),
        Some("calc(1rem + 2px)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("position-area"))
            .map(String::as_str),
        Some("top center")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("position-try"))
            .map(String::as_str),
        Some("flip-inline")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("position-visibility"))
            .map(String::as_str),
        Some("no-overflow")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("margin-trim"))
            .map(String::as_str),
        Some("inline")
    );
}

#[test]
fn parses_css_border_radius_corners_into_portable_tokens() {
    let web = WebProps::new()
        .style("borderRadius", "1px 2px 3px 4px / 5px 6px 7px 8px")
        .style("borderTopLeftRadius", "10px 20px")
        .style("borderStartStartRadius", "12px")
        .style("borderStartEndRadius", "13px")
        .style("borderEndEndRadius", "14px")
        .style("borderEndStartRadius", "15px");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.border_radii.top_left,
        Some(CornerRadius::elliptical(
            StyleLength::Points(10.0),
            StyleLength::Points(20.0)
        ))
    );
    assert_eq!(
        style.border_radii.top_right,
        Some(CornerRadius::elliptical(
            StyleLength::Points(2.0),
            StyleLength::Points(6.0)
        ))
    );
    assert_eq!(
        style.border_radii.bottom_right,
        Some(CornerRadius::elliptical(
            StyleLength::Points(3.0),
            StyleLength::Points(7.0)
        ))
    );
    assert_eq!(
        style.border_radii.bottom_left,
        Some(CornerRadius::elliptical(
            StyleLength::Points(4.0),
            StyleLength::Points(8.0)
        ))
    );
    assert_eq!(
        style.logical_border_radii.start_start,
        Some(CornerRadius::circular(StyleLength::Points(12.0)))
    );
    assert_eq!(
        style.logical_border_radii.start_end,
        Some(CornerRadius::circular(StyleLength::Points(13.0)))
    );
    assert_eq!(
        style.logical_border_radii.end_end,
        Some(CornerRadius::circular(StyleLength::Points(14.0)))
    );
    assert_eq!(
        style.logical_border_radii.end_start,
        Some(CornerRadius::circular(StyleLength::Points(15.0)))
    );
    assert_eq!(
        style
            .declarations
            .get("border-top-left-radius")
            .map(String::as_str),
        Some("10px 20px")
    );
    assert!(!style.unsupported.contains_key("border-top-left-radius"));
    assert!(!style.unsupported.contains_key("border-start-start-radius"));
}

#[test]
fn parses_tailwind_border_radius_corner_utilities() {
    let web = WebProps::new().class_name(
        "rounded-full rounded-t-lg rounded-br-[2rem] rounded-s-sm \
             rounded-ee-(--radius-end) md:rounded-ss-xl \
             hover:rounded-bl-[calc(1rem_+_2px)]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.border_radius,
        Some(StyleLength::Css("calc(infinity * 1px)".to_string()))
    );
    assert_eq!(
        style.border_radii.top_left,
        Some(CornerRadius::circular(StyleLength::Points(8.0)))
    );
    assert_eq!(
        style.border_radii.top_right,
        Some(CornerRadius::circular(StyleLength::Points(8.0)))
    );
    assert_eq!(
        style.border_radii.bottom_right,
        Some(CornerRadius::circular(StyleLength::Points(32.0)))
    );
    assert_eq!(
        style.logical_border_radii.start_start,
        Some(CornerRadius::circular(StyleLength::Points(4.0)))
    );
    assert_eq!(
        style.logical_border_radii.end_start,
        Some(CornerRadius::circular(StyleLength::Points(4.0)))
    );
    assert_eq!(
        style.logical_border_radii.end_end,
        Some(CornerRadius::circular(StyleLength::Css(
            "var(--radius-end)".to_string()
        )))
    );
    assert_eq!(
        style
            .declarations
            .get("border-bottom-right-radius")
            .map(String::as_str),
        Some("32px")
    );
    assert_eq!(
        style
            .declarations
            .get("border-end-end-radius")
            .map(String::as_str),
        Some("var(--radius-end)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("border-start-start-radius"))
            .map(String::as_str),
        Some("12px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("border-bottom-left-radius"))
            .map(String::as_str),
        Some("calc(1rem + 2px)")
    );
}

#[test]
fn parses_tailwind_size_and_child_spacing_utilities() {
    let web = WebProps::new().class_name(
        "size-10 space-x-4 -space-y-[2px] space-y-reverse \
             md:size-[calc(100%_-_2rem)] hover:space-x-(--cluster-gap)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.width, Some(StyleLength::Points(40.0)));
    assert_eq!(style.height, Some(StyleLength::Points(40.0)));
    assert_eq!(style.space_x, Some(StyleLength::Points(16.0)));
    assert_eq!(style.space_y, Some(StyleLength::Points(-2.0)));
    assert_eq!(style.space_x_reverse.as_deref(), Some("0"));
    assert_eq!(style.space_y_reverse.as_deref(), Some("1"));
    assert_eq!(
        style.declarations.get("width").map(String::as_str),
        Some("40px")
    );
    assert_eq!(
        style.declarations.get("height").map(String::as_str),
        Some("40px")
    );
    assert_eq!(
        style.declarations.get("space-x").map(String::as_str),
        Some("16px")
    );
    assert_eq!(
        style.declarations.get("space-y").map(String::as_str),
        Some("-2px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("width"))
            .map(String::as_str),
        Some("calc(100% - 2rem)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("height"))
            .map(String::as_str),
        Some("calc(100% - 2rem)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("space-x"))
            .map(String::as_str),
        Some("var(--cluster-gap)")
    );
}

#[test]
fn parses_tailwind_ring_and_divide_utilities() {
    let web = WebProps::new().class_name(
        "ring-inset ring-2 ring-blue-500/50 inset-ring inset-ring-[#663399]/50 \
             divide-x-2 divide-y-[3px] divide-y-reverse divide-dashed divide-[#663399]/50 \
             hover:ring-[3px] focus:divide-solid active:inset-ring-(--inner-ring)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.ring_shadow.as_deref(), Some("0 0 0 2px"));
    assert_eq!(style.ring_color.as_deref(), Some("blue-500 / 50%"));
    assert_eq!(style.inset_ring_shadow.as_deref(), Some("inset 0 0 0 1px"));
    assert_eq!(
        style.inset_ring_color.as_deref(),
        Some("rgba(102, 51, 153, 0.5)")
    );
    assert_eq!(
        style.box_shadow.as_deref(),
        Some("inset 0 0 0 1px rgba(102, 51, 153, 0.5), inset 0 0 0 2px blue-500 / 50%")
    );
    assert_eq!(style.divide_x_width, Some(StyleLength::Points(2.0)));
    assert_eq!(style.divide_y_width, Some(StyleLength::Points(3.0)));
    assert_eq!(style.divide_x_reverse.as_deref(), Some("0"));
    assert_eq!(style.divide_y_reverse.as_deref(), Some("1"));
    assert_eq!(
        style.divide_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(style.divide_style, Some(BorderStyle::Dashed));
    assert_eq!(
        style
            .custom_properties
            .get("--tw-ring-inset")
            .map(String::as_str),
        Some("inset")
    );
    assert_eq!(
        style.declarations.get("divide-x-width").map(String::as_str),
        Some("2px")
    );
    assert_eq!(
        style.declarations.get("divide-y-width").map(String::as_str),
        Some("3px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("--tw-ring-shadow"))
            .map(String::as_str),
        Some("0 0 0 3px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("divide-style"))
            .map(String::as_str),
        Some("solid")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("--tw-inset-ring-shadow"))
            .map(String::as_str),
        Some("inset 0 0 0 var(--inner-ring)")
    );
}
