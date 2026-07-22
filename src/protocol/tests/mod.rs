use super::*;
use crate::accessibility::AccessibilityRole;
use crate::backend::{CommandExecutingHost, RecordingBackend};
use crate::event::NativeEventKind;
use crate::host::HeadlessHost;
use crate::platform::{Gtk4Adapter, NativeWidgetSetter};

#[derive(Default)]
struct FailingUpdateHost {
    inner: HeadlessHost,
    fail_updates: bool,
}

impl NativeHost for FailingUpdateHost {
    fn create(&mut self, element: &NativeElement) -> GuiResult<HostNodeId> {
        self.inner.create(element)
    }

    fn update(&mut self, id: HostNodeId, props: &NativeProps) -> GuiResult<()> {
        if self.fail_updates {
            return Err(GuiError::host("forced host update failure"));
        }
        self.inner.update(id, props)
    }

    fn insert_child(
        &mut self,
        parent: HostNodeId,
        child: HostNodeId,
        index: usize,
    ) -> GuiResult<()> {
        self.inner.insert_child(parent, child, index)
    }

    fn remove(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.inner.remove(id)
    }

    fn set_root(&mut self, id: HostNodeId) -> GuiResult<()> {
        self.inner.set_root(id)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct CounterState {
    count: u32,
}

fn counter_frame(state: &CounterState) -> GuiResult<UiFrame> {
    serde_json::from_value(serde_json::json!({
        "frameId": "counter",
        "actions": [{"id": "increment"}],
        "root": {
            "kind": "element",
            "key": "increment",
            "tag": "Button",
            "props": {"events": {"onPress": "increment"}},
            "children": [
                {
                    "kind": "text",
                    "key": "label",
                    "value": format!("Count {}", state.count)
                }
            ]
        }
    }))
    .map_err(|error| GuiError::invalid_tree(format!("invalid counter frame: {error}")))
}

fn counter_reduce(state: &mut CounterState, invocation: &ActionInvocation) -> GuiResult<()> {
    match invocation.action.as_str() {
        "increment" => {
            state.count += 1;
            Ok(())
        }
        other => Err(GuiError::host(format!("unexpected action {other}"))),
    }
}

#[cfg(feature = "authoring")]
fn rsx_counter_frame(state: &CounterState) -> GuiResult<UiFrame> {
    let state = serde_json::json!({ "count": state.count });
    UiFrame::from_rsx_source_with_state(
        "rsx-counter",
        r#"
            <Button key="increment" onPress={increment}>
              Count {state.count}
            </Button>
            "#,
        &state,
    )
}

mod accessibility;
mod events;
mod frame;
mod v1;
mod values;
mod window_wire;
