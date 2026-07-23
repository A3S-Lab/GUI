use super::*;
use crate::backend::{CommandExecutingHost, RecordingBackend};
use crate::compiler::{CompiledOrientation, CompiledProps, CompiledRsxNode, CompiledStyleValue};
use crate::event::{ActionInvocation, NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::platform::Gtk4Adapter;
use crate::protocol::HostEvent;
use serde::{Deserialize, Serialize};

mod actions;
mod composition;
mod effects;

#[derive(Debug, Clone, PartialEq, Default)]
struct CounterState {
    count: u32,
}

#[test]
fn rsx_component_hooks_render_state_props_and_reduce_actions() {
    let component = counter_component();
    let mut app = component.into_protocol_app(Gtk4Adapter, CounterState::default());
    let rendered = app.render().unwrap();

    assert_eq!(
        rendered
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 0")
    );

    let response = app
        .dispatch_host_event(&HostEvent {
            frame_id: "counter".to_string(),
            event: NativeEvent::new(rendered.root, NativeEventKind::Press),
        })
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1")
    );
}

#[test]
fn component_cx_bare_compile_does_not_install_default_components() {
    let bare = ComponentCx::<()>::compile_bare(
        "bare",
        |_cx| crate::rsx!(<UiButton key="button">Bare</UiButton>),
    )
    .unwrap();
    let with_defaults = ComponentCx::<()>::compile(
        "defaults",
        |_cx| crate::rsx!(<UiButton key="button">Default</UiButton>),
    )
    .unwrap();

    assert!(bare.component_registry().is_empty());
    assert!(with_defaults.component_registry().contains("UiButton"));
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ComponentHookState {
    count: u32,
    effects: u32,
    theme: String,
}

#[test]
fn rsx_view_template_uses_rust_component_hooks() {
    let component = RsxComponent::new(
        "hook-demo",
        r#"
        fn CounterView(props: CounterViewProps) -> RSX {
          (
            <Toolbar key="root" orientation="vertical" data-theme={context.theme}>
              <Text key="summary" label={derived.summary} />
              <Button key="increment" onPress={props.onIncrement}>
                Increment
              </Button>
            </Toolbar>
          )
        }
        "#,
    )
    .unwrap()
    .use_prop("onIncrement", |_state: &ComponentHookState| "increment")
    .use_state("count", |state: &ComponentHookState| state.count)
    .use_memo("summary", |state: &ComponentHookState| {
        format!("Count {} Effects {}", state.count, state.effects)
    })
    .use_context("theme", |state: &ComponentHookState| {
        if state.theme.is_empty() {
            "system".to_string()
        } else {
            state.theme.clone()
        }
    })
    .use_reducer(
        "increment",
        |state: &mut ComponentHookState, _invocation| {
            state.count += 1;
            Ok(())
        },
    )
    .use_action_effect("increment", |state: &mut ComponentHookState, invocation| {
        if invocation.action == "increment" {
            state.effects += 1;
        }
        Ok(())
    });

    let mut state = ComponentHookState::default();
    let frame = component.render(&state).unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Count 0 Effects 0", ""]);
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(
        props.attributes.get("data-theme").map(String::as_str),
        Some("system")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    assert_eq!(state.count, 1);
    assert_eq!(state.effects, 1);
    assert_eq!(child_labels(&frame.root), vec!["Count 1 Effects 1", ""]);
}

#[test]
fn component_cx_compiles_counter_function_with_logic_data_view_split() {
    #[deny(unused_variables)]
    fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
        let count = cx.use_state("count", |state: &CounterState| state.count);
        let increment = cx.use_reducer("increment", |state: &mut CounterState, _invocation| {
            state.count += 1;
            Ok(())
        });

        crate::rsx!(<Button key="increment" onPress={increment}>Count {count}</Button>)
    }

    let component = ComponentCx::compile("counter", counter).unwrap();
    let mut state = CounterState::default();
    let frame = component.render(&state).unwrap();

    assert_eq!(text_values(&frame.root), vec!["Count ", "0"]);
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("increment")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    assert_eq!(state.count, 1);
    assert_eq!(text_values(&frame.root), vec!["Count ", "1"]);
}

#[test]
fn component_cx_react_identity_and_debug_hooks_have_runtime_state() {
    let mut generated_ids = Vec::new();
    let mut exposed_handle = None;

    let component = ComponentCx::compile("identity hooks", |cx: &mut ComponentCx<CounterState>| {
        generated_ids.push(cx.use_id());
        generated_ids.push(cx.use_id());
        let handle = cx.use_ref(None::<String>);
        exposed_handle = Some(handle.clone());
        cx.use_imperative_handle(handle, |state: &mut CounterState| {
            Ok(format!("count:{}", state.count))
        });
        cx.use_debug_value("count", |state: &CounterState| state.count);
        let increment = cx.use_callback("increment", |state: &mut CounterState, _invocation| {
            state.count += 1;
            Ok(())
        });

        crate::rsx!(<Button key="increment" onPress={increment}>Increment</Button>)
    })
    .unwrap();

    assert_eq!(
        generated_ids,
        vec![
            "rsx-identity-hooks-0".to_string(),
            "rsx-identity-hooks-1".to_string()
        ]
    );
    assert_eq!(
        component.debug_values(&CounterState::default()).unwrap(),
        vec![RsxDebugValue {
            label: "count".to_string(),
            value: serde_json::json!(0)
        }]
    );

    let handle = exposed_handle.expect("imperative ref handle");
    assert_eq!(handle.current().unwrap(), None);

    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = component.into_runtime_app(host, CounterState::default());
    let rendered = app.render().unwrap();
    assert_eq!(handle.current().unwrap(), Some("count:0".to_string()));

    app.dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();
    assert_eq!(app.state().count, 1);
    assert_eq!(handle.current().unwrap(), Some("count:1".to_string()));

    app.cleanup_effects().unwrap();
    assert_eq!(handle.current().unwrap(), None);
}

#[test]
fn component_cx_use_sync_external_store_reads_snapshots_and_notifies_subscribers() {
    let store = SyncExternalStore::new("offline".to_string());
    let notifications = std::sync::Arc::new(std::sync::Mutex::new(0_u32));
    let subscriber_notifications = std::sync::Arc::clone(&notifications);
    let _subscription = store
        .subscribe(move || {
            *subscriber_notifications.lock().unwrap() += 1;
            Ok(())
        })
        .unwrap();

    let component = ComponentCx::compile("external-store", |cx: &mut ComponentCx<CounterState>| {
        let status = cx.use_sync_external_store("status", store.clone());

        crate::rsx!(<Text key="status">{status}</Text>)
    })
    .unwrap();

    let frame = component.render(&CounterState::default()).unwrap();
    assert_eq!(text_values(&frame.root), vec!["offline"]);

    store.set("online".to_string()).unwrap();
    assert_eq!(*notifications.lock().unwrap(), 1);
    assert_eq!(store.version().unwrap(), 1);

    let frame = component.render(&CounterState::default()).unwrap();
    assert_eq!(text_values(&frame.root), vec!["online"]);
}

#[test]
fn component_cx_use_optimistic_exposes_overlay_until_cleared() {
    let mut optimistic_handle = None;
    let component = ComponentCx::compile("optimistic", |cx: &mut ComponentCx<CounterState>| {
        let count = cx.use_optimistic("count", |state: &CounterState| state.count);
        optimistic_handle = Some(count.clone());

        crate::rsx!(<Text key="count">{count}</Text>)
    })
    .unwrap();
    let optimistic = optimistic_handle.expect("optimistic handle");
    let mut state = CounterState { count: 1 };

    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["1"]);

    optimistic.set_optimistic(9).unwrap();
    state.count = 2;
    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["9"]);

    optimistic.clear_optimistic().unwrap();
    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["2"]);
}

#[test]
fn component_cx_use_action_state_and_form_status_track_action_results() {
    #[derive(Debug, Clone, PartialEq, Default)]
    struct SubmitState {
        count: u32,
        fail: bool,
    }

    let component = ComponentCx::compile("action-state", |cx: &mut ComponentCx<SubmitState>| {
        let submit = cx.use_action_state(
            "submission",
            "submit",
            |state: &mut SubmitState, _invocation| {
                if state.fail {
                    return Err(GuiError::invalid_tree("submit failed"));
                }
                state.count += 1;
                Ok(format!("saved:{}", state.count))
            },
        );
        cx.use_form_status("formStatus", submit.clone());

        crate::rsx!(
            <Toolbar key="root" orientation="vertical">
              <Button key="submit" onPress={submit} label="Submit" />
              <Text key="data" label={derived.submission.data} />
              <Text key="error" label={derived.submission.error} />
              <Text key="pending" label={derived.formStatus.pending} />
              <Text key="action" label={derived.formStatus.action} />
              <Text key="status-data" label={derived.formStatus.data} />
            </Toolbar>
        )
    })
    .unwrap();
    let mut state = SubmitState::default();
    let frame = component.render(&state).unwrap();
    assert_eq!(direct_child_label(&frame.root, "submit"), Some("Submit"));
    assert_eq!(direct_child_label(&frame.root, "data"), Some(""));
    assert_eq!(direct_child_label(&frame.root, "error"), Some(""));
    assert_eq!(direct_child_label(&frame.root, "pending"), Some("false"));
    assert_eq!(direct_child_label(&frame.root, "action"), Some("submit"));
    assert_eq!(direct_child_label(&frame.root, "status-data"), Some(""));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "submit".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    let frame = component.render(&state).unwrap();
    assert_eq!(direct_child_label(&frame.root, "data"), Some("saved:1"));
    assert_eq!(direct_child_label(&frame.root, "error"), Some(""));
    assert_eq!(direct_child_label(&frame.root, "pending"), Some("false"));
    assert_eq!(direct_child_label(&frame.root, "action"), Some("submit"));
    assert_eq!(
        direct_child_label(&frame.root, "status-data"),
        Some("saved:1")
    );

    state.fail = true;
    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "submit".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    let frame = component.render(&state).unwrap();
    assert_eq!(direct_child_label(&frame.root, "data"), Some("saved:1"));
    assert_eq!(
        direct_child_label(&frame.root, "error"),
        Some("invalid native UI tree: submit failed")
    );
    assert_eq!(direct_child_label(&frame.root, "pending"), Some("false"));
    assert_eq!(direct_child_label(&frame.root, "action"), Some("submit"));
    assert_eq!(
        direct_child_label(&frame.root, "status-data"),
        Some("saved:1")
    );
}

#[test]
fn component_cx_accepts_requested_counter_shape_without_explicit_key() {
    #[allow(non_snake_case)]
    #[deny(unused_variables)]
    fn Counter(cx: &mut ComponentCx<CounterState>) -> RSX {
        let count = cx.use_state("count", |s: &CounterState| s.count);
        let increment = cx.use_reducer("increment", |s: &mut CounterState, _| {
            s.count += 1;
            Ok(())
        });

        crate::rsx!(<Button onPress={increment}>Count {count}</Button>)
    }

    let component = ComponentCx::compile("counter", Counter).unwrap();
    let mut state = CounterState::default();
    let frame = component.render(&state).unwrap();

    let CompiledRsxNode::Element { key, props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(key, "root");
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("increment")
    );
    assert_eq!(text_values(&frame.root), vec!["Count ", "0"]);

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert_eq!(state.count, 1);
}

#[test]
fn component_cx_view_can_consume_explicit_state_scope() {
    #[deny(unused_variables)]
    fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
        cx.use_state("count", |state: &CounterState| state.count);
        let increment = cx.use_reducer("increment", |state: &mut CounterState, _invocation| {
            state.count += 1;
            Ok(())
        });

        crate::rsx!(<Button onPress={increment}>Count {state.count}</Button>)
    }

    let component = ComponentCx::compile("counter", counter).unwrap();
    let mut state = CounterState::default();
    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["Count ", "0"]);

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["Count ", "1"]);
}

#[test]
fn component_cx_use_selector_aliases_state_selectors() {
    #[deny(unused_variables)]
    fn counter(cx: &mut ComponentCx<CounterState>) -> RSX {
        let count = cx.use_selector("count", |state: &CounterState| state.count);

        crate::rsx!(<Text key="count">{count}</Text>)
    }

    let component = ComponentCx::compile("counter", counter).unwrap();
    let frame = component.render(&CounterState { count: 7 }).unwrap();

    assert_eq!(text_values(&frame.root), vec!["7"]);
}

#[test]
fn rsx_component_use_selector_aliases_state_selectors() {
    let component = RsxComponent::new("counter", r#"<Text key="count">{state.count}</Text>"#)
        .unwrap()
        .use_selector("count", |state: &CounterState| state.count);

    let frame = component.render(&CounterState { count: 9 }).unwrap();

    assert_eq!(text_values(&frame.root), vec!["9"]);
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ReactiveProfile {
    name: String,
    age: u32,
    settings: ReactiveSettings,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ReactiveSettings {
    theme: String,
    enabled: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct ReactiveHookState {
    profile: ReactiveProfile,
}

#[test]
fn component_cx_use_reactive_exposes_nested_state_object() {
    fn profile_view(cx: &mut ComponentCx<ReactiveHookState>) -> RSX {
        let profile = cx.use_reactive("profile", |state: &ReactiveHookState| state.profile.clone());
        let rename =
            cx.use_value_reducer("rename", |state: &mut ReactiveHookState, name: String| {
                state.profile.name = name;
                Ok(())
            });
        let enable_dark = cx.use_reducer(
            "enable_dark",
            |state: &mut ReactiveHookState, _invocation| {
                state.profile.settings.theme = "dark".to_string();
                state.profile.settings.enabled = true;
                Ok(())
            },
        );

        assert_eq!(profile.binding_path(), "state.profile");

        crate::rsx!(
            <Toolbar
              key="root"
              data-name={profile.name}
              data-theme={profile.settings.theme}
              data-enabled={profile.settings.enabled}
            >
              <Text key="name" label={profile.name} />
              <Text key="age" label={profile.age} />
              <Text key="theme" label={profile.settings.theme} />
              <Button key="rename" onPress={rename}>Rename</Button>
              <Button key="dark" onPress={enable_dark}>Dark</Button>
            </Toolbar>
        )
    }

    let component = ComponentCx::compile("reactive-profile", profile_view).unwrap();
    let mut state = ReactiveHookState {
        profile: ReactiveProfile {
            name: "Ada".to_string(),
            age: 37,
            settings: ReactiveSettings {
                theme: "light".to_string(),
                enabled: false,
            },
        },
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(direct_child_label(&frame.root, "name"), Some("Ada"));
    assert_eq!(direct_child_label(&frame.root, "age"), Some("37"));
    assert_eq!(direct_child_label(&frame.root, "theme"), Some("light"));
    assert_eq!(
        props.attributes.get("data-name").map(String::as_str),
        Some("Ada")
    );
    assert_eq!(
        props.attributes.get("data-theme").map(String::as_str),
        Some("light")
    );
    assert_eq!(
        props.attributes.get("data-enabled").map(String::as_str),
        Some("false")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "rename".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("Grace".to_string()),
            },
        )
        .unwrap();
    let frame = component.render(&state).unwrap();
    assert_eq!(direct_child_label(&frame.root, "name"), Some("Grace"));
    assert_eq!(direct_child_label(&frame.root, "age"), Some("37"));
    assert_eq!(direct_child_label(&frame.root, "theme"), Some("light"));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "enable_dark".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(direct_child_label(&frame.root, "name"), Some("Grace"));
    assert_eq!(direct_child_label(&frame.root, "age"), Some("37"));
    assert_eq!(direct_child_label(&frame.root, "theme"), Some("dark"));
    assert_eq!(
        props.attributes.get("data-theme").map(String::as_str),
        Some("dark")
    );
    assert_eq!(
        props.attributes.get("data-enabled").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_handles_cover_derived_context_and_resource_data() {
    fn status(cx: &mut ComponentCx<ComponentHookState>) -> RSX {
        let summary = cx.use_derived("summary", |state: &ComponentHookState| {
            format!("Count {}", state.count)
        });
        let theme = cx.use_context("theme", |state: &ComponentHookState| {
            if state.theme.is_empty() {
                "system".to_string()
            } else {
                state.theme.clone()
            }
        });
        let profile = cx.use_resource("profile", |state: &ComponentHookState| {
            RsxResource::ready(format!("Profile {}", state.count))
        });

        crate::rsx!(<Toolbar key="root" data-theme={theme}><Text key="summary" label={summary} /><Text key="profile" label={profile.data} /></Toolbar>)
    }

    let component = ComponentCx::compile("status", status).unwrap();
    let frame = component
        .render(&ComponentHookState {
            count: 7,
            effects: 0,
            theme: "dark".to_string(),
        })
        .unwrap();

    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    assert_eq!(
        props.attributes.get("data-theme").map(String::as_str),
        Some("dark")
    );
    assert_eq!(children.len(), 2);
    assert_eq!(child_labels(&frame.root), vec!["Count 7", "Profile 7"]);
}

#[derive(Debug, Clone, PartialEq, Default)]
struct LifecycleHookState {
    title: String,
    disabled: bool,
    mounts: u32,
    clicks: u32,
    effects: u32,
}

#[test]
fn component_cx_owns_lifecycle_effect_and_action_state_hooks() {
    fn lifecycle(cx: &mut ComponentCx<LifecycleHookState>) -> RSX {
        let title = cx.use_state("title", |state: &LifecycleHookState| state.title.clone());
        let click = cx.use_reducer("click", |state: &mut LifecycleHookState, _invocation| {
            state.clicks += 1;
            state.title = format!("Clicked {}", state.clicks);
            Ok(())
        });
        cx.use_mount(|state: &mut LifecycleHookState| {
            state.mounts += 1;
            state.title = "Mounted".to_string();
        });
        cx.use_action_effect("click", |state: &mut LifecycleHookState, invocation| {
            if invocation.action == "click" {
                state.effects += 1;
            }
            Ok(())
        });
        cx.use_action_disabled("click", |state: &LifecycleHookState| state.disabled);

        crate::rsx!(<Button onPress={click}>{title}</Button>)
    }

    let component = ComponentCx::compile("lifecycle", lifecycle).unwrap();
    let mut state = LifecycleHookState::default();
    component.mount(&mut state).unwrap();
    let frame = component.render(&state).unwrap();

    assert_eq!(state.mounts, 1);
    assert_eq!(text_values(&frame.root), vec!["Mounted"]);
    assert_eq!(
        frame
            .actions
            .iter()
            .find(|action| action.id == "click")
            .map(|action| action.disabled),
        Some(false)
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "click".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert_eq!(state.clicks, 1);
    assert_eq!(state.effects, 1);

    state.disabled = true;
    let frame = component.render(&state).unwrap();
    assert_eq!(
        frame
            .actions
            .iter()
            .find(|action| action.id == "click")
            .map(|action| action.disabled),
        Some(true)
    );
}

#[test]
fn component_cx_use_effect_runs_after_runtime_commit() {
    fn lifecycle(cx: &mut ComponentCx<LifecycleHookState>) -> RSX {
        let title = cx.use_state("title", |state: &LifecycleHookState| state.title.clone());
        cx.use_effect(|state: &mut LifecycleHookState| {
            state.effects += 1;
            state.title = format!("Effect {}", state.effects);
            Ok(())
        });

        crate::rsx!(<Text key="title" label={title} />)
    }

    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = ComponentCx::compile("lifecycle", lifecycle)
        .unwrap()
        .into_runtime_app(
            host,
            LifecycleHookState {
                title: "Before".to_string(),
                ..LifecycleHookState::default()
            },
        );

    app.render().unwrap();

    assert_eq!(app.state().effects, 1);
    assert_eq!(app.state().title, "Effect 1");
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .and_then(|tree| tree.label),
        Some("Before".to_string())
    );
}

#[derive(Debug, Clone, PartialEq, Default)]
struct PressState {
    pressed: bool,
    presses: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct LinkState {
    disabled: bool,
    pressed: bool,
    opens: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FocusState {
    focused: bool,
    focus_visible: bool,
    focus_within: bool,
    focus_changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct GroupHookState {
    label: String,
    disabled: bool,
    invalid: bool,
    read_only: bool,
    hovered: bool,
    focused: bool,
    focus_visible: bool,
    focus_within: bool,
    actions: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct VirtualizerHookState {
    label: String,
    item_count: usize,
    visible_start: usize,
    visible_end: usize,
    scrolling: bool,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct InteractionHookState {
    hovered: bool,
    keyboard_active: bool,
    clipboard_disabled: bool,
    copy_value: String,
    pressed: bool,
    long_pressed: bool,
    dragging: bool,
    drop_target: bool,
    moving: bool,
    x_delta: f64,
    y_delta: f64,
    actions: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FieldState {
    label: String,
    invalid: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FormHookState {
    disabled: bool,
    invalid: bool,
    no_validate: bool,
    submitted: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct BreadcrumbHookState {
    label: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct I18nState {
    locale: String,
    direction: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct OverlayState {
    open: bool,
    disabled: bool,
    changes: u32,
    closes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct MenuState {
    selected: bool,
    disabled: bool,
    open: bool,
    pressed: bool,
    actions: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct CollectionState {
    item_count: usize,
    empty: bool,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct CollectionItemState {
    selected: bool,
    disabled: bool,
    expanded: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct LoadMoreItemState {
    loading: bool,
    disabled: bool,
    actions: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct RadioState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct TabState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct TableHookState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct TextHookState {
    label: String,
    text_value: String,
    level: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct StructureHookState {
    label: String,
    orientation: String,
    disabled: bool,
    selected: bool,
    target: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FeedbackHookState {
    title: String,
    description: String,
    label: String,
    close_action: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DateTimeHookState {
    date: String,
    time: String,
    range_start: String,
    range_end: String,
    open: bool,
    invalid: bool,
    selected: bool,
    unavailable: bool,
    pressed: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ColorHookState {
    color: String,
    hue: f64,
    saturation: f64,
    brightness: f64,
    invalid: bool,
    selected: bool,
    disabled: bool,
    pressed: bool,
    dragging: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct SelectionState {
    selected: String,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct SelectionInputState {
    selected: String,
    input: String,
    open: bool,
    invalid: bool,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DisclosureState {
    expanded: bool,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct DisclosureGroupState {
    expanded_keys: String,
    allows_multiple_expanded: bool,
    disabled: bool,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct RangeState {
    value: f64,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct NumberFieldState {
    value: f64,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct SliderPartState {
    label: String,
    value: String,
    value_number: f64,
    orientation: String,
    disabled: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ToggleState {
    selected: bool,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct TextFieldState {
    value: String,
    changes: u32,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FileHookState {
    disabled: bool,
    pressed: bool,
    selected: String,
    drops: u32,
}

#[test]
fn component_cx_press_hook_returns_props_consumed_by_rsx_view() {
    fn pressable(cx: &mut ComponentCx<PressState>) -> RSX {
        let press_action = cx.use_reducer("press", |state: &mut PressState, _invocation| {
            state.presses += 1;
            state.pressed = true;
            Ok(())
        });
        let action = press_action.clone();
        let props = cx.use_press(move |state: &PressState| {
            crate::semantic_ui::UsePressProps::new()
                .on_press(Some(&action))
                .pressed(state.pressed)
        });
        assert_eq!(props.press_props.binding_path(), "props.pressProps");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");
        crate::rsx!(
            <button key="root" {...props.pressProps} data-active={props.isPressed}>
              Press
            </button>
        )
    }

    let component = ComponentCx::compile("pressable", pressable).unwrap();
    let mut state = PressState::default();
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("press")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("button")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("false")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("false")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "press".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(state.presses, 1);
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_link_hook_returns_props_consumed_by_rsx_view() {
    fn link(cx: &mut ComponentCx<LinkState>) -> RSX {
        let open_start = cx.use_reducer("openStart", |_state: &mut LinkState, _invocation| Ok(()));
        let open_end = cx.use_reducer("openEnd", |_state: &mut LinkState, _invocation| Ok(()));
        let open = cx.use_reducer("open", |state: &mut LinkState, _invocation| {
            state.opens += 1;
            state.pressed = true;
            state.disabled = false;
            Ok(())
        });
        let start_action = open_start.clone();
        let end_action = open_end.clone();
        let action = open.clone();
        let props = cx.use_link(move |state: &LinkState| {
            crate::semantic_ui::UseLinkProps::new()
                .href(Some("/docs"))
                .on_press(Some(&action))
                .on_press_start(Some(&start_action))
                .on_press_end(Some(&end_action))
                .disabled(state.disabled)
                .pressed(state.pressed)
        });
        assert_eq!(props.link_props.binding_path(), "props.linkProps");
        assert_eq!(props.href.binding_path(), "props.href");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");

        crate::rsx!(
            <a key="root" {...props.linkProps} data-active={props.isPressed}>
              Docs
            </a>
        )
    }

    let component = ComponentCx::compile("link", link).unwrap();
    let mut state = LinkState {
        disabled: true,
        pressed: false,
        opens: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(props.href.as_deref(), Some("/docs"));
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("open")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("openStart")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("openEnd")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("link")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("-1")
    );
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("aria-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("false")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "open".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };
    assert_eq!(state.opens, 1);
    assert!(!props.is_disabled);
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("0")
    );
    assert_eq!(props.attributes.get("aria-disabled"), None);
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_interaction_hooks_return_props_for_view_consumption() {
    fn interactions(cx: &mut ComponentCx<InteractionHookState>) -> RSX {
        let hover_start = cx.use_reducer(
            "hoverStart",
            |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            },
        );
        let key_down = cx.use_reducer(
            "keyDown",
            |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            },
        );
        let key_up = cx.use_reducer("keyUp", |_state: &mut InteractionHookState, _invocation| {
            Ok(())
        });
        let copy = cx.use_reducer("copy", |state: &mut InteractionHookState, _invocation| {
            state.actions += 1;
            Ok(())
        });
        let cut = cx.use_reducer("cut", |_state: &mut InteractionHookState, _invocation| {
            Ok(())
        });
        let paste = cx.use_reducer("paste", |_state: &mut InteractionHookState, _invocation| {
            Ok(())
        });
        let long_press = cx.use_reducer(
            "longPress",
            |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            },
        );
        let move_start = cx.use_reducer(
            "moveStart",
            |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            },
        );
        let move_update = cx
            .use_reducer("move", |_state: &mut InteractionHookState, _invocation| {
                Ok(())
            });
        let move_end = cx.use_reducer(
            "moveEnd",
            |_state: &mut InteractionHookState, _invocation| Ok(()),
        );
        let drag_start = cx.use_reducer(
            "dragStart",
            |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            },
        );
        let drag_end = cx.use_reducer(
            "dragEnd",
            |_state: &mut InteractionHookState, _invocation| Ok(()),
        );
        let drop_action =
            cx.use_reducer("drop", |state: &mut InteractionHookState, _invocation| {
                state.actions += 1;
                Ok(())
            });
        let drop_enter = cx.use_reducer(
            "dropEnter",
            |_state: &mut InteractionHookState, _invocation| Ok(()),
        );

        let hover_action = hover_start.clone();
        let hover = cx.use_hover(move |state: &InteractionHookState| {
            crate::semantic_ui::UseHoverProps::new()
                .on_hover_start(Some(&hover_action))
                .hovered(state.hovered)
        });
        let key_down_action = key_down.clone();
        let key_up_action = key_up.clone();
        let keyboard = cx.use_keyboard_interaction(move |state: &InteractionHookState| {
            crate::semantic_ui::UseKeyboardInteractionProps::new()
                .on_key_down(Some(&key_down_action))
                .on_key_up(Some(&key_up_action))
                .keyboard_active(state.keyboard_active)
                .tab_index(5)
        });
        let copy_action = copy.clone();
        let cut_action = cut.clone();
        let paste_action = paste.clone();
        let clipboard = cx.use_clipboard(move |state: &InteractionHookState| {
            crate::semantic_ui::UseClipboardProps::new()
                .on_copy(Some(&copy_action))
                .on_cut(Some(&cut_action))
                .on_paste(Some(&paste_action))
                .copy_value(Some(state.copy_value.clone()))
                .copy_mime_type(Some("text/plain"))
                .accepted_mime_types(Some("text/plain,text/html"))
                .disabled(state.clipboard_disabled)
        });
        let long_press_action = long_press.clone();
        let long_press = cx.use_long_press(move |state: &InteractionHookState| {
            crate::semantic_ui::UseLongPressProps::new()
                .on_long_press(Some(&long_press_action))
                .pressed(state.pressed)
                .long_pressed(state.long_pressed)
        });
        let move_start_action = move_start.clone();
        let move_action = move_update.clone();
        let move_end_action = move_end.clone();
        let movement = cx.use_move(move |state: &InteractionHookState| {
            crate::semantic_ui::UseMoveProps::new()
                .on_move_start(Some(&move_start_action))
                .on_move(Some(&move_action))
                .on_move_end(Some(&move_end_action))
                .moving(state.moving)
                .x_delta(state.x_delta)
                .y_delta(state.y_delta)
        });
        let drag_start_action = drag_start.clone();
        let drag_end_action = drag_end.clone();
        let drag = cx.use_drag(move |state: &InteractionHookState| {
            crate::semantic_ui::UseDragProps::new()
                .on_drag_start(Some(&drag_start_action))
                .on_drag_end(Some(&drag_end_action))
                .drag_type(Some("text/plain"))
                .drag_value(Some("alpha"))
                .dragging(state.dragging)
        });
        let drop_action = drop_action.clone();
        let drop_enter_action = drop_enter.clone();
        let drop = cx.use_drop(move |state: &InteractionHookState| {
            crate::semantic_ui::UseDropProps::new()
                .label(Some("Drop target"))
                .on_drop(Some(&drop_action))
                .on_drop_enter(Some(&drop_enter_action))
                .accepted_drag_types(Some("text/plain"))
                .drop_operation(Some("move"))
                .drop_target(state.drop_target)
        });

        assert_eq!(hover.hover_props.binding_path(), "props.hoverProps");
        assert_eq!(hover.is_hovered.binding_path(), "props.isHovered");
        assert_eq!(
            keyboard.keyboard_interaction_props.binding_path(),
            "props.keyboardInteractionProps"
        );
        assert_eq!(
            keyboard.is_keyboard_active.binding_path(),
            "props.isKeyboardActive"
        );
        assert_eq!(
            clipboard.clipboard_props.binding_path(),
            "props.clipboardProps"
        );
        assert_eq!(
            clipboard.is_clipboard_disabled.binding_path(),
            "props.isClipboardDisabled"
        );
        assert_eq!(
            long_press.long_press_props.binding_path(),
            "props.longPressProps"
        );
        assert_eq!(
            long_press.is_long_pressed.binding_path(),
            "props.isLongPressed"
        );
        assert_eq!(movement.move_props.binding_path(), "props.moveProps");
        assert_eq!(movement.x_delta.binding_path(), "props.xDelta");
        assert_eq!(movement.y_delta.binding_path(), "props.yDelta");
        assert_eq!(drag.drag_props.binding_path(), "props.dragProps");
        assert_eq!(
            drag.drag_button_props.binding_path(),
            "props.dragButtonProps"
        );
        assert_eq!(drag.is_dragging.binding_path(), "props.isDragging");
        assert_eq!(drop.drop_props.binding_path(), "props.dropProps");
        assert_eq!(
            drop.drop_button_props.binding_path(),
            "props.dropButtonProps"
        );
        assert_eq!(drop.is_drop_target.binding_path(), "props.isDropTarget");

        crate::rsx!(
            <Group key="root">
              <Group key="hover" {...props.hoverProps} data-active={props.isHovered} />
              <Group
                key="keyboard"
                {...props.keyboardInteractionProps}
                data-active={props.isKeyboardActive}
              />
              <Group
                key="clipboard"
                {...props.clipboardProps}
                data-active={props.isClipboardDisabled}
              />
              <Group
                key="long-press"
                {...props.longPressProps}
                data-active={props.isLongPressed}
              />
              <Group
                key="move"
                {...props.moveProps}
                data-active={props.isMoving}
                data-x={props.xDelta}
                data-y={props.yDelta}
              />
              <Group
                key="drag"
                {...props.dragProps}
                data-active={props.isDragging}
              />
              <Group
                key="drop"
                {...props.dropProps}
                label={props.label}
                data-active={props.isDropTarget}
              />
            </Group>
        )
    }

    let component = ComponentCx::compile("interactions", interactions).unwrap();
    let mut state = InteractionHookState {
        hovered: true,
        keyboard_active: true,
        clipboard_disabled: false,
        copy_value: "hello".to_string(),
        pressed: true,
        long_pressed: true,
        dragging: true,
        drop_target: true,
        moving: true,
        x_delta: 3.5,
        y_delta: -2.0,
        actions: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let child_props = |key: &str| {
        children
            .iter()
            .find_map(|child| match child {
                CompiledRsxNode::Element {
                    key: child_key,
                    props,
                    ..
                } if child_key == key => Some(props),
                _ => None,
            })
            .unwrap_or_else(|| panic!("missing child {key}"))
    };

    let hover = child_props("hover");
    assert_eq!(
        hover.events.get("onHoverStart").map(String::as_str),
        Some("hoverStart")
    );
    assert_eq!(
        hover.attributes.get("data-hovered").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        hover.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    let keyboard = child_props("keyboard");
    assert_eq!(
        keyboard.events.get("onKeyDown").map(String::as_str),
        Some("keyDown")
    );
    assert_eq!(
        keyboard.events.get("onKeyUp").map(String::as_str),
        Some("keyUp")
    );
    assert_eq!(
        keyboard.attributes.get("tabIndex").map(String::as_str),
        Some("5")
    );
    assert_eq!(
        keyboard
            .attributes
            .get("data-keyboard-active")
            .map(String::as_str),
        Some("true")
    );

    let clipboard = child_props("clipboard");
    assert_eq!(
        clipboard.events.get("onCopy").map(String::as_str),
        Some("copy")
    );
    assert_eq!(
        clipboard.events.get("onCut").map(String::as_str),
        Some("cut")
    );
    assert_eq!(
        clipboard.events.get("onPaste").map(String::as_str),
        Some("paste")
    );
    assert_eq!(
        clipboard.attributes.get("role").map(String::as_str),
        Some("textbox")
    );
    assert_eq!(
        clipboard.attributes.get("tabIndex").map(String::as_str),
        Some("0")
    );
    assert_eq!(
        clipboard
            .attributes
            .get("data-copy-value")
            .map(String::as_str),
        Some("hello")
    );
    assert_eq!(
        clipboard
            .attributes
            .get("data-copy-mime-type")
            .map(String::as_str),
        Some("text/plain")
    );
    assert_eq!(
        clipboard
            .attributes
            .get("data-accepted-mime-types")
            .map(String::as_str),
        Some("text/plain,text/html")
    );
    assert_eq!(
        clipboard
            .attributes
            .get("data-clipboard-disabled")
            .map(String::as_str),
        Some("false")
    );
    assert_eq!(
        clipboard.attributes.get("data-active").map(String::as_str),
        Some("false")
    );

    let long_press = child_props("long-press");
    assert_eq!(
        long_press.events.get("onLongPress").map(String::as_str),
        Some("longPress")
    );
    assert_eq!(
        long_press.attributes.get("role").map(String::as_str),
        Some("button")
    );
    assert_eq!(
        long_press
            .attributes
            .get("data-long-pressed")
            .map(String::as_str),
        Some("true")
    );

    let movement = child_props("move");
    assert_eq!(
        movement.events.get("onMoveStart").map(String::as_str),
        Some("moveStart")
    );
    assert_eq!(
        movement.events.get("onMove").map(String::as_str),
        Some("move")
    );
    assert_eq!(
        movement.events.get("onMoveEnd").map(String::as_str),
        Some("moveEnd")
    );
    assert_eq!(
        movement.attributes.get("data-moving").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        movement.attributes.get("data-x-delta").map(String::as_str),
        Some("3.5")
    );
    assert_eq!(
        movement.attributes.get("data-y-delta").map(String::as_str),
        Some("-2.0")
    );

    let drag = child_props("drag");
    assert_eq!(
        drag.events.get("onDragStart").map(String::as_str),
        Some("dragStart")
    );
    assert_eq!(
        drag.events.get("onDragEnd").map(String::as_str),
        Some("dragEnd")
    );
    assert_eq!(
        drag.attributes.get("draggable").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        drag.attributes.get("data-dragging").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        drag.attributes.get("data-drag-type").map(String::as_str),
        Some("text/plain")
    );
    assert_eq!(
        drag.attributes.get("data-drag-value").map(String::as_str),
        Some("alpha")
    );

    let drop = child_props("drop");
    assert_eq!(drop.label.as_deref(), Some("Drop target"));
    assert_eq!(drop.events.get("onDrop").map(String::as_str), Some("drop"));
    assert_eq!(
        drop.events.get("onDropEnter").map(String::as_str),
        Some("dropEnter")
    );
    assert_eq!(
        drop.attributes
            .get("data-accepted-drag-types")
            .map(String::as_str),
        Some("text/plain")
    );
    assert_eq!(
        drop.attributes
            .get("data-drop-operation")
            .map(String::as_str),
        Some("move")
    );
    assert_eq!(
        drop.attributes.get("data-drop-target").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "keyDown".to_string(),
                event: NativeEventKind::KeyDown,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert_eq!(state.actions, 1);

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "copy".to_string(),
                event: NativeEventKind::Copy,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert_eq!(state.actions, 2);
}

#[test]
fn component_cx_file_hooks_return_file_props_for_view_consumption() {
    fn file_trigger(cx: &mut ComponentCx<FileHookState>) -> RSX {
        let open_action = cx
            .use_reducer("openPicker", |_state: &mut FileHookState, _invocation| {
                Ok(())
            });
        let select_action =
            cx.use_reducer("selectFiles", |state: &mut FileHookState, invocation| {
                state.selected = invocation.value.clone().unwrap_or_default();
                Ok(())
            });
        let open = open_action.clone();
        let action = select_action.clone();
        let props = cx.use_file_trigger(move |state: &FileHookState| {
            crate::semantic_ui::UseFileTriggerProps::new()
                .on_press(Some(&open))
                .on_select(Some(&action))
                .accepted_file_types(Some(".rsx"))
                .allows_multiple(true)
                .disabled(state.disabled)
                .pressed(state.pressed)
        });
        assert_eq!(
            props.file_trigger_props.binding_path(),
            "props.fileTriggerProps"
        );
        assert_eq!(
            props.accepted_file_types.binding_path(),
            "props.acceptedFileTypes"
        );
        assert_eq!(props.allows_multiple.binding_path(), "props.allowsMultiple");

        crate::rsx!(<button key="root" {...props.fileTriggerProps}>Files</button>)
    }

    fn drop_zone(cx: &mut ComponentCx<FileHookState>) -> RSX {
        let drop_action = cx.use_reducer("dropFiles", |state: &mut FileHookState, _invocation| {
            state.drops += 1;
            Ok(())
        });
        let enter_action = cx.use_reducer(
            "enterDrop",
            |_state: &mut FileHookState, _invocation| Ok(()),
        );
        let leave_action = cx.use_reducer(
            "leaveDrop",
            |_state: &mut FileHookState, _invocation| Ok(()),
        );
        let action = drop_action.clone();
        let enter = enter_action.clone();
        let leave = leave_action.clone();
        let props = cx.use_drop_zone(move |state: &FileHookState| {
            crate::semantic_ui::UseDropZoneProps::new()
                .label(Some("Drop files"))
                .on_drop(Some(&action))
                .on_drag_enter(Some(&enter))
                .on_drag_leave(Some(&leave))
                .disabled(state.disabled)
        });
        assert_eq!(props.drop_zone_props.binding_path(), "props.dropZoneProps");
        assert_eq!(props.label.binding_path(), "props.label");

        crate::rsx!(<Group key="root" {...props.dropZoneProps} />)
    }

    let mut state = FileHookState {
        disabled: false,
        pressed: true,
        selected: String::new(),
        drops: 0,
    };
    let frame = ComponentCx::compile("file-trigger", file_trigger)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("file trigger element");
    };
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("openPicker")
    );
    assert_eq!(
        props.events.get("onSelect").map(String::as_str),
        Some("selectFiles")
    );
    assert_eq!(
        props.attributes.get("accept").map(String::as_str),
        Some(".rsx")
    );
    assert_eq!(
        props.attributes.get("multiple").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );

    let component = ComponentCx::compile("drop-zone", drop_zone).unwrap();
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("drop zone element");
    };
    assert_eq!(props.label.as_deref(), Some("Drop files"));
    assert_eq!(
        props.events.get("onDrop").map(String::as_str),
        Some("dropFiles")
    );
    assert_eq!(
        props.events.get("onDragEnter").map(String::as_str),
        Some("enterDrop")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "dropFiles".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert_eq!(state.drops, 1);
}

#[test]
fn component_cx_form_hook_returns_form_props_for_view_consumption() {
    fn form(cx: &mut ComponentCx<FormHookState>) -> RSX {
        let submit = cx.use_reducer("submit", |state: &mut FormHookState, _invocation| {
            state.submitted += 1;
            Ok(())
        });
        let reset = cx.use_reducer("reset", |_state: &mut FormHookState, _invocation| Ok(()));
        let invalid = cx.use_reducer("invalid", |_state: &mut FormHookState, _invocation| Ok(()));
        let submit_action = submit.clone();
        let reset_action = reset.clone();
        let invalid_action = invalid.clone();
        let props = cx.use_form(move |state: &FormHookState| {
            crate::semantic_ui::UseFormProps::new()
                .label(Some("Profile"))
                .on_submit(Some(&submit_action))
                .on_reset(Some(&reset_action))
                .on_invalid(Some(&invalid_action))
                .validation_behavior(Some("aria"))
                .disabled(state.disabled)
                .invalid(state.invalid)
                .no_validate(state.no_validate)
        });
        assert_eq!(props.form_props.binding_path(), "props.formProps");
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(
            props.validation_behavior.binding_path(),
            "props.validationBehavior"
        );
        assert_eq!(props.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(props.no_validate.binding_path(), "props.noValidate");

        crate::rsx!(
            <Form
              key="root"
              {...props.formProps}
              data-active={props.isInvalid}
              data-behavior={props.validationBehavior}
            >
              Profile
            </Form>
        )
    }

    let component = ComponentCx::compile("form", form).unwrap();
    let mut state = FormHookState {
        disabled: false,
        invalid: true,
        no_validate: true,
        submitted: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("form element");
    };
    assert_eq!(props.label.as_deref(), Some("Profile"));
    assert!(props.is_invalid);
    assert_eq!(
        props.events.get("onSubmit").map(String::as_str),
        Some("submit")
    );
    assert_eq!(
        props.events.get("onReset").map(String::as_str),
        Some("reset")
    );
    assert_eq!(
        props.events.get("onInvalid").map(String::as_str),
        Some("invalid")
    );
    assert_eq!(
        props.attributes.get("noValidate").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-validation-behavior")
            .map(String::as_str),
        Some("aria")
    );
    assert_eq!(
        props.attributes.get("data-invalid").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "submit".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert_eq!(state.submitted, 1);
}

#[test]
fn component_cx_breadcrumbs_hook_returns_navigation_props_for_view_consumption() {
    fn breadcrumbs(cx: &mut ComponentCx<BreadcrumbHookState>) -> RSX {
        let props = cx.use_breadcrumbs(|state: &BreadcrumbHookState| {
            crate::semantic_ui::UseBreadcrumbsProps::new().label(Some(state.label.clone()))
        });
        assert_eq!(
            props.breadcrumbs_props.binding_path(),
            "props.breadcrumbsProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");

        crate::rsx!(
            <Navigation key="root" {...props.breadcrumbsProps}>
              <Text key="current" label={props.label} />
            </Navigation>
        )
    }

    let component = ComponentCx::compile("breadcrumbs", breadcrumbs).unwrap();
    let frame = component
        .render(&BreadcrumbHookState {
            label: "Project path".to_string(),
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("breadcrumbs root");
    };
    assert_eq!(props.label.as_deref(), Some("Project path"));
    assert_eq!(
        props.attributes.get("data-breadcrumbs").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_focus_hook_returns_focus_props_for_view_consumption() {
    fn focusable(cx: &mut ComponentCx<FocusState>) -> RSX {
        let focus_action = cx.use_reducer("setFocus", |state: &mut FocusState, invocation| {
            state.focus_changes += 1;
            state.focused = invocation.value.as_deref() == Some("true");
            Ok(())
        });
        let action = focus_action.clone();
        let props = cx.use_focus(move |state: &FocusState| {
            crate::semantic_ui::UseFocusableProps::new()
                .on_focus_change(Some(&action))
                .focused(state.focused)
                .auto_focus(true)
                .tab_index(2)
        });
        assert_eq!(props.focus_props.binding_path(), "props.focusProps");
        assert_eq!(props.is_focused.binding_path(), "props.isFocused");

        crate::rsx!(
            <Group key="root" {...props.focusProps} data-active={props.isFocused}>
              Focusable
            </Group>
        )
    }

    let component = ComponentCx::compile("focusable", focusable).unwrap();
    let mut state = FocusState {
        focused: true,
        focus_visible: false,
        focus_within: false,
        focus_changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(
        props.events.get("onFocusChange").map(String::as_str),
        Some("setFocus")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("2")
    );
    assert_eq!(
        props.attributes.get("autoFocus").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-focused").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setFocus".to_string(),
                event: NativeEventKind::Blur,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();

    assert!(!state.focused);
    assert_eq!(state.focus_changes, 1);
}

#[test]
fn component_cx_focus_within_hook_returns_boundary_props_for_view_consumption() {
    fn focus_within(cx: &mut ComponentCx<FocusState>) -> RSX {
        let action = cx.use_reducer("setFocusWithin", |state: &mut FocusState, invocation| {
            state.focus_changes += 1;
            state.focus_within = invocation.value.as_deref() == Some("true");
            Ok(())
        });
        let change_action = action.clone();
        let props = cx.use_focus_within(move |state: &FocusState| {
            crate::semantic_ui::UseFocusWithinProps::new()
                .on_focus_within(Some(&action))
                .on_blur_within(Some(&action))
                .on_focus_within_change(Some(&change_action))
                .focus_within(state.focus_within)
        });
        assert_eq!(
            props.focus_within_props.binding_path(),
            "props.focusWithinProps"
        );
        assert_eq!(props.is_focus_within.binding_path(), "props.isFocusWithin");

        crate::rsx!(
            <Group
              key="root"
              {...props.focusWithinProps}
              data-within={props.isFocusWithin}
            >
              Focus within
            </Group>
        )
    }

    let component = ComponentCx::compile("focus-within", focus_within).unwrap();
    let state = FocusState {
        focus_within: true,
        ..FocusState::default()
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("focus within element");
    };

    assert_eq!(
        props.events.get("onFocusWithin").map(String::as_str),
        Some("setFocusWithin")
    );
    assert_eq!(
        props.events.get("onBlurWithin").map(String::as_str),
        Some("setFocusWithin")
    );
    assert_eq!(
        props.events.get("onFocusWithinChange").map(String::as_str),
        Some("setFocusWithin")
    );
    assert_eq!(
        props.attributes.get("data-within").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_focus_ring_and_scope_hooks_return_props_for_view_consumption() {
    fn focus_ring(cx: &mut ComponentCx<FocusState>) -> RSX {
        let focus_action = cx.use_reducer("setFocus", |state: &mut FocusState, invocation| {
            state.focus_changes += 1;
            state.focused = invocation.value.as_deref() == Some("true");
            Ok(())
        });
        let action = focus_action.clone();
        let props = cx.use_focus_ring(move |state: &FocusState| {
            crate::semantic_ui::UseFocusRingProps::new()
                .on_focus_change(Some(&action))
                .focused(state.focused)
                .focus_visible(state.focus_visible)
                .within(true)
                .focus_within(state.focus_within)
                .auto_focus(true)
                .tab_index(3)
        });
        assert_eq!(
            props.focus_ring_props.binding_path(),
            "props.focusRingProps"
        );
        assert_eq!(props.is_focused.binding_path(), "props.isFocused");
        assert_eq!(
            props.is_focus_visible.binding_path(),
            "props.isFocusVisible"
        );
        assert_eq!(props.is_focus_within.binding_path(), "props.isFocusWithin");

        crate::rsx!(
            <Group
              key="root"
              {...props.focusRingProps}
              data-visible={props.isFocusVisible}
              data-within={props.isFocusWithin}
            >
              Focus ring
            </Group>
        )
    }

    fn focus_scope(cx: &mut ComponentCx<FocusState>) -> RSX {
        let props = cx.use_focus_scope(|state: &FocusState| {
            crate::semantic_ui::UseFocusScopeProps::new()
                .contain(state.focus_within)
                .restore_focus(state.focused)
                .auto_focus(state.focus_visible)
                .tab_index(4)
        });
        assert_eq!(
            props.focus_scope_props.binding_path(),
            "props.focusScopeProps"
        );
        assert_eq!(props.contain.binding_path(), "props.contain");
        assert_eq!(props.restore_focus.binding_path(), "props.restoreFocus");
        assert_eq!(props.auto_focus.binding_path(), "props.autoFocus");

        crate::rsx!(
            <Group key="root" {...props.focusScopeProps}>
              Focus scope
            </Group>
        )
    }

    let mut state = FocusState {
        focused: true,
        focus_visible: true,
        focus_within: true,
        focus_changes: 0,
    };
    let frame = ComponentCx::compile("focus-ring", focus_ring)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("focus ring element");
    };
    assert_eq!(
        props.events.get("onFocusChange").map(String::as_str),
        Some("setFocus")
    );
    assert_eq!(
        props
            .attributes
            .get("data-focus-visible")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-focus-within")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-visible").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("3")
    );

    let component = ComponentCx::compile("focus-ring-reducer", focus_ring).unwrap();
    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setFocus".to_string(),
                event: NativeEventKind::Blur,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();
    assert!(!state.focused);
    assert_eq!(state.focus_changes, 1);

    let frame = ComponentCx::compile("focus-scope", focus_scope)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("focus scope element");
    };
    assert_eq!(
        props.attributes.get("data-focus-scope").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-contain").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-restore-focus")
            .map(String::as_str),
        Some("false")
    );
    assert_eq!(
        props.attributes.get("data-auto-focus").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("4")
    );
}

#[test]
fn component_cx_group_hook_returns_props_for_view_consumption() {
    fn group(cx: &mut ComponentCx<GroupHookState>) -> RSX {
        let focus_action = cx.use_reducer("setFocus", |state: &mut GroupHookState, invocation| {
            state.actions += 1;
            state.focused = invocation.value.as_deref() == Some("true");
            Ok(())
        });
        let hover_action = cx.use_reducer("startHover", |state: &mut GroupHookState, _| {
            state.actions += 1;
            state.hovered = true;
            Ok(())
        });
        let focus_action = focus_action.clone();
        let hover_action = hover_action.clone();
        let props = cx.use_group(move |state: &GroupHookState| {
            crate::semantic_ui::UseGroupProps::new()
                .label(Some(state.label.clone()))
                .on_focus_change(Some(&focus_action))
                .on_hover_start(Some(&hover_action))
                .disabled(state.disabled)
                .invalid(state.invalid)
                .read_only(state.read_only)
                .hovered(state.hovered)
                .focused(state.focused)
                .focus_visible(state.focus_visible)
                .focus_within(state.focus_within)
                .auto_focus(true)
                .tab_index(6)
        });
        assert_eq!(props.group_props.binding_path(), "props.groupProps");
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(props.is_read_only.binding_path(), "props.isReadOnly");
        assert_eq!(props.is_hovered.binding_path(), "props.isHovered");
        assert_eq!(props.is_focused.binding_path(), "props.isFocused");
        assert_eq!(
            props.is_focus_visible.binding_path(),
            "props.isFocusVisible"
        );
        assert_eq!(props.is_focus_within.binding_path(), "props.isFocusWithin");

        crate::rsx!(
            <Group
                key="root"
                {...props.groupProps}
                data-active={props.isFocusWithin}
            >
              Group
            </Group>
        )
    }

    let mut state = GroupHookState {
        label: "Inspector".to_string(),
        disabled: false,
        invalid: true,
        read_only: true,
        hovered: true,
        focused: true,
        focus_visible: true,
        focus_within: true,
        actions: 0,
    };
    let component = ComponentCx::compile("group-hook", group).unwrap();
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("group element");
    };

    assert_eq!(props.label.as_deref(), Some("Inspector"));
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onFocusChange").map(String::as_str),
        Some("setFocus")
    );
    assert_eq!(
        props.events.get("onHoverStart").map(String::as_str),
        Some("startHover")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("group")
    );
    assert_eq!(
        props.attributes.get("data-hovered").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-focus-visible")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-focus-within")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setFocus".to_string(),
                event: NativeEventKind::Blur,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();

    assert!(!state.focused);
    assert_eq!(state.actions, 1);
}

#[test]
fn component_cx_virtualizer_hook_returns_props_for_view_consumption() {
    fn virtualizer(cx: &mut ComponentCx<VirtualizerHookState>) -> RSX {
        let props = cx.use_virtualizer(|state: &VirtualizerHookState| {
            crate::semantic_ui::UseVirtualizerProps::new()
                .label(Some(state.label.clone()))
                .layout(Some("grid"))
                .orientation(Some("horizontal"))
                .item_count(state.item_count)
                .estimated_item_size(72)
                .visible_start(state.visible_start)
                .visible_end(state.visible_end)
                .overscan(4)
                .gap(8)
                .padding(12)
                .scrolling(state.scrolling)
                .disabled(state.disabled)
                .tab_index(3)
        });
        assert_eq!(
            props.virtualizer_props.binding_path(),
            "props.virtualizerProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.layout.binding_path(), "props.layout");
        assert_eq!(props.orientation.binding_path(), "props.orientation");
        assert_eq!(props.item_count.binding_path(), "props.itemCount");
        assert_eq!(
            props.estimated_item_size.binding_path(),
            "props.estimatedItemSize"
        );
        assert_eq!(props.visible_start.binding_path(), "props.visibleStart");
        assert_eq!(props.visible_end.binding_path(), "props.visibleEnd");
        assert_eq!(props.overscan.binding_path(), "props.overscan");
        assert_eq!(props.gap.binding_path(), "props.gap");
        assert_eq!(props.padding.binding_path(), "props.padding");
        assert_eq!(props.is_scrolling.binding_path(), "props.isScrolling");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Group
                key="root"
                {...props.virtualizerProps}
                data-visible-window={props.visibleEnd}
            >
              Virtual rows
            </Group>
        )
    }

    let component = ComponentCx::compile("virtualizer-hook", virtualizer).unwrap();
    let frame = component
        .render(&VirtualizerHookState {
            label: "Results".to_string(),
            item_count: 100,
            visible_start: 20,
            visible_end: 36,
            scrolling: true,
            disabled: false,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("virtualizer element");
    };

    assert_eq!(props.label.as_deref(), Some("Results"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("grid")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("3")
    );
    assert_eq!(
        props.attributes.get("data-virtualizer").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-layout").map(String::as_str),
        Some("grid")
    );
    assert_eq!(
        props.attributes.get("data-orientation").map(String::as_str),
        Some("horizontal")
    );
    assert_eq!(
        props.attributes.get("data-item-count").map(String::as_str),
        Some("100")
    );
    assert_eq!(
        props
            .attributes
            .get("data-visible-start")
            .map(String::as_str),
        Some("20")
    );
    assert_eq!(
        props.attributes.get("data-visible-end").map(String::as_str),
        Some("36")
    );
    assert_eq!(
        props.attributes.get("data-overscan").map(String::as_str),
        Some("4")
    );
    assert_eq!(
        props.attributes.get("data-gap").map(String::as_str),
        Some("8")
    );
    assert_eq!(
        props.attributes.get("data-padding").map(String::as_str),
        Some("12")
    );
    assert_eq!(
        props.attributes.get("data-scrolling").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-visible-window")
            .map(String::as_str),
        Some("36")
    );
}

#[test]
fn component_cx_field_hook_returns_field_props_for_view_consumption() {
    fn field_set(cx: &mut ComponentCx<FieldState>) -> RSX {
        let props = cx.use_field(|state: &FieldState| {
            crate::semantic_ui::UseFieldProps::new()
                .label(Some(state.label.clone()))
                .disabled(true)
                .required(true)
                .invalid(state.invalid)
                .read_only(true)
        });
        assert_eq!(props.field_props.binding_path(), "props.fieldProps");
        assert_eq!(props.label.binding_path(), "props.label");

        crate::rsx!(
            <FieldSet key="root" {...props.fieldProps} label={props.label}>
              Field content
            </FieldSet>
        )
    }

    fn checkbox_group(cx: &mut ComponentCx<FieldState>) -> RSX {
        let props = cx.use_checkbox_group(|state: &FieldState| {
            crate::semantic_ui::UseCheckboxGroupProps::new()
                .label(Some(state.label.clone()))
                .value(Some("email"))
                .on_change(Some("setChannels"))
                .disabled(true)
                .required(true)
                .invalid(state.invalid)
                .read_only(true)
        });
        assert_eq!(
            props.checkbox_group_props.binding_path(),
            "props.checkboxGroupProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.selected_value.binding_path(), "props.selectedValue");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_required.binding_path(), "props.isRequired");
        assert_eq!(props.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(props.is_read_only.binding_path(), "props.isReadOnly");

        crate::rsx!(
            <FieldSet
              key="root"
              {...props.checkboxGroupProps}
              data-selected={props.selectedValue}
            >
              Checkbox group
            </FieldSet>
        )
    }

    let component = ComponentCx::compile("field-set", field_set).unwrap();
    let frame = component
        .render(&FieldState {
            label: "Preferences".to_string(),
            invalid: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.label.as_deref(), Some("Preferences"));
    assert!(props.is_disabled);
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.attributes.get("aria-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-required").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-invalid").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-readonly").map(String::as_str),
        Some("true")
    );

    let component = ComponentCx::compile("checkbox-group", checkbox_group)
        .unwrap()
        .use_value_reducer("setChannels", |_state: &mut FieldState, _value: String| {
            Ok(())
        });
    let frame = component
        .render(&FieldState {
            label: "Channels".to_string(),
            invalid: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("checkbox group element");
    };

    assert_eq!(props.label.as_deref(), Some("Channels"));
    assert_eq!(props.value.as_deref(), Some("email"));
    assert!(props.is_disabled);
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setChannels")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("group")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("email")
    );
}

#[test]
fn component_cx_i18n_hook_returns_locale_props_for_view_consumption() {
    fn provider(cx: &mut ComponentCx<I18nState>) -> RSX {
        let props = cx.use_i18n(|state: &I18nState| {
            crate::semantic_ui::UseI18nProps::new()
                .locale(Some(state.locale.clone()))
                .direction(Some(state.direction.clone()))
        });
        assert_eq!(props.i18n_props.binding_path(), "props.i18nProps");
        assert_eq!(props.locale.binding_path(), "props.locale");
        assert_eq!(props.direction.binding_path(), "props.direction");
        assert_eq!(props.is_rtl.binding_path(), "props.isRtl");

        crate::rsx!(
            <Group key="root" {...props.i18nProps} data-active-rtl={props.isRtl}>
              Locale
            </Group>
        )
    }

    let component = ComponentCx::compile("i18n-provider", provider).unwrap();
    let frame = component
        .render(&I18nState {
            locale: "ar-EG".to_string(),
            direction: String::new(),
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(
        props.attributes.get("lang").map(String::as_str),
        Some("ar-EG")
    );
    assert_eq!(props.attributes.get("dir").map(String::as_str), Some("rtl"));
    assert_eq!(
        props.attributes.get("data-locale").map(String::as_str),
        Some("ar-EG")
    );
    assert_eq!(
        props.attributes.get("data-rtl").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active-rtl").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_overlay_hook_returns_overlay_props_for_view_consumption() {
    fn overlay(cx: &mut ComponentCx<OverlayState>) -> RSX {
        let toggle_action =
            cx.use_reducer("toggleOverlay", |state: &mut OverlayState, _invocation| {
                state.open = !state.open;
                state.changes += 1;
                Ok(())
            });
        let close_action =
            cx.use_reducer("closeOverlay", |state: &mut OverlayState, _invocation| {
                state.open = false;
                state.closes += 1;
                Ok(())
            });
        let toggle = toggle_action.clone();
        let close = close_action.clone();
        let props = cx.use_overlay(move |state: &OverlayState| {
            crate::semantic_ui::UseOverlayProps::new()
                .open(state.open)
                .on_open_change(Some(&toggle))
                .on_close(Some(&close))
                .disabled(state.disabled)
                .trigger_kind(Some("dialog"))
        });
        assert_eq!(props.overlay_props.binding_path(), "props.overlayProps");
        assert_eq!(
            props.overlay_trigger_props.binding_path(),
            "props.overlayTriggerProps"
        );
        assert_eq!(props.is_open.binding_path(), "props.isOpen");

        crate::rsx!(
            <Group key="root">
              <button
                key="trigger"
                {...props.overlayTriggerProps}
                data-open-state={props.isOpen}
              >
                Open
              </button>
              <dialog
                key="panel"
                {...props.overlayProps}
                data-open-state={props.isOpen}
              >
                Content
              </dialog>
            </Group>
        )
    }

    let component = ComponentCx::compile("overlay", overlay).unwrap();
    let mut state = OverlayState {
        open: true,
        disabled: true,
        changes: 0,
        closes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let trigger_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "trigger" => Some(props),
            _ => None,
        })
        .expect("trigger element");
    let panel_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "panel" => Some(props),
            _ => None,
        })
        .expect("panel element");

    assert_eq!(
        trigger_props.events.get("onPress").map(String::as_str),
        Some("toggleOverlay")
    );
    assert!(trigger_props.is_disabled);
    assert_eq!(
        trigger_props
            .attributes
            .get("aria-disabled")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("aria-expanded")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("aria-haspopup")
            .map(String::as_str),
        Some("dialog")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("data-open")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("data-open-state")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props.events.get("onOpenChange").map(String::as_str),
        Some("toggleOverlay")
    );
    assert_eq!(
        panel_props.events.get("onClose").map(String::as_str),
        Some("closeOverlay")
    );
    assert_eq!(
        panel_props.attributes.get("open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props
            .attributes
            .get("data-overlay")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props
            .attributes
            .get("data-open-state")
            .map(String::as_str),
        Some("true")
    );
    assert!(!panel_props.attributes.contains_key("aria-hidden"));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "toggleOverlay".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert!(!state.open);
    assert_eq!(state.changes, 1);

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(2),
                current_target: None,
                action: "closeOverlay".to_string(),
                event: NativeEventKind::Close,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert!(!state.open);
    assert_eq!(state.closes, 1);
}

#[test]
fn component_cx_menu_hooks_return_menu_props_for_view_consumption() {
    fn menu(cx: &mut ComponentCx<MenuState>) -> RSX {
        let action = cx.use_reducer("activateItem", |state: &mut MenuState, _invocation| {
            state.actions += 1;
            Ok(())
        });
        let menu_props = cx.use_menu(|state: &MenuState| {
            crate::semantic_ui::UseMenuProps::new()
                .label(Some("Actions"))
                .disabled(state.disabled)
        });
        let item_action = action.clone();
        let item_props = cx.use_menu_item(move |state: &MenuState| {
            crate::semantic_ui::UseMenuItemProps::new()
                .text_value(Some("Open file"))
                .action_value(Some("open"))
                .on_action(Some(&item_action))
                .disabled(state.disabled)
                .selected(state.selected)
        });
        assert_eq!(menu_props.menu_props.binding_path(), "props.menuProps");
        assert_eq!(menu_props.label.binding_path(), "props.label");
        assert_eq!(
            item_props.menu_item_props.binding_path(),
            "props.menuItemProps"
        );
        assert_eq!(item_props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(item_props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(item_props.text_value.binding_path(), "props.textValue");

        crate::rsx!(
            <Menu key="root" {...props.menuProps}>
              <MenuItem
                key="open"
                {...props.menuItemProps}
                data-active={props.isSelected}
              >
                Open
              </MenuItem>
            </Menu>
        )
    }

    let component = ComponentCx::compile("menu", menu).unwrap();
    let mut state = MenuState {
        selected: true,
        disabled: true,
        open: false,
        pressed: false,
        actions: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    let item_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "open" => Some(props),
            _ => None,
        })
        .expect("menu item element");

    assert_eq!(props.label.as_deref(), Some("Actions"));
    assert!(props.is_disabled);
    assert_eq!(props.aria_label.as_deref(), Some("Actions"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("menu")
    );
    assert_eq!(item_props.text_value.as_deref(), Some("Open file"));
    assert!(item_props.is_disabled);
    assert!(item_props.is_selected);
    assert_eq!(
        item_props.events.get("onPress").map(String::as_str),
        Some("activateItem")
    );
    assert_eq!(
        item_props.attributes.get("actionValue").map(String::as_str),
        Some("open")
    );
    assert_eq!(
        item_props.attributes.get("role").map(String::as_str),
        Some("menuitem")
    );
    assert_eq!(
        item_props
            .attributes
            .get("data-selected")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        item_props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "activateItem".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert_eq!(state.actions, 1);
}

#[test]
fn component_cx_submenu_trigger_hook_returns_props_for_view_consumption() {
    fn submenu_trigger(cx: &mut ComponentCx<MenuState>) -> RSX {
        let action = cx.use_reducer("toggleSubmenu", |state: &mut MenuState, _invocation| {
            state.open = !state.open;
            state.actions += 1;
            Ok(())
        });
        cx.use_reducer("startSubmenu", |_state: &mut MenuState, _invocation| Ok(()));
        cx.use_reducer("endSubmenu", |_state: &mut MenuState, _invocation| Ok(()));
        let trigger_action = action.clone();
        let props = cx.use_submenu_trigger(move |state: &MenuState| {
            crate::semantic_ui::UseSubmenuTriggerProps::new()
                .on_press(Some(&trigger_action))
                .on_press_start(Some("startSubmenu"))
                .on_press_end(Some("endSubmenu"))
                .action_value(Some("more"))
                .disabled(state.disabled)
                .pressed(state.pressed)
                .open(state.open)
        });
        assert_eq!(
            props.submenu_trigger_props.binding_path(),
            "props.submenuTriggerProps"
        );
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");
        assert_eq!(props.is_open.binding_path(), "props.isOpen");

        crate::rsx!(
            <MenuItem
              key="root"
              {...props.submenuTriggerProps}
              data-active={props.isPressed}
              data-open={props.isOpen}
            >
              More
            </MenuItem>
        )
    }

    let component = ComponentCx::compile("submenu-trigger", submenu_trigger).unwrap();
    let mut state = MenuState {
        selected: false,
        disabled: true,
        open: true,
        pressed: true,
        actions: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("toggleSubmenu")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("startSubmenu")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("endSubmenu")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("menuitem")
    );
    assert_eq!(
        props.attributes.get("aria-haspopup").map(String::as_str),
        Some("menu")
    );
    assert_eq!(
        props.attributes.get("aria-expanded").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("actionValue").map(String::as_str),
        Some("more")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "toggleSubmenu".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert!(!state.open);
    assert_eq!(state.actions, 1);
}

#[test]
fn component_cx_collection_hook_returns_collection_props_for_view_consumption() {
    fn collection(cx: &mut ComponentCx<CollectionState>) -> RSX {
        let props = cx.use_collection(|state: &CollectionState| {
            crate::semantic_ui::UseCollectionProps::new()
                .label(Some("Rows"))
                .item_count(state.item_count)
                .empty(state.empty)
                .disabled(state.disabled)
        });
        assert_eq!(
            props.collection_props.binding_path(),
            "props.collectionProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.item_count.binding_path(), "props.itemCount");
        assert_eq!(props.is_empty.binding_path(), "props.isEmpty");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Group
                key="root"
                {...props.collectionProps}
                data-count={props.itemCount}
            >
              Rows
            </Group>
        )
    }

    let component = ComponentCx::compile("collection", collection).unwrap();
    let frame = component
        .render(&CollectionState {
            item_count: 12,
            empty: false,
            disabled: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("collection element");
    };

    assert_eq!(props.label.as_deref(), Some("Rows"));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("group")
    );
    assert_eq!(
        props.attributes.get("data-collection").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-item-count").map(String::as_str),
        Some("12")
    );
    assert_eq!(
        props.attributes.get("data-empty").map(String::as_str),
        Some("false")
    );
    assert_eq!(
        props.attributes.get("data-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-count").map(String::as_str),
        Some("12")
    );
}

#[test]
fn component_cx_collection_section_hook_returns_section_props_for_view_consumption() {
    fn section(cx: &mut ComponentCx<CollectionState>) -> RSX {
        let props = cx.use_collection_section(|state: &CollectionState| {
            crate::semantic_ui::UseCollectionSectionProps::new()
                .label(Some("People"))
                .collection_kind(crate::semantic_ui::CollectionSectionKind::ListBox)
                .disabled(state.disabled)
        });
        assert_eq!(
            props.collection_section_props.binding_path(),
            "props.collectionSectionProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.collection_kind.binding_path(), "props.collectionKind");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Section
                key="root"
                {...props.collectionSectionProps}
                data-kind={props.collectionKind}
            >
              People
            </Section>
        )
    }

    let component = ComponentCx::compile("collection-section", section).unwrap();
    let frame = component
        .render(&CollectionState {
            disabled: true,
            ..CollectionState::default()
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("collection section element");
    };

    assert_eq!(props.label.as_deref(), Some("People"));
    assert_eq!(props.aria_label.as_deref(), Some("People"));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("group")
    );
    assert_eq!(
        props
            .attributes
            .get("data-collection-section")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-collection-kind")
            .map(String::as_str),
        Some("list-box")
    );
    assert_eq!(
        props.attributes.get("data-kind").map(String::as_str),
        Some("listBox")
    );
    assert_eq!(
        props.attributes.get("data-disabled").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_collection_item_hook_returns_item_props_for_view_consumption() {
    fn item(cx: &mut ComponentCx<CollectionItemState>) -> RSX {
        let props = cx.use_collection_item(|state: &CollectionItemState| {
            crate::semantic_ui::UseCollectionItemProps::new()
                .value(Some("alpha"))
                .text_value(Some("Alpha"))
                .selected(state.selected)
                .disabled(state.disabled)
                .expanded(Some(state.expanded))
        });
        assert_eq!(
            props.collection_item_props.binding_path(),
            "props.collectionItemProps"
        );
        assert_eq!(props.value.binding_path(), "props.value");
        assert_eq!(props.text_value.binding_path(), "props.textValue");
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_expanded.binding_path(), "props.isExpanded");

        crate::rsx!(
            <ListBoxItem
                key="root"
                {...props.collectionItemProps}
                data-active={props.isSelected}
            >
              Alpha
            </ListBoxItem>
        )
    }

    let component = ComponentCx::compile("collection-item", item).unwrap();
    let frame = component
        .render(&CollectionItemState {
            selected: true,
            disabled: true,
            expanded: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.value.as_deref(), Some("alpha"));
    assert_eq!(props.text_value.as_deref(), Some("Alpha"));
    assert!(props.is_selected);
    assert!(props.is_disabled);
    assert_eq!(props.is_expanded, Some(true));
    assert_eq!(
        props.attributes.get("aria-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-expanded").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_load_more_item_hook_returns_props_for_view_consumption() {
    fn load_more_item(cx: &mut ComponentCx<LoadMoreItemState>) -> RSX {
        let load_more = cx.use_reducer("loadMore", |state: &mut LoadMoreItemState, _invocation| {
            state.actions += 1;
            Ok(())
        });
        let action = load_more.clone();
        let props = cx.use_load_more_item(move |state: &LoadMoreItemState| {
            crate::semantic_ui::UseLoadMoreItemProps::new()
                .label(Some("Load more"))
                .on_press(Some(&action))
                .action_value(Some("next-page"))
                .action_payload(serde_json::json!({ "cursor": "b" }))
                .loading(state.loading)
                .disabled(state.disabled)
        });
        assert_eq!(
            props.load_more_item_props.binding_path(),
            "props.loadMoreItemProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.text_value.binding_path(), "props.textValue");
        assert_eq!(props.action_value.binding_path(), "props.actionValue");
        assert_eq!(props.action_payload.binding_path(), "props.actionPayload");
        assert_eq!(props.is_loading.binding_path(), "props.isLoading");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <ListBoxItem
                key="root"
                {...props.loadMoreItemProps}
                data-active={props.isLoading}
            >
              Load more
            </ListBoxItem>
        )
    }

    let component = ComponentCx::compile("load-more-item", load_more_item).unwrap();
    let mut state = LoadMoreItemState {
        loading: true,
        disabled: false,
        actions: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.label.as_deref(), Some("Load more"));
    assert_eq!(props.text_value.as_deref(), Some("Load more"));
    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("loadMore")
    );
    assert_eq!(
        props.attributes.get("actionValue").map(String::as_str),
        Some("next-page")
    );
    assert_eq!(
        props.attributes.get("actionPayload").map(String::as_str),
        Some(r#"{"cursor":"b"}"#)
    );
    assert_eq!(
        props.attributes.get("aria-busy").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-loading").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "loadMore".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert_eq!(state.actions, 1);
}

#[test]
fn component_cx_radio_hook_returns_radio_props_for_view_consumption() {
    fn radio_group(cx: &mut ComponentCx<RadioState>) -> RSX {
        let group = cx.use_radio_group(|state: &RadioState| {
            crate::semantic_ui::UseRadioGroupProps::new()
                .label(Some("Theme"))
                .value(Some("dark"))
                .on_selection_change(Some("setTheme"))
                .disabled(state.disabled)
                .required(true)
                .invalid(true)
                .read_only(true)
                .selection_mode(Some("single"))
        });
        assert_eq!(
            group.radio_group_props.binding_path(),
            "props.radioGroupProps"
        );
        assert_eq!(group.label.binding_path(), "props.label");
        assert_eq!(group.selected_value.binding_path(), "props.selectedValue");
        assert_eq!(group.selection_mode.binding_path(), "props.selectionMode");
        assert_eq!(group.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(group.is_required.binding_path(), "props.isRequired");
        assert_eq!(group.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(group.is_read_only.binding_path(), "props.isReadOnly");

        crate::rsx!(
            <RadioGroup
                key="root"
                {...props.radioGroupProps}
                data-mode={props.selectionMode}
            >
              <Slot key="content" />
            </RadioGroup>
        )
    }

    fn radio(cx: &mut ComponentCx<RadioState>) -> RSX {
        let props = cx.use_radio(|state: &RadioState| {
            crate::semantic_ui::UseRadioProps::new()
                .value(Some("dark"))
                .text_value(Some("Dark"))
                .selected(state.selected)
                .disabled(state.disabled)
        });
        assert_eq!(props.radio_props.binding_path(), "props.radioProps");
        assert_eq!(props.value.binding_path(), "props.value");
        assert_eq!(props.text_value.binding_path(), "props.textValue");
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_checked.binding_path(), "props.isChecked");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Radio
                key="root"
                {...props.radioProps}
                data-active={props.isChecked}
            >
              Dark
            </Radio>
        )
    }

    let group = ComponentCx::compile("radio-group", radio_group)
        .unwrap()
        .use_value_reducer("setTheme", |_state: &mut RadioState, _theme: String| Ok(()));
    let frame = group
        .render(&RadioState {
            selected: false,
            disabled: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("radio group element");
    };

    assert_eq!(props.label.as_deref(), Some("Theme"));
    assert_eq!(props.value.as_deref(), Some("dark"));
    assert!(props.is_disabled);
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("setTheme")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("radiogroup")
    );
    assert_eq!(
        props.attributes.get("aria-required").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-invalid").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-readonly").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-mode").map(String::as_str),
        Some("single")
    );

    let component = ComponentCx::compile("radio", radio).unwrap();
    let frame = component
        .render(&RadioState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.value.as_deref(), Some("dark"));
    assert_eq!(props.text_value.as_deref(), Some("Dark"));
    assert!(props.is_selected);
    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("radio")
    );
    assert_eq!(
        props.attributes.get("aria-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_tab_hooks_return_tab_props_for_view_consumption() {
    fn tab_list(cx: &mut ComponentCx<TabState>) -> RSX {
        let list = cx.use_tab_list(|state: &TabState| {
            crate::semantic_ui::UseTabListProps::new()
                .label(Some("Settings"))
                .orientation(Some("vertical"))
                .disabled(state.disabled)
        });
        assert_eq!(list.tab_list_props.binding_path(), "props.tabListProps");
        assert_eq!(list.label.binding_path(), "props.label");
        assert_eq!(list.orientation.binding_path(), "props.orientation");
        assert_eq!(list.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <TabList
              key="root"
              {...props.tabListProps}
              data-orientation={props.orientation}
            >
              <Slot key="content" />
            </TabList>
        )
    }

    fn tab_trigger(cx: &mut ComponentCx<TabState>) -> RSX {
        let tab = cx.use_tab(|state: &TabState| {
            crate::semantic_ui::UseTabProps::new()
                .value(Some("profile"))
                .text_value(Some("Profile"))
                .selected(state.selected)
                .disabled(state.disabled)
        });
        assert_eq!(tab.tab_props.binding_path(), "props.tabProps");
        assert_eq!(tab.value.binding_path(), "props.value");
        assert_eq!(tab.text_value.binding_path(), "props.textValue");
        assert_eq!(tab.is_selected.binding_path(), "props.isSelected");
        assert_eq!(tab.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Tab key="root" {...props.tabProps} data-active={props.isSelected}>
              Profile
            </Tab>
        )
    }

    fn tab_panel(cx: &mut ComponentCx<TabState>) -> RSX {
        let panel = cx.use_tab_panel(|_state: &TabState| {
            crate::semantic_ui::UseTabPanelProps::new().value(Some("profile"))
        });
        assert_eq!(panel.tab_panel_props.binding_path(), "props.tabPanelProps");
        assert_eq!(panel.value.binding_path(), "props.value");

        crate::rsx!(
            <TabPanel key="root" {...props.tabPanelProps}>
              Profile panel
            </TabPanel>
        )
    }

    let list = ComponentCx::compile("tab-list", tab_list).unwrap();
    let frame = list
        .render(&TabState {
            selected: false,
            disabled: true,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("tab list element");
    };

    assert_eq!(props.label.as_deref(), Some("Settings"));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("tablist")
    );
    assert_eq!(
        props.attributes.get("aria-orientation").map(String::as_str),
        Some("vertical")
    );
    assert_eq!(
        props.attributes.get("aria-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-orientation").map(String::as_str),
        Some("vertical")
    );

    let trigger = ComponentCx::compile("tab-trigger", tab_trigger).unwrap();
    let frame = trigger
        .render(&TabState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let CompiledRsxNode::Element {
        props: trigger_props,
        ..
    } = &frame.root
    else {
        panic!("tab trigger element");
    };

    assert_eq!(trigger_props.value.as_deref(), Some("profile"));
    assert_eq!(trigger_props.text_value.as_deref(), Some("Profile"));
    assert!(trigger_props.is_selected);
    assert!(trigger_props.is_disabled);
    assert_eq!(
        trigger_props.attributes.get("role").map(String::as_str),
        Some("tab")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("aria-selected")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        trigger_props
            .attributes
            .get("data-active")
            .map(String::as_str),
        Some("true")
    );

    let panel = ComponentCx::compile("tab-panel", tab_panel).unwrap();
    let frame = panel.render(&TabState::default()).unwrap();
    let CompiledRsxNode::Element {
        props: panel_props, ..
    } = &frame.root
    else {
        panic!("tab panel element");
    };

    assert_eq!(panel_props.value.as_deref(), Some("profile"));
    assert_eq!(
        panel_props.attributes.get("role").map(String::as_str),
        Some("tabpanel")
    );
}

#[test]
fn component_cx_table_hooks_return_table_props_for_view_consumption() {
    fn table(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let table = cx.use_table(|_state: &TableHookState| {
            crate::semantic_ui::UseTableProps::new().label(Some("Files"))
        });
        assert_eq!(table.table_props.binding_path(), "props.tableProps");
        assert_eq!(table.label.binding_path(), "props.label");

        crate::rsx!(<Table key="root" {...props.tableProps} />)
    }

    fn section(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let section = cx.use_table_section(|_state: &TableHookState| {
            crate::semantic_ui::UseTableSectionProps::new()
                .kind(crate::semantic_ui::TableSectionKind::Header)
                .label(Some("File columns"))
        });
        assert_eq!(
            section.table_section_props.binding_path(),
            "props.tableSectionProps"
        );
        assert_eq!(section.section_kind.binding_path(), "props.sectionKind");
        assert_eq!(section.label.binding_path(), "props.label");

        crate::rsx!(<TableSection key="root" {...props.tableSectionProps} />)
    }

    fn row(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let row = cx.use_table_row(|state: &TableHookState| {
            crate::semantic_ui::UseTableRowProps::new()
                .selected(state.selected)
                .disabled(state.disabled)
        });
        assert_eq!(row.table_row_props.binding_path(), "props.tableRowProps");
        assert_eq!(row.is_selected.binding_path(), "props.isSelected");
        assert_eq!(row.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <TableRow key="root" {...props.tableRowProps} data-active={props.isSelected} />
        )
    }

    fn cell(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let cell = cx.use_table_cell(|_state: &TableHookState| {
            crate::semantic_ui::UseTableCellProps::new()
                .label(Some("Name"))
                .text_value(Some("Ada"))
        });
        assert_eq!(cell.table_cell_props.binding_path(), "props.tableCellProps");
        assert_eq!(cell.label.binding_path(), "props.label");
        assert_eq!(cell.text_value.binding_path(), "props.textValue");

        crate::rsx!(<TableCell key="root" {...props.tableCellProps} />)
    }

    fn column(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let column = cx.use_table_column(|_state: &TableHookState| {
            crate::semantic_ui::UseTableColumnProps::new()
                .label(Some("Name"))
                .text_value(Some("Name"))
        });
        assert_eq!(
            column.table_column_props.binding_path(),
            "props.tableColumnProps"
        );
        assert_eq!(column.label.binding_path(), "props.label");
        assert_eq!(column.text_value.binding_path(), "props.textValue");

        crate::rsx!(<TableColumn key="root" {...props.tableColumnProps} />)
    }

    fn caption(cx: &mut ComponentCx<TableHookState>) -> RSX {
        let caption = cx.use_table_caption(|_state: &TableHookState| {
            crate::semantic_ui::UseTableCaptionProps::new()
                .label(Some("Files table"))
                .text_value(Some("Files table"))
        });
        assert_eq!(
            caption.table_caption_props.binding_path(),
            "props.tableCaptionProps"
        );
        assert_eq!(caption.label.binding_path(), "props.label");
        assert_eq!(caption.text_value.binding_path(), "props.textValue");

        crate::rsx!(<TableCaption key="root" {...props.tableCaptionProps} />)
    }

    let state = TableHookState {
        selected: true,
        disabled: true,
    };

    let frame = ComponentCx::compile("table", table)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("table element");
    };
    assert_eq!(props.label.as_deref(), Some("Files"));
    assert_eq!(props.aria_label.as_deref(), Some("Files"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("table")
    );

    let frame = ComponentCx::compile("table-section", section)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("section element");
    };
    assert_eq!(props.label.as_deref(), Some("File columns"));
    assert_eq!(props.aria_label.as_deref(), Some("File columns"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("rowgroup")
    );
    assert_eq!(
        props
            .attributes
            .get("data-table-section")
            .map(String::as_str),
        Some("header")
    );

    let frame = ComponentCx::compile("table-row", row)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("row element");
    };
    assert!(props.is_selected);
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("row")
    );
    assert_eq!(
        props.attributes.get("aria-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("table-cell", cell)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("cell element");
    };
    assert_eq!(props.label.as_deref(), Some("Name"));
    assert_eq!(props.text_value.as_deref(), Some("Ada"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("cell")
    );

    let frame = ComponentCx::compile("table-column", column)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("column element");
    };
    assert_eq!(props.label.as_deref(), Some("Name"));
    assert_eq!(props.text_value.as_deref(), Some("Name"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("columnheader")
    );

    let frame = ComponentCx::compile("table-caption", caption)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("caption element");
    };
    assert_eq!(props.label.as_deref(), Some("Files table"));
    assert_eq!(props.text_value.as_deref(), Some("Files table"));
}

#[test]
fn component_cx_text_hooks_return_props_for_view_consumption() {
    fn text(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_text(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(props.text_props.binding_path(), "props.textProps");
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.text_value.binding_path(), "props.textValue");

        crate::rsx!(<Text key="root" {...props.textProps} />)
    }

    fn label(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_label(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(props.label_props.binding_path(), "props.labelProps");
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.text_value.binding_path(), "props.textValue");

        crate::rsx!(<Label key="root" {...props.labelProps} />)
    }

    fn description(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_description(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.description_props.binding_path(),
            "props.descriptionProps"
        );

        crate::rsx!(<Text key="root" {...props.descriptionProps} />)
    }

    fn field_error(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_field_error(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.field_error_props.binding_path(),
            "props.fieldErrorProps"
        );

        crate::rsx!(<Text key="root" {...props.fieldErrorProps} />)
    }

    fn legend(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_legend(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(props.legend_props.binding_path(), "props.legendProps");

        crate::rsx!(<Legend key="root" {...props.legendProps} />)
    }

    fn visually_hidden(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_visually_hidden(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.visually_hidden_props.binding_path(),
            "props.visuallyHiddenProps"
        );

        crate::rsx!(<Text key="root" {...props.visuallyHiddenProps} />)
    }

    fn keyboard(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_keyboard(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(props.keyboard_props.binding_path(), "props.keyboardProps");

        crate::rsx!(<KeyboardInput key="root" {...props.keyboardProps} />)
    }

    fn list_box_header(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_list_box_header(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.list_box_header_props.binding_path(),
            "props.listBoxHeaderProps"
        );

        crate::rsx!(<Header key="root" {...props.listBoxHeaderProps} />)
    }

    fn grid_list_header(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_grid_list_header(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.grid_list_header_props.binding_path(),
            "props.gridListHeaderProps"
        );

        crate::rsx!(<Header key="root" {...props.gridListHeaderProps} />)
    }

    fn tree_header(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_tree_header(|state: &TextHookState| {
            crate::semantic_ui::UseTextProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
        });
        assert_eq!(
            props.tree_header_props.binding_path(),
            "props.treeHeaderProps"
        );

        crate::rsx!(<Header key="root" {...props.treeHeaderProps} />)
    }

    fn heading(cx: &mut ComponentCx<TextHookState>) -> RSX {
        let props = cx.use_heading(|state: &TextHookState| {
            crate::semantic_ui::UseHeadingProps::new()
                .label(Some(state.label.clone()))
                .text_value(Some(state.text_value.clone()))
                .level(state.level)
        });
        assert_eq!(props.heading_props.binding_path(), "props.headingProps");
        assert_eq!(props.level.binding_path(), "props.level");

        crate::rsx!(<Heading key="root" {...props.headingProps} />)
    }

    let state = TextHookState {
        label: "Command".to_string(),
        text_value: "Cmd+K".to_string(),
        level: 9,
    };

    for render in [
        text as fn(&mut ComponentCx<TextHookState>) -> RSX,
        label,
        description,
        field_error,
        legend,
        visually_hidden,
        keyboard,
        list_box_header,
        grid_list_header,
        tree_header,
    ] {
        let frame = ComponentCx::compile("text-hook", render)
            .unwrap()
            .render(&state)
            .unwrap();
        let CompiledRsxNode::Element { props, .. } = &frame.root else {
            panic!("text element");
        };
        assert_eq!(props.label.as_deref(), Some("Command"));
        assert_eq!(props.text_value.as_deref(), Some("Cmd+K"));
    }

    let frame = ComponentCx::compile("heading-hook", heading)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("heading element");
    };
    assert_eq!(props.label.as_deref(), Some("Command"));
    assert_eq!(props.text_value.as_deref(), Some("Cmd+K"));
    assert_eq!(
        props.attributes.get("aria-level").map(String::as_str),
        Some("6")
    );
}

#[test]
fn component_cx_structure_hooks_return_props_for_view_consumption() {
    fn separator(cx: &mut ComponentCx<StructureHookState>) -> RSX {
        let props = cx.use_separator(|state: &StructureHookState| {
            crate::semantic_ui::UseSeparatorProps::new()
                .orientation(Some(state.orientation.clone()))
        });
        assert_eq!(props.separator_props.binding_path(), "props.separatorProps");
        assert_eq!(props.orientation.binding_path(), "props.orientation");

        crate::rsx!(<Separator key="root" {...props.separatorProps} />)
    }

    fn toolbar(cx: &mut ComponentCx<StructureHookState>) -> RSX {
        let props = cx.use_toolbar(|state: &StructureHookState| {
            crate::semantic_ui::UseToolbarProps::new()
                .label(Some(state.label.clone()))
                .orientation(Some(state.orientation.clone()))
                .disabled(state.disabled)
        });
        assert_eq!(props.toolbar_props.binding_path(), "props.toolbarProps");
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.orientation.binding_path(), "props.orientation");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(<Toolbar key="root" {...props.toolbarProps} />)
    }

    fn drop_indicator(cx: &mut ComponentCx<StructureHookState>) -> RSX {
        let props = cx.use_drop_indicator(|state: &StructureHookState| {
            crate::semantic_ui::UseDropIndicatorProps::new()
                .orientation(Some(state.orientation.clone()))
                .target(state.target)
        });
        assert_eq!(
            props.drop_indicator_props.binding_path(),
            "props.dropIndicatorProps"
        );
        assert_eq!(props.is_target.binding_path(), "props.isTarget");

        crate::rsx!(<Group key="root" {...props.dropIndicatorProps} />)
    }

    fn selection_indicator(cx: &mut ComponentCx<StructureHookState>) -> RSX {
        let props = cx.use_selection_indicator(|state: &StructureHookState| {
            crate::semantic_ui::UseSelectionIndicatorProps::new()
                .label(Some(state.label.clone()))
                .selected(state.selected)
        });
        assert_eq!(
            props.selection_indicator_props.binding_path(),
            "props.selectionIndicatorProps"
        );
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");

        crate::rsx!(<Text key="root" {...props.selectionIndicatorProps} />)
    }

    fn landmark(cx: &mut ComponentCx<StructureHookState>) -> RSX {
        let props = cx.use_landmark(|state: &StructureHookState| {
            crate::semantic_ui::UseLandmarkProps::new()
                .kind(Some("navigation"))
                .label(Some(state.label.clone()))
        });
        assert_eq!(props.landmark_props.binding_path(), "props.landmarkProps");
        assert_eq!(props.landmark_kind.binding_path(), "props.landmarkKind");
        assert_eq!(props.label.binding_path(), "props.label");

        crate::rsx!(
            <Navigation
                key="root"
                {...props.landmarkProps}
                data-kind={props.landmarkKind}
            />
        )
    }

    let state = StructureHookState {
        label: "Actions".to_string(),
        orientation: "vertical".to_string(),
        disabled: true,
        selected: true,
        target: true,
    };

    let frame = ComponentCx::compile("separator-hook", separator)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("separator element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(
        props.attributes.get("data-orientation").map(String::as_str),
        Some("vertical")
    );

    let frame = ComponentCx::compile("toolbar-hook", toolbar)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("toolbar element");
    };
    assert_eq!(props.label.as_deref(), Some("Actions"));
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("aria-disabled").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("drop-indicator-hook", drop_indicator)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("drop indicator element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(
        props.attributes.get("data-target").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("selection-indicator-hook", selection_indicator)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("selection indicator element");
    };
    assert_eq!(props.label.as_deref(), Some("Actions"));
    assert!(props.is_selected);
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("landmark-hook", landmark)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("landmark element");
    };
    assert_eq!(props.label.as_deref(), Some("Actions"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("navigation")
    );
    assert_eq!(
        props
            .attributes
            .get("data-landmark-kind")
            .map(String::as_str),
        Some("navigation")
    );
}

#[test]
fn component_cx_feedback_hooks_return_props_for_view_consumption() {
    fn toast(cx: &mut ComponentCx<FeedbackHookState>) -> RSX {
        let props = cx.use_toast(|state: &FeedbackHookState| {
            crate::semantic_ui::UseToastProps::new()
                .title(Some(state.title.clone()))
                .description(Some(state.description.clone()))
                .on_close(Some(state.close_action.clone()))
        });
        assert_eq!(props.toast_props.binding_path(), "props.toastProps");
        assert_eq!(props.title.binding_path(), "props.title");
        assert_eq!(props.description.binding_path(), "props.description");

        crate::rsx!(
            <Group
              key="root"
              {...props.toastProps}
              data-title={props.title}
              data-description-value={props.description}
            />
        )
    }

    fn toast_region(cx: &mut ComponentCx<FeedbackHookState>) -> RSX {
        let props = cx.use_toast_region(|state: &FeedbackHookState| {
            crate::semantic_ui::UseToastRegionProps::new().label(Some(state.label.clone()))
        });
        assert_eq!(
            props.toast_region_props.binding_path(),
            "props.toastRegionProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");

        crate::rsx!(<Group key="root" {...props.toastRegionProps} />)
    }

    let state = FeedbackHookState {
        title: "Saved".to_string(),
        description: "Changes synced".to_string(),
        label: "Notifications".to_string(),
        close_action: "dismissToast".to_string(),
    };

    let frame = ComponentCx::compile("toast-hook", toast)
        .unwrap()
        .use_reducer(
            "dismissToast",
            |_state: &mut FeedbackHookState, _invocation| Ok(()),
        )
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("toast element");
    };
    assert_eq!(props.label.as_deref(), Some("Saved"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("status")
    );
    assert_eq!(
        props.attributes.get("title").map(String::as_str),
        Some("Saved")
    );
    assert_eq!(
        props.attributes.get("data-description").map(String::as_str),
        Some("Changes synced")
    );
    assert_eq!(
        props.events.get("onClose").map(String::as_str),
        Some("dismissToast")
    );

    let frame = ComponentCx::compile("toast-region-hook", toast_region)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("toast region element");
    };
    assert_eq!(props.label.as_deref(), Some("Notifications"));
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("region")
    );
    assert_eq!(
        props
            .attributes
            .get("data-toast-region")
            .map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_date_time_hooks_return_props_for_view_consumption() {
    fn props_by_key<'a>(frame: &'a UiFrame, key: &str) -> &'a CompiledProps {
        fn find<'a>(node: &'a CompiledRsxNode, key: &str) -> Option<&'a CompiledProps> {
            match node {
                CompiledRsxNode::Element {
                    key: node_key,
                    props,
                    children,
                    ..
                } => {
                    if node_key == key {
                        Some(props)
                    } else {
                        children.iter().find_map(|child| find(child, key))
                    }
                }
                CompiledRsxNode::Text { .. } => None,
            }
        }

        find(&frame.root, key).unwrap_or_else(|| panic!("element {key:?}"))
    }

    fn date_field(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_date = cx.use_reducer("setDate", |state: &mut DateTimeHookState, invocation| {
            state.date = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_date.clone();
        let props = cx.use_date_field(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseDateFieldProps::new()
                .label(Some("Due date"))
                .value(Some(state.date.clone()))
                .placeholder(Some("yyyy-mm-dd"))
                .on_change(Some(&action))
                .granularity(Some("day"))
                .required(true)
                .invalid(state.invalid)
        });
        assert_eq!(
            props.date_field_props.binding_path(),
            "props.dateFieldProps"
        );
        assert_eq!(
            props.date_field_input_props.binding_path(),
            "props.dateFieldInputProps"
        );

        crate::rsx!(
            <TextField key="root" {...props.dateFieldProps}>
              <Input key="input" {...props.dateFieldInputProps} />
            </TextField>
        )
    }

    fn time_field(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_time = cx.use_reducer("setTime", |state: &mut DateTimeHookState, invocation| {
            state.time = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_time.clone();
        let props = cx.use_time_field(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseTimeFieldProps::new()
                .label(Some("Start time"))
                .value(Some(state.time.clone()))
                .placeholder(Some("hh:mm"))
                .on_change(Some(&action))
                .granularity(Some("minute"))
                .hour_cycle(Some("24"))
        });
        assert_eq!(
            props.time_field_props.binding_path(),
            "props.timeFieldProps"
        );
        assert_eq!(
            props.time_field_input_props.binding_path(),
            "props.timeFieldInputProps"
        );

        crate::rsx!(
            <TextField key="root" {...props.timeFieldProps}>
              <Input key="input" {...props.timeFieldInputProps} />
            </TextField>
        )
    }

    fn date_input(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let props = cx.use_date_input(|state: &DateTimeHookState| {
            crate::semantic_ui::UseDateInputProps::new()
                .label(Some("Date input"))
                .value(Some(state.date.clone()))
        });
        assert_eq!(
            props.date_input_props.binding_path(),
            "props.dateInputProps"
        );

        crate::rsx!(<Group key="root" {...props.dateInputProps} />)
    }

    fn date_segment(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let props = cx.use_date_segment(|state: &DateTimeHookState| {
            crate::semantic_ui::UseDateSegmentProps::new()
                .segment_type(Some("day"))
                .value(Some("06"))
                .text_value(Some("06"))
                .placeholder(Some("dd"))
                .placeholder_segment(state.date.is_empty())
                .invalid(state.invalid)
        });
        assert_eq!(
            props.date_segment_props.binding_path(),
            "props.dateSegmentProps"
        );

        crate::rsx!(<Text key="root" {...props.dateSegmentProps} />)
    }

    fn calendar(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_date = cx.use_reducer("setDate", |state: &mut DateTimeHookState, invocation| {
            state.date = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_date.clone();
        let props = cx.use_calendar(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseCalendarProps::new()
                .label(Some("July 2026"))
                .value(Some(state.date.clone()))
                .on_change(Some(&action))
                .invalid(state.invalid)
        });
        assert_eq!(props.calendar_props.binding_path(), "props.calendarProps");

        crate::rsx!(<Group key="root" {...props.calendarProps} />)
    }

    fn range_calendar(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_range = cx.use_reducer("setRange", |_state: &mut DateTimeHookState, _| Ok(()));
        let action = set_range.clone();
        let props = cx.use_range_calendar(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseRangeCalendarProps::new()
                .label(Some("Sprint"))
                .start_value(Some(state.range_start.clone()))
                .end_value(Some(state.range_end.clone()))
                .on_change(Some(&action))
        });
        assert_eq!(
            props.range_calendar_props.binding_path(),
            "props.rangeCalendarProps"
        );

        crate::rsx!(<Group key="root" {...props.rangeCalendarProps} />)
    }

    fn calendar_cell(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let select = cx.use_reducer("selectDate", |_state: &mut DateTimeHookState, _| Ok(()));
        let action = select.clone();
        let props = cx.use_calendar_cell(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseCalendarCellProps::new()
                .value(Some(state.date.clone()))
                .text_value(Some("6"))
                .action_value(Some(state.date.clone()))
                .on_press(Some(&action))
                .selected(state.selected)
                .unavailable(state.unavailable)
                .today(true)
                .pressed(state.pressed)
        });
        assert_eq!(
            props.calendar_cell_props.binding_path(),
            "props.calendarCellProps"
        );

        crate::rsx!(<button key="root" {...props.calendarCellProps} />)
    }

    fn date_picker(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_date = cx.use_reducer("setDate", |state: &mut DateTimeHookState, invocation| {
            state.date = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let toggle = cx.use_reducer("toggleDate", |state: &mut DateTimeHookState, _| {
            state.open = !state.open;
            Ok(())
        });
        let set_action = set_date.clone();
        let toggle_action = toggle.clone();
        let props = cx.use_date_picker(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseDatePickerProps::new()
                .label(Some("Ship date"))
                .value(Some(state.date.clone()))
                .placeholder(Some("Select date"))
                .on_change(Some(&set_action))
                .on_open_change(Some(&toggle_action))
                .open(state.open)
                .invalid(state.invalid)
        });
        assert_eq!(
            props.date_picker_props.binding_path(),
            "props.datePickerProps"
        );
        assert_eq!(
            props.date_picker_input_props.binding_path(),
            "props.datePickerInputProps"
        );
        assert_eq!(
            props.date_picker_trigger_props.binding_path(),
            "props.datePickerTriggerProps"
        );

        crate::rsx!(
            <Group key="root" {...props.datePickerProps}>
              <Input key="input" {...props.datePickerInputProps} />
              <button key="trigger" {...props.datePickerTriggerProps} />
            </Group>
        )
    }

    fn date_range_picker(cx: &mut ComponentCx<DateTimeHookState>) -> RSX {
        let set_start = cx.use_reducer("setStart", |state: &mut DateTimeHookState, invocation| {
            state.range_start = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let set_end = cx.use_reducer("setEnd", |state: &mut DateTimeHookState, invocation| {
            state.range_end = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let toggle = cx.use_reducer("toggleRange", |state: &mut DateTimeHookState, _| {
            state.open = !state.open;
            Ok(())
        });
        let start_action = set_start.clone();
        let end_action = set_end.clone();
        let toggle_action = toggle.clone();
        let props = cx.use_date_range_picker(move |state: &DateTimeHookState| {
            crate::semantic_ui::UseDateRangePickerProps::new()
                .label(Some("Sprint dates"))
                .start_value(Some(state.range_start.clone()))
                .end_value(Some(state.range_end.clone()))
                .placeholder(Some("yyyy-mm-dd"))
                .on_start_change(Some(&start_action))
                .on_end_change(Some(&end_action))
                .on_open_change(Some(&toggle_action))
                .open(state.open)
        });
        assert_eq!(
            props.date_range_picker_props.binding_path(),
            "props.dateRangePickerProps"
        );
        assert_eq!(
            props.date_range_picker_start_input_props.binding_path(),
            "props.dateRangePickerStartInputProps"
        );
        assert_eq!(
            props.date_range_picker_end_input_props.binding_path(),
            "props.dateRangePickerEndInputProps"
        );
        assert_eq!(
            props.date_range_picker_trigger_props.binding_path(),
            "props.dateRangePickerTriggerProps"
        );

        crate::rsx!(
            <Group key="root" {...props.dateRangePickerProps}>
              <Input key="start" {...props.dateRangePickerStartInputProps} />
              <Input key="end" {...props.dateRangePickerEndInputProps} />
              <button key="trigger" {...props.dateRangePickerTriggerProps} />
            </Group>
        )
    }

    let state = DateTimeHookState {
        date: "2026-07-06".to_string(),
        time: "09:30".to_string(),
        range_start: "2026-07-01".to_string(),
        range_end: "2026-07-12".to_string(),
        open: true,
        invalid: true,
        selected: true,
        unavailable: true,
        pressed: true,
    };

    let frame = ComponentCx::compile("date-field", date_field)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    let input = props_by_key(&frame, "input");
    assert_eq!(props.label.as_deref(), Some("Due date"));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert_eq!(
        props.attributes.get("data-granularity").map(String::as_str),
        Some("day")
    );
    assert_eq!(input.input_type.as_deref(), Some("date"));
    assert_eq!(input.value.as_deref(), Some("2026-07-06"));
    assert_eq!(input.placeholder.as_deref(), Some("yyyy-mm-dd"));
    assert_eq!(
        input.events.get("onInput").map(String::as_str),
        Some("setDate")
    );

    let frame = ComponentCx::compile("time-field", time_field)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    let input = props_by_key(&frame, "input");
    assert_eq!(
        props.attributes.get("data-hour-cycle").map(String::as_str),
        Some("24")
    );
    assert_eq!(input.input_type.as_deref(), Some("time"));
    assert_eq!(input.value.as_deref(), Some("09:30"));

    let frame = ComponentCx::compile("date-input", date_input)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("2026-07-06")
    );

    let frame = ComponentCx::compile("date-segment", date_segment)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.label.as_deref(), Some("06"));
    assert_eq!(props.text_value.as_deref(), Some("06"));
    assert_eq!(
        props.attributes.get("data-type").map(String::as_str),
        Some("day")
    );
    assert_eq!(
        props.attributes.get("data-invalid").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("calendar", calendar)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.label.as_deref(), Some("July 2026"));
    assert_eq!(props.value.as_deref(), Some("2026-07-06"));
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setDate")
    );

    let frame = ComponentCx::compile("range-calendar", range_calendar)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(
        props.attributes.get("data-start-value").map(String::as_str),
        Some("2026-07-01")
    );
    assert_eq!(
        props.attributes.get("data-end-value").map(String::as_str),
        Some("2026-07-12")
    );
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setRange")
    );

    let frame = ComponentCx::compile("calendar-cell", calendar_cell)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert!(props.is_selected);
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("button")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("-1")
    );
    assert_eq!(
        props.attributes.get("data-unavailable").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("selectDate")
    );

    let frame = ComponentCx::compile("date-picker", date_picker)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    let input = props_by_key(&frame, "input");
    let trigger = props_by_key(&frame, "trigger");
    assert_eq!(
        props.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(input.input_type.as_deref(), Some("date"));
    assert_eq!(
        input.events.get("onInput").map(String::as_str),
        Some("setDate")
    );
    assert_eq!(
        trigger.events.get("onPress").map(String::as_str),
        Some("toggleDate")
    );

    let frame = ComponentCx::compile("date-range-picker", date_range_picker)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    let start = props_by_key(&frame, "start");
    let end = props_by_key(&frame, "end");
    let trigger = props_by_key(&frame, "trigger");
    assert_eq!(
        props.attributes.get("data-start-value").map(String::as_str),
        Some("2026-07-01")
    );
    assert_eq!(
        props.attributes.get("data-end-value").map(String::as_str),
        Some("2026-07-12")
    );
    assert_eq!(start.value.as_deref(), Some("2026-07-01"));
    assert_eq!(end.value.as_deref(), Some("2026-07-12"));
    assert_eq!(
        start.events.get("onInput").map(String::as_str),
        Some("setStart")
    );
    assert_eq!(
        end.events.get("onInput").map(String::as_str),
        Some("setEnd")
    );
    assert_eq!(
        trigger.events.get("onPress").map(String::as_str),
        Some("toggleRange")
    );
}

#[test]
fn component_cx_color_hooks_return_props_for_view_consumption() {
    fn props_by_key<'a>(frame: &'a UiFrame, key: &str) -> &'a CompiledProps {
        fn find<'a>(node: &'a CompiledRsxNode, key: &str) -> Option<&'a CompiledProps> {
            match node {
                CompiledRsxNode::Element {
                    key: node_key,
                    props,
                    children,
                    ..
                } => {
                    if node_key == key {
                        Some(props)
                    } else {
                        children.iter().find_map(|child| find(child, key))
                    }
                }
                CompiledRsxNode::Text { .. } => None,
            }
        }

        find(&frame.root, key).unwrap_or_else(|| panic!("element {key:?}"))
    }

    fn color_field(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_color = cx.use_reducer("setColor", |state: &mut ColorHookState, invocation| {
            state.color = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_color.clone();
        let props = cx.use_color_field(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorFieldProps::new()
                .label(Some("Hex"))
                .value(Some(state.color.clone()))
                .placeholder(Some("#000000"))
                .on_change(Some(&action))
                .color_space(Some("srgb"))
                .required(true)
                .invalid(state.invalid)
        });
        assert_eq!(
            props.color_field_props.binding_path(),
            "props.colorFieldProps"
        );
        assert_eq!(
            props.color_field_input_props.binding_path(),
            "props.colorFieldInputProps"
        );

        crate::rsx!(
            <TextField key="root" {...props.colorFieldProps}>
              <Input key="input" {...props.colorFieldInputProps} />
            </TextField>
        )
    }

    fn color_picker(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_color = cx.use_reducer("setColor", |state: &mut ColorHookState, invocation| {
            state.color = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_color.clone();
        let props = cx.use_color_picker(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorPickerProps::new()
                .label(Some("Accent"))
                .value(Some(state.color.clone()))
                .on_change(Some(&action))
        });
        assert_eq!(
            props.color_picker_props.binding_path(),
            "props.colorPickerProps"
        );

        crate::rsx!(<Group key="root" {...props.colorPickerProps} />)
    }

    fn color_area(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_color = cx.use_reducer("setColor", |state: &mut ColorHookState, invocation| {
            state.color = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_color.clone();
        let props = cx.use_color_area(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorAreaProps::new()
                .label(Some("Saturation and brightness"))
                .value(Some(state.color.clone()))
                .x_channel(Some("saturation"))
                .y_channel(Some("brightness"))
                .x_value(state.saturation)
                .y_value(state.brightness)
                .on_change(Some(&action))
        });
        assert_eq!(
            props.color_area_props.binding_path(),
            "props.colorAreaProps"
        );

        crate::rsx!(<Group key="root" {...props.colorAreaProps} />)
    }

    fn color_slider(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_hue = cx.use_reducer("setHue", |state: &mut ColorHookState, invocation| {
            state.hue = invocation
                .value
                .as_deref()
                .unwrap_or_default()
                .parse()
                .unwrap_or(0.0);
            Ok(())
        });
        let action = set_hue.clone();
        let props = cx.use_color_slider(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorRangeProps::new()
                .label(Some("Hue"))
                .channel(Some("hue"))
                .value_number(state.hue)
                .min_value(0.0)
                .max_value(360.0)
                .step_value(1.0)
                .on_change(Some(&action))
        });
        assert_eq!(
            props.color_slider_props.binding_path(),
            "props.colorSliderProps"
        );

        crate::rsx!(<Slider key="root" {...props.colorSliderProps} />)
    }

    fn color_wheel(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_hue = cx.use_reducer("setHue", |state: &mut ColorHookState, invocation| {
            state.hue = invocation
                .value
                .as_deref()
                .unwrap_or_default()
                .parse()
                .unwrap_or(0.0);
            Ok(())
        });
        let action = set_hue.clone();
        let props = cx.use_color_wheel(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorRangeProps::new()
                .label(Some("Hue wheel"))
                .value_number(state.hue)
                .min_value(0.0)
                .max_value(360.0)
                .step_value(1.0)
                .on_change(Some(&action))
        });
        assert_eq!(
            props.color_wheel_props.binding_path(),
            "props.colorWheelProps"
        );

        crate::rsx!(<Group key="root" {...props.colorWheelProps} />)
    }

    fn color_swatch_picker(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let set_color = cx.use_reducer("setColor", |state: &mut ColorHookState, invocation| {
            state.color = invocation.value.clone().unwrap_or_default();
            Ok(())
        });
        let action = set_color.clone();
        let props = cx.use_color_swatch_picker(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorSwatchPickerProps::new()
                .label(Some("Saved colors"))
                .value(Some(state.color.clone()))
                .on_selection_change(Some(&action))
                .selection_mode(Some("single"))
        });
        assert_eq!(
            props.color_swatch_picker_props.binding_path(),
            "props.colorSwatchPickerProps"
        );

        crate::rsx!(<ListBox key="root" {...props.colorSwatchPickerProps} />)
    }

    fn color_swatch_picker_item(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let props = cx.use_color_swatch_picker_item(|state: &ColorHookState| {
            crate::semantic_ui::UseColorSwatchPickerItemProps::new()
                .value(Some(state.color.clone()))
                .text_value(Some("Preview"))
                .selected(state.selected)
        });
        assert_eq!(
            props.color_swatch_picker_item_props.binding_path(),
            "props.colorSwatchPickerItemProps"
        );

        crate::rsx!(<ListBoxItem key="root" {...props.colorSwatchPickerItemProps} />)
    }

    fn color_swatch(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let props = cx.use_color_swatch(|state: &ColorHookState| {
            crate::semantic_ui::UseColorSwatchProps::new()
                .label(Some("Preview"))
                .value(Some(state.color.clone()))
                .disabled(state.disabled)
        });
        assert_eq!(
            props.color_swatch_props.binding_path(),
            "props.colorSwatchProps"
        );

        crate::rsx!(<Group key="root" {...props.colorSwatchProps} />)
    }

    fn color_thumb(cx: &mut ComponentCx<ColorHookState>) -> RSX {
        let commit = cx.use_reducer("commitColor", |_state: &mut ColorHookState, _| Ok(()));
        let action = commit.clone();
        let props = cx.use_color_thumb(move |state: &ColorHookState| {
            crate::semantic_ui::UseColorThumbProps::new()
                .value(Some(state.color.clone()))
                .x_value(state.saturation)
                .y_value(state.brightness)
                .action_value(Some(state.color.clone()))
                .action_payload(serde_json::Value::String("payload".to_string()))
                .on_press(Some(&action))
                .pressed(state.pressed)
                .dragging(state.dragging)
        });
        assert_eq!(
            props.color_thumb_props.binding_path(),
            "props.colorThumbProps"
        );

        crate::rsx!(<button key="root" {...props.colorThumbProps} />)
    }

    let state = ColorHookState {
        color: "#8145b5".to_string(),
        hue: 271.0,
        saturation: 62.0,
        brightness: 71.0,
        invalid: true,
        selected: true,
        disabled: true,
        pressed: true,
        dragging: true,
    };

    let frame = ComponentCx::compile("color-field", color_field)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    let input = props_by_key(&frame, "input");
    assert_eq!(props.label.as_deref(), Some("Hex"));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert_eq!(
        props.attributes.get("data-color-space").map(String::as_str),
        Some("srgb")
    );
    assert_eq!(input.value.as_deref(), Some("#8145b5"));
    assert_eq!(input.placeholder.as_deref(), Some("#000000"));
    assert_eq!(
        input.events.get("onInput").map(String::as_str),
        Some("setColor")
    );

    let frame = ComponentCx::compile("color-picker", color_picker)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.label.as_deref(), Some("Accent"));
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("#8145b5")
    );
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setColor")
    );

    let frame = ComponentCx::compile("color-area", color_area)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(
        props.attributes.get("data-x-channel").map(String::as_str),
        Some("saturation")
    );
    assert_eq!(
        props.attributes.get("data-y-channel").map(String::as_str),
        Some("brightness")
    );
    assert_eq!(
        props.attributes.get("data-x-value").map(String::as_str),
        Some("62.0")
    );
    assert_eq!(
        props.attributes.get("data-y-value").map(String::as_str),
        Some("71.0")
    );

    let frame = ComponentCx::compile("color-slider", color_slider)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.label.as_deref(), Some("Hue"));
    assert_eq!(props.value_number, Some(271.0));
    assert_eq!(props.min_value, Some(0.0));
    assert_eq!(props.max_value, Some(360.0));
    assert_eq!(props.step_value, Some(1.0));
    assert_eq!(
        props.attributes.get("data-channel").map(String::as_str),
        Some("hue")
    );
    assert_eq!(
        props.attributes.get("aria-valuenow").map(String::as_str),
        Some("271.0")
    );
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setHue")
    );

    let frame = ComponentCx::compile("color-wheel", color_wheel)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(
        props.attributes.get("data-channel").map(String::as_str),
        Some("hue")
    );
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("271.0")
    );

    let frame = ComponentCx::compile("color-swatch-picker", color_swatch_picker)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.value.as_deref(), Some("#8145b5"));
    assert_eq!(
        props
            .attributes
            .get("data-selected-value")
            .map(String::as_str),
        Some("#8145b5")
    );
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("setColor")
    );

    let frame = ComponentCx::compile("color-swatch-picker-item", color_swatch_picker_item)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.value.as_deref(), Some("#8145b5"));
    assert_eq!(props.text_value.as_deref(), Some("Preview"));
    assert!(props.is_selected);
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );

    let frame = ComponentCx::compile("color-swatch", color_swatch)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(props.label.as_deref(), Some("Preview"));
    assert!(props.is_disabled);
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("#8145b5")
    );

    let frame = ComponentCx::compile("color-thumb", color_thumb)
        .unwrap()
        .render(&state)
        .unwrap();
    let props = props_by_key(&frame, "root");
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("button")
    );
    assert_eq!(
        props.attributes.get("tabIndex").map(String::as_str),
        Some("0")
    );
    assert_eq!(
        props.attributes.get("data-x-value").map(String::as_str),
        Some("62.0")
    );
    assert_eq!(
        props.attributes.get("data-y-value").map(String::as_str),
        Some("71.0")
    );
    assert_eq!(
        props.attributes.get("data-dragging").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("actionValue").map(String::as_str),
        Some("#8145b5")
    );
    assert_eq!(
        props.attributes.get("actionPayload").map(String::as_str),
        Some("payload")
    );
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("commitColor")
    );
}

#[test]
fn component_cx_selection_hook_returns_selection_props_for_view_consumption() {
    fn selectable(cx: &mut ComponentCx<SelectionState>) -> RSX {
        let select_action =
            cx.use_reducer("selectItem", |state: &mut SelectionState, invocation| {
                state.selected = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            });
        let action = select_action.clone();
        let props = cx.use_selection(move |state: &SelectionState| {
            crate::semantic_ui::UseSelectionProps::new()
                .value(Some(state.selected.clone()))
                .on_selection_change(Some(&action))
                .selection_mode(Some("multiple"))
        });
        assert_eq!(props.selection_props.binding_path(), "props.selectionProps");
        assert_eq!(props.selected_value.binding_path(), "props.selectedValue");
        assert_eq!(props.selected_keys.binding_path(), "props.selectedKeys");
        assert_eq!(props.selection_mode.binding_path(), "props.selectionMode");
        assert_eq!(
            props.selection_behavior.binding_path(),
            "props.selectionBehavior"
        );
        assert_eq!(
            props.disabled_behavior.binding_path(),
            "props.disabledBehavior"
        );
        assert_eq!(
            props.escape_key_behavior.binding_path(),
            "props.escapeKeyBehavior"
        );

        crate::rsx!(
            <ListBox
                key="root"
                {...props.selectionProps}
                data-selected={props.selectedValue}
                data-mode={props.selectionMode}
            >
              Selectable
            </ListBox>
        )
    }

    let component = ComponentCx::compile("selection", selectable).unwrap();
    let mut state = SelectionState {
        selected: "alpha".to_string(),
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.value.as_deref(), Some("alpha"));
    assert_eq!(
        props.attributes.get("selectedKeys").map(String::as_str),
        Some("[\"alpha\"]")
    );
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("selectItem")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selected-value")
            .map(String::as_str),
        Some("alpha")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selection-mode")
            .map(String::as_str),
        Some("multiple")
    );
    assert_eq!(
        props
            .attributes
            .get("aria-multiselectable")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("alpha")
    );
    assert_eq!(
        props.attributes.get("data-mode").map(String::as_str),
        Some("multiple")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "selectItem".to_string(),
                event: NativeEventKind::SelectionChange,
                context: Default::default(),
                value: Some("beta".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.selected, "beta");
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_selection_input_hooks_return_props_for_view_consumption() {
    fn props_by_key<'a>(frame: &'a UiFrame, key: &str) -> &'a CompiledProps {
        fn find<'a>(node: &'a CompiledRsxNode, key: &str) -> Option<&'a CompiledProps> {
            match node {
                CompiledRsxNode::Element {
                    key: node_key,
                    props,
                    children,
                    ..
                } => {
                    if node_key == key {
                        Some(props)
                    } else {
                        children.iter().find_map(|child| find(child, key))
                    }
                }
                CompiledRsxNode::Text { .. } => None,
            }
        }

        find(&frame.root, key).unwrap_or_else(|| panic!("element {key:?}"))
    }

    fn combo_box(cx: &mut ComponentCx<SelectionInputState>) -> RSX {
        let change_input =
            cx.use_reducer("setInput", |state: &mut SelectionInputState, invocation| {
                state.input = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            });
        let change_selection = cx.use_reducer(
            "setSelection",
            |state: &mut SelectionInputState, invocation| {
                state.selected = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            },
        );
        let toggle_open = cx.use_reducer(
            "toggleOpen",
            |state: &mut SelectionInputState, _invocation| {
                state.open = !state.open;
                Ok(())
            },
        );

        let combo_input_action = change_input.clone();
        let combo_selection_action = change_selection.clone();
        let combo_open_action = toggle_open.clone();
        let combo = cx.use_combo_box(move |state: &SelectionInputState| {
            crate::semantic_ui::UseComboBoxProps::new()
                .label(Some("Assignee"))
                .value(Some(state.selected.clone()))
                .input_value(Some(state.input.clone()))
                .placeholder(Some("Search people"))
                .on_change(Some(&combo_input_action))
                .on_selection_change(Some(&combo_selection_action))
                .on_open_change(Some(&combo_open_action))
                .open(state.open)
                .required(true)
                .invalid(state.invalid)
                .selection_mode(Some("multiple"))
        });
        assert_eq!(combo.combo_box_props.binding_path(), "props.comboBoxProps");
        assert_eq!(
            combo.combo_box_input_props.binding_path(),
            "props.comboBoxInputProps"
        );
        assert_eq!(
            combo.combo_box_trigger_props.binding_path(),
            "props.comboBoxTriggerProps"
        );
        assert_eq!(combo.input_value.binding_path(), "props.inputValue");

        crate::rsx!(
            <ComboBox key="root" {...props.comboBoxProps}>
              <Input key="input" {...props.comboBoxInputProps} />
              <Button key="trigger" {...props.comboBoxTriggerProps} />
            </ComboBox>
        )
    }

    fn autocomplete(cx: &mut ComponentCx<SelectionInputState>) -> RSX {
        let change_input =
            cx.use_reducer("setInput", |state: &mut SelectionInputState, invocation| {
                state.input = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            });
        let change_selection = cx.use_reducer(
            "setSelection",
            |state: &mut SelectionInputState, invocation| {
                state.selected = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            },
        );

        let autocomplete_input_action = change_input.clone();
        let autocomplete_selection_action = change_selection.clone();
        let autocomplete = cx.use_autocomplete(move |state: &SelectionInputState| {
            crate::semantic_ui::UseAutocompleteProps::new()
                .label(Some("Command"))
                .value(Some(state.selected.clone()))
                .input_value(Some(state.input.clone()))
                .placeholder(Some("Run command"))
                .on_change(Some(&autocomplete_input_action))
                .on_selection_change(Some(&autocomplete_selection_action))
                .invalid(state.invalid)
        });
        assert_eq!(
            autocomplete.autocomplete_props.binding_path(),
            "props.autocompleteProps"
        );
        assert_eq!(
            autocomplete.autocomplete_input_props.binding_path(),
            "props.autocompleteInputProps"
        );

        crate::rsx!(
            <ComboBox key="root" {...props.autocompleteProps}>
              <Input key="input" {...props.autocompleteInputProps} />
            </ComboBox>
        )
    }

    fn select(cx: &mut ComponentCx<SelectionInputState>) -> RSX {
        let change_selection = cx.use_reducer(
            "setSelection",
            |state: &mut SelectionInputState, invocation| {
                state.selected = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            },
        );
        let toggle_open = cx.use_reducer(
            "toggleOpen",
            |state: &mut SelectionInputState, _invocation| {
                state.open = !state.open;
                Ok(())
            },
        );

        let select_selection_action = change_selection.clone();
        let select_open_action = toggle_open.clone();
        let select = cx.use_select(move |state: &SelectionInputState| {
            crate::semantic_ui::UseSelectProps::new()
                .label(Some("Density"))
                .value(Some(state.selected.clone()))
                .placeholder(Some("Choose density"))
                .on_selection_change(Some(&select_selection_action))
                .on_open_change(Some(&select_open_action))
                .open(state.open)
                .invalid(state.invalid)
        });
        assert_eq!(select.select_props.binding_path(), "props.selectProps");
        assert_eq!(
            select.select_trigger_props.binding_path(),
            "props.selectTriggerProps"
        );

        crate::rsx!(
            <Select key="root" {...props.selectProps}>
              <Button key="trigger" {...props.selectTriggerProps} />
            </Select>
        )
    }

    fn select_value(cx: &mut ComponentCx<SelectionInputState>) -> RSX {
        let select_display = cx.use_select_display(|state: &SelectionInputState| {
            crate::semantic_ui::UseSelectDisplayProps::new()
                .value(Some(state.selected.clone()))
                .placeholder(Some("Choose density"))
        });
        assert_eq!(
            select_display.select_value_props.binding_path(),
            "props.selectValueProps"
        );

        crate::rsx!(<SelectValue key="root" {...props.selectValueProps} />)
    }

    fn combo_box_value(cx: &mut ComponentCx<SelectionInputState>) -> RSX {
        let combo_display = cx.use_combo_box_display(|state: &SelectionInputState| {
            crate::semantic_ui::UseComboBoxDisplayProps::new()
                .value(Some(state.selected.clone()))
                .placeholder(Some("Search people"))
        });
        assert_eq!(
            combo_display.combo_box_value_props.binding_path(),
            "props.comboBoxValueProps"
        );

        crate::rsx!(<SelectValue key="root" {...props.comboBoxValueProps} />)
    }

    let component = ComponentCx::compile("combo-box", combo_box).unwrap();
    let mut state = SelectionInputState {
        selected: "ada".to_string(),
        input: "ad".to_string(),
        open: true,
        invalid: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();

    let combo = props_by_key(&frame, "root");
    let combo_input = props_by_key(&frame, "input");
    let combo_trigger = props_by_key(&frame, "trigger");
    assert_eq!(combo.label.as_deref(), Some("Assignee"));
    assert_eq!(combo.value.as_deref(), Some("ada"));
    assert!(combo.is_required);
    assert!(combo.is_invalid);
    assert_eq!(
        combo.attributes.get("data-input-value").map(String::as_str),
        Some("ad")
    );
    assert_eq!(
        combo.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        combo
            .attributes
            .get("data-selection-mode")
            .map(String::as_str),
        Some("multiple")
    );
    assert_eq!(combo_input.value.as_deref(), Some("ad"));
    assert_eq!(combo_input.placeholder.as_deref(), Some("Search people"));
    assert_eq!(
        combo_input.events.get("onInput").map(String::as_str),
        Some("setInput")
    );
    assert_eq!(
        combo.events.get("onSelectionChange").map(String::as_str),
        Some("setSelection")
    );
    assert_eq!(
        combo_trigger.events.get("onPress").map(String::as_str),
        Some("toggleOpen")
    );
    assert_eq!(
        combo_trigger.attributes.get("role").map(String::as_str),
        Some("button")
    );

    let frame = ComponentCx::compile("autocomplete", autocomplete)
        .unwrap()
        .render(&state)
        .unwrap();
    let autocomplete = props_by_key(&frame, "root");
    let autocomplete_input = props_by_key(&frame, "input");
    assert_eq!(autocomplete.label.as_deref(), Some("Command"));
    assert_eq!(
        autocomplete
            .attributes
            .get("data-input-value")
            .map(String::as_str),
        Some("ad")
    );
    assert_eq!(
        autocomplete_input.events.get("onInput").map(String::as_str),
        Some("setInput")
    );

    let frame = ComponentCx::compile("select", select)
        .unwrap()
        .render(&state)
        .unwrap();
    let select = props_by_key(&frame, "root");
    let select_trigger = props_by_key(&frame, "trigger");
    assert_eq!(select.label.as_deref(), Some("Density"));
    assert_eq!(
        select.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        select.events.get("onSelectionChange").map(String::as_str),
        Some("setSelection")
    );
    assert_eq!(
        select_trigger.events.get("onPress").map(String::as_str),
        Some("toggleOpen")
    );

    let frame = ComponentCx::compile("select-value", select_value)
        .unwrap()
        .render(&state)
        .unwrap();
    let select_value = props_by_key(&frame, "root");
    assert_eq!(select_value.label.as_deref(), Some("ada"));
    assert_eq!(
        select_value
            .attributes
            .get("data-placeholder")
            .map(String::as_str),
        Some("false")
    );

    let frame = ComponentCx::compile("combo-box-value", combo_box_value)
        .unwrap()
        .render(&state)
        .unwrap();
    let combo_value = props_by_key(&frame, "root");
    assert_eq!(combo_value.label.as_deref(), Some("ada"));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(2),
                current_target: None,
                action: "setInput".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("gr".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.input, "gr");
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_disclosure_hook_returns_disclosure_props_for_view_consumption() {
    fn disclosure(cx: &mut ComponentCx<DisclosureState>) -> RSX {
        let toggle_action = cx.use_reducer("toggle", |state: &mut DisclosureState, _invocation| {
            state.expanded = !state.expanded;
            state.changes += 1;
            Ok(())
        });
        let action = toggle_action.clone();
        let props = cx.use_disclosure(move |state: &DisclosureState| {
            crate::semantic_ui::UseDisclosureProps::new()
                .expanded(state.expanded)
                .on_expanded_change(Some(&action))
        });
        assert_eq!(
            props.disclosure_props.binding_path(),
            "props.disclosureProps"
        );
        assert_eq!(
            props.disclosure_trigger_props.binding_path(),
            "props.disclosureTriggerProps"
        );
        assert_eq!(
            props.disclosure_panel_props.binding_path(),
            "props.disclosurePanelProps"
        );
        assert_eq!(props.is_expanded.binding_path(), "props.isExpanded");

        crate::rsx!(
            <Disclosure key="root" {...props.disclosureProps}>
              <DisclosureSummary
                key="summary"
                {...props.disclosureTriggerProps}
                data-open={props.isExpanded}
              >
                Details
              </DisclosureSummary>
              <Section key="panel" {...props.disclosurePanelProps}>
                Content
              </Section>
            </Disclosure>
        )
    }

    let component = ComponentCx::compile("disclosure", disclosure).unwrap();
    let mut state = DisclosureState {
        expanded: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    let summary_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "summary" => Some(props),
            _ => None,
        })
        .expect("summary element");
    let panel_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "panel" => Some(props),
            _ => None,
        })
        .expect("panel element");

    assert_eq!(props.is_expanded, Some(true));
    assert_eq!(
        props.events.get("onExpandedChange").map(String::as_str),
        Some("toggle")
    );
    assert_eq!(
        props.attributes.get("data-expanded").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        summary_props.events.get("onPress").map(String::as_str),
        Some("toggle")
    );
    assert_eq!(
        summary_props
            .attributes
            .get("aria-expanded")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        summary_props
            .attributes
            .get("data-open")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props
            .attributes
            .get("data-expanded")
            .map(String::as_str),
        Some("true")
    );
    assert!(!panel_props.attributes.contains_key("hidden"));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "toggle".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();

    assert!(!state.expanded);
    assert_eq!(state.changes, 1);

    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let panel_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { key, props, .. } if key == "panel" => Some(props),
            _ => None,
        })
        .expect("panel element");
    assert_eq!(
        panel_props
            .attributes
            .get("data-expanded")
            .map(String::as_str),
        Some("false")
    );
    assert_eq!(
        panel_props
            .attributes
            .get("aria-hidden")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        panel_props.attributes.get("hidden").map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_disclosure_group_hook_returns_props_for_view_consumption() {
    fn disclosure_group(cx: &mut ComponentCx<DisclosureGroupState>) -> RSX {
        let change_action = cx.use_reducer(
            "setExpandedKeys",
            |state: &mut DisclosureGroupState, invocation| {
                state.expanded_keys = invocation.value.clone().unwrap_or_default();
                state.changes += 1;
                Ok(())
            },
        );
        let action = change_action.clone();
        let props = cx.use_disclosure_group(move |state: &DisclosureGroupState| {
            crate::semantic_ui::UseDisclosureGroupProps::new()
                .label(Some("Settings"))
                .expanded_keys(Some(state.expanded_keys.clone()))
                .on_expanded_change(Some(&action))
                .allows_multiple_expanded(state.allows_multiple_expanded)
                .disabled(state.disabled)
        });
        assert_eq!(
            props.disclosure_group_props.binding_path(),
            "props.disclosureGroupProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.expanded_keys.binding_path(), "props.expandedKeys");
        assert_eq!(
            props.allows_multiple_expanded.binding_path(),
            "props.allowsMultipleExpanded"
        );
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(
            <Group
                key="root"
                {...props.disclosureGroupProps}
                data-open-keys={props.expandedKeys}
            >
              <Slot key="content" />
            </Group>
        )
    }

    let component = ComponentCx::compile("disclosure-group", disclosure_group).unwrap();
    let mut state = DisclosureGroupState {
        expanded_keys: "advanced,billing".to_string(),
        allows_multiple_expanded: true,
        disabled: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("disclosure group element");
    };

    assert_eq!(props.label.as_deref(), Some("Settings"));
    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onExpandedChange").map(String::as_str),
        Some("setExpandedKeys")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("group")
    );
    assert_eq!(
        props
            .attributes
            .get("data-disclosure-group")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-expanded-keys")
            .map(String::as_str),
        Some("advanced,billing")
    );
    assert_eq!(
        props
            .attributes
            .get("data-allows-multiple-expanded")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-disabled").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-open-keys").map(String::as_str),
        Some("advanced,billing")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setExpandedKeys".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: Some("advanced".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.expanded_keys, "advanced");
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_range_hook_returns_range_props_for_view_consumption() {
    fn slider(cx: &mut ComponentCx<RangeState>) -> RSX {
        let change_action = cx.use_reducer("setValue", |state: &mut RangeState, invocation| {
            state.value = invocation
                .value
                .as_deref()
                .and_then(|value| value.parse::<f64>().ok())
                .unwrap_or_default();
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_range(move |state: &RangeState| {
            crate::semantic_ui::UseRangeProps::new()
                .value_number(state.value)
                .min_value(0.0)
                .max_value(10.0)
                .step_value(0.5)
                .on_change(Some(&action))
                .required(true)
                .invalid(true)
                .read_only(true)
        });
        assert_eq!(props.range_props.binding_path(), "props.rangeProps");
        assert_eq!(
            props.range_input_props.binding_path(),
            "props.rangeInputProps"
        );
        assert_eq!(props.value_number.binding_path(), "props.valueNumber");
        assert_eq!(props.value_percent.binding_path(), "props.valuePercent");

        crate::rsx!(
            <Slider
              key="root"
              {...props.rangeProps}
              data-active-value={props.valueNumber}
              data-active-percent={props.valuePercent}
            >
              <Input key="input" {...props.rangeInputProps} />
            </Slider>
        )
    }

    let component = ComponentCx::compile("range", slider).unwrap();
    let mut state = RangeState {
        value: 12.0,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    let input_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { props, .. } => Some(props),
            CompiledRsxNode::Text { .. } => None,
        })
        .expect("input element");

    assert_eq!(props.value_number, Some(10.0));
    assert_eq!(props.min_value, Some(0.0));
    assert_eq!(props.max_value, Some(10.0));
    assert_eq!(props.step_value, Some(0.5));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setValue")
    );
    assert_eq!(
        props.events.get("onInput").map(String::as_str),
        Some("setValue")
    );
    assert_eq!(
        props
            .attributes
            .get("data-value-percent")
            .map(String::as_str),
        Some("100.0")
    );
    assert_eq!(
        props
            .attributes
            .get("data-active-value")
            .map(String::as_str),
        Some("10.0")
    );
    assert_eq!(
        props
            .attributes
            .get("data-active-percent")
            .map(String::as_str),
        Some("100.0")
    );
    assert_eq!(input_props.input_type.as_deref(), Some("number"));
    assert_eq!(input_props.value_number, Some(10.0));
    assert_eq!(
        input_props.events.get("onInput").map(String::as_str),
        Some("setValue")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setValue".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("7.5".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.value, 7.5);
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_number_field_hook_returns_props_for_view_consumption() {
    fn number_field(cx: &mut ComponentCx<NumberFieldState>) -> RSX {
        let change_action =
            cx.use_reducer("setQuantity", |state: &mut NumberFieldState, invocation| {
                state.value = invocation
                    .value
                    .as_deref()
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or_default();
                state.changes += 1;
                Ok(())
            });
        let action = change_action.clone();
        let props = cx.use_number_field(move |state: &NumberFieldState| {
            crate::semantic_ui::UseNumberFieldProps::new()
                .label(Some("Quantity"))
                .value_number(state.value)
                .placeholder(Some("0-10"))
                .min_value(0.0)
                .max_value(10.0)
                .step_value(1.0)
                .on_change(Some(&action))
                .required(true)
                .invalid(true)
                .read_only(true)
        });
        assert_eq!(
            props.number_field_props.binding_path(),
            "props.numberFieldProps"
        );
        assert_eq!(
            props.number_field_input_props.binding_path(),
            "props.numberFieldInputProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.placeholder.binding_path(), "props.placeholder");
        assert_eq!(props.value_number.binding_path(), "props.valueNumber");
        assert_eq!(props.value_percent.binding_path(), "props.valuePercent");

        crate::rsx!(
            <TextField
              key="root"
              {...props.numberFieldProps}
              data-active-value={props.valueNumber}
              data-active-percent={props.valuePercent}
            >
              <Input key="input" {...props.numberFieldInputProps} />
            </TextField>
        )
    }

    let component = ComponentCx::compile("number-field", number_field).unwrap();
    let mut state = NumberFieldState {
        value: 12.0,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    let input_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { props, .. } => Some(props),
            CompiledRsxNode::Text { .. } => None,
        })
        .expect("input element");

    assert_eq!(props.label.as_deref(), Some("Quantity"));
    assert_eq!(props.value_number, Some(10.0));
    assert_eq!(props.min_value, Some(0.0));
    assert_eq!(props.max_value, Some(10.0));
    assert_eq!(props.step_value, Some(1.0));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setQuantity")
    );
    assert_eq!(
        props
            .attributes
            .get("data-active-percent")
            .map(String::as_str),
        Some("100.0")
    );
    assert_eq!(input_props.input_type.as_deref(), Some("number"));
    assert_eq!(input_props.placeholder.as_deref(), Some("0-10"));
    assert_eq!(input_props.value_number, Some(10.0));
    assert_eq!(
        input_props.events.get("onInput").map(String::as_str),
        Some("setQuantity")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setQuantity".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("7.5".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.value, 7.5);
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_slider_part_hooks_return_props_for_view_consumption() {
    fn track(cx: &mut ComponentCx<SliderPartState>) -> RSX {
        let props = cx.use_slider_track(|state: &SliderPartState| {
            crate::semantic_ui::UseSliderTrackProps::new()
                .orientation(Some(state.orientation.clone()))
                .disabled(state.disabled)
        });
        assert_eq!(
            props.slider_track_props.binding_path(),
            "props.sliderTrackProps"
        );
        assert_eq!(props.orientation.binding_path(), "props.orientation");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");

        crate::rsx!(<Group key="root" {...props.sliderTrackProps} />)
    }

    fn fill(cx: &mut ComponentCx<SliderPartState>) -> RSX {
        let props = cx.use_slider_fill(|state: &SliderPartState| {
            crate::semantic_ui::UseSliderFillProps::new()
                .orientation(Some(state.orientation.clone()))
                .value_number(state.value_number)
                .disabled(state.disabled)
        });
        assert_eq!(
            props.slider_fill_props.binding_path(),
            "props.sliderFillProps"
        );
        assert_eq!(props.value_number.binding_path(), "props.valueNumber");

        crate::rsx!(<Group key="root" {...props.sliderFillProps} />)
    }

    fn output(cx: &mut ComponentCx<SliderPartState>) -> RSX {
        let props = cx.use_slider_output(|state: &SliderPartState| {
            crate::semantic_ui::UseSliderOutputProps::new()
                .label(Some(state.label.clone()))
                .value(Some(state.value.clone()))
                .value_number(state.value_number)
        });
        assert_eq!(
            props.slider_output_props.binding_path(),
            "props.sliderOutputProps"
        );
        assert_eq!(props.label.binding_path(), "props.label");
        assert_eq!(props.value.binding_path(), "props.value");

        crate::rsx!(<Output key="root" {...props.sliderOutputProps} />)
    }

    let state = SliderPartState {
        label: "Volume".to_string(),
        value: "75%".to_string(),
        value_number: 75.0,
        orientation: "vertical".to_string(),
        disabled: true,
    };

    let frame = ComponentCx::compile("slider-track-hook", track)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("slider track element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert!(props.is_disabled);

    let frame = ComponentCx::compile("slider-fill-hook", fill)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("slider fill element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(props.value_number, Some(75.0));
    assert!(props.is_disabled);

    let frame = ComponentCx::compile("slider-output-hook", output)
        .unwrap()
        .render(&state)
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("slider output element");
    };
    assert_eq!(props.label.as_deref(), Some("Volume"));
    assert_eq!(props.value.as_deref(), Some("75%"));
    assert_eq!(props.value_number, Some(75.0));
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("75%")
    );
}

#[test]
fn component_cx_toggle_hook_returns_toggle_props_for_view_consumption() {
    fn toggle(cx: &mut ComponentCx<ToggleState>) -> RSX {
        let change_action = cx.use_reducer("setChecked", |state: &mut ToggleState, invocation| {
            state.selected = invocation.value.as_deref() == Some("true");
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_toggle(move |state: &ToggleState| {
            crate::semantic_ui::UseToggleProps::new()
                .on_change(Some(&action))
                .selected(state.selected)
                .required(true)
                .invalid(true)
                .read_only(true)
        });
        assert_eq!(props.toggle_props.binding_path(), "props.toggleProps");
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_checked.binding_path(), "props.isChecked");

        crate::rsx!(
            <Checkbox
                key="root"
                {...props.toggleProps}
                data-selected={props.isSelected}
                data-checked-state={props.isChecked}
            >
              Toggle
            </Checkbox>
        )
    }

    let component = ComponentCx::compile("toggle", toggle).unwrap();
    let mut state = ToggleState {
        selected: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setChecked")
    );
    assert_eq!(
        props.attributes.get("data-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("aria-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-checked-state")
            .map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setChecked".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();

    assert!(!state.selected);
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_checkbox_hook_returns_checkbox_props_for_view_consumption() {
    fn checkbox(cx: &mut ComponentCx<ToggleState>) -> RSX {
        let change_action = cx.use_reducer("setAccepted", |state: &mut ToggleState, invocation| {
            state.selected = invocation.value.as_deref() == Some("true");
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_checkbox(move |state: &ToggleState| {
            crate::semantic_ui::UseCheckboxProps::new()
                .value(Some("accepted"))
                .on_change(Some(&action))
                .checked(state.selected)
                .required(true)
                .invalid(true)
                .read_only(true)
        });
        assert_eq!(props.checkbox_props.binding_path(), "props.checkboxProps");
        assert_eq!(props.value.binding_path(), "props.value");
        assert_eq!(props.is_checked.binding_path(), "props.isChecked");
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_required.binding_path(), "props.isRequired");
        assert_eq!(props.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(props.is_read_only.binding_path(), "props.isReadOnly");

        crate::rsx!(
            <Checkbox
                key="root"
                {...props.checkboxProps}
                data-selected-state={props.isSelected}
                data-checked-state={props.isChecked}
            >
              Accepted
            </Checkbox>
        )
    }

    let component = ComponentCx::compile("checkbox", checkbox).unwrap();
    let mut state = ToggleState {
        selected: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.value.as_deref(), Some("accepted"));
    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setAccepted")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("checkbox")
    );
    assert_eq!(
        props.attributes.get("data-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selected-state")
            .map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setAccepted".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();

    assert!(!state.selected);
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_switch_hook_returns_switch_props_for_view_consumption() {
    fn switch(cx: &mut ComponentCx<ToggleState>) -> RSX {
        let change_action = cx.use_reducer("setEnabled", |state: &mut ToggleState, invocation| {
            state.selected = invocation.value.as_deref() == Some("true");
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_switch(move |state: &ToggleState| {
            crate::semantic_ui::UseSwitchProps::new()
                .on_change(Some(&action))
                .checked(state.selected)
                .required(true)
                .invalid(true)
                .read_only(true)
        });
        assert_eq!(props.switch_props.binding_path(), "props.switchProps");
        assert_eq!(props.is_checked.binding_path(), "props.isChecked");
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_disabled.binding_path(), "props.isDisabled");
        assert_eq!(props.is_required.binding_path(), "props.isRequired");
        assert_eq!(props.is_invalid.binding_path(), "props.isInvalid");
        assert_eq!(props.is_read_only.binding_path(), "props.isReadOnly");

        crate::rsx!(
            <Switch
                key="root"
                {...props.switchProps}
                data-selected-state={props.isSelected}
                data-checked-state={props.isChecked}
            >
              Enabled
            </Switch>
        )
    }

    let component = ComponentCx::compile("switch", switch).unwrap();
    let mut state = ToggleState {
        selected: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setEnabled")
    );
    assert_eq!(
        props.attributes.get("role").map(String::as_str),
        Some("switch")
    );
    assert_eq!(
        props.attributes.get("data-checked").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "setEnabled".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("false".to_string()),
            },
        )
        .unwrap();

    assert!(!state.selected);
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_toggle_button_hooks_return_props_for_view_consumption() {
    fn toggle_button(cx: &mut ComponentCx<ToggleState>) -> RSX {
        let press_action = cx.use_reducer("pressToggle", |state: &mut ToggleState, _invocation| {
            state.selected = !state.selected;
            state.changes += 1;
            Ok(())
        });
        let action = press_action.clone();
        let props = cx.use_toggle_button(move |state: &ToggleState| {
            crate::semantic_ui::UseToggleButtonProps::new()
                .on_press(Some(&action))
                .action_value(Some("compact"))
                .action_payload(serde_json::json!("payload"))
                .selected(state.selected)
                .pressed(true)
        });
        assert_eq!(
            props.toggle_button_props.binding_path(),
            "props.toggleButtonProps"
        );
        assert_eq!(props.is_selected.binding_path(), "props.isSelected");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");

        crate::rsx!(<button key="root" {...props.toggleButtonProps}>Toggle</button>)
    }

    fn toggle_button_group(cx: &mut ComponentCx<ToggleState>) -> RSX {
        let change_action = cx.use_reducer("setView", |state: &mut ToggleState, invocation| {
            state.selected = invocation.value.as_deref() == Some("compact");
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_toggle_button_group(move |state: &ToggleState| {
            crate::semantic_ui::UseToggleButtonGroupProps::new()
                .label(Some("View"))
                .value(if state.selected {
                    Some("compact")
                } else {
                    None
                })
                .orientation(Some("vertical"))
                .on_selection_change(Some(&action))
                .selection_mode(Some("multiple"))
        });
        assert_eq!(
            props.toggle_button_group_props.binding_path(),
            "props.toggleButtonGroupProps"
        );
        assert_eq!(props.orientation.binding_path(), "props.orientation");
        assert_eq!(props.selected_value.binding_path(), "props.selectedValue");

        crate::rsx!(<Toolbar key="root" {...props.toggleButtonGroupProps} />)
    }

    let component = ComponentCx::compile("toggle-button", toggle_button).unwrap();
    let mut state = ToggleState {
        selected: true,
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("toggle button element");
    };
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("pressToggle")
    );
    assert!(props.is_selected);
    assert_eq!(
        props.attributes.get("aria-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-selected").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("actionValue").map(String::as_str),
        Some("compact")
    );
    assert_eq!(
        props.attributes.get("actionPayload").map(String::as_str),
        Some("payload")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "pressToggle".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap();
    assert!(!state.selected);
    assert_eq!(state.changes, 1);

    let frame = ComponentCx::compile("toggle-button-group", toggle_button_group)
        .unwrap()
        .render(&ToggleState {
            selected: true,
            changes: 0,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("toggle button group element");
    };
    assert_eq!(props.label.as_deref(), Some("View"));
    assert_eq!(props.value.as_deref(), Some("compact"));
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("setView")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selection-mode")
            .map(String::as_str),
        Some("multiple")
    );
    assert_eq!(
        props
            .attributes
            .get("aria-multiselectable")
            .map(String::as_str),
        Some("true")
    );
}

#[test]
fn component_cx_text_field_hook_returns_field_and_input_props_for_view_consumption() {
    fn text_field(cx: &mut ComponentCx<TextFieldState>) -> RSX {
        let change_action = cx.use_reducer("setValue", |state: &mut TextFieldState, invocation| {
            state.value = invocation.value.clone().unwrap_or_default();
            state.changes += 1;
            Ok(())
        });
        let action = change_action.clone();
        let props = cx.use_text_field(move |state: &TextFieldState| {
            crate::semantic_ui::UseTextFieldProps::new()
                .value(Some(state.value.clone()))
                .placeholder(Some("Search"))
                .input_type(Some("search"))
                .on_change(Some(&action))
                .required(true)
                .invalid(true)
        });
        assert_eq!(props.input_props.binding_path(), "props.inputProps");
        assert_eq!(props.field_props.binding_path(), "props.fieldProps");
        assert_eq!(props.value.binding_path(), "props.value");

        crate::rsx!(
            <TextField key="root" {...props.fieldProps} data-value={props.value}>
              <Input key="input" {...props.inputProps} />
            </TextField>
        )
    }

    let component = ComponentCx::compile("text-field", text_field).unwrap();
    let mut state = TextFieldState {
        value: "alpha".to_string(),
        changes: 0,
    };
    let frame = component.render(&state).unwrap();
    let CompiledRsxNode::Element {
        props, children, ..
    } = &frame.root
    else {
        panic!("root element");
    };
    let input_props = children
        .iter()
        .find_map(|child| match child {
            CompiledRsxNode::Element { props, .. } => Some(props),
            CompiledRsxNode::Text { .. } => None,
        })
        .expect("input element");

    assert!(props.is_required);
    assert!(props.is_invalid);
    assert_eq!(
        props.attributes.get("data-value").map(String::as_str),
        Some("alpha")
    );
    assert_eq!(input_props.value.as_deref(), Some("alpha"));
    assert_eq!(input_props.placeholder.as_deref(), Some("Search"));
    assert_eq!(input_props.input_type.as_deref(), Some("search"));
    assert!(input_props.is_required);
    assert!(input_props.is_invalid);
    assert_eq!(
        input_props.events.get("onChange").map(String::as_str),
        Some("setValue")
    );
    assert_eq!(
        input_props.events.get("onInput").map(String::as_str),
        Some("setValue")
    );

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(2),
                current_target: None,
                action: "setValue".to_string(),
                event: NativeEventKind::Change,
                context: Default::default(),
                value: Some("beta".to_string()),
            },
        )
        .unwrap();

    assert_eq!(state.value, "beta");
    assert_eq!(state.changes, 1);
}

#[test]
fn component_cx_button_hook_returns_button_props_for_view_consumption() {
    fn button(cx: &mut ComponentCx<PressState>) -> RSX {
        let press_action = cx.use_reducer("press", |state: &mut PressState, _invocation| {
            state.presses += 1;
            Ok(())
        });
        let action = press_action.clone();
        let props = cx.use_button(move |state: &PressState| {
            crate::semantic_ui::UseButtonProps::new()
                .on_press(Some(&action))
                .pressed(state.pressed)
        });
        assert_eq!(props.button_props.binding_path(), "props.buttonProps");
        assert_eq!(props.press_props.binding_path(), "props.pressProps");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");

        crate::rsx!(<button key="root" {...props.buttonProps} data-active={props.isPressed}>Press</button>)
    }

    let component = ComponentCx::compile("button", button).unwrap();
    let frame = component
        .render(&PressState {
            pressed: true,
            presses: 0,
        })
        .unwrap();
    let CompiledRsxNode::Element { props, .. } = &frame.root else {
        panic!("root element");
    };

    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("press")
    );
    assert_eq!(
        props.attributes.get("data-pressed").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-active").map(String::as_str),
        Some("true")
    );
}

#[test]
fn rsx_component_hooks_mount_into_embedded_runtime_app() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = counter_component().into_runtime_app(host, CounterState::default());
    let rendered = app.render().unwrap();

    let response = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1")
    );
}

#[derive(Debug, Clone, PartialEq, Default)]
struct MountState {
    title: String,
    mounts: u32,
    unmounts: u32,
    clicks: u32,
}

#[test]
fn rsx_component_mount_hooks_initialize_state_before_first_render() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "mounted",
        r#"<Button key="click" onPress={click} label={state.title} />"#,
    )
    .unwrap()
    .use_state("title", |state: &MountState| state.title.clone())
    .use_mount(|state: &mut MountState| {
        state.mounts += 1;
        state.title = format!("Mounted {}", state.mounts);
    })
    .use_action("click", |state: &mut MountState, _invocation| {
        state.clicks += 1;
        state.title = format!("Clicked {}", state.clicks);
        Ok(())
    })
    .into_runtime_app(host, MountState::default());

    assert_eq!(app.state().mounts, 1);
    let rendered = app.render().unwrap();
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Mounted 1")
    );

    app.dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().mounts, 1);
    assert_eq!(app.state().clicks, 1);
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Clicked 1")
    );
}

#[test]
fn rsx_component_fallible_mount_hooks_return_errors_before_first_render() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let component = RsxComponent::new("mounted", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &MountState| state.title.clone())
        .use_mount_result(|state: &mut MountState| {
            state.mounts += 1;
            Err(GuiError::host("restore failed"))
        });

    let error = match component.try_into_runtime_app(host, MountState::default()) {
        Ok(_) => panic!("fallible mount should return an error"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("restore failed"));
}

#[test]
fn rsx_component_mount_returns_fallible_hook_errors() {
    let component = RsxComponent::new("mounted", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &MountState| state.title.clone())
        .use_mount_result(|state: &mut MountState| {
            state.mounts += 1;
            Err(GuiError::host("restore failed"))
        });
    let mut state = MountState::default();

    let error = component.mount(&mut state).unwrap_err();

    assert_eq!(state.mounts, 1);
    assert!(error.to_string().contains("restore failed"));
}

#[test]
fn rsx_component_infallible_constructor_surfaces_mount_errors_on_render() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new("mounted", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &MountState| state.title.clone())
        .use_mount_result(|state: &mut MountState| {
            state.mounts += 1;
            Err(GuiError::host("restore failed"))
        })
        .into_runtime_app(host, MountState::default());

    assert_eq!(app.state().mounts, 1);
    let error = app.render().unwrap_err();

    assert!(error.to_string().contains("restore failed"));
}

#[test]
fn rsx_component_unmount_hooks_run_manual_cleanup() {
    let component = RsxComponent::new("mounted", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &MountState| state.title.clone())
        .use_unmount(|state: &mut MountState| {
            state.unmounts += 1;
            state.title = format!("Unmounted {}", state.unmounts);
        });
    let mut state = MountState {
        title: "Mounted".to_string(),
        ..MountState::default()
    };

    component.unmount(&mut state).unwrap();

    assert_eq!(state.unmounts, 1);
    assert_eq!(state.title, "Unmounted 1");
}

#[test]
fn rsx_component_unmount_returns_fallible_hook_errors() {
    let component = RsxComponent::new("mounted", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &MountState| state.title.clone())
        .use_unmount_result(|state: &mut MountState| {
            state.unmounts += 1;
            Err(GuiError::host("cleanup failed"))
        });
    let mut state = MountState::default();

    let error = component.unmount(&mut state).unwrap_err();

    assert_eq!(state.unmounts, 1);
    assert!(error.to_string().contains("cleanup failed"));
}

#[test]
fn rsx_component_hooks_build_nested_scope_paths() {
    #[derive(Debug)]
    struct ProfileState {
        title: String,
    }

    let component = RsxComponent::new(
        "profile",
        r#"<Text key="title" label={state.profile.title} class={props.title.className} />"#,
    )
    .unwrap()
    .use_state("profile.title", |state: &ProfileState| state.title.clone())
    .use_prop("title.className", |_state: &ProfileState| "text-ink");
    let frame = component
        .render(&ProfileState {
            title: "RSX".to_string(),
        })
        .unwrap();

    let CompiledRsxNode::Element { props, .. } = frame.root else {
        panic!("root element");
    };
    assert_eq!(props.label.as_deref(), Some("RSX"));
    assert_eq!(props.class_name.as_deref(), Some("text-ink"));
}

#[test]
fn rsx_component_hooks_render_context_scope() {
    #[derive(Debug)]
    struct SessionState {
        user_name: String,
        theme: String,
    }

    let component = RsxComponent::new(
        "context",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="user" label={context.session.userName} />
              <Text key="theme" label={context.theme.name} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_context_value("session", |state: &SessionState| {
        Ok(serde_json::json!({ "userName": state.user_name }))
    })
    .use_context("theme.name", |state: &SessionState| state.theme.clone());

    let frame = component
        .render(&SessionState {
            user_name: "Ada".to_string(),
            theme: "dark".to_string(),
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Ada", "dark"]);
}

#[test]
fn rsx_component_fallible_typed_selectors_render_all_scope_kinds() {
    #[derive(Debug, Default)]
    struct FallibleSelectorState {
        title: String,
        user: String,
    }

    let component = RsxComponent::new(
        "fallible-selectors",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="title" label={state.title} />
          <Text key="subtitle" label={props.subtitle} />
          <Text key="summary" label={derived.summary} />
          <Text key="memo" label={derived.memo} />
          <Text key="user" label={context.session.user} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_state_result("title", |state: &FallibleSelectorState| {
        Ok(state.title.clone())
    })
    .use_prop_result("subtitle", |state: &FallibleSelectorState| {
        Ok(format!("{} subtitle", state.title))
    })
    .use_derived_result("summary", |state: &FallibleSelectorState| {
        Ok(format!("{} for {}", state.title, state.user))
    })
    .use_memo_result("memo", |_state: &FallibleSelectorState| {
        Ok("Memoized label".to_string())
    })
    .use_context_result("session.user", |state: &FallibleSelectorState| {
        Ok(state.user.clone())
    });

    let frame = component
        .render(&FallibleSelectorState {
            title: "Inbox".to_string(),
            user: "Ada".to_string(),
        })
        .unwrap();

    assert_eq!(
        child_labels(&frame.root),
        vec![
            "Inbox",
            "Inbox subtitle",
            "Inbox for Ada",
            "Memoized label",
            "Ada"
        ]
    );
}

#[test]
fn rsx_component_fallible_typed_selectors_return_render_errors() {
    let component = RsxComponent::new(
        "fallible-selectors",
        r#"<Text key="title" label={state.title} />"#,
    )
    .unwrap()
    .use_state_result::<String, _>("title", |_state: &CounterState| {
        Err(GuiError::host("state selector failed"))
    });

    let error = component.render(&CounterState::default()).unwrap_err();

    assert!(error.to_string().contains("state selector failed"));
}

#[test]
fn rsx_component_context_scope_flows_through_registered_components() {
    let component = RsxComponent::new(
        "context",
        r#"
            <Toolbar key="root" orientation="vertical">
              <ProfileBadge key="badge" title={state.title} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_component(
        "ProfileBadge",
        r#"
            <Toolbar key="root" orientation="horizontal">
              <Text key="title" label={props.title} />
              <Text key="user" label={context.session.userName} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone())
    .use_context("session.userName", |state: &ListState| {
        state.selected_id.clone().unwrap_or_default()
    });

    let frame = component
        .render(&ListState {
            title: "Inbox".to_string(),
            selected_id: Some("Ada".to_string()),
            ..ListState::default()
        })
        .unwrap();

    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let CompiledRsxNode::Element {
        children: badge_children,
        ..
    } = &children[0]
    else {
        panic!("badge element");
    };
    assert_eq!(
        badge_children
            .iter()
            .map(|child| {
                let CompiledRsxNode::Element { props, .. } = child else {
                    panic!("badge child element");
                };
                props.label.as_deref().unwrap_or_default()
            })
            .collect::<Vec<_>>(),
        vec!["Inbox", "Ada"]
    );
}

#[test]
fn rsx_component_named_sources_render_registered_component_templates() {
    #[derive(Debug, Default)]
    struct NamedSourceState {
        title: String,
        subtitle: String,
    }

    let component = RsxComponent::from_source(
        "profile",
        "ui/profile.rsx",
        r#"
        <Toolbar key="root" orientation="vertical">
          <ProfileCard
            key="card"
            title={state.title}
            subtitle={state.subtitle}
          />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_component_source_with_contract(
        "ProfileCard",
        "ui/components/profile-card.rsx",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="title" label={props.title} />
          <Text key="subtitle" label={props.subtitle} />
        </Toolbar>
        "#,
        RsxComponentContract::new()
            .required(["title"])
            .optional(["subtitle"]),
    )
    .unwrap()
    .use_state("title", |state: &NamedSourceState| state.title.clone())
    .use_state("subtitle", |state: &NamedSourceState| {
        state.subtitle.clone()
    });

    assert_eq!(component.template().source_name(), Some("ui/profile.rsx"));

    let frame = component
        .render(&NamedSourceState {
            title: "A3S".to_string(),
            subtitle: "Native RSX".to_string(),
        })
        .unwrap();

    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let CompiledRsxNode::Element {
        children: card_children,
        ..
    } = &children[0]
    else {
        panic!("card element");
    };
    assert_eq!(
        card_children
            .iter()
            .map(|child| {
                let CompiledRsxNode::Element { props, .. } = child else {
                    panic!("card child element");
                };
                props.label.as_deref().unwrap_or_default()
            })
            .collect::<Vec<_>>(),
        vec!["A3S", "Native RSX"]
    );
}

#[test]
fn rsx_template_from_file_reads_rsx_files_with_source_names() {
    let path = std::env::temp_dir().join(format!(
        "a3s-gui-rsx-template-{}-{}.rsx",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::write(&path, r#"<Text key="title" label={state.title} />"#).unwrap();

    let source_name = path.display().to_string();
    let template = RsxTemplate::from_file(&path).unwrap();
    let frame = template
        .render_with_state("profile", &serde_json::json!({ "title": "From file" }))
        .unwrap();

    std::fs::remove_file(&path).unwrap();

    assert_eq!(template.source_name(), Some(source_name.as_str()));
    let CompiledRsxNode::Element { props, .. } = frame.root else {
        panic!("text element");
    };
    assert_eq!(props.label.as_deref(), Some("From file"));
}

#[test]
fn rsx_component_resolves_static_computed_binding_paths() {
    #[derive(Debug, Default)]
    struct IndexedState;

    let component = RsxComponent::new(
        "indexed",
        r#"
            <Text
              key="first"
              label={state.items[0].title}
              class={props.classes["primary"]}
              data-theme={context["theme"].name}
            />
            "#,
    )
    .unwrap()
    .use_state_value("items", |_state: &IndexedState| {
        Ok(serde_json::json!([{ "title": "First" }]))
    })
    .use_prop_value("classes", |_state: &IndexedState| {
        Ok(serde_json::json!({ "primary": "font-medium" }))
    })
    .use_context_value("theme", |_state: &IndexedState| {
        Ok(serde_json::json!({ "name": "dark" }))
    });

    let frame = component.render(&IndexedState).unwrap();

    let CompiledRsxNode::Element { props, .. } = frame.root else {
        panic!("text element");
    };
    assert_eq!(props.label.as_deref(), Some("First"));
    assert_eq!(props.class_name.as_deref(), Some("font-medium"));
    assert_eq!(
        props.attributes.get("data-theme").map(String::as_str),
        Some("dark")
    );
}

#[test]
fn rsx_component_resolves_spread_props_from_prop_hooks() {
    let component = RsxComponent::new(
        "spread",
        r#"
            <Button
              key="save"
              {...props.primaryButton}
              label="Save"
              disabled={false}
              data-source="explicit"
            />
            "#,
    )
    .unwrap()
    .use_prop("primaryButton", |_state: &CounterState| {
        serde_json::json!({
            "label": "Spread label",
            "isDisabled": true,
            "className": "rounded-md border border-hairline bg-canvas",
            "style": "opacity: 0.5",
            "onPress": "saveDocument",
            "data-source": "spread",
            "data-kind": "primary"
        })
    })
    .use_action("saveDocument", |state: &mut CounterState, _invocation| {
        state.count += 1;
        Ok(())
    });

    let frame = component.render(&CounterState::default()).unwrap();

    let CompiledRsxNode::Element { props, .. } = frame.root else {
        panic!("button element");
    };
    assert_eq!(props.label.as_deref(), Some("Save"));
    assert!(!props.is_disabled);
    assert_eq!(
        props.class_name.as_deref(),
        Some("rounded-md border border-hairline bg-canvas")
    );
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("saveDocument")
    );
    assert_eq!(
        props.attributes.get("data-source").map(String::as_str),
        Some("explicit")
    );
    assert_eq!(
        props.attributes.get("data-kind").map(String::as_str),
        Some("primary")
    );
    assert_eq!(
        props.style.get("opacity"),
        Some(&CompiledStyleValue::Number(0.5))
    );
}

#[test]
fn rsx_component_rejects_spread_props_that_are_not_objects() {
    let component = RsxComponent::new("spread", r#"<Button key="save" {...props.bad} />"#)
        .unwrap()
        .use_prop("bad", |_state: &CounterState| "not an object");

    let error = component.render(&CounterState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX spread props.bad must resolve to an object"));
}

#[test]
fn rsx_component_requires_reducer_hooks_for_template_actions() {
    let component = RsxComponent::<CounterState>::new(
        "counter",
        r#"<Button key="counter" onPress={increment}>Count</Button>"#,
    )
    .unwrap();

    let error = component.render(&CounterState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("has no reducer hook; add use_action or use_reducer"));
}

#[test]
fn rsx_component_validates_configured_hooks_before_mounting() {
    counter_component().validate().unwrap();
}

#[test]
fn rsx_component_validate_rejects_static_actions_before_render() {
    let component = RsxComponent::<CounterState>::new(
        "counter",
        r#"
            <Show key="hidden" when={false}>
              <Button key="counter" onPress={increment}>Count</Button>
            </Show>
            "#,
    )
    .unwrap();

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"increment\" has no reducer hook"));
}

#[test]
fn rsx_component_validate_rejects_window_actions_before_render() {
    let component =
        RsxComponent::<CounterState>::new("window", r#"<Text key="title" label="Ready" />"#)
            .unwrap()
            .with_window(WindowOptions {
                title: "Counter".to_string(),
                on_close: Some("closeWindow".to_string()),
                width: None,
                height: None,
                min_width: None,
                min_height: None,
                max_width: None,
                max_height: None,
                resizable: true,
            });

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"closeWindow\" has no reducer hook"));
}

#[test]
fn rsx_component_try_into_runtime_app_validates_before_mounting() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let component = RsxComponent::<CounterState>::new(
        "counter",
        r#"<Button key="counter" onPress={increment}>Count</Button>"#,
    )
    .unwrap();

    let error = match component.try_into_runtime_app(host, CounterState::default()) {
        Ok(_) => panic!("component should fail preflight validation"),
        Err(error) => error,
    };

    assert!(error
        .to_string()
        .contains("RSX action \"increment\" has no reducer hook"));
}

#[test]
fn rsx_component_rejects_duplicate_hook_paths() {
    let component = RsxComponent::new("title", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |_state: &CounterState| "First")
        .use_state("title", |_state: &CounterState| "Second");

    let error = component.render(&CounterState::default()).unwrap_err();

    assert!(error.to_string().contains("registered more than once"));
}

#[test]
fn rsx_component_validate_rejects_duplicate_hook_paths_before_render() {
    let component = RsxComponent::new("title", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |_state: &CounterState| "First")
        .use_state("title", |_state: &CounterState| "Second");

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX state hook path \"title\" was registered more than once"));
}

#[test]
fn rsx_component_validate_rejects_missing_state_hooks_before_render() {
    let component =
        RsxComponent::<CounterState>::new("title", r#"<Text key="title" label={state.title} />"#)
            .unwrap();

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX binding state.title has no state hook"));
}

#[test]
fn rsx_component_validate_rejects_missing_prop_hooks_before_render() {
    let component = RsxComponent::<CounterState>::new(
        "title",
        r#"<Text key="title" className={props.titleClass} label="Title" />"#,
    )
    .unwrap();

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX binding props.titleClass has no props hook"));
}

#[derive(Debug, Clone, PartialEq, Default)]
struct VisibilityState {
    ready: bool,
    hidden: bool,
    message: String,
}

#[test]
fn rsx_component_flattens_native_show_controls() {
    let component = RsxComponent::new(
        "visibility",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="always" label="Always" />
              <Show key="ready-slot" when={state.ready}>
                <Text key="ready" label={state.message} />
              </Show>
              <When key="visible-slot" unless={state.hidden}>
                <Text key="visible" label="Visible" />
              </When>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("ready", |state: &VisibilityState| state.ready)
    .use_state("hidden", |state: &VisibilityState| state.hidden)
    .use_state("message", |state: &VisibilityState| state.message.clone());

    let waiting = component
        .render(&VisibilityState {
            ready: false,
            hidden: false,
            message: "Ready".to_string(),
        })
        .unwrap();
    assert_eq!(child_labels(&waiting.root), vec!["Always", "Visible"]);

    let ready = component
        .render(&VisibilityState {
            ready: true,
            hidden: true,
            message: "Ready".to_string(),
        })
        .unwrap();
    assert_eq!(child_labels(&ready.root), vec!["Always", "Ready"]);
}

#[test]
fn rsx_component_lowers_simple_rsx_condition_sugar_to_native_controls() {
    let component = RsxComponent::new(
        "visibility",
        r#"
            <Toolbar key="root" orientation="vertical">
              {state.ready && <Text key="ready" label={state.message} />}
              {!state.hidden && <Text key="visible" label="Visible" />}
              {state.ready
                ? <Text key="ternary-ready" label="Ready branch" />
                : <Text key="ternary-waiting" label="Waiting branch" />}
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("ready", |state: &VisibilityState| state.ready)
    .use_state("hidden", |state: &VisibilityState| state.hidden)
    .use_state("message", |state: &VisibilityState| state.message.clone());

    let waiting = component
        .render(&VisibilityState {
            ready: false,
            hidden: false,
            message: "Ready".to_string(),
        })
        .unwrap();
    assert_eq!(
        child_labels(&waiting.root),
        vec!["Visible", "Waiting branch"]
    );

    let ready = component
        .render(&VisibilityState {
            ready: true,
            hidden: true,
            message: "Ready".to_string(),
        })
        .unwrap();
    assert_eq!(child_labels(&ready.root), vec!["Ready", "Ready branch"]);
}

#[test]
fn rsx_component_skips_unrendered_show_child_bindings() {
    let component = RsxComponent::new(
        "visibility",
        r#"
            <Show key="root" when={state.ready}>
              <Text key="missing" label={state.missing} />
            </Show>
            "#,
    )
    .unwrap()
    .use_state("ready", |state: &VisibilityState| state.ready);

    component.validate().unwrap();

    let hidden = component.render(&VisibilityState::default()).unwrap();
    assert_eq!(child_labels(&hidden.root), Vec::<&str>::new());

    let error = component
        .render(&VisibilityState {
            ready: true,
            ..VisibilityState::default()
        })
        .unwrap_err();
    assert!(error
        .to_string()
        .contains("cannot resolve missing path segment"));
}

#[test]
fn rsx_component_rejects_show_without_condition() {
    let component = RsxComponent::<VisibilityState>::new(
        "visibility",
        r#"
            <Show key="root">
              <Text key="ready" label="Ready" />
            </Show>
            "#,
    )
    .unwrap();

    let error = component.render(&VisibilityState::default()).unwrap_err();
    assert!(error
        .to_string()
        .contains("needs a boolean when={...} or unless={...} binding"));
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
struct ListItem {
    id: String,
    title: String,
    visible: bool,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ListState {
    items: Vec<ListItem>,
    title: String,
    selected_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct FormState {
    email: String,
    title: String,
    volume: f64,
    theme: String,
    notifications: bool,
}

#[test]
fn rsx_component_expands_native_for_controls_with_local_bindings() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" indexAs="index" keyBy="id">
                <Show key="visible" when={item.visible}>
                  <Text key="row" label={item.title} data-index={index} />
                </Show>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone());

    let frame = component
        .render(&ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", false),
                list_item("gamma", "Gamma", true),
            ],
            title: String::new(),
            selected_id: None,
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Alpha", "Gamma"]);
    assert_eq!(
        child_keys(&frame.root),
        vec!["items-alpha-row", "items-gamma-row"]
    );
    assert_eq!(child_attributes(&frame.root, "data-index"), vec!["0", "2"]);
}

#[test]
fn rsx_component_lowers_map_sugar_to_native_for_controls() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              {state.items.map((item, index) => (
                <Text key={item.id} label={item.title} data-index={index} />
              ))}
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone());

    let frame = component
        .render(&ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: None,
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Alpha", "Beta"]);
    assert_eq!(child_attributes(&frame.root, "data-index"), vec!["0", "1"]);
    assert!(child_keys(&frame.root)
        .iter()
        .all(|key| key.contains("alpha") || key.contains("beta")));
}

#[test]
fn rsx_component_rejects_for_each_that_is_not_an_array() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.title} as="item">
                <Text key="row" label={item} />
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone());

    let error = component
        .render(&ListState {
            title: "Not a list".to_string(),
            ..ListState::default()
        })
        .unwrap_err();

    assert!(error.to_string().contains("must resolve to an array"));
}

#[test]
fn rsx_component_expands_registered_child_components_with_props_and_actions() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <CommandRow
                  key="row"
                  title={item.title}
                  selected={item.visible}
                  onPress={selectItem}
                  actionPayload={item}
                />
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_component(
        "CommandRow",
        r#"
            <Button
              key="root"
              onPress={props.onPress}
              isSelected={props.selected}
              actionPayload={props.actionPayload}
            >
              {props.title}
            </Button>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_reducer(
        "selectItem",
        |state: &mut ListState, invocation: &ActionInvocation| {
            let Some(item) = invocation.payload::<ListItem>()? else {
                return Err(GuiError::host("selectItem expected an item payload"));
            };
            state.selected_id = Some(item.id);
            state.title = item.title;
            Ok(())
        },
    )
    .into_runtime_app(
        host,
        ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", false),
            ],
            title: String::new(),
            selected_id: None,
        },
    );

    let rendered = app.render().unwrap();
    let planning = app.runtime().host().planning();
    let root = planning.node(rendered.root).unwrap();
    assert_eq!(
        root.children
            .iter()
            .map(|id| {
                planning
                    .node(*id)
                    .and_then(|node| node.blueprint.label.as_deref())
                    .unwrap_or_default()
            })
            .collect::<Vec<_>>(),
        vec!["Alpha", "Beta"]
    );
    assert!(
        planning
            .node(root.children[0])
            .unwrap()
            .blueprint
            .control_state
            .selected
    );
    assert!(
        !planning
            .node(root.children[1])
            .unwrap()
            .blueprint
            .control_state
            .selected
    );
    let beta = action_node(&app, "selectItem", Some("beta"));

    app.dispatch_native_event(NativeEvent::new(beta, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().selected_id.as_deref(), Some("beta"));
    assert_eq!(app.state().title, "Beta");
}

#[test]
fn rsx_component_validates_registered_component_prop_contracts() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <CommandRow
                key="row"
                title={state.title}
                selected={state.ready}
                onPress={selectItem}
                actionPayload={state.payload}
              />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_component_with_contract(
        "CommandRow",
        r#"
            <Button
              key="root"
              label={props.title}
              onPress={props.onPress}
              isSelected={props.selected}
              actionPayload={props.actionPayload}
            />
            "#,
        RsxComponentContract::new()
            .required(["title", "onPress"])
            .optional(["selected", "actionPayload"]),
    )
    .unwrap()
    .use_state("title", |state: &VisibilityState| state.message.clone())
    .use_state("ready", |state: &VisibilityState| state.ready)
    .use_state("payload", |state: &VisibilityState| state.message.clone())
    .use_reducer("selectItem", |_state: &mut VisibilityState, _invocation| {
        Ok(())
    });

    component.validate().unwrap();
    let frame = component
        .render(&VisibilityState {
            ready: true,
            hidden: false,
            message: "Open".to_string(),
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Open"]);
}

#[test]
fn rsx_component_rejects_missing_required_component_props() {
    let component = RsxComponent::new("items", r#"<CommandRow key="row" onPress={selectItem} />"#)
        .unwrap()
        .use_component_with_contract(
            "CommandRow",
            r#"<Button key="root" label={props.title} onPress={props.onPress} />"#,
            RsxComponentContract::new().required(["title", "onPress"]),
        )
        .unwrap()
        .use_reducer("selectItem", |_state: &mut ListState, _invocation| Ok(()));

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX component \"CommandRow\" requires prop \"title\""));
}

#[test]
fn rsx_component_rejects_unknown_component_props_when_contract_is_closed() {
    let component = RsxComponent::new(
        "items",
        r#"<CommandRow key="row" title={state.title} titel="Typo" onPress={selectItem} />"#,
    )
    .unwrap()
    .use_component_with_contract(
        "CommandRow",
        r#"<Button key="root" label={props.title} onPress={props.onPress} />"#,
        RsxComponentContract::new().required(["title", "onPress"]),
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone())
    .use_reducer("selectItem", |_state: &mut ListState, _invocation| Ok(()));

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX component \"CommandRow\" received unknown prop \"titel\""));
}

#[test]
fn rsx_component_rejects_component_template_bindings_missing_from_contract() {
    let component = RsxComponent::new(
        "items",
        r#"<CommandRow key="row" title={state.title} onPress={selectItem} />"#,
    )
    .unwrap()
    .use_component_with_contract(
        "CommandRow",
        r#"
            <Button
              key="root"
              label={props.title}
              data-detail={props.detail}
              onPress={props.onPress}
            />
            "#,
        RsxComponentContract::new().required(["title", "onPress"]),
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone())
    .use_reducer("selectItem", |_state: &mut ListState, _invocation| Ok(()));

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX component \"CommandRow\" template binding props.detail is not declared"));
}

#[test]
fn rsx_component_allows_extra_component_props_for_open_contracts() {
    let component = RsxComponent::new(
        "items",
        r#"<CommandRow key="row" title={state.title} data-kind="command" onPress={selectItem} />"#,
    )
    .unwrap()
    .use_component_with_contract(
        "CommandRow",
        r#"<Button key="root" label={props.title} onPress={props.onPress} />"#,
        RsxComponentContract::new()
            .required(["title", "onPress"])
            .allow_extra_props(),
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone())
    .use_reducer("selectItem", |_state: &mut ListState, _invocation| Ok(()));

    component.validate().unwrap();
}

#[test]
fn rsx_component_contract_default_props_render_and_allow_overrides() {
    let component = RsxComponent::<ListState>::new(
        "badges",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Badge key="default" />
          <Badge key="danger" tone="danger" />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_component_with_contract(
        "Badge",
        r#"<Text key="root" label={props.tone} />"#,
        RsxComponentContract::new()
            .default_prop("tone", "neutral")
            .unwrap(),
    )
    .unwrap();

    let frame = component.render(&ListState::default()).unwrap();

    assert_eq!(child_labels(&frame.root), vec!["neutral", "danger"]);
    component.validate().unwrap();
}

#[test]
fn rsx_component_contract_default_props_satisfy_template_bindings() {
    let component = RsxComponent::<ListState>::new("badge", r#"<Badge key="badge" />"#)
        .unwrap()
        .use_component_with_contract(
            "Badge",
            r#"<Text key="root" label={props.tone} />"#,
            RsxComponentContract::new()
                .default_prop("tone", "neutral")
                .unwrap(),
        )
        .unwrap();

    component.validate().unwrap();
}

#[test]
fn rsx_component_contract_rejects_invalid_default_prop_values() {
    let contract = RsxComponentContract::new()
        .default_prop_value("metadata", serde_json::json!({ "tone": "neutral" }))
        .unwrap();
    let error = match RsxComponent::<ListState>::new("badge", r#"<Badge key="badge" />"#)
        .unwrap()
        .use_component_with_contract(
            "Badge",
            r#"<Text key="root" label={props.metadata} />"#,
            contract,
        ) {
        Ok(_) => panic!("component contract should reject invalid default prop values"),
        Err(error) => error,
    };

    assert!(error
        .to_string()
        .contains("binding for property \"metadata\" must resolve to a scalar value"));
}

#[test]
fn rsx_component_expands_registered_component_default_slots() {
    let component = RsxComponent::new(
        "panel",
        r#"
            <Panel key="panel" title={state.title}>
              <Text key="body" label="Ready" />
            </Panel>
            "#,
    )
    .unwrap()
    .use_component(
        "Panel",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="title" label={props.title} />
              <Slot key="content" />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone());

    let frame = component
        .render(&ListState {
            title: "Inbox".to_string(),
            ..ListState::default()
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Inbox", "Ready"]);
    assert_eq!(
        child_keys(&frame.root),
        vec!["panel-title", "panel-content-body"]
    );
}

#[test]
fn rsx_component_expands_registered_component_named_slots() {
    let component = RsxComponent::new(
        "panel",
        r#"
            <Panel key="panel" title={state.title}>
              <Text key="body" label="Ready" />
              <Text key="confirm" slot="footer" label="Confirm" />
              <Text key="cancel" slot="footer" label="Cancel" />
            </Panel>
            "#,
    )
    .unwrap()
    .use_component(
        "Panel",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="title" label={props.title} />
              <Slot key="content" />
              <Toolbar key="footer" orientation="horizontal">
                <Slot key="footer-items" name="footer" />
              </Toolbar>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("title", |state: &ListState| state.title.clone());

    let frame = component
        .render(&ListState {
            title: "Inbox".to_string(),
            ..ListState::default()
        })
        .unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Inbox", "Ready", ""]);
    assert_eq!(
        child_keys(&frame.root),
        vec!["panel-title", "panel-content-body", "panel-footer"]
    );

    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let CompiledRsxNode::Element {
        children: footer_children,
        ..
    } = &children[2]
    else {
        panic!("footer element");
    };
    assert_eq!(
        footer_children
            .iter()
            .map(|child| match child {
                CompiledRsxNode::Element { key, props, .. } => {
                    assert!(!props.attributes.contains_key("slot"));
                    (key.as_str(), props.label.as_deref().unwrap_or_default())
                }
                CompiledRsxNode::Text { .. } => panic!("footer item element"),
            })
            .collect::<Vec<_>>(),
        vec![
            ("panel-footer-items-confirm", "Confirm"),
            ("panel-footer-items-cancel", "Cancel")
        ]
    );
}

#[test]
fn rsx_component_omits_empty_registered_component_default_slots() {
    let component =
        RsxComponent::<ListState>::new("panel", r#"<Panel key="panel" title="Inbox" />"#)
            .unwrap()
            .use_component(
                "Panel",
                r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="title" label={props.title} />
              <Slot key="content" />
            </Toolbar>
            "#,
            )
            .unwrap();

    let frame = component.render(&ListState::default()).unwrap();

    assert_eq!(child_labels(&frame.root), vec!["Inbox"]);
    assert_eq!(child_keys(&frame.root), vec!["panel-title"]);
}

#[test]
fn rsx_component_preserves_slot_keys_inside_for_controls() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Panel key="panel" title={item.title}>
                  <Text key="body" label={item.title} data-id={item.id} />
                </Panel>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_component(
        "Panel",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="title" label={props.title} />
              <Slot key="content" />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone());

    let frame = component
        .render(&ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            ..ListState::default()
        })
        .unwrap();

    assert_eq!(
        child_keys(&frame.root),
        vec!["items-alpha-panel-root", "items-beta-panel-root"]
    );

    let CompiledRsxNode::Element { children, .. } = &frame.root else {
        panic!("root element");
    };
    let CompiledRsxNode::Element {
        children: first_children,
        ..
    } = &children[0]
    else {
        panic!("first row element");
    };
    assert_eq!(
        first_children
            .iter()
            .map(|child| match child {
                CompiledRsxNode::Element { key, .. } | CompiledRsxNode::Text { key, .. } =>
                    key.as_str(),
            })
            .collect::<Vec<_>>(),
        vec!["items-alpha-panel-title", "items-alpha-panel-content-body"]
    );
}

#[test]
fn rsx_component_rejects_component_cycles() {
    let component = RsxComponent::<ListState>::new("items", r#"<CommandRow key="row" />"#)
        .unwrap()
        .use_component("CommandRow", r#"<CommandRow key="again" />"#)
        .unwrap();

    let error = component.render(&ListState::default()).unwrap_err();

    assert!(error.to_string().contains("component cycle detected"));
}

#[test]
fn rsx_component_hooks_render_derived_scope_and_memo_aliases() {
    let component = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="summary" label={derived.summary} className={derived.summaryClass} />
              <Show key="selected-slot" when={derived.hasSelection}>
                <Text key="selected" label={derived.selectedLabel} />
              </Show>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_derived("summary", |state: &ListState| {
        format!("{} items", state.items.len())
    })
    .use_derived("hasSelection", |state: &ListState| {
        state.selected_id.is_some()
    })
    .use_memo("summaryClass", |state: &ListState| {
        if state.items.is_empty() {
            "text-body"
        } else {
            "text-ink"
        }
    })
    .use_memo("selectedLabel", |state: &ListState| {
        state
            .selected_id
            .as_deref()
            .map(|id| format!("Selected {id}"))
            .unwrap_or_default()
    });

    let empty_selection = component
        .render(&ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: None,
        })
        .unwrap();
    assert_eq!(child_labels(&empty_selection.root), vec!["2 items"]);

    let selected = component
        .render(&ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: Some("beta".to_string()),
        })
        .unwrap();
    assert_eq!(
        child_labels(&selected.root),
        vec!["2 items", "Selected beta"]
    );
    let CompiledRsxNode::Element { children, .. } = &selected.root else {
        panic!("root element");
    };
    let CompiledRsxNode::Element { props, .. } = &children[0] else {
        panic!("summary element");
    };
    assert_eq!(props.class_name.as_deref(), Some("text-ink"));
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct ProfileResource {
    name: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ResourceState {
    profile: RsxResource<ProfileResource>,
}

#[test]
fn rsx_component_resource_hooks_render_status_data_and_error_branches() {
    let component = RsxComponent::new(
        "profile",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Show key="loading-slot" when={resource.profile.isLoading}>
            <Text key="loading" label="Loading profile" />
          </Show>
          <Show key="ready-slot" when={resource.profile.isReady}>
            <Text key="name" label={resource.profile.data.name} />
          </Show>
          <Show key="error-slot" when={resource.profile.isError}>
            <Text key="error" label={resource.profile.error} />
          </Show>
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_resource("profile", |state: &ResourceState| state.profile.clone());

    let loading = component
        .render(&ResourceState {
            profile: RsxResource::loading(),
        })
        .unwrap();
    assert_eq!(child_labels(&loading.root), vec!["Loading profile"]);

    let ready = component
        .render(&ResourceState {
            profile: RsxResource::ready(ProfileResource {
                name: "Ada".to_string(),
            }),
        })
        .unwrap();
    assert_eq!(child_labels(&ready.root), vec!["Ada"]);

    let failed = component
        .render(&ResourceState {
            profile: RsxResource::failed("offline"),
        })
        .unwrap();
    assert_eq!(child_labels(&failed.root), vec!["offline"]);
}

#[test]
fn rsx_component_validate_rejects_missing_resource_hooks_before_render() {
    let component = RsxComponent::<ResourceState>::new(
        "profile",
        r#"<Text key="status" label={resource.profile.status} />"#,
    )
    .unwrap();

    let error = component.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX binding resource.profile.status has no resource hook"));
}

fn counter_component() -> RsxComponent<CounterState> {
    RsxComponent::new(
        "counter",
        r#"
        fn CounterView(props: CounterViewProps) -> RSX {
          (
            <Button key="counter" onPress={increment} className={props.counterClass}>
              Count {state.count}
            </Button>
          )
        }
            "#,
    )
    .unwrap()
    .use_state("count", |state: &CounterState| state.count)
    .use_prop("counterClass", |state: &CounterState| {
        if state.count == 0 {
            "text-body"
        } else {
            "text-ink"
        }
    })
    .use_reducer("increment", |state: &mut CounterState, _invocation| {
        state.count += 1;
        Ok(())
    })
}

fn list_item(id: &str, title: &str, visible: bool) -> ListItem {
    ListItem {
        id: id.to_string(),
        title: title.to_string(),
        visible,
    }
}

fn child_labels(root: &CompiledRsxNode) -> Vec<&str> {
    let CompiledRsxNode::Element { children, .. } = root else {
        panic!("root element");
    };
    children
        .iter()
        .map(|child| {
            let CompiledRsxNode::Element { props, .. } = child else {
                panic!("child element");
            };
            props.label.as_deref().unwrap_or_default()
        })
        .collect()
}

fn direct_child_label<'a>(root: &'a CompiledRsxNode, key: &str) -> Option<&'a str> {
    let CompiledRsxNode::Element { children, .. } = root else {
        panic!("root element");
    };
    children.iter().find_map(|child| match child {
        CompiledRsxNode::Element {
            key: child_key,
            props,
            ..
        } if child_key == key => props.label.as_deref(),
        _ => None,
    })
}

fn text_values(root: &CompiledRsxNode) -> Vec<&str> {
    fn collect<'a>(node: &'a CompiledRsxNode, values: &mut Vec<&'a str>) {
        match node {
            CompiledRsxNode::Text { value, .. } => values.push(value),
            CompiledRsxNode::Element {
                props, children, ..
            } => {
                if let Some(value) = props.text_value.as_deref() {
                    values.push(value);
                }
                if let Some(value) = props.label.as_deref() {
                    values.push(value);
                }
                for child in children {
                    collect(child, values);
                }
            }
        }
    }

    let mut values = Vec::new();
    collect(root, &mut values);
    values
}

fn child_keys(root: &CompiledRsxNode) -> Vec<&str> {
    let CompiledRsxNode::Element { children, .. } = root else {
        panic!("root element");
    };
    children
        .iter()
        .map(|child| match child {
            CompiledRsxNode::Element { key, .. } | CompiledRsxNode::Text { key, .. } => {
                key.as_str()
            }
        })
        .collect()
}

fn child_attributes<'a>(root: &'a CompiledRsxNode, name: &str) -> Vec<&'a str> {
    let CompiledRsxNode::Element { children, .. } = root else {
        panic!("root element");
    };
    children
        .iter()
        .map(|child| {
            let CompiledRsxNode::Element { props, .. } = child else {
                panic!("child element");
            };
            props
                .attributes
                .get(name)
                .map(String::as_str)
                .unwrap_or_default()
        })
        .collect()
}

fn action_node<S, F, R>(
    app: &NativeRuntimeApp<CommandExecutingHost<Gtk4Adapter, RecordingBackend>, S, F, R>,
    action: &str,
    value: Option<&str>,
) -> HostNodeId
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    action_node_for_event(app, "onPress", action, value)
}

fn action_node_for_event<S, F, R>(
    app: &NativeRuntimeApp<CommandExecutingHost<Gtk4Adapter, RecordingBackend>, S, F, R>,
    event: &str,
    action: &str,
    value: Option<&str>,
) -> HostNodeId
where
    F: Fn(&S) -> GuiResult<UiFrame>,
    R: FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
{
    app.runtime()
        .host()
        .planning()
        .nodes()
        .iter()
        .find_map(|(id, node)| {
            let action_matches =
                node.blueprint.events.get(event).map(String::as_str) == Some(action);
            let value_matches = value.map_or(true, |value| {
                ["actionValue", "actionPayload"].into_iter().any(|name| {
                    node.blueprint
                        .metadata
                        .get(name)
                        .is_some_and(|payload| payload == value || payload.contains(value))
                })
            });
            (action_matches && value_matches).then_some(*id)
        })
        .unwrap_or_else(|| panic!("missing action node {action}"))
}
