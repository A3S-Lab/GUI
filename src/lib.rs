//! Rust-native GUI primitives for rendering Rust-style RSX UI trees.
//!
//! `a3s-gui` converts semantic component trees into a compact native UI IR,
//! reconciles that tree, and drives a [`NativeHost`] implementation for each
//! platform without embedding a WebView. Rust-style RSX function components
//! and semantic UI component names are supported as native-first input formats.

#![deny(unsafe_code)]
#![recursion_limit = "256"]

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
pub mod renderer;
pub mod rsx;
pub mod rsx_app;
pub mod rsx_ui;
pub mod runtime;
pub mod semantic_ui;
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
pub use app::{NativeRuntimeApp, NativeRuntimeEventBatch, NativeRuntimeEventResponse};
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
pub use compiler::{
    CompiledBinding, CompiledBindingSource, CompiledProps, CompiledRsxNode, ComponentClassVariants,
    RsxCompilerBridge,
};
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
pub use renderer::Renderer;
pub use rsx::{parse_rsx, parse_rsx_file, parse_rsx_source};
pub use rsx_app::{
    ActionHandle, ButtonHook, ComponentCx, ContextHandle, DerivedHandle, PressHook, PropHandle,
    ResourceHandle, RsxActionTransition, RsxComponent, RsxComponentContract, RsxResource,
    RsxRouteTransition, RsxRouter, RsxTemplate, StateHandle, RSX,
};
pub use rsx_ui::{
    ui_autocomplete, ui_badge, ui_badge_variants, ui_breadcrumb, ui_breadcrumbs, ui_button,
    ui_button_variants, ui_card, ui_card_content, ui_card_description, ui_card_footer,
    ui_card_header, ui_card_title, ui_checkbox, ui_checkbox_group, ui_combo_box, ui_dialog,
    ui_disclosure, ui_disclosure_group, ui_disclosure_summary, ui_drop_zone, ui_field_set,
    ui_file_trigger, ui_form, ui_grid_list, ui_grid_list_item, ui_group, ui_heading, ui_input,
    ui_label, ui_legend, ui_link, ui_list_box, ui_list_box_item, ui_menu, ui_menu_item, ui_meter,
    ui_modal, ui_number_field, ui_popover, ui_progress_bar, ui_radio, ui_radio_group,
    ui_search_field, ui_select, ui_select_value, ui_separator, ui_slider, ui_switch, ui_table,
    ui_table_body, ui_table_caption, ui_table_cell, ui_table_column, ui_table_header, ui_table_row,
    ui_tabs, ui_tabs_content, ui_tabs_list, ui_tabs_trigger, ui_tag, ui_tag_group, ui_text,
    ui_text_field, ui_textarea, ui_toast, ui_toast_region, ui_toggle_button,
    ui_toggle_button_group, ui_toolbar, ui_tooltip, ui_tree, ui_tree_item, ui_tree_item_content,
    ui_virtualizer, UiAutocompleteProps, UiBadgeProps, UiBreadcrumbProps, UiBreadcrumbsProps,
    UiButtonProps, UiCardContentProps, UiCardDescriptionProps, UiCardFooterProps,
    UiCardHeaderProps, UiCardProps, UiCardTitleProps, UiCheckboxGroupProps, UiCheckboxProps,
    UiComboBoxProps, UiDialogProps, UiDisclosureGroupProps, UiDisclosureProps,
    UiDisclosureSummaryProps, UiDropZoneProps, UiFieldSetProps, UiFileTriggerProps, UiFormProps,
    UiGridListItemProps, UiGridListProps, UiGroupProps, UiHeadingProps, UiInputProps, UiLabelProps,
    UiLegendProps, UiLinkProps, UiListBoxItemProps, UiListBoxProps, UiMenuItemProps, UiMenuProps,
    UiMeterProps, UiModalProps, UiNumberFieldProps, UiPopoverProps, UiProgressBarProps,
    UiRadioGroupProps, UiRadioProps, UiSearchFieldProps, UiSelectProps, UiSelectValueProps,
    UiSeparatorProps, UiSliderProps, UiSwitchProps, UiTableBodyProps, UiTableCaptionProps,
    UiTableCellProps, UiTableColumnProps, UiTableHeaderProps, UiTableProps, UiTableRowProps,
    UiTabsContentProps, UiTabsListProps, UiTabsProps, UiTabsTriggerProps, UiTagGroupProps,
    UiTagProps, UiTextFieldProps, UiTextProps, UiTextareaProps, UiToastProps, UiToastRegionProps,
    UiToggleButtonGroupProps, UiToggleButtonProps, UiToolbarProps, UiTooltipProps,
    UiTreeItemContentProps, UiTreeItemProps, UiTreeProps, UiVirtualizerProps, UI_BADGE_BASE_CLASS,
    UI_BADGE_CLASS, UI_BUTTON_BASE_CLASS, UI_BUTTON_CLASS, UI_BUTTON_DEFAULT_SIZE_CLASS,
    UI_BUTTON_DEFAULT_VARIANT_CLASS, UI_CARD_CLASS, UI_CARD_CONTENT_CLASS,
    UI_CARD_DESCRIPTION_CLASS, UI_CARD_FOOTER_CLASS, UI_CARD_HEADER_CLASS, UI_CARD_TITLE_CLASS,
    UI_INPUT_CLASS, UI_SEPARATOR_CLASS, UI_TABS_CLASS, UI_TABS_CONTENT_CLASS, UI_TABS_LIST_CLASS,
    UI_TABS_TRIGGER_CLASS, UI_TEXTAREA_CLASS,
};
pub use runtime::{GuiRuntime, HandledNativeEvent};
pub use semantic_ui::{
    use_press, use_press_value, PressProps, SemanticComponent, SemanticElement, SemanticMapper,
    SemanticProps, UsePressProps, UsePressResult,
};
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
