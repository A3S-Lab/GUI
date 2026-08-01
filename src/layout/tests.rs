use crate::geometry::{Orientation, Rect, Size};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::web::WebProps;

use super::*;

fn element(key: &str, role: NativeRole, class_name: &str) -> NativeElement {
    NativeElement::new(key, role)
        .with_props(NativeProps::new().web(WebProps::new().class_name(class_name)))
}

fn oriented(
    key: &str,
    role: NativeRole,
    orientation: Orientation,
    class_name: &str,
) -> NativeElement {
    NativeElement::new(key, role).with_props(
        NativeProps::new()
            .orientation(orientation)
            .web(WebProps::new().class_name(class_name)),
    )
}

#[test]
fn public_layout_records_are_send_and_sync() {
    fn assert_send_sync<T: Send + Sync>() {}

    assert_send_sync::<LayoutSnapshot>();
    assert_send_sync::<LayoutNodeRecord>();
    assert_send_sync::<LayoutHitRegion>();
    assert_send_sync::<LayoutDiff>();
    assert_send_sync::<LayoutElementId>();
}

#[test]
fn logical_surface_must_remain_positive_after_quantization() {
    let root = element("root", NativeRole::View, "h-px w-px");

    let error = layout_native_tree(&root, Size::new(0.001, 1.0)).unwrap_err();

    assert!(error
        .to_string()
        .contains("layout logical size must be finite and greater than zero"));
}

#[test]
fn row_layout_quantizes_box_model_alignment_and_gap() {
    let root = oriented(
        "root",
        NativeRole::View,
        Orientation::Horizontal,
        "h-[120px] w-[200px] items-center justify-center gap-[5px] bg-white p-[10px]",
    )
    .child(element("a", NativeRole::View, "h-5 w-[50px] bg-black"))
    .child(element("b", NativeRole::View, "h-5 w-10 bg-black"));

    let snapshot = layout_native_tree(&root, Size::new(200.0, 120.0)).unwrap();

    assert!(!snapshot.has_errors(), "{:?}", snapshot.diagnostics);
    assert_eq!(snapshot.schema_version, LAYOUT_SCHEMA_VERSION);
    assert_eq!(snapshot.nodes.len(), 3);
    assert_eq!(snapshot.hit_regions.len(), 3);
    assert_eq!(
        snapshot.nodes[0].content_box,
        Rect::new(10.0, 10.0, 180.0, 100.0)
    );
    assert_eq!(
        snapshot.nodes[1].border_box,
        Rect::new(52.5, 50.0, 50.0, 20.0)
    );
    assert_eq!(
        snapshot.nodes[2].border_box,
        Rect::new(107.5, 50.0, 40.0, 20.0)
    );
    assert_eq!(snapshot.nodes[1].id.as_str(), "4:root/1:a");
}

#[test]
fn flex_defaults_to_horizontal_flow() {
    let root = element("root", NativeRole::View, "flex h-5 w-10")
        .child(element("a", NativeRole::View, "h-5 w-5"))
        .child(element("b", NativeRole::View, "h-5 w-5"));

    let snapshot = layout_native_tree(&root, Size::new(40.0, 20.0)).unwrap();

    assert_eq!(
        snapshot
            .node(&LayoutElementId::root("root").child("a"))
            .unwrap()
            .border_box,
        Rect::new(0.0, 0.0, 20.0, 20.0)
    );
    assert_eq!(
        snapshot
            .node(&LayoutElementId::root("root").child("b"))
            .unwrap()
            .border_box,
        Rect::new(20.0, 0.0, 20.0, 20.0)
    );
}

#[test]
fn absolute_positioning_and_overflow_produce_a_child_clip() {
    let root = element(
        "root",
        NativeRole::View,
        "relative h-20 w-[100px] overflow-hidden bg-white p-[10px]",
    )
    .child(element(
        "overlay",
        NativeRole::View,
        "absolute bottom-[6px] right-[5px] h-[10px] w-5 bg-black",
    ));

    let snapshot = layout_native_tree(&root, Size::new(100.0, 80.0)).unwrap();

    assert!(!snapshot.has_errors(), "{:?}", snapshot.diagnostics);
    assert_eq!(
        snapshot.nodes[1].border_box,
        Rect::new(65.0, 54.0, 20.0, 10.0)
    );
    assert_eq!(
        snapshot.nodes[1].clip,
        Some(Rect::new(0.0, 0.0, 100.0, 80.0))
    );
}

#[test]
fn relative_positioning_accepts_negative_insets() {
    let child = NativeElement::new("child", NativeRole::View).with_props(
        NativeProps::new().web(
            WebProps::new()
                .style("position", "relative")
                .style("left", "-5px")
                .style("width", "10px")
                .style("height", "10px"),
        ),
    );
    let root = element("root", NativeRole::View, "h-5 w-5").child(child);

    let snapshot = layout_native_tree(&root, Size::new(20.0, 20.0)).unwrap();
    let child = snapshot
        .node(&LayoutElementId::root("root").child("child"))
        .unwrap();

    assert_eq!(child.border_box, Rect::new(-5.0, 0.0, 10.0, 10.0));
    assert!(!snapshot.has_errors(), "{:?}", snapshot.diagnostics);
}

#[test]
fn stable_key_paths_survive_reordering_and_drive_layout_diff() {
    let root = |children: Vec<NativeElement>| {
        oriented(
            "root",
            NativeRole::View,
            Orientation::Horizontal,
            "h-10 w-[100px] gap-1",
        )
        .children(children)
    };
    let first = layout_native_tree(
        &root(vec![
            element("a", NativeRole::View, "h-10 w-5"),
            element("b", NativeRole::View, "h-10 w-5"),
        ]),
        Size::new(100.0, 40.0),
    )
    .unwrap();
    let reordered = layout_native_tree(
        &root(vec![
            element("b", NativeRole::View, "h-10 w-5"),
            element("a", NativeRole::View, "h-10 w-5"),
        ]),
        Size::new(100.0, 40.0),
    )
    .unwrap();

    let a = LayoutElementId::root("root").child("a");
    let b = LayoutElementId::root("root").child("b");
    assert!(first.node(&a).is_some());
    assert!(reordered.node(&a).is_some());
    assert!(first.node(&b).is_some());
    assert!(reordered.node(&b).is_some());
    let diff = first.diff(&reordered);
    assert!(!diff.full_rebuild);
    assert_eq!(diff.changes.len(), 2);
    assert!(diff
        .changes
        .iter()
        .all(|change| change.kind == LayoutChangeKind::Changed));
}

#[test]
fn z_index_changes_paint_order_without_moving_flow_items() {
    let root = oriented(
        "root",
        NativeRole::View,
        Orientation::Horizontal,
        "h-5 w-10",
    )
    .child(element("front", NativeRole::View, "z-10 h-5 w-5"))
    .child(element("back", NativeRole::View, "z-0 h-5 w-5"));

    let snapshot = layout_native_tree(&root, Size::new(40.0, 20.0)).unwrap();
    let front = snapshot
        .node(&LayoutElementId::root("root").child("front"))
        .unwrap();
    let back = snapshot
        .node(&LayoutElementId::root("root").child("back"))
        .unwrap();

    assert_eq!(front.border_box, Rect::new(0.0, 0.0, 20.0, 20.0));
    assert_eq!(back.border_box, Rect::new(20.0, 0.0, 20.0, 20.0));
    assert!(front.paint_order > back.paint_order);
}

#[test]
fn hit_region_state_changes_are_visible_to_layout_diff() {
    let button = |disabled| {
        NativeElement::new("button", NativeRole::Button).with_props(
            NativeProps::new()
                .disabled(disabled)
                .web(WebProps::new().class_name("h-5 w-5")),
        )
    };
    let enabled = layout_native_tree(&button(false), Size::new(20.0, 20.0)).unwrap();
    let disabled = layout_native_tree(&button(true), Size::new(20.0, 20.0)).unwrap();

    let diff = enabled.diff(&disabled);

    assert_eq!(diff.changes.len(), 1);
    assert_eq!(diff.changes[0].kind, LayoutChangeKind::Changed);
}

#[test]
fn snapshots_are_serializable_and_repeatably_fingerprinted() {
    let root = element("root", NativeRole::View, "h-8 w-8 bg-[#123456]");
    let first = layout_native_tree(&root, Size::new(32.0, 32.0)).unwrap();
    let second = layout_native_tree(&root, Size::new(32.0, 32.0)).unwrap();
    let encoded = serde_json::to_string(&first).unwrap();
    let decoded: LayoutSnapshot = serde_json::from_str(&encoded).unwrap();

    assert_eq!(decoded, first);
    assert_eq!(first.fingerprint().unwrap(), second.fingerprint().unwrap());
    assert!(first.diff(&second).is_empty());
}

#[test]
fn unresolved_m3_lengths_are_errors_instead_of_silent_omissions() {
    let root = NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(
            WebProps::new()
                .style("width", "calc(100% - 2rem)")
                .style("height", "20px"),
        ),
    );

    let snapshot = layout_native_tree(&root, Size::new(100.0, 20.0)).unwrap();

    assert!(snapshot.has_errors());
    assert!(snapshot.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == LayoutDiagnosticCode::UnresolvedLength
            && diagnostic.field.as_deref() == Some("width")
    }));
    assert!(snapshot.require_supported().is_err());
}

#[test]
fn elliptical_percentage_radii_are_rejected() {
    let root = NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(
            WebProps::new()
                .style("width", "40px")
                .style("height", "20px")
                .style("border-radius", "50%"),
        ),
    );

    let snapshot = layout_native_tree(&root, Size::new(40.0, 20.0)).unwrap();

    assert!(snapshot.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == LayoutDiagnosticCode::UnsupportedM3StyleField
            && diagnostic.message.contains("elliptical corner radii")
    }));
}

#[test]
fn circular_corner_radii_use_css_edge_sum_normalization() {
    let root = NativeElement::new("root", NativeRole::View).with_props(
        NativeProps::new().web(
            WebProps::new()
                .style("width", "100px")
                .style("height", "50px")
                .style("border-top-left-radius", "80px")
                .style("border-top-right-radius", "40px"),
        ),
    );

    let snapshot = layout_native_tree(&root, Size::new(100.0, 50.0)).unwrap();

    assert_eq!(snapshot.nodes[0].paint.corner_radii.top_left, 50.0);
    assert_eq!(snapshot.nodes[0].paint.corner_radii.top_right, 25.0);
}

#[test]
fn minimum_size_wins_when_maximum_is_smaller() {
    let root = element(
        "root",
        NativeRole::View,
        "h-5 min-w-[60px] max-w-10 w-[50px]",
    );

    let snapshot = layout_native_tree(&root, Size::new(100.0, 20.0)).unwrap();

    assert_eq!(snapshot.nodes[0].border_box.width, 60.0);
}

#[test]
fn constrained_parent_size_drives_child_percentage_resolution() {
    let parent = element("parent", NativeRole::View, "h-5 w-[200px] max-w-[100px]").child(element(
        "child",
        NativeRole::View,
        "h-5 w-[50%]",
    ));
    let root = element("root", NativeRole::View, "h-5 w-[200px]").child(parent);

    let snapshot = layout_native_tree(&root, Size::new(200.0, 20.0)).unwrap();

    assert_eq!(
        snapshot
            .node(&LayoutElementId::root("root").child("parent"))
            .unwrap()
            .border_box
            .width,
        100.0
    );
    assert_eq!(
        snapshot
            .node(&LayoutElementId::root("root").child("parent").child("child"))
            .unwrap()
            .border_box
            .width,
        50.0
    );
}

#[test]
fn duplicate_sibling_keys_are_rejected_before_layout() {
    let root = element("root", NativeRole::View, "h-10 w-10")
        .child(element("same", NativeRole::View, "h-2 w-2"))
        .child(element("same", NativeRole::View, "h-2 w-2"));

    let error = layout_native_tree(&root, Size::new(40.0, 40.0)).unwrap_err();

    assert!(error.to_string().contains("duplicate key \"same\""));
}

#[test]
fn calculator_rectangle_subset_uses_one_generic_row_column_tree() {
    let button = |key: &str, color: &str| {
        element(
            key,
            NativeRole::Button,
            &format!(
                "h-14 min-h-14 w-[94px] min-w-[94px] rounded-[5px] border border-[#e5e5e5] {color}"
            ),
        )
    };
    let row = oriented(
        "row",
        NativeRole::Toolbar,
        Orientation::Horizontal,
        "h-14 w-96 gap-[3px] bg-[#f3f3f3]",
    )
    .child(button("seven", "bg-white"))
    .child(button("eight", "bg-white"))
    .child(button("nine", "bg-white"))
    .child(button("multiply", "bg-[#f9f9f9]"));
    let root = oriented(
        "calculator",
        NativeRole::Toolbar,
        Orientation::Vertical,
        "h-[620px] w-[396px] bg-[#f3f3f3]",
    )
    .child(row);

    let snapshot = layout_native_tree(&root, Size::new(410.0, 620.0)).unwrap();

    assert!(!snapshot.has_errors(), "{:?}", snapshot.diagnostics);
    assert_eq!(snapshot.nodes.len(), 6);
    assert_eq!(
        snapshot.nodes[0].border_box,
        Rect::new(0.0, 0.0, 396.0, 620.0)
    );
    assert_eq!(
        snapshot.nodes[1].border_box,
        Rect::new(0.0, 0.0, 384.0, 56.0)
    );
    assert_eq!(
        snapshot.nodes[2].border_box,
        Rect::new(0.0, 0.0, 94.0, 56.0)
    );
    assert_eq!(
        snapshot.nodes[5].border_box,
        Rect::new(291.0, 0.0, 94.0, 56.0)
    );
    assert_eq!(
        snapshot.nodes[2].paint.corner_radii,
        LayoutCornerRadii::all(5.0)
    );
    assert_eq!(
        snapshot.nodes[2].paint.border_widths,
        LayoutEdgeWidths::all(1.0)
    );
}
