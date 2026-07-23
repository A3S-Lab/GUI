use super::*;
use std::rc::Rc;

use crate::compiler::{CompiledOrientation, CompiledRsxNode, RsxCompilerBridge};
use crate::event::{ActionInvocation, NativeEvent, NativeEventKind};
use crate::host::HostNodeId;
use crate::native::NativeRole;
use crate::platform::{AppKitAdapter, PlatformAdapter};
use crate::protocol::{HostEvent, NativeProtocolApp};
use crate::rsx_app::{ComponentCx, RsxComponent, RsxTemplate, RSX};
use crate::style::{DisplayMode, PortableStyle, StyleColor};
use crate::web::WebProps;

#[derive(Debug, Default)]
struct FormState {
    email: String,
    tab: String,
    saved: bool,
}

#[derive(Debug, Default)]
struct SelectionUiState {
    selected: String,
    read_only: bool,
}

#[derive(Debug, Default)]
struct ToggleUiState {
    enabled: bool,
}

#[derive(Debug, Default)]
struct TextInputUiState {
    value: String,
}

#[derive(Debug, Default)]
struct FieldUiState {
    disabled: bool,
    invalid: bool,
}

#[derive(Debug, Default)]
struct DisclosureUiState {
    expanded: bool,
}

#[derive(Debug, Default)]
struct RangeUiState {
    value: f64,
}

#[derive(Debug, Default)]
struct OverlayUiState {
    open: bool,
    disabled: bool,
}

#[derive(Debug, Default)]
struct MenuUiState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Default)]
struct CollectionItemUiState {
    selected: bool,
    disabled: bool,
    expanded: bool,
}

#[derive(Debug, Default)]
struct LoadMoreUiState {
    loading: bool,
    disabled: bool,
}

#[derive(Debug, Default)]
struct RadioUiState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Default)]
struct TabUiState {
    selected: bool,
    disabled: bool,
}

#[derive(Debug, Default)]
struct ControlState {
    accepted: bool,
    enabled: bool,
    theme: String,
    density: String,
    volume: f64,
}

#[derive(Debug, Default)]
struct PrimitiveState {
    name: String,
    query: String,
    expanded: bool,
    dialog_open: bool,
    progress: f64,
    quota: f64,
}

#[derive(Debug, Default)]
struct CollectionState {
    assignee: String,
    assignee_query: String,
    quantity: f64,
    compact: bool,
    view: String,
    selected_row: bool,
    modal_open: bool,
    tooltip_open: bool,
}

#[derive(Debug, Default)]
struct StructureState {
    selected: String,
    query: String,
    tree_expanded: bool,
    toast_open: bool,
}

#[derive(Debug, Default)]
struct DateTimeState {
    date: String,
    time: String,
    range_start: String,
    range_end: String,
    month: String,
    year: String,
    calendar_open: bool,
}

#[derive(Debug, Default)]
struct ColorState {
    color: String,
    hue: f64,
    saturation: f64,
    brightness: f64,
}

#[derive(Debug, Default)]
struct ComponentPartsState {
    volume: f64,
    color: String,
    saturation: f64,
    brightness: f64,
    dragging: bool,
    volume_committed: bool,
    color_committed: bool,
}

#[derive(Debug, Default)]
struct SemanticPartsState {
    menu_open: bool,
    dialog_open: bool,
    tooltip_open: bool,
    disclosure_expanded: bool,
    selected: String,
}

#[derive(Debug, Default)]
struct RouterUiState {
    path: String,
    navigations: u32,
}

fn run_rsx_stress_test(test: impl FnOnce() + Send + 'static) {
    let handle = std::thread::Builder::new()
        .name("rsx-ui-stress-test".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(test)
        .expect("spawn RSX stress test thread");

    if let Err(panic) = handle.join() {
        std::panic::resume_unwind(panic);
    }
}

#[test]
fn rsx_ui_renders_design_md_components_from_tokens() {
    let component = RsxComponent::new(
        "settings",
        r#"
        <div key="root" class="bg-canvas text-ink">
          <UiCard key="card" className="w-full">
            <UiCardHeader key="header">
              <UiCardTitle key="title">Settings</UiCardTitle>
              <UiCardDescription key="description">Native RSX controls</UiCardDescription>
            </UiCardHeader>
            <UiCardContent key="content">
              <UiInput
                key="email"
                value={state.email}
                placeholder="Email"
                onChange={setEmail}
              />
            </UiCardContent>
            <UiCardFooter key="footer">
              <UiButton key="save" className="w-full" onPress={saveProfile}>
                Save
              </UiButton>
            </UiCardFooter>
          </UiCard>
        </div>
        "#,
    )
    .unwrap()
    .use_state("email", |state: &FormState| state.email.clone())
    .use_value_reducer("setEmail", |state: &mut FormState, email: String| {
        state.email = email;
        Ok(())
    })
    .use_reducer(
        "saveProfile",
        |state: &mut FormState, _invocation: &ActionInvocation| {
            state.saved = true;
            Ok(())
        },
    );

    let frame = component
        .render(&FormState {
            email: "grace@example.com".to_string(),
            tab: String::new(),
            saved: false,
        })
        .unwrap();

    let actions = frame
        .actions
        .iter()
        .map(|action| action.id.as_str())
        .collect::<Vec<_>>();
    assert!(actions.contains(&"setEmail"));
    assert!(actions.contains(&"saveProfile"));

    let card = find_element_by_attribute(&frame.root, "data-slot", "card").unwrap();
    let button = find_element_by_attribute(&frame.root, "data-slot", "button").unwrap();
    let input = find_element_by_attribute(&frame.root, "data-slot", "input").unwrap();

    assert_class_contains(card, "bg-canvas");
    assert_class_contains(card, "w-full");
    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "bg-primary");
    assert_class_contains(button, "text-on-primary");
    assert_class_contains(button, "rounded-md");
    assert_class_contains(button, "h-9");
    assert_class_contains(button, "w-full");
    assert_class_contains(input, "bg-surface-card");
    assert_class_contains(input, "border-hairline-strong");
    assert_class_contains(input, "rounded-md");

    let button_style =
        PortableStyle::from_web(&WebProps::new().class_name(class_name(button).to_string()));
    assert_eq!(button_style.display, Some(DisplayMode::InlineFlex));
    assert_eq!(
        button_style.background_color,
        Some(StyleColor::Rgba {
            red: 0x00,
            green: 0x00,
            blue: 0x00,
            alpha: 255,
        })
    );
}

#[test]
fn rsx_ui_router_components_render_active_route_and_navigation_actions() {
    #[allow(non_snake_case)]
    fn router_demo(cx: &mut ComponentCx<RouterUiState>) -> RSX {
        let currentPath = cx.use_state("currentPath", |state: &RouterUiState| {
            if state.path.is_empty() {
                "/".to_string()
            } else {
                state.path.clone()
            }
        });
        let homeActive = cx.use_state("homeActive", |state: &RouterUiState| {
            state.path.is_empty() || state.path == "/"
        });
        let settingsActive = cx.use_state("settingsActive", |state: &RouterUiState| {
            state.path == "/settings"
        });
        let navigate = cx.use_reducer(
            "navigate",
            |state: &mut RouterUiState, invocation: &ActionInvocation| {
                if let Some(path) = invocation.value() {
                    state.path = path.to_string();
                    state.navigations += 1;
                }
                Ok(())
            },
        );

        crate::rsx!(
            <UiRouter key="router" currentPath={currentPath} className="h-full">
                <UiNavigation key="nav" label="Routes" className="gap-1">
                    <UiNavLink key="home-link" to="/" onNavigate={navigate} isActive={homeActive}>Home</UiNavLink>
                    <UiNavigateButton key="settings-button" to="/settings" onNavigate={navigate} isActive={settingsActive}>Settings</UiNavigateButton>
                </UiNavigation>
                <UiRoutes key="routes" label="Application routes">
                    <UiRoute key="home-route" path="/" label="Home" isActive={homeActive}>
                        <UiText key="home-title" label="Home page" />
                    </UiRoute>
                    <UiRoute key="settings-route" path="/settings" label="Settings" isActive={settingsActive}>
                        <UiText key="settings-title" label="Settings page" />
                    </UiRoute>
                </UiRoutes>
            </UiRouter>
        )
    }

    let component = Rc::new(ComponentCx::compile("router-demo", router_demo).unwrap());
    let mut state = RouterUiState::default();

    let home = component.render(&state).unwrap();
    let router = find_element_by_attribute(&home.root, "data-slot", "router").unwrap();
    assert_eq!(attribute_value(router, "data-current-path"), Some("/"));
    assert!(find_element_by_attribute(&home.root, "data-route-path", "/").is_some());
    assert!(find_element_by_attribute(&home.root, "data-route-path", "/settings").is_none());
    let home_link = find_element_by_attributes(
        &home.root,
        &[("data-slot", "nav-link"), ("data-route-to", "/")],
    )
    .unwrap();
    assert_eq!(attribute_value(home_link, "data-active"), Some("true"));
    assert_eq!(attribute_value(home_link, "actionValue"), Some("/"));
    assert_eq!(event_value(home_link, "onPress"), Some("navigate"));
    let settings_button = find_element_by_attributes(
        &home.root,
        &[
            ("data-slot", "navigate-button"),
            ("data-route-to", "/settings"),
        ],
    )
    .unwrap();
    assert_eq!(
        attribute_value(settings_button, "data-active"),
        Some("false")
    );
    assert_eq!(
        attribute_value(settings_button, "actionValue"),
        Some("/settings")
    );
    assert_eq!(event_value(settings_button, "onPress"), Some("navigate"));

    component
        .reduce(
            &mut state,
            &ActionInvocation {
                node: HostNodeId::new(1),
                current_target: None,
                action: "navigate".to_string(),
                event: NativeEventKind::Press,
                context: Default::default(),
                value: Some("/settings".to_string()),
            },
        )
        .unwrap();

    let settings = component.render(&state).unwrap();
    assert_eq!(state.path, "/settings");
    assert_eq!(state.navigations, 1);
    assert!(find_element_by_attribute(&settings.root, "data-route-path", "/").is_none());
    assert!(find_element_by_attribute(&settings.root, "data-route-path", "/settings").is_some());
}

#[test]
fn rsx_ui_navigate_button_routes_press_through_appkit_blueprint() {
    #[allow(non_snake_case)]
    fn router_demo(cx: &mut ComponentCx<RouterUiState>) -> RSX {
        let settingsActive = cx.use_state("settingsActive", |state: &RouterUiState| {
            state.path == "/settings"
        });
        let navigate = cx.use_reducer(
            "navigate",
            |state: &mut RouterUiState, invocation: &ActionInvocation| {
                if let Some(path) = invocation.value() {
                    state.path = path.to_string();
                    state.navigations += 1;
                }
                Ok(())
            },
        );

        crate::rsx!(
            <UiNavigation key="nav" label="Routes" className="gap-1">
                <UiNavigateButton key="settings-button" to="/settings" onNavigate={navigate} isActive={settingsActive}>Settings</UiNavigateButton>
            </UiNavigation>
        )
    }

    let component = Rc::new(ComponentCx::compile("router-demo", router_demo).unwrap());
    let render_component = component.clone();
    let reduce_component = component.clone();
    let mut app = NativeProtocolApp::new(
        AppKitAdapter,
        RouterUiState::default(),
        move |state| render_component.render(state),
        move |state, invocation| reduce_component.reduce(state, invocation),
    );

    let rendered = app.render().unwrap();
    let settings_button = app
        .session()
        .runtime()
        .host()
        .nodes()
        .iter()
        .find_map(|(id, node)| {
            let metadata = &node.blueprint.metadata;
            (metadata.get("data-route-to").map(String::as_str) == Some("/settings")
                && metadata.get("actionValue").map(String::as_str) == Some("/settings"))
            .then_some(*id)
        })
        .expect("settings navigate button blueprint");

    let response = app
        .dispatch_host_event(&HostEvent {
            frame_id: rendered.frame_id,
            event: NativeEvent::new(settings_button, NativeEventKind::Press),
        })
        .unwrap();

    assert_eq!(response.invocation.as_ref().unwrap().action, "navigate");
    assert_eq!(
        response.invocation.as_ref().unwrap().value.as_deref(),
        Some("/settings")
    );
    assert_eq!(app.state().path, "/settings");
    assert_eq!(app.state().navigations, 1);
}

#[test]
fn rsx_ui_components_are_available_without_manual_registration() {
    let template = RsxTemplate::parse(
        r#"
        <UiButton key="save" variant="outline" size="sm" onPress={saveProfile}>
          Save
        </UiButton>
        "#,
    )
    .unwrap();
    let component = RsxComponent::<FormState>::from_template("button", template)
        .unwrap()
        .use_reducer("saveProfile", |_state, _invocation| Ok(()));

    let frame = component.render(&FormState::default()).unwrap();
    let button = find_element_by_attribute(&frame.root, "data-slot", "button").unwrap();

    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "border");
    assert_class_contains(button, "h-8");
}

#[test]
fn rsx_ui_components_are_single_file_rust_rsx_modules() {
    let button_source = include_str!("components/button/button.rsx");
    let component_index = include_str!("components/catalog.rs");

    assert!(button_source.contains("pub struct UiButtonProps"));
    assert!(button_source.contains("pub fn ui_button(cx: &mut ComponentCx<UiButtonProps>) -> RSX"));
    assert!(button_source.contains("crate::rsx!("));
    assert!(component_index.contains("#[path = \"button/button.rsx\"]"));
    assert!(!component_index.contains(concat!("register", "_ui_")));
    assert!(!component_index.contains(concat!("install", "_ui_")));
    assert!(!component_index.contains(concat!("install", "_default_components")));
}

#[test]
fn rsx_ui_component_defaults_use_design_tokens() {
    let manifest_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let mut source_paths = vec![
        manifest_dir.join("src/rsx_ui/classes.rs"),
        manifest_dir.join("src/rsx_ui/variants.rs"),
    ];
    collect_rsx_component_sources(
        &manifest_dir.join("src/rsx_ui/components"),
        &mut source_paths,
    );

    let forbidden_tokens = [
        "bg-accent",
        "bg-background",
        "bg-card",
        "bg-popover",
        "bg-secondary",
        "bg-muted",
        "bg-border",
        "bg-input",
        "bg-destructive",
        "bg-surface-pressed",
        "bg-black-elevated",
        "text-foreground",
        "text-muted-foreground",
        "text-accent-foreground",
        "text-card-foreground",
        "text-popover-foreground",
        "text-primary-foreground",
        "text-destructive",
        "text-destructive-foreground",
        "text-mute",
        "border-input",
        "border-border",
        "border-background",
        "border-destructive",
        "border-ring",
        "outline-ring",
        "caret-ring",
        "ring-ring",
        "ring-border",
        "ring-destructive",
        "link-blue",
        "link-deep",
        "link-soft",
        "sidebar-border",
        "bg-sidebar",
    ];

    let mut violations = Vec::new();
    for path in source_paths {
        let source = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
        for token in forbidden_tokens {
            if source_contains_class_token(&source, token) {
                violations.push(format!("{} contains `{token}`", path.display()));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "RSX UI component defaults must use DESIGN.md tokens:\n{}",
        violations.join("\n")
    );
}

fn source_contains_class_token(source: &str, token: &str) -> bool {
    source.match_indices(token).any(|(index, _)| {
        let before = source[..index].chars().next_back();
        let after = source[index + token.len()..].chars().next();

        before.map_or(true, |ch| !is_class_name_char(ch))
            && after.map_or(true, |ch| !is_class_name_char(ch))
    })
}

fn is_class_name_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_')
}

fn collect_rsx_component_sources(path: &std::path::Path, sources: &mut Vec<std::path::PathBuf>) {
    let mut entries = std::fs::read_dir(path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_else(|error| panic!("read {} entry: {error}", path.display()));
    entries.sort();

    for entry in entries {
        if entry.is_dir() {
            collect_rsx_component_sources(&entry, sources);
        } else if entry
            .extension()
            .is_some_and(|extension| extension == "rsx")
        {
            sources.push(entry);
        }
    }
}

#[test]
fn rsx_ui_tabs_render_design_classes_and_native_tab_semantics() {
    let component = RsxComponent::new(
        "tabs",
        r#"
        <UiTabs
          key="settings"
          value={state.tab}
          onSelectionChange={setTab}
          className="w-full"
        >
          <UiTabsList key="list" className="grid w-full grid-cols-2">
            <UiTabsTrigger
              key="profile-trigger"
              value="profile"
              isSelected={state.profileSelected}
            >
              Profile
            </UiTabsTrigger>
            <UiTabsTrigger
              key="billing-trigger"
              value="billing"
              isSelected={state.billingSelected}
            >
              Billing
            </UiTabsTrigger>
          </UiTabsList>
          <UiTabsContent key="profile-panel" value="profile">
            <Text key="profile-copy">Profile settings</Text>
          </UiTabsContent>
          <UiTabsContent key="billing-panel" value="billing">
            <Text key="billing-copy">Billing settings</Text>
          </UiTabsContent>
        </UiTabs>
        "#,
    )
    .unwrap()
    .use_state("tab", |state: &FormState| state.tab.clone())
    .use_state("profileSelected", |state: &FormState| {
        state.tab == "profile"
    })
    .use_state("billingSelected", |state: &FormState| {
        state.tab == "billing"
    })
    .use_value_reducer("setTab", |state: &mut FormState, tab: String| {
        state.tab = tab;
        Ok(())
    });

    let frame = component
        .render(&FormState {
            email: String::new(),
            tab: "profile".to_string(),
            saved: false,
        })
        .unwrap();

    let tabs = find_element_by_attribute(&frame.root, "data-slot", "tabs").unwrap();
    let list = find_element_by_attribute(&frame.root, "data-slot", "tabs-list").unwrap();
    let selected = find_element_by_attribute(&frame.root, "data-selected", "true").unwrap();
    let content = find_element_by_attribute(&frame.root, "data-slot", "tabs-content").unwrap();

    assert_class_contains(tabs, "flex");
    assert_class_contains(tabs, "w-full");
    assert_class_contains(list, "bg-canvas-soft");
    assert_class_contains(list, "grid-cols-2");
    assert_class_contains(selected, "data-[selected=true]:bg-canvas");
    assert_class_contains(selected, "rounded-sm");
    assert_class_contains(content, "outline-none");

    let native = RsxCompilerBridge::new()
        .lower_to_native(&frame.root)
        .unwrap();
    assert_eq!(native.role, NativeRole::Tabs);
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setTab")
    );
    assert_eq!(native.children.len(), 2);
    assert_eq!(native.children[0].role, NativeRole::Tab);
    assert_eq!(native.children[0].props.value.as_deref(), Some("profile"));
    assert!(native.children[0].props.selected);
    assert_eq!(native.children[0].children[0].role, NativeRole::TabPanel);
}

#[test]
fn rsx_ui_tab_parts_use_tab_hook_props() {
    let component = RsxComponent::new(
        "hooked-tabs",
        r#"
        <UiTabs key="tabs" value="profile">
          <UiTabsList
            key="list"
            label="Settings"
            orientation="vertical"
            isDisabled={state.disabled}
          >
            <UiTabsTrigger
              key="trigger"
              value="profile"
              isSelected={state.selected}
              isDisabled={state.disabled}
            >
              Profile
            </UiTabsTrigger>
          </UiTabsList>
          <UiTabsContent key="panel" value="profile">
            Profile settings
          </UiTabsContent>
        </UiTabs>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &TabUiState| state.selected)
    .use_state("disabled", |state: &TabUiState| state.disabled);

    let frame = component
        .render(&TabUiState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let list = find_element_by_attribute(&frame.root, "data-slot", "tabs-list").unwrap();
    let tab = find_element_by_attribute(&frame.root, "data-slot", "tabs-trigger").unwrap();
    let panel = find_element_by_attribute(&frame.root, "data-slot", "tabs-content").unwrap();
    let CompiledRsxNode::Element {
        props: list_props, ..
    } = list
    else {
        panic!("tab list element");
    };
    let CompiledRsxNode::Element { props, .. } = tab else {
        panic!("tab trigger element");
    };
    let CompiledRsxNode::Element {
        props: panel_props, ..
    } = panel
    else {
        panic!("tab panel element");
    };

    assert_eq!(list_props.label.as_deref(), Some("Settings"));
    assert_eq!(list_props.aria_label.as_deref(), Some("Settings"));
    assert!(list_props.is_disabled);
    assert_eq!(attribute_value(list, "role"), Some("tablist"));
    assert_eq!(attribute_value(list, "aria-orientation"), Some("vertical"));
    assert_eq!(attribute_value(list, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(list, "data-disabled"), Some("true"));
    assert_eq!(props.value.as_deref(), Some("profile"));
    assert!(props.is_selected);
    assert!(props.is_disabled);
    assert_eq!(attribute_value(tab, "role"), Some("tab"));
    assert_eq!(attribute_value(tab, "aria-selected"), Some("true"));
    assert_eq!(attribute_value(tab, "data-selected"), Some("true"));
    assert_eq!(attribute_value(tab, "aria-disabled"), Some("true"));
    assert_eq!(panel_props.value.as_deref(), Some("profile"));
    assert_eq!(attribute_value(panel, "role"), Some("tabpanel"));

    let bridge = RsxCompilerBridge::new();
    let list_native = bridge.lower_to_native(list).unwrap();
    assert_eq!(list_native.role, NativeRole::TabList);
    assert_eq!(list_native.props.label.as_deref(), Some("Settings"));
    assert!(list_native.props.disabled);

    let tab_native = bridge.lower_to_native(tab).unwrap();
    assert_eq!(tab_native.role, NativeRole::Tab);
    assert_eq!(tab_native.props.value.as_deref(), Some("profile"));
    assert!(tab_native.props.selected);
    assert!(tab_native.props.disabled);

    let panel_native = bridge.lower_to_native(panel).unwrap();
    assert_eq!(panel_native.role, NativeRole::TabPanel);
    assert_eq!(panel_native.props.value.as_deref(), Some("profile"));
}

#[test]
fn rsx_ui_renders_collection_interaction_and_structure_parts() {
    run_rsx_stress_test(|| {
        let component = RsxComponent::new(
            "collection-interaction-parts",
            r#"
        <UiGroup key="root" label="Interaction parts">
          <UiTabs
            key="tabs"
            value={state.tab}
            onSelectionChange={setTab}
          >
            <UiTabsList key="tab-list">
              <UiTabsTrigger
                key="profile-tab"
                value="profile"
                isSelected={state.profileSelected}
              >
                Profile
              </UiTabsTrigger>
            </UiTabsList>
            <UiTabsContent key="profile-panel" value="profile">
              <UiText key="profile-copy">Profile panel</UiText>
            </UiTabsContent>
          </UiTabs>
          <UiTextArea
            key="notes"
            value={state.email}
            placeholder="Notes"
            onChange={setEmail}
          />
          <UiComboBoxValue
            key="combo-value"
            value={state.email}
            placeholder="Assignee"
          />
          <UiCollection key="collection" label="Rows">
            <UiDropIndicator
              key="drop-indicator"
              orientation="horizontal"
              isTarget={true}
            />
          </UiCollection>
          <UiPressable
            key="pressable"
            onPress={pressAlias}
            isPressed={true}
            actionValue="pressable"
          >
            Pressable
          </UiPressable>
          <UiHoverable
            key="hoverable"
            onHoverStart={hoverAlias}
            onHoverEnd={hoverEndAlias}
            isHovered={true}
          >
            Hoverable
          </UiHoverable>
          <UiKeyboardTarget
            key="keyboard-target"
            onKeyDown={keyDownAlias}
            onKeyUp={keyUpAlias}
            isKeyboardActive={true}
            tabIndex={5}
          >
            Keyboard
          </UiKeyboardTarget>
          <UiLongPressable
            key="long-pressable"
            onLongPress={longPressAlias}
            isPressed={true}
            isLongPressed={true}
            actionValue="long-press"
            accessibilityDescription="Hold to open actions"
            threshold={720}
          >
            Long press
          </UiLongPressable>
          <UiMovable
            key="movable"
            onMoveStart={moveStartAlias}
            onMove={moveAlias}
            onMoveEnd={moveEndAlias}
            isMoving={true}
            xDelta={4}
            yDelta={2}
          >
            Move
          </UiMovable>
          <UiDraggable
            key="draggable"
            onDragStart={dragStartAlias}
            onDragEnd={dragEndAlias}
            dragType="text/plain"
            dragValue="profile"
            isDragging={true}
          >
            Drag
          </UiDraggable>
          <UiDroppable
            key="droppable"
            label="Drop profile"
            onDrop={dropAlias}
            onDropEnter={dropEnterAlias}
            acceptedDragTypes="text/plain"
            dropOperation="move"
            isDropTarget={true}
          >
            Drop
          </UiDroppable>
          <UiFocusable
            key="focusable"
            autoFocus={true}
            isFocused={state.profileSelected}
            tabIndex={2}
            onFocusChange={setFocus}
          >
            Focusable
          </UiFocusable>
          <UiFocusRing
            key="focus-ring"
            isFocused={state.profileSelected}
            isFocusVisible={true}
            within={true}
            isFocusWithin={true}
            tabIndex={3}
            onFocusChange={setFocus}
          >
            Focus ring
          </UiFocusRing>
          <UiFocusWithin
            key="focus-within"
            isFocusWithin={true}
            onFocusWithin={setFocus}
            onBlurWithin={setFocus}
            onFocusWithinChange={setFocus}
          >
            Focus within
          </UiFocusWithin>
          <UiFocusScope
            key="focus-scope"
            contain={true}
            restoreFocus={true}
            autoFocus={true}
            tabIndex={4}
          >
            Focus scope
          </UiFocusScope>
          <UiVisuallyHidden
            key="visually-hidden"
            label="Screen reader only"
            textValue="Screen reader only"
          />
          <UiSharedElementTransition
            key="shared-transition"
            id="route-card"
            isTransitioning={true}
          >
            <UiSharedElement key="shared-element" id="route-card">
              Shared
            </UiSharedElement>
          </UiSharedElementTransition>
          <UiTabPanels key="tab-panels">
            <UiTabsContent key="settings-panel" value="settings">
              Settings
            </UiTabsContent>
          </UiTabPanels>
          <UiTable key="parts-table" label="Parts">
            <UiTableHeader key="parts-head">
              <UiTableRow key="parts-head-row">
                <UiTableColumn key="parts-column" textValue="Name">Name</UiTableColumn>
              </UiTableRow>
            </UiTableHeader>
            <UiTableBody key="parts-body">
              <UiTableRow key="parts-row" isSelected={true}>
                <UiTableCell key="parts-cell" textValue="Ada">Ada</UiTableCell>
              </UiTableRow>
            </UiTableBody>
          </UiTable>
          <UiTree key="parts-tree" label="Parts tree">
            <UiTreeLoadMoreItem
              key="tree-load-more"
              label="Load more"
              onPress={loadMore}
              isLoading={true}
              actionValue="tree"
            />
          </UiTree>
          <UiColorWheelTrack key="wheel-track" label="Hue track">
            <UiColorThumb key="wheel-thumb" value="black" />
          </UiColorWheelTrack>
          <UiTagList key="tag-list" label="Tags" value={state.tab}>
            <UiTag key="tag" value="profile" textValue="Profile">
              Profile
            </UiTag>
          </UiTagList>
        </UiGroup>
        "#,
        )
        .unwrap()
        .use_state("email", |state: &FormState| state.email.clone())
        .use_state("tab", |state: &FormState| state.tab.clone())
        .use_state("profileSelected", |state: &FormState| {
            state.tab == "profile"
        })
        .use_value_reducer("setEmail", |state: &mut FormState, email: String| {
            state.email = email;
            Ok(())
        })
        .use_value_reducer("setTab", |state: &mut FormState, tab: String| {
            state.tab = tab;
            Ok(())
        })
        .use_value_reducer("setFocus", |state: &mut FormState, focused: bool| {
            state.saved = focused;
            Ok(())
        })
        .use_reducer("pressAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("hoverAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer(
            "hoverEndAlias",
            |_state: &mut FormState, _invocation| Ok(()),
        )
        .use_reducer("keyDownAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("keyUpAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("longPressAlias", |_state: &mut FormState, _invocation| {
            Ok(())
        })
        .use_reducer("moveStartAlias", |_state: &mut FormState, _invocation| {
            Ok(())
        })
        .use_reducer("moveAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("moveEndAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("dragStartAlias", |_state: &mut FormState, _invocation| {
            Ok(())
        })
        .use_reducer("dragEndAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("dropAlias", |_state: &mut FormState, _invocation| Ok(()))
        .use_reducer("dropEnterAlias", |_state: &mut FormState, _invocation| {
            Ok(())
        })
        .use_reducer("loadMore", |_state: &mut FormState, _invocation| Ok(()));

        let frame = component
            .render(&FormState {
                email: "grace@example.com".to_string(),
                tab: "profile".to_string(),
                saved: false,
            })
            .unwrap();

        let bridge = RsxCompilerBridge::new();
        let tab_list = find_element_by_attribute(&frame.root, "data-slot", "tabs-list").unwrap();
        let tab = find_element_by_attribute(&frame.root, "data-slot", "tabs-trigger").unwrap();
        let tab_panel =
            find_element_by_attribute(&frame.root, "data-slot", "tabs-content").unwrap();
        let text_area = find_element_by_attribute(&frame.root, "data-slot", "textarea").unwrap();
        let combo_value =
            find_element_by_attribute(&frame.root, "data-slot", "combo-box-value").unwrap();
        let collection = find_element_by_attribute(&frame.root, "data-slot", "collection").unwrap();
        let drop_indicator =
            find_element_by_attribute(&frame.root, "data-slot", "drop-indicator").unwrap();
        let pressable = find_element_by_attribute(&frame.root, "data-slot", "pressable").unwrap();
        let hoverable = find_element_by_attribute(&frame.root, "data-slot", "hoverable").unwrap();
        let keyboard_target =
            find_element_by_attribute(&frame.root, "data-slot", "keyboard-target").unwrap();
        let long_pressable =
            find_element_by_attribute(&frame.root, "data-slot", "long-pressable").unwrap();
        let movable = find_element_by_attribute(&frame.root, "data-slot", "movable").unwrap();
        let draggable = find_element_by_attribute(&frame.root, "data-slot", "draggable").unwrap();
        let droppable = find_element_by_attribute(&frame.root, "data-slot", "droppable").unwrap();
        let focusable = find_element_by_attribute(&frame.root, "data-slot", "focusable").unwrap();
        let focus_ring = find_element_by_attribute(&frame.root, "data-slot", "focus-ring").unwrap();
        let focus_within =
            find_element_by_attribute(&frame.root, "data-slot", "focus-within").unwrap();
        let focus_scope =
            find_element_by_attribute(&frame.root, "data-slot", "focus-scope").unwrap();
        let visually_hidden =
            find_element_by_attribute(&frame.root, "data-slot", "visually-hidden").unwrap();
        let shared_transition =
            find_element_by_attribute(&frame.root, "data-slot", "shared-element-transition")
                .unwrap();
        let shared_element =
            find_element_by_attribute(&frame.root, "data-slot", "shared-element").unwrap();
        let tab_panels = find_element_by_attribute(&frame.root, "data-slot", "tab-panels").unwrap();
        let table_row = find_element_by_attributes(
            &frame.root,
            &[("data-slot", "table-row"), ("data-selected", "true")],
        )
        .unwrap();
        let table_column =
            find_element_by_attribute(&frame.root, "data-slot", "table-column").unwrap();
        let table_cell = find_element_by_attribute(&frame.root, "data-slot", "table-cell").unwrap();
        let tree_load_more =
            find_element_by_attribute(&frame.root, "data-slot", "tree-load-more-item").unwrap();
        let wheel_track =
            find_element_by_attribute(&frame.root, "data-slot", "color-wheel-track").unwrap();
        let tag_list = find_element_by_attribute(&frame.root, "data-slot", "tag-list").unwrap();

        assert_class_contains(drop_indicator, "data-[target=true]:opacity-100");
        assert_class_contains(pressable, "focus-visible:ring-[2px]");
        assert_class_contains(hoverable, "data-[hovered=true]:bg-canvas-soft");
        assert_class_contains(keyboard_target, "data-[keyboard-active=true]:ring-[2px]");
        assert_class_contains(long_pressable, "data-[long-pressed=true]:bg-canvas-soft");
        assert_class_contains(movable, "data-[moving=true]:cursor-grabbing");
        assert_class_contains(draggable, "data-[dragging=true]:opacity-70");
        assert_class_contains(droppable, "data-[drop-target=true]:ring-[2px]");
        assert_class_contains(focusable, "focus-visible:ring-[2px]");
        assert_class_contains(focus_ring, "data-[focus-visible=true]:ring-[2px]");
        assert_class_contains(focus_within, "data-[focus-within=true]:ring-[2px]");
        assert_class_contains(focus_scope, "data-[contain=true]:isolate");
        assert_class_contains(visually_hidden, "sr-only");
        assert_class_contains(shared_transition, "contents");
        assert_class_contains(shared_element, "contents");
        assert_class_contains(tab_panels, "grid");
        assert_class_contains(tree_load_more, "border-dashed");
        assert_class_contains(wheel_track, "rounded-full");
        assert_class_contains(text_area, "bg-surface-card");

        assert_eq!(attribute_value(pressable, "data-pressed"), Some("true"));
        assert_eq!(attribute_value(hoverable, "data-hovered"), Some("true"));
        assert_eq!(
            attribute_value(keyboard_target, "data-keyboard-active"),
            Some("true")
        );
        assert_eq!(attribute_value(keyboard_target, "tabIndex"), Some("5"));
        assert_eq!(
            attribute_value(long_pressable, "data-long-pressed"),
            Some("true")
        );
        assert_eq!(
            attribute_value(long_pressable, "actionValue"),
            Some("long-press")
        );
        assert_eq!(attribute_value(movable, "data-moving"), Some("true"));
        assert_eq!(attribute_value(movable, "data-x-delta"), Some("4.0"));
        assert_eq!(attribute_value(movable, "data-y-delta"), Some("2.0"));
        assert_eq!(attribute_value(draggable, "draggable"), Some("true"));
        assert_eq!(attribute_value(draggable, "data-dragging"), Some("true"));
        assert_eq!(
            attribute_value(draggable, "data-drag-type"),
            Some("text/plain")
        );
        assert_eq!(
            attribute_value(draggable, "data-drag-value"),
            Some("profile")
        );
        assert_eq!(attribute_value(droppable, "data-drop-target"), Some("true"));
        assert_eq!(
            attribute_value(droppable, "data-accepted-drag-types"),
            Some("text/plain")
        );
        assert_eq!(
            attribute_value(droppable, "data-drop-operation"),
            Some("move")
        );
        assert_eq!(attribute_value(focusable, "data-focused"), Some("true"));
        assert_eq!(attribute_value(focusable, "tabIndex"), Some("2"));
        assert_eq!(attribute_value(focusable, "autoFocus"), Some("true"));
        assert_eq!(attribute_value(focus_ring, "data-focused"), Some("true"));
        assert_eq!(
            attribute_value(focus_ring, "data-focus-visible"),
            Some("true")
        );
        assert_eq!(
            attribute_value(focus_ring, "data-focus-within"),
            Some("true")
        );
        assert_eq!(attribute_value(focus_ring, "tabIndex"), Some("3"));
        assert_eq!(
            attribute_value(focus_scope, "data-focus-scope"),
            Some("true")
        );
        assert_eq!(attribute_value(focus_scope, "data-contain"), Some("true"));
        assert_eq!(
            attribute_value(focus_scope, "data-restore-focus"),
            Some("true")
        );
        assert_eq!(attribute_value(focus_scope, "tabIndex"), Some("4"));
        assert_eq!(
            attribute_value(shared_transition, "data-shared-element-id"),
            Some("route-card")
        );
        assert_eq!(
            attribute_value(shared_transition, "data-transitioning"),
            Some("true")
        );
        assert_eq!(
            attribute_value(shared_element, "data-shared-element-id"),
            Some("route-card")
        );
        assert_eq!(
            attribute_value(tree_load_more, "data-loading"),
            Some("true")
        );

        assert_eq!(
            bridge.lower_to_native(tab_list).unwrap().role,
            NativeRole::TabList
        );
        assert_eq!(bridge.lower_to_native(tab).unwrap().role, NativeRole::Tab);
        assert_eq!(
            bridge.lower_to_native(tab_panel).unwrap().role,
            NativeRole::TabPanel
        );
        assert_eq!(
            bridge.lower_to_native(text_area).unwrap().role,
            NativeRole::TextField
        );
        assert_eq!(
            bridge.lower_to_native(combo_value).unwrap().role,
            NativeRole::Text
        );
        assert_eq!(
            bridge.lower_to_native(collection).unwrap().role,
            NativeRole::View
        );
        assert_eq!(
            bridge.lower_to_native(drop_indicator).unwrap().role,
            NativeRole::View
        );
        let pressable_native = bridge.lower_to_native(pressable).unwrap();
        assert_eq!(pressable_native.role, NativeRole::View);
        assert_eq!(pressable_native.props.action.as_deref(), Some("pressAlias"));
        assert_eq!(
            pressable_native
                .props
                .metadata
                .get("actionValue")
                .map(String::as_str),
            Some("pressable")
        );
        let hoverable_native = bridge.lower_to_native(hoverable).unwrap();
        assert_eq!(hoverable_native.role, NativeRole::View);
        assert_eq!(
            hoverable_native
                .props
                .web
                .events
                .get("onHoverStart")
                .map(String::as_str),
            Some("hoverAlias")
        );
        let keyboard_target_native = bridge.lower_to_native(keyboard_target).unwrap();
        assert_eq!(keyboard_target_native.role, NativeRole::View);
        assert_eq!(keyboard_target_native.props.tab_index, Some(5));
        assert_eq!(
            keyboard_target_native
                .props
                .web
                .events
                .get("onKeyDown")
                .map(String::as_str),
            Some("keyDownAlias")
        );
        let long_pressable_native = bridge.lower_to_native(long_pressable).unwrap();
        assert_eq!(long_pressable_native.role, NativeRole::View);
        assert_eq!(
            long_pressable_native
                .props
                .metadata
                .get("actionValue")
                .map(String::as_str),
            Some("long-press")
        );
        assert_eq!(
            long_pressable_native
                .props
                .metadata
                .get("threshold")
                .map(String::as_str),
            Some("720")
        );
        assert_eq!(
            long_pressable_native
                .props
                .accessibility_description
                .description
                .as_deref(),
            Some("Hold to open actions")
        );
        assert_eq!(
            long_pressable_native
                .props
                .web
                .events
                .get("onLongPress")
                .map(String::as_str),
            Some("longPressAlias")
        );
        let movable_native = bridge.lower_to_native(movable).unwrap();
        assert_eq!(movable_native.role, NativeRole::View);
        assert_eq!(
            movable_native
                .props
                .web
                .events
                .get("onMoveStart")
                .map(String::as_str),
            Some("moveStartAlias")
        );
        let draggable_native = bridge.lower_to_native(draggable).unwrap();
        assert_eq!(draggable_native.role, NativeRole::View);
        assert_eq!(draggable_native.props.draggable.as_deref(), Some("true"));
        assert_eq!(
            draggable_native
                .props
                .web
                .events
                .get("onDragStart")
                .map(String::as_str),
            Some("dragStartAlias")
        );
        let droppable_native = bridge.lower_to_native(droppable).unwrap();
        assert_eq!(droppable_native.role, NativeRole::View);
        assert_eq!(
            droppable_native.props.label.as_deref(),
            Some("Drop profile")
        );
        assert_eq!(
            droppable_native
                .props
                .web
                .events
                .get("onDrop")
                .map(String::as_str),
            Some("dropAlias")
        );
        let focusable_native = bridge.lower_to_native(focusable).unwrap();
        assert_eq!(focusable_native.role, NativeRole::View);
        assert!(focusable_native.props.auto_focus);
        assert_eq!(focusable_native.props.tab_index, Some(2));
        assert_eq!(
            focusable_native
                .props
                .web
                .events
                .get("onFocusChange")
                .map(String::as_str),
            Some("setFocus")
        );
        let focus_ring_native = bridge.lower_to_native(focus_ring).unwrap();
        assert_eq!(focus_ring_native.role, NativeRole::View);
        assert_eq!(focus_ring_native.props.tab_index, Some(3));
        assert_eq!(
            focus_ring_native
                .props
                .web
                .events
                .get("onFocusChange")
                .map(String::as_str),
            Some("setFocus")
        );
        let focus_within_native = bridge.lower_to_native(focus_within).unwrap();
        assert_eq!(focus_within_native.role, NativeRole::View);
        assert_eq!(
            focus_within_native
                .props
                .web
                .events
                .get("onFocusWithin")
                .map(String::as_str),
            Some("setFocus")
        );
        assert_eq!(
            focus_within_native
                .props
                .web
                .events
                .get("onBlurWithin")
                .map(String::as_str),
            Some("setFocus")
        );
        let focus_scope_native = bridge.lower_to_native(focus_scope).unwrap();
        assert_eq!(focus_scope_native.role, NativeRole::View);
        assert!(!focus_scope_native.props.auto_focus);
        assert_eq!(
            focus_scope_native
                .props
                .web
                .attributes
                .get("data-auto-focus")
                .map(String::as_str),
            Some("true")
        );
        assert_eq!(focus_scope_native.props.tab_index, Some(4));
        assert_eq!(
            bridge.lower_to_native(visually_hidden).unwrap().role,
            NativeRole::Text
        );
        assert_eq!(
            bridge.lower_to_native(shared_transition).unwrap().role,
            NativeRole::View
        );
        assert_eq!(
            bridge.lower_to_native(shared_element).unwrap().role,
            NativeRole::View
        );
        assert_eq!(
            bridge.lower_to_native(tab_panels).unwrap().role,
            NativeRole::View
        );
        assert_eq!(
            bridge.lower_to_native(table_row).unwrap().role,
            NativeRole::TableRow
        );
        assert_eq!(
            bridge.lower_to_native(table_column).unwrap().role,
            NativeRole::TableColumn
        );
        assert_eq!(
            bridge.lower_to_native(table_cell).unwrap().role,
            NativeRole::TableCell
        );
        let tree_load_more_native = bridge.lower_to_native(tree_load_more).unwrap();
        assert_eq!(tree_load_more_native.role, NativeRole::TreeItem);
        assert_eq!(
            tree_load_more_native.props.action.as_deref(),
            Some("loadMore")
        );
        assert_eq!(
            bridge.lower_to_native(wheel_track).unwrap().role,
            NativeRole::View
        );
        assert_eq!(
            bridge.lower_to_native(tag_list).unwrap().role,
            NativeRole::ListBox
        );
    });
}

#[test]
fn rsx_ui_focus_ring_descendant_styles_are_opt_in() {
    let direct = RsxComponent::<FormState>::new(
        "focus-ring-direct",
        r#"<UiFocusRing key="ring">Direct focus</UiFocusRing>"#,
    )
    .unwrap()
    .render(&FormState::default())
    .unwrap();
    assert_eq!(
        attribute_value(&direct.root, "data-focus-ring-within"),
        Some("false")
    );
    assert_class_contains(
        &direct.root,
        "data-[focus-ring-within=true]:data-[focus-visible-within=true]:ring-[2px]",
    );

    let within = RsxComponent::<FormState>::new(
        "focus-ring-within",
        r#"<UiFocusRing key="ring" within={true}>Descendant focus</UiFocusRing>"#,
    )
    .unwrap()
    .render(&FormState::default())
    .unwrap();
    assert_eq!(
        attribute_value(&within.root, "data-focus-ring-within"),
        Some("true")
    );
    assert_class_contains(
        &within.root,
        "data-[focus-ring-within=true]:data-[focus-visible-within=true]:ring-[2px]",
    );
}

#[test]
fn rsx_ui_i18n_provider_derives_native_locale_direction() {
    let component = RsxComponent::<FormState>::new(
        "i18n-provider",
        r#"
        <UiI18nProvider key="root" locale="ar-EG" className="contents">
          <UiText key="label" textValue="مرحبا">مرحبا</UiText>
        </UiI18nProvider>
        "#,
    )
    .unwrap();

    let frame = component.render(&FormState::default()).unwrap();
    let provider = find_element_by_attribute(&frame.root, "data-slot", "i18n-provider").unwrap();

    assert_class_contains(provider, "contents");
    assert_eq!(attribute_value(provider, "lang"), Some("ar-EG"));
    assert_eq!(attribute_value(provider, "dir"), Some("rtl"));
    assert_eq!(attribute_value(provider, "data-locale"), Some("ar-EG"));
    assert_eq!(attribute_value(provider, "data-rtl"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(provider).unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.lang.as_deref(), Some("ar-EG"));
    assert_eq!(native.props.dir.as_deref(), Some("rtl"));
}

#[test]
fn rsx_ui_list_box_uses_selection_hook_props() {
    let component = RsxComponent::new(
        "selection-list-box",
        r#"
        <UiListBox
          key="items"
          value={state.selected}
          onSelectionChange={setSelected}
          selectionMode="multiple"
          isReadOnly={state.readOnly}
        >
          <UiListBoxItem key="alpha" value="alpha" textValue="Alpha">Alpha</UiListBoxItem>
          <UiListBoxItem key="beta" value="beta" textValue="Beta">Beta</UiListBoxItem>
        </UiListBox>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &SelectionUiState| {
        state.selected.clone()
    })
    .use_state("readOnly", |state: &SelectionUiState| state.read_only)
    .use_reducer(
        "setSelected",
        |_state: &mut SelectionUiState, _invocation| Ok(()),
    );

    let frame = component
        .render(&SelectionUiState {
            selected: "beta".to_string(),
            read_only: true,
        })
        .unwrap();
    let list_box = find_element_by_attribute(&frame.root, "data-slot", "list-box").unwrap();
    let CompiledRsxNode::Element { props, .. } = list_box else {
        panic!("list box element");
    };

    assert_eq!(props.value.as_deref(), Some("beta"));
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("setSelected")
    );
    assert_eq!(
        props
            .attributes
            .get("data-selected-value")
            .map(String::as_str),
        Some("beta")
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

    let native = RsxCompilerBridge::new().lower_to_native(list_box).unwrap();
    assert_eq!(native.role, NativeRole::ListBox);
    assert_eq!(native.props.value.as_deref(), Some("beta"));
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setSelected")
    );
}

#[test]
fn rsx_ui_switch_uses_switch_hook_props() {
    let component = RsxComponent::new(
        "hooked-switch",
        r#"
        <UiSwitch
          key="sync"
          isChecked={state.enabled}
          isDisabled={true}
          isRequired={true}
          isInvalid={true}
          isReadOnly={true}
          onChange={setEnabled}
        >
          Sync
        </UiSwitch>
        "#,
    )
    .unwrap()
    .use_state("enabled", |state: &ToggleUiState| state.enabled)
    .use_reducer("setEnabled", |_state: &mut ToggleUiState, _invocation| {
        Ok(())
    });

    let frame = component.render(&ToggleUiState { enabled: true }).unwrap();
    let switch = find_element_by_attribute(&frame.root, "data-slot", "switch").unwrap();
    let CompiledRsxNode::Element { props, .. } = switch else {
        panic!("switch element");
    };

    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_disabled);
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setEnabled")
    );
    assert_eq!(attribute_value(switch, "role"), Some("switch"));
    assert_eq!(attribute_value(switch, "data-checked"), Some("true"));
    assert_eq!(attribute_value(switch, "data-selected"), Some("true"));
    assert_eq!(attribute_value(switch, "aria-checked"), Some("true"));
    assert_eq!(attribute_value(switch, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(switch, "aria-required"), Some("true"));
    assert_eq!(attribute_value(switch, "aria-invalid"), Some("true"));
    assert_eq!(attribute_value(switch, "aria-readonly"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(switch).unwrap();
    assert_eq!(native.role, NativeRole::Switch);
    assert_eq!(native.props.checked, Some(true));
    assert!(native.props.disabled);
    assert!(native.props.required);
    assert!(native.props.invalid);
    assert!(native.props.read_only);
    assert_eq!(native.props.action.as_deref(), Some("setEnabled"));
}

#[test]
fn rsx_ui_text_field_uses_text_field_hook_props() {
    let component = RsxComponent::new(
        "hooked-text-field",
        r#"
        <UiTextField
          key="email"
          label="Email"
          value={state.value}
          placeholder="name@example.com"
          isRequired={true}
          isInvalid={true}
          onChange={setEmail}
        />
        "#,
    )
    .unwrap()
    .use_state("value", |state: &TextInputUiState| state.value.clone())
    .use_reducer("setEmail", |_state: &mut TextInputUiState, _invocation| {
        Ok(())
    });

    let frame = component
        .render(&TextInputUiState {
            value: "ada@example.com".to_string(),
        })
        .unwrap();
    let text_field = find_element_by_attribute(&frame.root, "data-slot", "text-field").unwrap();
    let input = find_element_by_attribute(&frame.root, "data-slot", "text-field-input").unwrap();
    let CompiledRsxNode::Element { props, .. } = input else {
        panic!("input element");
    };

    assert_eq!(props.value.as_deref(), Some("ada@example.com"));
    assert_eq!(props.placeholder.as_deref(), Some("name@example.com"));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert_eq!(
        props.events.get("onInput").map(String::as_str),
        Some("setEmail")
    );
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setEmail")
    );

    let native = RsxCompilerBridge::new()
        .lower_to_native(text_field)
        .unwrap();
    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.label.as_deref(), Some("Email"));
    assert_eq!(native.props.value.as_deref(), Some("ada@example.com"));
    assert_eq!(
        native.props.placeholder.as_deref(),
        Some("name@example.com")
    );
    assert!(native.props.required);
    assert!(native.props.invalid);
    assert_eq!(native.props.action.as_deref(), Some("setEmail"));
}

#[test]
fn rsx_ui_checkbox_group_uses_checkbox_group_hook_props() {
    let component = RsxComponent::new(
        "hooked-checkbox-group",
        r#"
        <UiCheckboxGroup
          key="channels"
          label="Channels"
          value="email"
          onChange={setChannels}
          isDisabled={state.disabled}
          isRequired={true}
          isInvalid={state.invalid}
          isReadOnly={true}
        >
          <UiCheckbox key="email" isChecked={true}>Email</UiCheckbox>
        </UiCheckboxGroup>
        "#,
    )
    .unwrap()
    .use_state("disabled", |state: &FieldUiState| state.disabled)
    .use_state("invalid", |state: &FieldUiState| state.invalid)
    .use_value_reducer(
        "setChannels",
        |_state: &mut FieldUiState, _value: String| Ok(()),
    );

    let frame = component
        .render(&FieldUiState {
            disabled: true,
            invalid: true,
        })
        .unwrap();
    let checkbox_group =
        find_element_by_attribute(&frame.root, "data-slot", "checkbox-group").unwrap();
    let CompiledRsxNode::Element { props, .. } = checkbox_group else {
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
    assert_eq!(attribute_value(checkbox_group, "role"), Some("group"));
    assert_eq!(
        attribute_value(checkbox_group, "aria-disabled"),
        Some("true")
    );
    assert_eq!(
        attribute_value(checkbox_group, "aria-required"),
        Some("true")
    );
    assert_eq!(
        attribute_value(checkbox_group, "aria-invalid"),
        Some("true")
    );
    assert_eq!(
        attribute_value(checkbox_group, "aria-readonly"),
        Some("true")
    );
    assert_eq!(
        attribute_value(checkbox_group, "data-selected-value"),
        Some("email")
    );

    let native = RsxCompilerBridge::new()
        .lower_to_native(checkbox_group)
        .unwrap();
    assert_eq!(native.role, NativeRole::FieldSet);
    assert_eq!(native.props.label.as_deref(), Some("Channels"));
    assert!(native.props.disabled);
    assert!(native.props.required);
    assert!(native.props.invalid);
    assert!(native.props.read_only);
}

#[test]
fn rsx_ui_checkbox_uses_checkbox_hook_props() {
    let component = RsxComponent::new(
        "hooked-checkbox",
        r#"
        <UiCheckbox
          key="accepted"
          value="accepted"
          isChecked={true}
          isDisabled={state.disabled}
          isRequired={true}
          isInvalid={state.invalid}
          isReadOnly={true}
          onChange={setAccepted}
        >
          Accepted
        </UiCheckbox>
        "#,
    )
    .unwrap()
    .use_state("disabled", |state: &FieldUiState| state.disabled)
    .use_state("invalid", |state: &FieldUiState| state.invalid)
    .use_value_reducer("setAccepted", |_state: &mut FieldUiState, _value: bool| {
        Ok(())
    });

    let frame = component
        .render(&FieldUiState {
            disabled: true,
            invalid: true,
        })
        .unwrap();
    let checkbox = find_element_by_attribute(&frame.root, "data-slot", "checkbox").unwrap();
    let CompiledRsxNode::Element { props, .. } = checkbox else {
        panic!("checkbox element");
    };

    assert_eq!(props.value.as_deref(), Some("accepted"));
    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_disabled);
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setAccepted")
    );
    assert_eq!(attribute_value(checkbox, "role"), Some("checkbox"));
    assert_eq!(attribute_value(checkbox, "aria-checked"), Some("true"));
    assert_eq!(attribute_value(checkbox, "data-checked"), Some("true"));
    assert_eq!(attribute_value(checkbox, "data-selected"), Some("true"));
    assert_eq!(attribute_value(checkbox, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(checkbox, "aria-required"), Some("true"));
    assert_eq!(attribute_value(checkbox, "aria-invalid"), Some("true"));
    assert_eq!(attribute_value(checkbox, "aria-readonly"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(checkbox).unwrap();
    assert_eq!(native.role, NativeRole::Checkbox);
    assert_eq!(native.props.checked, Some(true));
    assert_eq!(native.props.value.as_deref(), Some("accepted"));
    assert_eq!(native.props.action.as_deref(), Some("setAccepted"));
    assert!(native.props.disabled);
    assert!(native.props.required);
    assert!(native.props.invalid);
    assert!(native.props.read_only);
}

#[test]
fn rsx_ui_disclosure_uses_disclosure_hook_props() {
    let component = RsxComponent::new(
        "hooked-disclosure",
        r#"
        <UiDisclosure
          key="advanced"
          label="Advanced"
          isExpanded={state.expanded}
          onExpandedChange={toggleAdvanced}
        >
          <UiDisclosureSummary
            key="advanced-summary"
            isExpanded={state.expanded}
            onPress={toggleAdvanced}
          >
            Advanced
          </UiDisclosureSummary>
          <UiDisclosurePanel
            key="advanced-panel"
            label="Advanced panel"
            isExpanded={state.expanded}
          >
            Extra options
          </UiDisclosurePanel>
        </UiDisclosure>
        "#,
    )
    .unwrap()
    .use_state("expanded", |state: &DisclosureUiState| state.expanded)
    .use_reducer(
        "toggleAdvanced",
        |_state: &mut DisclosureUiState, _invocation| Ok(()),
    );

    let frame = component
        .render(&DisclosureUiState { expanded: true })
        .unwrap();
    let disclosure = find_element_by_attribute(&frame.root, "data-slot", "disclosure").unwrap();
    let summary =
        find_element_by_attribute(&frame.root, "data-slot", "disclosure-summary").unwrap();
    let panel = find_element_by_attribute(&frame.root, "data-slot", "disclosure-panel").unwrap();
    let CompiledRsxNode::Element { props, .. } = disclosure else {
        panic!("disclosure element");
    };

    assert_eq!(props.label.as_deref(), Some("Advanced"));
    assert_eq!(props.is_expanded, Some(true));
    assert_eq!(
        props.events.get("onExpandedChange").map(String::as_str),
        Some("toggleAdvanced")
    );
    assert_eq!(attribute_value(disclosure, "data-expanded"), Some("true"));
    assert_eq!(attribute_value(summary, "aria-expanded"), Some("true"));
    assert_eq!(attribute_value(summary, "data-expanded"), Some("true"));
    assert_eq!(attribute_value(summary, "role"), Some("button"));
    assert_eq!(attribute_value(panel, "data-expanded"), Some("true"));
    assert_eq!(attribute_value(panel, "hidden"), None);

    let native = RsxCompilerBridge::new()
        .lower_to_native(disclosure)
        .unwrap();
    assert_eq!(native.role, NativeRole::Disclosure);
    assert_eq!(native.props.label.as_deref(), Some("Advanced"));
    assert_eq!(native.props.expanded, Some(true));
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onExpandedChange")
            .map(String::as_str),
        Some("toggleAdvanced")
    );

    let collapsed = component
        .render(&DisclosureUiState { expanded: false })
        .unwrap();
    let panel =
        find_element_by_attribute(&collapsed.root, "data-slot", "disclosure-panel").unwrap();
    assert_eq!(attribute_value(panel, "data-expanded"), Some("false"));
    assert_eq!(attribute_value(panel, "aria-hidden"), Some("true"));
    assert_eq!(attribute_value(panel, "hidden"), Some("true"));
}

#[test]
fn rsx_ui_disclosure_group_uses_disclosure_group_hook_props() {
    let component = ComponentCx::compile("ui-disclosure-group", ui_disclosure_group)
        .unwrap()
        .use_reducer(
            "setExpanded",
            |_state: &mut UiDisclosureGroupProps, _invocation| Ok(()),
        );

    let frame = component
        .render(&UiDisclosureGroupProps {
            class_name: "gap-3".to_string(),
            label: "Settings groups".to_string(),
            expanded_keys: "advanced,billing".to_string(),
            on_expanded_change: Some("setExpanded".to_string()),
            allows_multiple_expanded: true,
            is_disabled: true,
        })
        .unwrap();

    let group = find_element_by_attribute(&frame.root, "data-slot", "disclosure-group").unwrap();
    let CompiledRsxNode::Element { props, .. } = group else {
        panic!("disclosure group element");
    };

    assert_eq!(props.label.as_deref(), Some("Settings groups"));
    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onExpandedChange").map(String::as_str),
        Some("setExpanded")
    );
    assert_eq!(attribute_value(group, "role"), Some("group"));
    assert_eq!(attribute_value(group, "aria-disabled"), Some("true"));
    assert_eq!(
        attribute_value(group, "data-disclosure-group"),
        Some("true")
    );
    assert_eq!(
        attribute_value(group, "data-expanded-keys"),
        Some("advanced,billing")
    );
    assert_eq!(
        attribute_value(group, "data-allows-multiple-expanded"),
        Some("true")
    );
    assert_eq!(attribute_value(group, "data-disabled"), Some("true"));
    assert_class_contains(group, "gap-3");

    let native = RsxCompilerBridge::new().lower_to_native(group).unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Settings groups"));
    assert!(native.props.disabled);
    assert_eq!(native.props.explicit_role.as_deref(), Some("group"));
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onExpandedChange")
            .map(String::as_str),
        Some("setExpanded")
    );
}

#[test]
fn rsx_ui_slider_uses_range_hook_props() {
    let component = RsxComponent::new(
        "hooked-slider",
        r#"
        <UiSlider
          key="volume"
          label="Volume"
          valueNumber={state.value}
          minValue="0"
          maxValue="100"
          stepValue="5"
          onChange={setVolume}
        />
        "#,
    )
    .unwrap()
    .use_state("value", |state: &RangeUiState| state.value)
    .use_reducer("setVolume", |_state: &mut RangeUiState, _invocation| Ok(()));

    let frame = component.render(&RangeUiState { value: 150.0 }).unwrap();
    let slider = find_element_by_attribute(&frame.root, "data-slot", "slider").unwrap();
    let CompiledRsxNode::Element { props, .. } = slider else {
        panic!("slider element");
    };

    assert_eq!(props.label.as_deref(), Some("Volume"));
    assert_eq!(props.value_number, Some(100.0));
    assert_eq!(props.min_value, Some(0.0));
    assert_eq!(props.max_value, Some(100.0));
    assert_eq!(props.step_value, Some(5.0));
    assert_eq!(
        props.events.get("onChange").map(String::as_str),
        Some("setVolume")
    );
    assert_eq!(
        props.events.get("onInput").map(String::as_str),
        Some("setVolume")
    );
    assert_eq!(attribute_value(slider, "aria-valuenow"), Some("100.0"));
    assert_eq!(attribute_value(slider, "data-value-percent"), Some("100.0"));

    let native = RsxCompilerBridge::new().lower_to_native(slider).unwrap();
    assert_eq!(native.role, NativeRole::Slider);
    assert_eq!(native.props.current, Some(100.0));
    assert_eq!(native.props.min, Some(0.0));
    assert_eq!(native.props.max, Some(100.0));
    assert_eq!(native.props.step, Some(5.0));
    assert_eq!(native.props.action.as_deref(), Some("setVolume"));
}

#[test]
fn rsx_ui_slider_parts_consume_slider_part_hook_props() {
    let track_frame = ComponentCx::compile("ui-slider-track", ui_slider_track)
        .unwrap()
        .render(&UiSliderTrackProps {
            orientation: "vertical".to_string(),
            is_disabled: true,
            class_name: "rounded-sm".to_string(),
        })
        .unwrap();
    let track = find_element_by_attribute(&track_frame.root, "data-slot", "slider-track").unwrap();
    let CompiledRsxNode::Element { props, .. } = track else {
        panic!("slider track element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(track, "aria-disabled"), Some("true"));
    assert_class_contains(track, "rounded-sm");

    let fill_frame = ComponentCx::compile("ui-slider-fill", ui_slider_fill)
        .unwrap()
        .render(&UiSliderFillProps {
            orientation: "vertical".to_string(),
            value_number: 42.0,
            is_disabled: true,
            class_name: "bg-surface-strong".to_string(),
        })
        .unwrap();
    let fill = find_element_by_attribute(&fill_frame.root, "data-slot", "slider-fill").unwrap();
    let CompiledRsxNode::Element { props, .. } = fill else {
        panic!("slider fill element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(props.value_number, Some(42.0));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(fill, "data-value-number"), Some("42.0"));
    assert_class_contains(fill, "bg-surface-strong");

    let output_frame = ComponentCx::compile("ui-slider-output", ui_slider_output)
        .unwrap()
        .render(&UiSliderOutputProps {
            label: "Volume".to_string(),
            value: "42%".to_string(),
            value_number: 42.0,
            class_name: "tabular-nums".to_string(),
        })
        .unwrap();
    let output =
        find_element_by_attribute(&output_frame.root, "data-slot", "slider-output").unwrap();
    let CompiledRsxNode::Element { props, .. } = output else {
        panic!("slider output element");
    };
    assert_eq!(props.label.as_deref(), Some("Volume"));
    assert_eq!(props.value.as_deref(), Some("42%"));
    assert_eq!(props.value_number, Some(42.0));
    assert_eq!(attribute_value(output, "data-value"), Some("42%"));
    assert_class_contains(output, "tabular-nums");
}

#[test]
fn rsx_ui_number_field_consumes_number_field_hook_props() {
    let component = RsxComponent::new(
        "hooked-number-field",
        r#"
        <UiNumberField
          key="quantity"
          label="Quantity"
          valueNumber={state.value}
          placeholder="0-10"
          minValue="0"
          maxValue="10"
          stepValue="1"
          isRequired={true}
          isInvalid={true}
          isReadOnly={true}
          onChange={setQuantity}
        />
        "#,
    )
    .unwrap()
    .use_state("value", |state: &RangeUiState| state.value)
    .use_reducer("setQuantity", |_state: &mut RangeUiState, _invocation| {
        Ok(())
    });

    let frame = component.render(&RangeUiState { value: -2.0 }).unwrap();
    let number_field = find_element_by_attribute(&frame.root, "data-slot", "number-field").unwrap();
    let input = find_element_by_attribute(&frame.root, "data-slot", "number-field-input").unwrap();
    let decrement =
        find_element_by_attribute(&frame.root, "data-slot", "number-field-decrement").unwrap();
    let increment =
        find_element_by_attribute(&frame.root, "data-slot", "number-field-increment").unwrap();
    let CompiledRsxNode::Element { props, .. } = input else {
        panic!("number input element");
    };

    assert_eq!(props.input_type.as_deref(), Some("number"));
    assert_eq!(props.placeholder.as_deref(), Some("0-10"));
    assert_eq!(props.value_number, Some(0.0));
    assert_eq!(props.min_value, Some(0.0));
    assert_eq!(props.max_value, Some(10.0));
    assert_eq!(props.step_value, Some(1.0));
    assert!(props.is_required);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onInput").map(String::as_str),
        Some("setQuantity")
    );
    assert_eq!(attribute_value(input, "data-value-percent"), Some("0.0"));
    let CompiledRsxNode::Element {
        props: decrement_props,
        ..
    } = decrement
    else {
        panic!("number decrement button");
    };
    let CompiledRsxNode::Element {
        props: increment_props,
        ..
    } = increment
    else {
        panic!("number increment button");
    };
    assert_eq!(attribute_value(decrement, "tabIndex"), Some("-1"));
    assert_eq!(
        decrement_props.events.get("onPress").map(String::as_str),
        Some("setQuantity")
    );
    assert_eq!(attribute_value(decrement, "actionValue"), Some("0"));
    assert_eq!(
        decrement_props.aria_label.as_deref(),
        Some("Decrease Quantity")
    );
    assert!(decrement_props.is_disabled);
    assert_eq!(attribute_value(increment, "actionValue"), Some("1"));
    assert_eq!(
        increment_props.aria_label.as_deref(),
        Some("Increase Quantity")
    );
    assert!(increment_props.is_disabled);

    let native = RsxCompilerBridge::new()
        .lower_to_native(number_field)
        .unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Quantity"));
    assert!(native.props.read_only);
    let controls = native
        .children
        .iter()
        .find(|child| child.role == NativeRole::View)
        .unwrap();
    assert_eq!(
        controls
            .children
            .iter()
            .map(|child| child.role)
            .collect::<Vec<_>>(),
        vec![
            NativeRole::Button,
            NativeRole::TextField,
            NativeRole::Button
        ]
    );
    let input_native = &controls.children[1];
    assert_eq!(input_native.props.label.as_deref(), Some("Quantity"));
    assert_eq!(input_native.props.placeholder.as_deref(), Some("0-10"));
    assert_eq!(input_native.props.current, Some(0.0));
    assert!(input_native.props.required);
    assert!(input_native.props.invalid);
    assert!(input_native.props.read_only);
}

#[test]
fn rsx_ui_number_field_formats_percent_values_in_model_space() {
    let component = RsxComponent::<()>::new(
        "percent-number-field",
        r#"
        <UiNumberField
          key="tax"
          label="Tax"
          valueNumber="0.45"
          minValue="0"
          maxValue="1"
          formatStyle="percent"
          minimumFractionDigits="1"
          maximumFractionDigits="1"
          signDisplay="exceptZero"
        />
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let number_field = find_element_by_attribute(&frame.root, "data-slot", "number-field").unwrap();
    let input = find_element_by_attribute(&frame.root, "data-slot", "number-field-input").unwrap();
    let decrement =
        find_element_by_attribute(&frame.root, "data-slot", "number-field-decrement").unwrap();
    let increment =
        find_element_by_attribute(&frame.root, "data-slot", "number-field-increment").unwrap();
    let CompiledRsxNode::Element { props, .. } = input else {
        panic!("percent number input element");
    };

    assert_eq!(props.value_number, Some(0.45));
    assert_eq!(props.step_value, Some(0.01));
    assert_eq!(attribute_value(input, "data-number-style"), Some("percent"));
    assert_eq!(
        attribute_value(input, "data-number-minimum-fraction-digits"),
        Some("1")
    );
    assert_eq!(
        attribute_value(input, "data-number-maximum-fraction-digits"),
        Some("1")
    );
    assert_eq!(
        attribute_value(input, "data-number-sign-display"),
        Some("exceptZero")
    );
    assert_eq!(attribute_value(decrement, "actionValue"), Some("0.44"));
    assert_eq!(attribute_value(increment, "actionValue"), Some("0.46"));

    let native = RsxCompilerBridge::new()
        .lower_to_native(number_field)
        .unwrap();
    assert_eq!(native.role, NativeRole::View);
    let controls = native
        .children
        .iter()
        .find(|child| child.role == NativeRole::View)
        .unwrap();
    let input_native = controls
        .children
        .iter()
        .find(|child| child.role == NativeRole::TextField)
        .unwrap();
    assert_eq!(input_native.props.current, Some(0.45));
    let blueprint = AppKitAdapter.blueprint(input_native);
    assert_eq!(blueprint.value.as_deref(), Some("+45.0%"));
    assert_eq!(
        blueprint
            .control_state
            .accessibility_description
            .value_text
            .as_deref(),
        Some("+45.0%")
    );
}

#[test]
fn rsx_ui_dialog_uses_overlay_hook_props() {
    let component = RsxComponent::new(
        "hooked-dialog",
        r#"
        <UiDialog
          key="confirm"
          label="Confirm"
          isOpen={state.open}
          onClose={closeDialog}
          isDismissable={true}
          isKeyboardDismissDisabled={true}
        >
          Confirm
        </UiDialog>
        "#,
    )
    .unwrap()
    .use_state("open", |state: &OverlayUiState| state.open)
    .use_reducer("closeDialog", |_state: &mut OverlayUiState, _invocation| {
        Ok(())
    });

    let frame = component
        .render(&OverlayUiState {
            open: true,
            disabled: false,
        })
        .unwrap();
    let dialog = find_element_by_attribute(&frame.root, "data-slot", "dialog").unwrap();
    let CompiledRsxNode::Element { props, .. } = dialog else {
        panic!("dialog element");
    };

    assert_eq!(props.label.as_deref(), Some("Confirm"));
    assert_eq!(
        props.events.get("onClose").map(String::as_str),
        Some("closeDialog")
    );
    assert_eq!(attribute_value(dialog, "open"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-open"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-overlay"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-overlay-modal"), Some("true"));
    assert_eq!(
        attribute_value(dialog, "data-overlay-dismissable"),
        Some("true")
    );
    assert_eq!(
        attribute_value(dialog, "data-overlay-keyboard-dismiss-disabled"),
        Some("true")
    );
    assert_eq!(attribute_value(dialog, "data-focus-scope"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-contain"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-restore-focus"), Some("true"));
    assert_eq!(attribute_value(dialog, "data-auto-focus"), Some("true"));
    assert_eq!(attribute_value(dialog, "aria-modal"), Some("true"));
    assert_eq!(attribute_value(dialog, "aria-hidden"), None);

    let native = RsxCompilerBridge::new().lower_to_native(dialog).unwrap();
    assert_eq!(native.role, NativeRole::Dialog);
    assert_eq!(native.props.label.as_deref(), Some("Confirm"));
    assert_eq!(native.props.html_dialog.open, Some(true));
    assert_eq!(native.props.accessibility_state.modal, Some(true));
    assert_eq!(
        native.props.web.events.get("onClose").map(String::as_str),
        Some("closeDialog")
    );

    let closed = component
        .render(&OverlayUiState {
            open: false,
            disabled: false,
        })
        .unwrap();
    let dialog = find_element_by_attribute(&closed.root, "data-slot", "dialog").unwrap();
    assert_eq!(attribute_value(dialog, "open"), Some("false"));
    assert_eq!(attribute_value(dialog, "data-open"), Some("false"));
    assert_eq!(attribute_value(dialog, "data-overlay"), None);
    assert_eq!(attribute_value(dialog, "data-focus-scope"), None);
    assert_eq!(attribute_value(dialog, "aria-hidden"), None);
    assert_eq!(
        RsxCompilerBridge::new()
            .lower_to_native(dialog)
            .unwrap()
            .props
            .html_dialog
            .open,
        Some(false)
    );
}

#[test]
fn rsx_ui_menu_trigger_uses_overlay_hook_props() {
    let component = RsxComponent::new(
        "hooked-menu-trigger",
        r#"
        <UiMenuTrigger
          key="trigger"
          isOpen={state.open}
          isDisabled={state.disabled}
          onPress={toggleMenu}
          actionValue="menu"
        >
          Actions
        </UiMenuTrigger>
        "#,
    )
    .unwrap()
    .use_state("open", |state: &OverlayUiState| state.open)
    .use_state("disabled", |state: &OverlayUiState| state.disabled)
    .use_reducer("toggleMenu", |_state: &mut OverlayUiState, _invocation| {
        Ok(())
    });

    let frame = component
        .render(&OverlayUiState {
            open: true,
            disabled: true,
        })
        .unwrap();
    let trigger = find_element_by_attribute(&frame.root, "data-slot", "menu-trigger").unwrap();
    let CompiledRsxNode::Element { props, .. } = trigger else {
        panic!("menu trigger element");
    };

    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("toggleMenu")
    );
    assert_eq!(attribute_value(trigger, "data-open"), Some("true"));
    assert_eq!(attribute_value(trigger, "aria-expanded"), Some("true"));
    assert_eq!(attribute_value(trigger, "aria-haspopup"), Some("menu"));
    assert_eq!(attribute_value(trigger, "aria-disabled"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(trigger).unwrap();
    assert_eq!(native.role, NativeRole::Button);
    assert_eq!(native.props.action.as_deref(), Some("toggleMenu"));
    assert!(native.props.disabled);
    assert_eq!(
        native.props.metadata.get("actionValue").map(String::as_str),
        Some("menu")
    );
}

#[test]
fn rsx_ui_submenu_trigger_uses_submenu_trigger_hook_props() {
    let component = RsxComponent::new(
        "hooked-submenu-trigger",
        r#"
        <UiSubmenuTrigger
          key="trigger"
          isOpen={state.open}
          isDisabled={state.disabled}
          isPressed={true}
          onPress={toggleMenu}
          onPressStart={startMenu}
          onPressEnd={endMenu}
          actionValue="more"
        >
          More
        </UiSubmenuTrigger>
        "#,
    )
    .unwrap()
    .use_state("open", |state: &OverlayUiState| state.open)
    .use_state("disabled", |state: &OverlayUiState| state.disabled)
    .use_reducer("toggleMenu", |_state: &mut OverlayUiState, _invocation| {
        Ok(())
    })
    .use_reducer("startMenu", |_state: &mut OverlayUiState, _invocation| {
        Ok(())
    })
    .use_reducer("endMenu", |_state: &mut OverlayUiState, _invocation| Ok(()));

    let frame = component
        .render(&OverlayUiState {
            open: true,
            disabled: true,
        })
        .unwrap();
    let trigger = find_element_by_attribute(&frame.root, "data-slot", "submenu-trigger").unwrap();
    let CompiledRsxNode::Element { props, .. } = trigger else {
        panic!("submenu trigger element");
    };

    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("toggleMenu")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("startMenu")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("endMenu")
    );
    assert_eq!(attribute_value(trigger, "role"), Some("menuitem"));
    assert_eq!(attribute_value(trigger, "aria-haspopup"), Some("menu"));
    assert_eq!(attribute_value(trigger, "aria-expanded"), Some("true"));
    assert_eq!(attribute_value(trigger, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(trigger, "data-open"), Some("true"));
    assert_eq!(attribute_value(trigger, "data-pressed"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(trigger).unwrap();
    assert_eq!(native.role, NativeRole::MenuItem);
    assert_eq!(native.props.action.as_deref(), Some("toggleMenu"));
    assert!(native.props.disabled);
    assert_eq!(
        native.props.metadata.get("actionValue").map(String::as_str),
        Some("more")
    );
}

#[test]
fn rsx_ui_popover_uses_overlay_hook_props() {
    let component = RsxComponent::new(
        "hooked-popover",
        r#"
        <UiPopover
          key="popover"
          isOpen={state.open}
          onClose={closePopover}
          className="shadow-lg"
        >
          Content
        </UiPopover>
        "#,
    )
    .unwrap()
    .use_state("open", |state: &OverlayUiState| state.open)
    .use_reducer(
        "closePopover",
        |_state: &mut OverlayUiState, _invocation| Ok(()),
    );

    let frame = component
        .render(&OverlayUiState {
            open: true,
            disabled: false,
        })
        .unwrap();
    let popover = find_element_by_attribute(&frame.root, "data-slot", "popover").unwrap();

    assert_class_contains(popover, "shadow-lg");
    assert_eq!(attribute_value(popover, "open"), Some("true"));
    assert_eq!(attribute_value(popover, "data-open"), Some("true"));
    assert_eq!(attribute_value(popover, "data-overlay"), Some("true"));
    assert_eq!(attribute_value(popover, "data-overlay-modal"), Some("true"));
    assert_eq!(
        attribute_value(popover, "data-overlay-dismissable"),
        Some("true")
    );
    assert_eq!(attribute_value(popover, "data-focus-scope"), Some("true"));
    assert_eq!(attribute_value(popover, "data-contain"), Some("true"));
    assert_eq!(attribute_value(popover, "data-restore-focus"), Some("true"));
    assert_eq!(attribute_value(popover, "data-auto-focus"), Some("true"));
    assert_eq!(attribute_value(popover, "aria-modal"), Some("true"));
    let CompiledRsxNode::Element { props, .. } = popover else {
        panic!("popover element");
    };
    assert_eq!(
        props.events.get("onClose").map(String::as_str),
        Some("closePopover")
    );
    assert_eq!(attribute_value(popover, "aria-hidden"), None);

    let native = RsxCompilerBridge::new().lower_to_native(popover).unwrap();
    assert_eq!(native.role, NativeRole::Popover);
    assert_eq!(native.props.accessibility_state.modal, Some(true));

    let closed = component
        .render(&OverlayUiState {
            open: false,
            disabled: false,
        })
        .unwrap();
    let popover = find_element_by_attribute(&closed.root, "data-slot", "popover").unwrap();
    assert_eq!(attribute_value(popover, "open"), Some("false"));
    assert_eq!(attribute_value(popover, "data-open"), Some("false"));
    assert_eq!(attribute_value(popover, "data-overlay"), None);
    assert_eq!(attribute_value(popover, "data-focus-scope"), None);
    assert_eq!(attribute_value(popover, "aria-hidden"), None);
}

#[test]
fn rsx_ui_non_modal_popover_keeps_focus_restore_without_modal_isolation() {
    let component = RsxComponent::new(
        "non-modal-popover",
        r#"
        <UiPopover key="popover" isOpen={true} isNonModal={true}>
          Content
        </UiPopover>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let popover = find_element_by_attribute(&frame.root, "data-slot", "popover").unwrap();

    assert_eq!(attribute_value(popover, "data-overlay"), Some("true"));
    assert_eq!(attribute_value(popover, "data-overlay-modal"), None);
    assert_eq!(attribute_value(popover, "data-overlay-dismissable"), None);
    assert_eq!(attribute_value(popover, "data-contain"), None);
    assert_eq!(attribute_value(popover, "aria-modal"), None);
    assert_eq!(attribute_value(popover, "data-focus-scope"), Some("true"));
    assert_eq!(attribute_value(popover, "data-restore-focus"), Some("true"));
    assert_eq!(attribute_value(popover, "data-auto-focus"), Some("true"));
}

#[test]
fn rsx_ui_menu_uses_menu_hooks_for_menu_and_items() {
    let component = RsxComponent::new(
        "hooked-menu",
        r#"
        <UiMenu key="menu" label="Actions" isDisabled={state.disabled}>
          <UiMenuItem
            key="open"
            textValue="Open file"
            actionValue="open"
            onAction={openFile}
            isSelected={state.selected}
            isDisabled={state.disabled}
          >
            Open
          </UiMenuItem>
        </UiMenu>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &MenuUiState| state.selected)
    .use_state("disabled", |state: &MenuUiState| state.disabled)
    .use_reducer("openFile", |_state: &mut MenuUiState, _invocation| Ok(()));

    let frame = component
        .render(&MenuUiState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let menu = find_element_by_attribute(&frame.root, "data-slot", "menu").unwrap();
    let item = find_element_by_attribute(&frame.root, "data-slot", "menu-item").unwrap();
    let CompiledRsxNode::Element { props, .. } = menu else {
        panic!("menu element");
    };
    let CompiledRsxNode::Element {
        props: item_props, ..
    } = item
    else {
        panic!("menu item element");
    };

    assert_eq!(props.label.as_deref(), Some("Actions"));
    assert_eq!(props.aria_label.as_deref(), Some("Actions"));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(menu, "role"), Some("menu"));
    assert_eq!(attribute_value(menu, "aria-disabled"), Some("true"));
    assert_eq!(item_props.text_value.as_deref(), Some("Open file"));
    assert!(item_props.is_disabled);
    assert!(item_props.is_selected);
    assert_eq!(
        item_props.events.get("onPress").map(String::as_str),
        Some("openFile")
    );
    assert_eq!(attribute_value(item, "role"), Some("menuitem"));
    assert_eq!(attribute_value(item, "actionValue"), Some("open"));
    assert_eq!(attribute_value(item, "data-selected"), Some("true"));
    assert_eq!(attribute_value(item, "data-disabled"), Some("true"));

    let bridge = RsxCompilerBridge::new();
    let native = bridge.lower_to_native(menu).unwrap();
    assert_eq!(native.role, NativeRole::Menu);
    assert_eq!(native.props.label.as_deref(), Some("Actions"));
    assert!(native.props.disabled);

    let item_native = bridge.lower_to_native(item).unwrap();
    assert_eq!(item_native.role, NativeRole::MenuItem);
    assert_eq!(item_native.props.label.as_deref(), Some("Open file"));
    assert_eq!(item_native.props.action.as_deref(), Some("openFile"));
    assert!(item_native.props.disabled);
    assert!(item_native.props.selected);
    assert_eq!(
        item_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("open")
    );
}

#[test]
fn rsx_ui_collection_uses_collection_hook_props() {
    let component = ComponentCx::compile("ui-collection", ui_collection).unwrap();

    let frame = component
        .render(&UiCollectionProps {
            class_name: "gap-4".to_string(),
            label: "Rows".to_string(),
            item_count: 24,
            is_empty: true,
            is_disabled: true,
        })
        .unwrap();

    let collection = find_element_by_attribute(&frame.root, "data-slot", "collection").unwrap();
    let CompiledRsxNode::Element { props, .. } = collection else {
        panic!("collection element");
    };

    assert_eq!(props.label.as_deref(), Some("Rows"));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(collection, "role"), Some("group"));
    assert_eq!(attribute_value(collection, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(collection, "data-collection"), Some("true"));
    assert_eq!(attribute_value(collection, "data-item-count"), Some("24"));
    assert_eq!(attribute_value(collection, "data-empty"), Some("true"));
    assert_eq!(attribute_value(collection, "data-disabled"), Some("true"));
    assert_class_contains(collection, "gap-4");

    let native = RsxCompilerBridge::new()
        .lower_to_native(collection)
        .unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Rows"));
    assert!(native.props.disabled);
    assert_eq!(native.props.explicit_role.as_deref(), Some("group"));
}

#[test]
fn rsx_ui_collection_items_use_collection_item_hook_props() {
    let component = RsxComponent::new(
        "hooked-collection-items",
        r#"
        <UiGroup key="root">
          <UiListBox key="list">
            <UiListBoxItem
              key="list-item"
              value="alpha"
              textValue="Alpha"
              isSelected={state.selected}
              isDisabled={state.disabled}
            >
              Alpha
            </UiListBoxItem>
          </UiListBox>
          <UiGridList key="grid">
            <UiGridListItem
              key="grid-item"
              value="report"
              textValue="Report"
              isSelected={state.selected}
              isDisabled={state.disabled}
            >
              Report
            </UiGridListItem>
          </UiGridList>
          <UiTree key="tree">
            <UiTreeItem
              key="tree-item"
              value="src"
              textValue="src"
              isExpanded={state.expanded}
              isSelected={state.selected}
              isDisabled={state.disabled}
            >
              src
            </UiTreeItem>
          </UiTree>
          <UiTagList key="tags">
            <UiTag
              key="tag"
              value="bug"
              textValue="Bug"
              isSelected={state.selected}
              isDisabled={state.disabled}
            >
              Bug
            </UiTag>
          </UiTagList>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &CollectionItemUiState| state.selected)
    .use_state("disabled", |state: &CollectionItemUiState| state.disabled)
    .use_state("expanded", |state: &CollectionItemUiState| state.expanded);

    let frame = component
        .render(&CollectionItemUiState {
            selected: true,
            disabled: true,
            expanded: true,
        })
        .unwrap();
    let list_item = find_element_by_attribute(&frame.root, "data-slot", "list-box-item").unwrap();
    let grid_item = find_element_by_attribute(&frame.root, "data-slot", "grid-list-item").unwrap();
    let tree_item = find_element_by_attribute(&frame.root, "data-slot", "tree-item").unwrap();
    let tag = find_element_by_attribute(&frame.root, "data-slot", "tag").unwrap();

    for item in [list_item, grid_item, tree_item, tag] {
        let CompiledRsxNode::Element { props, .. } = item else {
            panic!("collection item element");
        };
        assert!(props.is_selected);
        assert!(props.is_disabled);
        assert_eq!(attribute_value(item, "data-selected"), Some("true"));
        assert_eq!(attribute_value(item, "data-disabled"), Some("true"));
        assert_eq!(attribute_value(item, "aria-selected"), Some("true"));
        assert_eq!(attribute_value(item, "aria-disabled"), Some("true"));
    }

    let CompiledRsxNode::Element {
        props: list_props, ..
    } = list_item
    else {
        panic!("list item element");
    };
    let CompiledRsxNode::Element {
        props: grid_props, ..
    } = grid_item
    else {
        panic!("grid item element");
    };
    let CompiledRsxNode::Element {
        props: tree_props, ..
    } = tree_item
    else {
        panic!("tree item element");
    };
    let CompiledRsxNode::Element {
        props: tag_props, ..
    } = tag
    else {
        panic!("tag element");
    };

    assert_eq!(list_props.value.as_deref(), Some("alpha"));
    assert_eq!(list_props.text_value.as_deref(), Some("Alpha"));
    assert_eq!(grid_props.value.as_deref(), Some("report"));
    assert_eq!(grid_props.text_value.as_deref(), Some("Report"));
    assert_eq!(tree_props.value.as_deref(), Some("src"));
    assert_eq!(tree_props.text_value.as_deref(), Some("src"));
    assert_eq!(tree_props.is_expanded, Some(true));
    assert_eq!(attribute_value(tree_item, "role"), Some("treeitem"));
    assert_eq!(attribute_value(tree_item, "data-expanded"), Some("true"));
    assert_eq!(attribute_value(tree_item, "aria-expanded"), Some("true"));
    assert_eq!(tag_props.value.as_deref(), Some("bug"));
    assert_eq!(tag_props.text_value.as_deref(), Some("Bug"));

    let bridge = RsxCompilerBridge::new();
    for item in [list_item, grid_item, tag] {
        let native = bridge.lower_to_native(item).unwrap();
        assert_eq!(native.role, NativeRole::ListBoxItem);
        assert!(native.props.selected);
        assert!(native.props.disabled);
    }
    let tree_native = bridge.lower_to_native(tree_item).unwrap();
    assert_eq!(tree_native.role, NativeRole::TreeItem);
    assert!(tree_native.props.selected);
    assert!(tree_native.props.disabled);
    assert_eq!(tree_native.props.expanded, Some(true));
}

#[test]
fn rsx_ui_tree_uses_tree_hook_props() {
    let component = RsxComponent::new(
        "hooked-tree",
        r#"
        <UiTree
          key="tree"
          label="Project"
          value={state.selected}
          selectionMode="multiple"
          isDisabled={state.readOnly}
          isReadOnly={state.readOnly}
          onSelectionChange={setSelected}
        >
          <UiTreeItem
            key="src"
            value="src"
            textValue="src"
            isSelected={state.srcSelected}
            isExpanded={true}
            isDisabled={state.readOnly}
          >
            <UiTreeItemContent key="src-content">src</UiTreeItemContent>
          </UiTreeItem>
        </UiTree>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &SelectionUiState| {
        state.selected.clone()
    })
    .use_state("srcSelected", |state: &SelectionUiState| {
        state.selected == "src"
    })
    .use_state("readOnly", |state: &SelectionUiState| state.read_only)
    .use_value_reducer(
        "setSelected",
        |state: &mut SelectionUiState, selected: String| {
            state.selected = selected;
            Ok(())
        },
    );

    let frame = component
        .render(&SelectionUiState {
            selected: "src".to_string(),
            read_only: true,
        })
        .unwrap();
    let tree = find_element_by_attribute(&frame.root, "data-slot", "tree").unwrap();
    let tree_item = find_element_by_attribute(&frame.root, "data-slot", "tree-item").unwrap();
    let CompiledRsxNode::Element {
        props: tree_props, ..
    } = tree
    else {
        panic!("tree element");
    };
    let CompiledRsxNode::Element {
        props: tree_item_props,
        ..
    } = tree_item
    else {
        panic!("tree item element");
    };

    assert_eq!(tree_props.label.as_deref(), Some("Project"));
    assert_eq!(tree_props.aria_label.as_deref(), Some("Project"));
    assert_eq!(tree_props.value.as_deref(), Some("src"));
    assert!(tree_props.is_disabled);
    assert!(tree_props.is_read_only);
    assert_eq!(
        tree_props
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setSelected")
    );
    assert_eq!(attribute_value(tree, "role"), Some("tree"));
    assert_eq!(attribute_value(tree, "data-selected-value"), Some("src"));
    assert_eq!(
        attribute_value(tree, "data-selection-mode"),
        Some("multiple")
    );
    assert_eq!(attribute_value(tree, "aria-multiselectable"), Some("true"));
    assert_eq!(attribute_value(tree, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(tree, "aria-readonly"), Some("true"));

    assert_eq!(tree_item_props.value.as_deref(), Some("src"));
    assert_eq!(tree_item_props.text_value.as_deref(), Some("src"));
    assert!(tree_item_props.is_selected);
    assert!(tree_item_props.is_disabled);
    assert_eq!(tree_item_props.is_expanded, Some(true));
    assert_eq!(attribute_value(tree_item, "role"), Some("treeitem"));
    assert_eq!(attribute_value(tree_item, "tabIndex"), Some("-1"));
    assert_eq!(attribute_value(tree_item, "aria-selected"), Some("true"));
    assert_eq!(attribute_value(tree_item, "aria-expanded"), Some("true"));
    assert_eq!(attribute_value(tree_item, "aria-disabled"), Some("true"));

    let bridge = RsxCompilerBridge::new();
    let native = bridge.lower_to_native(tree).unwrap();
    assert_eq!(native.role, NativeRole::Tree);
    assert_eq!(native.props.label.as_deref(), Some("Project"));
    assert_eq!(native.props.value.as_deref(), Some("src"));
    assert!(native.props.disabled);
    assert!(native.props.read_only);
    assert_eq!(
        native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setSelected")
    );

    let native = bridge.lower_to_native(tree_item).unwrap();
    assert_eq!(native.role, NativeRole::TreeItem);
    assert_eq!(native.props.value.as_deref(), Some("src"));
    assert!(native.props.selected);
    assert!(native.props.disabled);
    assert_eq!(native.props.expanded, Some(true));
}

#[test]
fn rsx_ui_load_more_items_use_load_more_item_hook_props() {
    let component = RsxComponent::new(
        "hooked-load-more-items",
        r#"
        <UiGroup key="root">
          <UiListBox key="list">
            <UiListBoxLoadMoreItem
              key="list-more"
              label="More list"
              onPress={loadMore}
              isLoading={state.loading}
              isDisabled={state.disabled}
              actionValue="list-next"
              actionPayload="list-cursor"
            >
              More list
            </UiListBoxLoadMoreItem>
          </UiListBox>
          <UiGridList key="grid">
            <UiGridListLoadMoreItem
              key="grid-more"
              label="More grid"
              onPress={loadMore}
              isLoading={state.loading}
              isDisabled={state.disabled}
              actionValue="grid-next"
              actionPayload="grid-cursor"
            >
              More grid
            </UiGridListLoadMoreItem>
          </UiGridList>
          <UiTable key="table">
            <UiTableLoadMoreItem
              key="table-more"
              label="More rows"
              onPress={loadMore}
              isLoading={state.loading}
              isDisabled={state.disabled}
              actionValue="table-next"
              actionPayload="table-cursor"
            >
              More rows
            </UiTableLoadMoreItem>
          </UiTable>
          <UiTree key="tree">
            <UiTreeLoadMoreItem
              key="tree-more"
              label="More tree"
              onPress={loadMore}
              isLoading={state.loading}
              isDisabled={state.disabled}
              actionValue="tree-next"
              actionPayload="tree-cursor"
            >
              More tree
            </UiTreeLoadMoreItem>
          </UiTree>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("loading", |state: &LoadMoreUiState| state.loading)
    .use_state("disabled", |state: &LoadMoreUiState| state.disabled)
    .use_reducer("loadMore", |_state: &mut LoadMoreUiState, _invocation| {
        Ok(())
    });

    let frame = component
        .render(&LoadMoreUiState {
            loading: true,
            disabled: false,
        })
        .unwrap();
    let list_more =
        find_element_by_attribute(&frame.root, "data-slot", "list-box-load-more-item").unwrap();
    let grid_more =
        find_element_by_attribute(&frame.root, "data-slot", "grid-list-load-more-item").unwrap();
    let table_more =
        find_element_by_attribute(&frame.root, "data-slot", "table-load-more-item").unwrap();
    let tree_more =
        find_element_by_attribute(&frame.root, "data-slot", "tree-load-more-item").unwrap();

    for (item, label, action_value, action_payload) in [
        (list_more, "More list", "list-next", "list-cursor"),
        (grid_more, "More grid", "grid-next", "grid-cursor"),
        (table_more, "More rows", "table-next", "table-cursor"),
        (tree_more, "More tree", "tree-next", "tree-cursor"),
    ] {
        let CompiledRsxNode::Element { props, .. } = item else {
            panic!("load more item element");
        };
        assert_eq!(props.label.as_deref(), Some(label));
        assert_eq!(props.text_value.as_deref(), Some(label));
        assert!(props.is_disabled);
        assert_eq!(
            props.events.get("onPress").map(String::as_str),
            Some("loadMore")
        );
        assert_eq!(attribute_value(item, "actionValue"), Some(action_value));
        assert_eq!(attribute_value(item, "actionPayload"), Some(action_payload));
        assert_eq!(attribute_value(item, "aria-busy"), Some("true"));
        assert_eq!(attribute_value(item, "aria-disabled"), Some("true"));
        assert_eq!(attribute_value(item, "data-loading"), Some("true"));
        assert_eq!(attribute_value(item, "data-disabled"), Some("true"));
    }

    let bridge = RsxCompilerBridge::new();
    for item in [list_more, grid_more] {
        let native = bridge.lower_to_native(item).unwrap();
        assert_eq!(native.role, NativeRole::ListBoxItem);
        assert_eq!(native.props.action.as_deref(), Some("loadMore"));
        assert!(native.props.disabled);
    }
    let native = bridge.lower_to_native(tree_more).unwrap();
    assert_eq!(native.role, NativeRole::TreeItem);
    assert_eq!(native.props.action.as_deref(), Some("loadMore"));
    assert!(native.props.disabled);

    let native = bridge.lower_to_native(table_more).unwrap();
    assert_eq!(native.role, NativeRole::TableRow);
    assert_eq!(native.props.action.as_deref(), Some("loadMore"));
    assert!(native.props.disabled);
}

#[test]
fn rsx_ui_table_parts_use_table_hook_props() {
    let component = RsxComponent::new(
        "hooked-table",
        r#"
        <UiTable key="table" label="Files">
          <UiTableCaption key="caption" label="Files table" textValue="Files table">
            Files table
          </UiTableCaption>
          <UiTableHeader key="header">
            <UiTableRow key="header-row">
              <UiTableColumn key="name-column" label="Name" textValue="Name">
                Name
              </UiTableColumn>
            </UiTableRow>
          </UiTableHeader>
          <UiTableBody key="body">
            <UiTableRow key="ada-row" isSelected={state.selectedRow}>
              <UiTableCell key="ada-cell" label="Name" textValue="Ada">
                Ada
              </UiTableCell>
            </UiTableRow>
          </UiTableBody>
          <UiTableFooter key="footer">
            <UiTableRow key="footer-row">
              <UiTableCell key="footer-cell" label="Total" textValue="1 file">
                1 file
              </UiTableCell>
            </UiTableRow>
          </UiTableFooter>
        </UiTable>
        "#,
    )
    .unwrap()
    .use_state("selectedRow", |state: &CollectionState| state.selected_row);

    let frame = component
        .render(&CollectionState {
            selected_row: true,
            ..CollectionState::default()
        })
        .unwrap();
    let table = find_element_by_attribute(&frame.root, "data-slot", "table").unwrap();
    let caption = find_element_by_attribute(&frame.root, "data-slot", "table-caption").unwrap();
    let header = find_element_by_attribute(&frame.root, "data-slot", "table-header").unwrap();
    let body = find_element_by_attribute(&frame.root, "data-slot", "table-body").unwrap();
    let footer = find_element_by_attribute(&frame.root, "data-slot", "table-footer").unwrap();
    let row = find_element_by_attributes(
        &frame.root,
        &[("data-slot", "table-row"), ("data-selected", "true")],
    )
    .unwrap();
    let column = find_element_by_attribute(&frame.root, "data-slot", "table-column").unwrap();
    let cell = find_element_by_attribute(&frame.root, "data-slot", "table-cell").unwrap();

    let CompiledRsxNode::Element { props, .. } = table else {
        panic!("table element");
    };
    assert_eq!(props.label.as_deref(), Some("Files"));
    assert_eq!(props.aria_label.as_deref(), Some("Files"));
    assert_eq!(attribute_value(table, "role"), Some("table"));

    let CompiledRsxNode::Element { props, .. } = caption else {
        panic!("caption element");
    };
    assert_eq!(props.label.as_deref(), Some("Files table"));
    assert_eq!(props.text_value.as_deref(), Some("Files table"));

    for (section, kind) in [(header, "header"), (body, "body"), (footer, "footer")] {
        assert_eq!(attribute_value(section, "role"), Some("rowgroup"));
        assert_eq!(attribute_value(section, "data-table-section"), Some(kind));
    }

    let CompiledRsxNode::Element { props, .. } = row else {
        panic!("row element");
    };
    assert!(props.is_selected);
    assert_eq!(attribute_value(row, "role"), Some("row"));
    assert_eq!(attribute_value(row, "aria-selected"), Some("true"));
    assert_eq!(attribute_value(row, "data-selected"), Some("true"));

    let CompiledRsxNode::Element { props, .. } = column else {
        panic!("column element");
    };
    assert_eq!(props.label.as_deref(), Some("Name"));
    assert_eq!(props.text_value.as_deref(), Some("Name"));
    assert_eq!(attribute_value(column, "role"), Some("columnheader"));

    let CompiledRsxNode::Element { props, .. } = cell else {
        panic!("cell element");
    };
    assert_eq!(props.label.as_deref(), Some("Name"));
    assert_eq!(props.text_value.as_deref(), Some("Ada"));
    assert_eq!(attribute_value(cell, "role"), Some("cell"));

    let bridge = RsxCompilerBridge::new();
    assert_eq!(
        bridge.lower_to_native(table).unwrap().role,
        NativeRole::Table
    );
    assert_eq!(
        bridge.lower_to_native(caption).unwrap().role,
        NativeRole::TableCaption
    );
    for section in [header, body, footer] {
        assert_eq!(
            bridge.lower_to_native(section).unwrap().role,
            NativeRole::TableSection
        );
    }
    assert_eq!(
        bridge.lower_to_native(row).unwrap().role,
        NativeRole::TableRow
    );
    assert_eq!(
        bridge.lower_to_native(column).unwrap().role,
        NativeRole::TableColumn
    );
    assert_eq!(
        bridge.lower_to_native(cell).unwrap().role,
        NativeRole::TableCell
    );
}

#[test]
fn rsx_ui_resizable_table_container_uses_group_hook_props() {
    let component = RsxComponent::<()>::new(
        "resizable-table-container",
        r#"
        <UiResizableTableContainer key="container" label="Resizable members" className="max-h-80">
          <UiTable key="table" label="Members">
            <UiTableBody key="body" />
          </UiTable>
        </UiResizableTableContainer>
        "#,
    )
    .unwrap();

    let frame = component.render(&()).unwrap();
    let container =
        find_element_by_attribute(&frame.root, "data-slot", "resizable-table-container").unwrap();
    let CompiledRsxNode::Element { props, .. } = container else {
        panic!("resizable table container element");
    };

    assert_eq!(props.label.as_deref(), Some("Resizable members"));
    assert_eq!(attribute_value(container, "role"), Some("group"));
    assert_class_contains(container, "max-h-80");

    let native = RsxCompilerBridge::new().lower_to_native(container).unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Resizable members"));
    assert_eq!(native.props.explicit_role.as_deref(), Some("group"));
}

#[test]
fn rsx_ui_radio_group_uses_radio_group_hook_props() {
    let component = RsxComponent::new(
        "hooked-radio-group",
        r#"
        <UiRadioGroup
          key="theme"
          label="Theme"
          value="dark"
          onSelectionChange={setTheme}
          isDisabled={state.disabled}
          isRequired={true}
          isInvalid={true}
          isReadOnly={true}
        >
          <UiRadio
            key="dark"
            value="dark"
            textValue="Dark"
            isSelected={state.selected}
          >
            Dark
          </UiRadio>
        </UiRadioGroup>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &RadioUiState| state.selected)
    .use_state("disabled", |state: &RadioUiState| state.disabled)
    .use_value_reducer("setTheme", |_state: &mut RadioUiState, _theme: String| {
        Ok(())
    });

    let frame = component
        .render(&RadioUiState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let group = find_element_by_attribute(&frame.root, "data-slot", "radio-group").unwrap();
    let CompiledRsxNode::Element { props, .. } = group else {
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
    assert_eq!(attribute_value(group, "role"), Some("radiogroup"));
    assert_eq!(attribute_value(group, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(group, "aria-required"), Some("true"));
    assert_eq!(attribute_value(group, "aria-invalid"), Some("true"));
    assert_eq!(attribute_value(group, "aria-readonly"), Some("true"));
    assert_eq!(attribute_value(group, "data-selected-value"), Some("dark"));
    assert_eq!(
        attribute_value(group, "data-selection-mode"),
        Some("single")
    );

    let native = RsxCompilerBridge::new().lower_to_native(group).unwrap();
    assert_eq!(native.role, NativeRole::RadioGroup);
    assert_eq!(native.props.label.as_deref(), Some("Theme"));
    assert_eq!(native.props.value.as_deref(), Some("dark"));
    assert!(native.props.disabled);
}

#[test]
fn rsx_ui_radio_uses_radio_hook_props() {
    let component = RsxComponent::new(
        "hooked-radio",
        r#"
        <UiRadio
          key="dark"
          value="dark"
          textValue="Dark"
          isSelected={state.selected}
          isDisabled={state.disabled}
        >
          Dark
        </UiRadio>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &RadioUiState| state.selected)
    .use_state("disabled", |state: &RadioUiState| state.disabled);

    let frame = component
        .render(&RadioUiState {
            selected: true,
            disabled: true,
        })
        .unwrap();
    let radio = find_element_by_attribute(&frame.root, "data-slot", "radio").unwrap();
    let CompiledRsxNode::Element { props, .. } = radio else {
        panic!("radio element");
    };

    assert_eq!(props.value.as_deref(), Some("dark"));
    assert_eq!(props.text_value.as_deref(), Some("Dark"));
    assert!(props.is_selected);
    assert_eq!(props.is_checked, Some(true));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(radio, "role"), Some("radio"));
    assert_eq!(attribute_value(radio, "aria-checked"), Some("true"));
    assert_eq!(attribute_value(radio, "data-selected"), Some("true"));
    assert_eq!(attribute_value(radio, "data-disabled"), Some("true"));

    let native = RsxCompilerBridge::new().lower_to_native(radio).unwrap();
    assert_eq!(native.role, NativeRole::Radio);
    assert_eq!(native.props.value.as_deref(), Some("dark"));
    assert_eq!(native.props.label.as_deref(), Some("Dark"));
    assert_eq!(native.props.checked, Some(true));
    assert!(native.props.selected);
    assert!(native.props.disabled);
}

#[test]
fn rsx_ui_renders_composed_controls_to_native_roles() {
    let component = RsxComponent::new(
        "controls",
        r#"
        <div key="root" class="grid gap-4">
          <UiCheckbox
            key="accepted"
            isChecked={state.accepted}
            onChange={setAccepted}
          >
            Accept terms
          </UiCheckbox>
          <UiSwitch
            key="sync"
            isChecked={state.enabled}
            onChange={setEnabled}
          >
            Sync
          </UiSwitch>
          <UiRadioGroup
            key="theme"
            label="Theme"
            value={state.theme}
            onSelectionChange={setTheme}
          >
            <UiRadio
              key="light"
              value="light"
              textValue="Light"
              isSelected={state.lightSelected}
            >
              Light
            </UiRadio>
            <UiRadio
              key="dark"
              value="dark"
              textValue="Dark"
              isSelected={state.darkSelected}
            >
              Dark
            </UiRadio>
          </UiRadioGroup>
          <UiSelect
            key="density"
            value={state.density}
            onSelectionChange={setDensity}
          >
            <UiLabel key="density-label">Density</UiLabel>
            <UiButton key="density-button" variant="outline" onPress={openDensity}>
              <UiSelectValue
                key="density-value"
                value={state.density}
                placeholder="Density"
              />
            </UiButton>
            <UiPopover key="density-popover">
              <UiListBox key="density-list">
                <UiListBoxItem
                  key="compact"
                  value="compact"
                  textValue="Compact"
                  isSelected={state.compactSelected}
                >
                  Compact
                </UiListBoxItem>
                <UiListBoxItem
                  key="comfortable"
                  value="comfortable"
                  textValue="Comfortable"
                  isSelected={state.comfortableSelected}
                >
                  Comfortable
                </UiListBoxItem>
              </UiListBox>
            </UiPopover>
          </UiSelect>
          <UiMenu key="file-menu">
            <UiMenuItem key="archive" onAction={archive} actionValue="archive">
              Archive
            </UiMenuItem>
          </UiMenu>
          <UiSlider
            key="volume"
            label="Volume"
            valueNumber={state.volume}
            minValue="0"
            maxValue="100"
            stepValue="5"
            onChange={setVolume}
          />
        </div>
        "#,
    )
    .unwrap()
    .use_state("accepted", |state: &ControlState| state.accepted)
    .use_state("enabled", |state: &ControlState| state.enabled)
    .use_state("theme", |state: &ControlState| state.theme.clone())
    .use_state("lightSelected", |state: &ControlState| {
        state.theme == "light"
    })
    .use_state("darkSelected", |state: &ControlState| state.theme == "dark")
    .use_state("density", |state: &ControlState| state.density.clone())
    .use_state("compactSelected", |state: &ControlState| {
        state.density == "compact"
    })
    .use_state("comfortableSelected", |state: &ControlState| {
        state.density == "comfortable"
    })
    .use_state("volume", |state: &ControlState| state.volume)
    .use_value_reducer("setAccepted", |state: &mut ControlState, value: bool| {
        state.accepted = value;
        Ok(())
    })
    .use_value_reducer("setEnabled", |state: &mut ControlState, value: bool| {
        state.enabled = value;
        Ok(())
    })
    .use_value_reducer("setTheme", |state: &mut ControlState, theme: String| {
        state.theme = theme;
        Ok(())
    })
    .use_value_reducer("setDensity", |state: &mut ControlState, density: String| {
        state.density = density;
        Ok(())
    })
    .use_value_reducer("setVolume", |state: &mut ControlState, volume: f64| {
        state.volume = volume;
        Ok(())
    })
    .use_reducer("openDensity", |_state, _invocation| Ok(()))
    .use_reducer("archive", |_state, _invocation| Ok(()));

    let frame = component
        .render(&ControlState {
            accepted: true,
            enabled: true,
            theme: "dark".to_string(),
            density: "comfortable".to_string(),
            volume: 40.0,
        })
        .unwrap();

    let checkbox = find_element_by_attribute(&frame.root, "data-slot", "checkbox").unwrap();
    let switch = find_element_by_attribute(&frame.root, "data-slot", "switch").unwrap();
    let radio_group = find_element_by_attribute(&frame.root, "data-slot", "radio-group").unwrap();
    let select = find_element_by_attribute(&frame.root, "data-slot", "select").unwrap();
    let menu = find_element_by_attribute(&frame.root, "data-slot", "menu").unwrap();
    let slider = find_element_by_attribute(&frame.root, "data-slot", "slider").unwrap();

    assert_class_contains(checkbox, "size-4");
    assert_class_contains(switch, "rounded-full");
    assert_class_contains(select, "grid");

    let bridge = RsxCompilerBridge::new();
    let checkbox_native = bridge.lower_to_native(checkbox).unwrap();
    assert_eq!(checkbox_native.role, NativeRole::Checkbox);
    assert_eq!(checkbox_native.props.checked, Some(true));
    assert_eq!(checkbox_native.props.action.as_deref(), Some("setAccepted"));

    let switch_native = bridge.lower_to_native(switch).unwrap();
    assert_eq!(switch_native.role, NativeRole::Switch);
    assert_eq!(switch_native.props.checked, Some(true));
    assert_eq!(switch_native.props.action.as_deref(), Some("setEnabled"));

    let radio_native = bridge.lower_to_native(radio_group).unwrap();
    assert_eq!(radio_native.role, NativeRole::RadioGroup);
    assert_eq!(radio_native.props.label.as_deref(), Some("Theme"));
    assert_eq!(
        radio_native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setTheme")
    );
    let dark_radio = radio_native
        .children
        .iter()
        .find(|child| {
            child.role == NativeRole::Radio && child.props.value.as_deref() == Some("dark")
        })
        .unwrap();
    assert_eq!(dark_radio.props.checked, Some(true));

    let select_native = bridge.lower_to_native(select).unwrap();
    assert_eq!(select_native.role, NativeRole::Select);
    assert_eq!(select_native.props.label.as_deref(), Some("Density"));
    assert_eq!(
        select_native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setDensity")
    );
    assert_eq!(select_native.children.len(), 2);
    assert_eq!(select_native.children[1].role, NativeRole::ListBoxItem);
    assert_eq!(
        select_native.children[1].props.value.as_deref(),
        Some("comfortable")
    );
    assert!(select_native.children[1].props.selected);

    let menu_native = bridge.lower_to_native(menu).unwrap();
    assert_eq!(menu_native.role, NativeRole::Menu);
    let menu_item = menu_native
        .children
        .iter()
        .find(|child| child.role == NativeRole::MenuItem)
        .unwrap();
    assert_eq!(menu_item.role, NativeRole::MenuItem);
    assert_eq!(menu_item.props.action.as_deref(), Some("archive"));

    let slider_native = bridge.lower_to_native(slider).unwrap();
    assert_eq!(slider_native.role, NativeRole::Slider);
    assert_eq!(slider_native.props.label.as_deref(), Some("Volume"));
    assert_eq!(slider_native.props.current, Some(40.0));
    assert_eq!(slider_native.props.min, Some(0.0));
    assert_eq!(slider_native.props.max, Some(100.0));
    assert_eq!(slider_native.props.step, Some(5.0));
    assert_eq!(slider_native.props.action.as_deref(), Some("setVolume"));
}

#[test]
fn rsx_ui_renders_layout_feedback_and_text_primitives_to_native_roles() {
    let component = RsxComponent::new(
        "primitives",
        r#"
        <UiForm
          key="form"
          label="Profile"
          onSubmit={submitProfile}
          onReset={resetProfile}
          onInvalid={invalidProfile}
          validationBehavior="aria"
          isInvalid={true}
          noValidate={true}
        >
          <UiGroup key="section" label="Account" className="gap-3">
            <UiHeading key="title" level="2">Profile</UiHeading>
            <UiText key="copy">Native RSX primitives</UiText>
            <UiTextField
              key="name"
              label="Name"
              value={state.name}
              placeholder="Name"
              onChange={setName}
            />
            <UiSearchField
              key="search"
              label="Search"
              value={state.query}
              placeholder="Search"
              onChange={setQuery}
            />
          </UiGroup>
          <UiFieldSet key="prefs" label="Preferences">
            <UiLegend key="prefs-title">Preferences</UiLegend>
            <UiText key="prefs-copy">Workspace defaults</UiText>
          </UiFieldSet>
          <UiSeparator key="section-separator" orientation="horizontal" />
          <UiToolbar key="toolbar" label="Actions">
            <UiLink key="docs" href="/docs" onPress={openDocs}>Docs</UiLink>
            <UiButton key="save" onPress={saveProfile}>Save</UiButton>
          </UiToolbar>
          <UiDisclosure
            key="advanced"
            label="Advanced"
            isExpanded={state.expanded}
            onExpandedChange={toggleAdvanced}
          >
            <UiDisclosureSummary key="advanced-summary" onPress={toggleAdvanced}>
              Advanced
            </UiDisclosureSummary>
            <UiText key="advanced-body">Extra options</UiText>
          </UiDisclosure>
          <UiProgressBar
            key="sync"
            label="Sync"
            valueNumber={state.progress}
            minValue="0"
            maxValue="100"
          />
          <UiMeter
            key="quota"
            label="Quota"
            valueNumber={state.quota}
            minValue="0"
            maxValue="100"
          />
          <UiDialog
            key="confirm"
            label="Confirm"
            isOpen={state.dialogOpen}
            onClose={closeDialog}
          >
            <UiHeading key="confirm-title" level="3">Confirm</UiHeading>
            <UiText key="confirm-copy">Ready to save</UiText>
          </UiDialog>
        </UiForm>
        "#,
    )
    .unwrap()
    .use_state("name", |state: &PrimitiveState| state.name.clone())
    .use_state("query", |state: &PrimitiveState| state.query.clone())
    .use_state("expanded", |state: &PrimitiveState| state.expanded)
    .use_state("dialogOpen", |state: &PrimitiveState| state.dialog_open)
    .use_state("progress", |state: &PrimitiveState| state.progress)
    .use_state("quota", |state: &PrimitiveState| state.quota)
    .use_value_reducer("setName", |state: &mut PrimitiveState, name: String| {
        state.name = name;
        Ok(())
    })
    .use_value_reducer("setQuery", |state: &mut PrimitiveState, query: String| {
        state.query = query;
        Ok(())
    })
    .use_reducer(
        "toggleAdvanced",
        |state: &mut PrimitiveState, _invocation| {
            state.expanded = !state.expanded;
            Ok(())
        },
    )
    .use_reducer("closeDialog", |state: &mut PrimitiveState, _invocation| {
        state.dialog_open = false;
        Ok(())
    })
    .use_reducer("submitProfile", |_state, _invocation| Ok(()))
    .use_reducer("resetProfile", |_state, _invocation| Ok(()))
    .use_reducer("invalidProfile", |_state, _invocation| Ok(()))
    .use_reducer("openDocs", |_state, _invocation| Ok(()))
    .use_reducer("saveProfile", |_state, _invocation| Ok(()));

    let frame = component
        .render(&PrimitiveState {
            name: "Grace".to_string(),
            query: "rsx".to_string(),
            expanded: true,
            dialog_open: true,
            progress: 40.0,
            quota: 80.0,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let form = find_element_by_attribute(&frame.root, "data-slot", "form").unwrap();
    let group = find_element_by_attribute(&frame.root, "data-slot", "group").unwrap();
    let heading = find_element_by_attribute(&frame.root, "data-slot", "heading").unwrap();
    let text = find_element_by_attribute(&frame.root, "data-slot", "text").unwrap();
    let text_field = find_element_by_attribute(&frame.root, "data-slot", "text-field").unwrap();
    let search_field = find_element_by_attribute(&frame.root, "data-slot", "search-field").unwrap();
    let field_set = find_element_by_attribute(&frame.root, "data-slot", "field-set").unwrap();
    let separator = find_element_by_attribute(&frame.root, "data-slot", "separator").unwrap();
    let toolbar = find_element_by_attribute(&frame.root, "data-slot", "toolbar").unwrap();
    let link = find_element_by_attribute(&frame.root, "data-slot", "link").unwrap();
    let disclosure = find_element_by_attribute(&frame.root, "data-slot", "disclosure").unwrap();
    let summary =
        find_element_by_attribute(&frame.root, "data-slot", "disclosure-summary").unwrap();
    let progress = find_element_by_attribute(&frame.root, "data-slot", "progress-bar").unwrap();
    let meter = find_element_by_attribute(&frame.root, "data-slot", "meter").unwrap();
    let dialog = find_element_by_attribute(&frame.root, "data-slot", "dialog").unwrap();

    assert_class_contains(group, "gap-3");
    assert_class_contains(text_field, "grid");
    assert_class_contains(separator, "data-[orientation=horizontal]:h-px");
    assert_class_contains(toolbar, "border");
    assert_class_excludes(dialog, "shadow-lg");
    assert_eq!(attribute_value(form, "data-invalid"), Some("true"));
    assert_eq!(
        attribute_value(form, "data-validation-behavior"),
        Some("aria")
    );
    assert_eq!(attribute_value(form, "noValidate"), Some("true"));

    let form_native = bridge.lower_to_native(form).unwrap();
    assert_eq!(form_native.role, NativeRole::Form);
    assert_eq!(form_native.props.label.as_deref(), Some("Profile"));
    assert!(form_native.props.invalid);
    assert!(form_native.props.form_no_validate);
    assert_eq!(
        form_native
            .props
            .web
            .events
            .get("onSubmit")
            .map(String::as_str),
        Some("submitProfile")
    );
    assert_eq!(
        form_native
            .props
            .web
            .events
            .get("onReset")
            .map(String::as_str),
        Some("resetProfile")
    );
    assert_eq!(
        form_native
            .props
            .web
            .events
            .get("onInvalid")
            .map(String::as_str),
        Some("invalidProfile")
    );

    let group_native = bridge.lower_to_native(group).unwrap();
    assert_eq!(group_native.role, NativeRole::View);
    assert_eq!(group_native.props.label.as_deref(), Some("Account"));

    let separator_native = bridge.lower_to_native(separator).unwrap();
    assert_eq!(separator_native.role, NativeRole::Separator);

    let heading_native = bridge.lower_to_native(heading).unwrap();
    assert_eq!(heading_native.role, NativeRole::Heading);
    assert_eq!(heading_native.props.label.as_deref(), Some("Profile"));
    assert_eq!(heading_native.props.accessibility_structure.level, Some(2));

    let text_native = bridge.lower_to_native(text).unwrap();
    assert_eq!(text_native.role, NativeRole::Text);
    assert_eq!(
        text_native.props.label.as_deref(),
        Some("Native RSX primitives")
    );

    let field_native = bridge.lower_to_native(text_field).unwrap();
    assert_eq!(field_native.role, NativeRole::TextField);
    assert_eq!(field_native.props.label.as_deref(), Some("Name"));
    assert_eq!(field_native.props.value.as_deref(), Some("Grace"));
    assert_eq!(field_native.props.action.as_deref(), Some("setName"));

    let search_native = bridge.lower_to_native(search_field).unwrap();
    assert_eq!(search_native.role, NativeRole::TextField);
    assert_eq!(search_native.props.value.as_deref(), Some("rsx"));
    assert_eq!(
        search_native
            .props
            .web
            .attributes
            .get("type")
            .map(String::as_str),
        Some("search")
    );

    let field_set_native = bridge.lower_to_native(field_set).unwrap();
    assert_eq!(field_set_native.role, NativeRole::FieldSet);
    assert_eq!(field_set_native.props.label.as_deref(), Some("Preferences"));

    let toolbar_native = bridge.lower_to_native(toolbar).unwrap();
    assert_eq!(toolbar_native.role, NativeRole::Toolbar);
    assert_eq!(toolbar_native.props.label.as_deref(), Some("Actions"));

    let link_native = bridge.lower_to_native(link).unwrap();
    assert_eq!(link_native.role, NativeRole::Link);
    assert_eq!(link_native.props.href.as_deref(), Some("/docs"));
    assert_eq!(link_native.props.action.as_deref(), Some("openDocs"));

    let disclosure_native = bridge.lower_to_native(disclosure).unwrap();
    assert_eq!(disclosure_native.role, NativeRole::Disclosure);
    assert_eq!(disclosure_native.props.expanded, Some(true));
    assert_eq!(
        disclosure_native
            .props
            .web
            .events
            .get("onExpandedChange")
            .map(String::as_str),
        Some("toggleAdvanced")
    );

    let summary_native = bridge.lower_to_native(summary).unwrap();
    assert_eq!(summary_native.role, NativeRole::DisclosureSummary);
    assert_eq!(
        summary_native.props.action.as_deref(),
        Some("toggleAdvanced")
    );

    let progress_native = bridge.lower_to_native(progress).unwrap();
    assert_eq!(progress_native.role, NativeRole::ProgressBar);
    assert_eq!(progress_native.props.label.as_deref(), Some("Sync"));
    assert_eq!(progress_native.props.current, Some(40.0));

    let meter_native = bridge.lower_to_native(meter).unwrap();
    assert_eq!(meter_native.role, NativeRole::Meter);
    assert_eq!(meter_native.props.current, Some(80.0));

    let dialog_native = bridge.lower_to_native(dialog).unwrap();
    assert_eq!(dialog_native.role, NativeRole::Dialog);
    assert_eq!(dialog_native.props.label.as_deref(), Some("Confirm"));
    assert_eq!(dialog_native.props.html_dialog.open, Some(true));
    assert_eq!(
        dialog_native
            .props
            .web
            .events
            .get("onClose")
            .map(String::as_str),
        Some("closeDialog")
    );
}

#[test]
fn rsx_ui_link_uses_link_hook_props() {
    let component = RsxComponent::<FormState>::new(
        "link",
        r#"
        <UiLink
          key="docs"
          href="/docs"
          onPress={openDocs}
          onPressStart={openDocsStart}
          onPressEnd={openDocsEnd}
          isDisabled={true}
          isPressed={true}
          actionValue="docs"
          actionPayload="docs-payload"
        >
          Docs
        </UiLink>
        "#,
    )
    .unwrap()
    .use_reducer("openDocs", |_state, _invocation| Ok(()))
    .use_reducer("openDocsStart", |_state, _invocation| Ok(()))
    .use_reducer("openDocsEnd", |_state, _invocation| Ok(()));

    let frame = component.render(&FormState::default()).unwrap();
    let link = find_element_by_attribute(&frame.root, "data-slot", "link").unwrap();
    let CompiledRsxNode::Element { props, .. } = link else {
        panic!("link element");
    };

    assert_eq!(props.href.as_deref(), Some("/docs"));
    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("openDocs")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("openDocsStart")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("openDocsEnd")
    );
    assert_eq!(attribute_value(link, "role"), Some("link"));
    assert_eq!(attribute_value(link, "tabIndex"), Some("-1"));
    assert_eq!(attribute_value(link, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(link, "data-pressed"), Some("true"));
    assert_eq!(attribute_value(link, "actionValue"), Some("docs"));
    assert_eq!(attribute_value(link, "actionPayload"), Some("docs-payload"));

    let native = RsxCompilerBridge::new().lower_to_native(link).unwrap();
    assert_eq!(native.role, NativeRole::Link);
    assert_eq!(native.props.href.as_deref(), Some("/docs"));
    assert_eq!(native.props.action.as_deref(), Some("openDocs"));
    assert!(native.props.disabled);
}

#[test]
fn rsx_ui_breadcrumb_uses_link_hook_props() {
    let component = RsxComponent::<FormState>::new(
        "breadcrumb",
        r#"
        <UiBreadcrumb
          key="home"
          href="/"
          onPress={openHome}
          onPressStart={openHomeStart}
          onPressEnd={openHomeEnd}
          isDisabled={true}
          isPressed={true}
          actionValue="home"
          actionPayload="home-payload"
        >
          Home
        </UiBreadcrumb>
        "#,
    )
    .unwrap()
    .use_reducer("openHome", |_state, _invocation| Ok(()))
    .use_reducer("openHomeStart", |_state, _invocation| Ok(()))
    .use_reducer("openHomeEnd", |_state, _invocation| Ok(()));

    let frame = component.render(&FormState::default()).unwrap();
    let breadcrumb = find_element_by_attribute(&frame.root, "data-slot", "breadcrumb").unwrap();
    let CompiledRsxNode::Element { props, .. } = breadcrumb else {
        panic!("breadcrumb element");
    };

    assert_eq!(props.href.as_deref(), Some("/"));
    assert!(props.is_disabled);
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("openHome")
    );
    assert_eq!(
        props.events.get("onPressStart").map(String::as_str),
        Some("openHomeStart")
    );
    assert_eq!(
        props.events.get("onPressEnd").map(String::as_str),
        Some("openHomeEnd")
    );
    assert_eq!(attribute_value(breadcrumb, "role"), Some("link"));
    assert_eq!(attribute_value(breadcrumb, "tabIndex"), Some("-1"));
    assert_eq!(attribute_value(breadcrumb, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(breadcrumb, "data-pressed"), Some("true"));
    assert_eq!(attribute_value(breadcrumb, "actionValue"), Some("home"));
    assert_eq!(
        attribute_value(breadcrumb, "actionPayload"),
        Some("home-payload")
    );

    let native = RsxCompilerBridge::new()
        .lower_to_native(breadcrumb)
        .unwrap();
    assert_eq!(native.role, NativeRole::Link);
    assert_eq!(native.props.href.as_deref(), Some("/"));
    assert_eq!(native.props.action.as_deref(), Some("openHome"));
    assert!(native.props.disabled);
}

#[test]
fn rsx_ui_renders_collection_overlay_and_toggle_primitives_to_native_roles() {
    let component = RsxComponent::new(
        "collections",
        r#"
        <UiGroup key="root" label="Collections">
          <UiCheckboxGroup key="channels" label="Channels">
            <UiCheckbox
              key="email"
              isChecked={state.compact}
              onChange={setCompact}
            >
              Email
            </UiCheckbox>
          </UiCheckboxGroup>
          <UiComboBox
            key="assignee"
            label="Assignee"
            value={state.assignee}
            inputValue={state.assigneeQuery}
            placeholder="Assignee"
            onChange={setAssigneeQuery}
            onSelectionChange={setAssignee}
          >
            <UiPopover key="assignee-popover">
              <UiOverlayArrow key="assignee-arrow" placement="bottom" />
              <UiListBox key="assignee-list">
                <UiListBoxItem
                  key="ada"
                  value="ada"
                  textValue="Ada"
                  isSelected={state.adaSelected}
                >
                  Ada
                </UiListBoxItem>
                <UiListBoxItem key="grace" value="grace" textValue="Grace">
                  Grace
                </UiListBoxItem>
              </UiListBox>
            </UiPopover>
          </UiComboBox>
          <UiNumberField
            key="quantity"
            label="Quantity"
            valueNumber={state.quantity}
            minValue="0"
            maxValue="100"
            stepValue="5"
            onChange={setQuantity}
          />
          <UiToggleButtonGroup
            key="view"
            label="View"
            value={state.view}
            onSelectionChange={setView}
          >
            <UiToggleButton
              key="compact"
              isSelected={state.compact}
              onPress={setCompact}
              actionValue="compact"
            >
              Compact
            </UiToggleButton>
          </UiToggleButtonGroup>
          <UiModal
            key="modal"
            label="Review"
            isOpen={state.modalOpen}
            onClose={closeModal}
          >
            <UiHeading key="modal-title" level="3">Review</UiHeading>
          </UiModal>
          <UiTooltip key="tip" label="Hint" isOpen={state.tooltipOpen}>
            Hint text
          </UiTooltip>
          <UiTable key="table" label="Members">
            <UiTableCaption key="caption">Team members</UiTableCaption>
            <UiTableHeader key="head">
              <UiTableRow key="head-row">
                <UiTableColumn key="name-column">Name</UiTableColumn>
                <UiTableColumn key="role-column">Role</UiTableColumn>
              </UiTableRow>
            </UiTableHeader>
            <UiTableBody key="body">
              <UiTableRow key="ada-row" isSelected={state.selectedRow}>
                <UiTableCell key="ada-name">Ada</UiTableCell>
                <UiTableCell key="ada-role">Compiler</UiTableCell>
              </UiTableRow>
            </UiTableBody>
          </UiTable>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("assignee", |state: &CollectionState| state.assignee.clone())
    .use_state("assigneeQuery", |state: &CollectionState| {
        state.assignee_query.clone()
    })
    .use_state("quantity", |state: &CollectionState| state.quantity)
    .use_state("compact", |state: &CollectionState| state.compact)
    .use_state("view", |state: &CollectionState| state.view.clone())
    .use_state("adaSelected", |state: &CollectionState| {
        state.assignee == "ada"
    })
    .use_state("selectedRow", |state: &CollectionState| state.selected_row)
    .use_state("modalOpen", |state: &CollectionState| state.modal_open)
    .use_state("tooltipOpen", |state: &CollectionState| state.tooltip_open)
    .use_value_reducer(
        "setAssigneeQuery",
        |state: &mut CollectionState, value: String| {
            state.assignee_query = value;
            Ok(())
        },
    )
    .use_value_reducer(
        "setAssignee",
        |state: &mut CollectionState, value: String| {
            state.assignee = value;
            Ok(())
        },
    )
    .use_value_reducer("setQuantity", |state: &mut CollectionState, value: f64| {
        state.quantity = value;
        Ok(())
    })
    .use_value_reducer("setView", |state: &mut CollectionState, value: String| {
        state.view = value;
        Ok(())
    })
    .use_value_reducer("setCompact", |state: &mut CollectionState, value: bool| {
        state.compact = value;
        Ok(())
    })
    .use_reducer("closeModal", |state: &mut CollectionState, _invocation| {
        state.modal_open = false;
        Ok(())
    });

    let frame = component
        .render(&CollectionState {
            assignee: "ada".to_string(),
            assignee_query: "Ada".to_string(),
            quantity: 35.0,
            compact: true,
            view: "compact".to_string(),
            selected_row: true,
            modal_open: true,
            tooltip_open: true,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let checkbox_group =
        find_element_by_attribute(&frame.root, "data-slot", "checkbox-group").unwrap();
    let combo_box = find_element_by_attribute(&frame.root, "data-slot", "combo-box").unwrap();
    let overlay_arrow =
        find_element_by_attribute(&frame.root, "data-slot", "overlay-arrow").unwrap();
    let number_field = find_element_by_attribute(&frame.root, "data-slot", "number-field").unwrap();
    let toggle_group =
        find_element_by_attribute(&frame.root, "data-slot", "toggle-button-group").unwrap();
    let toggle_button =
        find_element_by_attribute(&frame.root, "data-slot", "toggle-button").unwrap();
    let modal = find_element_by_attribute(&frame.root, "data-slot", "modal").unwrap();
    let tooltip = find_element_by_attribute(&frame.root, "data-slot", "tooltip").unwrap();
    let table = find_element_by_attribute(&frame.root, "data-slot", "table").unwrap();
    let row = find_element_by_attributes(
        &frame.root,
        &[("data-slot", "table-row"), ("data-selected", "true")],
    )
    .unwrap();
    let column = find_element_by_attribute(&frame.root, "data-slot", "table-column").unwrap();
    let cell = find_element_by_attribute(&frame.root, "data-slot", "table-cell").unwrap();

    assert_class_contains(combo_box, "grid");
    assert_class_contains(overlay_arrow, "data-[placement=bottom]:border-b");
    assert_class_contains(toggle_button, "data-[selected=true]:bg-surface-card");
    assert_class_contains(table, "w-full");

    let checkbox_group_native = bridge.lower_to_native(checkbox_group).unwrap();
    assert_eq!(checkbox_group_native.role, NativeRole::FieldSet);
    assert_eq!(
        checkbox_group_native.props.label.as_deref(),
        Some("Channels")
    );

    let combo_native = bridge.lower_to_native(combo_box).unwrap();
    assert_eq!(combo_native.role, NativeRole::ComboBox);
    assert_eq!(combo_native.props.label.as_deref(), Some("Assignee"));
    assert_eq!(combo_native.props.value.as_deref(), Some("ada"));
    assert_eq!(
        combo_native.props.action.as_deref(),
        Some("setAssigneeQuery")
    );
    assert_eq!(
        combo_native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setAssignee")
    );

    let overlay_arrow_native = bridge.lower_to_native(overlay_arrow).unwrap();
    assert_eq!(overlay_arrow_native.role, NativeRole::View);
    assert_eq!(
        overlay_arrow_native.props.explicit_role.as_deref(),
        Some("presentation")
    );
    assert_eq!(
        overlay_arrow_native
            .props
            .web
            .attributes
            .get("aria-hidden")
            .map(String::as_str),
        Some("true")
    );
    assert_eq!(combo_native.children.len(), 2);
    assert_eq!(combo_native.children[0].role, NativeRole::ListBoxItem);
    assert!(combo_native.children[0].props.selected);

    let number_native = bridge.lower_to_native(number_field).unwrap();
    assert_eq!(number_native.role, NativeRole::View);
    assert_eq!(number_native.props.label.as_deref(), Some("Quantity"));
    let number_controls = number_native
        .children
        .iter()
        .find(|child| child.role == NativeRole::View)
        .unwrap();
    let number_input = &number_controls.children[1];
    assert_eq!(number_input.role, NativeRole::TextField);
    assert_eq!(number_input.props.current, Some(35.0));
    assert_eq!(number_input.props.min, Some(0.0));
    assert_eq!(number_input.props.max, Some(100.0));
    assert_eq!(number_input.props.step, Some(5.0));
    assert_eq!(number_input.props.action.as_deref(), Some("setQuantity"));
    assert_eq!(
        number_input
            .props
            .web
            .attributes
            .get("type")
            .map(String::as_str),
        Some("number")
    );

    let toggle_group_native = bridge.lower_to_native(toggle_group).unwrap();
    assert_eq!(toggle_group_native.role, NativeRole::Toolbar);
    assert_eq!(toggle_group_native.props.label.as_deref(), Some("View"));
    assert_eq!(
        toggle_group_native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setView")
    );

    let toggle_button_native = bridge.lower_to_native(toggle_button).unwrap();
    assert_eq!(toggle_button_native.role, NativeRole::Button);
    assert!(toggle_button_native.props.selected);
    assert_eq!(
        toggle_button_native.props.action.as_deref(),
        Some("setCompact")
    );

    let modal_native = bridge.lower_to_native(modal).unwrap();
    assert_eq!(modal_native.role, NativeRole::Dialog);
    assert_eq!(modal_native.props.html_dialog.open, Some(true));
    assert_eq!(
        attribute_value(modal, "data-overlay-underlay"),
        Some("true")
    );
    assert_eq!(
        modal_native
            .props
            .web
            .events
            .get("onClose")
            .map(String::as_str),
        Some("closeModal")
    );

    let tooltip_native = bridge.lower_to_native(tooltip).unwrap();
    assert_eq!(tooltip_native.role, NativeRole::Popover);
    assert_eq!(attribute_value(tooltip, "data-overlay"), None);
    assert_eq!(tooltip_native.props.label.as_deref(), Some("Hint"));
    assert_eq!(
        tooltip_native
            .props
            .web
            .attributes
            .get("open")
            .map(String::as_str),
        Some("true")
    );

    let table_native = bridge.lower_to_native(table).unwrap();
    assert_eq!(table_native.role, NativeRole::Table);
    assert_eq!(table_native.props.label.as_deref(), Some("Members"));

    let row_native = bridge.lower_to_native(row).unwrap();
    assert_eq!(row_native.role, NativeRole::TableRow);
    assert!(row_native.props.selected);

    let column_native = bridge.lower_to_native(column).unwrap();
    assert_eq!(column_native.role, NativeRole::TableColumn);
    assert_eq!(column_native.props.label.as_deref(), Some("Name"));

    let cell_native = bridge.lower_to_native(cell).unwrap();
    assert_eq!(cell_native.role, NativeRole::TableCell);
    assert_eq!(cell_native.props.label.as_deref(), Some("Ada"));
}

#[test]
fn rsx_ui_renders_structure_collection_and_file_primitives_to_native_roles() {
    let component = RsxComponent::new(
        "structure",
        r#"
        <UiGroup key="root" label="Structure">
          <UiBreadcrumbs key="breadcrumbs" label="Path">
            <UiBreadcrumb key="home" href="/" onPress={openHome}>Home</UiBreadcrumb>
            <UiBreadcrumb key="settings" href="/settings" onPress={openSettings}>
              Settings
            </UiBreadcrumb>
          </UiBreadcrumbs>
          <UiAutocomplete
            key="search"
            label="Search"
            value={state.selected}
            inputValue={state.query}
            placeholder="Search"
            onChange={setQuery}
            onSelectionChange={setSelected}
          >
            <UiPopover key="search-popover">
              <UiListBox key="search-list">
                <UiListBoxItem
                  key="alpha"
                  value="alpha"
                  textValue="Alpha"
                  isSelected={state.alphaSelected}
                >
                  Alpha
                </UiListBoxItem>
              </UiListBox>
            </UiPopover>
          </UiAutocomplete>
          <UiGridList key="grid" label="Cards" value={state.selected} onSelectionChange={setSelected}>
            <UiGridListItem
              key="alpha-card"
              value="alpha"
              textValue="Alpha"
              isSelected={state.alphaSelected}
            >
              Alpha
            </UiGridListItem>
          </UiGridList>
          <UiTagGroup key="tags" label="Tags" value={state.selected} onSelectionChange={setSelected}>
            <UiTag
              key="alpha-tag"
              value="alpha"
              textValue="Alpha"
              isSelected={state.alphaSelected}
              onRemove={removeTag}
            >
              Alpha
            </UiTag>
          </UiTagGroup>
          <UiTree key="tree" label="Files" value={state.selected} onSelectionChange={setSelected}>
            <UiTreeItem
              key="src"
              value="src"
              textValue="src"
              isExpanded={state.treeExpanded}
              isSelected={state.srcSelected}
            >
              <UiTreeItemContent key="src-content">src</UiTreeItemContent>
            </UiTreeItem>
          </UiTree>
          <UiFileTrigger
            key="file"
            acceptedFileTypes=".rsx"
            allowsMultiple={true}
            onPress={openFilePicker}
            onSelect={selectFiles}
          >
            Upload RSX
          </UiFileTrigger>
          <UiDropZone
            key="drop"
            label="Drop files"
            onDrop={dropFiles}
            onDragEnter={dragEnter}
            onDragLeave={dragLeave}
          >
            Drop files
          </UiDropZone>
          <UiVirtualizer key="virtualizer" label="Virtual list">
            <UiText key="virtual-copy">Virtual rows</UiText>
          </UiVirtualizer>
          <UiToastRegion key="toasts" label="Notifications">
            <UiToast
              key="saved"
              title="Saved"
              description="Changes synced"
              onClose={dismissToast}
            >
              Saved
            </UiToast>
          </UiToastRegion>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("selected", |state: &StructureState| state.selected.clone())
    .use_state("query", |state: &StructureState| state.query.clone())
    .use_state("alphaSelected", |state: &StructureState| {
        state.selected == "alpha"
    })
    .use_state("srcSelected", |state: &StructureState| {
        state.selected == "src"
    })
    .use_state("treeExpanded", |state: &StructureState| {
        state.tree_expanded
    })
    .use_state("toastOpen", |state: &StructureState| state.toast_open)
    .use_value_reducer("setQuery", |state: &mut StructureState, query: String| {
        state.query = query;
        Ok(())
    })
    .use_value_reducer(
        "setSelected",
        |state: &mut StructureState, selected: String| {
            state.selected = selected;
            Ok(())
        },
    )
    .use_reducer("openHome", |_state, _invocation| Ok(()))
    .use_reducer("openSettings", |_state, _invocation| Ok(()))
    .use_reducer("removeTag", |_state, _invocation| Ok(()))
    .use_reducer("openFilePicker", |_state, _invocation| Ok(()))
    .use_reducer("selectFiles", |_state, _invocation| Ok(()))
    .use_reducer("dropFiles", |_state, _invocation| Ok(()))
    .use_reducer("dragEnter", |_state, _invocation| Ok(()))
    .use_reducer("dragLeave", |_state, _invocation| Ok(()))
    .use_reducer("dismissToast", |state: &mut StructureState, _invocation| {
        state.toast_open = false;
        Ok(())
    });

    let frame = component
        .render(&StructureState {
            selected: "alpha".to_string(),
            query: "alp".to_string(),
            tree_expanded: true,
            toast_open: true,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let breadcrumbs = find_element_by_attribute(&frame.root, "data-slot", "breadcrumbs").unwrap();
    let breadcrumb = find_element_by_attribute(&frame.root, "data-slot", "breadcrumb").unwrap();
    let autocomplete = find_element_by_attribute(&frame.root, "data-slot", "autocomplete").unwrap();
    let grid = find_element_by_attribute(&frame.root, "data-slot", "grid-list").unwrap();
    let grid_item = find_element_by_attribute(&frame.root, "data-slot", "grid-list-item").unwrap();
    let tag_group = find_element_by_attribute(&frame.root, "data-slot", "tag-group").unwrap();
    let tag = find_element_by_attribute(&frame.root, "data-slot", "tag").unwrap();
    let tree = find_element_by_attribute(&frame.root, "data-slot", "tree").unwrap();
    let tree_item = find_element_by_attribute(&frame.root, "data-slot", "tree-item").unwrap();
    let tree_content =
        find_element_by_attribute(&frame.root, "data-slot", "tree-item-content").unwrap();
    let file_trigger = find_element_by_attribute(&frame.root, "data-slot", "file-trigger").unwrap();
    let drop_zone = find_element_by_attribute(&frame.root, "data-slot", "drop-zone").unwrap();
    let virtualizer = find_element_by_attribute(&frame.root, "data-slot", "virtualizer").unwrap();
    let toast_region = find_element_by_attribute(&frame.root, "data-slot", "toast-region").unwrap();
    let toast = find_element_by_attribute(&frame.root, "data-slot", "toast").unwrap();

    let breadcrumbs_native = bridge.lower_to_native(breadcrumbs).unwrap();
    assert_eq!(breadcrumbs_native.role, NativeRole::Navigation);
    assert_eq!(breadcrumbs_native.props.label.as_deref(), Some("Path"));
    assert_eq!(
        attribute_value(breadcrumbs, "data-breadcrumbs"),
        Some("true")
    );

    let breadcrumb_native = bridge.lower_to_native(breadcrumb).unwrap();
    assert_eq!(breadcrumb_native.role, NativeRole::Link);
    assert_eq!(breadcrumb_native.props.href.as_deref(), Some("/"));
    assert_eq!(breadcrumb_native.props.action.as_deref(), Some("openHome"));

    let autocomplete_native = bridge.lower_to_native(autocomplete).unwrap();
    assert_eq!(autocomplete_native.role, NativeRole::ComboBox);
    assert_eq!(autocomplete_native.props.label.as_deref(), Some("Search"));
    assert_eq!(autocomplete_native.props.value.as_deref(), Some("alpha"));
    assert_eq!(
        autocomplete_native.props.action.as_deref(),
        Some("setQuery")
    );

    let grid_native = bridge.lower_to_native(grid).unwrap();
    assert_eq!(grid_native.role, NativeRole::ListBox);
    assert_eq!(grid_native.props.label.as_deref(), Some("Cards"));

    let grid_item_native = bridge.lower_to_native(grid_item).unwrap();
    assert_eq!(grid_item_native.role, NativeRole::ListBoxItem);
    assert_eq!(grid_item_native.props.value.as_deref(), Some("alpha"));
    assert!(grid_item_native.props.selected);

    let tag_group_native = bridge.lower_to_native(tag_group).unwrap();
    assert_eq!(tag_group_native.role, NativeRole::ListBox);
    assert_eq!(tag_group_native.props.label.as_deref(), Some("Tags"));

    let tag_native = bridge.lower_to_native(tag).unwrap();
    assert_eq!(tag_native.role, NativeRole::ListBoxItem);
    assert_eq!(
        tag_native
            .props
            .web
            .events
            .get("onRemove")
            .map(String::as_str),
        Some("removeTag")
    );

    let tree_native = bridge.lower_to_native(tree).unwrap();
    assert_eq!(tree_native.role, NativeRole::Tree);
    assert_eq!(tree_native.props.label.as_deref(), Some("Files"));

    let tree_item_native = bridge.lower_to_native(tree_item).unwrap();
    assert_eq!(tree_item_native.role, NativeRole::TreeItem);
    assert_eq!(tree_item_native.props.expanded, Some(true));

    let tree_content_native = bridge.lower_to_native(tree_content).unwrap();
    assert_eq!(tree_content_native.role, NativeRole::View);

    let file_native = bridge.lower_to_native(file_trigger).unwrap();
    assert_eq!(file_native.role, NativeRole::Button);
    assert_eq!(file_native.props.action.as_deref(), Some("openFilePicker"));
    assert_eq!(
        file_native
            .props
            .web
            .events
            .get("onSelect")
            .map(String::as_str),
        Some("selectFiles")
    );
    assert_eq!(
        file_native
            .props
            .web
            .attributes
            .get("accept")
            .map(String::as_str),
        Some(".rsx")
    );
    assert_eq!(
        file_native
            .props
            .web
            .attributes
            .get("multiple")
            .map(String::as_str),
        Some("true")
    );

    let drop_native = bridge.lower_to_native(drop_zone).unwrap();
    assert_eq!(drop_native.role, NativeRole::View);
    assert_eq!(drop_native.props.label.as_deref(), Some("Drop files"));
    assert_eq!(
        drop_native
            .props
            .web
            .events
            .get("onDrop")
            .map(String::as_str),
        Some("dropFiles")
    );

    let virtualizer_native = bridge.lower_to_native(virtualizer).unwrap();
    assert_eq!(virtualizer_native.role, NativeRole::View);
    assert_eq!(
        virtualizer_native.props.label.as_deref(),
        Some("Virtual list")
    );
    assert_eq!(
        attribute_value(virtualizer, "data-virtualizer"),
        Some("true")
    );
    assert_eq!(attribute_value(virtualizer, "data-layout"), Some("list"));
    assert_eq!(attribute_value(virtualizer, "data-item-count"), Some("0"));
    assert_eq!(
        virtualizer_native.props.explicit_role.as_deref(),
        Some("list")
    );

    let region_native = bridge.lower_to_native(toast_region).unwrap();
    assert_eq!(region_native.role, NativeRole::View);
    assert_eq!(region_native.props.label.as_deref(), Some("Notifications"));

    let toast_native = bridge.lower_to_native(toast).unwrap();
    assert_eq!(toast_native.role, NativeRole::View);
    assert_eq!(toast_native.props.label.as_deref(), Some("Saved"));
    assert_eq!(
        toast_native
            .props
            .web
            .events
            .get("onClose")
            .map(String::as_str),
        Some("dismissToast")
    );
}

#[test]
fn rsx_ui_renders_date_time_primitives_to_native_roles() {
    let component = RsxComponent::new(
        "date-time",
        r#"
        <UiGroup key="root" label="Date and time">
          <UiDateField
            key="date-field"
            label="Due date"
            value={state.date}
            placeholder="YYYY-MM-DD"
            granularity="day"
            onChange={setDate}
            isInvalid={state.dateInvalid}
          >
            <UiDescription key="date-help">Use ISO date format</UiDescription>
            <UiFieldError key="date-error">Required</UiFieldError>
          </UiDateField>
          <UiTimeField
            key="time-field"
            label="Reminder"
            value={state.time}
            granularity="minute"
            hourCycle="24"
            onChange={setTime}
          />
          <UiDateInput key="date-input" label="Segmented date" value={state.date}>
            <UiDateSegment
              key="month-segment"
              segmentType="month"
              value="07"
              textValue="07"
            />
            <UiDateSegment
              key="day-segment"
              segmentType="day"
              value="06"
              textValue="06"
              isPlaceholder={false}
            />
          </UiDateInput>
          <UiDatePicker
            key="date-picker"
            label="Ship date"
            value={state.date}
            placeholder="Pick a date"
            onChange={setDate}
            onOpenChange={toggleCalendar}
            isOpen={state.calendarOpen}
          >
            <UiPopover key="date-popover">
              <UiCalendar key="calendar" label="July 2026" value={state.date} onChange={setDate}>
                <UiGroup key="calendar-header">
                  <UiButton key="prev" variant="ghost" onPress={previousMonth}>Prev</UiButton>
                  <UiCalendarHeading key="heading" level="3">July 2026</UiCalendarHeading>
                  <UiButton key="next" variant="ghost" onPress={nextMonth}>Next</UiButton>
                </UiGroup>
                <UiGroup key="calendar-pickers">
                  <UiCalendarMonthPicker key="month" value={state.month} onSelectionChange={setMonth}>
                    <UiListBox key="months">
                      <UiListBoxItem key="july" value="7" textValue="July">July</UiListBoxItem>
                    </UiListBox>
                  </UiCalendarMonthPicker>
                  <UiCalendarYearPicker key="year" value={state.year} onSelectionChange={setYear}>
                    <UiListBox key="years">
                      <UiListBoxItem key="year-2026" value="2026" textValue="2026">2026</UiListBoxItem>
                    </UiListBox>
                  </UiCalendarYearPicker>
                </UiGroup>
                <UiCalendarGrid key="grid" label="July 2026">
                  <UiCalendarGridHeader key="grid-header">
                    <UiTableRow key="weekday-row">
                      <UiCalendarHeaderCell key="mon" textValue="Mon">Mon</UiCalendarHeaderCell>
                    </UiTableRow>
                  </UiCalendarGridHeader>
                  <UiCalendarGridBody key="grid-body">
                    <UiTableRow key="week-1">
                      <UiCalendarCell
                        key="day-6"
                        value="2026-07-06"
                        textValue="6"
                        actionValue="2026-07-06"
                        isSelected={state.dateSelected}
                        isToday={true}
                        onPress={setDate}
                      >
                        6
                      </UiCalendarCell>
                    </UiTableRow>
                  </UiCalendarGridBody>
                </UiCalendarGrid>
              </UiCalendar>
            </UiPopover>
          </UiDatePicker>
          <UiDateRangePicker
            key="range-picker"
            label="Sprint"
            startValue={state.rangeStart}
            endValue={state.rangeEnd}
            placeholder="YYYY-MM-DD"
            onStartChange={setRangeStart}
            onEndChange={setRangeEnd}
            onOpenChange={toggleCalendar}
            isOpen={state.calendarOpen}
          />
          <UiRangeCalendar
            key="range-calendar"
            label="Sprint calendar"
            startValue={state.rangeStart}
            endValue={state.rangeEnd}
            onChange={setRange}
          >
            <UiCalendarGrid key="range-grid" label="Range">
              <UiCalendarGridBody key="range-body">
                <UiTableRow key="range-row">
                  <UiCalendarCell
                    key="range-start"
                    value="2026-07-01"
                    textValue="1"
                    actionValue="2026-07-01"
                    isSelected={true}
                    onPress={setRange}
                  >
                    1
                  </UiCalendarCell>
                </UiTableRow>
              </UiCalendarGridBody>
            </UiCalendarGrid>
          </UiRangeCalendar>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("date", |state: &DateTimeState| state.date.clone())
    .use_state("time", |state: &DateTimeState| state.time.clone())
    .use_state("rangeStart", |state: &DateTimeState| {
        state.range_start.clone()
    })
    .use_state("rangeEnd", |state: &DateTimeState| {
        state.range_end.clone()
    })
    .use_state("month", |state: &DateTimeState| state.month.clone())
    .use_state("year", |state: &DateTimeState| state.year.clone())
    .use_state("calendarOpen", |state: &DateTimeState| {
        state.calendar_open
    })
    .use_state("dateInvalid", |state: &DateTimeState| state.date.is_empty())
    .use_state("dateSelected", |state: &DateTimeState| {
        state.date == "2026-07-06"
    })
    .use_value_reducer("setDate", |state: &mut DateTimeState, date: String| {
        state.date = date;
        Ok(())
    })
    .use_value_reducer("setTime", |state: &mut DateTimeState, time: String| {
        state.time = time;
        Ok(())
    })
    .use_value_reducer(
        "setRangeStart",
        |state: &mut DateTimeState, date: String| {
            state.range_start = date;
            Ok(())
        },
    )
    .use_value_reducer("setRangeEnd", |state: &mut DateTimeState, date: String| {
        state.range_end = date;
        Ok(())
    })
    .use_value_reducer("setMonth", |state: &mut DateTimeState, month: String| {
        state.month = month;
        Ok(())
    })
    .use_value_reducer("setYear", |state: &mut DateTimeState, year: String| {
        state.year = year;
        Ok(())
    })
    .use_reducer("setRange", |_state: &mut DateTimeState, _invocation| Ok(()))
    .use_reducer("toggleCalendar", |state: &mut DateTimeState, _invocation| {
        state.calendar_open = !state.calendar_open;
        Ok(())
    })
    .use_reducer("previousMonth", |_state: &mut DateTimeState, _invocation| {
        Ok(())
    })
    .use_reducer("nextMonth", |_state: &mut DateTimeState, _invocation| Ok(()));

    let frame = component
        .render(&DateTimeState {
            date: "2026-07-06".to_string(),
            time: "09:30".to_string(),
            range_start: "2026-07-01".to_string(),
            range_end: "2026-07-10".to_string(),
            month: "7".to_string(),
            year: "2026".to_string(),
            calendar_open: true,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let date_field = find_element_by_attribute(&frame.root, "data-slot", "date-field").unwrap();
    let date_field_input =
        find_element_by_attribute(&frame.root, "data-slot", "date-field-input").unwrap();
    let time_field = find_element_by_attribute(&frame.root, "data-slot", "time-field").unwrap();
    let date_input = find_element_by_attribute(&frame.root, "data-slot", "date-input").unwrap();
    let date_segment = find_element_by_attribute(&frame.root, "data-slot", "date-segment").unwrap();
    let date_picker = find_element_by_attribute(&frame.root, "data-slot", "date-picker").unwrap();
    let range_picker =
        find_element_by_attribute(&frame.root, "data-slot", "date-range-picker").unwrap();
    let calendar = find_element_by_attribute(&frame.root, "data-slot", "calendar").unwrap();
    let heading = find_element_by_attribute(&frame.root, "data-slot", "calendar-heading").unwrap();
    let month_picker =
        find_element_by_attribute(&frame.root, "data-slot", "calendar-month-picker").unwrap();
    let year_picker =
        find_element_by_attribute(&frame.root, "data-slot", "calendar-year-picker").unwrap();
    let grid = find_element_by_attribute(&frame.root, "data-slot", "calendar-grid").unwrap();
    let grid_header =
        find_element_by_attribute(&frame.root, "data-slot", "calendar-grid-header").unwrap();
    let header_cell =
        find_element_by_attribute(&frame.root, "data-slot", "calendar-header-cell").unwrap();
    let grid_body =
        find_element_by_attribute(&frame.root, "data-slot", "calendar-grid-body").unwrap();
    let selected_day = find_element_by_attributes(
        &frame.root,
        &[("data-slot", "calendar-cell"), ("data-today", "true")],
    )
    .unwrap();
    let range_calendar =
        find_element_by_attribute(&frame.root, "data-slot", "range-calendar").unwrap();
    let field_error = find_element_by_attribute(&frame.root, "data-slot", "field-error").unwrap();
    let description = find_element_by_attribute(&frame.root, "data-slot", "description").unwrap();

    assert_class_contains(date_field_input, "border-hairline-strong");
    assert_class_contains(selected_day, "data-[selected=true]:bg-ink");

    let date_native = bridge.lower_to_native(date_field).unwrap();
    assert_eq!(date_native.role, NativeRole::TextField);
    assert_eq!(date_native.props.label.as_deref(), Some("Due date"));
    assert_eq!(date_native.props.value.as_deref(), Some("2026-07-06"));
    assert_eq!(date_native.props.action.as_deref(), Some("setDate"));
    assert_eq!(
        date_native
            .props
            .web
            .attributes
            .get("type")
            .map(String::as_str),
        Some("date")
    );

    let time_native = bridge.lower_to_native(time_field).unwrap();
    assert_eq!(time_native.role, NativeRole::TextField);
    assert_eq!(time_native.props.value.as_deref(), Some("09:30"));
    assert_eq!(
        time_native
            .props
            .web
            .attributes
            .get("type")
            .map(String::as_str),
        Some("time")
    );

    let date_input_native = bridge.lower_to_native(date_input).unwrap();
    assert_eq!(date_input_native.role, NativeRole::View);
    assert_eq!(
        date_input_native.props.label.as_deref(),
        Some("Segmented date")
    );
    assert_eq!(
        attribute_value(date_input, "data-value"),
        Some("2026-07-06")
    );

    let segment_native = bridge.lower_to_native(date_segment).unwrap();
    assert_eq!(segment_native.role, NativeRole::Text);
    assert_eq!(segment_native.props.label.as_deref(), Some("07"));

    let picker_native = bridge.lower_to_native(date_picker).unwrap();
    assert_eq!(picker_native.role, NativeRole::View);
    assert_eq!(picker_native.props.label.as_deref(), Some("Ship date"));
    assert_eq!(
        picker_native
            .props
            .web
            .events
            .get("onOpenChange")
            .map(String::as_str),
        Some("toggleCalendar")
    );

    let range_picker_native = bridge.lower_to_native(range_picker).unwrap();
    assert_eq!(range_picker_native.role, NativeRole::View);
    assert_eq!(range_picker_native.props.label.as_deref(), Some("Sprint"));
    assert_eq!(
        range_picker_native
            .props
            .web
            .events
            .get("onOpenChange")
            .map(String::as_str),
        Some("toggleCalendar")
    );

    let calendar_native = bridge.lower_to_native(calendar).unwrap();
    assert_eq!(calendar_native.role, NativeRole::View);
    assert_eq!(calendar_native.props.label.as_deref(), Some("July 2026"));
    assert_eq!(attribute_value(calendar, "data-value"), Some("2026-07-06"));

    let heading_native = bridge.lower_to_native(heading).unwrap();
    assert_eq!(heading_native.role, NativeRole::Heading);
    assert_eq!(heading_native.props.accessibility_structure.level, Some(3));
    assert_eq!(attribute_value(heading, "aria-level"), Some("3"));

    let month_native = bridge.lower_to_native(month_picker).unwrap();
    assert_eq!(month_native.role, NativeRole::Select);
    assert_eq!(month_native.props.value.as_deref(), Some("7"));

    let year_native = bridge.lower_to_native(year_picker).unwrap();
    assert_eq!(year_native.role, NativeRole::Select);
    assert_eq!(year_native.props.value.as_deref(), Some("2026"));

    let grid_native = bridge.lower_to_native(grid).unwrap();
    assert_eq!(grid_native.role, NativeRole::Table);

    let grid_header_native = bridge.lower_to_native(grid_header).unwrap();
    assert_eq!(grid_header_native.role, NativeRole::TableSection);
    assert_eq!(attribute_value(grid_header, "role"), Some("rowgroup"));
    assert_eq!(
        attribute_value(grid_header, "data-table-section"),
        Some("header")
    );

    let header_cell_native = bridge.lower_to_native(header_cell).unwrap();
    assert_eq!(header_cell_native.role, NativeRole::TableColumn);
    assert_eq!(header_cell_native.props.label.as_deref(), Some("Mon"));

    let grid_body_native = bridge.lower_to_native(grid_body).unwrap();
    assert_eq!(grid_body_native.role, NativeRole::TableSection);
    assert_eq!(attribute_value(grid_body, "role"), Some("rowgroup"));
    assert_eq!(
        attribute_value(grid_body, "data-table-section"),
        Some("body")
    );

    let selected_day_native = bridge.lower_to_native(selected_day).unwrap();
    assert_eq!(selected_day_native.role, NativeRole::Button);
    assert_eq!(
        selected_day_native.props.value.as_deref(),
        Some("2026-07-06")
    );
    assert_eq!(selected_day_native.props.action.as_deref(), Some("setDate"));
    assert_eq!(
        selected_day_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("2026-07-06")
    );
    assert!(selected_day_native.props.selected);

    let range_calendar_native = bridge.lower_to_native(range_calendar).unwrap();
    assert_eq!(range_calendar_native.role, NativeRole::View);
    assert_eq!(
        range_calendar_native.props.label.as_deref(),
        Some("Sprint calendar")
    );
    assert_eq!(
        attribute_value(range_calendar, "data-start-value"),
        Some("2026-07-01")
    );
    assert_eq!(
        attribute_value(range_calendar, "data-end-value"),
        Some("2026-07-10")
    );

    let error_native = bridge.lower_to_native(field_error).unwrap();
    assert_eq!(error_native.role, NativeRole::Text);
    assert_eq!(error_native.props.label.as_deref(), Some("Required"));

    let description_native = bridge.lower_to_native(description).unwrap();
    assert_eq!(description_native.role, NativeRole::Text);
    assert_eq!(
        description_native.props.label.as_deref(),
        Some("Use ISO date format")
    );
}

#[test]
fn rsx_ui_renders_color_primitives_to_native_roles() {
    let component = RsxComponent::new(
        "color",
        r##"
        <UiColorPicker
          key="picker"
          label="Accent color"
          value={state.color}
          onChange={setColor}
        >
          <UiColorArea
            key="area"
            label="Saturation and brightness"
            value={state.color}
            xChannel="saturation"
            yChannel="brightness"
            xValue={state.saturation}
            yValue={state.brightness}
            onChange={setColor}
          />
          <UiColorSlider
            key="hue-slider"
            label="Hue"
            channel="hue"
            valueNumber={state.hue}
            minValue="0"
            maxValue="360"
            stepValue="1"
            onChange={setHue}
          />
          <UiColorWheel
            key="wheel"
            label="Hue wheel"
            valueNumber={state.hue}
            onChange={setHue}
          />
          <UiColorField
            key="field"
            label="Hex"
            value={state.color}
            placeholder="#000000"
            colorSpace="srgb"
            onChange={setColor}
          >
            <UiDescription key="field-help">Use a hex color</UiDescription>
          </UiColorField>
          <UiColorSwatch
            key="preview"
            label="Preview"
            value={state.color}
          />
          <UiColorSwatchPicker
            key="swatches"
            label="Saved colors"
            value={state.color}
            onSelectionChange={setColor}
          >
            <UiColorSwatchPickerItem
              key="preview"
              value="#8145b5"
              textValue="Preview"
              isSelected={state.previewSelected}
            >
              <UiColorSwatch key="preview-swatch" value="#8145b5" label="Preview" />
            </UiColorSwatchPickerItem>
          </UiColorSwatchPicker>
        </UiColorPicker>
        "##,
    )
    .unwrap()
    .use_state("color", |state: &ColorState| state.color.clone())
    .use_state("hue", |state: &ColorState| state.hue)
    .use_state("saturation", |state: &ColorState| state.saturation)
    .use_state("brightness", |state: &ColorState| state.brightness)
    .use_state("previewSelected", |state: &ColorState| {
        state.color == "#8145b5"
    })
    .use_value_reducer("setColor", |state: &mut ColorState, color: String| {
        state.color = color;
        Ok(())
    })
    .use_value_reducer("setHue", |state: &mut ColorState, hue: f64| {
        state.hue = hue;
        Ok(())
    });

    let frame = component
        .render(&ColorState {
            color: "#8145b5".to_string(),
            hue: 271.0,
            saturation: 62.0,
            brightness: 71.0,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let picker = find_element_by_attribute(&frame.root, "data-slot", "color-picker").unwrap();
    let area = find_element_by_attribute(&frame.root, "data-slot", "color-area").unwrap();
    let slider = find_element_by_attribute(&frame.root, "data-slot", "color-slider").unwrap();
    let wheel = find_element_by_attribute(&frame.root, "data-slot", "color-wheel").unwrap();
    let field = find_element_by_attribute(&frame.root, "data-slot", "color-field").unwrap();
    let field_input =
        find_element_by_attribute(&frame.root, "data-slot", "color-field-input").unwrap();
    let swatch = find_element_by_attribute(&frame.root, "data-slot", "color-swatch").unwrap();
    let swatch_picker =
        find_element_by_attribute(&frame.root, "data-slot", "color-swatch-picker").unwrap();
    let swatch_item =
        find_element_by_attribute(&frame.root, "data-slot", "color-swatch-picker-item").unwrap();

    assert_class_contains(picker, "rounded-md");
    assert_class_contains(area, "h-40");
    assert_class_contains(field_input, "font-mono");
    assert_class_contains(swatch_item, "data-[selected=true]:ring-[2px]");

    let picker_native = bridge.lower_to_native(picker).unwrap();
    assert_eq!(picker_native.role, NativeRole::View);
    assert_eq!(picker_native.props.label.as_deref(), Some("Accent color"));
    assert_eq!(attribute_value(picker, "data-value"), Some("#8145b5"));
    assert_eq!(
        picker_native
            .props
            .web
            .events
            .get("onChange")
            .map(String::as_str),
        Some("setColor")
    );

    let area_native = bridge.lower_to_native(area).unwrap();
    assert_eq!(area_native.role, NativeRole::View);
    assert_eq!(
        area_native.props.label.as_deref(),
        Some("Saturation and brightness")
    );
    assert_eq!(attribute_value(area, "data-x-channel"), Some("saturation"));
    assert_eq!(attribute_value(area, "data-y-channel"), Some("brightness"));
    assert_eq!(attribute_value(area, "data-x-value"), Some("62.0"));
    assert_eq!(attribute_value(area, "data-y-value"), Some("71.0"));

    let slider_native = bridge.lower_to_native(slider).unwrap();
    assert_eq!(slider_native.role, NativeRole::Slider);
    assert_eq!(slider_native.props.label.as_deref(), Some("Hue"));
    assert_eq!(slider_native.props.current, Some(271.0));
    assert_eq!(slider_native.props.min, Some(0.0));
    assert_eq!(slider_native.props.max, Some(360.0));
    assert_eq!(slider_native.props.step, Some(1.0));
    assert_eq!(slider_native.props.action.as_deref(), Some("setHue"));
    assert_eq!(attribute_value(slider, "data-channel"), Some("hue"));
    assert_eq!(attribute_value(slider, "aria-valuenow"), Some("271.0"));
    let slider_percent = attribute_value(slider, "data-value-percent")
        .unwrap()
        .parse::<f64>()
        .unwrap();
    assert!((slider_percent - 75.277).abs() < 0.01);

    let wheel_native = bridge.lower_to_native(wheel).unwrap();
    assert_eq!(wheel_native.role, NativeRole::View);
    assert_eq!(wheel_native.props.label.as_deref(), Some("Hue wheel"));
    assert_eq!(attribute_value(wheel, "data-channel"), Some("hue"));
    assert_eq!(attribute_value(wheel, "data-value"), Some("271.0"));

    let field_native = bridge.lower_to_native(field).unwrap();
    assert_eq!(field_native.role, NativeRole::TextField);
    assert_eq!(field_native.props.label.as_deref(), Some("Hex"));
    assert_eq!(field_native.props.value.as_deref(), Some("#8145b5"));
    assert_eq!(field_native.props.action.as_deref(), Some("setColor"));
    assert_eq!(attribute_value(field, "data-color-space"), Some("srgb"));
    let CompiledRsxNode::Element {
        props: field_input_props,
        ..
    } = field_input
    else {
        panic!("color field input element")
    };
    assert_eq!(
        field_input_props.events.get("onInput").map(String::as_str),
        Some("setColor")
    );

    let swatch_native = bridge.lower_to_native(swatch).unwrap();
    assert_eq!(swatch_native.role, NativeRole::View);
    assert_eq!(swatch_native.props.label.as_deref(), Some("Preview"));
    assert_eq!(attribute_value(swatch, "data-value"), Some("#8145b5"));

    let picker_native = bridge.lower_to_native(swatch_picker).unwrap();
    assert_eq!(picker_native.role, NativeRole::ListBox);
    assert_eq!(picker_native.props.label.as_deref(), Some("Saved colors"));
    assert_eq!(picker_native.props.value.as_deref(), Some("#8145b5"));
    assert_eq!(
        picker_native
            .props
            .web
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setColor")
    );
    assert_eq!(
        attribute_value(swatch_picker, "data-selected-value"),
        Some("#8145b5")
    );
    assert_eq!(
        attribute_value(swatch_picker, "data-selection-mode"),
        Some("single")
    );

    let item_native = bridge.lower_to_native(swatch_item).unwrap();
    assert_eq!(item_native.role, NativeRole::ListBoxItem);
    assert_eq!(item_native.props.value.as_deref(), Some("#8145b5"));
    assert!(item_native.props.selected);
    assert_eq!(attribute_value(swatch_item, "data-selected"), Some("true"));
}

#[test]
fn rsx_ui_renders_layout_slider_and_color_parts_to_native_roles() {
    let component = RsxComponent::new(
        "parts",
        r##"
        <UiSection key="settings" label="Audio settings" className="rounded-md border">
          <UiHeader key="header" label="Audio header">
            <UiHeading key="title" level="2">Audio</UiHeading>
          </UiHeader>
          <UiSlider
            key="volume"
            label="Volume"
            valueNumber={state.volume}
            minValue="0"
            maxValue="100"
            stepValue="1"
            onChange={setVolume}
          >
            <UiSliderOutput
              key="volume-output"
              label="Volume output"
              value="42%"
              valueNumber={state.volume}
            />
            <UiSliderTrack key="track" orientation="horizontal">
              <UiSliderFill
                key="fill"
                orientation="horizontal"
                valueNumber={state.volume}
              />
            </UiSliderTrack>
            <UiSliderThumb
              key="thumb"
              valueNumber={state.volume}
              isDragging={state.dragging}
              onPress={commitVolume}
              actionValue="volume"
            />
          </UiSlider>
          <UiColorArea
            key="area"
            label="Saturation and brightness"
            value={state.color}
            xValue={state.saturation}
            yValue={state.brightness}
            onChange={setColor}
          >
            <UiColorThumb
              key="color-thumb"
              value={state.color}
              xValue={state.saturation}
              yValue={state.brightness}
              isDragging={state.dragging}
              onPress={commitColor}
              actionValue={state.color}
            />
          </UiColorArea>
          <UiFooter key="footer" label="Audio footer">
            <UiText key="status">Ready</UiText>
          </UiFooter>
        </UiSection>
        "##,
    )
    .unwrap()
    .use_state("volume", |state: &ComponentPartsState| state.volume)
    .use_state("color", |state: &ComponentPartsState| state.color.clone())
    .use_state("saturation", |state: &ComponentPartsState| state.saturation)
    .use_state("brightness", |state: &ComponentPartsState| state.brightness)
    .use_state("dragging", |state: &ComponentPartsState| state.dragging)
    .use_value_reducer(
        "setVolume",
        |state: &mut ComponentPartsState, volume: f64| {
            state.volume = volume;
            Ok(())
        },
    )
    .use_value_reducer(
        "setColor",
        |state: &mut ComponentPartsState, color: String| {
            state.color = color;
            Ok(())
        },
    )
    .use_reducer(
        "commitVolume",
        |state: &mut ComponentPartsState, _invocation| {
            state.volume_committed = true;
            Ok(())
        },
    )
    .use_reducer(
        "commitColor",
        |state: &mut ComponentPartsState, _invocation| {
            state.color_committed = true;
            Ok(())
        },
    );

    let frame = component
        .render(&ComponentPartsState {
            volume: 42.0,
            color: "#8145b5".to_string(),
            saturation: 62.0,
            brightness: 71.0,
            dragging: true,
            volume_committed: false,
            color_committed: false,
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let section = find_element_by_attribute(&frame.root, "data-slot", "section").unwrap();
    let header = find_element_by_attribute(&frame.root, "data-slot", "header").unwrap();
    let footer = find_element_by_attribute(&frame.root, "data-slot", "footer").unwrap();
    let track = find_element_by_attribute(&frame.root, "data-slot", "slider-track").unwrap();
    let fill = find_element_by_attribute(&frame.root, "data-slot", "slider-fill").unwrap();
    let output = find_element_by_attribute(&frame.root, "data-slot", "slider-output").unwrap();
    let thumb = find_element_by_attribute(&frame.root, "data-slot", "slider-thumb").unwrap();
    let color_thumb = find_element_by_attribute(&frame.root, "data-slot", "color-thumb").unwrap();

    assert_class_contains(section, "rounded-md");
    assert_class_contains(track, "bg-surface-strong");
    assert_class_contains(fill, "bg-ink");
    assert_class_contains(thumb, "rounded-full");
    assert_class_contains(color_thumb, "ring-hairline-strong");

    let section_native = bridge.lower_to_native(section).unwrap();
    assert_eq!(section_native.role, NativeRole::Section);
    assert_eq!(
        section_native.props.label.as_deref(),
        Some("Audio settings")
    );

    let header_native = bridge.lower_to_native(header).unwrap();
    assert_eq!(header_native.role, NativeRole::Header);
    assert_eq!(header_native.props.label.as_deref(), Some("Audio header"));

    let footer_native = bridge.lower_to_native(footer).unwrap();
    assert_eq!(footer_native.role, NativeRole::Footer);
    assert_eq!(footer_native.props.label.as_deref(), Some("Audio footer"));

    let track_native = bridge.lower_to_native(track).unwrap();
    assert_eq!(track_native.role, NativeRole::View);
    assert_eq!(
        attribute_value(track, "data-orientation"),
        Some("horizontal")
    );

    let fill_native = bridge.lower_to_native(fill).unwrap();
    assert_eq!(fill_native.role, NativeRole::View);
    assert_eq!(attribute_value(fill, "data-value-number"), Some("42.0"));

    let output_native = bridge.lower_to_native(output).unwrap();
    assert_eq!(output_native.role, NativeRole::Output);
    assert_eq!(output_native.props.label.as_deref(), Some("Volume output"));
    assert_eq!(output_native.props.value.as_deref(), Some("42%"));
    assert_eq!(attribute_value(output, "data-value-number"), Some("42.0"));

    let thumb_native = bridge.lower_to_native(thumb).unwrap();
    assert_eq!(thumb_native.role, NativeRole::Button);
    assert_eq!(thumb_native.props.action.as_deref(), Some("commitVolume"));
    assert_eq!(
        thumb_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("volume")
    );
    assert_eq!(attribute_value(thumb, "data-dragging"), Some("true"));
    assert_eq!(attribute_value(thumb, "data-value-number"), Some("42.0"));

    let color_thumb_native = bridge.lower_to_native(color_thumb).unwrap();
    assert_eq!(color_thumb_native.role, NativeRole::Button);
    assert_eq!(
        color_thumb_native.props.action.as_deref(),
        Some("commitColor")
    );
    assert_eq!(
        color_thumb_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("#8145b5")
    );
    assert_eq!(attribute_value(color_thumb, "data-value"), Some("#8145b5"));
    assert_eq!(attribute_value(color_thumb, "data-x-value"), Some("62.0"));
    assert_eq!(attribute_value(color_thumb, "data-y-value"), Some("71.0"));
    assert_eq!(attribute_value(color_thumb, "data-dragging"), Some("true"));
}

#[test]
fn rsx_ui_renders_trigger_collection_and_disclosure_parts_to_native_roles() {
    let component = RsxComponent::new(
        "semantic-parts",
        r#"
        <UiGroup key="root" label="Semantic parts">
          <UiMenuTrigger
            key="menu-trigger"
            isOpen={state.menuOpen}
            onPress={toggleMenu}
            actionValue="menu"
          >
            Actions
          </UiMenuTrigger>
          <UiMenu key="menu">
            <UiMenuSection key="menu-section" label="File actions">
              <UiMenuItem key="open" onAction={openFile} actionValue="open">
                Open
                <UiKeyboard key="open-shortcut" textValue="Cmd+O">Cmd+O</UiKeyboard>
              </UiMenuItem>
              <UiSubmenuTrigger
                key="submenu-trigger"
                isOpen={state.menuOpen}
                onPress={toggleMenu}
                actionValue="more"
              >
                More
              </UiSubmenuTrigger>
            </UiMenuSection>
          </UiMenu>
          <UiListBox key="people" onSelectionChange={selectItem}>
            <UiListBoxSection key="people-section" label="People">
              <UiListBoxHeader key="people-header" textValue="People">People</UiListBoxHeader>
              <UiListBoxItem
                key="ada"
                value="ada"
                textValue="Ada"
                isSelected={state.adaSelected}
              >
                <UiSelectionIndicator
                  key="ada-selected"
                  isSelected={state.adaSelected}
                  label="Selected"
                >
                  selected
                </UiSelectionIndicator>
                Ada
              </UiListBoxItem>
            </UiListBoxSection>
          </UiListBox>
          <UiGridList key="files" label="Files" onSelectionChange={selectItem}>
            <UiGridListSection key="files-section" label="Source files">
              <UiGridListHeader key="files-header" textValue="Source files">Source files</UiGridListHeader>
              <UiGridListItem key="main" value="main" textValue="main.rs">main.rs</UiGridListItem>
            </UiGridListSection>
          </UiGridList>
          <UiTree key="project" label="Project" onSelectionChange={selectItem}>
            <UiTreeSection key="project-section" label="Workspace">
              <UiTreeHeader key="project-header" textValue="Workspace">Workspace</UiTreeHeader>
              <UiTreeItem key="src" value="src" textValue="src" isExpanded={true}>
                <UiTreeItemContent key="src-content">src</UiTreeItemContent>
              </UiTreeItem>
            </UiTreeSection>
          </UiTree>
          <UiDialogTrigger
            key="dialog-trigger"
            isOpen={state.dialogOpen}
            onPress={openDialog}
            actionValue="dialog"
          >
            Open dialog
          </UiDialogTrigger>
          <UiDialog key="dialog" label="Preferences" isOpen={state.dialogOpen} onClose={closeDialog}>
            Preferences
          </UiDialog>
          <UiTooltipTrigger
            key="tooltip-trigger"
            isOpen={state.tooltipOpen}
            onPress={toggleTooltip}
            actionValue="help"
          >
            ?
          </UiTooltipTrigger>
          <UiTooltip key="tooltip" label="Help" isOpen={state.tooltipOpen}>Help</UiTooltip>
          <UiDisclosure
            key="disclosure"
            label="Details"
            isExpanded={state.disclosureExpanded}
            onExpandedChange={toggleDisclosure}
          >
            <UiDisclosureSummary key="summary" onPress={toggleDisclosure}>Details</UiDisclosureSummary>
            <UiDisclosurePanel
              key="panel"
              label="Details panel"
              isExpanded={state.disclosureExpanded}
            >
              More information
            </UiDisclosurePanel>
          </UiDisclosure>
        </UiGroup>
        "#,
    )
    .unwrap()
    .use_state("menuOpen", |state: &SemanticPartsState| state.menu_open)
    .use_state("dialogOpen", |state: &SemanticPartsState| {
        state.dialog_open
    })
    .use_state("tooltipOpen", |state: &SemanticPartsState| {
        state.tooltip_open
    })
    .use_state("disclosureExpanded", |state: &SemanticPartsState| {
        state.disclosure_expanded
    })
    .use_state("adaSelected", |state: &SemanticPartsState| {
        state.selected == "ada"
    })
    .use_value_reducer(
        "selectItem",
        |state: &mut SemanticPartsState, selected: String| {
            state.selected = selected;
            Ok(())
        },
    )
    .use_reducer("toggleMenu", |state: &mut SemanticPartsState, _invocation| {
        state.menu_open = !state.menu_open;
        Ok(())
    })
    .use_reducer("openFile", |_state: &mut SemanticPartsState, _invocation| {
        Ok(())
    })
    .use_reducer("openDialog", |state: &mut SemanticPartsState, _invocation| {
        state.dialog_open = true;
        Ok(())
    })
    .use_reducer("closeDialog", |state: &mut SemanticPartsState, _invocation| {
        state.dialog_open = false;
        Ok(())
    })
    .use_reducer(
        "toggleTooltip",
        |state: &mut SemanticPartsState, _invocation| {
            state.tooltip_open = !state.tooltip_open;
            Ok(())
        },
    )
    .use_reducer(
        "toggleDisclosure",
        |state: &mut SemanticPartsState, _invocation| {
            state.disclosure_expanded = !state.disclosure_expanded;
            Ok(())
        },
    );

    let frame = component
        .render(&SemanticPartsState {
            menu_open: true,
            dialog_open: true,
            tooltip_open: true,
            disclosure_expanded: true,
            selected: "ada".to_string(),
        })
        .unwrap();

    let bridge = RsxCompilerBridge::new();
    let menu_trigger = find_element_by_attribute(&frame.root, "data-slot", "menu-trigger").unwrap();
    let menu_section = find_element_by_attribute(&frame.root, "data-slot", "menu-section").unwrap();
    let submenu_trigger =
        find_element_by_attribute(&frame.root, "data-slot", "submenu-trigger").unwrap();
    let keyboard = find_element_by_attribute(&frame.root, "data-slot", "keyboard").unwrap();
    let selection_indicator =
        find_element_by_attribute(&frame.root, "data-slot", "selection-indicator").unwrap();
    let list_box_section =
        find_element_by_attribute(&frame.root, "data-slot", "list-box-section").unwrap();
    let list_box_header =
        find_element_by_attribute(&frame.root, "data-slot", "list-box-header").unwrap();
    let grid_list_section =
        find_element_by_attribute(&frame.root, "data-slot", "grid-list-section").unwrap();
    let grid_list_header =
        find_element_by_attribute(&frame.root, "data-slot", "grid-list-header").unwrap();
    let tree_section = find_element_by_attribute(&frame.root, "data-slot", "tree-section").unwrap();
    let tree_header = find_element_by_attribute(&frame.root, "data-slot", "tree-header").unwrap();
    let dialog_trigger =
        find_element_by_attribute(&frame.root, "data-slot", "dialog-trigger").unwrap();
    let tooltip_trigger =
        find_element_by_attribute(&frame.root, "data-slot", "tooltip-trigger").unwrap();
    let disclosure_panel =
        find_element_by_attribute(&frame.root, "data-slot", "disclosure-panel").unwrap();

    assert_class_contains(menu_trigger, "border-hairline-strong");
    assert_class_contains(submenu_trigger, "justify-between");
    assert_class_contains(selection_indicator, "data-[selected=true]:opacity-100");
    assert_class_contains(keyboard, "font-mono");
    assert_class_contains(disclosure_panel, "text-body");

    let menu_trigger_native = bridge.lower_to_native(menu_trigger).unwrap();
    assert_eq!(menu_trigger_native.role, NativeRole::Button);
    assert_eq!(
        menu_trigger_native.props.action.as_deref(),
        Some("toggleMenu")
    );
    assert_eq!(
        menu_trigger_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("menu")
    );
    assert_eq!(attribute_value(menu_trigger, "data-open"), Some("true"));
    assert_eq!(attribute_value(menu_trigger, "aria-haspopup"), Some("menu"));

    let menu_section_native = bridge.lower_to_native(menu_section).unwrap();
    assert_eq!(menu_section_native.role, NativeRole::Section);
    assert_eq!(
        menu_section_native.props.label.as_deref(),
        Some("File actions")
    );
    assert_eq!(
        attribute_value(menu_section, "data-collection-section"),
        Some("true")
    );
    assert_eq!(
        attribute_value(menu_section, "data-collection-kind"),
        Some("menu")
    );

    let submenu_native = bridge.lower_to_native(submenu_trigger).unwrap();
    assert_eq!(submenu_native.role, NativeRole::MenuItem);
    assert_eq!(submenu_native.props.action.as_deref(), Some("toggleMenu"));
    assert_eq!(
        submenu_native
            .props
            .metadata
            .get("actionValue")
            .map(String::as_str),
        Some("more")
    );

    let keyboard_native = bridge.lower_to_native(keyboard).unwrap();
    assert_eq!(keyboard_native.role, NativeRole::KeyboardInput);
    assert_eq!(keyboard_native.props.label.as_deref(), Some("Cmd+O"));

    let indicator_native = bridge.lower_to_native(selection_indicator).unwrap();
    assert_eq!(indicator_native.role, NativeRole::Text);
    assert_eq!(indicator_native.props.label.as_deref(), Some("Selected"));
    assert_eq!(
        attribute_value(selection_indicator, "data-selected"),
        Some("true")
    );

    let list_section_native = bridge.lower_to_native(list_box_section).unwrap();
    assert_eq!(list_section_native.role, NativeRole::Section);
    assert_eq!(list_section_native.props.label.as_deref(), Some("People"));
    assert_eq!(
        attribute_value(list_box_section, "data-collection-section"),
        Some("true")
    );
    assert_eq!(
        attribute_value(list_box_section, "data-collection-kind"),
        Some("list-box")
    );

    let list_header_native = bridge.lower_to_native(list_box_header).unwrap();
    assert_eq!(list_header_native.role, NativeRole::Header);
    assert_eq!(list_header_native.props.label.as_deref(), Some("People"));

    let grid_section_native = bridge.lower_to_native(grid_list_section).unwrap();
    assert_eq!(grid_section_native.role, NativeRole::Section);
    assert_eq!(
        grid_section_native.props.label.as_deref(),
        Some("Source files")
    );
    assert_eq!(
        attribute_value(grid_list_section, "data-collection-section"),
        Some("true")
    );
    assert_eq!(
        attribute_value(grid_list_section, "data-collection-kind"),
        Some("grid-list")
    );

    let grid_header_native = bridge.lower_to_native(grid_list_header).unwrap();
    assert_eq!(grid_header_native.role, NativeRole::Header);
    assert_eq!(
        grid_header_native.props.label.as_deref(),
        Some("Source files")
    );

    let tree_section_native = bridge.lower_to_native(tree_section).unwrap();
    assert_eq!(tree_section_native.role, NativeRole::Section);
    assert_eq!(
        tree_section_native.props.label.as_deref(),
        Some("Workspace")
    );
    assert_eq!(
        attribute_value(tree_section, "data-collection-section"),
        Some("true")
    );
    assert_eq!(
        attribute_value(tree_section, "data-collection-kind"),
        Some("tree")
    );

    let tree_header_native = bridge.lower_to_native(tree_header).unwrap();
    assert_eq!(tree_header_native.role, NativeRole::Header);
    assert_eq!(tree_header_native.props.label.as_deref(), Some("Workspace"));

    let dialog_trigger_native = bridge.lower_to_native(dialog_trigger).unwrap();
    assert_eq!(dialog_trigger_native.role, NativeRole::Button);
    assert_eq!(
        dialog_trigger_native.props.action.as_deref(),
        Some("openDialog")
    );
    assert_eq!(
        attribute_value(dialog_trigger, "aria-haspopup"),
        Some("dialog")
    );

    let tooltip_trigger_native = bridge.lower_to_native(tooltip_trigger).unwrap();
    assert_eq!(tooltip_trigger_native.role, NativeRole::Button);
    assert_eq!(
        tooltip_trigger_native.props.action.as_deref(),
        Some("toggleTooltip")
    );

    let disclosure_panel_native = bridge.lower_to_native(disclosure_panel).unwrap();
    assert_eq!(disclosure_panel_native.role, NativeRole::Section);
    assert_eq!(
        disclosure_panel_native.props.label.as_deref(),
        Some("Details panel")
    );
    assert_eq!(
        attribute_value(disclosure_panel, "data-expanded"),
        Some("true")
    );
}

#[test]
fn rsx_ui_textarea_uses_function_component_template_and_native_text_field_props() {
    let component = RsxComponent::new(
        "message",
        r#"
        <UiTextarea
          key="message"
          value={state.email}
          placeholder="Message"
          onChange={setEmail}
          rows="6"
          maxLength="280"
          className="resize-none"
        />
        "#,
    )
    .unwrap()
    .use_state("email", |state: &FormState| state.email.clone())
    .use_value_reducer("setEmail", |state: &mut FormState, email: String| {
        state.email = email;
        Ok(())
    });

    let frame = component
        .render(&FormState {
            email: "Hello from RSX".to_string(),
            tab: String::new(),
            saved: false,
        })
        .unwrap();

    let textarea = find_element_by_attribute(&frame.root, "data-slot", "textarea").unwrap();
    assert_class_contains(textarea, "border-hairline-strong");
    assert_class_contains(textarea, "min-h-20");
    assert_class_contains(textarea, "resize-none");

    let native = RsxCompilerBridge::new().lower_to_native(textarea).unwrap();
    assert_eq!(native.role, NativeRole::TextField);
    assert_eq!(native.props.value.as_deref(), Some("Hello from RSX"));
    assert_eq!(native.props.placeholder.as_deref(), Some("Message"));
    assert_eq!(native.props.rows, Some(6));
    assert_eq!(native.props.max_length, Some(280));
    assert_eq!(
        native.props.web.events.get("onInput").map(String::as_str),
        Some("setEmail")
    );
}

#[test]
fn rsx_ui_button_is_component_cx_component_with_button_hook() {
    let component = ComponentCx::compile("ui-button", ui_button)
        .unwrap()
        .use_reducer("press", |state: &mut UiButtonProps, _invocation| {
            state.is_pressed = true;
            Ok(())
        });

    let frame = component
        .render(&UiButtonProps {
            on_press: Some("press".to_string()),
            is_pressed: true,
            class_name: "w-full".to_string(),
            action_value: "primary".to_string(),
            action_payload: serde_json::json!("payload"),
            ..UiButtonProps::default()
        })
        .unwrap();
    let button = find_element_by_attribute(&frame.root, "data-slot", "button").unwrap();
    let CompiledRsxNode::Element { props, .. } = button else {
        panic!("button element");
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
        Some("true")
    );
    assert_eq!(
        props.attributes.get("data-slot").map(String::as_str),
        Some("button")
    );
    assert_eq!(
        props.attributes.get("actionValue").map(String::as_str),
        Some("primary")
    );
    assert_eq!(
        props.attributes.get("actionPayload").map(String::as_str),
        Some("payload")
    );
    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "w-full");
}

#[test]
fn rsx_ui_toggle_buttons_consume_toggle_button_hook_props() {
    let button_component = ComponentCx::compile("ui-toggle-button", ui_toggle_button)
        .unwrap()
        .use_reducer(
            "toggleCompact",
            |_state: &mut UiToggleButtonProps, _invocation| Ok(()),
        );
    let button_frame = button_component
        .render(&UiToggleButtonProps {
            on_press: Some("toggleCompact".to_string()),
            is_selected: true,
            is_pressed: true,
            action_value: "compact".to_string(),
            action_payload: serde_json::json!("payload"),
            class_name: "rounded-md".to_string(),
            ..UiToggleButtonProps::default()
        })
        .unwrap();
    let button =
        find_element_by_attribute(&button_frame.root, "data-slot", "toggle-button").unwrap();
    let CompiledRsxNode::Element { props, .. } = button else {
        panic!("toggle button element");
    };
    assert_eq!(
        props.events.get("onPress").map(String::as_str),
        Some("toggleCompact")
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
        props.attributes.get("actionPayload").map(String::as_str),
        Some("payload")
    );
    assert_class_contains(button, "rounded-md");

    let group_component = ComponentCx::compile("ui-toggle-button-group", ui_toggle_button_group)
        .unwrap()
        .use_reducer(
            "setView",
            |_state: &mut UiToggleButtonGroupProps, _invocation| Ok(()),
        );
    let group_frame = group_component
        .render(&UiToggleButtonGroupProps {
            label: "View".to_string(),
            value: "compact".to_string(),
            orientation: "vertical".to_string(),
            on_selection_change: "setView".to_string(),
            selection_mode: "multiple".to_string(),
            class_name: "gap-2".to_string(),
            ..UiToggleButtonGroupProps::default()
        })
        .unwrap();
    let group =
        find_element_by_attribute(&group_frame.root, "data-slot", "toggle-button-group").unwrap();
    let CompiledRsxNode::Element { props, .. } = group else {
        panic!("toggle button group element");
    };
    assert_eq!(props.label.as_deref(), Some("View"));
    assert_eq!(props.value.as_deref(), Some("compact"));
    assert_eq!(attribute_value(group, "data-orientation"), Some("vertical"));
    assert_eq!(
        props.events.get("onSelectionChange").map(String::as_str),
        Some("setView")
    );
    assert_eq!(
        props
            .attributes
            .get("aria-multiselectable")
            .map(String::as_str),
        Some("true")
    );
    assert_class_contains(group, "gap-2");
}

#[test]
fn rsx_ui_text_components_consume_text_hook_props() {
    let text_frame = ComponentCx::compile("ui-text", ui_text)
        .unwrap()
        .render(&UiTextProps {
            label: "Body".to_string(),
            text_value: "Body text".to_string(),
            class_name: "font-medium".to_string(),
        })
        .unwrap();
    let text = find_element_by_attribute(&text_frame.root, "data-slot", "text").unwrap();
    let CompiledRsxNode::Element { props, .. } = text else {
        panic!("text element");
    };
    assert_eq!(props.label.as_deref(), Some("Body"));
    assert_eq!(props.text_value.as_deref(), Some("Body text"));
    assert_class_contains(text, "font-medium");

    let heading_frame = ComponentCx::compile("ui-heading", ui_heading)
        .unwrap()
        .render(&UiHeadingProps {
            label: "Title".to_string(),
            text_value: "Title text".to_string(),
            level: 9,
            class_name: "tracking-normal".to_string(),
        })
        .unwrap();
    let heading = find_element_by_attribute(&heading_frame.root, "data-slot", "heading").unwrap();
    let CompiledRsxNode::Element { props, .. } = heading else {
        panic!("heading element");
    };
    assert_eq!(props.label.as_deref(), Some("Title"));
    assert_eq!(props.text_value.as_deref(), Some("Title text"));
    assert_eq!(attribute_value(heading, "aria-level"), Some("6"));
    assert_class_contains(heading, "tracking-normal");

    let label_frame = ComponentCx::compile("ui-label", ui_label)
        .unwrap()
        .render(&UiLabelProps {
            label: "Email".to_string(),
            class_name: "font-semibold".to_string(),
        })
        .unwrap();
    let label = find_element_by_attribute(&label_frame.root, "data-slot", "label").unwrap();
    let CompiledRsxNode::Element { props, .. } = label else {
        panic!("label element");
    };
    assert_eq!(props.label.as_deref(), Some("Email"));
    assert_class_contains(label, "font-semibold");

    let description_frame = ComponentCx::compile("ui-description", ui_description)
        .unwrap()
        .render(&UiDescriptionProps {
            label: "Details".to_string(),
            text_value: "Details text".to_string(),
            class_name: "leading-5".to_string(),
        })
        .unwrap();
    let description =
        find_element_by_attribute(&description_frame.root, "data-slot", "description").unwrap();
    let CompiledRsxNode::Element { props, .. } = description else {
        panic!("description element");
    };
    assert_eq!(props.label.as_deref(), Some("Details"));
    assert_eq!(props.text_value.as_deref(), Some("Details text"));
    assert_class_contains(description, "leading-5");

    let field_error_frame = ComponentCx::compile("ui-field-error", ui_field_error)
        .unwrap()
        .render(&UiFieldErrorProps {
            label: "Error".to_string(),
            text_value: "Required".to_string(),
            class_name: "mt-1".to_string(),
        })
        .unwrap();
    let field_error =
        find_element_by_attribute(&field_error_frame.root, "data-slot", "field-error").unwrap();
    let CompiledRsxNode::Element { props, .. } = field_error else {
        panic!("field error element");
    };
    assert_eq!(props.label.as_deref(), Some("Error"));
    assert_eq!(props.text_value.as_deref(), Some("Required"));
    assert_eq!(attribute_value(field_error, "data-invalid"), Some("true"));
    assert_class_contains(field_error, "mt-1");

    let legend_frame = ComponentCx::compile("ui-legend", ui_legend)
        .unwrap()
        .render(&UiLegendProps {
            label: "Options".to_string(),
            text_value: "Option group".to_string(),
            class_name: "uppercase".to_string(),
        })
        .unwrap();
    let legend = find_element_by_attribute(&legend_frame.root, "data-slot", "legend").unwrap();
    let CompiledRsxNode::Element { props, .. } = legend else {
        panic!("legend element");
    };
    assert_eq!(props.label.as_deref(), Some("Options"));
    assert_eq!(props.text_value.as_deref(), Some("Option group"));
    assert_class_contains(legend, "uppercase");

    let hidden_frame = ComponentCx::compile("ui-visually-hidden", ui_visually_hidden)
        .unwrap()
        .render(&UiVisuallyHiddenProps {
            label: "Hidden".to_string(),
            text_value: "Hidden text".to_string(),
            class_name: "sr-only".to_string(),
        })
        .unwrap();
    let hidden =
        find_element_by_attribute(&hidden_frame.root, "data-slot", "visually-hidden").unwrap();
    let CompiledRsxNode::Element { props, .. } = hidden else {
        panic!("visually hidden element");
    };
    assert_eq!(props.label.as_deref(), Some("Hidden"));
    assert_eq!(props.text_value.as_deref(), Some("Hidden text"));
    assert_class_contains(hidden, "sr-only");

    let keyboard_frame = ComponentCx::compile("ui-keyboard", ui_keyboard)
        .unwrap()
        .render(&UiKeyboardProps {
            label: "Shortcut".to_string(),
            text_value: "Cmd+K".to_string(),
            class_name: "shadow-none".to_string(),
        })
        .unwrap();
    let keyboard =
        find_element_by_attribute(&keyboard_frame.root, "data-slot", "keyboard").unwrap();
    let CompiledRsxNode::Element { props, .. } = keyboard else {
        panic!("keyboard element");
    };
    assert_eq!(props.label.as_deref(), Some("Shortcut"));
    assert_eq!(props.text_value.as_deref(), Some("Cmd+K"));
    assert_class_contains(keyboard, "shadow-none");

    let list_header_frame = ComponentCx::compile("ui-list-box-header", ui_list_box_header)
        .unwrap()
        .render(&UiListBoxHeaderProps {
            label: "People".to_string(),
            text_value: "People header".to_string(),
            class_name: "px-4".to_string(),
        })
        .unwrap();
    let list_header =
        find_element_by_attribute(&list_header_frame.root, "data-slot", "list-box-header").unwrap();
    let CompiledRsxNode::Element { props, .. } = list_header else {
        panic!("list box header element");
    };
    assert_eq!(props.label.as_deref(), Some("People"));
    assert_eq!(props.text_value.as_deref(), Some("People header"));
    assert_class_contains(list_header, "px-4");

    let grid_header_frame = ComponentCx::compile("ui-grid-list-header", ui_grid_list_header)
        .unwrap()
        .render(&UiGridListHeaderProps {
            label: "Files".to_string(),
            text_value: "Files header".to_string(),
            class_name: "px-3".to_string(),
        })
        .unwrap();
    let grid_header =
        find_element_by_attribute(&grid_header_frame.root, "data-slot", "grid-list-header")
            .unwrap();
    let CompiledRsxNode::Element { props, .. } = grid_header else {
        panic!("grid list header element");
    };
    assert_eq!(props.label.as_deref(), Some("Files"));
    assert_eq!(props.text_value.as_deref(), Some("Files header"));
    assert_class_contains(grid_header, "px-3");

    let tree_header_frame = ComponentCx::compile("ui-tree-header", ui_tree_header)
        .unwrap()
        .render(&UiTreeHeaderProps {
            label: "Project".to_string(),
            text_value: "Project header".to_string(),
            class_name: "px-5".to_string(),
        })
        .unwrap();
    let tree_header =
        find_element_by_attribute(&tree_header_frame.root, "data-slot", "tree-header").unwrap();
    let CompiledRsxNode::Element { props, .. } = tree_header else {
        panic!("tree header element");
    };
    assert_eq!(props.label.as_deref(), Some("Project"));
    assert_eq!(props.text_value.as_deref(), Some("Project header"));
    assert_class_contains(tree_header, "px-5");
}

#[test]
fn rsx_ui_structure_parts_consume_structure_hook_props() {
    let separator_frame = ComponentCx::compile("ui-separator", ui_separator)
        .unwrap()
        .render(&UiSeparatorProps {
            orientation: "vertical".to_string(),
            class_name: "mx-2".to_string(),
        })
        .unwrap();
    let separator =
        find_element_by_attribute(&separator_frame.root, "data-slot", "separator").unwrap();
    let CompiledRsxNode::Element { props, .. } = separator else {
        panic!("separator element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(
        attribute_value(separator, "data-orientation"),
        Some("vertical")
    );
    assert_class_contains(separator, "mx-2");

    let toolbar_frame = ComponentCx::compile("ui-toolbar", ui_toolbar)
        .unwrap()
        .render(&UiToolbarProps {
            label: "Formatting".to_string(),
            orientation: "vertical".to_string(),
            is_disabled: true,
            class_name: "gap-2".to_string(),
        })
        .unwrap();
    let toolbar = find_element_by_attribute(&toolbar_frame.root, "data-slot", "toolbar").unwrap();
    let CompiledRsxNode::Element { props, .. } = toolbar else {
        panic!("toolbar element");
    };
    assert_eq!(props.label.as_deref(), Some("Formatting"));
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(toolbar, "aria-disabled"), Some("true"));
    assert_class_contains(toolbar, "gap-2");

    let drop_frame = ComponentCx::compile("ui-drop-indicator", ui_drop_indicator)
        .unwrap()
        .render(&UiDropIndicatorProps {
            orientation: "vertical".to_string(),
            is_target: true,
            class_name: "opacity-100".to_string(),
        })
        .unwrap();
    let drop_indicator =
        find_element_by_attribute(&drop_frame.root, "data-slot", "drop-indicator").unwrap();
    let CompiledRsxNode::Element { props, .. } = drop_indicator else {
        panic!("drop indicator element");
    };
    assert_eq!(props.orientation, Some(CompiledOrientation::Vertical));
    assert_eq!(attribute_value(drop_indicator, "data-target"), Some("true"));
    assert_class_contains(drop_indicator, "opacity-100");

    let selection_frame = ComponentCx::compile("ui-selection-indicator", ui_selection_indicator)
        .unwrap()
        .render(&UiSelectionIndicatorProps {
            label: "Selected".to_string(),
            is_selected: true,
            class_name: "text-primary".to_string(),
        })
        .unwrap();
    let selection =
        find_element_by_attribute(&selection_frame.root, "data-slot", "selection-indicator")
            .unwrap();
    let CompiledRsxNode::Element { props, .. } = selection else {
        panic!("selection indicator element");
    };
    assert_eq!(props.label.as_deref(), Some("Selected"));
    assert!(props.is_selected);
    assert_eq!(attribute_value(selection, "data-selected"), Some("true"));
    assert_class_contains(selection, "text-primary");
}

#[test]
fn rsx_ui_layout_landmarks_use_landmark_hook_props() {
    let frame = RsxComponent::new(
        "layout-landmarks",
        r#"
        <UiMain key="main" label="Workspace">
          <UiNavigation key="nav" label="Primary navigation" />
          <UiHeader key="header" label="Top bar" />
          <UiSection key="section" label="Overview" />
          <UiArticle key="article" label="Release notes" />
          <UiAside key="aside" label="Context" />
          <UiSearch key="search" label="Global search" />
          <UiFooter key="footer" label="Status" />
        </UiMain>
        "#,
    )
    .unwrap()
    .render(&())
    .unwrap();

    let bridge = RsxCompilerBridge::new();
    for (slot, role, explicit_role, label) in [
        ("main", NativeRole::Main, "main", "Workspace"),
        (
            "navigation",
            NativeRole::Navigation,
            "navigation",
            "Primary navigation",
        ),
        ("header", NativeRole::Header, "banner", "Top bar"),
        ("section", NativeRole::Section, "region", "Overview"),
        ("article", NativeRole::Article, "article", "Release notes"),
        ("aside", NativeRole::Aside, "complementary", "Context"),
        ("search", NativeRole::Search, "search", "Global search"),
        ("footer", NativeRole::Footer, "contentinfo", "Status"),
    ] {
        let node = find_element_by_attribute(&frame.root, "data-slot", slot).unwrap();
        assert_eq!(attribute_value(node, "data-landmark"), Some("true"));
        let native = bridge.lower_to_native(node).unwrap();
        assert_eq!(native.role, role, "{slot}");
        assert_eq!(native.props.label.as_deref(), Some(label), "{slot}");
        assert_eq!(
            native.props.explicit_role.as_deref(),
            Some(explicit_role),
            "{slot}"
        );
    }
}

#[test]
fn rsx_ui_group_uses_group_hook_props() {
    let component = ComponentCx::compile("ui-group", ui_group)
        .unwrap()
        .use_reducer(
            "startHover",
            |_state: &mut UiGroupProps, _invocation| Ok(()),
        )
        .use_reducer("setFocus", |_state: &mut UiGroupProps, _invocation| Ok(()));

    let frame = component
        .render(&UiGroupProps {
            class_name: "gap-3".to_string(),
            label: "Inspector".to_string(),
            on_hover_start: Some("startHover".to_string()),
            on_hover_end: None,
            on_hover_change: None,
            on_focus: None,
            on_blur: None,
            on_focus_change: Some("setFocus".to_string()),
            on_focus_within: Some("setFocus".to_string()),
            on_blur_within: None,
            on_focus_within_change: Some("setFocus".to_string()),
            is_disabled: true,
            is_invalid: true,
            is_read_only: true,
            is_hovered: true,
            is_focused: true,
            is_focus_visible: true,
            is_focus_within: true,
            auto_focus: true,
            tab_index: 6,
        })
        .unwrap();

    let group = find_element_by_attribute(&frame.root, "data-slot", "group").unwrap();
    let CompiledRsxNode::Element { props, .. } = group else {
        panic!("group element");
    };

    assert_eq!(props.label.as_deref(), Some("Inspector"));
    assert!(props.is_disabled);
    assert!(props.is_invalid);
    assert!(props.is_read_only);
    assert_eq!(
        props.events.get("onHoverStart").map(String::as_str),
        Some("startHover")
    );
    assert_eq!(
        props.events.get("onFocusChange").map(String::as_str),
        Some("setFocus")
    );
    assert_eq!(attribute_value(group, "role"), Some("group"));
    assert_eq!(attribute_value(group, "tabIndex"), Some("-1"));
    assert_eq!(attribute_value(group, "autoFocus"), Some("true"));
    assert_eq!(attribute_value(group, "aria-disabled"), Some("true"));
    assert_eq!(attribute_value(group, "aria-invalid"), Some("true"));
    assert_eq!(attribute_value(group, "aria-readonly"), Some("true"));
    assert_eq!(attribute_value(group, "data-disabled"), Some("true"));
    assert_eq!(attribute_value(group, "data-invalid"), Some("true"));
    assert_eq!(attribute_value(group, "data-readonly"), Some("true"));
    assert_eq!(attribute_value(group, "data-hovered"), Some("true"));
    assert_eq!(attribute_value(group, "data-focused"), Some("true"));
    assert_eq!(attribute_value(group, "data-focus-visible"), Some("true"));
    assert_eq!(attribute_value(group, "data-focus-within"), Some("true"));
    assert_class_contains(group, "gap-3");

    let native = RsxCompilerBridge::new().lower_to_native(group).unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Inspector"));
    assert!(native.props.disabled);
    assert!(native.props.invalid);
    assert!(native.props.read_only);
    assert_eq!(native.props.explicit_role.as_deref(), Some("group"));
}

#[test]
fn rsx_ui_virtualizer_uses_virtualizer_hook_props() {
    let component = ComponentCx::compile("ui-virtualizer", ui_virtualizer).unwrap();

    let frame = component
        .render(&UiVirtualizerProps {
            class_name: "h-64".to_string(),
            label: "Virtual results".to_string(),
            layout: "grid".to_string(),
            orientation: "horizontal".to_string(),
            item_count: 240,
            estimated_item_size: 56,
            visible_start: 16,
            visible_end: 44,
            overscan: 6,
            gap: 10,
            padding: 14,
            is_scrolling: true,
            is_disabled: true,
            tab_index: 5,
        })
        .unwrap();

    let virtualizer = find_element_by_attribute(&frame.root, "data-slot", "virtualizer").unwrap();
    let CompiledRsxNode::Element { props, .. } = virtualizer else {
        panic!("virtualizer element");
    };

    assert_eq!(props.label.as_deref(), Some("Virtual results"));
    assert!(props.is_disabled);
    assert_eq!(attribute_value(virtualizer, "role"), Some("grid"));
    assert_eq!(attribute_value(virtualizer, "tabIndex"), Some("-1"));
    assert_eq!(attribute_value(virtualizer, "aria-disabled"), Some("true"));
    assert_eq!(
        attribute_value(virtualizer, "data-virtualizer"),
        Some("true")
    );
    assert_eq!(attribute_value(virtualizer, "data-layout"), Some("grid"));
    assert_eq!(
        attribute_value(virtualizer, "data-orientation"),
        Some("horizontal")
    );
    assert_eq!(attribute_value(virtualizer, "data-item-count"), Some("240"));
    assert_eq!(
        attribute_value(virtualizer, "data-estimated-item-size"),
        Some("56")
    );
    assert_eq!(
        attribute_value(virtualizer, "data-visible-start"),
        Some("16")
    );
    assert_eq!(attribute_value(virtualizer, "data-visible-end"), Some("44"));
    assert_eq!(attribute_value(virtualizer, "data-overscan"), Some("6"));
    assert_eq!(attribute_value(virtualizer, "data-gap"), Some("10"));
    assert_eq!(attribute_value(virtualizer, "data-padding"), Some("14"));
    assert_eq!(attribute_value(virtualizer, "data-scrolling"), Some("true"));
    assert_eq!(attribute_value(virtualizer, "data-disabled"), Some("true"));
    assert_class_contains(virtualizer, "overflow-auto");
    assert_class_contains(virtualizer, "h-64");

    let native = RsxCompilerBridge::new()
        .lower_to_native(virtualizer)
        .unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Virtual results"));
    assert!(native.props.disabled);
    assert_eq!(native.props.explicit_role.as_deref(), Some("grid"));
}

#[test]
fn rsx_ui_clipboard_target_uses_clipboard_hook_props() {
    let component = ComponentCx::compile("ui-clipboard-target", ui_clipboard_target)
        .unwrap()
        .use_reducer(
            "copySelection",
            |_state: &mut UiClipboardTargetProps, _invocation| Ok(()),
        )
        .use_reducer(
            "cutSelection",
            |_state: &mut UiClipboardTargetProps, _invocation| Ok(()),
        )
        .use_reducer(
            "pasteSelection",
            |_state: &mut UiClipboardTargetProps, _invocation| Ok(()),
        );

    let frame = component
        .render(&UiClipboardTargetProps {
            class_name: "rounded-md".to_string(),
            label: "Editor selection".to_string(),
            on_copy: Some("copySelection".to_string()),
            on_cut: Some("cutSelection".to_string()),
            on_paste: Some("pasteSelection".to_string()),
            copy_value: "selected text".to_string(),
            copy_mime_type: "text/plain".to_string(),
            accepted_mime_types: "text/plain,text/html".to_string(),
            is_disabled: false,
        })
        .unwrap();

    let target = find_element_by_attribute(&frame.root, "data-slot", "clipboard-target").unwrap();
    let CompiledRsxNode::Element { props, .. } = target else {
        panic!("clipboard target element");
    };

    assert_eq!(props.aria_label.as_deref(), Some("Editor selection"));
    assert_eq!(
        props.events.get("onCopy").map(String::as_str),
        Some("copySelection")
    );
    assert_eq!(
        props.events.get("onCut").map(String::as_str),
        Some("cutSelection")
    );
    assert_eq!(
        props.events.get("onPaste").map(String::as_str),
        Some("pasteSelection")
    );
    assert_eq!(attribute_value(target, "role"), Some("textbox"));
    assert_eq!(attribute_value(target, "tabIndex"), Some("0"));
    assert_eq!(
        attribute_value(target, "data-copy-value"),
        Some("selected text")
    );
    assert_eq!(
        attribute_value(target, "data-copy-mime-type"),
        Some("text/plain")
    );
    assert_eq!(
        attribute_value(target, "data-accepted-mime-types"),
        Some("text/plain,text/html")
    );
    assert_eq!(
        attribute_value(target, "data-clipboard-disabled"),
        Some("false")
    );
    assert_class_contains(target, "outline-none");
    assert_class_contains(target, "rounded-md");

    let native = RsxCompilerBridge::new().lower_to_native(target).unwrap();
    assert_eq!(native.role, NativeRole::View);
    assert_eq!(native.props.label.as_deref(), Some("Editor selection"));
    assert_eq!(native.props.explicit_role.as_deref(), Some("textbox"));
    assert_eq!(
        native.props.web.events.get("onCopy").map(String::as_str),
        Some("copySelection")
    );
}

#[test]
fn rsx_ui_selection_inputs_consume_selection_input_hook_props() {
    let combo_component = ComponentCx::compile("ui-combo-box", ui_combo_box)
        .unwrap()
        .use_reducer("setQuery", |_state: &mut UiComboBoxProps, _invocation| {
            Ok(())
        })
        .use_reducer(
            "setAssignee",
            |_state: &mut UiComboBoxProps, _invocation| Ok(()),
        )
        .use_reducer(
            "toggleAssignee",
            |_state: &mut UiComboBoxProps, _invocation| Ok(()),
        );
    let combo_frame = combo_component
        .render(&UiComboBoxProps {
            label: "Assignee".to_string(),
            value: "ada".to_string(),
            input_value: "ad".to_string(),
            placeholder: "Search people".to_string(),
            on_change: "setQuery".to_string(),
            on_selection_change: "setAssignee".to_string(),
            on_open_change: "toggleAssignee".to_string(),
            is_open: true,
            is_required: true,
            is_invalid: true,
            class_name: "w-full".to_string(),
            ..UiComboBoxProps::default()
        })
        .unwrap();
    let combo = find_element_by_attribute(&combo_frame.root, "data-slot", "combo-box").unwrap();
    let combo_input =
        find_element_by_attribute(&combo_frame.root, "data-slot", "combo-box-input").unwrap();
    let combo_trigger =
        find_element_by_attribute(&combo_frame.root, "data-slot", "combo-box-trigger").unwrap();

    let CompiledRsxNode::Element {
        props: combo_props, ..
    } = combo
    else {
        panic!("combo element");
    };
    let CompiledRsxNode::Element {
        props: combo_input_props,
        ..
    } = combo_input
    else {
        panic!("combo input element");
    };
    let CompiledRsxNode::Element {
        props: combo_trigger_props,
        ..
    } = combo_trigger
    else {
        panic!("combo trigger element");
    };

    assert_eq!(combo_props.label.as_deref(), Some("Assignee"));
    assert_eq!(combo_props.value.as_deref(), Some("ada"));
    assert!(combo_props.is_required);
    assert!(combo_props.is_invalid);
    assert_eq!(
        combo_props
            .attributes
            .get("data-input-value")
            .map(String::as_str),
        Some("ad")
    );
    assert_eq!(
        combo_props.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(combo_input_props.value.as_deref(), Some("ad"));
    assert_eq!(
        combo_input_props.events.get("onInput").map(String::as_str),
        Some("setQuery")
    );
    assert_eq!(
        combo_trigger_props
            .events
            .get("onPress")
            .map(String::as_str),
        Some("toggleAssignee")
    );
    assert_class_contains(combo, "w-full");

    let autocomplete_component = ComponentCx::compile("ui-autocomplete", ui_autocomplete)
        .unwrap()
        .use_reducer(
            "setCommandQuery",
            |_state: &mut UiAutocompleteProps, _invocation| Ok(()),
        )
        .use_reducer(
            "setCommand",
            |_state: &mut UiAutocompleteProps, _invocation| Ok(()),
        );
    let autocomplete_frame = autocomplete_component
        .render(&UiAutocompleteProps {
            label: "Command".to_string(),
            value: "open".to_string(),
            input_value: "op".to_string(),
            placeholder: "Run command".to_string(),
            on_change: "setCommandQuery".to_string(),
            on_selection_change: "setCommand".to_string(),
            is_invalid: true,
            ..UiAutocompleteProps::default()
        })
        .unwrap();
    let autocomplete =
        find_element_by_attribute(&autocomplete_frame.root, "data-slot", "autocomplete").unwrap();
    let autocomplete_input =
        find_element_by_attribute(&autocomplete_frame.root, "data-slot", "autocomplete-input")
            .unwrap();
    let CompiledRsxNode::Element {
        props: autocomplete_props,
        ..
    } = autocomplete
    else {
        panic!("autocomplete element");
    };
    let CompiledRsxNode::Element {
        props: autocomplete_input_props,
        ..
    } = autocomplete_input
    else {
        panic!("autocomplete input element");
    };
    assert_eq!(autocomplete_props.label.as_deref(), Some("Command"));
    assert_eq!(
        autocomplete_props
            .attributes
            .get("data-input-value")
            .map(String::as_str),
        Some("op")
    );
    assert_eq!(
        autocomplete_input_props
            .events
            .get("onInput")
            .map(String::as_str),
        Some("setCommandQuery")
    );

    let select_component = ComponentCx::compile("ui-select", ui_select)
        .unwrap()
        .use_reducer("setDensity", |_state: &mut UiSelectProps, _invocation| {
            Ok(())
        });
    let select_frame = select_component
        .render(&UiSelectProps {
            label: "Density".to_string(),
            value: "compact".to_string(),
            placeholder: "Choose density".to_string(),
            on_selection_change: "setDensity".to_string(),
            on_open_change: "toggleDensity".to_string(),
            is_open: true,
            is_invalid: true,
            ..UiSelectProps::default()
        })
        .unwrap();
    let select = find_element_by_attribute(&select_frame.root, "data-slot", "select").unwrap();
    let CompiledRsxNode::Element {
        props: select_props,
        ..
    } = select
    else {
        panic!("select element");
    };
    assert_eq!(select_props.label.as_deref(), Some("Density"));
    assert_eq!(select_props.value.as_deref(), Some("compact"));
    assert!(select_props.is_invalid);
    assert_eq!(
        select_props.attributes.get("data-open").map(String::as_str),
        Some("true")
    );
    assert_eq!(
        select_props
            .events
            .get("onSelectionChange")
            .map(String::as_str),
        Some("setDensity")
    );

    let select_value_component = ComponentCx::compile("ui-select-value", ui_select_value).unwrap();
    let select_value_frame = select_value_component
        .render(&UiSelectValueProps {
            value: "compact".to_string(),
            placeholder: "Choose density".to_string(),
            class_name: "truncate".to_string(),
        })
        .unwrap();
    let select_value =
        find_element_by_attribute(&select_value_frame.root, "data-slot", "select-value").unwrap();
    let CompiledRsxNode::Element {
        props: select_value_props,
        ..
    } = select_value
    else {
        panic!("select value element");
    };
    assert_eq!(select_value_props.label.as_deref(), Some("compact"));
    assert_eq!(
        select_value_props
            .attributes
            .get("data-placeholder")
            .map(String::as_str),
        Some("false")
    );
    assert_class_contains(select_value, "truncate");

    let combo_value_component =
        ComponentCx::compile("ui-combo-box-value", ui_combo_box_value).unwrap();
    let combo_value_frame = combo_value_component
        .render(&UiComboBoxValueProps {
            placeholder: "Assignee".to_string(),
            ..UiComboBoxValueProps::default()
        })
        .unwrap();
    let combo_value =
        find_element_by_attribute(&combo_value_frame.root, "data-slot", "combo-box-value").unwrap();
    let CompiledRsxNode::Element {
        props: combo_value_props,
        ..
    } = combo_value
    else {
        panic!("combo value element");
    };
    assert_eq!(combo_value_props.label.as_deref(), Some("Assignee"));
    assert_eq!(
        combo_value_props
            .attributes
            .get("data-placeholder")
            .map(String::as_str),
        Some("true")
    );
}

#[test]
fn rsx_ui_button_variants_merge_base_variant_size_and_caller_classes() {
    let component = RsxComponent::<FormState>::new(
        "button",
        r#"
        <UiButton
          key="save"
          variant="secondary"
          size="sm"
          className="w-full"
          onPress={saveProfile}
        >
          Save
        </UiButton>
        "#,
    )
    .unwrap()
    .use_reducer("saveProfile", |_state, _invocation| Ok(()));

    let frame = component.render(&FormState::default()).unwrap();
    let button = find_element_by_attribute(&frame.root, "data-slot", "button").unwrap();

    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "bg-surface-card");
    assert_class_contains(button, "border-hairline-strong");
    assert_class_contains(button, "text-ink");
    assert_class_contains(button, "h-8");
    assert_class_contains(button, "w-full");
    assert_class_excludes(button, "bg-primary");
}

#[test]
fn rsx_ui_badge_variants_render_status_tones() {
    let component = RsxComponent::<FormState>::new(
        "badge",
        r#"<UiBadge key="status" variant="outline" className="uppercase">Preview</UiBadge>"#,
    )
    .unwrap();

    let frame = component.render(&FormState::default()).unwrap();
    let badge = find_element_by_attribute(&frame.root, "data-slot", "badge").unwrap();

    assert_class_contains(badge, "inline-flex");
    assert_class_contains(badge, "text-ink");
    assert_class_contains(badge, "rounded-md");
    assert_class_contains(badge, "bg-surface-card");
    assert_class_contains(badge, "uppercase");
}

#[test]
fn rsx_ui_rejects_unknown_variant_values() {
    let component = RsxComponent::<FormState>::new(
        "button",
        r#"<UiButton key="save" variant="loud" onPress={saveProfile}>Save</UiButton>"#,
    )
    .unwrap()
    .use_reducer("saveProfile", |_state, _invocation| Ok(()));

    let error = component.render(&FormState::default()).unwrap_err();

    assert!(error
        .to_string()
        .contains("unsupported variant value \"loud\""));
}

fn assert_class_contains(node: &CompiledRsxNode, token: &str) {
    let class_name = class_name(node);
    assert!(
        class_name.split_whitespace().any(|class| class == token),
        "expected className {class_name:?} to contain {token:?}"
    );
}

fn assert_class_excludes(node: &CompiledRsxNode, token: &str) {
    let class_name = class_name(node);
    assert!(
        !class_name.split_whitespace().any(|class| class == token),
        "expected className {class_name:?} to exclude {token:?}"
    );
}

fn class_name(node: &CompiledRsxNode) -> &str {
    let CompiledRsxNode::Element { props, .. } = node else {
        panic!("element node")
    };
    props
        .class_name
        .as_deref()
        .unwrap_or_else(|| panic!("missing className"))
}

fn attribute_value<'a>(node: &'a CompiledRsxNode, name: &str) -> Option<&'a str> {
    let CompiledRsxNode::Element { props, .. } = node else {
        panic!("element node")
    };
    props.attributes.get(name).map(String::as_str)
}

fn event_value<'a>(node: &'a CompiledRsxNode, name: &str) -> Option<&'a str> {
    let CompiledRsxNode::Element { props, .. } = node else {
        panic!("element node")
    };
    props.events.get(name).map(String::as_str)
}

fn find_element_by_attribute<'a>(
    node: &'a CompiledRsxNode,
    name: &str,
    value: &str,
) -> Option<&'a CompiledRsxNode> {
    find_element_by_attributes(node, &[(name, value)])
}

fn find_element_by_attributes<'a>(
    node: &'a CompiledRsxNode,
    attributes: &[(&str, &str)],
) -> Option<&'a CompiledRsxNode> {
    match node {
        CompiledRsxNode::Text { .. } => None,
        CompiledRsxNode::Element {
            props, children, ..
        } => {
            if attributes.iter().all(|(name, value)| {
                props.attributes.get(*name).map(String::as_str) == Some(*value)
            }) {
                return Some(node);
            }
            children
                .iter()
                .find_map(|child| find_element_by_attributes(child, attributes))
        }
    }
}
