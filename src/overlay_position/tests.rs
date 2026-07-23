use super::*;
use crate::host::HostNodeId;
use crate::native::{ElementKey, NativeProps, NativeRole};
use crate::renderer::MountedNodeSnapshot;
use crate::web::WebProps;

fn options(placement: OverlayPlacement) -> OverlayPositionOptions {
    OverlayPositionOptions {
        placement,
        ..OverlayPositionOptions::default()
    }
}

#[test]
fn react_aria_placements_round_trip_as_canonical_strings() {
    for placement in OverlayPlacement::ALL {
        assert_eq!(
            placement.as_str().parse::<OverlayPlacement>().unwrap(),
            placement
        );
        assert_eq!(
            serde_json::from_str::<OverlayPlacement>(&serde_json::to_string(&placement).unwrap())
                .unwrap(),
            placement
        );
    }
    assert_eq!(
        "BOTTOM-START".parse::<OverlayPlacement>().unwrap(),
        OverlayPlacement::BottomStart
    );
}

#[test]
fn bottom_position_centers_on_the_anchor_and_applies_offsets() {
    let result = calculate_overlay_position(
        Rect::new(100.0, 100.0, 40.0, 20.0),
        Size::new(80.0, 60.0),
        Rect::new(0.0, 0.0, 300.0, 300.0),
        TextDirection::Ltr,
        OverlayPositionOptions {
            offset: 8.0,
            cross_offset: 10.0,
            ..options(OverlayPlacement::Bottom)
        },
    )
    .unwrap();

    assert_eq!(result.rect, Rect::new(90.0, 128.0, 80.0, 60.0));
    assert_eq!(result.placement.axis, OverlayPlacementAxis::Bottom);
    assert_eq!(result.arrow.x, Some(30.0));
    assert!(!result.flipped);
}

#[test]
fn logical_start_alignment_mirrors_in_rtl() {
    let anchor = Rect::new(100.0, 100.0, 40.0, 20.0);
    let overlay = Size::new(80.0, 40.0);
    let boundary = Rect::new(0.0, 0.0, 300.0, 300.0);
    let options = OverlayPositionOptions {
        cross_offset: 10.0,
        ..options(OverlayPlacement::BottomStart)
    };

    let ltr =
        calculate_overlay_position(anchor, overlay, boundary, TextDirection::Ltr, options).unwrap();
    let rtl =
        calculate_overlay_position(anchor, overlay, boundary, TextDirection::Rtl, options).unwrap();

    assert_eq!(ltr.rect.x, 110.0);
    assert_eq!(rtl.rect.x, 70.0);
    assert_eq!(
        OverlayPlacement::Start.resolve(TextDirection::Rtl).axis,
        OverlayPlacementAxis::Right
    );
}

#[test]
fn placement_flips_only_when_the_opposite_side_has_more_space() {
    let result = calculate_overlay_position(
        Rect::new(120.0, 260.0, 40.0, 20.0),
        Size::new(80.0, 100.0),
        Rect::new(0.0, 0.0, 300.0, 300.0),
        TextDirection::Ltr,
        OverlayPositionOptions {
            offset: 8.0,
            ..options(OverlayPlacement::Bottom)
        },
    )
    .unwrap();

    assert!(result.flipped);
    assert_eq!(result.placement.axis, OverlayPlacementAxis::Top);
    assert_eq!(result.rect, Rect::new(100.0, 152.0, 80.0, 100.0));
    assert_eq!(result.max_height, 240.0);
}

#[test]
fn disabling_flip_preserves_the_side_and_caps_available_height() {
    let result = calculate_overlay_position(
        Rect::new(120.0, 260.0, 40.0, 20.0),
        Size::new(80.0, 100.0),
        Rect::new(0.0, 0.0, 300.0, 300.0),
        TextDirection::Ltr,
        OverlayPositionOptions {
            offset: 8.0,
            should_flip: false,
            ..options(OverlayPlacement::Bottom)
        },
    )
    .unwrap();

    assert!(!result.flipped);
    assert_eq!(result.placement.axis, OverlayPlacementAxis::Bottom);
    assert_eq!(result.max_height, 0.0);
    assert_eq!(result.rect, Rect::new(100.0, 288.0, 80.0, 0.0));
}

#[test]
fn cross_axis_and_arrow_stay_within_the_padded_boundary() {
    let result = calculate_overlay_position(
        Rect::new(2.0, 80.0, 20.0, 20.0),
        Size::new(120.0, 50.0),
        Rect::new(0.0, 0.0, 200.0, 200.0),
        TextDirection::Ltr,
        OverlayPositionOptions {
            arrow_size: 12.0,
            arrow_boundary_offset: 8.0,
            ..options(OverlayPlacement::Bottom)
        },
    )
    .unwrap();

    assert_eq!(result.rect.x, 12.0);
    assert_eq!(result.arrow.x, Some(14.0));
}

#[test]
fn invalid_geometry_and_options_are_rejected() {
    let result = calculate_overlay_position(
        Rect::new(0.0, 0.0, 10.0, 10.0),
        Size::new(10.0, 10.0),
        Rect::new(0.0, 0.0, 100.0, 100.0),
        TextDirection::Ltr,
        OverlayPositionOptions {
            container_padding: -1.0,
            ..OverlayPositionOptions::default()
        },
    );

    assert!(result
        .unwrap_err()
        .to_string()
        .contains("containerPadding must be non-negative"));
    assert!("diagonal".parse::<OverlayPlacement>().is_err());
}

fn mounted(
    id: u64,
    parent: Option<u64>,
    key: &str,
    role: NativeRole,
    props: NativeProps,
) -> MountedNodeSnapshot {
    MountedNodeSnapshot {
        node: HostNodeId::new(id),
        parent: parent.map(HostNodeId::new),
        key: ElementKey::new(key),
        role,
        props,
    }
}

fn positioned_props(anchor: Option<&str>, open: bool) -> NativeProps {
    let mut props = NativeProps::new()
        .metadata(OVERLAY_POSITION_MARKER, "true")
        .metadata(OVERLAY_PLACEMENT_ATTRIBUTE, "bottom start")
        .metadata(OVERLAY_OFFSET_ATTRIBUTE, "8")
        .metadata("data-open", open.to_string());
    if let Some(anchor) = anchor {
        props.anchor = Some(anchor.to_string());
    }
    props
}

#[test]
fn mounted_positioning_resolves_a_contextual_trigger_sibling() {
    let snapshot = vec![
        mounted(1, None, "window", NativeRole::Window, NativeProps::new()),
        mounted(2, Some(1), "group", NativeRole::View, NativeProps::new()),
        mounted(
            3,
            Some(2),
            "trigger",
            NativeRole::Button,
            NativeProps::new().metadata("data-overlay-trigger", "true"),
        ),
        mounted(
            4,
            Some(2),
            "popover",
            NativeRole::Popover,
            positioned_props(None, true).dir("rtl"),
        ),
    ];

    let positions = mounted_overlay_positions(&snapshot).unwrap();

    assert_eq!(positions.len(), 1);
    assert_eq!(positions[0].overlay, HostNodeId::new(4));
    assert_eq!(positions[0].anchor, HostNodeId::new(3));
    assert_eq!(positions[0].request.direction, TextDirection::Rtl);
    assert_eq!(positions[0].request.options.offset, 8.0);
}

#[test]
fn mounted_positioning_resolves_a_portaled_explicit_anchor_id() {
    let snapshot = vec![
        mounted(1, None, "window", NativeRole::Window, NativeProps::new()),
        mounted(
            2,
            Some(1),
            "trigger-key",
            NativeRole::Button,
            NativeProps::new().web(WebProps::new().attribute("id", "help-trigger")),
        ),
        mounted(3, Some(1), "portal", NativeRole::View, NativeProps::new()),
        mounted(
            4,
            Some(3),
            "popover",
            NativeRole::Popover,
            positioned_props(Some("#help-trigger"), true),
        ),
    ];

    let positions = mounted_overlay_positions(&snapshot).unwrap();

    assert_eq!(positions[0].anchor, HostNodeId::new(2));
}

#[test]
fn closed_positioned_overlays_do_not_require_an_anchor() {
    let snapshot = vec![
        mounted(1, None, "window", NativeRole::Window, NativeProps::new()),
        mounted(
            2,
            Some(1),
            "popover",
            NativeRole::Popover,
            positioned_props(None, false),
        ),
    ];

    assert!(mounted_overlay_positions(&snapshot).unwrap().is_empty());
}

#[test]
fn unresolved_explicit_anchor_is_an_error() {
    let snapshot = vec![
        mounted(1, None, "window", NativeRole::Window, NativeProps::new()),
        mounted(
            2,
            Some(1),
            "popover",
            NativeRole::Popover,
            positioned_props(Some("missing"), true),
        ),
    ];

    assert!(mounted_overlay_positions(&snapshot)
        .unwrap_err()
        .to_string()
        .contains("does not match a mounted id or unique key"));
}
