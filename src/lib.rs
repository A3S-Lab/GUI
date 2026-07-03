//! Rust-native GUI primitives for rendering React/JSX-compatible UI trees.
//!
//! `a3s-gui` converts semantic component trees into a compact native UI IR,
//! reconciles that tree, and drives a [`NativeHost`] implementation for each
//! platform without embedding a WebView. React Aria-compatible component names
//! and props are supported as one input format.

#![deny(unsafe_code)]

pub mod accessibility;
pub mod app;
#[cfg(all(feature = "appkit", target_os = "macos"))]
pub mod appkit;
#[cfg(all(feature = "appkit-native", target_os = "macos"))]
pub mod appkit_native;
pub mod backend;
pub mod compiler;
mod css_text;
pub mod error;
pub mod event;
pub mod geometry;
#[cfg(feature = "gtk4")]
pub mod gtk4;
#[cfg(all(feature = "gtk4-native", target_os = "linux"))]
pub mod gtk4_native;
pub mod host;
pub mod html;
pub mod interaction;
pub mod native;
#[cfg(any(
    all(feature = "appkit-native", target_os = "macos"),
    all(feature = "gtk4-native", target_os = "linux"),
    all(feature = "winui-native", target_os = "windows")
))]
mod native_backends;
pub mod platform;
pub mod protocol;
pub mod react_aria;
pub mod renderer;
pub mod runtime;
pub mod style;
pub mod svg;
pub mod web;
#[cfg(feature = "winui")]
pub mod winui;
#[cfg(all(feature = "winui-native", target_os = "windows"))]
pub mod winui_native;

pub use accessibility::{
    AccessibilityDescriptionProps, AccessibilityNode, AccessibilityRelationshipProps,
    AccessibilityRole, AccessibilityStateProps, AccessibilityStructureProps, AccessibilityTreeHost,
};
pub use app::{NativeRuntimeApp, NativeRuntimeEventResponse};
#[cfg(all(feature = "appkit", target_os = "macos"))]
pub use appkit::{
    AppKitCommandExecutor, AppKitHandleAdapter, AppKitHandleCommandExecutor, AppKitHandleDriver,
    AppKitNativeHandle, AppKitNativeHandleState, AppKitNativeObject, AppKitWidgetDriver,
    AppKitWidgetKind,
};
#[cfg(all(feature = "appkit-native", target_os = "macos"))]
pub use appkit_native::{
    AppKitComboBoxItem, AppKitEventWait, AppKitNativeSurface, AppKitNativeSurfaceAdapter,
    AppKitNativeSurfaceCommandExecutor, AppKitNativeSurfaceDriver, AppKitOsHandle, AppKitOsWidget,
    AppKitRuntimeApp, AppKitRuntimeHost,
};
pub use backend::{
    CommandExecutingHost, DriverCommandExecutor, HandleWidgetDriver, NativeEventHost,
    NativeEventSource, NativeHandleAdapter, NativeWidgetDriver, NativeWidgetSurface,
    PlatformCommandExecutor, RecordedNativeObject, RecordingBackend, SurfaceHandleAdapter,
};
pub use compiler::{CompiledJsxNode, CompiledProps, ReactCompilerBridge};
pub use error::{GuiError, GuiResult};
pub use event::{
    ActionInvocation, ActionRegistry, EventRouter, NativeEvent, NativeEventKind, RegisteredAction,
};
pub use geometry::{Orientation, Rect, Size};
#[cfg(feature = "gtk4")]
pub use gtk4::{
    Gtk4CommandExecutor, Gtk4HandleAdapter, Gtk4HandleCommandExecutor, Gtk4HandleDriver,
    Gtk4NativeHandle, Gtk4NativeHandleState, Gtk4NativeObject, Gtk4WidgetDriver, Gtk4WidgetKind,
};
#[cfg(all(feature = "gtk4-native", target_os = "linux"))]
pub use gtk4_native::{
    Gtk4DropDownItem, Gtk4EventWait, Gtk4NativeSurface, Gtk4NativeSurfaceAdapter,
    Gtk4NativeSurfaceCommandExecutor, Gtk4NativeSurfaceDriver, Gtk4NotebookTab, Gtk4OsHandle,
    Gtk4OsWidget, Gtk4RuntimeApp, Gtk4RuntimeHost,
};
pub use host::{HeadlessHost, HostNodeId, HostOperation, NativeHost};
pub use html::{
    HtmlActivationProps, HtmlCollectionProps, HtmlDialogProps, HtmlFormAssociationProps,
    HtmlMicrodataProps, HtmlResourcePolicyProps, HtmlShadowProps, HtmlTextAnnotationProps,
    HTML_CONFORMING_ELEMENTS, HTML_ELEMENTS, HTML_TAG_METADATA_KEY,
};
pub use interaction::{InteractionChange, InteractionNodeState, InteractionState};
pub use native::{ElementKey, NativeElement, NativeProps, NativeRole};
pub use platform::{
    native_widget_name, AppKitAdapter, BlueprintHost, Gtk4Adapter, NativeBackendKind,
    NativeConfigValueChange, NativeControlState, NativeWidgetBlueprint, NativeWidgetConfig,
    NativeWidgetConfigPatch, NativeWidgetSetter, PlatformAdapter, PlatformCommand,
    PlatformPlannedNode, PlatformPlanningHost, WinUiAdapter,
};
pub use protocol::{
    HostEvent, HostEventResponse, NativeAppEventResponse, NativeHostEventResponse,
    NativeProtocolApp, NativeProtocolSession, NativeRenderResponse, RenderedFrame, UiAction,
    UiFrame, WindowOptions,
};
pub use react_aria::{AriaComponent, AriaElement, AriaProps, ReactAriaMapper};
pub use renderer::Renderer;
pub use runtime::{GuiRuntime, HandledNativeEvent};
pub use style::{
    AlignItems, BackfaceVisibility, BackgroundAttachment, BackgroundBox, BlendMode, BorderCollapse,
    BorderStyle, BoxDecorationBreak, BoxSizing, CaptionSide, ClearMode, ContainerType,
    ContentVisibility, CornerRadii, CornerRadius, DisplayMode, EdgeBorderStyles, EdgeColors,
    EdgeInsets, FillRule, FlexWrap, FloatMode, FontStyle, FontWeight, GridAutoFlow, HyphensMode,
    IsolationMode, JustifyContent, ListStylePosition, LogicalBorderStyles, LogicalCornerRadii,
    LogicalEdgeColors, LogicalEdgeInsets, ObjectFit, OverflowMode, OverflowWrapMode,
    OverscrollBehavior, PointerEvents, PortableStyle, PositionMode, ResizeMode, ScrollBehavior,
    SelfAlignment, StrokeLineCap, StrokeLineJoin, StyleColor, StyleLength, StyleTime, TableLayout,
    TextAlign, TextDecorationStyle, TextDirection, TextOrientation, TextOverflow, TextTransform,
    TextWrapMode, UnicodeBidi, UserSelect, VisibilityMode, WhiteSpaceMode, WordBreakMode,
};
pub use svg::{SVG_ELEMENTS, SVG_TAG_METADATA_KEY};
pub use web::WebProps;
#[cfg(feature = "winui")]
pub use winui::{
    WinUiCommandExecutor, WinUiHandleAdapter, WinUiHandleCommandExecutor, WinUiHandleDriver,
    WinUiNativeHandle, WinUiNativeHandleState, WinUiNativeObject, WinUiWidgetDriver,
    WinUiWidgetKind,
};
#[cfg(all(feature = "winui-native", target_os = "windows"))]
pub use winui_native::{
    WinUiComboBoxItem, WinUiEventWait, WinUiNativeSurface, WinUiNativeSurfaceAdapter,
    WinUiNativeSurfaceCommandExecutor, WinUiNativeSurfaceDriver, WinUiOsHandle, WinUiOsWidget,
    WinUiRuntimeApp, WinUiRuntimeHost, WinUiTabItem,
};
