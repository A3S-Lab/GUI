use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform_host::{PlatformElementId, PlatformPointerId, PlatformPointerPhase};
use crate::web::WebProps;
use crate::{GuiError, NativeEventKind};

use super::tests::{pointer_event, pointer_event_for, runtime};
use super::{SelfDrawnActionPropagation, SelfDrawnHostEventOutcome};

fn long_press_tree(threshold_ms: u64) -> NativeElement {
    NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(
            NativeElement::new("hold", NativeRole::Button).with_props(
                NativeProps::new()
                    .label("Hold")
                    .metadata("threshold", threshold_ms.to_string())
                    .web(
                        WebProps::new()
                            .class_name(
                                "absolute left-[10px] top-[10px] h-[30px] w-[50px] bg-black",
                            )
                            .event("onLongPressStart", "longStart")
                            .event("onLongPressEnd", "longEnd")
                            .event("onLongPress", "longPress")
                            .event("onPressStart", "pressStart")
                            .event("onPressUp", "pressUp")
                            .event("onPressEnd", "pressEnd")
                            .event("onPress", "press"),
                    ),
            ),
        )
}

#[test]
fn scheduled_long_press_is_stable_and_reducer_failure_restores_its_deadline() {
    let mut runtime = runtime();
    runtime.render(long_press_tree(100)).unwrap();
    let target = PlatformElementId::new("4:root/4:hold").unwrap();

    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(100_010));
    assert!(runtime.element_interaction(&target).unwrap().long_pressed);
    assert!(runtime.advance_interaction_time(100_009).unwrap().is_none());

    let error = runtime
        .advance_interaction_time_with_reducer(100_010, |_| {
            Err(GuiError::host("injected long-press reducer failure"))
        })
        .unwrap_err();
    assert!(error.to_string().contains("injected long-press"));
    assert_eq!(runtime.event_sequence(), 1);
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(100_010));
    assert!(runtime.element_interaction(&target).unwrap().pressed);
    assert!(runtime.element_interaction(&target).unwrap().long_pressed);
    assert_eq!(runtime.stats().interaction_ticks, 0);
    assert_eq!(runtime.stats().reducer_failures, 1);

    let dispatch = runtime
        .advance_interaction_time_with_reducer(100_010, |_| {
            Ok(SelfDrawnActionPropagation::Continue)
        })
        .unwrap()
        .unwrap();
    assert_eq!(dispatch.event_sequence, 2);
    assert_eq!(
        dispatch
            .invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.event))
            .collect::<Vec<_>>(),
        vec![
            ("longEnd", NativeEventKind::LongPressEnd),
            ("pressEnd", NativeEventKind::PressCancel),
            ("longPress", NativeEventKind::LongPress),
        ]
    );
    assert!(!runtime.element_interaction(&target).unwrap().pressed);
    assert!(!runtime.element_interaction(&target).unwrap().long_pressed);
    assert_eq!(runtime.next_interaction_deadline_micros(), None);
    assert_eq!(runtime.stats().interaction_ticks, 1);
}

#[test]
fn simultaneous_long_press_deadlines_drain_earliest_then_stable_pointer() {
    let mut runtime = runtime();
    runtime.render(long_press_tree(100)).unwrap();
    for (pointer, timestamp) in [(2, 10), (1, 20)] {
        runtime
            .handle_event(pointer_event_for(
                PlatformPointerId::new(pointer),
                PlatformPointerPhase::Pressed,
                20.0,
                20.0,
                timestamp,
            ))
            .unwrap();
    }
    let target = PlatformElementId::new("4:root/4:hold").unwrap();

    let first = runtime.advance_interaction_time(100_020).unwrap().unwrap();
    assert_eq!(first.event_sequence, 3);
    assert_eq!(
        first.invocations[0].context.pointer,
        Some(PlatformPointerId::new(2))
    );
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(100_020));
    assert!(runtime.element_interaction(&target).unwrap().pressed);
    assert!(runtime.element_interaction(&target).unwrap().long_pressed);

    let second = runtime.advance_interaction_time(100_020).unwrap().unwrap();
    assert_eq!(second.event_sequence, 4);
    assert_eq!(
        second.invocations[0].context.pointer,
        Some(PlatformPointerId::new(1))
    );
    assert_eq!(runtime.next_interaction_deadline_micros(), None);
    assert!(!runtime.element_interaction(&target).unwrap().pressed);
    assert!(!runtime.element_interaction(&target).unwrap().long_pressed);
}

#[test]
fn release_after_threshold_recognizes_long_press_without_a_scheduled_callback() {
    let mut runtime = runtime();
    runtime.render(long_press_tree(100)).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();

    let outcome = runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Released,
            20.0,
            20.0,
            100_010,
        ))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("release should route long-press fallback");
    };
    assert_eq!(
        dispatch
            .invocations
            .iter()
            .map(|invocation| (invocation.action.as_str(), invocation.event))
            .collect::<Vec<_>>(),
        vec![
            ("longEnd", NativeEventKind::LongPressEnd),
            ("pressEnd", NativeEventKind::PressCancel),
            ("longPress", NativeEventKind::LongPress),
        ]
    );
}

#[test]
fn leaving_cancels_long_press_and_reentry_starts_a_fresh_deadline() {
    let mut runtime = runtime();
    runtime.render(long_press_tree(100)).unwrap();
    runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();

    runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Moved,
            90.0,
            70.0,
            50_000,
        ))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), None);
    runtime
        .handle_event(pointer_event(
            PlatformPointerPhase::Moved,
            20.0,
            20.0,
            60_000,
        ))
        .unwrap();
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(160_000));
    assert!(runtime.advance_interaction_time(100_010).unwrap().is_none());
    assert!(runtime.advance_interaction_time(160_000).unwrap().is_some());
}

#[test]
fn style_only_long_press_variant_is_a_hit_target_without_an_action_binding() {
    let root = NativeElement::new("root", NativeRole::View)
        .with_props(
            NativeProps::new()
                .web(WebProps::new().class_name("relative h-[80px] w-[100px] bg-white")),
        )
        .child(NativeElement::new("hold", NativeRole::View).with_props(
            NativeProps::new().web(WebProps::new().class_name(
                "absolute left-[10px] top-[10px] h-[30px] w-[50px] data-[long-pressed=true]:opacity-60",
            )),
        ));
    let mut runtime = runtime();
    runtime.render(root).unwrap();

    let outcome = runtime
        .handle_event(pointer_event(PlatformPointerPhase::Pressed, 20.0, 20.0, 10))
        .unwrap();
    let SelfDrawnHostEventOutcome::Input(dispatch) = outcome else {
        panic!("style interaction should be hit tested");
    };
    let target = dispatch.target.unwrap();
    assert_eq!(target.as_str(), "4:root/4:hold");
    assert!(dispatch.invocations.is_empty());
    assert!(runtime.element_interaction(&target).unwrap().long_pressed);
    assert_eq!(runtime.next_interaction_deadline_micros(), Some(500_010));
}
