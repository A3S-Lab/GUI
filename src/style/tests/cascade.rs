use super::support::*;

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
    assert_eq!(style.border_radius, Some(StyleLength::Points(12.0)));
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
