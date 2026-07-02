use super::support::*;

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
