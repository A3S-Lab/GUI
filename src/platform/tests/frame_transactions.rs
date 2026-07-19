use crate::host::NativeHost;
use crate::native::{NativeElement, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};

#[test]
fn planning_frame_rollback_restores_exact_state_after_mid_frame_failure() {
    let mut host = PlatformPlanningHost::new(Gtk4Adapter);
    let stable = host
        .create(&NativeElement::new("stable", NativeRole::View))
        .unwrap();
    host.set_root(stable).unwrap();
    host.clear_commands();

    host.begin_frame().unwrap();
    let transient = host
        .create(&NativeElement::new("transient", NativeRole::Button))
        .unwrap();
    host.insert_child(stable, transient, 0).unwrap();
    let error = host.insert_child(transient, transient, 0).unwrap_err();
    assert!(error.to_string().contains("into itself"));
    assert!(host.node(transient).is_some());
    assert!(!host.commands().is_empty());

    host.rollback_frame().unwrap();

    assert_eq!(host.root(), Some(stable));
    assert_eq!(host.nodes().len(), 1);
    assert!(host.node(transient).is_none());
    assert!(host.node(stable).unwrap().children.is_empty());
    assert!(host.commands().is_empty());

    // The allocator is part of the frame checkpoint as well.
    let reused = host
        .create(&NativeElement::new("reused", NativeRole::Button))
        .unwrap();
    assert_eq!(reused, transient);
}

#[test]
fn planning_frame_commit_keeps_commands_for_protocol_drain() {
    let mut host = PlatformPlanningHost::new(Gtk4Adapter);
    host.begin_frame().unwrap();
    let root = host
        .create(&NativeElement::new("root", NativeRole::View))
        .unwrap();
    host.set_root(root).unwrap();

    let ack = host.commit_frame().unwrap();

    assert_eq!(ack.batch_id, None);
    assert_eq!(ack.applied_operations, 2);
    assert_eq!(host.commands().len(), 2);
    assert_eq!(host.take_commands().len(), 2);
    assert!(host.commands().is_empty());
}
