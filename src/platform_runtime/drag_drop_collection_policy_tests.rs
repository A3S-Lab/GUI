use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::PlatformPointerPhase;
use crate::web::WebProps;
use crate::NativeEventKind;

use super::drag_drop_tests::{action_events, input};
use super::tests::{pointer_event, runtime};
use super::{SelfDrawnCollectionDropTarget, SelfDrawnDropPosition, SelfDrawnInputDispatch};

fn collection(events: &[(&str, &str)], items: Vec<NativeElement>) -> NativeElement {
    let mut web = WebProps::new()
        .class_name("relative h-[150px] w-[150px] bg-white")
        .attribute("data-draggable-collection", "true")
        .attribute("data-droppable-collection", "true")
        .attribute("data-drop-orientation", "vertical")
        .attribute("data-accepted-drag-types", "text/plain")
        .attribute("data-allowed-drop-operations", "move")
        .attribute("data-drop-operation", "move")
        .event("onDragStart", "dragStart")
        .event("onDragMove", "dragMove")
        .event("onDragEnd", "dragEnd")
        .event("onDropEnter", "dropEnter")
        .event("onDropMove", "dropMove")
        .event("onDropExit", "dropExit");
    for (name, action) in events {
        web = web.event(*name, *action);
    }
    NativeElement::new("collection", NativeRole::ListBox)
        .with_props(NativeProps::new().tab_index(Some(-1)).web(web))
        .children(items)
}

fn item(key: &str, top: u32, selected: bool, parent: Option<&str>) -> NativeElement {
    let mut web = WebProps::new()
        .class_name(format!(
            "absolute left-[5px] top-[{top}px] h-[30px] w-[120px] bg-black"
        ))
        .attribute("data-collection-drop-item", "true")
        .attribute("data-collection-key", key)
        .attribute("data-drag-items", format!(r#"[{{"text/plain":"{key}"}}]"#));
    if let Some(parent) = parent {
        web = web.attribute("data-tree-parent-key", parent);
    }
    NativeElement::new(key, NativeRole::ListBoxItem).with_props(
        NativeProps::new()
            .tab_index(Some(if top == 5 { 0 } else { -1 }))
            .selected(selected)
            .web(web),
    )
}

fn flat_items() -> Vec<NativeElement> {
    vec![
        item("a", 5, true, None),
        item("b", 50, false, None),
        item("c", 95, false, None),
    ]
}

fn start_drag(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
) -> SelfDrawnInputDispatch {
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 15.0, 15.0, 10))
        .unwrap();
    input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 20.0, 15.0, 20))
            .unwrap(),
    )
}

fn move_to(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
    y: f64,
    timestamp: u64,
) -> SelfDrawnInputDispatch {
    input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Moved,
                15.0,
                y,
                timestamp,
            ))
            .unwrap(),
    )
}

fn drop_at(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
    y: f64,
    timestamp: u64,
) -> SelfDrawnInputDispatch {
    input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                15.0,
                y,
                timestamp,
            ))
            .unwrap(),
    )
}

fn drop_actions(dispatch: &SelfDrawnInputDispatch) -> Vec<&str> {
    dispatch
        .invocations
        .iter()
        .filter(|invocation| invocation.event == NativeEventKind::Drop)
        .map(|invocation| invocation.action.as_str())
        .collect()
}

#[test]
fn internal_item_drop_dispatches_item_then_move_callbacks() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onItemDrop", "itemDrop"), ("onCollectionMove", "move")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    move_to(&mut runtime, 65.0, 30);

    let dropped = drop_at(&mut runtime, 65.0, 40);
    assert_eq!(drop_actions(&dropped), ["itemDrop", "move"]);
    let drag = dropped.invocations[0].context.drag.as_ref().unwrap();
    assert!(drag.is_internal);
    assert_eq!(
        drag.target,
        Some(SelfDrawnCollectionDropTarget::Item {
            key: "b".to_string(),
            drop_position: SelfDrawnDropPosition::On,
        })
    );
}

#[test]
fn internal_between_drop_dispatches_move_then_same_parent_reorder() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onCollectionMove", "move"), ("onReorder", "reorder")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    move_to(&mut runtime, 52.0, 30);

    let dropped = drop_at(&mut runtime, 52.0, 40);
    assert_eq!(drop_actions(&dropped), ["move", "reorder"]);
}

#[test]
fn reorder_rejects_cross_parent_drop_but_move_accepts_it() {
    let items = || {
        vec![
            item("a", 5, true, Some("parent-a")),
            item("b", 50, false, Some("parent-b")),
            item("c", 95, false, Some("parent-b")),
        ]
    };
    let mut reorder_only = runtime();
    reorder_only
        .render(collection(&[("onReorder", "reorder")], items()))
        .unwrap();
    start_drag(&mut reorder_only);
    let rejected = move_to(&mut reorder_only, 52.0, 30);
    assert_eq!(
        action_events(&rejected),
        [
            ("dragMove", NativeEventKind::DragMove),
            ("dropExit", NativeEventKind::DropExit),
        ]
    );
    assert!(drop_actions(&drop_at(&mut reorder_only, 52.0, 40)).is_empty());

    let mut movable = runtime();
    movable
        .render(collection(
            &[("onReorder", "reorder"), ("onCollectionMove", "move")],
            items(),
        ))
        .unwrap();
    start_drag(&mut movable);
    move_to(&mut movable, 52.0, 30);
    assert_eq!(drop_actions(&drop_at(&mut movable, 52.0, 40)), ["move"]);
}

#[test]
fn selected_descendants_are_filtered_and_cannot_receive_their_parent() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onItemDrop", "itemDrop"), ("onCollectionMove", "move")],
            vec![
                item("parent", 5, true, None),
                item("child", 50, true, Some("parent")),
                item("sibling", 95, false, None),
            ],
        ))
        .unwrap();
    let started = start_drag(&mut runtime);
    assert_eq!(
        started.invocations[0]
            .context
            .drag
            .as_ref()
            .unwrap()
            .dragging_keys,
        ["parent"]
    );

    let rejected = move_to(&mut runtime, 65.0, 30);
    assert_eq!(
        action_events(&rejected),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert!(drop_actions(&drop_at(&mut runtime, 65.0, 40)).is_empty());
}

#[test]
fn internal_insert_and_root_callbacks_do_not_accept_the_collection() {
    let mut runtime = runtime();
    runtime
        .render(collection(
            &[("onInsert", "insert"), ("onRootDrop", "rootDrop")],
            flat_items(),
        ))
        .unwrap();
    start_drag(&mut runtime);
    let between = move_to(&mut runtime, 52.0, 30);
    assert_eq!(
        action_events(&between),
        [("dragMove", NativeEventKind::DragMove)]
    );
    let root = move_to(&mut runtime, 140.0, 40);
    assert_eq!(
        action_events(&root),
        [("dragMove", NativeEventKind::DragMove)]
    );
    assert!(drop_actions(&drop_at(&mut runtime, 140.0, 50)).is_empty());
}
