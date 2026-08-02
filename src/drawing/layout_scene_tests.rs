use crate::geometry::Size as GuiSize;
use crate::layout::{layout_native_tree as build_layout, LayoutOptions, LayoutSnapshot};
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::web::WebProps;

use super::*;

fn layout_native_tree(root: &NativeElement, size: GuiSize) -> crate::GuiResult<LayoutSnapshot> {
    build_layout(root, LayoutOptions::boxes_only(size))
}

fn styled(key: &str, class_name: &str) -> NativeElement {
    NativeElement::new(key, NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().class_name(class_name)))
}

#[test]
fn scene_rejects_error_level_layout_diagnostics() {
    let root = NativeElement::new("root", NativeRole::View)
        .with_props(NativeProps::new().web(WebProps::new().style("width", "calc(100% - 2rem)")));
    let layout = layout_native_tree(&root, GuiSize::new(10.0, 10.0)).unwrap();

    let error = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("layout field support is incomplete"));
}

#[test]
fn stable_element_paths_produce_stable_draw_ids() {
    let tree = |reversed: bool| {
        let mut children = vec![
            styled("a", "absolute left-[0px] top-[0px] h-2 w-2 bg-black"),
            styled("b", "absolute left-[2px] top-[0px] h-2 w-2 bg-white"),
        ];
        if reversed {
            children.reverse();
        }
        styled("root", "relative h-2 w-4").children(children)
    };
    let first = layout_native_tree(&tree(false), GuiSize::new(4.0, 2.0)).unwrap();
    let second = layout_native_tree(&tree(true), GuiSize::new(4.0, 2.0)).unwrap();
    let first = scene_from_layout(&first, LayoutSceneOptions::default()).unwrap();
    let second = scene_from_layout(&second, LayoutSceneOptions::default()).unwrap();
    let mut first_ids = first
        .commands
        .iter()
        .map(|command| command.id)
        .collect::<Vec<_>>();
    let mut second_ids = second
        .commands
        .iter()
        .map(|command| command.id)
        .collect::<Vec<_>>();
    first_ids.sort();
    second_ids.sort();

    assert_eq!(first_ids, second_ids);
}

#[cfg(feature = "software-reference")]
#[test]
fn sibling_z_index_controls_scene_paint_order() {
    let root = styled("root", "relative h-2 w-2")
        .child(styled(
            "front",
            "absolute left-[0px] top-[0px] z-10 h-2 w-2 bg-black",
        ))
        .child(styled(
            "back",
            "absolute left-[0px] top-[0px] z-0 h-2 w-2 bg-white",
        ));
    let layout = layout_native_tree(&root, GuiSize::new(2.0, 2.0)).unwrap();
    let scene = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap();
    let mut renderer = crate::drawing::ReferenceRenderer::new();

    let frame = renderer.render(scene).unwrap();

    assert_eq!(&frame.rgba8()[0..4], &[0, 0, 0, 255]);
}

#[cfg(feature = "software-reference")]
#[test]
fn rectangle_layout_lowers_through_the_reference_renderer() {
    let root = styled("root", "relative h-3 w-4 bg-white").child(styled(
        "pixel",
        "absolute left-[1px] top-[1px] h-1 w-2 bg-black",
    ));
    let layout = layout_native_tree(&root, GuiSize::new(4.0, 3.0)).unwrap();
    let scene = scene_from_layout(&layout, LayoutSceneOptions::default()).unwrap();
    let mut renderer = crate::drawing::ReferenceRenderer::new();

    let frame = renderer.render(scene).unwrap();

    assert_eq!((frame.width(), frame.height()), (4, 3));
    assert_eq!(&frame.rgba8()[0..4], &[255, 255, 255, 255]);
    let black = ((4 + 1) * 4) as usize;
    assert_eq!(&frame.rgba8()[black..black + 4], &[0, 0, 0, 255]);
}
