use super::support::*;

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
