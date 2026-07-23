use super::super::GuiRuntime;
use crate::native::{NativeElement, NativeProps, NativeRole};
use crate::platform::{Gtk4Adapter, PlatformPlanningHost};

#[test]
fn runtime_announces_live_region_text_updates_after_initial_render() {
    let status = |message: &str| {
        NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("status", NativeRole::View).with_props(
                NativeProps::new()
                    .explicit_role("status")
                    .label(message)
                    .live("polite")
                    .atomic(Some(true)),
            ),
        )
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_native(&status("Ready")).unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime.render_native(&status("Saved")).unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Saved");
    assert_eq!(
        announcements[0].priority,
        crate::accessibility::AccessibilityAnnouncementPriority::Polite
    );

    runtime.render_native(&status("Saved")).unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());
}

#[test]
fn runtime_announces_new_nested_live_regions_once_at_the_inner_priority() {
    let root = |message: Option<&str>| {
        let mut region = NativeElement::new("notifications", NativeRole::View).with_props(
            NativeProps::new()
                .explicit_role("region")
                .label("Notifications")
                .live("polite"),
        );
        if let Some(message) = message {
            region = region.child(
                NativeElement::new("toast", NativeRole::View)
                    .with_props(
                        NativeProps::new()
                            .explicit_role("alert")
                            .label(message)
                            .live("assertive")
                            .atomic(Some(true)),
                    )
                    .child(
                        NativeElement::new("toast-text", NativeRole::Text)
                            .with_props(NativeProps::new().label(message)),
                    ),
            );
        }
        NativeElement::new("root", NativeRole::View).child(region)
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_native(&root(None)).unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime.render_native(&root(Some("Upload failed"))).unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Upload failed");
    assert_eq!(
        announcements[0].priority,
        crate::accessibility::AccessibilityAnnouncementPriority::Assertive
    );
}

#[test]
fn runtime_announces_initial_alerts_but_not_initial_status_content() {
    let root = NativeElement::new("root", NativeRole::View)
        .child(
            NativeElement::new("status", NativeRole::View).with_props(
                NativeProps::new()
                    .explicit_role("status")
                    .label("Background sync complete"),
            ),
        )
        .child(
            NativeElement::new("alert", NativeRole::View).with_props(
                NativeProps::new()
                    .explicit_role("alert")
                    .label("Connection lost"),
            ),
        );
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_native(&root).unwrap();

    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Connection lost");
    assert_eq!(
        announcements[0].priority,
        crate::accessibility::AccessibilityAnnouncementPriority::Assertive
    );
}

#[test]
fn runtime_live_regions_respect_relevant_atomic_and_busy() {
    let region = |first: &str, second: Option<&str>, relevant: &str, atomic, busy| {
        let mut status = NativeElement::new("status", NativeRole::View)
            .with_props(
                NativeProps::new()
                    .live("polite")
                    .relevant(relevant)
                    .atomic(Some(atomic))
                    .busy(Some(busy)),
            )
            .child(
                NativeElement::new("first", NativeRole::Text)
                    .with_props(NativeProps::new().label(first)),
            );
        if let Some(second) = second {
            status = status.child(
                NativeElement::new("second", NativeRole::Text)
                    .with_props(NativeProps::new().label(second)),
            );
        }
        NativeElement::new("root", NativeRole::View).child(status)
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime
        .render_native(&region("Queued", None, "additions", false, false))
        .unwrap();
    runtime
        .render_native(&region("Processing", None, "additions", false, false))
        .unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime
        .render_native(&region(
            "Processing",
            Some("One item"),
            "additions",
            false,
            false,
        ))
        .unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "One item");

    runtime
        .render_native(&region("Processing", Some("Two items"), "all", true, true))
        .unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime
        .render_native(&region("Processing", Some("Two items"), "all", true, false))
        .unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Processing Two items");
}

#[test]
fn runtime_live_regions_announce_atomic_context_and_relevant_removals() {
    let region = |first: Option<&str>, second: &str, atomic: bool, relevant: &str| {
        let mut status = NativeElement::new("status", NativeRole::View).with_props(
            NativeProps::new()
                .live("polite")
                .atomic(Some(atomic))
                .relevant(relevant),
        );
        if let Some(first) = first {
            status = status.child(
                NativeElement::new("first", NativeRole::Text)
                    .with_props(NativeProps::new().label(first)),
            );
        }
        status = status.child(
            NativeElement::new("second", NativeRole::Text)
                .with_props(NativeProps::new().label(second)),
        );
        NativeElement::new("root", NativeRole::View).child(status)
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime
        .render_native(&region(Some("Queued"), "One item", true, "text"))
        .unwrap();
    runtime
        .render_native(&region(Some("Queued"), "Two items", true, "text"))
        .unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Queued Two items");

    runtime
        .render_native(&region(Some("Queued"), "Two items", false, "removals"))
        .unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());
    runtime
        .render_native(&region(None, "Two items", false, "removals"))
        .unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Queued");
}

#[test]
fn runtime_live_regions_defer_ancestor_busy_updates_and_redact_sensitive_values() {
    let root = |busy: bool, message: &str, password: &str| {
        NativeElement::new("root", NativeRole::View)
            .with_props(NativeProps::new().busy(Some(busy)))
            .child(
                NativeElement::new("status", NativeRole::View)
                    .with_props(NativeProps::new().live("polite").atomic(Some(true)))
                    .child(
                        NativeElement::new("credential", NativeRole::TextField).with_props(
                            NativeProps::new()
                                .input_type("password")
                                .label(message)
                                .value(password),
                        ),
                    ),
            )
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime
        .render_native(&root(false, "Credential ready", "first-secret"))
        .unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime
        .render_native(&root(true, "Credential saved", "second-secret"))
        .unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime
        .render_native(&root(false, "Credential saved", "second-secret"))
        .unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Credential saved");
    assert!(!announcements[0].message.contains("secret"));
}

#[test]
fn runtime_live_regions_only_flush_busy_regions_with_deferred_changes() {
    let root = |busy: bool, message: &str| {
        NativeElement::new("root", NativeRole::View).child(
            NativeElement::new("status", NativeRole::View)
                .with_props(
                    NativeProps::new()
                        .live("polite")
                        .atomic(Some(true))
                        .busy(Some(busy)),
                )
                .child(
                    NativeElement::new("message", NativeRole::Text)
                        .with_props(NativeProps::new().label(message)),
                ),
        )
    };
    let host = PlatformPlanningHost::new(Gtk4Adapter);
    let mut runtime = GuiRuntime::new(host);

    runtime.render_native(&root(false, "Ready")).unwrap();
    runtime.render_native(&root(true, "Ready")).unwrap();
    runtime.render_native(&root(false, "Ready")).unwrap();
    assert!(take_accessibility_announcements(&mut runtime).is_empty());

    runtime.render_native(&root(true, "Syncing")).unwrap();
    runtime.render_native(&root(true, "Complete")).unwrap();
    runtime.render_native(&root(false, "Complete")).unwrap();
    let announcements = take_accessibility_announcements(&mut runtime);
    assert_eq!(announcements.len(), 1);
    assert_eq!(announcements[0].message, "Complete");
}

fn take_accessibility_announcements(
    runtime: &mut GuiRuntime<PlatformPlanningHost<Gtk4Adapter>>,
) -> Vec<crate::accessibility::AccessibilityAnnouncement> {
    runtime
        .host_mut()
        .take_commands()
        .into_iter()
        .filter_map(|command| match command {
            crate::platform::PlatformCommand::AccessibilityAnnouncement { announcement } => {
                Some(announcement)
            }
            _ => None,
        })
        .collect()
}
