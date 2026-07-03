#[path = "support/dogfood_app.rs"]
mod dogfood_app;

use a3s_gui::{CommandExecutingHost, Gtk4Adapter, NativeRuntimeApp, RecordingBackend};

use crate::dogfood_app::{dogfood_frame, dogfood_reduce, dogfood_should_continue, DogfoodState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(
        host,
        DogfoodState::default(),
        dogfood_session_frame,
        dogfood_reduce,
    );
    let rendered = app.render()?;
    assert!(dogfood_should_continue(app.state()));
    println!(
        "dogfood frame {} rendered as node {}",
        rendered.frame_id,
        rendered.root.get()
    );
    Ok(())
}

fn dogfood_session_frame(state: &DogfoodState) -> a3s_gui::GuiResult<a3s_gui::UiFrame> {
    dogfood_frame(state, "dogfood-session", "A3S GUI Dogfood")
}

#[cfg(test)]
mod tests {
    use super::*;
    use a3s_gui::{HostNodeId, NativeEvent, NativeEventKind, NativeRole, NativeWidgetBlueprint};

    type DogfoodTestApp = NativeRuntimeApp<
        CommandExecutingHost<Gtk4Adapter, RecordingBackend>,
        DogfoodState,
        fn(&DogfoodState) -> a3s_gui::GuiResult<a3s_gui::UiFrame>,
        fn(&mut DogfoodState, &a3s_gui::ActionInvocation) -> a3s_gui::GuiResult<()>,
    >;

    #[test]
    fn dogfood_session_reduces_realistic_native_events() {
        let mut app = new_app();
        let rendered = app.render().unwrap();
        assert_eq!(rendered.frame_id, "dogfood-session");

        dispatch(
            &mut app,
            "onInput",
            "updateTitle",
            NativeEventKind::Change,
            "Harden layout",
        );
        assert_eq!(app.state().title, "Harden layout");
        assert_eq!(
            app.runtime().accessibility_tree().unwrap().label.as_deref(),
            Some("A3S GUI Dogfood")
        );

        dispatch(
            &mut app,
            "onSelectionChange",
            "setPriority",
            NativeEventKind::SelectionChange,
            "Normal",
        );
        assert_eq!(app.state().priority, "Normal");

        dispatch(
            &mut app,
            "onSelectionChange",
            "setAssignee",
            NativeEventKind::SelectionChange,
            "Grace",
        );
        assert_eq!(app.state().assignee, "Grace");

        dispatch(
            &mut app,
            "onChange",
            "setCompleted",
            NativeEventKind::Toggle,
            "true",
        );
        assert!(app.state().completed);

        dispatch(
            &mut app,
            "onChange",
            "setEstimate",
            NativeEventKind::Change,
            "9",
        );
        assert_eq!(app.state().estimate, 9.0);

        dispatch(
            &mut app,
            "onSelectionChange",
            "setStage",
            NativeEventKind::SelectionChange,
            "Review",
        );
        assert_eq!(app.state().stage, "Review");

        dispatch(&mut app, "onPress", "saveWork", NativeEventKind::Press, "");
        assert_eq!(app.state().saves, 1);

        dispatch(
            &mut app,
            "onKeyDown",
            "handleShortcut",
            NativeEventKind::KeyDown,
            "Enter",
        );
        assert_eq!(app.state().saves, 2);
        assert_eq!(app.state().last_event, "Saved from shortcut");

        dispatch(
            &mut app,
            "onKeyUp",
            "handleShortcutRelease",
            NativeEventKind::KeyUp,
            "Enter",
        );
        assert_eq!(app.state().last_event, "Released Enter");
    }

    #[test]
    fn dogfood_session_reduces_window_close_event() {
        let mut app = new_app();
        let rendered = app.render().unwrap();

        let response = app
            .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Close))
            .unwrap();

        assert!(app.state().close_requested);
        assert!(!app.state().review_open);
        assert_eq!(app.state().last_event, "Window close requested");
        assert_eq!(
            response
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some("closeDogfood")
        );
        assert!(response.render.is_some());
    }

    #[test]
    fn dogfood_session_close_menu_requests_app_exit() {
        let mut app = new_app();
        app.render().unwrap();

        let close = find_event_blueprint(&app, "onPress", "closeDogfood").1;
        assert_eq!(close.role, NativeRole::MenuItem);
        assert!(dogfood_should_continue(app.state()));

        dispatch(
            &mut app,
            "onPress",
            "closeDogfood",
            NativeEventKind::Press,
            "",
        );

        assert!(app.state().close_requested);
        assert_eq!(app.state().last_event, "Window close requested");
        assert!(!dogfood_should_continue(app.state()));
    }

    #[test]
    fn dogfood_frame_projects_native_size_and_focus_hints() {
        let mut app = new_app();
        app.render().unwrap();

        let frame = dogfood_session_frame(&DogfoodState::default()).unwrap();
        assert_eq!(
            frame
                .window
                .as_ref()
                .and_then(|window| window.on_close.as_deref()),
            Some("closeDogfood")
        );
        assert!(frame
            .actions
            .iter()
            .any(|action| action.id == "closeDogfood"));

        let window = find_event_blueprint(&app, "onClose", "closeDogfood").1;
        assert_eq!(window.role, NativeRole::Window);

        let root = find_event_blueprint(&app, "onKeyDown", "handleShortcut").1;
        let size = root.portable_style.native_size_constraints();
        assert_eq!(size.width, Some(700.0));
        assert_eq!(size.height, Some(620.0));
        assert_eq!(size.min_width, Some(480.0));
        assert_eq!(size.min_height, Some(420.0));

        let title = find_event_blueprint(&app, "onInput", "updateTitle").1;
        assert!(title.control_state.auto_focus);
        assert!(title.control_state.required);
        assert!(!title.control_state.invalid);
        assert_eq!(
            title.portable_style.native_size_constraints().width,
            Some(640.0)
        );

        dispatch(
            &mut app,
            "onInput",
            "updateTitle",
            NativeEventKind::Change,
            "",
        );
        let title = find_event_blueprint(&app, "onInput", "updateTitle").1;
        assert!(title.control_state.invalid);
    }

    #[test]
    fn dogfood_review_workflow_projects_menu_dialog_and_gates_completion() {
        let mut app = new_app();
        app.render().unwrap();

        let request_review = find_event_blueprint(&app, "onPress", "requestReview").1;
        assert_eq!(request_review.role, NativeRole::MenuItem);

        let review_dialog = find_blueprint_by_label(&app, NativeRole::Dialog, "Review gate");
        assert!(!review_dialog.config().visible);
        assert_eq!(review_dialog.control_state.html_dialog.open, Some(false));

        let review_status = find_blueprint_by_label(&app, NativeRole::TextField, "Review status");
        assert!(review_status.control_state.read_only);
        assert_eq!(
            review_status.value.as_deref(),
            Some("0/3 review checks complete")
        );

        let complete_review = find_blueprint_by_label(&app, NativeRole::Button, "Complete review");
        assert!(complete_review.control_state.disabled);

        dispatch(
            &mut app,
            "onPress",
            "requestReview",
            NativeEventKind::Press,
            "",
        );
        assert!(app.state().review_open);
        assert_eq!(app.state().stage, "Review");
        let review_dialog = find_blueprint_by_label(&app, NativeRole::Dialog, "Review gate");
        assert!(review_dialog.config().visible);
        assert_eq!(review_dialog.control_state.html_dialog.open, Some(true));

        dispatch(
            &mut app,
            "onChange",
            "setDesignReviewed",
            NativeEventKind::Toggle,
            "true",
        );
        dispatch(
            &mut app,
            "onChange",
            "setTestsReviewed",
            NativeEventKind::Toggle,
            "true",
        );
        dispatch(
            &mut app,
            "onChange",
            "setDocsUpdated",
            NativeEventKind::Toggle,
            "true",
        );
        assert!(app.state().review_ready());

        let complete_review = find_blueprint_by_label(&app, NativeRole::Button, "Complete review");
        assert!(!complete_review.control_state.disabled);
        dispatch(
            &mut app,
            "onPress",
            "finishReview",
            NativeEventKind::Press,
            "",
        );
        assert_eq!(app.state().stage, "Done");
        assert!(app.state().completed);
        assert!(!app.state().review_open);

        dispatch(
            &mut app,
            "onPress",
            "reopenWork",
            NativeEventKind::Press,
            "",
        );
        assert_eq!(app.state().stage, "Build");
        assert!(!app.state().completed);
        assert!(!app.state().design_reviewed);
        assert!(!app.state().review_ready());
    }

    fn new_app() -> DogfoodTestApp {
        let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
        NativeRuntimeApp::new(
            host,
            DogfoodState::default(),
            dogfood_session_frame,
            dogfood_reduce,
        )
    }

    fn dispatch(
        app: &mut DogfoodTestApp,
        event_name: &str,
        action: &str,
        kind: NativeEventKind,
        value: &str,
    ) {
        let node = find_event_blueprint(app, event_name, action).0;
        let event = if value.is_empty() {
            NativeEvent::new(node, kind)
        } else {
            NativeEvent::new(node, kind).value(value)
        };
        let response = app.dispatch_native_event(event).unwrap();
        assert_eq!(
            response
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some(action)
        );
        assert!(response.render.is_some());
    }

    fn find_event_blueprint<'a>(
        app: &'a DogfoodTestApp,
        event_name: &str,
        action: &str,
    ) -> (HostNodeId, &'a NativeWidgetBlueprint) {
        app.runtime()
            .host()
            .planning()
            .nodes()
            .iter()
            .find_map(|(id, node)| {
                (node.blueprint.events.get(event_name).map(String::as_str) == Some(action))
                    .then_some((*id, &node.blueprint))
            })
            .unwrap_or_else(|| panic!("missing node for {event_name} -> {action}"))
    }

    fn find_blueprint_by_label<'a>(
        app: &'a DogfoodTestApp,
        role: NativeRole,
        label: &str,
    ) -> &'a NativeWidgetBlueprint {
        app.runtime()
            .host()
            .planning()
            .nodes()
            .values()
            .find_map(|node| {
                (node.blueprint.role == role && node.blueprint.label.as_deref() == Some(label))
                    .then_some(&node.blueprint)
            })
            .unwrap_or_else(|| panic!("missing {role:?} blueprint labeled {label:?}"))
    }
}
