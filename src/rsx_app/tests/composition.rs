use super::*;
use crate::accessibility::AccessibilityNode;

#[test]
fn rsx_component_hook_bundles_compose_page_logic() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "profile",
        r#"
        <Toolbar key="root" orientation="vertical">
          <TextField key="email" label="Email" value={state.email} onChange={setEmail} />
          <Text key="summary" label={derived.summary} />
          <Text key="audit" label={state.title} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_hooks(profile_form_hooks)
    .into_runtime_app(host, FormState::default());
    app.render().unwrap();
    let email = action_node_for_event(&app, "onChange", "setEmail", None);

    let response = app
        .dispatch_native_event(
            NativeEvent::new(email, NativeEventKind::Change).value("grace@example.com"),
        )
        .unwrap();

    assert_eq!(app.state().email, "grace@example.com");
    assert_eq!(app.state().title, "Updated grace@example.com");
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .map(child_labels_from_accessibility),
        Some(vec![
            "Email".to_string(),
            "grace@example.com".to_string(),
            "Updated grace@example.com".to_string(),
        ])
    );
}

#[test]
fn rsx_component_try_hook_bundles_register_reusable_templates() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let mut app = RsxComponent::new(
        "items",
        r#"
        <Toolbar key="root" orientation="vertical">
          <For key="items" each={state.items} as="item" keyBy="id">
            <CommandRow
              key="row"
              title={item.title}
              onPress={selectItem}
              actionPayload={item}
            />
          </For>
        </Toolbar>
        "#,
    )
    .unwrap()
    .try_use_hooks(command_row_hooks)
    .unwrap()
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
fn rsx_component_try_hook_bundles_return_registration_errors() {
    let error = RsxComponent::<ListState>::new(
        "items",
        r#"<Toolbar key="root"><InvalidRow key="row" /></Toolbar>"#,
    )
    .unwrap()
    .try_use_hooks(|component| component.use_component("commandRow", "<Button key=\"root\" />"))
    .err()
    .unwrap();

    assert!(error
        .to_string()
        .contains("must be a PascalCase identifier"));
}

#[test]
fn rsx_component_rejects_duplicate_registered_components() {
    let error = RsxComponent::<ListState>::new(
        "items",
        r#"<Toolbar key="root"><CommandRow key="row" /></Toolbar>"#,
    )
    .unwrap()
    .use_component("CommandRow", r#"<Text key="root" label="First" />"#)
    .unwrap()
    .use_component("CommandRow", r#"<Text key="root" label="Second" />"#)
    .err()
    .unwrap();

    assert!(error
        .to_string()
        .contains("RSX component \"CommandRow\" was registered more than once"));
}

#[test]
fn rsx_component_rejects_hook_bundle_action_collisions() {
    let component = RsxComponent::new(
        "profile",
        r#"
        <Toolbar key="root" orientation="vertical">
          <TextField key="email" label="Email" value={state.email} onChange={setEmail} />
          <Text key="summary" label={derived.summary} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_hooks(profile_form_hooks)
    .use_hooks(|component| {
        component.use_action("setEmail", |_state: &mut FormState, _invocation| Ok(()))
    });

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX action hook \"setEmail\" was registered more than once"));
}

#[derive(Debug, Clone, PartialEq)]
struct RouteState {
    route: String,
    title: String,
}

#[test]
fn rsx_router_selects_pages_and_dispatches_active_page_actions() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .route("settings", settings_route_component())
        .unwrap()
        .default_route("home");
    let mut app = router.into_runtime_app(
        host,
        RouteState {
            route: "home".to_string(),
            title: "Settings".to_string(),
        },
    );

    let rendered = app.render().unwrap();
    assert_eq!(rendered.frame_id, "home");
    assert_eq!(
        app.runtime()
            .accessibility_tree()
            .as_ref()
            .map(child_labels_from_accessibility),
        Some(vec!["Home".to_string(), "Settings".to_string()])
    );

    let settings = action_node(&app, "openSettings", None);
    let opened = app
        .dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().route, "settings");
    assert!(app.runtime().actions().contains("renameSettings"));
    assert!(!app.runtime().actions().contains("openSettings"));
    assert_eq!(
        opened
            .accessibility_tree
            .as_ref()
            .map(child_labels_from_accessibility),
        Some(vec!["Mounted settings".to_string(), "Rename".to_string()])
    );

    let rename = action_node(&app, "renameSettings", None);
    app.dispatch_native_event(NativeEvent::new(rename, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().title, "Renamed settings");
}

#[test]
fn rsx_router_injects_active_route_id_into_context() {
    let page = |frame_id: &str| {
        RsxComponent::<RouteState>::new(
            frame_id,
            r#"
            <Toolbar key="root" orientation="vertical">
              <Text key="route" label={context.route.id} />
            </Toolbar>
            "#,
        )
        .unwrap()
    };
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", page("home"))
        .unwrap()
        .route("settings", page("settings"))
        .unwrap();

    let home = router
        .render(&RouteState {
            route: "home".to_string(),
            title: "Settings".to_string(),
        })
        .unwrap();
    assert_eq!(child_labels(&home.root), vec!["home"]);

    let settings = router
        .render(&RouteState {
            route: "settings".to_string(),
            title: "Settings".to_string(),
        })
        .unwrap();
    assert_eq!(child_labels(&settings.root), vec!["settings"]);
}

#[test]
fn rsx_router_route_context_hooks_render_route_scoped_values() {
    let page = RsxComponent::<RouteState>::new(
        "home",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="id" label={context.route.id} />
          <Text key="title" label={context.route.title} />
          <Text key="owner" label={context.route.meta.owner} />
        </Toolbar>
        "#,
    )
    .unwrap();
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", page)
        .unwrap()
        .use_route_context("title", |state: &RouteState, route| {
            format!("{}:{route}", state.title)
        })
        .use_route_context_value("meta", |_state: &RouteState, route| {
            Ok(serde_json::json!({ "owner": route }))
        });

    let frame = router
        .render(&RouteState {
            route: "home".to_string(),
            title: "Dashboard".to_string(),
        })
        .unwrap();

    assert_eq!(
        child_labels(&frame.root),
        vec!["home", "Dashboard:home", "home"]
    );
}

#[test]
fn rsx_router_layout_renders_active_route_in_outlet() {
    let layout = RsxComponent::new(
        "layout",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="chrome" label="Shell" />
          <Slot key="content" />
          <Button key="home" label="Home" onPress={openHome} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openHome", |state: &mut RouteState, _invocation| {
        state.route = "home".to_string();
        Ok(())
    });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home_route_component())
        .unwrap()
        .route("settings", settings_route_component())
        .unwrap();

    let frame = router
        .render(&RouteState {
            route: "home".to_string(),
            title: "Settings".to_string(),
        })
        .unwrap();

    assert_eq!(frame.frame_id, "layout");
    assert_eq!(
        child_keys(&frame.root),
        vec!["chrome", "content-root", "home"]
    );
    assert_eq!(child_labels(&frame.root), vec!["Shell", "", "Home"]);

    let route_root = direct_element_child(&frame.root, 1);
    assert_eq!(
        child_keys(route_root),
        vec!["content-title", "content-settings"]
    );
    assert_eq!(child_labels(route_root), vec!["Home", "Settings"]);
    assert_eq!(
        frame
            .actions
            .iter()
            .map(|action| action.id.as_str())
            .collect::<Vec<_>>(),
        vec!["openHome", "openSettings"]
    );
}

#[test]
fn rsx_router_layout_actions_and_active_route_actions_share_runtime() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let layout = RsxComponent::new(
        "layout",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Button key="home" label="Home" onPress={openHome} />
          <Slot key="route" name="route" />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openHome", |state: &mut RouteState, _invocation| {
        state.route = "home".to_string();
        Ok(())
    });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home_route_component())
        .unwrap()
        .route("settings", settings_route_component())
        .unwrap();
    let mut app = router.into_runtime_app(
        host,
        RouteState {
            route: "settings".to_string(),
            title: "Settings".to_string(),
        },
    );

    app.render().unwrap();
    assert!(app.runtime().actions().contains("openHome"));
    assert!(app.runtime().actions().contains("renameSettings"));
    assert!(!app.runtime().actions().contains("openSettings"));

    let home = action_node(&app, "openHome", None);
    app.dispatch_native_event(NativeEvent::new(home, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().route, "home");
    assert!(app.runtime().actions().contains("openHome"));
    assert!(app.runtime().actions().contains("openSettings"));
    assert!(!app.runtime().actions().contains("renameSettings"));
}

#[test]
fn rsx_router_rejects_layout_route_action_collisions() {
    let layout = RsxComponent::new(
        "layout",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Button key="settings" label="Settings" onPress={openSettings} />
          <Slot key="content" />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home_route_component())
        .unwrap();

    let error = router.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX router layout and route \"home\" both register action \"openSettings\""));
}

#[test]
fn rsx_router_mounts_layout_once_and_keeps_route_lifecycle_scoped() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let layout = RsxComponent::new(
        "layout",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="log" label={state.title} />
          <Button key="settings" label="Settings" onPress={openSettings} />
          <Slot key="content" />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_state("title", |state: &RouteState| state.title.clone())
    .use_mount(|state: &mut RouteState| {
        state.title.push_str(" -> layout mount");
    })
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    });
    let home = RsxComponent::new("home", r#"<Text key="home" label="Home" />"#)
        .unwrap()
        .use_mount(|state: &mut RouteState| {
            state.title.push_str(" -> home mount");
        })
        .use_unmount(|state: &mut RouteState| {
            state.title.push_str(" -> home unmount");
        });
    let settings = RsxComponent::new("settings", r#"<Text key="settings" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &RouteState| state.title.clone())
        .use_mount(|state: &mut RouteState| {
            state.title.push_str(" -> settings mount");
        });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home)
        .unwrap()
        .route("settings", settings)
        .unwrap();
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "start".to_string(),
            },
        )
        .unwrap();

    assert_eq!(app.state().title, "start -> layout mount -> home mount");
    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    app.dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(
        app.state().title,
        "start -> layout mount -> home mount -> home unmount -> settings mount"
    );
}

#[test]
fn rsx_router_rejects_layout_without_route_outlet() {
    let layout = RsxComponent::new(
        "layout",
        r#"<Toolbar key="root" orientation="vertical"><Text key="title" label="Shell" /></Toolbar>"#,
    )
    .unwrap();
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home_route_component())
        .unwrap();

    let error = router.validate().unwrap_err();

    assert!(error.to_string().contains(
        "RSX router layout needs exactly one <Slot /> or <Slot name=\"route\" /> outlet"
    ));
}

#[test]
fn rsx_router_rejects_layout_with_multiple_route_outlets() {
    let layout = RsxComponent::new(
        "layout",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Slot key="primary" />
          <Slot key="secondary" name="route" />
        </Toolbar>
        "#,
    )
    .unwrap();
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .layout(layout)
        .route("home", home_route_component())
        .unwrap();

    let error = router.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX router layout cannot contain more than one route outlet"));
}

#[test]
fn rsx_router_route_context_hook_errors_fail_render() {
    let page = RsxComponent::<RouteState>::new(
        "home",
        r#"<Text key="title" label={context.route.title} />"#,
    )
    .unwrap();
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", page)
        .unwrap()
        .use_route_context_result::<String, _>("title", |_state: &RouteState, _route| {
            Err(GuiError::host("route title failed"))
        });

    let error = router
        .render(&RouteState {
            route: "home".to_string(),
            title: "Dashboard".to_string(),
        })
        .unwrap_err();

    assert!(error.to_string().contains("route title failed"));
}

#[test]
fn rsx_router_rejects_route_context_id_overrides() {
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .use_route_context("id", |_state: &RouteState, _route| "shadowed");

    let error = router.validate().unwrap_err();

    assert!(error
        .to_string()
        .contains("conflicts with reserved route id"));
}

#[test]
fn rsx_router_route_bundles_compose_app_shell_logic() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .try_use_routes(settings_route_bundle)
        .unwrap()
        .use_routes(|router| router.default_route("home"));
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "Settings".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    app.dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().route, "settings");
    assert_eq!(app.state().title, "Mounted settings via openSettings");
}

#[test]
fn rsx_router_try_route_bundles_return_registration_errors() {
    let error = RsxRouter::new(|state: &RouteState| state.route.clone())
        .try_use_routes(|router| router.route("bad route", home_route_component()))
        .err()
        .unwrap();

    assert!(error
        .to_string()
        .contains("must be non-empty and contain no whitespace"));
}

#[test]
fn rsx_router_runs_unmount_before_mount_when_route_changes() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let home = RsxComponent::new(
        "home",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Button key="settings" label="Settings" onPress={openSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    })
    .use_unmount(|state: &mut RouteState| {
        state.title.push_str(" -> home unmount");
    });
    let settings = RsxComponent::new("settings", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &RouteState| state.title.clone())
        .use_mount(|state: &mut RouteState| {
            state.title.push_str(" -> settings mount");
        });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home)
        .unwrap()
        .route("settings", settings)
        .unwrap();
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "start".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    let response = app
        .dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(app.state().title, "start -> home unmount -> settings mount");
    assert_eq!(
        response
            .accessibility_tree
            .as_ref()
            .and_then(|tree| tree.label.as_deref()),
        Some("start -> home unmount -> settings mount")
    );
}

#[test]
fn rsx_router_runs_route_effects_after_route_lifecycle_hooks() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let home = RsxComponent::new(
        "home",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Button key="settings" label="Settings" onPress={openSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    })
    .use_unmount(|state: &mut RouteState| {
        state.title.push_str(" -> home unmount");
    });
    let settings = RsxComponent::new(
        "settings",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="title" label={state.title} />
          <Button key="rename" label="Rename" onPress={renameSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_state("title", |state: &RouteState| state.title.clone())
    .use_mount(|state: &mut RouteState| {
        state.title.push_str(" -> settings mount");
    })
    .use_action("renameSettings", |state: &mut RouteState, _invocation| {
        state.title.push_str(" -> renamed");
        Ok(())
    });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home)
        .unwrap()
        .route("settings", settings)
        .unwrap()
        .use_route_effect(|state: &mut RouteState, from, to, invocation| {
            state
                .title
                .push_str(&format!(" -> route {from}/{to}/{}", invocation.action));
            Ok(())
        });
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "start".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    app.dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(
        app.state().title,
        "start -> home unmount -> settings mount -> route home/settings/openSettings"
    );

    let rename = action_node(&app, "renameSettings", None);
    app.dispatch_native_event(NativeEvent::new(rename, NativeEventKind::Press))
        .unwrap();

    assert_eq!(
        app.state().title,
        "start -> home unmount -> settings mount -> route home/settings/openSettings -> renamed"
    );
}

#[test]
fn rsx_router_cleans_up_route_render_effects_when_active_route_changes() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let home = RsxComponent::new(
        "home",
        r#"<Button key="settings" label="Settings" onPress={openSettings} />"#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    })
    .use_effect_once_with_cleanup(|state: &mut RouteState| {
        state.title.push_str(" -> home effect");
        Ok(|state: &mut RouteState| {
            state.title.push_str(" -> home cleanup");
            Ok(())
        })
    });
    let settings = RsxComponent::new("settings", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &RouteState| state.title.clone())
        .use_effect_once(|state: &mut RouteState| {
            state.title.push_str(" -> settings effect");
            Ok(())
        });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home)
        .unwrap()
        .route("settings", settings)
        .unwrap();
    let mut app = router.into_runtime_app(
        host,
        RouteState {
            route: "home".to_string(),
            title: "start".to_string(),
        },
    );

    let rendered = app.render().unwrap();
    app.dispatch_native_event(NativeEvent::new(rendered.root, NativeEventKind::Press))
        .unwrap();

    assert_eq!(
        app.state().title,
        "start -> home effect -> home cleanup -> settings effect"
    );
}

#[test]
fn rsx_router_route_transition_effect_receives_transition_details() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .route("settings", settings_route_component())
        .unwrap()
        .use_route_transition_effect(
            |state: &mut RouteState, transition: &RsxRouteTransition<'_>| {
                assert_eq!(transition.invocation().event, NativeEventKind::Press);
                assert_eq!(transition.value(), None);
                state.title = format!(
                    "Transition {} -> {} by {}",
                    transition.from(),
                    transition.to(),
                    transition.action()
                );
                Ok(())
            },
        );
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "Settings".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    app.dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap();

    assert_eq!(
        app.state().title,
        "Transition home -> settings by openSettings"
    );
}

#[test]
fn rsx_router_uses_default_route_for_unknown_routes() {
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .default_route("home");

    let frame = router
        .render(&RouteState {
            route: "missing".to_string(),
            title: "Settings".to_string(),
        })
        .unwrap();

    assert_eq!(frame.frame_id, "home");
    assert_eq!(child_labels(&frame.root), vec!["Home", "Settings"]);
}

#[test]
fn rsx_router_rejects_unknown_routes_without_a_default() {
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap();

    let error = router
        .render(&RouteState {
            route: "missing".to_string(),
            title: "Settings".to_string(),
        })
        .unwrap_err();

    assert!(error
        .to_string()
        .contains("RSX route \"missing\" is not registered"));
}

#[test]
fn rsx_router_validates_default_routes_before_mounting() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .default_route("missing");

    let error = match router.try_into_runtime_app(
        host,
        RouteState {
            route: "home".to_string(),
            title: "Settings".to_string(),
        },
    ) {
        Ok(_) => panic!("router should reject an unknown default route"),
        Err(error) => error,
    };

    assert!(error
        .to_string()
        .contains("RSX router default route \"missing\" is not registered"));
}

#[test]
fn rsx_router_try_mount_returns_route_selector_errors() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new_result(|_state: &RouteState| {
        Err(GuiError::invalid_tree("route selector failed"))
    })
    .route("home", home_route_component())
    .unwrap();

    let error = match router.try_into_runtime_app(
        host,
        RouteState {
            route: "home".to_string(),
            title: "Settings".to_string(),
        },
    ) {
        Ok(_) => panic!("router should return route selector errors"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("route selector failed"));
}

#[test]
fn rsx_router_returns_mount_errors_when_route_changes() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .route(
            "settings",
            RsxComponent::new("settings", r#"<Text key="title" label={state.title} />"#)
                .unwrap()
                .use_state("title", |state: &RouteState| state.title.clone())
                .use_mount_result(|state: &mut RouteState| {
                    state.title = "Attempted settings mount".to_string();
                    Err(GuiError::host("settings restore failed"))
                }),
        )
        .unwrap();
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "Settings".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    let error = app
        .dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap_err();

    assert_eq!(app.state().route, "settings");
    assert_eq!(app.state().title, "Attempted settings mount");
    assert!(error.to_string().contains("settings restore failed"));
}

#[test]
fn rsx_router_returns_unmount_errors_when_route_changes() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let home = RsxComponent::new(
        "home",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Button key="settings" label="Settings" onPress={openSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    })
    .use_unmount_result(|state: &mut RouteState| {
        state.title = "Attempted home cleanup".to_string();
        Err(GuiError::host("home cleanup failed"))
    });
    let settings = RsxComponent::new("settings", r#"<Text key="title" label={state.title} />"#)
        .unwrap()
        .use_state("title", |state: &RouteState| state.title.clone())
        .use_mount(|state: &mut RouteState| {
            state.title = "Mounted settings".to_string();
        });
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home)
        .unwrap()
        .route("settings", settings)
        .unwrap();
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "Settings".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    let error = app
        .dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap_err();

    assert_eq!(app.state().route, "settings");
    assert_eq!(app.state().title, "Attempted home cleanup");
    assert!(error.to_string().contains("home cleanup failed"));
}

#[test]
fn rsx_router_returns_route_effect_errors_when_route_changes() {
    let host = CommandExecutingHost::new(Gtk4Adapter, RecordingBackend::default());
    let router = RsxRouter::new(|state: &RouteState| state.route.clone())
        .route("home", home_route_component())
        .unwrap()
        .route("settings", settings_route_component())
        .unwrap()
        .use_route_effect(|state: &mut RouteState, from, to, _invocation| {
            state.title = format!("Route effect attempted {from}->{to}");
            Err(GuiError::host("route analytics failed"))
        });
    let mut app = router
        .try_into_runtime_app(
            host,
            RouteState {
                route: "home".to_string(),
                title: "Settings".to_string(),
            },
        )
        .unwrap();

    app.render().unwrap();
    let settings = action_node(&app, "openSettings", None);
    let error = app
        .dispatch_native_event(NativeEvent::new(settings, NativeEventKind::Press))
        .unwrap_err();

    assert_eq!(app.state().route, "settings");
    assert_eq!(app.state().title, "Route effect attempted home->settings");
    assert!(error.to_string().contains("route analytics failed"));
}

fn profile_form_hooks(component: RsxComponent<FormState>) -> RsxComponent<FormState> {
    component
        .use_field(
            "email",
            "setEmail",
            |state: &FormState| state.email.clone(),
            |state: &mut FormState, email: String| {
                state.email = email;
                Ok(())
            },
        )
        .use_derived("summary", |state: &FormState| {
            if state.email.is_empty() {
                "No email".to_string()
            } else {
                state.email.clone()
            }
        })
        .use_state("title", |state: &FormState| state.title.clone())
        .use_value_effect("setEmail", |state: &mut FormState, email: String| {
            state.title = format!("Updated {email}");
            Ok(())
        })
}

fn home_route_component() -> RsxComponent<RouteState> {
    RsxComponent::new(
        "home",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="title" label="Home" />
          <Button key="settings" label="Settings" onPress={openSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_action("openSettings", |state: &mut RouteState, _invocation| {
        state.route = "settings".to_string();
        Ok(())
    })
}

fn settings_route_component() -> RsxComponent<RouteState> {
    RsxComponent::new(
        "settings",
        r#"
        <Toolbar key="root" orientation="vertical">
          <Text key="title" label={state.title} />
          <Button key="rename" label="Rename" onPress={renameSettings} />
        </Toolbar>
        "#,
    )
    .unwrap()
    .use_state("title", |state: &RouteState| state.title.clone())
    .use_mount(|state: &mut RouteState| {
        state.title = "Mounted settings".to_string();
    })
    .use_action("renameSettings", |state: &mut RouteState, _invocation| {
        state.title = "Renamed settings".to_string();
        Ok(())
    })
}

fn settings_route_bundle(router: RsxRouter<RouteState>) -> GuiResult<RsxRouter<RouteState>> {
    Ok(router
        .route("settings", settings_route_component())?
        .use_route_effect(|state: &mut RouteState, _from, _to, invocation| {
            state.title = format!("{} via {}", state.title, invocation.action);
            Ok(())
        }))
}

fn command_row_hooks(component: RsxComponent<ListState>) -> GuiResult<RsxComponent<ListState>> {
    Ok(component
        .use_component(
            "CommandRow",
            r#"
            <Button key="root" onPress={props.onPress} actionPayload={props.actionPayload}>
              {props.title}
            </Button>
            "#,
        )?
        .use_state("items", |state: &ListState| state.items.clone())
        .use_payload_reducer("selectItem", |state: &mut ListState, item: ListItem| {
            state.selected_id = Some(item.id);
            state.title = format!("Selected {}", item.title);
            Ok(())
        }))
}

fn child_labels_from_accessibility(root: &AccessibilityNode) -> Vec<String> {
    root.children
        .iter()
        .map(|child| child.label.clone().unwrap_or_default())
        .collect()
}

fn direct_element_child(root: &CompiledRsxNode, index: usize) -> &CompiledRsxNode {
    let CompiledRsxNode::Element { children, .. } = root else {
        panic!("root element");
    };
    children
        .get(index)
        .unwrap_or_else(|| panic!("child {index}"))
}
