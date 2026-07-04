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
    use a3s_gui::{
        HostEvent, HostNodeId, NativeEvent, NativeEventKind, NativeProtocolApp, NativeRole,
        NativeRuntimeEventResponse, NativeWidgetBlueprint, PlatformCommand,
    };

    type DogfoodTestApp = NativeRuntimeApp<
        CommandExecutingHost<Gtk4Adapter, RecordingBackend>,
        DogfoodState,
        fn(&DogfoodState) -> a3s_gui::GuiResult<a3s_gui::UiFrame>,
        fn(&mut DogfoodState, &a3s_gui::ActionInvocation) -> a3s_gui::GuiResult<()>,
    >;
    type DogfoodProtocolApp = NativeProtocolApp<
        Gtk4Adapter,
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
            "onChange",
            "setEstimate",
            NativeEventKind::Change,
            "99",
        );
        assert_eq!(app.state().estimate, 12.0);

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
        assert_eq!(root.widget_class, "gtk::ScrolledWindow+Box");
        let size = root.portable_style.native_size_constraints();
        assert_eq!(size.width, Some(700.0));
        assert_eq!(size.height, Some(620.0));
        assert_eq!(size.min_width, Some(480.0));
        assert_eq!(size.min_height, Some(420.0));

        let title = find_event_blueprint(&app, "onInput", "updateTitle").1;
        assert!(title.control_state.auto_focus);
        assert!(title.control_state.required);
        assert!(!title.control_state.invalid);
        assert_eq!(title.control_state.size, Some(48));
        assert_eq!(title.control_state.max_length, Some(96));
        assert_eq!(
            title.portable_style.native_size_constraints().width,
            Some(640.0)
        );
        let notes = find_event_blueprint(&app, "onInput", "updateNotes").1;
        assert_eq!(notes.control_state.rows, Some(4));
        assert_eq!(notes.control_state.cols, Some(54));
        assert_eq!(notes.control_state.max_length, Some(240));

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
    fn dogfood_session_covers_long_form_focus_and_input_edges() {
        let mut app = new_app();
        app.render().unwrap();

        let title_node = find_event_blueprint(&app, "onFocus", "focusTitle").0;
        assert!(app
            .runtime()
            .interactions()
            .node(title_node)
            .is_some_and(|state| state.focused));
        let response =
            dispatch_response(&mut app, "onBlur", "blurTitle", NativeEventKind::Blur, "");
        assert_eq!(app.state().focused_field, "none");
        assert_eq!(app.state().last_event, "Blurred title");
        assert!(response
            .interaction_changes
            .iter()
            .any(|change| { change.node == title_node && !change.after.focused }));

        let response = dispatch_response(
            &mut app,
            "onFocus",
            "focusTitle",
            NativeEventKind::Focus,
            "",
        );
        assert_eq!(app.state().focused_field, "title");
        assert_eq!(app.state().last_event, "Focused title");
        assert_eq!(response.interaction_changes.len(), 1);
        assert_eq!(response.interaction_changes[0].node, title_node);
        assert!(response.interaction_changes[0].after.focused);
        assert!(app
            .runtime()
            .interactions()
            .node(title_node)
            .is_some_and(|state| state.focused));
        assert_eq!(
            find_blueprint_by_label(&app, NativeRole::TextField, "Focus status")
                .value
                .as_deref(),
            Some("Focused field: title")
        );

        let long_title = format!("{}{}", "Native GUI hardening ".repeat(8), "aé日");
        let expected_title = truncate_chars(&long_title, 96);
        let response = dispatch_response(
            &mut app,
            "onInput",
            "updateTitle",
            NativeEventKind::Change,
            &long_title,
        );
        assert_eq!(app.state().title, expected_title);
        assert_eq!(
            response.event.value.as_deref(),
            Some(expected_title.as_str())
        );
        assert_eq!(
            response
                .invocation
                .as_ref()
                .and_then(|invocation| invocation.value.as_deref()),
            Some(expected_title.as_str())
        );
        assert!(response.interaction_changes.iter().any(|change| {
            change.node == title_node
                && change.after.value.as_deref() == Some(expected_title.as_str())
        }));
        assert_eq!(
            find_event_blueprint(&app, "onInput", "updateTitle")
                .1
                .value
                .as_deref(),
            Some(expected_title.as_str())
        );

        let title_node = find_event_blueprint(&app, "onFocus", "focusTitle").0;
        let notes_node = find_event_blueprint(&app, "onFocus", "focusNotes").0;
        let response = dispatch_response(
            &mut app,
            "onFocus",
            "focusNotes",
            NativeEventKind::Focus,
            "",
        );
        assert_eq!(app.state().focused_field, "notes");
        assert_eq!(app.state().last_event, "Focused notes");
        assert!(response
            .interaction_changes
            .iter()
            .any(|change| { change.node == title_node && !change.after.focused }));
        assert!(response
            .interaction_changes
            .iter()
            .any(|change| { change.node == notes_node && change.after.focused }));
        assert!(app
            .runtime()
            .interactions()
            .node(notes_node)
            .is_some_and(|state| state.focused));

        let long_notes = (0..32)
            .map(|index| format!("Line {index}: verify resize, focus, and input paths aé日.\n"))
            .collect::<String>();
        let expected_notes = truncate_chars(&long_notes, 240);
        let response = dispatch_response(
            &mut app,
            "onInput",
            "updateNotes",
            NativeEventKind::Change,
            &long_notes,
        );
        assert_eq!(app.state().notes, expected_notes);
        assert_eq!(
            response.event.value.as_deref(),
            Some(expected_notes.as_str())
        );
        assert!(response.interaction_changes.iter().any(|change| {
            change.node == notes_node
                && change.after.value.as_deref() == Some(expected_notes.as_str())
        }));
        assert_eq!(
            find_event_blueprint(&app, "onInput", "updateNotes")
                .1
                .value
                .as_deref(),
            Some(expected_notes.as_str())
        );

        let notes_node = find_event_blueprint(&app, "onBlur", "blurNotes").0;
        let response =
            dispatch_response(&mut app, "onBlur", "blurNotes", NativeEventKind::Blur, "");
        assert_eq!(app.state().focused_field, "none");
        assert_eq!(app.state().last_event, "Blurred notes");
        assert!(response
            .interaction_changes
            .iter()
            .any(|change| { change.node == notes_node && !change.after.focused }));
        assert_eq!(
            find_blueprint_by_label(&app, NativeRole::TextField, "Focus status")
                .value
                .as_deref(),
            Some("Focused field: none")
        );
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

    #[test]
    fn dogfood_disabled_review_completion_actions_are_suppressed() {
        let mut app = new_app();
        app.render().unwrap();
        let before = app.state().clone();
        let finish_nodes = app
            .runtime()
            .host()
            .planning()
            .nodes()
            .iter()
            .filter_map(|(id, node)| {
                (node.blueprint.events.get("onPress").map(String::as_str) == Some("finishReview"))
                    .then_some((
                        *id,
                        node.blueprint.role,
                        node.blueprint.label.clone(),
                        node.blueprint.control_state.disabled,
                    ))
            })
            .collect::<Vec<_>>();

        assert!(finish_nodes
            .iter()
            .any(|(_, role, label, disabled)| *role == NativeRole::MenuItem
                && label.as_deref() == Some("Complete review")
                && *disabled));
        assert!(finish_nodes
            .iter()
            .any(|(_, role, label, disabled)| *role == NativeRole::Button
                && label.as_deref() == Some("Complete review")
                && *disabled));

        for (node, _, _, _) in finish_nodes {
            let response = app
                .handle_native_event(NativeEvent::new(node, NativeEventKind::Press))
                .unwrap();

            assert!(response.invocation.is_none());
            assert!(response.render.is_none());
            assert_eq!(app.state(), &before);
        }
    }

    #[test]
    fn dogfood_protocol_app_replays_host_boundary_workflow() {
        let mut app = new_protocol_app();
        let rendered = app.render().unwrap();

        assert_eq!(rendered.frame_id, "dogfood-session");
        assert!(rendered.commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Create { blueprint, .. } if blueprint.role == NativeRole::Window
        )));

        let response = dispatch_host(
            &mut app,
            "onPress",
            "requestReview",
            NativeEventKind::Press,
            "",
        );
        assert!(app.state().review_open);
        assert_eq!(app.state().stage, "Review");
        assert_render_updated_dialog(&response, true);

        dispatch_host(
            &mut app,
            "onChange",
            "setDesignReviewed",
            NativeEventKind::Toggle,
            "true",
        );
        dispatch_host(
            &mut app,
            "onChange",
            "setTestsReviewed",
            NativeEventKind::Toggle,
            "true",
        );
        dispatch_host(
            &mut app,
            "onChange",
            "setDocsUpdated",
            NativeEventKind::Toggle,
            "true",
        );
        assert!(app.state().review_ready());

        let response = dispatch_host(
            &mut app,
            "onPress",
            "finishReview",
            NativeEventKind::Press,
            "",
        );
        assert_eq!(app.state().stage, "Done");
        assert!(app.state().completed);
        assert!(!app.state().review_open);
        assert_render_updated_dialog(&response, false);

        let response = dispatch_host(
            &mut app,
            "onClose",
            "closeDogfood",
            NativeEventKind::Close,
            "",
        );
        assert!(app.state().close_requested);
        assert!(!dogfood_should_continue(app.state()));
        assert_eq!(app.state().last_event, "Window close requested");
        assert!(response.render.is_some());
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

    fn new_protocol_app() -> DogfoodProtocolApp {
        NativeProtocolApp::new(
            Gtk4Adapter,
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
        dispatch_response(app, event_name, action, kind, value);
    }

    fn dispatch_response(
        app: &mut DogfoodTestApp,
        event_name: &str,
        action: &str,
        kind: NativeEventKind,
        value: &str,
    ) -> NativeRuntimeEventResponse {
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
        response
    }

    fn dispatch_host(
        app: &mut DogfoodProtocolApp,
        event_name: &str,
        action: &str,
        kind: NativeEventKind,
        value: &str,
    ) -> a3s_gui::NativeAppEventResponse {
        let node = find_protocol_event_blueprint(app, event_name, action).0;
        let event = if value.is_empty() {
            NativeEvent::new(node, kind)
        } else {
            NativeEvent::new(node, kind).value(value)
        };
        let response = app
            .handle_host_event(&HostEvent {
                frame_id: "dogfood-session".to_string(),
                event,
            })
            .unwrap();
        assert_eq!(
            response
                .invocation
                .as_ref()
                .map(|invocation| invocation.action.as_str()),
            Some(action)
        );
        assert!(response.render.is_some());
        response
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

    fn find_protocol_event_blueprint<'a>(
        app: &'a DogfoodProtocolApp,
        event_name: &str,
        action: &str,
    ) -> (HostNodeId, &'a NativeWidgetBlueprint) {
        app.session()
            .runtime()
            .host()
            .nodes()
            .iter()
            .find_map(|(id, node)| {
                (node.blueprint.events.get(event_name).map(String::as_str) == Some(action))
                    .then_some((*id, &node.blueprint))
            })
            .unwrap_or_else(|| panic!("missing protocol node for {event_name} -> {action}"))
    }

    fn assert_render_updated_dialog(response: &a3s_gui::NativeAppEventResponse, open: bool) {
        let render = response
            .render
            .as_ref()
            .expect("state action should render a follow-up frame");
        assert!(render.commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Update { blueprint, .. }
                if blueprint.role == NativeRole::Dialog
                    && blueprint.control_state.html_dialog.open == Some(open)
        )));
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

    fn truncate_chars(value: &str, max: usize) -> String {
        value.chars().take(max).collect()
    }
}
