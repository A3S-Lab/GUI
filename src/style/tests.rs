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
        .style("clip", "rect(0, 0, 0, 0)")
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
    assert_eq!(style.clip.as_deref(), Some("rect(0, 0, 0, 0)"));
    assert_eq!(style.box_shadow.as_deref(), Some("0 1px 3px black"));
    assert!(!style.unsupported.contains_key("clip"));
    assert!(!style.unsupported.contains_key("box-shadow"));
}

#[test]
fn parses_extended_css_display_modes() {
    let cases = [
        ("FLOW", DisplayMode::Block),
        ("inline-block", DisplayMode::InlineBlock),
        ("inline-flex", DisplayMode::InlineFlex),
        ("flow-root", DisplayMode::FlowRoot),
        ("contents", DisplayMode::Contents),
        ("list-item", DisplayMode::ListItem),
        ("table", DisplayMode::Table),
        ("inline-table", DisplayMode::InlineTable),
        ("table-caption", DisplayMode::TableCaption),
        ("table-cell", DisplayMode::TableCell),
        ("table-column", DisplayMode::TableColumn),
        ("table-column-group", DisplayMode::TableColumnGroup),
        ("table-footer-group", DisplayMode::TableFooterGroup),
        ("table-header-group", DisplayMode::TableHeaderGroup),
        ("table-row-group", DisplayMode::TableRowGroup),
        ("table-row", DisplayMode::TableRow),
        ("ruby", DisplayMode::Ruby),
        ("ruby-base", DisplayMode::RubyBase),
        ("ruby-text", DisplayMode::RubyText),
        ("ruby-base-container", DisplayMode::RubyBaseContainer),
        ("ruby-text-container", DisplayMode::RubyTextContainer),
    ];

    for (display, expected) in cases {
        let style = PortableStyle::from_web(&WebProps::new().style("display", display));
        assert_eq!(style.display, Some(expected));
        assert!(!style.unsupported.contains_key("display"));
    }
}

#[test]
fn parses_css_display_multi_keyword_modes() {
    let cases = [
        ("block flow", DisplayMode::Block),
        ("inline flow", DisplayMode::Inline),
        ("block flow-root", DisplayMode::FlowRoot),
        ("flow-root inline", DisplayMode::InlineBlock),
        ("block flex", DisplayMode::Flex),
        ("inline flex", DisplayMode::InlineFlex),
        ("flex inline", DisplayMode::InlineFlex),
        ("block grid", DisplayMode::Grid),
        ("inline grid", DisplayMode::InlineGrid),
        ("block table", DisplayMode::Table),
        ("inline table", DisplayMode::InlineTable),
        ("inline ruby", DisplayMode::Ruby),
        ("block list-item", DisplayMode::ListItem),
        ("flow list-item", DisplayMode::ListItem),
    ];

    for (display, expected) in cases {
        let style = PortableStyle::from_web(&WebProps::new().style("display", display));
        assert_eq!(style.display, Some(expected), "{display}");
        assert_eq!(
            style.declarations.get("display").map(String::as_str),
            Some(display)
        );
        assert!(!style.unsupported.contains_key("display"));
    }

    let unsupported = PortableStyle::from_web(&WebProps::new().style("display", "block ruby"));
    assert_eq!(unsupported.display, None);
    assert_eq!(
        unsupported.declarations.get("display").map(String::as_str),
        Some("block ruby")
    );
    assert!(!unsupported.unsupported.contains_key("display"));
}

#[test]
fn parses_css_ruby_layout_properties() {
    let web = WebProps::new()
        .style("display", "ruby")
        .style("rubyAlign", "space-around")
        .style("rubyPosition", "under")
        .style("rubyMerge", "collapse")
        .style("rubyOverhang", "auto");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::Ruby));
    assert_eq!(style.ruby_align.as_deref(), Some("space-around"));
    assert_eq!(style.ruby_position.as_deref(), Some("under"));
    assert_eq!(style.ruby_merge.as_deref(), Some("collapse"));
    assert_eq!(style.ruby_overhang.as_deref(), Some("auto"));
    assert!(!style.unsupported.contains_key("ruby-align"));
    assert!(!style.unsupported.contains_key("ruby-position"));
    assert!(!style.unsupported.contains_key("ruby-merge"));
    assert!(!style.unsupported.contains_key("ruby-overhang"));
}

#[test]
fn parses_tailwind_arbitrary_ruby_layout_properties() {
    let web = WebProps::new().class_name(
        "[display:ruby] [ruby-align:space-around] [ruby-position:over] \
             [ruby-merge:collapse] [ruby-overhang:auto] \
             motion-safe:[display:ruby-text] hover:[ruby-position:under] \
             focus:[ruby-align:center] active:[ruby-merge:separate] \
             rtl:[ruby-overhang:none]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::Ruby));
    assert_eq!(style.ruby_align.as_deref(), Some("space-around"));
    assert_eq!(style.ruby_position.as_deref(), Some("over"));
    assert_eq!(style.ruby_merge.as_deref(), Some("collapse"));
    assert_eq!(style.ruby_overhang.as_deref(), Some("auto"));
    assert_eq!(
        style
            .variant_declarations
            .get("motion-safe")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("ruby-text")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("ruby-position"))
            .map(String::as_str),
        Some("under")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("ruby-align"))
            .map(String::as_str),
        Some("center")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("ruby-merge"))
            .map(String::as_str),
        Some("separate")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("rtl")
            .and_then(|styles| styles.get("ruby-overhang"))
            .map(String::as_str),
        Some("none")
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
fn applies_tailwind_important_utilities_after_normal_utilities() {
    let web = WebProps::new().class_name(
        "!mt-4 mt-2 \
             hover:![background-color:red] hover:[background-color:blue] \
             ![color:red] [color:blue]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.margin.top, Some(StyleLength::Points(16.0)));
    assert_eq!(
        style.declarations.get("margin-top").map(String::as_str),
        Some("16px")
    );
    assert_eq!(
        style.declarations.get("color").map(String::as_str),
        Some("red")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("background-color"))
            .map(String::as_str),
        Some("red")
    );
}

#[test]
fn decodes_tailwind_arbitrary_variant_keys() {
    let web = WebProps::new().class_name(
        "[&_p]:mt-4 group-[.is-open_&]:block \
             [@media(width_>=_48rem)]:grid [&_.nav\\_item]:text-white",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style
            .variant_declarations
            .get("[& p]")
            .and_then(|styles| styles.get("margin-top"))
            .map(String::as_str),
        Some("16px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("group-[.is-open &]")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("block")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("[@media(width >= 48rem)]")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("grid")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("[& .nav_item]")
            .and_then(|styles| styles.get("color"))
            .map(String::as_str),
        Some("rgb(255, 255, 255)")
    );
}

#[test]
fn parses_tailwind_display_utilities() {
    let web = WebProps::new().class_name(
        "inline-block hover:inline-flex focus:flow-root active:contents disabled:list-item \
             sm:table md:inline-table lg:table-caption xl:table-cell \
             2xl:table-column dark:table-column-group rtl:table-footer-group \
             ltr:table-header-group portrait:table-row-group landscape:table-row",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::InlineBlock));
    assert_eq!(
        style.declarations.get("display").map(String::as_str),
        Some("inline-block")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("inline-flex")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("flow-root")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("contents")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("disabled")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("list-item")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("sm")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("inline-table")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-caption")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("xl")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-cell")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("2xl")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-column")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("dark")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-column-group")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("rtl")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-footer-group")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("ltr")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-header-group")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("portrait")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-row-group")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("landscape")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("table-row")
    );
}

#[test]
fn parses_tailwind_arbitrary_multi_keyword_display_properties() {
    let web = WebProps::new().class_name(
        "[display:inline_flex] hover:[display:block_grid] \
             focus:[display:inline_flow-root] active:[display:block_list-item]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.display, Some(DisplayMode::InlineFlex));
    assert_eq!(
        style.declarations.get("display").map(String::as_str),
        Some("inline flex")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("block grid")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("inline flow-root")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("block list-item")
    );
    assert!(!style.unsupported.contains_key("display"));
}

#[test]
fn parses_tailwind_screen_reader_utilities() {
    let web = WebProps::new().class_name("sr-only focus:not-sr-only");
    let style = PortableStyle::from_web(&web);

    assert_eq!(style.position, Some(PositionMode::Absolute));
    assert_eq!(style.width, Some(StyleLength::Points(1.0)));
    assert_eq!(style.height, Some(StyleLength::Points(1.0)));
    assert_eq!(style.padding.top, Some(StyleLength::Points(0.0)));
    assert_eq!(style.padding.right, Some(StyleLength::Points(0.0)));
    assert_eq!(style.padding.bottom, Some(StyleLength::Points(0.0)));
    assert_eq!(style.padding.left, Some(StyleLength::Points(0.0)));
    assert_eq!(style.margin.top, Some(StyleLength::Points(-1.0)));
    assert_eq!(style.margin.right, Some(StyleLength::Points(-1.0)));
    assert_eq!(style.margin.bottom, Some(StyleLength::Points(-1.0)));
    assert_eq!(style.margin.left, Some(StyleLength::Points(-1.0)));
    assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
    assert_eq!(style.clip.as_deref(), Some("rect(0, 0, 0, 0)"));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::NoWrap));
    assert_eq!(style.border_width.top, Some(StyleLength::Points(0.0)));
    assert_eq!(style.border_width.right, Some(StyleLength::Points(0.0)));
    assert_eq!(style.border_width.bottom, Some(StyleLength::Points(0.0)));
    assert_eq!(style.border_width.left, Some(StyleLength::Points(0.0)));
    assert_eq!(
        style.declarations.get("clip").map(String::as_str),
        Some("rect(0, 0, 0, 0)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("position"))
            .map(String::as_str),
        Some("static")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("width"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("height"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("clip"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("white-space"))
            .map(String::as_str),
        Some("normal")
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
            .get("border-inline-width")
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
fn parses_css_cascade_global_reset_property() {
    let web = WebProps::new().style("all", "revert-layer");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.all.as_deref(), Some("revert-layer"));
    assert_eq!(
        style.declarations.get("all").map(String::as_str),
        Some("revert-layer")
    );
    assert!(!style.unsupported.contains_key("all"));
}

#[test]
fn parses_tailwind_cascade_global_reset_property() {
    let web = WebProps::new().class_name("[all:unset] hover:[all:revert-layer]");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.all.as_deref(), Some("unset"));
    assert_eq!(
        style.declarations.get("all").map(String::as_str),
        Some("unset")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("all"))
            .map(String::as_str),
        Some("revert-layer")
    );
}

#[test]
fn parses_css_formatting_and_table_layout_properties() {
    let web = WebProps::new()
        .style("boxSizing", "border-box")
        .style("boxDecorationBreak", "clone")
        .style("isolation", "isolate")
        .style("float", "inline-start")
        .style("clear", "both")
        .style("verticalAlign", "text-top")
        .style("tableLayout", "fixed")
        .style("borderCollapse", "separate")
        .style("borderSpacing", "4px 8px")
        .style("captionSide", "bottom")
        .style("emptyCells", "hide");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.box_sizing, Some(BoxSizing::BorderBox));
    assert_eq!(style.box_decoration_break, Some(BoxDecorationBreak::Clone));
    assert_eq!(style.isolation, Some(IsolationMode::Isolate));
    assert_eq!(style.float, Some(FloatMode::InlineStart));
    assert_eq!(style.clear, Some(ClearMode::Both));
    assert_eq!(style.vertical_align.as_deref(), Some("text-top"));
    assert_eq!(style.table_layout, Some(TableLayout::Fixed));
    assert_eq!(style.border_collapse, Some(BorderCollapse::Separate));
    assert_eq!(style.border_spacing.as_deref(), Some("4px 8px"));
    assert_eq!(style.caption_side, Some(CaptionSide::Bottom));
    assert_eq!(style.empty_cells.as_deref(), Some("hide"));
    assert!(!style.unsupported.contains_key("box-sizing"));
    assert!(!style.unsupported.contains_key("table-layout"));
    assert!(!style.unsupported.contains_key("empty-cells"));
}

#[test]
fn parses_tailwind_formatting_and_table_layout_utilities() {
    let web = WebProps::new().class_name(
        "box-border box-decoration-clone isolate float-start clear-both \
             align-text-bottom table-fixed border-separate border-spacing-x-2 \
             border-spacing-y-4 caption-bottom [empty-cells:show] \
             hover:align-[4px] md:table-auto focus:[empty-cells:hide]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.box_sizing, Some(BoxSizing::BorderBox));
    assert_eq!(style.box_decoration_break, Some(BoxDecorationBreak::Clone));
    assert_eq!(style.isolation, Some(IsolationMode::Isolate));
    assert_eq!(style.float, Some(FloatMode::InlineStart));
    assert_eq!(style.clear, Some(ClearMode::Both));
    assert_eq!(style.vertical_align.as_deref(), Some("text-bottom"));
    assert_eq!(style.table_layout, Some(TableLayout::Fixed));
    assert_eq!(style.border_collapse, Some(BorderCollapse::Separate));
    assert_eq!(style.border_spacing.as_deref(), Some("8px 16px"));
    assert_eq!(style.caption_side, Some(CaptionSide::Bottom));
    assert_eq!(style.empty_cells.as_deref(), Some("show"));
    assert_eq!(
        style
            .custom_properties
            .get("--tw-border-spacing-x")
            .map(String::as_str),
        Some("8px")
    );
    assert_eq!(
        style
            .custom_properties
            .get("--tw-border-spacing-y")
            .map(String::as_str),
        Some("16px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("vertical-align"))
            .map(String::as_str),
        Some("4px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("table-layout"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("empty-cells"))
            .map(String::as_str),
        Some("hide")
    );
}

#[test]
fn parses_css_blend_mode_properties() {
    let web = WebProps::new()
        .style("mixBlendMode", "plus-lighter")
        .style("backgroundBlendMode", "multiply, screen");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.mix_blend_mode, Some(BlendMode::PlusLighter));
    assert_eq!(
        style.background_blend_mode.as_deref(),
        Some("multiply, screen")
    );
    assert_eq!(
        style.declarations.get("mix-blend-mode").map(String::as_str),
        Some("plus-lighter")
    );
    assert_eq!(
        style
            .declarations
            .get("background-blend-mode")
            .map(String::as_str),
        Some("multiply, screen")
    );
    assert!(!style.unsupported.contains_key("mix-blend-mode"));
    assert!(!style.unsupported.contains_key("background-blend-mode"));
}

#[test]
fn parses_tailwind_blend_mode_utilities() {
    let web = WebProps::new().class_name(
        "mix-blend-color-dodge bg-blend-overlay hover:mix-blend-plus-lighter \
             md:bg-blend-[multiply,_screen] focus:mix-blend-(--blend-mode)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.mix_blend_mode, Some(BlendMode::ColorDodge));
    assert_eq!(style.background_blend_mode.as_deref(), Some("overlay"));
    assert_eq!(
        style.declarations.get("mix-blend-mode").map(String::as_str),
        Some("color-dodge")
    );
    assert_eq!(
        style
            .declarations
            .get("background-blend-mode")
            .map(String::as_str),
        Some("overlay")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("mix-blend-mode"))
            .map(String::as_str),
        Some("plus-lighter")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("background-blend-mode"))
            .map(String::as_str),
        Some("multiply, screen")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("mix-blend-mode"))
            .map(String::as_str),
        Some("var(--blend-mode)")
    );
}

#[test]
fn parses_css_masking_properties() {
    let web = WebProps::new()
        .style("clipPath", "circle(50% at center)")
        .style("mask", "url(mask.svg) center / contain no-repeat")
        .style("-webkitMaskImage", "linear-gradient(black, transparent)")
        .style("maskMode", "luminance")
        .style("maskRepeat", "no-repeat")
        .style("maskPosition", "center")
        .style("maskSize", "cover")
        .style("maskOrigin", "border-box")
        .style("maskClip", "content-box")
        .style("maskComposite", "exclude")
        .style("maskType", "alpha")
        .style("maskBorder", "url(border.svg) 30 fill / 10px")
        .style("maskBorderSource", "url(border.svg)")
        .style("maskBorderMode", "luminance")
        .style("maskBorderSlice", "30 fill")
        .style("maskBorderWidth", "10px")
        .style("maskBorderOutset", "2px")
        .style("maskBorderRepeat", "round");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.clip_path.as_deref(), Some("circle(50% at center)"));
    assert_eq!(
        style.mask.as_deref(),
        Some("url(mask.svg) center / contain no-repeat")
    );
    assert_eq!(
        style.mask_image.as_deref(),
        Some("linear-gradient(black, transparent)")
    );
    assert_eq!(style.mask_mode.as_deref(), Some("luminance"));
    assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(style.mask_position.as_deref(), Some("center"));
    assert_eq!(style.mask_size.as_deref(), Some("cover"));
    assert_eq!(style.mask_origin.as_deref(), Some("border-box"));
    assert_eq!(style.mask_clip.as_deref(), Some("content-box"));
    assert_eq!(style.mask_composite.as_deref(), Some("exclude"));
    assert_eq!(style.mask_type.as_deref(), Some("alpha"));
    assert_eq!(
        style.mask_border.as_deref(),
        Some("url(border.svg) 30 fill / 10px")
    );
    assert_eq!(style.mask_border_source.as_deref(), Some("url(border.svg)"));
    assert_eq!(style.mask_border_mode.as_deref(), Some("luminance"));
    assert_eq!(style.mask_border_slice.as_deref(), Some("30 fill"));
    assert_eq!(style.mask_border_width.as_deref(), Some("10px"));
    assert_eq!(style.mask_border_outset.as_deref(), Some("2px"));
    assert_eq!(style.mask_border_repeat.as_deref(), Some("round"));
    assert!(!style.unsupported.contains_key("clip-path"));
    assert!(!style.unsupported.contains_key("-webkit-mask-image"));
    assert!(!style.unsupported.contains_key("mask-border"));
}

#[test]
fn parses_tailwind_mask_utilities() {
    let web = WebProps::new().class_name(
        "mask-[url(/mask.svg)] mask-cover mask-no-repeat mask-center \
             mask-origin-content mask-clip-padding mask-add mask-alpha \
             mask-type-luminance hover:mask-size-[50%_50%] \
             md:mask-[position:30%_50%,70%_50%] focus:mask-(--mask-image)",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.mask_image.as_deref(), Some("url(/mask.svg)"));
    assert_eq!(style.mask_size.as_deref(), Some("cover"));
    assert_eq!(style.mask_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(style.mask_position.as_deref(), Some("center"));
    assert_eq!(style.mask_origin.as_deref(), Some("content-box"));
    assert_eq!(style.mask_clip.as_deref(), Some("padding-box"));
    assert_eq!(style.mask_composite.as_deref(), Some("add"));
    assert_eq!(style.mask_mode.as_deref(), Some("alpha"));
    assert_eq!(style.mask_type.as_deref(), Some("luminance"));
    assert_eq!(
        style.declarations.get("mask-image").map(String::as_str),
        Some("url(/mask.svg)")
    );
    assert_eq!(
        style.declarations.get("mask-composite").map(String::as_str),
        Some("add")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("mask-size"))
            .map(String::as_str),
        Some("50% 50%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("mask-position"))
            .map(String::as_str),
        Some("30% 50%,70% 50%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("mask-image"))
            .map(String::as_str),
        Some("var(--mask-image)")
    );
}

#[test]
fn parses_css_containment_and_container_properties() {
    let web = WebProps::new()
        .style("contain", "layout paint")
        .style("container", "sidebar / inline-size")
        .style("containerType", "size")
        .style("containerName", "main")
        .style("content", "\"*\"")
        .style("counterReset", "section 0")
        .style("counterIncrement", "section 1")
        .style("counterSet", "chapter 2")
        .style("quotes", "\"\\201C\" \"\\201D\" \"\\2018\" \"\\2019\"")
        .style("stringSet", "chapter content(text)")
        .style("contentVisibility", "auto")
        .style("containIntrinsicSize", "auto 320px")
        .style("containIntrinsicWidth", "240px")
        .style("containIntrinsicHeight", "120px")
        .style("containIntrinsicInlineSize", "50vw")
        .style("containIntrinsicBlockSize", "25vh");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.contain.as_deref(), Some("layout paint"));
    assert_eq!(style.container.as_deref(), Some("sidebar / inline-size"));
    assert_eq!(style.container_type, Some(ContainerType::Size));
    assert_eq!(style.container_name.as_deref(), Some("main"));
    assert_eq!(style.content.as_deref(), Some("\"*\""));
    assert_eq!(style.counter_reset.as_deref(), Some("section 0"));
    assert_eq!(style.counter_increment.as_deref(), Some("section 1"));
    assert_eq!(style.counter_set.as_deref(), Some("chapter 2"));
    assert_eq!(
        style.quotes.as_deref(),
        Some("\"\\201C\" \"\\201D\" \"\\2018\" \"\\2019\"")
    );
    assert_eq!(style.string_set.as_deref(), Some("chapter content(text)"));
    assert_eq!(style.content_visibility, Some(ContentVisibility::Auto));
    assert_eq!(style.contain_intrinsic_size.as_deref(), Some("auto 320px"));
    assert_eq!(style.contain_intrinsic_width.as_deref(), Some("240px"));
    assert_eq!(style.contain_intrinsic_height.as_deref(), Some("120px"));
    assert_eq!(style.contain_intrinsic_inline_size.as_deref(), Some("50vw"));
    assert_eq!(style.contain_intrinsic_block_size.as_deref(), Some("25vh"));
    assert!(!style.unsupported.contains_key("contain"));
    assert!(!style.unsupported.contains_key("container-type"));
    assert!(!style.unsupported.contains_key("content"));
    assert!(!style.unsupported.contains_key("counter-reset"));
    assert!(!style.unsupported.contains_key("counter-increment"));
    assert!(!style.unsupported.contains_key("counter-set"));
    assert!(!style.unsupported.contains_key("quotes"));
    assert!(!style.unsupported.contains_key("string-set"));
    assert!(!style.unsupported.contains_key("content-visibility"));
}

#[test]
fn parses_tailwind_arbitrary_counter_and_quotes_properties() {
    let web = WebProps::new().class_name(
        "[counter-reset:section_0] [counter-increment:section_1] \
             [counter-set:chapter_2] [quotes:\"\\201C\"_\"\\201D\"] \
             [string-set:chapter_content(text)] \
             hover:[counter-reset:item_4] focus:[counter-increment:item_-1] \
             active:[counter-set:chapter_3] before:[quotes:none] \
             after:[string-set:section_content(before)]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.counter_reset.as_deref(), Some("section 0"));
    assert_eq!(style.counter_increment.as_deref(), Some("section 1"));
    assert_eq!(style.counter_set.as_deref(), Some("chapter 2"));
    assert_eq!(style.quotes.as_deref(), Some("\"\\201C\" \"\\201D\""));
    assert_eq!(style.string_set.as_deref(), Some("chapter content(text)"));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("counter-reset"))
            .map(String::as_str),
        Some("item 4")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("counter-increment"))
            .map(String::as_str),
        Some("item -1")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("counter-set"))
            .map(String::as_str),
        Some("chapter 3")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("quotes"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("string-set"))
            .map(String::as_str),
        Some("section content(before)")
    );
}

#[test]
fn parses_tailwind_container_query_markers_and_variants() {
    let web = WebProps::new().class_name(
        "@container/sidebar @md:flex @container-size/[detail_panel] \
             hover:@container-normal lg:@container/[wide_panel]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.container_type, Some(ContainerType::Size));
    assert_eq!(style.container_name.as_deref(), Some("detail panel"));
    assert_eq!(
        style.declarations.get("container-type").map(String::as_str),
        Some("size")
    );
    assert_eq!(
        style.declarations.get("container-name").map(String::as_str),
        Some("detail panel")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("@md")
            .and_then(|styles| styles.get("display"))
            .map(String::as_str),
        Some("flex")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("container-type"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("container-type"))
            .map(String::as_str),
        Some("inline-size")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("container-name"))
            .map(String::as_str),
        Some("wide panel")
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
        .style("readingFlow", "grid-rows")
        .style("readingOrder", "2")
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
    assert_eq!(style.reading_flow.as_deref(), Some("grid-rows"));
    assert_eq!(style.reading_order.as_deref(), Some("2"));
    assert_eq!(style.align_content, Some(JustifyContent::SpaceBetween));
    assert_eq!(style.align_self, Some(SelfAlignment::Stretch));
    assert_eq!(style.justify_items, Some(AlignItems::Center));
    assert_eq!(style.justify_self, Some(SelfAlignment::End));
    assert_eq!(style.place_content.as_deref(), Some("center stretch"));
    assert_eq!(style.place_items.as_deref(), Some("start"));
    assert_eq!(style.place_self.as_deref(), Some("end"));
    assert!(!style.unsupported.contains_key("flex-basis"));
    assert!(!style.unsupported.contains_key("reading-flow"));
    assert!(!style.unsupported.contains_key("reading-order"));
    assert!(!style.unsupported.contains_key("align-self"));
}

#[test]
fn parses_tailwind_flex_item_and_box_alignment_utilities() {
    let web = WebProps::new().class_name(
        "flex-1 basis-1/2 grow-2 shrink-0 order-first -order-2 \
             content-between self-end justify-items-center justify-self-stretch \
             place-content-evenly place-items-baseline place-self-start \
             [reading-flow:grid-rows] [reading-order:2] \
             md:basis-[calc(50%_-_1rem)] hover:order-[7] \
             focus:[reading-flow:flex-visual] active:[reading-order:5]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.flex.as_deref(), Some("1"));
    assert_eq!(style.flex_basis, Some(StyleLength::Percent(50.0)));
    assert_eq!(style.flex_grow.as_deref(), Some("2"));
    assert_eq!(style.flex_shrink.as_deref(), Some("0"));
    assert_eq!(style.order.as_deref(), Some("calc(2 * -1)"));
    assert_eq!(style.reading_flow.as_deref(), Some("grid-rows"));
    assert_eq!(style.reading_order.as_deref(), Some("2"));
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
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("reading-flow"))
            .map(String::as_str),
        Some("flex-visual")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("reading-order"))
            .map(String::as_str),
        Some("5")
    );
}

#[test]
fn parses_css_typography_text_properties_into_portable_tokens() {
    let web = WebProps::new()
        .style("font", "italic small-caps 16px/1.5 ui-serif")
        .style("fontFamily", "ui-monospace, monospace")
        .style("fontStyle", "italic")
        .style("fontStretch", "semi-condensed")
        .style("fontWidth", "condensed")
        .style("fontPalette", "dark")
        .style("fontLanguageOverride", "\"TRK\"")
        .style("fontKerning", "normal")
        .style("fontOpticalSizing", "auto")
        .style("WebkitFontSmoothing", "antialiased")
        .style("MozOsxFontSmoothing", "grayscale")
        .style("fontFeatureSettings", "\"kern\" 1, \"liga\" 0")
        .style("fontVariationSettings", "\"wght\" 650")
        .style("fontSizeAdjust", "0.5")
        .style("fontVariant", "small-caps tabular-nums")
        .style("fontVariantAlternates", "historical-forms")
        .style("fontVariantCaps", "small-caps")
        .style("fontVariantEastAsian", "jis78")
        .style("fontVariantEmoji", "emoji")
        .style("fontVariantLigatures", "common-ligatures")
        .style("fontVariantNumeric", "tabular-nums slashed-zero")
        .style("fontVariantPosition", "sub")
        .style("fontSynthesis", "weight style")
        .style("fontSynthesisWeight", "none")
        .style("fontSynthesisStyle", "auto")
        .style("fontSynthesisSmallCaps", "none")
        .style("fontSynthesisPosition", "auto")
        .style("lineHeightStep", "4px")
        .style("blockStep", "1lh center up")
        .style("blockStepSize", "1lh")
        .style("blockStepInsert", "margin-box")
        .style("blockStepAlign", "center")
        .style("blockStepRound", "up")
        .style("lineGrid", "create")
        .style("lineSnap", "baseline")
        .style("boxSnap", "block-start")
        .style("mathDepth", "add(1)")
        .style("mathShift", "compact")
        .style("mathStyle", "compact")
        .style("dominantBaseline", "central")
        .style("baselineSource", "first")
        .style("alignmentBaseline", "text-before-edge")
        .style("baselineShift", "super")
        .style("lineFitEdge", "leading")
        .style("inlineSizing", "stretch")
        .style("initialLetter", "3 2")
        .style("initialLetterAlign", "border-box")
        .style("initialLetterWrap", "first")
        .style("letterSpacing", "0.025em")
        .style("wordSpacing", "0.125em")
        .style("tabSize", "4")
        .style("textSizeAdjust", "100%")
        .style("WebkitTextSizeAdjust", "none")
        .style("MozTextSizeAdjust", "auto")
        .style("msTextSizeAdjust", "80%")
        .style("textAlignAll", "justify")
        .style("textAlignLast", "center")
        .style("textGroupAlign", "end")
        .style("textJustify", "inter-character")
        .style("wordSpaceTransform", "space")
        .style("direction", "rtl")
        .style("unicodeBidi", "isolate-override")
        .style("-webkitWritingMode", "vertical-lr")
        .style("textOrientation", "upright")
        .style("textCombineUpright", "digits 2")
        .style("textTransform", "uppercase")
        .style("textIndent", "2rem")
        .style("textWrap", "balance")
        .style("textWrapMode", "nowrap")
        .style("textWrapStyle", "pretty")
        .style("wrapBefore", "avoid")
        .style("wrapAfter", "avoid")
        .style("wrapInside", "avoid")
        .style("linePadding", "0.5em")
        .style("textSpacing", "trim-start allow-end")
        .style("textSpacingTrim", "trim-start")
        .style("textAutospace", "ideograph-alpha ideograph-numeric")
        .style("textBox", "trim-both cap alphabetic")
        .style("textBoxTrim", "trim-both")
        .style("textBoxEdge", "cap alphabetic")
        .style("hangingPunctuation", "first allow-end")
        .style("lineClamp", "3")
        .style("blockEllipsis", "\"...\"")
        .style("continue", "discard")
        .style("maxLines", "4")
        .style("display", "-webkit-box")
        .style("-webkitBoxOrient", "vertical")
        .style("textDecorationLine", "underline")
        .style("textDecorationColor", "#663399")
        .style("textDecorationStyle", "wavy")
        .style("textDecorationThickness", "from-font")
        .style("textDecorationSkip", "objects spaces")
        .style("textDecorationSkipBox", "all")
        .style("textDecorationSkipInk", "none")
        .style("textDecorationSkipInset", "0.15em")
        .style("textDecorationSkipSelf", "skip-all")
        .style("textDecorationSkipSpaces", "start end")
        .style("textUnderlineOffset", "4px")
        .style("textUnderlinePosition", "under left")
        .style("textEmphasis", "filled sesame #663399")
        .style("textEmphasisSkip", "spaces")
        .style("WebkitTextEmphasisPosition", "under right")
        .style("textShadow", "0 1px 2px rgb(0 0 0 / 0.3)")
        .style("textOverflow", "ellipsis")
        .style("lineBreak", "strict")
        .style("whiteSpace", "nowrap")
        .style("whiteSpaceCollapse", "preserve-breaks")
        .style("whiteSpaceTrim", "discard-before")
        .style("wordBreak", "keep-all")
        .style("overflowWrap", "anywhere")
        .style("hyphens", "auto")
        .style("hyphenateCharacter", "\"=\"")
        .style("hyphenateLimitZone", "8%")
        .style("hyphenateLimitChars", "6 3 2")
        .style("hyphenateLimitLines", "2")
        .style("hyphenateLimitLast", "always");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.font.as_deref(),
        Some("italic small-caps 16px/1.5 ui-serif")
    );
    assert_eq!(
        style.font_family.as_deref(),
        Some("ui-monospace, monospace")
    );
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert_eq!(style.font_stretch.as_deref(), Some("semi-condensed"));
    assert_eq!(style.font_width.as_deref(), Some("condensed"));
    assert_eq!(style.font_palette.as_deref(), Some("dark"));
    assert_eq!(style.font_language_override.as_deref(), Some("\"TRK\""));
    assert_eq!(style.font_kerning.as_deref(), Some("normal"));
    assert_eq!(style.font_optical_sizing.as_deref(), Some("auto"));
    assert_eq!(style.webkit_font_smoothing.as_deref(), Some("antialiased"));
    assert_eq!(style.moz_osx_font_smoothing.as_deref(), Some("grayscale"));
    assert_eq!(
        style.font_feature_settings.as_deref(),
        Some("\"kern\" 1, \"liga\" 0")
    );
    assert_eq!(
        style.font_variation_settings.as_deref(),
        Some("\"wght\" 650")
    );
    assert_eq!(style.font_size_adjust.as_deref(), Some("0.5"));
    assert_eq!(
        style.font_variant.as_deref(),
        Some("small-caps tabular-nums")
    );
    assert_eq!(
        style.font_variant_alternates.as_deref(),
        Some("historical-forms")
    );
    assert_eq!(style.font_variant_caps.as_deref(), Some("small-caps"));
    assert_eq!(style.font_variant_east_asian.as_deref(), Some("jis78"));
    assert_eq!(style.font_variant_emoji.as_deref(), Some("emoji"));
    assert_eq!(
        style.font_variant_ligatures.as_deref(),
        Some("common-ligatures")
    );
    assert_eq!(
        style.font_variant_numeric.as_deref(),
        Some("tabular-nums slashed-zero")
    );
    assert_eq!(style.font_variant_position.as_deref(), Some("sub"));
    assert_eq!(style.font_synthesis.as_deref(), Some("weight style"));
    assert_eq!(style.font_synthesis_weight.as_deref(), Some("none"));
    assert_eq!(style.font_synthesis_style.as_deref(), Some("auto"));
    assert_eq!(style.font_synthesis_small_caps.as_deref(), Some("none"));
    assert_eq!(style.font_synthesis_position.as_deref(), Some("auto"));
    assert_eq!(style.line_height_step.as_deref(), Some("4px"));
    assert_eq!(style.block_step.as_deref(), Some("1lh center up"));
    assert_eq!(style.block_step_size.as_deref(), Some("1lh"));
    assert_eq!(style.block_step_insert.as_deref(), Some("margin-box"));
    assert_eq!(style.block_step_align.as_deref(), Some("center"));
    assert_eq!(style.block_step_round.as_deref(), Some("up"));
    assert_eq!(style.line_grid.as_deref(), Some("create"));
    assert_eq!(style.line_snap.as_deref(), Some("baseline"));
    assert_eq!(style.box_snap.as_deref(), Some("block-start"));
    assert_eq!(style.math_depth.as_deref(), Some("add(1)"));
    assert_eq!(style.math_shift.as_deref(), Some("compact"));
    assert_eq!(style.math_style.as_deref(), Some("compact"));
    assert_eq!(style.dominant_baseline.as_deref(), Some("central"));
    assert_eq!(style.baseline_source.as_deref(), Some("first"));
    assert_eq!(
        style.alignment_baseline.as_deref(),
        Some("text-before-edge")
    );
    assert_eq!(style.baseline_shift.as_deref(), Some("super"));
    assert_eq!(style.line_fit_edge.as_deref(), Some("leading"));
    assert_eq!(style.inline_sizing.as_deref(), Some("stretch"));
    assert_eq!(style.initial_letter.as_deref(), Some("3 2"));
    assert_eq!(style.initial_letter_align.as_deref(), Some("border-box"));
    assert_eq!(style.initial_letter_wrap.as_deref(), Some("first"));
    assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
    assert_eq!(style.word_spacing, Some(StyleLength::Points(2.0)));
    assert_eq!(style.tab_size.as_deref(), Some("4"));
    assert_eq!(style.text_size_adjust.as_deref(), Some("100%"));
    assert_eq!(style.webkit_text_size_adjust.as_deref(), Some("none"));
    assert_eq!(style.moz_text_size_adjust.as_deref(), Some("auto"));
    assert_eq!(style.ms_text_size_adjust.as_deref(), Some("80%"));
    assert_eq!(style.text_align_all.as_deref(), Some("justify"));
    assert_eq!(style.text_align_last.as_deref(), Some("center"));
    assert_eq!(style.text_group_align.as_deref(), Some("end"));
    assert_eq!(style.text_justify.as_deref(), Some("inter-character"));
    assert_eq!(style.word_space_transform.as_deref(), Some("space"));
    assert_eq!(style.direction, Some(TextDirection::Rtl));
    assert_eq!(style.unicode_bidi, Some(UnicodeBidi::IsolateOverride));
    assert_eq!(style.writing_mode, Some(WritingMode::VerticalLr));
    assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
    assert_eq!(style.text_combine_upright.as_deref(), Some("digits 2"));
    assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
    assert_eq!(style.text_indent, Some(StyleLength::Points(32.0)));
    assert_eq!(style.text_wrap, Some(TextWrapMode::NoWrap));
    assert_eq!(style.text_wrap_mode.as_deref(), Some("nowrap"));
    assert_eq!(style.text_wrap_style.as_deref(), Some("pretty"));
    assert_eq!(style.wrap_before.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_after.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_inside.as_deref(), Some("avoid"));
    assert_eq!(style.line_padding.as_deref(), Some("0.5em"));
    assert_eq!(style.text_spacing.as_deref(), Some("trim-start allow-end"));
    assert_eq!(style.text_spacing_trim.as_deref(), Some("trim-start"));
    assert_eq!(
        style.text_autospace.as_deref(),
        Some("ideograph-alpha ideograph-numeric")
    );
    assert_eq!(style.text_box.as_deref(), Some("trim-both cap alphabetic"));
    assert_eq!(style.text_box_trim.as_deref(), Some("trim-both"));
    assert_eq!(style.text_box_edge.as_deref(), Some("cap alphabetic"));
    assert_eq!(
        style.hanging_punctuation.as_deref(),
        Some("first allow-end")
    );
    assert_eq!(style.line_clamp.as_deref(), Some("3"));
    assert_eq!(style.block_ellipsis.as_deref(), Some("\"...\""));
    assert_eq!(style.continue_mode.as_deref(), Some("discard"));
    assert_eq!(style.max_lines.as_deref(), Some("4"));
    assert_eq!(style.display, Some(DisplayMode::WebkitBox));
    assert_eq!(style.box_orient.as_deref(), Some("vertical"));
    assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
    assert_eq!(
        style.text_decoration_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
    assert_eq!(
        style.text_decoration_thickness,
        Some(StyleLength::Css("from-font".to_string()))
    );
    assert_eq!(
        style.text_decoration_skip.as_deref(),
        Some("objects spaces")
    );
    assert_eq!(style.text_decoration_skip_box.as_deref(), Some("all"));
    assert_eq!(style.text_decoration_skip_ink.as_deref(), Some("none"));
    assert_eq!(style.text_decoration_skip_inset.as_deref(), Some("0.15em"));
    assert_eq!(style.text_decoration_skip_self.as_deref(), Some("skip-all"));
    assert_eq!(
        style.text_decoration_skip_spaces.as_deref(),
        Some("start end")
    );
    assert_eq!(style.text_underline_offset, Some(StyleLength::Points(4.0)));
    assert_eq!(style.text_underline_position.as_deref(), Some("under left"));
    assert_eq!(style.text_emphasis_style.as_deref(), Some("filled sesame"));
    assert_eq!(
        style.text_emphasis_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.text_emphasis_position.as_deref(), Some("under right"));
    assert_eq!(style.text_emphasis_skip.as_deref(), Some("spaces"));
    assert_eq!(
        style.text_shadow.as_deref(),
        Some("0 1px 2px rgb(0 0 0 / 0.3)")
    );
    assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
    assert_eq!(style.line_break.as_deref(), Some("strict"));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::NoWrap));
    assert_eq!(
        style.white_space_collapse.as_deref(),
        Some("preserve-breaks")
    );
    assert_eq!(style.white_space_trim.as_deref(), Some("discard-before"));
    assert_eq!(style.word_break, Some(WordBreakMode::KeepAll));
    assert_eq!(style.overflow_wrap, Some(OverflowWrapMode::Anywhere));
    assert_eq!(style.hyphens, Some(HyphensMode::Auto));
    assert_eq!(style.hyphenate_character.as_deref(), Some("\"=\""));
    assert_eq!(style.hyphenate_limit_zone.as_deref(), Some("8%"));
    assert_eq!(style.hyphenate_limit_chars.as_deref(), Some("6 3 2"));
    assert_eq!(style.hyphenate_limit_lines.as_deref(), Some("2"));
    assert_eq!(style.hyphenate_limit_last.as_deref(), Some("always"));
    assert!(!style.unsupported.contains_key("text-decoration-line"));
    assert!(!style.unsupported.contains_key("text-decoration-skip"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-box"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-ink"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-inset"));
    assert!(!style.unsupported.contains_key("text-decoration-skip-self"));
    assert!(!style
        .unsupported
        .contains_key("text-decoration-skip-spaces"));
    assert!(!style.unsupported.contains_key("text-underline-position"));
    assert!(!style.unsupported.contains_key("text-emphasis"));
    assert!(!style.unsupported.contains_key("text-emphasis-skip"));
    assert!(!style
        .unsupported
        .contains_key("webkit-text-emphasis-position"));
    assert!(!style.unsupported.contains_key("font"));
    assert!(!style.unsupported.contains_key("font-feature-settings"));
    assert!(!style.unsupported.contains_key("font-size-adjust"));
    assert!(!style.unsupported.contains_key("font-width"));
    assert!(!style.unsupported.contains_key("font-palette"));
    assert!(!style.unsupported.contains_key("font-language-override"));
    assert!(!style.unsupported.contains_key("font-variant-numeric"));
    assert!(!style.unsupported.contains_key("line-height-step"));
    assert!(!style.unsupported.contains_key("block-step"));
    assert!(!style.unsupported.contains_key("block-step-size"));
    assert!(!style.unsupported.contains_key("block-step-insert"));
    assert!(!style.unsupported.contains_key("block-step-align"));
    assert!(!style.unsupported.contains_key("block-step-round"));
    assert!(!style.unsupported.contains_key("line-grid"));
    assert!(!style.unsupported.contains_key("line-snap"));
    assert!(!style.unsupported.contains_key("box-snap"));
    assert!(!style.unsupported.contains_key("math-depth"));
    assert!(!style.unsupported.contains_key("math-shift"));
    assert!(!style.unsupported.contains_key("math-style"));
    assert!(!style.unsupported.contains_key("text-size-adjust"));
    assert!(!style.unsupported.contains_key("dominant-baseline"));
    assert!(!style.unsupported.contains_key("baseline-source"));
    assert!(!style.unsupported.contains_key("alignment-baseline"));
    assert!(!style.unsupported.contains_key("baseline-shift"));
    assert!(!style.unsupported.contains_key("line-fit-edge"));
    assert!(!style.unsupported.contains_key("inline-sizing"));
    assert!(!style.unsupported.contains_key("initial-letter"));
    assert!(!style.unsupported.contains_key("initial-letter-align"));
    assert!(!style.unsupported.contains_key("initial-letter-wrap"));
    assert!(!style.unsupported.contains_key("block-ellipsis"));
    assert!(!style.unsupported.contains_key("continue"));
    assert!(!style.unsupported.contains_key("max-lines"));
    assert!(!style.unsupported.contains_key("webkit-text-size-adjust"));
    assert!(!style.unsupported.contains_key("moz-text-size-adjust"));
    assert!(!style.unsupported.contains_key("ms-text-size-adjust"));
    assert!(!style.unsupported.contains_key("text-align-all"));
    assert!(!style.unsupported.contains_key("text-align-last"));
    assert!(!style.unsupported.contains_key("text-group-align"));
    assert!(!style.unsupported.contains_key("text-justify"));
    assert!(!style.unsupported.contains_key("word-space-transform"));
    assert!(!style.unsupported.contains_key("white-space"));
    assert!(!style.unsupported.contains_key("text-shadow"));
    assert!(!style.unsupported.contains_key("webkit-font-smoothing"));
    assert!(!style.unsupported.contains_key("moz-osx-font-smoothing"));
    assert!(!style.unsupported.contains_key("-webkit-writing-mode"));
    assert!(!style.unsupported.contains_key("text-combine-upright"));
    assert!(!style.unsupported.contains_key("hanging-punctuation"));
    assert!(!style.unsupported.contains_key("text-wrap"));
    assert!(!style.unsupported.contains_key("text-wrap-mode"));
    assert!(!style.unsupported.contains_key("text-wrap-style"));
    assert!(!style.unsupported.contains_key("wrap-before"));
    assert!(!style.unsupported.contains_key("wrap-after"));
    assert!(!style.unsupported.contains_key("wrap-inside"));
    assert!(!style.unsupported.contains_key("line-padding"));
    assert!(!style.unsupported.contains_key("text-spacing"));
    assert!(!style.unsupported.contains_key("text-spacing-trim"));
    assert!(!style.unsupported.contains_key("text-autospace"));
    assert!(!style.unsupported.contains_key("text-box"));
    assert!(!style.unsupported.contains_key("text-box-trim"));
    assert!(!style.unsupported.contains_key("text-box-edge"));
    assert!(!style.unsupported.contains_key("white-space-collapse"));
    assert!(!style.unsupported.contains_key("white-space-trim"));
    assert!(!style.unsupported.contains_key("hyphenate-character"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-zone"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-chars"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-lines"));
    assert!(!style.unsupported.contains_key("hyphenate-limit-last"));
    assert!(!style.unsupported.contains_key("-webkit-line-clamp"));
}

#[test]
fn parses_prefixed_text_combine_aliases() {
    let webkit = PortableStyle::from_web(&WebProps::new().style("WebkitTextCombine", "digits 2"));
    assert_eq!(webkit.text_combine_upright.as_deref(), Some("digits 2"));
    assert!(!webkit.unsupported.contains_key("webkit-text-combine"));

    let ms = PortableStyle::from_web(&WebProps::new().style("-msTextCombineHorizontal", "all"));
    assert_eq!(ms.text_combine_upright.as_deref(), Some("all"));
    assert!(!ms.unsupported.contains_key("-ms-text-combine-horizontal"));
}

#[test]
fn parses_prefixed_font_language_override_alias() {
    let style =
        PortableStyle::from_web(&WebProps::new().style("MozFontLanguageOverride", "\"SRB\""));

    assert_eq!(style.font_language_override.as_deref(), Some("\"SRB\""));
    assert!(!style.unsupported.contains_key("moz-font-language-override"));
}

#[test]
fn parses_prefixed_text_autospace_alias() {
    let style =
        PortableStyle::from_web(&WebProps::new().style("MsTextAutospace", "ideograph-alpha"));

    assert_eq!(style.text_autospace.as_deref(), Some("ideograph-alpha"));
    assert!(!style.unsupported.contains_key("ms-text-autospace"));
}

#[test]
fn parses_tailwind_typography_text_utilities() {
    let web = WebProps::new().class_name(
        "font-mono italic antialiased tracking-wide uppercase underline decoration-wavy \
             decoration-[#663399]/50 decoration-2 underline-offset-4 truncate \
             whitespace-pre-wrap break-all wrap-anywhere hyphens-auto -indent-[2px] text-balance \
             ordinal slashed-zero tabular-nums diagonal-fractions \
             font-stretch-condensed font-features-[\"kern\"_1] tab-4 text-shadow-sm \
             [font:italic_1rem/1.5_serif] [font-size-adjust:0.5] \
             [font-width:condensed] [font-palette:dark] [font-language-override:\"TRK\"] \
             [dominant-baseline:central] [baseline-source:first] \
             [alignment-baseline:text-before-edge] [baseline-shift:super] \
             [line-fit-edge:leading] [inline-sizing:stretch] \
             [line-height-step:4px] [block-step:1lh_center_up] \
             [block-step-size:1lh] [block-step-insert:margin-box] \
             [block-step-align:center] [block-step-round:up] \
             [line-grid:create] [line-snap:baseline] [box-snap:block-start] \
             [math-depth:add(1)] [math-shift:compact] [math-style:compact] \
             [initial-letter:3_2] [initial-letter-align:border-box] \
             [initial-letter-wrap:first] \
             [text-size-adjust:100%] \
             [-webkit-text-size-adjust:none] [-moz-text-size-adjust:auto] \
             [-ms-text-size-adjust:80%] \
             [text-align-all:justify] [text-align-last:center] \
             [text-group-align:end] [text-justify:inter-word] \
             [word-space-transform:space] \
             [direction:rtl] [unicode-bidi:isolate] [writing-mode:vertical-rl] \
             [text-orientation:upright] [text-combine-upright:all] \
             [text-decoration-skip:objects_spaces] [text-decoration-skip-box:all] \
             [text-decoration-skip-ink:none] \
             [text-decoration-skip-inset:0.15em] [text-decoration-skip-self:skip-all] \
             [text-decoration-skip-spaces:start_end] \
             [text-underline-position:under_left] [text-emphasis-style:open_dot] \
             [text-emphasis-color:#663399] [text-emphasis-position:under_right] \
             [text-emphasis-skip:spaces] \
             line-clamp-3 md:tracking-[0.2em] hover:decoration-[3px] \
             focus:text-pretty lg:line-clamp-none ltr:[direction:ltr] \
             rtl:[unicode-bidi:plaintext] md:[writing-mode:horizontal-tb] \
             hover:[text-orientation:sideways] md:font-stretch-[87.5%] \
             hover:[font-width:expanded] focus:[math-depth:0] \
             active:[math-style:normal] before:[math-shift:normal] \
             hover:[font-size-adjust:0.6] focus:[text-size-adjust:none] \
             hover:[font-palette:light] focus:[font-language-override:normal] \
             hover:[dominant-baseline:hanging] focus:[baseline-shift:sub] \
             active:[line-height-step:8px] hover:[block-step:none] \
             focus:[block-step-size:2lh] before:[line-grid:match-parent] \
             after:[line-snap:contain] visited:[box-snap:none] \
             active:[initial-letter:2] before:[initial-letter-wrap:all] \
             after:[block-ellipsis:auto] visited:[max-lines:none] \
             md:[text-align-all:center] hover:[text-group-align:start] \
             focus:[word-space-transform:ideographs] \
             hover:[text-align-last:right] focus:[text-justify:inter-character] \
             visited:[hanging-punctuation:last_force-end] \
             active:[text-combine-upright:digits_2] \
             [white-space-collapse:preserve-breaks] [text-wrap-mode:nowrap] \
             [text-wrap-style:stable] [wrap-before:avoid] [wrap-after:avoid] \
             [wrap-inside:avoid] [line-padding:0.5em] \
             [text-spacing:trim-start_allow-end] [text-spacing-trim:space-all] \
             [text-autospace:ideograph-alpha] [text-box:trim-both_cap_alphabetic] \
             [text-box-trim:trim-start] [text-box-edge:cap_alphabetic] \
             [block-ellipsis:\"...\"] [continue:discard] [max-lines:4] \
             [white-space-trim:discard-before] [hyphenate-character:\"=\"] \
             [hyphenate-limit-zone:8%] [hyphenate-limit-chars:6_3_2] \
             [hyphenate-limit-lines:2] [hyphenate-limit-last:always] \
             hover:[text-decoration-skip:objects] focus:[text-decoration-skip-box:none] \
             active:[text-decoration-skip-inset:auto] visited:[text-decoration-skip-self:skip] \
             before:[text-decoration-skip-spaces:none] after:[text-emphasis-skip:punctuation] \
             hover:[text-decoration-skip-ink:all] focus:[text-underline-position:left] \
             hover:[text-emphasis-style:filled_sesame] focus:[text-emphasis-color:#663399] \
             active:[text-emphasis-position:over_left] \
             hover:font-features-(--font-features) focus:text-shadow-[0_1px_2px_rgb(0_0_0_/_0.3)] \
             lg:normal-nums content-none before:content-['Hello_World'] \
             after:content-(--suffix-content) hover:content-['Hello\\_World'] \
             hover:[white-space-collapse:preserve-spaces] focus:[text-wrap-style:pretty] \
             active:[text-spacing-trim:trim-auto] before:[wrap-before:auto] \
             after:[hyphenate-limit-lines:no-limit] before:[text-box-trim:none] \
             after:[text-box-edge:text] visited:[text-autospace:no-autospace] \
             [hanging-punctuation:first_allow-end] \
             hover:wrap-break-word focus:wrap-[normal] active:wrap-(--overflow-wrap) \
             md:subpixel-antialiased",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.font_family.as_deref(),
        Some("ui-monospace, monospace")
    );
    assert_eq!(style.font_style, Some(FontStyle::Italic));
    assert_eq!(style.letter_spacing, Some(StyleLength::Points(0.4)));
    assert_eq!(
        style.declarations.get("letter-spacing").map(String::as_str),
        Some("0.025em")
    );
    assert_eq!(style.font_stretch.as_deref(), Some("condensed"));
    assert_eq!(style.webkit_font_smoothing.as_deref(), Some("antialiased"));
    assert_eq!(style.moz_osx_font_smoothing.as_deref(), Some("grayscale"));
    assert_eq!(style.font.as_deref(), Some("italic 1rem/1.5 serif"));
    assert_eq!(style.font_feature_settings.as_deref(), Some("\"kern\" 1"));
    assert_eq!(style.font_size_adjust.as_deref(), Some("0.5"));
    assert_eq!(style.font_width.as_deref(), Some("condensed"));
    assert_eq!(style.font_palette.as_deref(), Some("dark"));
    assert_eq!(style.font_language_override.as_deref(), Some("\"TRK\""));
    assert_eq!(style.dominant_baseline.as_deref(), Some("central"));
    assert_eq!(style.baseline_source.as_deref(), Some("first"));
    assert_eq!(
        style.alignment_baseline.as_deref(),
        Some("text-before-edge")
    );
    assert_eq!(style.baseline_shift.as_deref(), Some("super"));
    assert_eq!(style.line_fit_edge.as_deref(), Some("leading"));
    assert_eq!(style.inline_sizing.as_deref(), Some("stretch"));
    assert_eq!(style.line_height_step.as_deref(), Some("4px"));
    assert_eq!(style.block_step.as_deref(), Some("1lh center up"));
    assert_eq!(style.block_step_size.as_deref(), Some("1lh"));
    assert_eq!(style.block_step_insert.as_deref(), Some("margin-box"));
    assert_eq!(style.block_step_align.as_deref(), Some("center"));
    assert_eq!(style.block_step_round.as_deref(), Some("up"));
    assert_eq!(style.line_grid.as_deref(), Some("create"));
    assert_eq!(style.line_snap.as_deref(), Some("baseline"));
    assert_eq!(style.box_snap.as_deref(), Some("block-start"));
    assert_eq!(style.math_depth.as_deref(), Some("add(1)"));
    assert_eq!(style.math_shift.as_deref(), Some("compact"));
    assert_eq!(style.math_style.as_deref(), Some("compact"));
    assert_eq!(style.initial_letter.as_deref(), Some("3 2"));
    assert_eq!(style.initial_letter_align.as_deref(), Some("border-box"));
    assert_eq!(style.initial_letter_wrap.as_deref(), Some("first"));
    assert_eq!(
        style.font_variant_numeric.as_deref(),
        Some("ordinal slashed-zero tabular-nums diagonal-fractions")
    );
    assert_eq!(style.tab_size.as_deref(), Some("4"));
    assert_eq!(style.text_size_adjust.as_deref(), Some("100%"));
    assert_eq!(style.webkit_text_size_adjust.as_deref(), Some("none"));
    assert_eq!(style.moz_text_size_adjust.as_deref(), Some("auto"));
    assert_eq!(style.ms_text_size_adjust.as_deref(), Some("80%"));
    assert_eq!(style.text_align_all.as_deref(), Some("justify"));
    assert_eq!(style.text_align_last.as_deref(), Some("center"));
    assert_eq!(style.text_group_align.as_deref(), Some("end"));
    assert_eq!(style.text_justify.as_deref(), Some("inter-word"));
    assert_eq!(style.word_space_transform.as_deref(), Some("space"));
    assert_eq!(style.text_shadow.as_deref(), Some("var(--text-shadow-sm)"));
    assert_eq!(style.text_transform, Some(TextTransform::Uppercase));
    assert_eq!(style.direction, Some(TextDirection::Rtl));
    assert_eq!(style.unicode_bidi, Some(UnicodeBidi::Isolate));
    assert_eq!(style.writing_mode, Some(WritingMode::VerticalRl));
    assert_eq!(style.text_orientation, Some(TextOrientation::Upright));
    assert_eq!(style.text_combine_upright.as_deref(), Some("all"));
    assert_eq!(style.text_indent, Some(StyleLength::Points(-2.0)));
    assert_eq!(style.text_wrap, Some(TextWrapMode::NoWrap));
    assert_eq!(style.text_wrap_mode.as_deref(), Some("nowrap"));
    assert_eq!(style.text_wrap_style.as_deref(), Some("stable"));
    assert_eq!(style.wrap_before.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_after.as_deref(), Some("avoid"));
    assert_eq!(style.wrap_inside.as_deref(), Some("avoid"));
    assert_eq!(style.line_padding.as_deref(), Some("0.5em"));
    assert_eq!(style.text_spacing.as_deref(), Some("trim-start allow-end"));
    assert_eq!(style.text_spacing_trim.as_deref(), Some("space-all"));
    assert_eq!(style.text_autospace.as_deref(), Some("ideograph-alpha"));
    assert_eq!(style.text_box.as_deref(), Some("trim-both cap alphabetic"));
    assert_eq!(style.text_box_trim.as_deref(), Some("trim-start"));
    assert_eq!(style.text_box_edge.as_deref(), Some("cap alphabetic"));
    assert_eq!(
        style.hanging_punctuation.as_deref(),
        Some("first allow-end")
    );
    assert_eq!(style.line_clamp.as_deref(), Some("3"));
    assert_eq!(style.block_ellipsis.as_deref(), Some("\"...\""));
    assert_eq!(style.continue_mode.as_deref(), Some("discard"));
    assert_eq!(style.max_lines.as_deref(), Some("4"));
    assert_eq!(style.box_orient.as_deref(), Some("vertical"));
    assert_eq!(style.display, Some(DisplayMode::WebkitBox));
    assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
    assert_eq!(style.text_decoration_line.as_deref(), Some("underline"));
    assert_eq!(style.text_decoration_style, Some(TextDecorationStyle::Wavy));
    assert_eq!(
        style.text_decoration_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(
        style.text_decoration_thickness,
        Some(StyleLength::Points(2.0))
    );
    assert_eq!(
        style.text_decoration_skip.as_deref(),
        Some("objects spaces")
    );
    assert_eq!(style.text_decoration_skip_box.as_deref(), Some("all"));
    assert_eq!(style.text_decoration_skip_ink.as_deref(), Some("none"));
    assert_eq!(style.text_decoration_skip_inset.as_deref(), Some("0.15em"));
    assert_eq!(style.text_decoration_skip_self.as_deref(), Some("skip-all"));
    assert_eq!(
        style.text_decoration_skip_spaces.as_deref(),
        Some("start end")
    );
    assert_eq!(style.text_underline_offset, Some(StyleLength::Points(4.0)));
    assert_eq!(style.text_underline_position.as_deref(), Some("under left"));
    assert_eq!(style.text_emphasis_style.as_deref(), Some("open dot"));
    assert_eq!(
        style.text_emphasis_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.text_emphasis_position.as_deref(), Some("under right"));
    assert_eq!(style.text_emphasis_skip.as_deref(), Some("spaces"));
    assert_eq!(style.text_overflow, Some(TextOverflow::Ellipsis));
    assert_eq!(style.overflow_x, Some(OverflowMode::Hidden));
    assert_eq!(style.overflow_y, Some(OverflowMode::Hidden));
    assert_eq!(style.white_space, Some(WhiteSpaceMode::PreWrap));
    assert_eq!(
        style.white_space_collapse.as_deref(),
        Some("preserve-breaks")
    );
    assert_eq!(style.white_space_trim.as_deref(), Some("discard-before"));
    assert_eq!(style.word_break, Some(WordBreakMode::BreakAll));
    assert_eq!(style.overflow_wrap, Some(OverflowWrapMode::Anywhere));
    assert_eq!(style.hyphens, Some(HyphensMode::Auto));
    assert_eq!(style.hyphenate_character.as_deref(), Some("\"=\""));
    assert_eq!(style.hyphenate_limit_zone.as_deref(), Some("8%"));
    assert_eq!(style.hyphenate_limit_chars.as_deref(), Some("6 3 2"));
    assert_eq!(style.hyphenate_limit_lines.as_deref(), Some("2"));
    assert_eq!(style.hyphenate_limit_last.as_deref(), Some("always"));
    assert_eq!(style.content.as_deref(), Some("none"));
    assert_eq!(
        style
            .variant_declarations
            .get("ltr")
            .and_then(|styles| styles.get("direction"))
            .map(String::as_str),
        Some("ltr")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("rtl")
            .and_then(|styles| styles.get("unicode-bidi"))
            .map(String::as_str),
        Some("plaintext")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("-webkit-font-smoothing"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("-moz-osx-font-smoothing"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("letter-spacing"))
            .map(String::as_str),
        Some("0.2em")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("font-stretch"))
            .map(String::as_str),
        Some("87.5%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-width"))
            .map(String::as_str),
        Some("expanded")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("math-depth"))
            .map(String::as_str),
        Some("0")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("math-style"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("math-shift"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-size-adjust"))
            .map(String::as_str),
        Some("0.6")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-palette"))
            .map(String::as_str),
        Some("light")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("font-language-override"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("dominant-baseline"))
            .map(String::as_str),
        Some("hanging")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("baseline-shift"))
            .map(String::as_str),
        Some("sub")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("line-height-step"))
            .map(String::as_str),
        Some("8px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("block-step"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("block-step-size"))
            .map(String::as_str),
        Some("2lh")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("line-grid"))
            .map(String::as_str),
        Some("match-parent")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("line-snap"))
            .map(String::as_str),
        Some("contain")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("box-snap"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("initial-letter"))
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("initial-letter-wrap"))
            .map(String::as_str),
        Some("all")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("block-ellipsis"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("max-lines"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("text-align-all"))
            .map(String::as_str),
        Some("center")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-group-align"))
            .map(String::as_str),
        Some("start")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("word-space-transform"))
            .map(String::as_str),
        Some("ideographs")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("wrap-before"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("hyphenate-limit-lines"))
            .map(String::as_str),
        Some("no-limit")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-size-adjust"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-align-last"))
            .map(String::as_str),
        Some("right")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-justify"))
            .map(String::as_str),
        Some("inter-character")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("white-space-collapse"))
            .map(String::as_str),
        Some("preserve-spaces")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-wrap-style"))
            .map(String::as_str),
        Some("pretty")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-spacing-trim"))
            .map(String::as_str),
        Some("trim-auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("text-box-trim"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("text-box-edge"))
            .map(String::as_str),
        Some("text")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("text-autospace"))
            .map(String::as_str),
        Some("no-autospace")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("hanging-punctuation"))
            .map(String::as_str),
        Some("last force-end")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("writing-mode"))
            .map(String::as_str),
        Some("horizontal-tb")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-combine-upright"))
            .map(String::as_str),
        Some("digits 2")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-thickness"))
            .map(String::as_str),
        Some("3px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("break-word")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("overflow-wrap"))
            .map(String::as_str),
        Some("var(--overflow-wrap)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-skip"))
            .map(String::as_str),
        Some("objects")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-decoration-skip-box"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-decoration-skip-inset"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("text-decoration-skip-self"))
            .map(String::as_str),
        Some("skip")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("text-decoration-skip-spaces"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("text-emphasis-skip"))
            .map(String::as_str),
        Some("punctuation")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-decoration-skip-ink"))
            .map(String::as_str),
        Some("all")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-underline-position"))
            .map(String::as_str),
        Some("left")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-emphasis-style"))
            .map(String::as_str),
        Some("filled sesame")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-emphasis-color"))
            .map(String::as_str),
        Some("#663399")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("text-emphasis-position"))
            .map(String::as_str),
        Some("over left")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("text-orientation"))
            .map(String::as_str),
        Some("sideways")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("font-feature-settings"))
            .map(String::as_str),
        Some("var(--font-features)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("'Hello World'")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("after")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("var(--suffix-content)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("content"))
            .map(String::as_str),
        Some("'Hello_World'")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-wrap"))
            .map(String::as_str),
        Some("pretty")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("text-shadow"))
            .map(String::as_str),
        Some("0 1px 2px rgb(0 0 0 / 0.3)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("-webkit-line-clamp"))
            .map(String::as_str),
        Some("unset")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("font-variant-numeric"))
            .map(String::as_str),
        Some("normal")
    );
}

#[test]
fn parses_css_background_shorthand_property() {
    let web = WebProps::new().style(
        "background",
        "center / cover no-repeat fixed padding-box border-box url('/hero.png') #101820",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.background.as_deref(),
        Some("center / cover no-repeat fixed padding-box border-box url('/hero.png') #101820")
    );
    assert_eq!(
        style.declarations.get("background").map(String::as_str),
        Some("center / cover no-repeat fixed padding-box border-box url('/hero.png') #101820")
    );
    assert_eq!(style.background_color, None);
    assert!(!style.unsupported.contains_key("background"));

    let color_only = PortableStyle::from_web(&WebProps::new().style("background", "#663399"));
    assert_eq!(color_only.background.as_deref(), Some("#663399"));
    assert_eq!(
        color_only.background_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
}

#[test]
fn parses_tailwind_arbitrary_background_shorthand_property() {
    let web = WebProps::new().class_name(
        "[background:center_/_cover_no-repeat_url('/hero.png')_#101820] \
             hover:[background:linear-gradient(red,_blue)]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.background.as_deref(),
        Some("center / cover no-repeat url('/hero.png') #101820")
    );
    assert_eq!(
        style.declarations.get("background").map(String::as_str),
        Some("center / cover no-repeat url('/hero.png') #101820")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("background"))
            .map(String::as_str),
        Some("linear-gradient(red, blue)")
    );
    assert!(!style.unsupported.contains_key("background"));
}

#[test]
fn parses_css_background_object_list_and_columns_properties() {
    let web = WebProps::new()
        .style("backgroundImage", "url('/hero.png')")
        .style("backgroundPosition", "center top")
        .style("backgroundSize", "cover")
        .style("backgroundRepeat", "no-repeat")
        .style("backgroundAttachment", "fixed")
        .style("backgroundOrigin", "content-box")
        .style("backgroundClip", "padding-box")
        .style("imageRendering", "pixelated")
        .style("imageOrientation", "from-image")
        .style("imageResolution", "300dpi")
        .style("objectFit", "cover")
        .style("objectPosition", "left bottom")
        .style("shapeOutside", "circle(50% at 50% 50%)")
        .style("shapeInside", "polygon(0 0, 100% 0, 100% 100%)")
        .style("shapeMargin", "2rem")
        .style("shapePadding", "1.5rem")
        .style("shapeImageThreshold", "65%")
        .style("listStyleType", "disc")
        .style("listStylePosition", "inside")
        .style("listStyleImage", "url('/marker.svg')")
        .style("markerSide", "match-parent")
        .style("columns", "3")
        .style("columnCount", "2")
        .style("columnWidth", "12rem")
        .style("columnRule", "2px dashed #663399")
        .style("columnRuleWidth", "4px")
        .style("columnRuleStyle", "dotted")
        .style("columnRuleColor", "rgb(10 20 30)")
        .style("columnSpan", "all")
        .style("columnFill", "balance")
        .style("size", "A4 landscape")
        .style("page", "chapter")
        .style("pageOrientation", "rotate-left")
        .style("bleed", "6pt")
        .style("marks", "crop cross")
        .style("orphans", "3")
        .style("widows", "4")
        .style("bookmarkLabel", "content(text)")
        .style("bookmarkLevel", "2")
        .style("bookmarkState", "open")
        .style("footnoteDisplay", "block")
        .style("footnotePolicy", "line")
        .style("breakBefore", "page")
        .style("breakAfter", "avoid-column")
        .style("pageBreakInside", "avoid")
        .style("float", "footnote");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.background_image.as_deref(), Some("url('/hero.png')"));
    assert_eq!(style.background_position.as_deref(), Some("center top"));
    assert_eq!(style.background_size.as_deref(), Some("cover"));
    assert_eq!(style.background_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(
        style.background_attachment,
        Some(BackgroundAttachment::Fixed)
    );
    assert_eq!(style.background_origin, Some(BackgroundBox::ContentBox));
    assert_eq!(style.background_clip, Some(BackgroundBox::PaddingBox));
    assert_eq!(style.image_rendering.as_deref(), Some("pixelated"));
    assert_eq!(style.image_orientation.as_deref(), Some("from-image"));
    assert_eq!(style.image_resolution.as_deref(), Some("300dpi"));
    assert_eq!(style.object_fit, Some(ObjectFit::Cover));
    assert_eq!(style.object_position.as_deref(), Some("left bottom"));
    assert_eq!(
        style.shape_outside.as_deref(),
        Some("circle(50% at 50% 50%)")
    );
    assert_eq!(
        style.shape_inside.as_deref(),
        Some("polygon(0 0, 100% 0, 100% 100%)")
    );
    assert_eq!(style.shape_margin, Some(StyleLength::Points(32.0)));
    assert_eq!(style.shape_padding, Some(StyleLength::Points(24.0)));
    assert_eq!(style.shape_image_threshold, Some(0.65));
    assert_eq!(style.list_style_type.as_deref(), Some("disc"));
    assert_eq!(style.list_style_position, Some(ListStylePosition::Inside));
    assert_eq!(
        style.list_style_image.as_deref(),
        Some("url('/marker.svg')")
    );
    assert_eq!(style.marker_side.as_deref(), Some("match-parent"));
    assert_eq!(style.columns.as_deref(), Some("3"));
    assert_eq!(style.column_count.as_deref(), Some("2"));
    assert_eq!(style.column_width, Some(StyleLength::Points(192.0)));
    assert_eq!(style.column_rule.as_deref(), Some("2px dashed #663399"));
    assert_eq!(style.column_rule_width, Some(StyleLength::Points(4.0)));
    assert_eq!(style.column_rule_style, Some(BorderStyle::Dotted));
    assert_eq!(
        style.column_rule_color,
        Some(StyleColor::Rgba {
            red: 10,
            green: 20,
            blue: 30,
            alpha: 255,
        })
    );
    assert_eq!(style.column_span.as_deref(), Some("all"));
    assert_eq!(style.column_fill.as_deref(), Some("balance"));
    assert_eq!(style.page_size.as_deref(), Some("A4 landscape"));
    assert_eq!(style.page.as_deref(), Some("chapter"));
    assert_eq!(style.page_orientation.as_deref(), Some("rotate-left"));
    assert_eq!(style.bleed.as_deref(), Some("6pt"));
    assert_eq!(style.marks.as_deref(), Some("crop cross"));
    assert_eq!(style.orphans.as_deref(), Some("3"));
    assert_eq!(style.widows.as_deref(), Some("4"));
    assert_eq!(style.bookmark_label.as_deref(), Some("content(text)"));
    assert_eq!(style.bookmark_level.as_deref(), Some("2"));
    assert_eq!(style.bookmark_state.as_deref(), Some("open"));
    assert_eq!(style.footnote_display.as_deref(), Some("block"));
    assert_eq!(style.footnote_policy.as_deref(), Some("line"));
    assert_eq!(style.break_before.as_deref(), Some("page"));
    assert_eq!(style.break_after.as_deref(), Some("avoid-column"));
    assert_eq!(style.break_inside.as_deref(), Some("avoid"));
    assert_eq!(style.float, Some(FloatMode::Footnote));
    assert!(!style.unsupported.contains_key("background-image"));
    assert!(!style.unsupported.contains_key("image-rendering"));
    assert!(!style.unsupported.contains_key("image-orientation"));
    assert!(!style.unsupported.contains_key("image-resolution"));
    assert!(!style.unsupported.contains_key("object-fit"));
    assert!(!style.unsupported.contains_key("shape-outside"));
    assert!(!style.unsupported.contains_key("shape-inside"));
    assert!(!style.unsupported.contains_key("shape-margin"));
    assert!(!style.unsupported.contains_key("shape-padding"));
    assert!(!style.unsupported.contains_key("shape-image-threshold"));
    assert!(!style.unsupported.contains_key("list-style-image"));
    assert!(!style.unsupported.contains_key("marker-side"));
    assert!(!style.unsupported.contains_key("column-rule"));
    assert!(!style.unsupported.contains_key("size"));
    assert!(!style.unsupported.contains_key("page"));
    assert!(!style.unsupported.contains_key("page-orientation"));
    assert!(!style.unsupported.contains_key("bleed"));
    assert!(!style.unsupported.contains_key("marks"));
    assert!(!style.unsupported.contains_key("orphans"));
    assert!(!style.unsupported.contains_key("widows"));
    assert!(!style.unsupported.contains_key("bookmark-label"));
    assert!(!style.unsupported.contains_key("bookmark-level"));
    assert!(!style.unsupported.contains_key("bookmark-state"));
    assert!(!style.unsupported.contains_key("footnote-display"));
    assert!(!style.unsupported.contains_key("footnote-policy"));
    assert!(!style.unsupported.contains_key("page-break-inside"));
}

#[test]
fn parses_tailwind_background_object_list_and_columns_utilities() {
    let web = WebProps::new().class_name(
        "bg-[url('/hero.png')] bg-cover bg-center bg-no-repeat bg-fixed \
             bg-origin-content bg-clip-padding object-cover object-left-bottom \
             [image-rendering:pixelated] [image-orientation:from-image] \
             [image-resolution:300dpi] [shape-outside:circle(50%_at_50%_50%)] \
             [shape-inside:polygon(0_0,100%_0,100%_100%)] \
             [shape-margin:2rem] [shape-padding:1.5rem] [shape-image-threshold:65%] \
             list-inside list-disc list-image-[url('/marker.svg')] [marker-side:match-parent] \
             columns-3 \
             [size:A4_landscape] [page:chapter] [page-orientation:rotate-left] \
             [bleed:6pt] [marks:crop_cross] [orphans:3] [widows:4] \
             [bookmark-label:content(text)] [bookmark-level:2] [bookmark-state:open] \
             [footnote-display:block] [footnote-policy:line] [float:footnote] \
             break-before-page break-after-avoid-column \
             break-inside-avoid md:bg-[length:50%_auto] hover:object-[25%_75%] \
             md:list-image-(--marker-image) hover:list-image-none \
             focus:break-before-[recto] lg:break-inside-(--break-inside) \
             hover:[image-rendering:crisp-edges] md:[image-resolution:from-image] \
             focus:[shape-outside:inset(10px)] active:[shape-margin:calc(1rem_+_2px)] \
             focus:[shape-inside:circle(40%)] active:[shape-padding:calc(1rem_+_2px)] \
             before:[shape-image-threshold:0.25] hover:[page:appendix] \
             focus:[size:letter] active:[page-orientation:upright] marker:[bleed:3mm] \
             print:[marks:none] focus:[orphans:2] active:[widows:5] marker:[marker-side:list-item] \
             hover:[bookmark-label:attr(title)] focus:[bookmark-state:closed] \
             active:[footnote-policy:block] before:[float:inline-start]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.background_image.as_deref(), Some("url('/hero.png')"));
    assert_eq!(style.background_size.as_deref(), Some("cover"));
    assert_eq!(style.background_position.as_deref(), Some("center"));
    assert_eq!(style.background_repeat.as_deref(), Some("no-repeat"));
    assert_eq!(
        style.background_attachment,
        Some(BackgroundAttachment::Fixed)
    );
    assert_eq!(style.background_origin, Some(BackgroundBox::ContentBox));
    assert_eq!(style.background_clip, Some(BackgroundBox::PaddingBox));
    assert_eq!(style.image_rendering.as_deref(), Some("pixelated"));
    assert_eq!(style.image_orientation.as_deref(), Some("from-image"));
    assert_eq!(style.image_resolution.as_deref(), Some("300dpi"));
    assert_eq!(style.object_fit, Some(ObjectFit::Cover));
    assert_eq!(style.object_position.as_deref(), Some("left bottom"));
    assert_eq!(
        style.shape_outside.as_deref(),
        Some("circle(50% at 50% 50%)")
    );
    assert_eq!(
        style.shape_inside.as_deref(),
        Some("polygon(0 0,100% 0,100% 100%)")
    );
    assert_eq!(style.shape_margin, Some(StyleLength::Points(32.0)));
    assert_eq!(style.shape_padding, Some(StyleLength::Points(24.0)));
    assert_eq!(style.shape_image_threshold, Some(0.65));
    assert_eq!(style.list_style_position, Some(ListStylePosition::Inside));
    assert_eq!(style.list_style_type.as_deref(), Some("disc"));
    assert_eq!(
        style.list_style_image.as_deref(),
        Some("url('/marker.svg')")
    );
    assert_eq!(style.marker_side.as_deref(), Some("match-parent"));
    assert_eq!(style.columns.as_deref(), Some("3"));
    assert_eq!(style.page_size.as_deref(), Some("A4 landscape"));
    assert_eq!(style.page.as_deref(), Some("chapter"));
    assert_eq!(style.page_orientation.as_deref(), Some("rotate-left"));
    assert_eq!(style.bleed.as_deref(), Some("6pt"));
    assert_eq!(style.marks.as_deref(), Some("crop cross"));
    assert_eq!(style.orphans.as_deref(), Some("3"));
    assert_eq!(style.widows.as_deref(), Some("4"));
    assert_eq!(style.bookmark_label.as_deref(), Some("content(text)"));
    assert_eq!(style.bookmark_level.as_deref(), Some("2"));
    assert_eq!(style.bookmark_state.as_deref(), Some("open"));
    assert_eq!(style.footnote_display.as_deref(), Some("block"));
    assert_eq!(style.footnote_policy.as_deref(), Some("line"));
    assert_eq!(style.break_before.as_deref(), Some("page"));
    assert_eq!(style.break_after.as_deref(), Some("avoid-column"));
    assert_eq!(style.break_inside.as_deref(), Some("avoid"));
    assert_eq!(style.float, Some(FloatMode::Footnote));
    assert_eq!(
        style
            .declarations
            .get("background-image")
            .map(String::as_str),
        Some("url('/hero.png')")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("list-style-image"))
            .map(String::as_str),
        Some("var(--marker-image)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("list-style-image"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("marker")
            .and_then(|styles| styles.get("marker-side"))
            .map(String::as_str),
        Some("list-item")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("bookmark-label"))
            .map(String::as_str),
        Some("attr(title)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("bookmark-state"))
            .map(String::as_str),
        Some("closed")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("footnote-policy"))
            .map(String::as_str),
        Some("block")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("float"))
            .map(String::as_str),
        Some("inline-start")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("background-size"))
            .map(String::as_str),
        Some("50% auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("object-position"))
            .map(String::as_str),
        Some("25% 75%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("image-rendering"))
            .map(String::as_str),
        Some("crisp-edges")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("image-resolution"))
            .map(String::as_str),
        Some("from-image")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("shape-outside"))
            .map(String::as_str),
        Some("inset(10px)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("shape-margin"))
            .map(String::as_str),
        Some("calc(1rem + 2px)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("shape-inside"))
            .map(String::as_str),
        Some("circle(40%)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("shape-padding"))
            .map(String::as_str),
        Some("calc(1rem + 2px)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("shape-image-threshold"))
            .map(String::as_str),
        Some("0.25")
    );
    assert!(!style.unsupported.contains_key("image-rendering"));
    assert!(!style.unsupported.contains_key("image-orientation"));
    assert!(!style.unsupported.contains_key("image-resolution"));
    assert!(!style.unsupported.contains_key("shape-outside"));
    assert!(!style.unsupported.contains_key("shape-inside"));
    assert!(!style.unsupported.contains_key("shape-margin"));
    assert!(!style.unsupported.contains_key("shape-padding"));
    assert!(!style.unsupported.contains_key("shape-image-threshold"));
    assert!(!style.unsupported.contains_key("size"));
    assert!(!style.unsupported.contains_key("page-orientation"));
    assert!(!style.unsupported.contains_key("bleed"));
    assert!(!style.unsupported.contains_key("marks"));
    assert!(!style.unsupported.contains_key("page"));
    assert!(!style.unsupported.contains_key("orphans"));
    assert!(!style.unsupported.contains_key("widows"));
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("break-before"))
            .map(String::as_str),
        Some("recto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("lg")
            .and_then(|styles| styles.get("break-inside"))
            .map(String::as_str),
        Some("var(--break-inside)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("page"))
            .map(String::as_str),
        Some("appendix")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("size"))
            .map(String::as_str),
        Some("letter")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("page-orientation"))
            .map(String::as_str),
        Some("upright")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("marker")
            .and_then(|styles| styles.get("bleed"))
            .map(String::as_str),
        Some("3mm")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("print")
            .and_then(|styles| styles.get("marks"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("orphans"))
            .map(String::as_str),
        Some("2")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("widows"))
            .map(String::as_str),
        Some("5")
    );
}

#[test]
fn preserves_css_length_expressions_as_portable_tokens() {
    let web = WebProps::new()
        .style("width", "calc(100% - 2rem)")
        .style("height", "calc-size(auto, size + 2rem)")
        .style("minWidth", "min-content")
        .style("maxHeight", "clamp(240px, 50vh, 640px)")
        .style("inlineSize", "round(nearest, 100%, 1px)")
        .style("blockSize", "hypot(3px, 4px)")
        .style("interpolateSize", "allow-keywords")
        .style("gap", "var(--space)")
        .style("marginTop", "abs(-2rem)")
        .style("borderWidth", "fit-content");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.width,
        Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
    );
    assert_eq!(
        style.height,
        Some(StyleLength::Css("calc-size(auto, size + 2rem)".to_string()))
    );
    assert_eq!(
        style.min_width,
        Some(StyleLength::Css("min-content".to_string()))
    );
    assert_eq!(
        style.max_height,
        Some(StyleLength::Css("clamp(240px, 50vh, 640px)".to_string()))
    );
    assert_eq!(
        style.inline_size,
        Some(StyleLength::Css("round(nearest, 100%, 1px)".to_string()))
    );
    assert_eq!(
        style.block_size,
        Some(StyleLength::Css("hypot(3px, 4px)".to_string()))
    );
    assert_eq!(
        style.gap,
        Some(StyleLength::Css("var(--space)".to_string()))
    );
    assert_eq!(
        style.margin.top,
        Some(StyleLength::Css("abs(-2rem)".to_string()))
    );
    assert_eq!(
        style.border_width.top,
        Some(StyleLength::Css("fit-content".to_string()))
    );
    assert_eq!(style.interpolate_size.as_deref(), Some("allow-keywords"));
    assert!(!style.unsupported.contains_key("interpolate-size"));
}

#[test]
fn preserves_tailwind_arbitrary_css_length_expressions() {
    let web = WebProps::new().class_name(
        "w-[calc(100%_-_2rem)] h-[calc-size(auto,size_+_2rem)] min-w-[min-content] \
             max-h-[clamp(240px,_50vh,_640px)] gap-[var(--space)] \
             [inline-size:round(nearest,100%,1px)] [block-size:hypot(3px,4px)] \
             [margin-top:abs(-2rem)] \
             [interpolate-size:allow-keywords] hover:[interpolate-size:numeric-only]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.width,
        Some(StyleLength::Css("calc(100% - 2rem)".to_string()))
    );
    assert_eq!(
        style.height,
        Some(StyleLength::Css("calc-size(auto,size + 2rem)".to_string()))
    );
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
        style.inline_size,
        Some(StyleLength::Css("round(nearest,100%,1px)".to_string()))
    );
    assert_eq!(
        style.block_size,
        Some(StyleLength::Css("hypot(3px,4px)".to_string()))
    );
    assert_eq!(
        style.margin.top,
        Some(StyleLength::Css("abs(-2rem)".to_string()))
    );
    assert_eq!(
        style.declarations.get("width").map(String::as_str),
        Some("calc(100% - 2rem)")
    );
    assert_eq!(style.interpolate_size.as_deref(), Some("allow-keywords"));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("interpolate-size"))
            .map(String::as_str),
        Some("numeric-only")
    );
}

#[test]
fn preserves_tailwind_arbitrary_escaped_and_url_underscores() {
    let web = WebProps::new().class_name(
        "w-[var(--space\\_lg)] bg-[url('/what_a_rush.png')] \
             list-image-[url('/marker_icon.svg')] \
             hover:bg-[image:linear-gradient(to_right,red,blue)]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.width,
        Some(StyleLength::Css("var(--space_lg)".to_string()))
    );
    assert_eq!(
        style.background_image.as_deref(),
        Some("url('/what_a_rush.png')")
    );
    assert_eq!(
        style.list_style_image.as_deref(),
        Some("url('/marker_icon.svg')")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("background-image"))
            .map(String::as_str),
        Some("linear-gradient(to right,red,blue)")
    );
}

#[test]
fn parses_css_color_functions_and_alpha_syntax() {
    let web = WebProps::new()
        .style("color", "hsl(210 50% 40% / 50%)")
        .style("backgroundColor", "rgb(10 20 30 / 25%)")
        .style("borderColor", "HSLA(120, 100%, 25%, 0.75)");

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
fn preserves_modern_css_color_functions() {
    let web = WebProps::new()
        .style("color", "oklch(70% 0.2 260)")
        .style("background", "color-mix(in srgb, red 40%, blue)")
        .style("outlineColor", "light-dark(#111, #eee)")
        .style("caretColor", "lab(50% 20 30)")
        .style("divideColor", "alpha(from rebeccapurple / 50%)")
        .style("textDecorationColor", "device-cmyk(0% 81% 81% 30%)");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.color,
        Some(StyleColor::Function("oklch(70% 0.2 260)".to_string()))
    );
    assert_eq!(
        style.background_color,
        Some(StyleColor::Function(
            "color-mix(in srgb, red 40%, blue)".to_string()
        ))
    );
    assert_eq!(
        style.outline_color,
        Some(StyleColor::Function("light-dark(#111, #eee)".to_string()))
    );
    assert_eq!(
        style.caret_color,
        Some(StyleColor::Function("lab(50% 20 30)".to_string()))
    );
    assert_eq!(
        style.divide_color,
        Some(StyleColor::Function(
            "alpha(from rebeccapurple / 50%)".to_string()
        ))
    );
    assert_eq!(
        style.text_decoration_color,
        Some(StyleColor::Function(
            "device-cmyk(0% 81% 81% 30%)".to_string()
        ))
    );
    assert_eq!(
        style.declarations.get("background").map(String::as_str),
        Some("color-mix(in srgb, red 40%, blue)")
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
fn preserves_tailwind_arbitrary_color_functions() {
    let web = WebProps::new().class_name(
        "bg-[oklch(70%_0.2_260)]/50 text-[color-mix(in_srgb,red_40%,blue)] \
             caret-[light-dark(#111,#eee)] hover:border-[lab(50%_20_30)]/25",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.background_color,
        Some(StyleColor::Function(
            "color-mix(in srgb, oklch(70% 0.2 260) 50%, transparent)".to_string()
        ))
    );
    assert_eq!(
        style.color,
        Some(StyleColor::Function(
            "color-mix(in srgb,red 40%,blue)".to_string()
        ))
    );
    assert_eq!(
        style.caret_color,
        Some(StyleColor::Function("light-dark(#111,#eee)".to_string()))
    );
    assert_eq!(
        style
            .declarations
            .get("background-color")
            .map(String::as_str),
        Some("color-mix(in srgb, oklch(70% 0.2 260) 50%, transparent)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("border-color"))
            .map(String::as_str),
        Some("color-mix(in srgb, lab(50% 20 30) 25%, transparent)")
    );
}

#[test]
fn parses_css_speech_properties() {
    let web = WebProps::new()
        .style("speak", "auto")
        .style("speakAs", "spell-out")
        .style("pause", "200ms 400ms")
        .style("pauseBefore", "250ms")
        .style("pauseAfter", "500ms")
        .style("rest", "weak medium")
        .style("restBefore", "strong")
        .style("restAfter", "2s")
        .style("cue", "url('/open.ogg') none")
        .style("cueBefore", "url('/before.ogg')")
        .style("cueAfter", "none")
        .style("voiceFamily", "male 1")
        .style("voiceBalance", "left")
        .style("voiceDuration", "auto")
        .style("voicePitch", "medium")
        .style("voiceRange", "high")
        .style("voiceRate", "fast")
        .style("voiceStress", "strong")
        .style("voiceVolume", "x-loud");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.speak.as_deref(), Some("auto"));
    assert_eq!(style.speak_as.as_deref(), Some("spell-out"));
    assert_eq!(style.pause.as_deref(), Some("200ms 400ms"));
    assert_eq!(style.pause_before.as_deref(), Some("250ms"));
    assert_eq!(style.pause_after.as_deref(), Some("500ms"));
    assert_eq!(style.rest.as_deref(), Some("weak medium"));
    assert_eq!(style.rest_before.as_deref(), Some("strong"));
    assert_eq!(style.rest_after.as_deref(), Some("2s"));
    assert_eq!(style.cue.as_deref(), Some("url('/open.ogg') none"));
    assert_eq!(style.cue_before.as_deref(), Some("url('/before.ogg')"));
    assert_eq!(style.cue_after.as_deref(), Some("none"));
    assert_eq!(style.voice_family.as_deref(), Some("male 1"));
    assert_eq!(style.voice_balance.as_deref(), Some("left"));
    assert_eq!(style.voice_duration.as_deref(), Some("auto"));
    assert_eq!(style.voice_pitch.as_deref(), Some("medium"));
    assert_eq!(style.voice_range.as_deref(), Some("high"));
    assert_eq!(style.voice_rate.as_deref(), Some("fast"));
    assert_eq!(style.voice_stress.as_deref(), Some("strong"));
    assert_eq!(style.voice_volume.as_deref(), Some("x-loud"));
    assert!(!style.unsupported.contains_key("speak"));
    assert!(!style.unsupported.contains_key("speak-as"));
    assert!(!style.unsupported.contains_key("pause"));
    assert!(!style.unsupported.contains_key("pause-before"));
    assert!(!style.unsupported.contains_key("pause-after"));
    assert!(!style.unsupported.contains_key("rest"));
    assert!(!style.unsupported.contains_key("rest-before"));
    assert!(!style.unsupported.contains_key("rest-after"));
    assert!(!style.unsupported.contains_key("cue"));
    assert!(!style.unsupported.contains_key("cue-before"));
    assert!(!style.unsupported.contains_key("cue-after"));
    assert!(!style.unsupported.contains_key("voice-family"));
    assert!(!style.unsupported.contains_key("voice-balance"));
    assert!(!style.unsupported.contains_key("voice-duration"));
    assert!(!style.unsupported.contains_key("voice-pitch"));
    assert!(!style.unsupported.contains_key("voice-range"));
    assert!(!style.unsupported.contains_key("voice-rate"));
    assert!(!style.unsupported.contains_key("voice-stress"));
    assert!(!style.unsupported.contains_key("voice-volume"));
}

#[test]
fn parses_tailwind_arbitrary_speech_properties() {
    let web = WebProps::new().class_name(
        "[speak:auto] [speak-as:spell-out] [pause:200ms_400ms] \
             [pause-before:250ms] [pause-after:500ms] [rest:weak_medium] \
             [rest-before:strong] [rest-after:2s] [cue:url('/open.ogg')_none] \
             [cue-before:url('/before.ogg')] [cue-after:none] \
             [voice-family:male_1] [voice-balance:left] [voice-duration:auto] \
             [voice-pitch:medium] [voice-range:high] [voice-rate:fast] \
             [voice-stress:strong] [voice-volume:x-loud] \
             hover:[speak:never] focus:[voice-rate:slow] active:[cue-before:none]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.speak.as_deref(), Some("auto"));
    assert_eq!(style.speak_as.as_deref(), Some("spell-out"));
    assert_eq!(style.pause.as_deref(), Some("200ms 400ms"));
    assert_eq!(style.pause_before.as_deref(), Some("250ms"));
    assert_eq!(style.pause_after.as_deref(), Some("500ms"));
    assert_eq!(style.rest.as_deref(), Some("weak medium"));
    assert_eq!(style.rest_before.as_deref(), Some("strong"));
    assert_eq!(style.rest_after.as_deref(), Some("2s"));
    assert_eq!(style.cue.as_deref(), Some("url('/open.ogg') none"));
    assert_eq!(style.cue_before.as_deref(), Some("url('/before.ogg')"));
    assert_eq!(style.cue_after.as_deref(), Some("none"));
    assert_eq!(style.voice_family.as_deref(), Some("male 1"));
    assert_eq!(style.voice_balance.as_deref(), Some("left"));
    assert_eq!(style.voice_duration.as_deref(), Some("auto"));
    assert_eq!(style.voice_pitch.as_deref(), Some("medium"));
    assert_eq!(style.voice_range.as_deref(), Some("high"));
    assert_eq!(style.voice_rate.as_deref(), Some("fast"));
    assert_eq!(style.voice_stress.as_deref(), Some("strong"));
    assert_eq!(style.voice_volume.as_deref(), Some("x-loud"));
    assert!(!style.unsupported.contains_key("speak"));
    assert!(!style.unsupported.contains_key("voice-rate"));
    assert!(!style.unsupported.contains_key("cue-before"));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("speak"))
            .map(String::as_str),
        Some("never")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("voice-rate"))
            .map(String::as_str),
        Some("slow")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("cue-before"))
            .map(String::as_str),
        Some("none")
    );
}

#[test]
fn parses_svg_presentation_properties_into_portable_tokens() {
    let web = WebProps::new()
        .style("fill", "#663399")
        .style("fillOpacity", "50%")
        .style("fillRule", "evenodd")
        .style("clipRule", "nonzero")
        .style("stroke", "currentColor")
        .style("strokeWidth", "2")
        .style("strokeLinecap", "round")
        .style("strokeLinejoin", "bevel")
        .style("strokeMiterlimit", "4")
        .style("strokeDasharray", "2 4")
        .style("strokeDashoffset", "1px")
        .style("strokeOpacity", "0.25")
        .style("vectorEffect", "non-scaling-stroke")
        .style("paintOrder", "stroke fill markers")
        .style("shapeRendering", "geometricPrecision")
        .style("textRendering", "optimizeLegibility")
        .style("colorRendering", "optimizeQuality")
        .style("colorInterpolation", "sRGB")
        .style("colorInterpolationFilters", "linearRGB")
        .style("marker", "url(#dot)")
        .style("markerStart", "url(#start)")
        .style("markerMid", "url(#mid)")
        .style("markerEnd", "url(#end)")
        .style("stopColor", "#ff0000")
        .style("stopOpacity", "75%")
        .style("floodColor", "rgb(10 20 30)")
        .style("floodOpacity", "0.25")
        .style("lightingColor", "currentColor")
        .style("pointerEvents", "visiblePainted");

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.fill,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(style.fill_opacity, Some(0.5));
    assert_eq!(style.fill_rule, Some(FillRule::Evenodd));
    assert_eq!(style.clip_rule, Some(FillRule::Nonzero));
    assert_eq!(
        style.stroke,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.stroke_width, Some(StyleLength::Points(2.0)));
    assert_eq!(style.stroke_linecap, Some(StrokeLineCap::Round));
    assert_eq!(style.stroke_linejoin, Some(StrokeLineJoin::Bevel));
    assert_eq!(style.stroke_miterlimit.as_deref(), Some("4"));
    assert_eq!(style.stroke_dasharray.as_deref(), Some("2 4"));
    assert_eq!(style.stroke_dashoffset, Some(StyleLength::Points(1.0)));
    assert_eq!(style.stroke_opacity, Some(0.25));
    assert_eq!(style.vector_effect.as_deref(), Some("non-scaling-stroke"));
    assert_eq!(style.paint_order.as_deref(), Some("stroke fill markers"));
    assert_eq!(style.shape_rendering.as_deref(), Some("geometricPrecision"));
    assert_eq!(style.text_rendering.as_deref(), Some("optimizeLegibility"));
    assert_eq!(style.color_rendering.as_deref(), Some("optimizeQuality"));
    assert_eq!(style.color_interpolation.as_deref(), Some("sRGB"));
    assert_eq!(
        style.color_interpolation_filters.as_deref(),
        Some("linearRGB")
    );
    assert_eq!(style.marker.as_deref(), Some("url(#dot)"));
    assert_eq!(style.marker_start.as_deref(), Some("url(#start)"));
    assert_eq!(style.marker_mid.as_deref(), Some("url(#mid)"));
    assert_eq!(style.marker_end.as_deref(), Some("url(#end)"));
    assert_eq!(
        style.stop_color,
        Some(StyleColor::Rgba {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        })
    );
    assert_eq!(style.stop_opacity, Some(0.75));
    assert_eq!(
        style.flood_color,
        Some(StyleColor::Rgba {
            red: 10,
            green: 20,
            blue: 30,
            alpha: 255,
        })
    );
    assert_eq!(style.flood_opacity, Some(0.25));
    assert_eq!(
        style.lighting_color,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.pointer_events, Some(PointerEvents::VisiblePainted));
    assert!(!style.unsupported.contains_key("fill-rule"));
    assert!(!style.unsupported.contains_key("stroke-width"));
    assert!(!style.unsupported.contains_key("color-rendering"));
    assert!(!style.unsupported.contains_key("marker-start"));
    assert!(!style.unsupported.contains_key("stop-color"));
    assert!(!style.unsupported.contains_key("flood-color"));
    assert!(!style.unsupported.contains_key("lighting-color"));
}

#[test]
fn parses_tailwind_svg_presentation_utilities() {
    let web = WebProps::new().class_name(
        "fill-[#663399]/50 stroke-current stroke-2 hover:fill-none \
             [color-rendering:optimizeQuality] [marker:url(#dot)] \
             [marker-start:url(#start)] [marker-mid:url(#mid)] [marker-end:url(#end)] \
             [stop-color:#ff0000] [stop-opacity:75%] \
             [flood-color:rgb(10_20_30)] [flood-opacity:0.25] \
             [lighting-color:currentColor] [pointer-events:visiblePainted] \
             md:stroke-[3px] focus:stroke-[#ff0000] active:fill-(--icon-fill) \
             hover:[marker-end:url(#hover)] focus:[stop-color:#00ff00] \
             active:[flood-opacity:50%] visited:[pointer-events:bounding-box]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.fill,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(
        style.stroke,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.stroke_width, Some(StyleLength::Points(2.0)));
    assert_eq!(
        style.declarations.get("fill").map(String::as_str),
        Some("rgba(102, 51, 153, 0.5)")
    );
    assert_eq!(
        style.declarations.get("stroke-width").map(String::as_str),
        Some("2")
    );
    assert_eq!(style.color_rendering.as_deref(), Some("optimizeQuality"));
    assert_eq!(style.marker.as_deref(), Some("url(#dot)"));
    assert_eq!(style.marker_start.as_deref(), Some("url(#start)"));
    assert_eq!(style.marker_mid.as_deref(), Some("url(#mid)"));
    assert_eq!(style.marker_end.as_deref(), Some("url(#end)"));
    assert_eq!(
        style.stop_color,
        Some(StyleColor::Rgba {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 255,
        })
    );
    assert_eq!(style.stop_opacity, Some(0.75));
    assert_eq!(
        style.flood_color,
        Some(StyleColor::Rgba {
            red: 10,
            green: 20,
            blue: 30,
            alpha: 255,
        })
    );
    assert_eq!(style.flood_opacity, Some(0.25));
    assert_eq!(
        style.lighting_color,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.pointer_events, Some(PointerEvents::VisiblePainted));
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("fill"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("stroke-width"))
            .map(String::as_str),
        Some("3px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("stroke"))
            .map(String::as_str),
        Some("#ff0000")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("fill"))
            .map(String::as_str),
        Some("var(--icon-fill)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("marker-end"))
            .map(String::as_str),
        Some("url(#hover)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("stop-color"))
            .map(String::as_str),
        Some("#00ff00")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("flood-opacity"))
            .map(String::as_str),
        Some("50%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("visited")
            .and_then(|styles| styles.get("pointer-events"))
            .map(String::as_str),
        Some("bounding-box")
    );
}

#[test]
fn parses_css_visual_effect_and_interaction_properties() {
    let web = WebProps::new()
        .style("boxShadow", "0 2px 8px rgb(0 0 0 / 25%)")
        .style("outline", "2px dashed #ff0000")
        .style("outlineOffset", "4px")
        .style("transform", "translateX(4px) rotate(15deg)")
        .style("transformBox", "fill-box")
        .style("offset", "path('M 0 0 L 100 0') 40% auto")
        .style("offsetPath", "ray(45deg closest-side)")
        .style("offsetDistance", "40%")
        .style("offsetRotate", "auto 90deg")
        .style("offsetAnchor", "center")
        .style("offsetPosition", "left top")
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
    assert_eq!(style.transform_box.as_deref(), Some("fill-box"));
    assert_eq!(
        style.offset.as_deref(),
        Some("path('M 0 0 L 100 0') 40% auto")
    );
    assert_eq!(
        style.offset_path.as_deref(),
        Some("ray(45deg closest-side)")
    );
    assert_eq!(style.offset_distance.as_deref(), Some("40%"));
    assert_eq!(style.offset_rotate.as_deref(), Some("auto 90deg"));
    assert_eq!(style.offset_anchor.as_deref(), Some("center"));
    assert_eq!(style.offset_position.as_deref(), Some("left top"));
    assert_eq!(style.filter.as_deref(), Some("blur(4px)"));
    assert_eq!(style.backdrop_filter.as_deref(), Some("saturate(150%)"));
    assert_eq!(style.aspect_ratio.as_deref(), Some("4 / 3"));
    assert_eq!(style.cursor.as_deref(), Some("pointer"));
    assert_eq!(style.pointer_events, Some(PointerEvents::None));
    assert_eq!(style.user_select, Some(UserSelect::Text));
    assert!(!style.unsupported.contains_key("box-shadow"));
    assert!(!style.unsupported.contains_key("transform-box"));
    assert!(!style.unsupported.contains_key("offset"));
    assert!(!style.unsupported.contains_key("offset-path"));
    assert!(!style.unsupported.contains_key("offset-distance"));
    assert!(!style.unsupported.contains_key("offset-rotate"));
    assert!(!style.unsupported.contains_key("offset-anchor"));
    assert!(!style.unsupported.contains_key("offset-position"));
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
    assert_eq!(style.rotate.as_deref(), Some("45deg"));
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

#[test]
fn parses_composable_tailwind_transform_and_filter_utilities() {
    let web = WebProps::new().class_name(
        "translate-x-4 translate-y-2 scale-x-125 scale-y-75 -rotate-45 \
             rotate-x-12 rotate-y-[35deg] skew-x-6 transform-gpu origin-top-right \
             perspective-near backface-hidden blur-sm brightness-125 contrast-150 \
             grayscale hue-rotate-15 invert-0 saturate-200 sepia drop-shadow-md \
             backdrop-blur-md backdrop-brightness-75 backdrop-contrast-125 \
             backdrop-grayscale backdrop-hue-rotate-30 backdrop-invert \
             backdrop-opacity-50 backdrop-saturate-150 backdrop-sepia \
             [transform-box:fill-box] [offset:path('M_0_0_L_100_0')_40%_auto] \
             [offset-path:ray(45deg_closest-side)] [offset-distance:40%] \
             [offset-rotate:auto_90deg] [offset-anchor:center] [offset-position:left_top] \
             hover:blur-[2px] focus:translate-x-[calc(100%_-_1rem)] \
             md:[transform-box:view-box] hover:[offset-distance:75%] \
             focus:[offset-path:path('M_0_0_L_0_100')] active:[offset-rotate:reverse]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.translate.as_deref(), Some("16px 8px"));
    assert_eq!(style.scale.as_deref(), Some("125% 75%"));
    assert_eq!(style.rotate.as_deref(), Some("-45deg"));
    assert_eq!(
        style.transform.as_deref(),
        Some("translateZ(0) rotateX(12deg) rotateY(35deg) skewX(6deg)")
    );
    assert_eq!(style.transform_origin.as_deref(), Some("top right"));
    assert_eq!(style.transform_box.as_deref(), Some("fill-box"));
    assert_eq!(
        style.offset.as_deref(),
        Some("path('M 0 0 L 100 0') 40% auto")
    );
    assert_eq!(
        style.offset_path.as_deref(),
        Some("ray(45deg closest-side)")
    );
    assert_eq!(style.offset_distance.as_deref(), Some("40%"));
    assert_eq!(style.offset_rotate.as_deref(), Some("auto 90deg"));
    assert_eq!(style.offset_anchor.as_deref(), Some("center"));
    assert_eq!(style.offset_position.as_deref(), Some("left top"));
    assert_eq!(style.perspective, Some(StyleLength::Points(300.0)));
    assert_eq!(style.backface_visibility, Some(BackfaceVisibility::Hidden));
    assert_eq!(style.filter_blur.as_deref(), Some("blur(8px)"));
    assert_eq!(style.filter_brightness.as_deref(), Some("brightness(125%)"));
    assert_eq!(style.filter_contrast.as_deref(), Some("contrast(150%)"));
    assert_eq!(style.filter_grayscale.as_deref(), Some("grayscale(100%)"));
    assert_eq!(
        style.filter_hue_rotate.as_deref(),
        Some("hue-rotate(15deg)")
    );
    assert_eq!(style.filter_invert.as_deref(), Some("invert(0%)"));
    assert_eq!(style.filter_saturate.as_deref(), Some("saturate(200%)"));
    assert_eq!(style.filter_sepia.as_deref(), Some("sepia(100%)"));
    assert_eq!(
            style.filter.as_deref(),
            Some("blur(8px) brightness(125%) contrast(150%) drop-shadow(0 3px 3px rgb(0 0 0 / 0.12)) grayscale(100%) hue-rotate(15deg) invert(0%) saturate(200%) sepia(100%)")
        );
    assert_eq!(
            style.backdrop_filter.as_deref(),
            Some("blur(12px) brightness(75%) contrast(125%) grayscale(100%) hue-rotate(30deg) invert(100%) opacity(50%) saturate(150%) sepia(100%)")
        );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("--tw-blur"))
            .map(String::as_str),
        Some("blur(2px)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("filter"))
            .map(String::as_str),
        Some(tailwind_filter_pipeline().as_str())
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("translate"))
            .map(String::as_str),
        Some("var(--tw-translate-x) var(--tw-translate-y, 0)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("transform-box"))
            .map(String::as_str),
        Some("view-box")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("offset-distance"))
            .map(String::as_str),
        Some("75%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("offset-path"))
            .map(String::as_str),
        Some("path('M 0 0 L 0 100')")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("offset-rotate"))
            .map(String::as_str),
        Some("reverse")
    );
}

#[test]
fn parses_css_motion_scroll_and_interaction_properties() {
    let web = WebProps::new()
        .style("transition", "opacity 200ms ease-in")
        .style("transitionProperty", "opacity")
        .style("transitionDuration", "round(nearest, 1s, 100ms)")
        .style("transitionTimingFunction", "ease-in")
        .style("transitionDelay", "0.25s")
        .style("transitionBehavior", "allow-discrete")
        .style("overlay", "auto")
        .style("animation", "fade 1.5s ease-out both")
        .style("animationName", "fade")
        .style("animationDuration", "1.5s")
        .style("animationTimingFunction", "ease-out")
        .style("animationDelay", "var(--delay)")
        .style("animationIterationCount", "infinite")
        .style("animationDirection", "alternate")
        .style("animationFillMode", "both")
        .style("animationPlayState", "running")
        .style("animationComposition", "add")
        .style("animationTimeline", "--gallery")
        .style("animationRange", "entry 10% cover 80%")
        .style("animationRangeStart", "entry 10%")
        .style("animationRangeEnd", "cover 80%")
        .style("viewTransitionName", "card")
        .style("viewTransitionClass", "shared card")
        .style("viewTransitionGroup", "contain")
        .style("viewTransitionScope", "root")
        .style("willChange", "transform")
        .style("colorScheme", "light dark")
        .style("forcedColorAdjust", "none")
        .style("printColorAdjust", "exact")
        .style("WebkitPrintColorAdjust", "economy")
        .style("colorAdjust", "exact")
        .style("fieldSizing", "content")
        .style("appearance", "none")
        .style("accentColor", "#663399")
        .style("caretColor", "currentColor")
        .style("caret", "bar currentColor")
        .style("caretAnimation", "manual")
        .style("caretShape", "block")
        .style("resize", "horizontal")
        .style("scrollBehavior", "smooth")
        .style("scrollTimeline", "--scroller inline")
        .style("scrollTimelineName", "--scroller")
        .style("scrollTimelineAxis", "inline")
        .style("viewTimeline", "--card block")
        .style("viewTimelineName", "--card")
        .style("viewTimelineAxis", "block")
        .style("viewTimelineInset", "10% 20%")
        .style("timelineScope", "--gallery")
        .style("scrollMargin", "1px 2px")
        .style("scrollMarginTop", "12px")
        .style("scrollPadding", "4px 8px")
        .style("scrollPaddingInline", "10px")
        .style("scrollSnapType", "x mandatory")
        .style("scrollSnapAlign", "center")
        .style("scrollSnapStop", "always")
        .style("scrollInitialTarget", "nearest")
        .style("scrollStartTarget", "nearest")
        .style("scrollTargetGroup", "auto")
        .style("scrollMarkerGroup", "before")
        .style("scrollbarGutter", "stable both-edges")
        .style("scrollbarWidth", "thin")
        .style("scrollbarColor", "red blue")
        .style("overflowBlock", "clip")
        .style("overflowInline", "auto")
        .style("overflowClipMargin", "content-box 8px")
        .style("overflowAnchor", "none")
        .style("overscrollBehavior", "contain")
        .style("overscrollBehaviorX", "none")
        .style("overscrollBehaviorBlock", "contain")
        .style("overscrollBehaviorInline", "none")
        .style("touchAction", "pan-x pinch-zoom")
        .style("navUp", "#previous")
        .style("navRight", "auto")
        .style("navDown", "#next")
        .style("navLeft", "current")
        .style("spatialNavigationAction", "focus")
        .style("spatialNavigationContain", "contain")
        .style("spatialNavigationFunction", "grid")
        .style("interactivity", "inert");

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.transition.as_deref(), Some("opacity 200ms ease-in"));
    assert_eq!(style.transition_property.as_deref(), Some("opacity"));
    assert_eq!(
        style.transition_duration,
        Some(StyleTime::Css("round(nearest, 1s, 100ms)".to_string()))
    );
    assert_eq!(style.transition_timing_function.as_deref(), Some("ease-in"));
    assert_eq!(style.transition_delay, Some(StyleTime::Milliseconds(250.0)));
    assert_eq!(style.transition_behavior.as_deref(), Some("allow-discrete"));
    assert_eq!(style.overlay.as_deref(), Some("auto"));
    assert_eq!(style.animation.as_deref(), Some("fade 1.5s ease-out both"));
    assert_eq!(style.animation_name.as_deref(), Some("fade"));
    assert_eq!(
        style.animation_duration,
        Some(StyleTime::Milliseconds(1500.0))
    );
    assert_eq!(style.animation_timing_function.as_deref(), Some("ease-out"));
    assert_eq!(
        style.animation_delay,
        Some(StyleTime::Css("var(--delay)".to_string()))
    );
    assert_eq!(style.animation_iteration_count.as_deref(), Some("infinite"));
    assert_eq!(style.animation_direction.as_deref(), Some("alternate"));
    assert_eq!(style.animation_fill_mode.as_deref(), Some("both"));
    assert_eq!(style.animation_play_state.as_deref(), Some("running"));
    assert_eq!(style.animation_composition.as_deref(), Some("add"));
    assert_eq!(style.animation_timeline.as_deref(), Some("--gallery"));
    assert_eq!(
        style.animation_range.as_deref(),
        Some("entry 10% cover 80%")
    );
    assert_eq!(style.animation_range_start.as_deref(), Some("entry 10%"));
    assert_eq!(style.animation_range_end.as_deref(), Some("cover 80%"));
    assert_eq!(style.view_transition_name.as_deref(), Some("card"));
    assert_eq!(style.view_transition_class.as_deref(), Some("shared card"));
    assert_eq!(style.view_transition_group.as_deref(), Some("contain"));
    assert_eq!(style.view_transition_scope.as_deref(), Some("root"));
    assert_eq!(style.will_change.as_deref(), Some("transform"));
    assert_eq!(style.color_scheme.as_deref(), Some("light dark"));
    assert_eq!(style.forced_color_adjust.as_deref(), Some("none"));
    assert_eq!(style.print_color_adjust.as_deref(), Some("exact"));
    assert_eq!(style.color_adjust.as_deref(), Some("exact"));
    assert_eq!(style.field_sizing.as_deref(), Some("content"));
    assert_eq!(style.appearance.as_deref(), Some("none"));
    assert_eq!(
        style.accent_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 255,
        })
    );
    assert_eq!(
        style.caret_color,
        Some(StyleColor::Keyword("currentColor".to_string()))
    );
    assert_eq!(style.caret.as_deref(), Some("bar currentColor"));
    assert_eq!(style.caret_animation.as_deref(), Some("manual"));
    assert_eq!(style.caret_shape.as_deref(), Some("block"));
    assert_eq!(style.resize, Some(ResizeMode::Horizontal));
    assert_eq!(style.scroll_behavior, Some(ScrollBehavior::Smooth));
    assert_eq!(style.scroll_timeline.as_deref(), Some("--scroller inline"));
    assert_eq!(style.scroll_timeline_name.as_deref(), Some("--scroller"));
    assert_eq!(style.scroll_timeline_axis.as_deref(), Some("inline"));
    assert_eq!(style.view_timeline.as_deref(), Some("--card block"));
    assert_eq!(style.view_timeline_name.as_deref(), Some("--card"));
    assert_eq!(style.view_timeline_axis.as_deref(), Some("block"));
    assert_eq!(style.view_timeline_inset.as_deref(), Some("10% 20%"));
    assert_eq!(style.timeline_scope.as_deref(), Some("--gallery"));
    assert_eq!(style.scroll_margin.top, Some(StyleLength::Points(12.0)));
    assert_eq!(style.scroll_margin.right, Some(StyleLength::Points(2.0)));
    assert_eq!(style.scroll_margin.bottom, Some(StyleLength::Points(1.0)));
    assert_eq!(style.scroll_margin.left, Some(StyleLength::Points(2.0)));
    assert_eq!(style.scroll_padding.top, Some(StyleLength::Points(4.0)));
    assert_eq!(style.scroll_padding.right, Some(StyleLength::Points(10.0)));
    assert_eq!(style.scroll_padding.bottom, Some(StyleLength::Points(4.0)));
    assert_eq!(style.scroll_padding.left, Some(StyleLength::Points(10.0)));
    assert_eq!(style.scroll_snap_type.as_deref(), Some("x mandatory"));
    assert_eq!(style.scroll_snap_align.as_deref(), Some("center"));
    assert_eq!(style.scroll_snap_stop.as_deref(), Some("always"));
    assert_eq!(style.scroll_initial_target.as_deref(), Some("nearest"));
    assert_eq!(style.scroll_target_group.as_deref(), Some("auto"));
    assert_eq!(style.scroll_marker_group.as_deref(), Some("before"));
    assert_eq!(style.scrollbar_gutter.as_deref(), Some("stable both-edges"));
    assert_eq!(style.scrollbar_width.as_deref(), Some("thin"));
    assert_eq!(style.scrollbar_color.as_deref(), Some("red blue"));
    assert_eq!(style.overflow_block, Some(OverflowMode::Clip));
    assert_eq!(style.overflow_inline, Some(OverflowMode::Auto));
    assert_eq!(
        style.overflow_clip_margin.as_deref(),
        Some("content-box 8px")
    );
    assert_eq!(style.overflow_anchor.as_deref(), Some("none"));
    assert_eq!(style.overscroll_behavior_x, Some(OverscrollBehavior::None));
    assert_eq!(
        style.overscroll_behavior_y,
        Some(OverscrollBehavior::Contain)
    );
    assert_eq!(
        style.overscroll_behavior_block,
        Some(OverscrollBehavior::Contain)
    );
    assert_eq!(
        style.overscroll_behavior_inline,
        Some(OverscrollBehavior::None)
    );
    assert_eq!(style.touch_action.as_deref(), Some("pan-x pinch-zoom"));
    assert_eq!(style.nav_up.as_deref(), Some("#previous"));
    assert_eq!(style.nav_right.as_deref(), Some("auto"));
    assert_eq!(style.nav_down.as_deref(), Some("#next"));
    assert_eq!(style.nav_left.as_deref(), Some("current"));
    assert_eq!(style.spatial_navigation_action.as_deref(), Some("focus"));
    assert_eq!(style.spatial_navigation_contain.as_deref(), Some("contain"));
    assert_eq!(style.spatial_navigation_function.as_deref(), Some("grid"));
    assert_eq!(style.interactivity.as_deref(), Some("inert"));
    assert!(!style.unsupported.contains_key("transition-duration"));
    assert!(!style.unsupported.contains_key("overlay"));
    assert!(!style.unsupported.contains_key("animation-composition"));
    assert!(!style.unsupported.contains_key("animation-timeline"));
    assert!(!style.unsupported.contains_key("animation-range"));
    assert!(!style.unsupported.contains_key("animation-range-start"));
    assert!(!style.unsupported.contains_key("animation-range-end"));
    assert!(!style.unsupported.contains_key("view-transition-name"));
    assert!(!style.unsupported.contains_key("view-transition-class"));
    assert!(!style.unsupported.contains_key("view-transition-group"));
    assert!(!style.unsupported.contains_key("view-transition-scope"));
    assert!(!style.unsupported.contains_key("scroll-timeline"));
    assert!(!style.unsupported.contains_key("scroll-timeline-name"));
    assert!(!style.unsupported.contains_key("scroll-timeline-axis"));
    assert!(!style.unsupported.contains_key("view-timeline"));
    assert!(!style.unsupported.contains_key("view-timeline-name"));
    assert!(!style.unsupported.contains_key("view-timeline-axis"));
    assert!(!style.unsupported.contains_key("view-timeline-inset"));
    assert!(!style.unsupported.contains_key("timeline-scope"));
    assert!(!style.unsupported.contains_key("scroll-snap-type"));
    assert!(!style.unsupported.contains_key("scroll-initial-target"));
    assert!(!style.unsupported.contains_key("scroll-start-target"));
    assert!(!style.unsupported.contains_key("scroll-target-group"));
    assert!(!style.unsupported.contains_key("scroll-marker-group"));
    assert!(!style.unsupported.contains_key("color-scheme"));
    assert!(!style.unsupported.contains_key("print-color-adjust"));
    assert!(!style.unsupported.contains_key("webkit-print-color-adjust"));
    assert!(!style.unsupported.contains_key("color-adjust"));
    assert!(!style.unsupported.contains_key("caret"));
    assert!(!style.unsupported.contains_key("caret-animation"));
    assert!(!style.unsupported.contains_key("caret-shape"));
    assert!(!style.unsupported.contains_key("scrollbar-color"));
    assert!(!style.unsupported.contains_key("overflow-block"));
    assert!(!style.unsupported.contains_key("overflow-inline"));
    assert!(!style.unsupported.contains_key("overflow-clip-margin"));
    assert!(!style.unsupported.contains_key("overflow-anchor"));
    assert!(!style.unsupported.contains_key("overscroll-behavior-block"));
    assert!(!style.unsupported.contains_key("overscroll-behavior-inline"));
    assert!(!style.unsupported.contains_key("nav-up"));
    assert!(!style.unsupported.contains_key("nav-right"));
    assert!(!style.unsupported.contains_key("nav-down"));
    assert!(!style.unsupported.contains_key("nav-left"));
    assert!(!style.unsupported.contains_key("spatial-navigation-action"));
    assert!(!style.unsupported.contains_key("spatial-navigation-contain"));
    assert!(!style
        .unsupported
        .contains_key("spatial-navigation-function"));
    assert!(!style.unsupported.contains_key("interactivity"));
}

#[test]
fn parses_tailwind_motion_scroll_and_interaction_utilities() {
    let web = WebProps::new().class_name(
        "transition-opacity duration-300 delay-75 ease-in-out transition-discrete \
             animate-spin will-change-transform appearance-none accent-[#663399]/50 \
             caret-white resize-y scroll-smooth scroll-mt-4 scroll-px-2 \
             [overlay:auto] \
             [animation-timeline:--gallery] [animation-range:entry_10%_cover_80%] \
             [animation-range-start:entry_10%] [animation-range-end:cover_80%] \
             [animation-composition:add] \
             [view-transition-name:card] [view-transition-class:shared_card] \
             [view-transition-group:contain] [view-transition-scope:root] \
             [scroll-timeline:--scroller_inline] [scroll-timeline-name:--scroller] \
             [scroll-timeline-axis:inline] [view-timeline:--card_block] \
             [view-timeline-name:--card] [view-timeline-axis:block] \
             [view-timeline-inset:10%_20%] [timeline-scope:--gallery] \
             snap-x snap-mandatory snap-center snap-always overscroll-x-contain \
             overscroll-y-none touch-pan-x scheme-only-dark forced-color-adjust-none \
             [print-color-adjust:exact] [color-adjust:exact] \
             [caret:bar_currentColor] [caret-animation:manual] [caret-shape:block] \
             field-sizing-content scrollbar-gutter-both scrollbar-thin \
             scrollbar-thumb-[#663399]/50 scrollbar-track-(--scrollbar-track) \
             [scroll-initial-target:nearest] [scroll-target-group:auto] \
             [scroll-marker-group:before] \
             md:duration-[1s] md:scheme-light-dark \
             hover:animate-[wiggle_1s_ease-in-out_infinite] hover:scrollbar-thumb-blue-500 \
             focus:will-change-[opacity] md:[animation-timeline:view()] \
             hover:[animation-range:cover_0%_contain_100%] \
             focus:[animation-composition:accumulate] hover:[overlay:none] \
             md:[view-transition-name:avatar] hover:[view-transition-class:active_card] \
             focus:[print-color-adjust:economy] active:[caret-shape:underscore] \
             focus:[scroll-timeline-axis:block] active:[view-timeline-inset:auto] \
             before:[timeline-scope:none] \
             hover:[scroll-start-target:nearest] focus:[scroll-target-group:none] \
             active:[scroll-marker-group:after] \
             [overflow-block:clip] [overflow-inline:auto] \
             [overflow-clip-margin:content-box_8px] [overflow-anchor:none] \
             [overscroll-behavior-block:contain] [overscroll-behavior-inline:none] \
             [nav-up:#previous] [nav-right:auto] [nav-down:#next] [nav-left:current] \
             [spatial-navigation-action:focus] [spatial-navigation-contain:contain] \
             [spatial-navigation-function:grid] [interactivity:inert] \
             hover:[overflow-clip-margin:border-box_2px] focus:[overflow-anchor:auto] \
             active:[overscroll-behavior-block:none] hover:[nav-right:#next] \
             focus:[spatial-navigation-action:scroll] active:[spatial-navigation-function:normal] \
             disabled:[interactivity:auto]",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.transition_property.as_deref(), Some("opacity"));
    assert_eq!(
        style.transition_duration,
        Some(StyleTime::Milliseconds(300.0))
    );
    assert_eq!(style.transition_delay, Some(StyleTime::Milliseconds(75.0)));
    assert_eq!(
        style.transition_timing_function.as_deref(),
        Some("cubic-bezier(0.4, 0, 0.2, 1)")
    );
    assert_eq!(style.transition_behavior.as_deref(), Some("allow-discrete"));
    assert_eq!(style.overlay.as_deref(), Some("auto"));
    assert_eq!(style.animation.as_deref(), Some("spin 1s linear infinite"));
    assert_eq!(style.animation_composition.as_deref(), Some("add"));
    assert_eq!(style.animation_timeline.as_deref(), Some("--gallery"));
    assert_eq!(
        style.animation_range.as_deref(),
        Some("entry 10% cover 80%")
    );
    assert_eq!(style.animation_range_start.as_deref(), Some("entry 10%"));
    assert_eq!(style.animation_range_end.as_deref(), Some("cover 80%"));
    assert_eq!(style.view_transition_name.as_deref(), Some("card"));
    assert_eq!(style.view_transition_class.as_deref(), Some("shared card"));
    assert_eq!(style.view_transition_group.as_deref(), Some("contain"));
    assert_eq!(style.view_transition_scope.as_deref(), Some("root"));
    assert_eq!(style.will_change.as_deref(), Some("transform"));
    assert_eq!(style.color_scheme.as_deref(), Some("only dark"));
    assert_eq!(style.forced_color_adjust.as_deref(), Some("none"));
    assert_eq!(style.print_color_adjust.as_deref(), Some("exact"));
    assert_eq!(style.color_adjust.as_deref(), Some("exact"));
    assert_eq!(style.field_sizing.as_deref(), Some("content"));
    assert_eq!(style.appearance.as_deref(), Some("none"));
    assert_eq!(
        style.accent_color,
        Some(StyleColor::Rgba {
            red: 0x66,
            green: 0x33,
            blue: 0x99,
            alpha: 128,
        })
    );
    assert_eq!(
        style.caret_color,
        Some(StyleColor::Rgba {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 255,
        })
    );
    assert_eq!(style.caret.as_deref(), Some("bar currentColor"));
    assert_eq!(style.caret_animation.as_deref(), Some("manual"));
    assert_eq!(style.caret_shape.as_deref(), Some("block"));
    assert_eq!(style.resize, Some(ResizeMode::Vertical));
    assert_eq!(style.scroll_behavior, Some(ScrollBehavior::Smooth));
    assert_eq!(style.scroll_timeline.as_deref(), Some("--scroller inline"));
    assert_eq!(style.scroll_timeline_name.as_deref(), Some("--scroller"));
    assert_eq!(style.scroll_timeline_axis.as_deref(), Some("inline"));
    assert_eq!(style.view_timeline.as_deref(), Some("--card block"));
    assert_eq!(style.view_timeline_name.as_deref(), Some("--card"));
    assert_eq!(style.view_timeline_axis.as_deref(), Some("block"));
    assert_eq!(style.view_timeline_inset.as_deref(), Some("10% 20%"));
    assert_eq!(style.timeline_scope.as_deref(), Some("--gallery"));
    assert_eq!(style.scroll_margin.top, Some(StyleLength::Points(16.0)));
    assert_eq!(style.scroll_padding.left, Some(StyleLength::Points(8.0)));
    assert_eq!(style.scroll_padding.right, Some(StyleLength::Points(8.0)));
    assert_eq!(
        style.scroll_snap_type.as_deref(),
        Some("x var(--tw-scroll-snap-strictness)")
    );
    assert_eq!(
        style
            .custom_properties
            .get("--tw-scroll-snap-strictness")
            .map(String::as_str),
        Some("mandatory")
    );
    assert_eq!(style.scroll_snap_align.as_deref(), Some("center"));
    assert_eq!(style.scroll_snap_stop.as_deref(), Some("always"));
    assert_eq!(style.scroll_initial_target.as_deref(), Some("nearest"));
    assert_eq!(style.scroll_target_group.as_deref(), Some("auto"));
    assert_eq!(style.scroll_marker_group.as_deref(), Some("before"));
    assert_eq!(
        style.overscroll_behavior_x,
        Some(OverscrollBehavior::Contain)
    );
    assert_eq!(style.overscroll_behavior_y, Some(OverscrollBehavior::None));
    assert_eq!(style.touch_action.as_deref(), Some("pan-x"));
    assert_eq!(style.scrollbar_gutter.as_deref(), Some("stable both-edges"));
    assert_eq!(style.scrollbar_width.as_deref(), Some("thin"));
    assert_eq!(
        style.scrollbar_thumb_color.as_deref(),
        Some("rgba(102, 51, 153, 0.5)")
    );
    assert_eq!(
        style.scrollbar_track_color.as_deref(),
        Some("var(--scrollbar-track)")
    );
    assert_eq!(
        style.scrollbar_color.as_deref(),
        Some("rgba(102, 51, 153, 0.5) var(--scrollbar-track)")
    );
    assert_eq!(style.overflow_block, Some(OverflowMode::Clip));
    assert_eq!(style.overflow_inline, Some(OverflowMode::Auto));
    assert_eq!(
        style.overflow_clip_margin.as_deref(),
        Some("content-box 8px")
    );
    assert_eq!(style.overflow_anchor.as_deref(), Some("none"));
    assert_eq!(
        style.overscroll_behavior_block,
        Some(OverscrollBehavior::Contain)
    );
    assert_eq!(
        style.overscroll_behavior_inline,
        Some(OverscrollBehavior::None)
    );
    assert_eq!(style.nav_up.as_deref(), Some("#previous"));
    assert_eq!(style.nav_right.as_deref(), Some("auto"));
    assert_eq!(style.nav_down.as_deref(), Some("#next"));
    assert_eq!(style.nav_left.as_deref(), Some("current"));
    assert_eq!(style.spatial_navigation_action.as_deref(), Some("focus"));
    assert_eq!(style.spatial_navigation_contain.as_deref(), Some("contain"));
    assert_eq!(style.spatial_navigation_function.as_deref(), Some("grid"));
    assert_eq!(style.interactivity.as_deref(), Some("inert"));
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("transition-duration"))
            .map(String::as_str),
        Some("1s")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("color-scheme"))
            .map(String::as_str),
        Some("light dark")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("animation"))
            .map(String::as_str),
        Some("wiggle 1s ease-in-out infinite")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("animation-timeline"))
            .map(String::as_str),
        Some("view()")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("animation-range"))
            .map(String::as_str),
        Some("cover 0% contain 100%")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("animation-composition"))
            .map(String::as_str),
        Some("accumulate")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("overlay"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("md")
            .and_then(|styles| styles.get("view-transition-name"))
            .map(String::as_str),
        Some("avatar")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("view-transition-class"))
            .map(String::as_str),
        Some("active card")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("print-color-adjust"))
            .map(String::as_str),
        Some("economy")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("caret-shape"))
            .map(String::as_str),
        Some("underscore")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("scroll-timeline-axis"))
            .map(String::as_str),
        Some("block")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("view-timeline-inset"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("before")
            .and_then(|styles| styles.get("timeline-scope"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("scroll-start-target"))
            .map(String::as_str),
        Some("nearest")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("scroll-target-group"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("scroll-marker-group"))
            .map(String::as_str),
        Some("after")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("overflow-clip-margin"))
            .map(String::as_str),
        Some("border-box 2px")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("overflow-anchor"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("overscroll-behavior-block"))
            .map(String::as_str),
        Some("none")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("nav-right"))
            .map(String::as_str),
        Some("#next")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("spatial-navigation-action"))
            .map(String::as_str),
        Some("scroll")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("spatial-navigation-function"))
            .map(String::as_str),
        Some("normal")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("disabled")
            .and_then(|styles| styles.get("interactivity"))
            .map(String::as_str),
        Some("auto")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("--tw-scrollbar-thumb"))
            .map(String::as_str),
        Some("blue-500")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("scrollbar-color"))
            .map(String::as_str),
        Some(tailwind_scrollbar_color_pipeline())
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("will-change"))
            .map(String::as_str),
        Some("opacity")
    );
}
