#[path = "support/dogfood_app.rs"]
mod dogfood_app;

use a3s_gui::{CommandExecutingHost, Gtk4Adapter, NativeRuntimeApp, RecordingBackend};

use crate::dogfood_app::{dogfood_frame, dogfood_reduce, DogfoodState};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = NativeRuntimeApp::new(
        host,
        DogfoodState::default(),
        dogfood_session_frame,
        dogfood_reduce,
    );
    let rendered = app.render()?;
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
    use a3s_gui::{HostNodeId, NativeEvent, NativeEventKind, NativeWidgetBlueprint};

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
    }

    #[test]
    fn dogfood_frame_projects_native_size_and_focus_hints() {
        let mut app = new_app();
        app.render().unwrap();

        let root = find_event_blueprint(&app, "onKeyDown", "handleShortcut").1;
        let size = root.portable_style.native_size_constraints();
        assert_eq!(size.width, Some(680.0));
        assert_eq!(size.height, Some(500.0));
        assert_eq!(size.min_width, Some(480.0));
        assert_eq!(size.min_height, Some(360.0));

        let title = find_event_blueprint(&app, "onInput", "updateTitle").1;
        assert!(title.control_state.auto_focus);
        assert_eq!(
            title.portable_style.native_size_constraints().width,
            Some(620.0)
        );
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
}
