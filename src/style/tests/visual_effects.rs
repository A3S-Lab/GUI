use super::support::*;

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
