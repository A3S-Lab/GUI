use crate::backend::NativeEventHost;
use crate::compiler::{CompiledJsxNode, ReactCompilerBridge};
use crate::error::{GuiError, GuiResult};
use crate::event::{ActionInvocation, ActionRegistry, EventRouter, NativeEvent};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionState;
use crate::native::NativeElement;
use crate::platform::{BlueprintHost, NativeWidgetBlueprint};
use crate::react_aria::{AriaElement, ReactAriaMapper};
use crate::renderer::Renderer;

#[derive(Debug)]
pub struct GuiRuntime<H: NativeHost> {
    bridge: ReactCompilerBridge,
    mapper: ReactAriaMapper,
    renderer: Renderer,
    host: H,
    event_router: EventRouter,
    action_registry: ActionRegistry,
    interaction_state: InteractionState,
}

impl<H: NativeHost> GuiRuntime<H> {
    pub fn new(host: H) -> Self {
        Self {
            bridge: ReactCompilerBridge::new(),
            mapper: ReactAriaMapper::new(),
            renderer: Renderer::new(),
            host,
            event_router: EventRouter::new(),
            action_registry: ActionRegistry::new(),
            interaction_state: InteractionState::new(),
        }
    }

    pub fn render_compiled(&mut self, node: &CompiledJsxNode) -> GuiResult<HostNodeId> {
        let native = self.bridge.lower_to_native(node)?;
        self.render_native(&native)
    }

    pub fn render_aria(&mut self, element: &AriaElement) -> GuiResult<HostNodeId> {
        let native = self.mapper.map(element)?;
        self.render_native(&native)
    }

    pub fn render_native(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        self.renderer.render(element, &mut self.host)
    }

    pub fn host(&self) -> &H {
        &self.host
    }

    pub fn host_mut(&mut self) -> &mut H {
        &mut self.host
    }

    pub fn actions(&self) -> &ActionRegistry {
        &self.action_registry
    }

    pub fn actions_mut(&mut self) -> &mut ActionRegistry {
        &mut self.action_registry
    }

    pub fn interactions(&self) -> &InteractionState {
        &self.interaction_state
    }

    pub fn dispatch_event(
        &mut self,
        blueprint: &NativeWidgetBlueprint,
        event: NativeEvent,
    ) -> GuiResult<ActionInvocation> {
        self.interaction_state.apply_event(blueprint, &event);
        let invocation = self
            .event_router
            .route(blueprint, &event)
            .ok_or_else(|| GuiError::host("native event has no registered Web action"))?;
        self.action_registry.invoke(invocation.clone())?;
        Ok(invocation)
    }

    pub fn into_host(self) -> H {
        self.host
    }
}

impl<H: NativeHost + BlueprintHost> GuiRuntime<H> {
    pub fn dispatch_native_event(&mut self, event: NativeEvent) -> GuiResult<ActionInvocation> {
        let blueprint = self.host.blueprint(event.node).cloned().ok_or_else(|| {
            GuiError::host(format!("native node {} has no blueprint", event.node.get()))
        })?;
        self.dispatch_event(&blueprint, event)
    }
}

impl<H: NativeHost + BlueprintHost + NativeEventHost> GuiRuntime<H> {
    pub fn dispatch_pending_native_events(&mut self) -> GuiResult<Vec<ActionInvocation>> {
        let events = self.host.take_native_events();
        events
            .into_iter()
            .map(|event| self.dispatch_native_event(event))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::platform::{Gtk4Adapter, PlatformCommand, PlatformPlanningHost, WinUiAdapter};

    #[test]
    fn runtime_renders_compiled_jsx_to_platform_host() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);

        let root_id = runtime.render_compiled(&compiled).unwrap();

        let root = runtime.host().node(root_id).unwrap();
        assert_eq!(root.blueprint.widget_class, "gtk::Button");
        assert_eq!(root.blueprint.action.as_deref(), Some("saveDocument"));
    }

    #[test]
    fn runtime_dispatches_native_event_to_registered_web_action() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "save",
              "tag": "Button",
              "props": {"events": {"onClick": "saveDocument"}},
              "children": [{"kind": "text", "key": "text", "value": "Save"}]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("saveDocument");

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let blueprint = runtime.host().node(root_id).unwrap().blueprint.clone();
        let invocation = runtime
            .dispatch_event(
                &blueprint,
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Press),
            )
            .unwrap();

        assert_eq!(invocation.action, "saveDocument");
        assert_eq!(runtime.actions().invocations().len(), 1);
    }

    #[test]
    fn runtime_updates_interaction_state_before_dispatching_action() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "email",
              "tag": "TextField",
              "children": [
                {"kind": "element", "key": "label", "tag": "Label", "children": [
                  {"kind": "text", "key": "label-text", "value": "Email"}
                ]},
                {"kind": "element", "key": "input", "tag": "Input", "props": {
                  "events": {"onChange": "setEmail"}
                }}
              ]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(Gtk4Adapter);
        let mut runtime = GuiRuntime::new(host);
        runtime.actions_mut().register("setEmail");

        let root_id = runtime.render_compiled(&compiled).unwrap();
        let invocation = runtime
            .dispatch_native_event(
                crate::event::NativeEvent::new(root_id, crate::event::NativeEventKind::Change)
                    .value("a@b.c"),
            )
            .unwrap();

        assert_eq!(invocation.action, "setEmail");
        assert_eq!(
            runtime
                .interactions()
                .node(root_id)
                .unwrap()
                .value
                .as_deref(),
            Some("a@b.c")
        );
    }

    #[test]
    fn runtime_renders_compiled_jsx_to_native_command_stream() {
        let compiled: CompiledJsxNode = serde_json::from_str(
            r#"
            {
              "kind": "element",
              "key": "form",
              "tag": "form",
              "props": {"className": "profile-form"},
              "children": [
                {
                  "kind": "element",
                  "key": "email",
                  "tag": "TextField",
                  "children": [
                    {"kind": "element", "key": "label", "tag": "Label", "children": [
                      {"kind": "text", "key": "label-text", "value": "Email"}
                    ]},
                    {"kind": "element", "key": "input", "tag": "Input", "props": {
                      "placeholder": "you@example.com",
                      "events": {"onChange": "setEmail"}
                    }}
                  ]
                },
                {
                  "kind": "element",
                  "key": "save",
                  "tag": "Button",
                  "props": {"events": {"onPress": "saveProfile"}},
                  "children": [{"kind": "text", "key": "save-text", "value": "Save"}]
                }
              ]
            }
            "#,
        )
        .unwrap();
        let host = PlatformPlanningHost::new(WinUiAdapter);
        let mut runtime = GuiRuntime::new(host);

        runtime.render_compiled(&compiled).unwrap();

        let commands = runtime.host().commands();
        assert!(commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.TextBox"
                && blueprint.label.as_deref() == Some("Email")
        )));
        assert!(commands.iter().any(|command| matches!(
            command,
            PlatformCommand::Create {
                blueprint,
                ..
            } if blueprint.widget_class == "Microsoft.UI.Xaml.Controls.Button"
                && blueprint.action.as_deref() == Some("saveProfile")
        )));
    }
}
