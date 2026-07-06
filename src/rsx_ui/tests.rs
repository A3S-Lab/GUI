use super::*;
use crate::compiler::{CompiledRsxNode, RsxCompilerBridge};
use crate::event::ActionInvocation;
use crate::native::NativeRole;
use crate::rsx_app::{ComponentCx, RsxComponent, RsxTemplate};
use crate::style::{DisplayMode, PortableStyle, StyleColor};
use crate::web::WebProps;

#[derive(Debug, Default)]
struct FormState {
    email: String,
    tab: String,
    saved: bool,
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

#[test]
fn rsx_ui_renders_shadcn_like_components_with_vercel_tokens() {
    let component = RsxComponent::new(
        "settings",
        r#"
        <div key="root" class="bg-background text-foreground">
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

    assert_class_contains(card, "bg-card");
    assert_class_contains(card, "w-full");
    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "bg-primary");
    assert_class_contains(button, "h-9");
    assert_class_contains(button, "w-full");
    assert_class_contains(input, "border-input");

    let button_style =
        PortableStyle::from_web(&WebProps::new().class_name(class_name(button).to_string()));
    assert_eq!(button_style.display, Some(DisplayMode::InlineFlex));
    assert_eq!(
        button_style.background_color,
        Some(StyleColor::Rgba {
            red: 0x17,
            green: 0x17,
            blue: 0x17,
            alpha: 255,
        })
    );
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
    let button_source = include_str!("components/button.rsx");
    let component_index = include_str!("components/mod.rs");

    assert!(button_source.contains("pub struct UiButtonProps"));
    assert!(button_source.contains("pub fn ui_button(cx: &mut ComponentCx<UiButtonProps>) -> RSX"));
    assert!(button_source.contains("crate::rsx!("));
    assert!(component_index.contains("#[path = \"button.rsx\"]"));
    assert!(!component_index.contains(concat!("register", "_ui_")));
    assert!(!component_index.contains(concat!("install", "_ui_")));
    assert!(!component_index.contains(concat!("install", "_default_components")));
}

#[test]
fn rsx_ui_tabs_render_shadcn_classes_and_native_tab_semantics() {
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
    assert_class_contains(list, "bg-muted");
    assert_class_contains(list, "grid-cols-2");
    assert_class_contains(selected, "data-[selected=true]:bg-background");
    assert_class_contains(selected, "data-[selected=true]:shadow-sm");
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
        <UiForm key="form" label="Profile" onSubmit={submitProfile}>
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
    assert_class_contains(toolbar, "border");
    assert_class_contains(dialog, "shadow-lg");

    let form_native = bridge.lower_to_native(form).unwrap();
    assert_eq!(form_native.role, NativeRole::Form);
    assert_eq!(form_native.props.label.as_deref(), Some("Profile"));
    assert_eq!(
        form_native
            .props
            .web
            .events
            .get("onSubmit")
            .map(String::as_str),
        Some("submitProfile")
    );

    let group_native = bridge.lower_to_native(group).unwrap();
    assert_eq!(group_native.role, NativeRole::View);
    assert_eq!(group_native.props.label.as_deref(), Some("Account"));

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
    assert_class_contains(toggle_button, "data-[selected=true]:bg-accent");
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
    assert_eq!(combo_native.children.len(), 2);
    assert_eq!(combo_native.children[0].role, NativeRole::ListBoxItem);
    assert!(combo_native.children[0].props.selected);

    let number_native = bridge.lower_to_native(number_field).unwrap();
    assert_eq!(number_native.role, NativeRole::TextField);
    assert_eq!(number_native.props.label.as_deref(), Some("Quantity"));
    assert_eq!(number_native.props.current, Some(35.0));
    assert_eq!(number_native.props.min, Some(0.0));
    assert_eq!(number_native.props.max, Some(100.0));
    assert_eq!(number_native.props.step, Some(5.0));
    assert_eq!(number_native.props.action.as_deref(), Some("setQuantity"));
    assert_eq!(
        number_native
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
    assert_eq!(tree_native.role, NativeRole::ListBox);
    assert_eq!(tree_native.props.label.as_deref(), Some("Files"));

    let tree_item_native = bridge.lower_to_native(tree_item).unwrap();
    assert_eq!(tree_item_native.role, NativeRole::ListBoxItem);
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
    assert_class_contains(textarea, "border-input");
    assert_class_contains(textarea, "min-h-16");
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
    assert_class_contains(button, "inline-flex");
    assert_class_contains(button, "w-full");
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
    assert_class_contains(button, "bg-secondary");
    assert_class_contains(button, "text-secondary-foreground");
    assert_class_contains(button, "h-8");
    assert_class_contains(button, "w-full");
    assert_class_excludes(button, "bg-primary");
    assert_class_excludes(button, "h-9");
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
    assert_class_contains(badge, "text-foreground");
    assert_class_contains(badge, "uppercase");
    assert_class_excludes(badge, "bg-secondary");
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
