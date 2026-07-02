use super::support::*;

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
