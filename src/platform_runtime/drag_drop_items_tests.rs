use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{PlatformKeyState, PlatformPointerPhase};
use crate::web::WebProps;
use crate::NativeEventKind;

use super::drag_drop_tests::{action_events, input};
use super::tests::{key_event, pointer_event, runtime};

const MULTI_ITEMS: &str = r#"[
  {
    "text/plain": "alpha",
    "text/html": "<strong>alpha</strong>",
    "application/x-a3s-item": "{\"id\":1}"
  },
  {
    "text/plain": "beta",
    "application/json": "{\"id\":2}"
  }
]"#;

fn multi_item_tree(target_tab_index: i32) -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("source", NativeRole::View).with_props(
                NativeProps::new().tab_index(Some(0)).draggable("true").web(
                    WebProps::new()
                        .class_name("absolute left-[5px] top-[10px] h-[30px] w-[25px] bg-black")
                        .attribute("data-drag-items", MULTI_ITEMS)
                        .event("onDragStart", "dragStart")
                        .event("onDragMove", "dragMove")
                        .event("onDragEnd", "dragEnd"),
                ),
            ),
        )
        .child(
            NativeElement::new("target", NativeRole::View).with_props(
                NativeProps::new().tab_index(Some(target_tab_index)).web(
                    WebProps::new()
                        .class_name("absolute left-[40px] top-[10px] h-[30px] w-[25px] bg-white")
                        .attribute("data-accepted-drag-types", "application/json")
                        .attribute("data-drop-operation", "copy")
                        .event("onDropEnter", "enter")
                        .event("onDropMove", "move")
                        .event("onDropExit", "exit")
                        .event("onDrop", "drop"),
                ),
            ),
        )
}

fn assert_full_source_items(context: &super::SelfDrawnDragContext) {
    assert_eq!(context.items.len(), 2);
    assert_eq!(context.items[0].kind(), "text");
    assert_eq!(
        context.items[0].get_text("text/html"),
        Some("<strong>alpha</strong>")
    );
    assert_eq!(
        context.items[0].get_text("application/x-a3s-item"),
        Some("{\"id\":1}")
    );
    assert_eq!(context.items[1].get_text("text/plain"), Some("beta"));
    assert_eq!(
        context.items[1].get_text("application/json"),
        Some("{\"id\":2}")
    );
}

fn assert_filtered_target_items(context: &super::SelfDrawnDragContext) {
    assert_eq!(context.items.len(), 1);
    assert_eq!(context.items[0].get_text("text/plain"), Some("beta"));
    assert_eq!(
        context.items[0].get_text("application/json"),
        Some("{\"id\":2}")
    );
    assert_eq!(context.items[0].types(), ["application/json", "text/plain"]);
}

#[test]
fn pointer_drag_preserves_per_item_formats_and_filters_target_items() {
    let mut runtime = runtime();
    runtime.render(multi_item_tree(-1)).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();
    let started = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20))
            .unwrap(),
    );
    assert_eq!(
        action_events(&started),
        vec![
            ("dragStart", NativeEventKind::DragStart),
            ("dragMove", NativeEventKind::DragMove),
        ]
    );
    assert_full_source_items(started.invocations[0].context.drag.as_ref().unwrap());
    assert_eq!(
        started.invocations[0].context.drag.as_ref().unwrap().types,
        [
            "application/x-a3s-item",
            "text/html",
            "text/plain",
            "application/json",
        ]
    );

    let entered = input(
        runtime
            .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
            .unwrap(),
    );
    assert_eq!(
        action_events(&entered),
        vec![
            ("dragMove", NativeEventKind::DragMove),
            ("enter", NativeEventKind::DropEnter),
            ("move", NativeEventKind::DropMove),
        ]
    );
    assert_filtered_target_items(entered.invocations[1].context.drag.as_ref().unwrap());
    assert_filtered_target_items(entered.invocations[2].context.drag.as_ref().unwrap());

    let dropped = input(
        runtime
            .handle_event(pointer_event(
                PlatformPointerPhase::Released,
                45.0,
                20.0,
                40,
            ))
            .unwrap(),
    );
    assert_eq!(
        action_events(&dropped),
        vec![
            ("drop", NativeEventKind::Drop),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_filtered_target_items(dropped.invocations[0].context.drag.as_ref().unwrap());
    assert_full_source_items(dropped.invocations[1].context.drag.as_ref().unwrap());
}

#[test]
fn keyboard_drop_carries_the_same_filtered_item_contract() {
    let mut runtime = runtime();
    runtime.render(multi_item_tree(0)).unwrap();
    for (key, timestamp) in [("Tab", 10), ("Enter", 20), ("Tab", 30)] {
        runtime
            .handle_event(key_event(key, PlatformKeyState::Pressed, timestamp))
            .unwrap();
    }
    let dropped = input(
        runtime
            .handle_event(key_event("Enter", PlatformKeyState::Pressed, 40))
            .unwrap(),
    );
    assert_eq!(
        action_events(&dropped),
        vec![
            ("drop", NativeEventKind::Drop),
            ("dragEnd", NativeEventKind::DragEnd),
        ]
    );
    assert_filtered_target_items(dropped.invocations[0].context.drag.as_ref().unwrap());
    assert_full_source_items(dropped.invocations[1].context.drag.as_ref().unwrap());
}
