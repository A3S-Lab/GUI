use super::*;

#[derive(Debug, Clone, PartialEq, Default)]
struct EffectState {
    count: u32,
    effects: u32,
    action_effects: u32,
    audit: Vec<String>,
}

#[test]
fn rsx_component_use_effect_runs_after_committed_renders() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"<Text key="summary" label={derived.summary} />"#,
    )
    .unwrap()
    .use_derived("summary", |state: &EffectState| {
        format!("Count {} Effects {}", state.count, state.effects)
    })
    .use_effect(|state: &mut EffectState| {
        state.effects += 1;
        Ok(())
    })
    .into_runtime_app(host, EffectState::default());

    let rendered = app.render().unwrap();

    assert_eq!(app.state().effects, 1);
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .and_then(|tree| tree.label),
        Some("Count 0 Effects 0".to_string())
    );
    assert_eq!(app.root(), Some(rendered.root));

    app.render().unwrap();
    assert_eq!(app.state().effects, 2);
}

#[test]
fn rsx_component_use_effect_runs_after_protocol_commits() {
    let mut app = RsxComponent::new(
        "effects",
        r#"<Text key="summary" label={derived.summary} />"#,
    )
    .unwrap()
    .use_derived("summary", |state: &EffectState| {
        format!("Effects {}", state.effects)
    })
    .use_effect(|state: &mut EffectState| {
        state.effects += 1;
        Ok(())
    })
    .into_protocol_app(Gtk4Adapter, EffectState::default());

    let rendered = app.render().unwrap();

    assert_eq!(app.state().effects, 1);
    assert_eq!(
        rendered
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Effects 0")
    );
}

#[test]
fn rsx_component_use_effect_once_runs_only_after_first_commit() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new("effects", r#"<Text key="summary" label="Ready" />"#)
        .unwrap()
        .use_effect_once(|state: &mut EffectState| {
            state.effects += 1;
            Ok(())
        })
        .into_runtime_app(host, EffectState::default());

    app.render().unwrap();
    app.render().unwrap();

    assert_eq!(app.state().effects, 1);
}

#[test]
fn rsx_component_use_effect_with_deps_skips_unchanged_dependencies() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="horizontal">
              <Button key="increment" onPress={increment} label="Increment" />
              <Button key="noop" onPress={noop} label="Noop" />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action("noop", |_state: &mut EffectState, _invocation| Ok(()))
    .use_effect_with_deps::<u32, _, _>(
        |state: &EffectState| state.count,
        |state: &mut EffectState| {
            state.effects += 1;
            state.audit.push(format!("effect:{}", state.count));
            Ok(())
        },
    )
    .into_runtime_app(host, EffectState::default());
    app.render().unwrap();
    let increment = action_node(&app, "increment", None);
    let noop = action_node(&app, "noop", None);

    app.render().unwrap();
    app.dispatch_native_event(NativeEvent::new(noop, NativeEventKind::Press))
        .unwrap();
    app.dispatch_native_event(NativeEvent::new(increment, NativeEventKind::Press))
        .unwrap();
    app.render().unwrap();

    assert_eq!(app.state().effects, 2);
    assert_eq!(app.state().audit, vec!["effect:0", "effect:1"]);
}

#[test]
fn rsx_component_use_effect_cleanup_runs_before_next_matching_effect_and_manual_cleanup() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"<Button key="increment" onPress={increment} label="Increment" />"#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_effect_with_deps_and_cleanup::<u32, _, _, _>(
        |state: &EffectState| state.count,
        |state: &mut EffectState| {
            let count = state.count;
            state.audit.push(format!("effect:{count}"));
            Ok(move |state: &mut EffectState| {
                state.audit.push(format!("cleanup:{count}"));
                Ok(())
            })
        },
    )
    .into_runtime_app(host, EffectState::default());
    let rendered = app.render().unwrap();

    app.dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();
    app.cleanup_effects().unwrap();

    assert_eq!(
        app.state().audit,
        vec!["effect:0", "cleanup:0", "effect:1", "cleanup:1"]
    );
}

#[test]
fn rsx_component_render_effect_phases_run_and_cleanup_in_react_order() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new("effects", r#"<Text key="summary" label="Ready" />"#)
        .unwrap()
        .use_effect_with_cleanup(|state: &mut EffectState| {
            state.audit.push("passive".to_string());
            Ok(|state: &mut EffectState| {
                state.audit.push("cleanup:passive".to_string());
                Ok(())
            })
        })
        .use_layout_effect_with_cleanup(|state: &mut EffectState| {
            state.audit.push("layout".to_string());
            Ok(|state: &mut EffectState| {
                state.audit.push("cleanup:layout".to_string());
                Ok(())
            })
        })
        .use_insertion_effect_with_cleanup(|state: &mut EffectState| {
            state.audit.push("insertion".to_string());
            Ok(|state: &mut EffectState| {
                state.audit.push("cleanup:insertion".to_string());
                Ok(())
            })
        })
        .into_runtime_app(host, EffectState::default());

    app.render().unwrap();
    app.cleanup_effects().unwrap();

    assert_eq!(
        app.state().audit,
        vec![
            "insertion",
            "layout",
            "passive",
            "cleanup:passive",
            "cleanup:layout",
            "cleanup:insertion",
        ]
    );
}

#[test]
fn rsx_component_use_deferred_value_lags_one_committed_render() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "deferred",
        r#"<Button key="increment" onPress={increment} label={derived.count} />"#,
    )
    .unwrap()
    .use_deferred_value("count", |state: &EffectState| state.count.to_string())
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .into_runtime_app(host, EffectState::default());

    let rendered = app.render().unwrap();
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .and_then(|tree| tree.label),
        Some("0".to_string())
    );

    app.dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .and_then(|tree| tree.label),
        Some("0".to_string())
    );

    app.render().unwrap();
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .and_then(|tree| tree.label),
        Some("1".to_string())
    );
}

#[test]
fn component_cx_use_effect_event_reads_latest_state_from_effects() {
    fn view(cx: &mut ComponentCx<EffectState>) -> RSX {
        let event = cx.use_effect_event(|state: &mut EffectState| {
            state.audit.push(format!("event:{}", state.count));
            Ok(())
        });
        cx.use_effect(move |state| event.call(state));

        crate::rsx!(<Text key="summary" label="Ready" />)
    }

    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = ComponentCx::compile("effect-event", view)
        .unwrap()
        .into_runtime_app(host, EffectState::default());

    app.render().unwrap();
    app.state_mut().count = 7;
    app.render().unwrap();

    assert_eq!(app.state().audit, vec!["event:0", "event:7"]);
}

#[test]
fn rsx_component_action_effect_hooks_run_after_reducers_before_rerender() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"<Button key="counter" onPress={increment} label={derived.summary} />"#,
    )
    .unwrap()
    .use_derived("summary", |state: &EffectState| {
        format!("Count {} Effects {}", state.count, state.effects)
    })
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action_effect("increment", |state: &mut EffectState, invocation| {
        assert_eq!(invocation.action, "increment");
        state.effects += state.count;
        Ok(())
    })
    .into_runtime_app(host, EffectState::default());
    let rendered = app.render().unwrap();

    let response = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(app.state().effects, 1);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1 Effects 1")
    );
}

#[test]
fn rsx_component_action_effect_hooks_filter_by_action_id() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="horizontal">
              <Button key="increment" onPress={increment} label="Increment" />
              <Button key="reset" onPress={reset} label="Reset" />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action("reset", |state: &mut EffectState, _invocation| {
        state.count = 0;
        Ok(())
    })
    .use_action_effect("reset", |state: &mut EffectState, _invocation| {
        state.effects += 1;
        Ok(())
    })
    .use_action_effect("increment", |state: &mut EffectState, _invocation| {
        state.effects += 1;
        Ok(())
    })
    .use_action_effect("increment", |state: &mut EffectState, _invocation| {
        state.action_effects += 1;
        Ok(())
    })
    .into_runtime_app(host, EffectState::default());
    app.render().unwrap();
    let reset = action_node(&app, "reset", None);
    let increment = action_node(&app, "increment", None);

    app.dispatch_native_event(NativeEvent::new(reset, NativeEventKind::Press))
        .unwrap();
    app.dispatch_native_event(NativeEvent::new(increment, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().effects, 2);
    assert_eq!(app.state().action_effects, 1);
}

#[test]
fn rsx_component_action_transition_effect_hooks_receive_before_state_after_plain_reducers() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"<Button key="counter" onPress={increment} label={state.summary} />"#,
    )
    .unwrap()
    .use_state("summary", |state: &EffectState| {
        format!("Count {} Effects {}", state.count, state.effects)
    })
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_transition_effect(
        |state: &mut EffectState, transition: &RsxActionTransition<'_, EffectState>| {
            state.audit.push(format!(
                "transition:{}->{}",
                transition.before().count,
                state.count
            ));
            state.effects += state.count - transition.before().count;
            Ok(())
        },
    )
    .use_action_effect("increment", |state: &mut EffectState, _invocation| {
        state.audit.push(format!("effect:{}", state.effects));
        Ok(())
    })
    .into_runtime_app(host, EffectState::default());
    let rendered = app.render().unwrap();

    let response = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(app.state().effects, 1);
    assert_eq!(app.state().audit, vec!["transition:0->1", "effect:1"]);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1 Effects 1")
    );
}

#[test]
fn rsx_component_action_transition_effect_hooks_filter_by_action_id() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="horizontal">
              <Button key="increment" onPress={increment} label="Increment" />
              <Button key="reset" onPress={reset} label="Reset" />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action("reset", |state: &mut EffectState, _invocation| {
        state.count = 0;
        Ok(())
    })
    .use_action_transition_effect(
        "increment",
        |state: &mut EffectState, transition: &RsxActionTransition<'_, EffectState>| {
            state.audit.push(format!(
                "{}:{}->{}",
                transition.action(),
                transition.before().count,
                state.count
            ));
            Ok(())
        },
    )
    .into_runtime_app(host, EffectState::default());
    app.render().unwrap();
    let reset = action_node(&app, "reset", None);
    let increment = action_node(&app, "increment", None);

    app.dispatch_native_event(NativeEvent::new(reset, NativeEventKind::Press))
        .unwrap();
    app.dispatch_native_event(NativeEvent::new(increment, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().audit, vec!["increment:0->1"]);
}

#[test]
fn rsx_component_rejects_action_transition_effects_without_reducer_hooks() {
    let component = RsxComponent::new(
        "effects",
        r#"<Button key="increment" onPress={increment} label="Increment" />"#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action_transition_effect(
        "incremnt",
        |state: &mut EffectState, _transition: &RsxActionTransition<'_, EffectState>| {
            state.action_effects += 1;
            Ok(())
        },
    );

    let error = component.render(&EffectState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action effect \"incremnt\" has no reducer hook"));
}

#[test]
fn rsx_component_value_transition_effect_hooks_decode_action_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="vertical">
              <TextField key="email" label="Email" value={state.email} onChange={setEmail} />
              <Text key="audit" label={state.title} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_field(
        "email",
        "setEmail",
        |state: &FormState| state.email.clone(),
        |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        },
    )
    .use_state("title", |state: &FormState| state.title.clone())
    .use_value_transition_effect(
        "setEmail",
        |state: &mut FormState, transition: &RsxActionTransition<'_, FormState>, email: String| {
            state.title = format!("{} -> {}", transition.before().email, email);
            Ok(())
        },
    )
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    app.dispatch_native_event(
        NativeEvent::new(email, NativeEventKind::Change).value("grace@example.com"),
    )
    .unwrap();

    assert_eq!(app.state().email, "grace@example.com");
    assert_eq!(app.state().title, " -> grace@example.com");
}

#[test]
fn rsx_component_payload_transition_effect_hooks_decode_action_payloads() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Button key="select" onPress={selectItem} actionPayload={item}>
                  {item.title}
                </Button>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_payload_reducer("selectItem", |state: &mut ListState, item: ListItem| {
        state.selected_id = Some(item.id);
        Ok(())
    })
    .use_payload_transition_effect(
        "selectItem",
        |state: &mut ListState, transition: &RsxActionTransition<'_, ListState>, item: ListItem| {
            assert_eq!(transition.before().selected_id, None);
            state.title = format!("Selected {}", item.title);
            Ok(())
        },
    )
    .into_runtime_app(
        host,
        ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: None,
        },
    );
    app.render().unwrap();
    let beta = action_node(&app, "selectItem", Some("beta"));

    app.dispatch_native_event(NativeEvent::new(beta, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().selected_id.as_deref(), Some("beta"));
    assert_eq!(app.state().title, "Selected Beta");
}

#[test]
fn rsx_component_transition_reducer_receives_before_state_and_after_state() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "transition",
        r#"<Button key="counter" onPress={increment} label={state.summary} />"#,
    )
    .unwrap()
    .use_state("summary", |state: &EffectState| {
        format!("Count {} Effects {}", state.count, state.effects)
    })
    .use_transition_reducer(
        "increment",
        |state: &mut EffectState, _invocation| {
            state.count += 1;
            Ok(())
        },
        |state: &mut EffectState, transition: &RsxActionTransition<'_, EffectState>| {
            assert_eq!(transition.before().count, 0);
            assert_eq!(state.count, 1);
            assert_eq!(transition.action(), "increment");
            assert_eq!(transition.value(), None);
            state.effects = transition.before().count + state.count;
            state.audit.push(format!(
                "{}:{}->{}",
                transition.action(),
                transition.before().count,
                state.count
            ));
            Ok(())
        },
    )
    .into_runtime_app(host, EffectState::default());
    let rendered = app.render().unwrap();

    let response = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().count, 1);
    assert_eq!(app.state().effects, 1);
    assert_eq!(app.state().audit, vec!["increment:0->1"]);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("Count 1 Effects 1")
    );
}

#[test]
fn rsx_component_value_transition_reducer_decodes_values_for_reducer_and_effect() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "value-transition",
        r#"
            <Toolbar key="root" orientation="vertical">
              <TextField key="email" label="Email" value={state.email} onChange={setEmail} />
              <Text key="audit" label={state.title} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("email", |state: &FormState| state.email.clone())
    .use_state("title", |state: &FormState| state.title.clone())
    .use_value_transition_reducer(
        "setEmail",
        |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        },
        |state: &mut FormState, transition: &RsxActionTransition<'_, FormState>, email: String| {
            state.title = format!(
                "{} -> {} via {}",
                transition.before().email,
                email,
                transition.action()
            );
            Ok(())
        },
    )
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    app.dispatch_native_event(
        NativeEvent::new(email, NativeEventKind::Change).value("grace@example.com"),
    )
    .unwrap();

    assert_eq!(app.state().email, "grace@example.com");
    assert_eq!(app.state().title, " -> grace@example.com via setEmail");
}

#[test]
fn rsx_component_payload_transition_reducer_decodes_payloads_for_reducer_and_effect() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "payload-transition",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Button key="select" onPress={selectItem} actionPayload={item}>
                  {item.title}
                </Button>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_payload_transition_reducer(
        "selectItem",
        |state: &mut ListState, item: ListItem| {
            state.selected_id = Some(item.id);
            Ok(())
        },
        |state: &mut ListState, transition: &RsxActionTransition<'_, ListState>, item: ListItem| {
            assert_eq!(transition.before().selected_id, None);
            assert_eq!(state.selected_id.as_deref(), Some(item.id.as_str()));
            state.title = format!("Selected {} via {}", item.title, transition.action());
            Ok(())
        },
    )
    .into_runtime_app(
        host,
        ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: None,
        },
    );
    app.render().unwrap();
    let beta = action_node(&app, "selectItem", Some("beta"));

    app.dispatch_native_event(NativeEvent::new(beta, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().selected_id.as_deref(), Some("beta"));
    assert_eq!(app.state().title, "Selected Beta via selectItem");
}

#[test]
fn rsx_component_rejects_action_effects_without_reducer_hooks() {
    let component = RsxComponent::new(
        "effects",
        r#"<Button key="increment" onPress={increment} label="Increment" />"#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action_effect("incremnt", |state: &mut EffectState, _invocation| {
        state.action_effects += 1;
        Ok(())
    });

    let error = component.render(&EffectState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action effect \"incremnt\" has no reducer hook"));
}

#[test]
fn rsx_component_rejects_typed_effects_without_reducer_hooks() {
    let component = RsxComponent::new(
        "effects",
        r#"<TextField key="email" label="Email" value={state.email} onChange={setEmail} />"#,
    )
    .unwrap()
    .use_field(
        "email",
        "setEmail",
        |state: &FormState| state.email.clone(),
        |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        },
    )
    .use_value_effect("setEmai", |state: &mut FormState, email: String| {
        state.title = email;
        Ok(())
    });

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action effect \"setEmai\" has no reducer hook"));
}

#[test]
fn rsx_component_rejects_orphan_action_effects_before_direct_reduce() {
    let component = RsxComponent::new(
        "effects",
        r#"<Button key="increment" onPress={increment} label="Increment" />"#,
    )
    .unwrap()
    .use_action("increment", |state: &mut EffectState, _invocation| {
        state.count += 1;
        Ok(())
    })
    .use_action_effect("typo", |state: &mut EffectState, _invocation| {
        state.action_effects += 1;
        Ok(())
    });
    let mut state = EffectState::default();

    let error = component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                action: "increment".to_string(),
                event: NativeEventKind::Press,
                value: None,
            },
        )
        .unwrap_err();

    assert_eq!(state.count, 0);
    assert!(error
        .to_string()
        .contains("RSX action effect \"typo\" has no reducer hook"));
}

#[test]
fn rsx_component_value_effect_hooks_decode_action_values_before_rerender() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="vertical">
              <TextField key="email" label="Email" value={state.email} onChange={setEmail} />
              <Text key="audit" label={state.title} />
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_field(
        "email",
        "setEmail",
        |state: &FormState| state.email.clone(),
        |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        },
    )
    .use_state("title", |state: &FormState| state.title.clone())
    .use_value_effect("setEmail", |state: &mut FormState, email: String| {
        assert_eq!(state.email, email);
        state.title = format!("Changed {email}");
        Ok(())
    })
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    let response = app
        .dispatch_native_event(
            NativeEvent::new(email, NativeEventKind::Change).value("grace@example.com"),
        )
        .unwrap();

    assert_eq!(app.state().title, "Changed grace@example.com");
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.children.get(1))
            .and_then(|tree| tree.label.as_deref()),
        Some("Changed grace@example.com")
    );
}

#[test]
fn rsx_component_payload_effect_hooks_decode_action_payloads() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Button key="select" onPress={selectItem} actionPayload={item}>
                  {item.title}
                </Button>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_payload_reducer("selectItem", |state: &mut ListState, item: ListItem| {
        state.selected_id = Some(item.id);
        Ok(())
    })
    .use_payload_effect("selectItem", |state: &mut ListState, item: ListItem| {
        assert_eq!(state.selected_id.as_deref(), Some(item.id.as_str()));
        state.title = format!("Effect {}", item.title);
        Ok(())
    })
    .into_runtime_app(
        host,
        ListState {
            items: vec![
                list_item("alpha", "Alpha", true),
                list_item("beta", "Beta", true),
            ],
            title: String::new(),
            selected_id: None,
        },
    );
    app.render().unwrap();
    let beta = action_node(&app, "selectItem", Some("beta"));

    app.dispatch_native_event(NativeEvent::new(beta, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().selected_id.as_deref(), Some("beta"));
    assert_eq!(app.state().title, "Effect Beta");
}

#[test]
fn rsx_component_payload_effect_hooks_reject_missing_payloads() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "effects",
        r#"<Button key="select" onPress={selectItem}>Select</Button>"#,
    )
    .unwrap()
    .use_action("selectItem", |_state: &mut ListState, _invocation| Ok(()))
    .use_payload_effect("selectItem", |state: &mut ListState, item: ListItem| {
        state.selected_id = Some(item.id);
        Ok(())
    })
    .into_runtime_app(host, ListState::default());
    let rendered = app.render().unwrap();

    let error = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"selectItem\" expected payload"));
}
