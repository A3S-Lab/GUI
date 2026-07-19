use super::support::*;

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
fn parses_design_md_semantic_tailwind_colors() {
    let web = WebProps::new().class_name(
        "bg-canvas text-ink border-hairline caret-ink \
         ring-ink hover:bg-surface-card focus:text-body active:border-hairline-strong",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(
        style.background_color,
        Some(StyleColor::Rgba {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
            alpha: 255,
        })
    );
    assert_eq!(
        style.color,
        Some(StyleColor::Rgba {
            red: 0x17,
            green: 0x17,
            blue: 0x17,
            alpha: 255,
        })
    );
    assert_eq!(
        style.border_color,
        Some(StyleColor::Rgba {
            red: 0xf0,
            green: 0xf0,
            blue: 0xf3,
            alpha: 255,
        })
    );
    assert_eq!(
        style.caret_color,
        Some(StyleColor::Rgba {
            red: 0x17,
            green: 0x17,
            blue: 0x17,
            alpha: 255,
        })
    );
    assert_eq!(
        style
            .variant_declarations
            .get("hover")
            .and_then(|styles| styles.get("background-color"))
            .map(String::as_str),
        Some("rgb(255, 255, 255)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("focus")
            .and_then(|styles| styles.get("color"))
            .map(String::as_str),
        Some("rgb(96, 100, 108)")
    );
    assert_eq!(
        style
            .custom_properties
            .get("--tw-ring-color")
            .map(String::as_str),
        Some("rgb(23, 23, 23)")
    );
    assert_eq!(
        style
            .variant_declarations
            .get("active")
            .and_then(|styles| styles.get("border-color"))
            .map(String::as_str),
        Some("rgb(220, 222, 224)")
    );
}

#[test]
fn rejects_non_design_md_semantic_color_aliases() {
    let web = WebProps::new().class_name(
        "bg-background text-foreground border-border caret-ring ring-ring \
         hover:bg-card focus:text-muted-foreground active:border-sidebar-border \
         bg-destructive text-destructive text-primary-foreground border-input text-link-blue",
    );

    let style = PortableStyle::from_web(&web);

    assert_eq!(style.background_color, None);
    assert_eq!(style.color, None);
    assert_eq!(style.border_color, None);
    assert_eq!(style.caret_color, None);
    assert!(style.custom_properties.get("--tw-ring-color").is_none());
    assert!(style
        .variant_declarations
        .get("hover")
        .and_then(|styles| styles.get("background-color"))
        .is_none());
    assert!(style
        .variant_declarations
        .get("focus")
        .and_then(|styles| styles.get("color"))
        .is_none());
    assert!(style
        .variant_declarations
        .get("active")
        .and_then(|styles| styles.get("border-color"))
        .is_none());
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
