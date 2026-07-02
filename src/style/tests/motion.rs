use super::support::*;

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
