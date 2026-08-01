use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{PlatformElementId, PlatformKeyState, PlatformPointerPhase};
use crate::web::WebProps;
use crate::NativeEventKind;

use super::drag_drop_tests::{action_events, input};
use super::tests::{key_event, pointer_event, runtime};
use super::{SelfDrawnCollectionDropTarget, SelfDrawnDropPosition};

const SOURCE_ITEMS: [(&str, &str, bool, u32); 3] = [
    ("source-a", "alpha", true, 5),
    ("source-b", "beta", true, 45),
    ("source-c", "gamma", false, 85),
];

pub(super) fn collection_tree(low_level_drop: bool) -> NativeElement {
    let source = NativeElement::new("source-list", NativeRole::ListBox)
        .with_props(
            NativeProps::new().tab_index(Some(-1)).web(
                WebProps::new()
                    .class_name("absolute left-[5px] top-[5px] h-[125px] w-[80px] bg-white")
                    .attribute("data-draggable-collection", "true")
                    .attribute("data-allowed-drop-operations", "move")
                    .event("onDragStart", "collectionDragStart")
                    .event("onDragMove", "collectionDragMove")
                    .event("onDragEnd", "collectionDragEnd"),
            ),
        )
        .children(SOURCE_ITEMS.into_iter().map(source_item));

    let mut target_web = WebProps::new()
        .class_name("absolute left-[110px] top-[5px] h-[125px] w-[100px] bg-white")
        .attribute("data-droppable-collection", "true")
        .attribute("data-drop-orientation", "vertical")
        .attribute("data-accepted-drag-types", "text/plain")
        .attribute("data-drop-operation", "move")
        .event("onDropEnter", "collectionEnter")
        .event("onDropMove", "collectionMove")
        .event("onDropExit", "collectionExit")
        .event("onRootDrop", "rootDrop")
        .event("onItemDrop", "itemDrop")
        .event("onInsert", "insert");
    if low_level_drop {
        target_web = target_web.event("onDrop", "rawDrop");
    }
    let target = NativeElement::new("target-list", NativeRole::ListBox)
        .with_props(NativeProps::new().tab_index(Some(-1)).web(target_web))
        .child(target_item("target-a", 10))
        .child(target_item("target-b", 65));

    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[140px] w-[220px] bg-white")),
        )
        .child(source)
        .child(target)
}

fn source_item((key, value, selected, top): (&str, &str, bool, u32)) -> NativeElement {
    let items = format!(r#"[{{"text/plain":"{value}"}}]"#);
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .tab_index(Some(if key == "source-a" { 0 } else { -1 }))
            .selected(selected)
            .web(
                WebProps::new()
                    .class_name(format!(
                        "absolute left-[5px] top-[{top}px] h-[30px] w-[70px] bg-black"
                    ))
                    .attribute("data-collection-drop-item", "true")
                    .attribute("data-collection-key", key)
                    .attribute("data-drag-items", items),
            ),
    )
}

fn target_item(key: &str, top: u32) -> NativeElement {
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(
        NativeProps::new().tab_index(Some(-1)).web(
            WebProps::new()
                .class_name(format!(
                    "absolute left-[5px] top-[{top}px] h-[40px] w-[90px] bg-white"
                ))
                .attribute("data-collection-drop-item", "true")
                .attribute("data-collection-key", key),
        ),
    )
}

pub(super) fn start_selected_pointer_drag(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
) {
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 15.0, 15.0, 10))
        .unwrap();
    let started = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 20.0, 15.0, 20))
            .unwrap(),
    );
    assert_eq!(
        action_events(&started),
        vec![
            ("collectionDragStart", NativeEventKind::DragStart),
            ("collectionDragMove", NativeEventKind::DragMove),
        ]
    );
    let drag = started.invocations[0].context.drag.as_ref().unwrap();
    assert_eq!(drag.dragging_keys, ["source-a", "source-b"]);
    assert_eq!(drag.items.len(), 2);
    assert_eq!(drag.items[0].get_text("text/plain"), Some("alpha"));
    assert_eq!(drag.items[1].get_text("text/plain"), Some("beta"));
    for id in [
        "4:root/11:source-list/8:source-a",
        "4:root/11:source-list/8:source-b",
    ] {
        assert!(
            runtime
                .element_interaction(&PlatformElementId::new(id).unwrap())
                .unwrap()
                .dragging
        );
    }
}

#[test]
fn pointer_delegate_distinguishes_before_on_and_root_targets() {
    let mut runtime = runtime();
    runtime.render(collection_tree(false)).unwrap();
    start_selected_pointer_drag(&mut runtime);

    let before = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 18.0, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&before),
        vec![
            ("collectionDragMove", NativeEventKind::DragMove),
            ("collectionEnter", NativeEventKind::DropEnter),
            ("collectionMove", NativeEventKind::DropMove),
        ]
    );
    assert_eq!(
        before
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "target-a".to_string(),
            drop_position: SelfDrawnDropPosition::Before,
        })
    );

    let on = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 35.0, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&on),
        vec![
            ("collectionDragMove", NativeEventKind::DragMove),
            ("collectionExit", NativeEventKind::DropExit),
            ("collectionEnter", NativeEventKind::DropEnter),
            ("collectionMove", NativeEventKind::DropMove),
        ]
    );
    assert_eq!(
        on.invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "target-a".to_string(),
            drop_position: SelfDrawnDropPosition::On,
        })
    );

    let root_target = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 205.0, 125.0, 50))
            .unwrap(),
    );
    assert_eq!(
        root_target
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Root)
    );
}

#[test]
fn collection_drop_routes_high_level_handler_and_low_level_override() {
    for (low_level, expected) in [(false, "insert"), (true, "rawDrop")] {
        let mut runtime = runtime();
        runtime.render(collection_tree(low_level)).unwrap();
        start_selected_pointer_drag(&mut runtime);
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 18.0, 30))
            .unwrap();
        let dropped = input(
            runtime
                .handle_event(pointer_event(
                    PlatformPointerPhase::Released,
                    125.0,
                    18.0,
                    40,
                ))
                .unwrap(),
        );
        assert_eq!(dropped.invocations[0].action, expected);
        let drag = dropped.invocations[0].context.drag.as_ref().unwrap();
        assert!(drag.is_internal == false);
        assert_eq!(
            drag.target,
            Some(SelfDrawnCollectionDropTarget::Item {
                key: "target-a".to_string(),
                drop_position: SelfDrawnDropPosition::Before,
            })
        );
    }
}

#[test]
fn pointer_root_drop_routes_root_handler() {
    let mut runtime = runtime();
    runtime.render(collection_tree(false)).unwrap();
    start_selected_pointer_drag(&mut runtime);
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 205.0, 125.0, 30))
        .unwrap();

    let dropped = input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                205.0,
                125.0,
                40,
            ))
            .unwrap(),
    );
    assert_eq!(dropped.invocations[0].action, "rootDrop");
    assert_eq!(
        dropped.invocations[0].context.drag.as_ref().unwrap().target,
        Some(SelfDrawnCollectionDropTarget::Root)
    );
}

#[test]
fn keyboard_uses_one_collection_tab_stop_then_arrows_within_it() {
    let mut runtime = runtime();
    runtime.render(collection_tree(false)).unwrap();
    runtime
        .handle_event(key_event("Tab", PlatformKeyState::Pressed, 10))
        .unwrap();
    let started = input(
        runtime
            .handle_event(key_event("Enter", PlatformKeyState::Pressed, 20))
            .unwrap(),
    );
    assert_eq!(started.invocations[0].action, "collectionDragStart");

    let entered = input(
        runtime
            .handle_event(key_event("Tab", PlatformKeyState::Pressed, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&entered),
        vec![("collectionEnter", NativeEventKind::DropEnter)]
    );
    assert_eq!(
        entered.invocations[0].context.drag.as_ref().unwrap().target,
        Some(SelfDrawnCollectionDropTarget::Root)
    );

    let before = input(
        runtime
            .handle_event(key_event("ArrowDown", PlatformKeyState::Pressed, 40))
            .unwrap(),
    );
    assert_eq!(
        before
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "target-a".to_string(),
            drop_position: SelfDrawnDropPosition::Before,
        })
    );
    let on = input(
        runtime
            .handle_event(key_event("ArrowDown", PlatformKeyState::Pressed, 50))
            .unwrap(),
    );
    assert_eq!(
        on.invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "target-a".to_string(),
            drop_position: SelfDrawnDropPosition::On,
        })
    );

    let dropped = input(
        runtime
            .handle_event(key_event("Enter", PlatformKeyState::Pressed, 60))
            .unwrap(),
    );
    assert_eq!(dropped.invocations[0].action, "itemDrop");
}

#[test]
fn adjacent_insertion_boundaries_are_one_logical_target() {
    let mut runtime = runtime();
    runtime.render(collection_tree(false)).unwrap();
    start_selected_pointer_drag(&mut runtime);

    let after_first = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 47.0, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&after_first),
        vec![
            ("collectionDragMove", NativeEventKind::DragMove),
            ("collectionEnter", NativeEventKind::DropEnter),
            ("collectionMove", NativeEventKind::DropMove),
        ]
    );

    let before_second = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 67.0, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&before_second),
        vec![
            ("collectionDragMove", NativeEventKind::DragMove),
            ("collectionMove", NativeEventKind::DropMove),
        ]
    );
    assert_eq!(
        before_second
            .invocations
            .last()
            .unwrap()
            .context
            .drag
            .as_ref()
            .unwrap()
            .target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "target-a".to_string(),
            drop_position: SelfDrawnDropPosition::After,
        })
    );
}
