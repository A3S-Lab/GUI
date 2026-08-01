use crate::platform_host::{PlatformKeyState, PlatformPointerPhase};
use crate::{GuiError, NativeEventKind, NativeInputModality, PlatformPoint};

use super::drag_drop_collection_tests::{collection_tree, start_selected_pointer_drag};
use super::drag_drop_tests::{action_events, drag_drop_tree, TARGET_A_ID, TARGET_B_ID};
use super::tests::{key_event, pointer_event, runtime};
use super::{
    SelfDrawnActionPropagation, SelfDrawnCollectionDropTarget, SelfDrawnDropOperation,
    SelfDrawnDropPosition,
};

fn start_pointer_drag_to_a(
    runtime: &mut super::SelfDrawnWindowRuntime<
        crate::platform_host::RecordingPlatformHost,
        super::RecordingScenePresenter,
    >,
) {
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 10.0, 20.0, 10))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 15.0, 20.0, 20))
        .unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 45.0, 20.0, 30))
        .unwrap();
}

#[test]
fn pointer_drop_activation_uses_fixed_deadline_latest_context_and_transactional_retry() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    start_pointer_drag_to_a(&mut runtime);
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_030));
    assert!(runtime.advance_interaction_time(800_029).unwrap().is_none());

    runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Moved,
            50.0,
            20.0,
            100_000,
        ))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_030));
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-white"))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_030));

    let sequence_before = runtime.event_sequence();
    let error = runtime
        .advance_interaction_time_with_reducer(800_030, |_| {
            Err(GuiError::host("injected drop-activate reducer failure"))
        })
        .unwrap_err();
    assert!(error.to_string().contains("injected drop-activate"));
    assert_eq!(runtime.event_sequence(), sequence_before);
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_030));

    let activated = runtime
        .advance_interaction_time_with_reducer(800_030, |_| {
            Ok(SelfDrawnActionPropagation::Continue)
        })
        .unwrap()
        .unwrap();
    assert_eq!(
        action_events(&activated),
        vec![("activatea", NativeEventKind::DropActivate)]
    );
    let invocation = &activated.invocations[0];
    assert_eq!(invocation.node.as_str(), TARGET_A_ID);
    assert_eq!(invocation.context.timestamp_micros, 800_030);
    assert_eq!(
        invocation.context.position,
        Some(PlatformPoint::new(10.0, 10.0))
    );
    assert_eq!(
        invocation.context.drag.as_ref().unwrap().drop_operation,
        SelfDrawnDropOperation::Move
    );
    assert_eq!(runtime.next_interaction_deadline_micros(), None);
    assert!(runtime.advance_interaction_time(900_000).unwrap().is_none());
}

#[test]
fn changing_or_leaving_a_drop_target_resets_or_cancels_activation() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    start_pointer_drag_to_a(&mut runtime);

    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 75.0, 20.0, 50))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_050));
    assert!(runtime.advance_interaction_time(800_030).unwrap().is_none());

    let activated = runtime.advance_interaction_time(800_050).unwrap().unwrap();
    assert_eq!(
        action_events(&activated),
        vec![("activateb", NativeEventKind::DropActivate)]
    );
    assert_eq!(activated.invocations[0].node.as_str(), TARGET_B_ID);

    runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Moved,
            45.0,
            20.0,
            900_000,
        ))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(1_700_000));
    runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Moved,
            99.0,
            70.0,
            910_000,
        ))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), None);
}

#[test]
fn keyboard_drag_schedules_drop_activation_without_a_pointer_position() {
    let mut runtime = runtime();
    runtime
        .render(drag_drop_tree("text/plain", "all", "all", "bg-black"))
        .unwrap();
    for (key, timestamp) in [("Tab", 10), ("Enter", 20), ("Tab", 30)] {
        runtime
            .handle_event(key_event(key, PlatformKeyState::Pressed, timestamp))
            .unwrap();
    }
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_030));

    let activated = runtime.advance_interaction_time(800_030).unwrap().unwrap();
    assert_eq!(
        action_events(&activated),
        vec![("activatea", NativeEventKind::DropActivate)]
    );
    assert_eq!(
        activated.invocations[0].context.modality,
        NativeInputModality::Keyboard
    );
    assert_eq!(activated.invocations[0].context.position, None);
}

#[test]
fn collection_drop_activation_ignores_root_and_reports_the_item_descriptor() {
    let mut runtime = runtime();
    runtime.render(collection_tree(false)).unwrap();
    start_selected_pointer_drag(&mut runtime);

    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 205.0, 125.0, 30))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), None);

    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Moved, 125.0, 18.0, 40))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(800_040));
    let activated = runtime.advance_interaction_time(800_040).unwrap().unwrap();
    assert_eq!(
        action_events(&activated),
        vec![("collectionActivate", NativeEventKind::DropActivate)]
    );
    assert_eq!(
        activated.invocations[0]
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
}
