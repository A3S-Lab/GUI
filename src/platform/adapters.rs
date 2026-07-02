use crate::host::HostNodeId;
use crate::native::NativeElement;

use super::types::{NativeBackendKind, NativeWidgetBlueprint};
use super::widget_names::widget_blueprint;

pub trait PlatformAdapter: Send + Sync {
    fn kind(&self) -> NativeBackendKind;

    fn blueprint(&self, element: &NativeElement) -> NativeWidgetBlueprint {
        widget_blueprint(self.kind(), element)
    }
}

pub trait BlueprintHost {
    fn blueprint(&self, id: HostNodeId) -> Option<&NativeWidgetBlueprint>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct AppKitAdapter;

#[derive(Debug, Default, Clone, Copy)]
pub struct WinUiAdapter;

#[derive(Debug, Default, Clone, Copy)]
pub struct Gtk4Adapter;

impl PlatformAdapter for AppKitAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::AppKit
    }
}

impl PlatformAdapter for WinUiAdapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::WinUI
    }
}

impl PlatformAdapter for Gtk4Adapter {
    fn kind(&self) -> NativeBackendKind {
        NativeBackendKind::Gtk4
    }
}
