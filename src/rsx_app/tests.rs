use super::*;
use crate::backend::{CommandExecutingHost, RecordingBackend};
use crate::compiler::{CompiledRsxNode, CompiledStyleValue};
use crate::event::{ActionInvocation, NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::platform::Gtk4Adapter;
use crate::protocol::HostEvent;
use serde::Deserialize;

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
    .use_effect(|state: &mut ComponentHookState, invocation| {
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
                action: "increment".to_string(),
                event: NativeEventKind::Press,
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
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    assert_eq!(state.count, 1);
    assert_eq!(text_values(&frame.root), vec!["Count ", "1"]);
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
                action: "increment".to_string(),
                event: NativeEventKind::Press,
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
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                value: None,
            },
        )
        .unwrap();

    let frame = component.render(&state).unwrap();
    assert_eq!(text_values(&frame.root), vec!["Count ", "1"]);
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
        cx.use_effect(|state: &mut LifecycleHookState, invocation| {
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
                action: "click".to_string(),
                event: NativeEventKind::Press,
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

#[derive(Debug, Clone, PartialEq, Default)]
struct PressState {
    pressed: bool,
    presses: u32,
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
                action: "press".to_string(),
                event: NativeEventKind::Press,
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
fn component_cx_button_hook_returns_press_props_for_view_consumption() {
    fn button(cx: &mut ComponentCx<PressState>) -> RSX {
        let press_action = cx.use_reducer("press", |state: &mut PressState, _invocation| {
            state.presses += 1;
            Ok(())
        });
        let action = press_action.clone();
        let props = cx.use_button(move |state: &PressState| {
            crate::semantic_ui::UsePressProps::new()
                .on_press(Some(&action))
                .pressed(state.pressed)
        });
        assert_eq!(props.press_props.binding_path(), "props.pressProps");
        assert_eq!(props.is_pressed.binding_path(), "props.isPressed");

        crate::rsx!(<button key="root" {...props.pressProps} data-active={props.isPressed}>Press</button>)
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
    .use_prop("title.className", |_state: &ProfileState| "text-foreground");
    let frame = component
        .render(&ProfileState {
            title: "RSX".to_string(),
        })
        .unwrap();

    let CompiledRsxNode::Element { props, .. } = frame.root else {
        panic!("root element");
    };
    assert_eq!(props.label.as_deref(), Some("RSX"));
    assert_eq!(props.class_name.as_deref(), Some("text-foreground"));
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
            "className": "rounded-md border border-border bg-background",
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
        Some("rounded-md border border-border bg-background")
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
            "text-muted-foreground"
        } else {
            "text-foreground"
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
    assert_eq!(props.class_name.as_deref(), Some("text-foreground"));
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
            "text-muted-foreground"
        } else {
            "text-foreground"
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
