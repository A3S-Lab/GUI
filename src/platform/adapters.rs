use crate::capability::NativeCapabilities;
use crate::host::HostNodeId;
use crate::native::NativeElement;

use super::types::{NativeBackendKind, NativeWidgetBlueprint};
use super::widget_names::widget_blueprint;

pub trait PlatformAdapter: Send + Sync {
    fn kind(&self) -> NativeBackendKind;

    fn capabilities(&self) -> NativeCapabilities {
        NativeCapabilities::for_backend(self.kind())
    }

    fn blueprint(&self, element: &NativeElement) -> NativeWidgetBlueprint {
        widget_blueprint(self.kind(), element)
    }
}

pub trait BlueprintHost {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint>;
}

/// Nonvisual planner used by protocol and transaction tests.
///
/// Visible applications use the self-drawn platform runtime instead of a
/// widget adapter.
#[derive(Debug, Default, Clone, Copy)]
pub struct HeadlessAdapter;

impl PlatformAdapter for HeadlessAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::Headless
    }
}
