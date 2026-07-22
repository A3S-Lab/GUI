use super::*;

#[test]
fn rsx_component_passes_static_action_values_to_reducers() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Button key="select" onPress={selectItem} actionValue={item.id}>
                  {item.title}
                </Button>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_reducer(
        "selectItem",
        |state: &mut ListState, invocation: &ActionInvocation| {
            state.selected_id = invocation.value.clone();
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
}

#[test]
fn rsx_component_passes_json_action_payloads_to_reducers() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
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
    assert_eq!(app.state().title, "Beta");
}

#[test]
fn rsx_component_payload_reducer_decodes_json_action_payloads() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
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
        state.title = item.title;
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
    assert_eq!(app.state().title, "Beta");
}

#[test]
fn rsx_component_payload_reducer_decodes_scalar_action_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
        r#"
            <Toolbar key="root" orientation="vertical">
              <For key="items" each={state.items} as="item" keyBy="id">
                <Button key="select" onPress={selectItem} actionValue={item.id}>
                  {item.title}
                </Button>
              </For>
            </Toolbar>
            "#,
    )
    .unwrap()
    .use_state("items", |state: &ListState| state.items.clone())
    .use_payload_reducer("selectItem", |state: &mut ListState, id: String| {
        state.selected_id = Some(id);
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
}

#[test]
fn rsx_component_payload_reducer_rejects_missing_payloads() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
        r#"<Button key="select" onPress={selectItem}>Select</Button>"#,
    )
    .unwrap()
    .use_payload_reducer("selectItem", |state: &mut ListState, id: String| {
        state.selected_id = Some(id);
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

#[test]
fn rsx_component_field_hook_controls_text_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
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
    .into_runtime_app(
        host,
        FormState {
            email: "ada@example.com".to_string(),
            ..FormState::default()
        },
    );
    app.render().unwrap();
    assert_eq!(
        app.runtime().accessibility_tree().unwrap().value.as_deref(),
        Some("ada@example.com")
    );
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    let response = app
        .dispatch_native_event(
            NativeEvent::new(email, NativeEventKind::Change).value("grace@example.com"),
        )
        .unwrap();

    assert_eq!(app.state().email, "grace@example.com");
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("grace@example.com")
    );
}

#[test]
fn rsx_component_field_hook_controls_boolean_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"
            <Switch
              key="notifications"
              label="Notifications"
              isChecked={state.notifications}
              onChange={setNotifications}
            />
            "#,
    )
    .unwrap()
    .use_field(
        "notifications",
        "setNotifications",
        |state: &FormState| state.notifications,
        |state: &mut FormState, notifications: bool| {
            state.notifications = notifications;
            Ok(())
        },
    )
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let toggle = action_node_for_event(&app, "onChange", "setNotifications", None);

    let response = app
        .dispatch_native_event(NativeEvent::new(toggle, NativeEventKind::Toggle))
        .unwrap();

    assert!(app.state().notifications);
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.checked),
        Some(true)
    );
}

#[test]
fn rsx_component_labeled_field_hook_registers_action_label() {
    let component = RsxComponent::new(
        "form",
        r#"<TextField key="email" label="Email" value={state.email} onChange={setEmail} />"#,
    )
    .unwrap()
    .use_labeled_field(
        "email",
        "setEmail",
        Some("Set email"),
        |state: &FormState| state.email.clone(),
        |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        },
    );
    let frame = component.render(&FormState::default()).unwrap();

    assert_eq!(
        frame
            .actions
            .iter()
            .find(|action| action.id == "setEmail")
            .and_then(|action| action.label.as_deref()),
        Some("Set email")
    );
}

#[test]
fn rsx_component_field_hook_rejects_missing_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={setEmail}>Save</Button>"#,
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
    .into_runtime_app(host, FormState::default());
    let rendered = app.render().unwrap();

    let error = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"setEmail\" expected value"));
}

#[test]
fn rsx_component_value_reducer_decodes_text_change_values_and_rerenders() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"<TextField key="email" label="Email" value={state.email} onChange={setEmail} />"#,
    )
    .unwrap()
    .use_state("email", |state: &FormState| state.email.clone())
    .use_value_reducer("setEmail", |state: &mut FormState, email: String| {
        state.email = email;
        Ok(())
    })
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    let response = app
        .dispatch_native_event(
            NativeEvent::new(email, NativeEventKind::Change).value("ada@example.com"),
        )
        .unwrap();

    assert_eq!(app.state().email, "ada@example.com");
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.value.as_deref()),
        Some("ada@example.com")
    );
}

#[test]
fn rsx_component_value_reducer_decodes_numeric_change_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"
            <Slider
              key="volume"
              min={0}
              max={100}
              step={5}
              valueNumber={state.volume}
              onChange={setVolume}
            />
            "#,
    )
    .unwrap()
    .use_state("volume", |state: &FormState| state.volume)
    .use_value_reducer("setVolume", |state: &mut FormState, volume: f64| {
        state.volume = volume;
        Ok(())
    })
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let slider = action_node_for_event(&app, "onChange", "setVolume", None);

    app.dispatch_native_event(NativeEvent::new(slider, NativeEventKind::Change).value("50"))
        .unwrap();

    assert_eq!(app.state().volume, 50.0);
}

#[test]
fn rsx_component_value_reducer_decodes_selection_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"
            <Select
              key="theme"
              label="Theme"
              value={state.theme}
              onSelectionChange={setTheme}
            >
              <option key="light" value="light">Light</option>
              <option key="dark" value="dark">Dark</option>
            </Select>
            "#,
    )
    .unwrap()
    .use_state("theme", |state: &FormState| state.theme.clone())
    .use_value_reducer("setTheme", |state: &mut FormState, theme: String| {
        state.theme = theme;
        Ok(())
    })
    .into_runtime_app(
        host,
        FormState {
            theme: "light".to_string(),
            ..FormState::default()
        },
    );
    app.render().unwrap();
    let select = action_node_for_event(&app, "onSelectionChange", "setTheme", None);

    app.dispatch_native_event(
        NativeEvent::new(select, NativeEventKind::SelectionChange).value("dark"),
    )
    .unwrap();

    assert_eq!(app.state().theme, "dark");
}

#[test]
fn rsx_component_value_reducer_decodes_toggle_values_and_rerenders() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"
            <Switch
              key="notifications"
              label="Notifications"
              isChecked={state.notifications}
              onChange={setNotifications}
            />
            "#,
    )
    .unwrap()
    .use_state("notifications", |state: &FormState| state.notifications)
    .use_value_reducer(
        "setNotifications",
        |state: &mut FormState, notifications: bool| {
            state.notifications = notifications;
            Ok(())
        },
    )
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let toggle = action_node_for_event(&app, "onChange", "setNotifications", None);

    let enabled = app
        .dispatch_native_event(NativeEvent::new(toggle, NativeEventKind::Toggle))
        .unwrap();
    assert!(app.state().notifications);
    assert_eq!(
        enabled
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.checked),
        Some(true)
    );

    let disabled = app
        .dispatch_native_event(NativeEvent::new(toggle, NativeEventKind::Toggle))
        .unwrap();
    assert!(!app.state().notifications);
    assert_eq!(
        disabled
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.checked),
        Some(false)
    );
}

#[test]
fn rsx_component_value_reducer_rejects_missing_values() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={setEmail}>Save</Button>"#,
    )
    .unwrap()
    .use_value_reducer("setEmail", |state: &mut FormState, email: String| {
        state.email = email;
        Ok(())
    })
    .into_runtime_app(host, FormState::default());
    let rendered = app.render().unwrap();

    let error = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"setEmail\" expected value"));
}

#[test]
fn rsx_component_renders_state_driven_disabled_actions() {
    let component = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action("saveDocument", |_state: &mut FormState, _invocation| Ok(()))
    .use_action_enabled("saveDocument", |state: &FormState| !state.email.is_empty());

    let disabled = component.render(&FormState::default()).unwrap();
    assert_eq!(disabled.actions.len(), 1);
    assert_eq!(disabled.actions[0].id, "saveDocument");
    assert!(disabled.actions[0].disabled);

    let enabled = component
        .render(&FormState {
            email: "team@example.com".to_string(),
            ..FormState::default()
        })
        .unwrap();
    assert!(!enabled.actions[0].disabled);
}

#[test]
fn rsx_component_rejects_disabled_actions_before_runtime_reduce() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action("saveDocument", |state: &mut FormState, _invocation| {
        state.email = "saved@example.com".to_string();
        Ok(())
    })
    .use_action_disabled("saveDocument", |state: &FormState| state.email.is_empty())
    .into_runtime_app(host, FormState::default());
    let rendered = app.render().unwrap();

    let error = app
        .dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap_err();

    assert!(error.to_string().contains("disabled action saveDocument"));
    assert!(app.state().email.is_empty());
    assert!(app
        .runtime()
        .actions()
        .registered("saveDocument")
        .is_some_and(|action| action.disabled));
    assert!(app.runtime().actions().invocations().is_empty());
}

#[test]
fn rsx_component_rejects_disabled_actions_before_direct_reduce() {
    let component = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action("saveDocument", |state: &mut FormState, _invocation| {
        state.email = "saved@example.com".to_string();
        Ok(())
    })
    .use_action_disabled("saveDocument", |state: &FormState| state.email.is_empty());
    let mut state = FormState::default();

    let error = component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "saveDocument".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action \"saveDocument\" is disabled"));
    assert!(state.email.is_empty());
}

#[test]
fn rsx_component_rejects_disabled_action_hooks_without_reducers() {
    let component = RsxComponent::<FormState>::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action_disabled("saveDocument", |_state| true);

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action disabled hook \"saveDocument\" has no reducer hook"));
}

#[test]
fn rsx_component_rejects_duplicate_action_hooks() {
    let component = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action("saveDocument", |_state: &mut FormState, _invocation| Ok(()))
    .use_action("saveDocument", |_state: &mut FormState, _invocation| Ok(()));

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action hook \"saveDocument\" was registered more than once"));
}

#[test]
fn rsx_component_rejects_field_and_reducer_action_collisions() {
    let component = RsxComponent::new(
        "form",
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
    .use_reducer("setEmail", |_state: &mut FormState, _invocation| Ok(()));

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action hook \"setEmail\" was registered more than once"));
}

#[test]
fn rsx_component_rejects_payload_and_plain_reducer_collisions() {
    let component = RsxComponent::new(
        "items",
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
    .use_payload_reducer("selectItem", |_state: &mut ListState, _item: ListItem| {
        Ok(())
    })
    .use_reducer("selectItem", |_state: &mut ListState, _invocation| Ok(()));

    let error = component.render(&ListState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action hook \"selectItem\" was registered more than once"));
}

#[test]
fn rsx_component_rejects_duplicate_action_hooks_before_direct_reduce() {
    let component = RsxComponent::new(
        "form",
        r#"<Button key="save" onPress={saveDocument}>Save</Button>"#,
    )
    .unwrap()
    .use_action("saveDocument", |_state: &mut FormState, _invocation| Ok(()))
    .use_action("saveDocument", |_state: &mut FormState, _invocation| Ok(()));
    let mut state = FormState::default();

    let error = component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "saveDocument".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: None,
            },
        )
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action hook \"saveDocument\" was registered more than once"));
}
