use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::{GuiError, GuiResult};
use crate::event::ActionInvocation;
use crate::i18n::{NumberFormatOptions, NumberFormatStyle};
use crate::selection::{CollectionKey, Selection};
use crate::semantic_ui::{
    use_autocomplete_value, use_breadcrumbs_value, use_button_value, use_calendar_cell_value,
    use_calendar_value, use_checkbox_group_value, use_checkbox_value, use_collection_item_value,
    use_collection_section_value, use_collection_value, use_color_area_value,
    use_color_field_value, use_color_picker_value, use_color_slider_value,
    use_color_swatch_picker_item_value, use_color_swatch_picker_value, use_color_swatch_value,
    use_color_thumb_value, use_color_wheel_value, use_combo_box_display_value, use_combo_box_value,
    use_date_field_value, use_date_input_value, use_date_picker_value, use_date_range_picker_value,
    use_date_segment_value, use_description_value, use_disclosure_group_value,
    use_disclosure_value, use_drag_value, use_drop_value, use_drop_zone_value,
    use_field_error_value, use_field_value, use_file_trigger_value, use_focus_ring_value,
    use_focus_scope_value, use_focus_within_value, use_focusable_value, use_form_value,
    use_grid_list_header_value, use_group_value, use_heading_value, use_i18n_value,
    use_keyboard_value, use_label_value, use_legend_value, use_link_value,
    use_list_box_header_value, use_load_more_item_value, use_menu_item_value, use_menu_value,
    use_overlay_value, use_press_value, use_radio_group_value, use_radio_value,
    use_range_calendar_value, use_range_value, use_select_display_value, use_select_value,
    use_selection_value, use_submenu_trigger_value, use_switch_value, use_tab_list_value,
    use_tab_panel_value, use_tab_value, use_table_caption_value, use_table_cell_value,
    use_table_column_value, use_table_row_value, use_table_section_value, use_table_value,
    use_text_field_value, use_text_value, use_time_field_value, use_toggle_button_group_value,
    use_toggle_button_value, use_toggle_value, use_tree_header_value, use_tree_item_value,
    use_tree_value, use_visually_hidden_value, AutocompleteProps, BreadcrumbsProps, ButtonProps,
    CalendarCellProps, CalendarProps, CheckboxGroupProps, CheckboxProps, CollectionItemProps,
    CollectionProps, CollectionSectionKind, CollectionSectionProps, ColorAreaProps,
    ColorFieldInputProps, ColorFieldProps, ColorPickerProps, ColorRangeProps,
    ColorSwatchPickerItemProps, ColorSwatchPickerProps, ColorSwatchProps, ColorThumbProps,
    ComboBoxInputProps, ComboBoxProps, DateFieldInputProps, DateFieldProps, DateInputProps,
    DatePickerInputProps, DatePickerProps, DatePickerTriggerProps, DateRangePickerInputProps,
    DateRangePickerProps, DateSegmentProps, DisclosureGroupProps, DisclosurePanelProps,
    DisclosureProps, DisclosureTriggerProps, DragButtonProps, DragProps, DropButtonProps,
    DropProps, DropZoneProps, FieldProps, FileTriggerProps, FocusProps, FocusRingProps,
    FocusScopeProps, FocusWithinProps, FormProps, GroupProps, HeadingProps, I18nProps, LinkProps,
    LoadMoreItemProps, MenuItemProps, MenuProps, OverlayProps, OverlayTriggerProps, PressProps,
    RadioGroupProps, RadioProps, RangeCalendarProps, RangeInputProps, RangeProps, SelectProps,
    SelectValueProps, SelectionInputTriggerProps, SelectionProps, SubmenuTriggerProps, SwitchProps,
    TabListProps, TabPanelProps, TabProps, TableCaptionProps, TableCellProps, TableColumnProps,
    TableProps, TableRowProps, TableSectionKind, TableSectionProps, TextFieldProps, TextInputProps,
    TextProps, TimeFieldInputProps, TimeFieldProps, ToggleButtonGroupProps, ToggleButtonProps,
    ToggleProps, TreeItemProps, TreeProps, UseAutocompleteProps, UseBreadcrumbsProps,
    UseButtonProps, UseCalendarCellProps, UseCalendarProps, UseCheckboxGroupProps,
    UseCheckboxProps, UseCollectionItemProps, UseCollectionProps, UseCollectionSectionProps,
    UseColorAreaProps, UseColorFieldProps, UseColorPickerProps, UseColorRangeProps,
    UseColorSwatchPickerItemProps, UseColorSwatchPickerProps, UseColorSwatchProps,
    UseColorThumbProps, UseComboBoxDisplayProps, UseComboBoxProps, UseDateFieldProps,
    UseDateInputProps, UseDatePickerProps, UseDateRangePickerProps, UseDateSegmentProps,
    UseDisclosureGroupProps, UseDisclosureProps, UseDragProps, UseDropProps, UseDropZoneProps,
    UseFieldProps, UseFileTriggerProps, UseFocusRingProps, UseFocusScopeProps, UseFocusWithinProps,
    UseFocusableProps, UseFormProps, UseGroupProps, UseHeadingProps, UseI18nProps, UseLinkProps,
    UseLoadMoreItemProps, UseMenuItemProps, UseMenuProps, UseOverlayProps, UsePressProps,
    UseRadioGroupProps, UseRadioProps, UseRangeCalendarProps, UseRangeProps, UseSelectDisplayProps,
    UseSelectProps, UseSelectionProps, UseSubmenuTriggerProps, UseSwitchProps, UseTabListProps,
    UseTabPanelProps, UseTabProps, UseTableCaptionProps, UseTableCellProps, UseTableColumnProps,
    UseTableProps, UseTableRowProps, UseTableSectionProps, UseTextFieldProps, UseTextProps,
    UseTimeFieldProps, UseToggleButtonGroupProps, UseToggleButtonProps, UseToggleProps,
    UseTreeItemProps, UseTreeProps,
};
use crate::semantic_ui::{
    use_clipboard_value, use_hover_value, use_keyboard_interaction_value, use_long_press_value,
    use_move_value, ClipboardProps, HoverProps, KeyboardInteractionProps, LongPressProps,
    MoveProps, UseClipboardProps, UseHoverProps, UseKeyboardInteractionProps, UseLongPressProps,
    UseMoveProps,
};
use crate::semantic_ui::{
    use_drop_indicator_value, use_selection_indicator_value, use_separator_value,
    use_toolbar_value, DropIndicatorProps, SelectionIndicatorProps, SeparatorProps, ToolbarProps,
    UseDropIndicatorProps, UseSelectionIndicatorProps, UseSeparatorProps, UseToolbarProps,
};
use crate::semantic_ui::{use_landmark_value, LandmarkProps, UseLandmarkProps};
use crate::semantic_ui::{
    use_number_field_value, NumberFieldButtonProps, NumberFieldInputProps, NumberFieldProps,
    UseNumberFieldProps,
};
use crate::semantic_ui::{
    use_overlay_position_value, OverlayArrowProps, OverlayPositionProps, UseOverlayPositionProps,
};
use crate::semantic_ui::{
    use_slider_fill_value, use_slider_output_value, use_slider_track_value, SliderFillProps,
    SliderOutputProps, SliderTrackProps, UseSliderFillProps, UseSliderOutputProps,
    UseSliderTrackProps,
};
use crate::semantic_ui::{
    use_toast_region_value, use_toast_value, ToastProps, ToastRegionProps, UseToastProps,
    UseToastRegionProps,
};
use crate::semantic_ui::{use_virtualizer_value, UseVirtualizerProps, VirtualizerProps};

use super::{ComponentRegistry, RsxComponent, RsxResource};

type ComponentRegistration<S> =
    Box<dyn FnOnce(RsxComponent<S>) -> GuiResult<RsxComponent<S>> + Send>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RSX {
    source: String,
}

impl RSX {
    pub fn source(source: impl Into<String>) -> Self {
        Self {
            source: source.into(),
        }
    }

    pub fn as_source(&self) -> &str {
        &self.source
    }

    pub fn into_source(self) -> String {
        self.source
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

pub type SelectorHandle<T = JsonValue> = StateHandle<T>;

impl<T> StateHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("state.{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReactiveHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

impl<T> ReactiveHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("state.{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

impl<T> PropHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("props.{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DerivedHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

impl<T> DerivedHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("derived.{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

impl<T> ContextHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("context.{}", self.path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceHandle<T = JsonValue> {
    path: String,
    _value: PhantomData<fn() -> T>,
}

impl<T> ResourceHandle<T> {
    fn new(path: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            _value: PhantomData,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn binding_path(&self) -> String {
        format!("resource.{}", self.path)
    }
}

#[derive(Debug)]
pub struct RefHandle<T> {
    value: Arc<Mutex<T>>,
}

impl<T> Clone for RefHandle<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
        }
    }
}

impl<T> RefHandle<T> {
    fn new(initial: T) -> Self {
        Self {
            value: Arc::new(Mutex::new(initial)),
        }
    }

    pub fn with<R>(&self, read: impl FnOnce(&T) -> R) -> GuiResult<R> {
        let guard = self.value.lock().map_err(|_| {
            GuiError::invalid_tree("RSX ref handle lock was poisoned while reading")
        })?;
        Ok(read(&guard))
    }

    pub fn with_mut<R>(&self, write: impl FnOnce(&mut T) -> R) -> GuiResult<R> {
        let mut guard = self.value.lock().map_err(|_| {
            GuiError::invalid_tree("RSX ref handle lock was poisoned while writing")
        })?;
        Ok(write(&mut guard))
    }

    pub fn set(&self, value: T) -> GuiResult<()> {
        self.with_mut(|slot| *slot = value)
    }
}

impl<T: Clone> RefHandle<T> {
    pub fn current(&self) -> GuiResult<T> {
        self.with(Clone::clone)
    }
}

struct SyncExternalStoreState<T> {
    snapshot: T,
    version: u64,
    next_subscriber: usize,
    subscribers: BTreeMap<usize, Arc<dyn Fn() -> GuiResult<()> + Send + Sync>>,
}

pub struct SyncExternalStore<T> {
    state: Arc<Mutex<SyncExternalStoreState<T>>>,
}

impl<T> Clone for SyncExternalStore<T> {
    fn clone(&self) -> Self {
        Self {
            state: Arc::clone(&self.state),
        }
    }
}

impl<T> std::fmt::Debug for SyncExternalStore<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.state.lock() {
            Ok(state) => f
                .debug_struct("SyncExternalStore")
                .field("snapshot", &state.snapshot)
                .field("version", &state.version)
                .field("subscriber_count", &state.subscribers.len())
                .finish(),
            Err(_) => f
                .debug_struct("SyncExternalStore")
                .field("poisoned", &true)
                .finish(),
        }
    }
}

impl<T> SyncExternalStore<T> {
    pub fn new(snapshot: T) -> Self {
        Self {
            state: Arc::new(Mutex::new(SyncExternalStoreState {
                snapshot,
                version: 0,
                next_subscriber: 0,
                subscribers: BTreeMap::new(),
            })),
        }
    }

    pub fn snapshot(&self) -> GuiResult<T>
    where
        T: Clone,
    {
        let state = self.state.lock().map_err(|_| {
            GuiError::invalid_tree("RSX external store lock was poisoned while reading")
        })?;
        Ok(state.snapshot.clone())
    }

    pub fn version(&self) -> GuiResult<u64> {
        let state = self.state.lock().map_err(|_| {
            GuiError::invalid_tree("RSX external store lock was poisoned while reading")
        })?;
        Ok(state.version)
    }

    pub fn set(&self, snapshot: T) -> GuiResult<()> {
        let subscribers = {
            let mut state = self.state.lock().map_err(|_| {
                GuiError::invalid_tree("RSX external store lock was poisoned while writing")
            })?;
            state.snapshot = snapshot;
            state.version += 1;
            state.subscribers.values().cloned().collect::<Vec<_>>()
        };
        for subscriber in subscribers {
            subscriber()?;
        }
        Ok(())
    }

    pub fn update(&self, update: impl FnOnce(&mut T)) -> GuiResult<()> {
        let subscribers = {
            let mut state = self.state.lock().map_err(|_| {
                GuiError::invalid_tree("RSX external store lock was poisoned while writing")
            })?;
            update(&mut state.snapshot);
            state.version += 1;
            state.subscribers.values().cloned().collect::<Vec<_>>()
        };
        for subscriber in subscribers {
            subscriber()?;
        }
        Ok(())
    }

    pub fn subscribe(
        &self,
        subscriber: impl Fn() -> GuiResult<()> + Send + Sync + 'static,
    ) -> GuiResult<SyncExternalStoreSubscription<T>> {
        let mut state = self.state.lock().map_err(|_| {
            GuiError::invalid_tree("RSX external store lock was poisoned while subscribing")
        })?;
        let id = state.next_subscriber;
        state.next_subscriber += 1;
        state.subscribers.insert(id, Arc::new(subscriber));
        Ok(SyncExternalStoreSubscription {
            store: self.clone(),
            id,
            active: true,
        })
    }

    fn unsubscribe(&self, id: usize) -> GuiResult<()> {
        let mut state = self.state.lock().map_err(|_| {
            GuiError::invalid_tree("RSX external store lock was poisoned while unsubscribing")
        })?;
        state.subscribers.remove(&id);
        Ok(())
    }
}

pub struct SyncExternalStoreSubscription<T> {
    store: SyncExternalStore<T>,
    id: usize,
    active: bool,
}

impl<T> SyncExternalStoreSubscription<T> {
    pub fn unsubscribe(&mut self) -> GuiResult<()> {
        if self.active {
            self.store.unsubscribe(self.id)?;
            self.active = false;
        }
        Ok(())
    }
}

impl<T> Drop for SyncExternalStoreSubscription<T> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.store.unsubscribe(self.id);
            self.active = false;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionStateSnapshot<D = JsonValue, E = String> {
    pub pending: bool,
    pub data: Option<D>,
    pub error: Option<E>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FormStatusSnapshot<D = JsonValue, E = String> {
    pub pending: bool,
    pub action: Option<String>,
    pub data: Option<D>,
    pub error: Option<E>,
}

#[derive(Debug)]
struct ActionStateInner<D, E> {
    pending: bool,
    data: Option<D>,
    error: Option<E>,
}

#[derive(Debug)]
pub struct ActionStateHandle<D = JsonValue, E = String> {
    action: String,
    inner: Arc<Mutex<ActionStateInner<D, E>>>,
}

impl<D, E> Clone for ActionStateHandle<D, E> {
    fn clone(&self) -> Self {
        Self {
            action: self.action.clone(),
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<D, E> ActionStateHandle<D, E> {
    fn new(action: impl Into<String>) -> Self {
        Self {
            action: action.into(),
            inner: Arc::new(Mutex::new(ActionStateInner {
                pending: false,
                data: None,
                error: None,
            })),
        }
    }

    pub fn action(&self) -> &str {
        &self.action
    }

    pub fn set_pending(&self, pending: bool) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX action state lock was poisoned while setting pending")
        })?;
        inner.pending = pending;
        Ok(())
    }

    pub fn set_data(&self, data: D) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX action state lock was poisoned while setting data")
        })?;
        inner.pending = false;
        inner.data = Some(data);
        inner.error = None;
        Ok(())
    }

    pub fn set_error(&self, error: E) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX action state lock was poisoned while setting error")
        })?;
        inner.pending = false;
        inner.error = Some(error);
        Ok(())
    }

    pub fn clear(&self) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX action state lock was poisoned while clearing")
        })?;
        inner.pending = false;
        inner.data = None;
        inner.error = None;
        Ok(())
    }
}

impl<D, E> ActionStateHandle<D, E>
where
    D: Clone,
    E: Clone,
{
    pub fn snapshot(&self) -> GuiResult<ActionStateSnapshot<D, E>> {
        let inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX action state lock was poisoned while reading")
        })?;
        Ok(ActionStateSnapshot {
            pending: inner.pending,
            data: inner.data.clone(),
            error: inner.error.clone(),
        })
    }

    pub fn form_status(&self) -> GuiResult<FormStatusSnapshot<D, E>> {
        let snapshot = self.snapshot()?;
        Ok(FormStatusSnapshot {
            pending: snapshot.pending,
            action: Some(self.action.clone()),
            data: snapshot.data,
            error: snapshot.error,
        })
    }
}

impl<D, E> From<ActionStateHandle<D, E>> for String {
    fn from(handle: ActionStateHandle<D, E>) -> Self {
        handle.action
    }
}

impl<D, E> From<&ActionStateHandle<D, E>> for String {
    fn from(handle: &ActionStateHandle<D, E>) -> Self {
        handle.action.clone()
    }
}

#[derive(Debug)]
struct OptimisticState<T> {
    base: Option<T>,
    optimistic: Option<T>,
}

#[derive(Debug)]
pub struct OptimisticHandle<T> {
    inner: Arc<Mutex<OptimisticState<T>>>,
}

impl<T> Clone for OptimisticHandle<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> OptimisticHandle<T> {
    fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(OptimisticState {
                base: None,
                optimistic: None,
            })),
        }
    }

    pub fn set_base(&self, base: T) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX optimistic state lock was poisoned while setting base")
        })?;
        inner.base = Some(base);
        Ok(())
    }

    pub fn set_optimistic(&self, optimistic: T) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree(
                "RSX optimistic state lock was poisoned while setting optimistic value",
            )
        })?;
        inner.optimistic = Some(optimistic);
        Ok(())
    }

    pub fn clear_optimistic(&self) -> GuiResult<()> {
        let mut inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree(
                "RSX optimistic state lock was poisoned while clearing optimistic value",
            )
        })?;
        inner.optimistic = None;
        Ok(())
    }
}

impl<T> OptimisticHandle<T>
where
    T: Clone,
{
    pub fn current(&self) -> GuiResult<T> {
        let inner = self.inner.lock().map_err(|_| {
            GuiError::invalid_tree("RSX optimistic state lock was poisoned while reading")
        })?;
        inner
            .optimistic
            .clone()
            .or_else(|| inner.base.clone())
            .ok_or_else(|| GuiError::invalid_tree("RSX optimistic state has no base value yet"))
    }
}

pub struct EffectEventHandle<S> {
    event: Arc<dyn Fn(&mut S) -> GuiResult<()> + Send + Sync>,
}

impl<S> Clone for EffectEventHandle<S> {
    fn clone(&self) -> Self {
        Self {
            event: Arc::clone(&self.event),
        }
    }
}

impl<S> EffectEventHandle<S> {
    fn new(event: impl Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static) -> Self {
        Self {
            event: Arc::new(event),
        }
    }

    pub fn call(&self, state: &mut S) -> GuiResult<()> {
        (self.event)(state)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionHandle {
    id: String,
}

impl ActionHandle {
    fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

impl From<ActionHandle> for String {
    fn from(handle: ActionHandle) -> Self {
        handle.id
    }
}

impl From<&ActionHandle> for String {
    fn from(handle: &ActionHandle) -> Self {
        handle.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PressHook {
    pub press_props: PropHandle<PressProps>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkHook {
    pub link_props: PropHandle<LinkProps>,
    pub href: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverHook {
    pub hover_props: PropHandle<HoverProps>,
    pub is_hovered: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardInteractionHook {
    pub keyboard_interaction_props: PropHandle<KeyboardInteractionProps>,
    pub is_keyboard_active: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardHook {
    pub clipboard_props: PropHandle<ClipboardProps>,
    pub is_clipboard_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LongPressHook {
    pub long_press_props: PropHandle<LongPressProps>,
    pub is_pressed: PropHandle<bool>,
    pub is_long_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveHook {
    pub move_props: PropHandle<MoveProps>,
    pub is_moving: PropHandle<bool>,
    pub x_delta: PropHandle<f64>,
    pub y_delta: PropHandle<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonHook {
    pub button_props: PropHandle<ButtonProps>,
    pub press_props: PropHandle<ButtonProps>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileTriggerHook {
    pub file_trigger_props: PropHandle<FileTriggerProps>,
    pub accepted_file_types: PropHandle<Option<String>>,
    pub allows_multiple: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropZoneHook {
    pub drop_zone_props: PropHandle<DropZoneProps>,
    pub label: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DragHook {
    pub drag_props: PropHandle<DragProps>,
    pub drag_button_props: PropHandle<DragButtonProps>,
    pub is_dragging: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropHook {
    pub drop_props: PropHandle<DropProps>,
    pub drop_button_props: PropHandle<DropButtonProps>,
    pub label: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_drop_target: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupHook {
    pub group_props: PropHandle<GroupProps>,
    pub label: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
    pub is_hovered: PropHandle<bool>,
    pub is_focused: PropHandle<bool>,
    pub is_focus_visible: PropHandle<bool>,
    pub is_focus_within: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualizerHook {
    pub virtualizer_props: PropHandle<VirtualizerProps>,
    pub label: PropHandle<Option<String>>,
    pub layout: PropHandle<String>,
    pub orientation: PropHandle<String>,
    pub item_count: PropHandle<usize>,
    pub estimated_item_size: PropHandle<u32>,
    pub visible_start: PropHandle<usize>,
    pub visible_end: PropHandle<usize>,
    pub overscan: PropHandle<usize>,
    pub gap: PropHandle<u32>,
    pub padding: PropHandle<u32>,
    pub is_scrolling: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusableHook {
    pub focus_props: PropHandle<FocusProps>,
    pub is_focused: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusWithinHook {
    pub focus_within_props: PropHandle<FocusWithinProps>,
    pub is_focus_within: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusRingHook {
    pub focus_ring_props: PropHandle<FocusRingProps>,
    pub is_focused: PropHandle<bool>,
    pub is_focus_visible: PropHandle<bool>,
    pub is_focus_within: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FocusScopeHook {
    pub focus_scope_props: PropHandle<FocusScopeProps>,
    pub contain: PropHandle<bool>,
    pub restore_focus: PropHandle<bool>,
    pub auto_focus: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FormHook {
    pub form_props: PropHandle<FormProps>,
    pub label: PropHandle<Option<String>>,
    pub validation_behavior: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub no_validate: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreadcrumbsHook {
    pub breadcrumbs_props: PropHandle<BreadcrumbsProps>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LandmarkHook {
    pub landmark_props: PropHandle<LandmarkProps>,
    pub landmark_kind: PropHandle<String>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldHook {
    pub field_props: PropHandle<FieldProps>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckboxHook {
    pub checkbox_props: PropHandle<CheckboxProps>,
    pub value: PropHandle<Option<String>>,
    pub is_checked: PropHandle<bool>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckboxGroupHook {
    pub checkbox_group_props: PropHandle<CheckboxGroupProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SeparatorHook {
    pub separator_props: PropHandle<SeparatorProps>,
    pub orientation: PropHandle<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolbarHook {
    pub toolbar_props: PropHandle<ToolbarProps>,
    pub label: PropHandle<Option<String>>,
    pub orientation: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DropIndicatorHook {
    pub drop_indicator_props: PropHandle<DropIndicatorProps>,
    pub orientation: PropHandle<String>,
    pub is_target: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionIndicatorHook {
    pub selection_indicator_props: PropHandle<SelectionIndicatorProps>,
    pub label: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct I18nHook {
    pub i18n_props: PropHandle<I18nProps>,
    pub locale: PropHandle<Option<String>>,
    pub direction: PropHandle<Option<String>>,
    pub is_rtl: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionHook {
    pub collection_props: PropHandle<CollectionProps>,
    pub label: PropHandle<Option<String>>,
    pub item_count: PropHandle<usize>,
    pub is_empty: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionSectionHook {
    pub collection_section_props: PropHandle<CollectionSectionProps>,
    pub label: PropHandle<Option<String>>,
    pub collection_kind: PropHandle<CollectionSectionKind>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionItemHook {
    pub collection_item_props: PropHandle<CollectionItemProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_expanded: PropHandle<Option<bool>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadMoreItemHook {
    pub load_more_item_props: PropHandle<LoadMoreItemProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub action_value: PropHandle<Option<String>>,
    pub action_payload: PropHandle<JsonValue>,
    pub is_loading: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuHook {
    pub menu_props: PropHandle<MenuProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MenuItemHook {
    pub menu_item_props: PropHandle<MenuItemProps>,
    pub is_disabled: PropHandle<bool>,
    pub is_selected: PropHandle<bool>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmenuTriggerHook {
    pub submenu_trigger_props: PropHandle<SubmenuTriggerProps>,
    pub is_disabled: PropHandle<bool>,
    pub is_pressed: PropHandle<bool>,
    pub is_open: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioHook {
    pub radio_props: PropHandle<RadioProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_checked: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RadioGroupHook {
    pub radio_group_props: PropHandle<RadioGroupProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selection_mode: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabHook {
    pub tab_props: PropHandle<TabProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabListHook {
    pub tab_list_props: PropHandle<TabListProps>,
    pub label: PropHandle<Option<String>>,
    pub orientation: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabPanelHook {
    pub tab_panel_props: PropHandle<TabPanelProps>,
    pub value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableHook {
    pub table_props: PropHandle<TableProps>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableSectionHook {
    pub table_section_props: PropHandle<TableSectionProps>,
    pub section_kind: PropHandle<TableSectionKind>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableRowHook {
    pub table_row_props: PropHandle<TableRowProps>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCellHook {
    pub table_cell_props: PropHandle<TableCellProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableColumnHook {
    pub table_column_props: PropHandle<TableColumnProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableCaptionHook {
    pub table_caption_props: PropHandle<TableCaptionProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextHook {
    pub text_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelHook {
    pub label_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptionHook {
    pub description_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldErrorHook {
    pub field_error_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegendHook {
    pub legend_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisuallyHiddenHook {
    pub visually_hidden_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardHook {
    pub keyboard_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBoxHeaderHook {
    pub list_box_header_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridListHeaderHook {
    pub grid_list_header_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeHeaderHook {
    pub tree_header_props: PropHandle<TextProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeadingHook {
    pub heading_props: PropHandle<HeadingProps>,
    pub label: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub level: PropHandle<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateFieldHook {
    pub date_field_props: PropHandle<DateFieldProps>,
    pub date_field_input_props: PropHandle<DateFieldInputProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub granularity: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimeFieldHook {
    pub time_field_props: PropHandle<TimeFieldProps>,
    pub time_field_input_props: PropHandle<TimeFieldInputProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub granularity: PropHandle<Option<String>>,
    pub hour_cycle: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateInputHook {
    pub date_input_props: PropHandle<DateInputProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateSegmentHook {
    pub date_segment_props: PropHandle<DateSegmentProps>,
    pub segment_type: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub is_placeholder: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarHook {
    pub calendar_props: PropHandle<CalendarProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeCalendarHook {
    pub range_calendar_props: PropHandle<RangeCalendarProps>,
    pub label: PropHandle<Option<String>>,
    pub start_value: PropHandle<Option<String>>,
    pub end_value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarCellHook {
    pub calendar_cell_props: PropHandle<CalendarCellProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_unavailable: PropHandle<bool>,
    pub is_outside_month: PropHandle<bool>,
    pub is_today: PropHandle<bool>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatePickerHook {
    pub date_picker_props: PropHandle<DatePickerProps>,
    pub date_picker_input_props: PropHandle<DatePickerInputProps>,
    pub date_picker_trigger_props: PropHandle<DatePickerTriggerProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub is_open: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateRangePickerHook {
    pub date_range_picker_props: PropHandle<DateRangePickerProps>,
    pub date_range_picker_start_input_props: PropHandle<DateRangePickerInputProps>,
    pub date_range_picker_end_input_props: PropHandle<DateRangePickerInputProps>,
    pub date_range_picker_trigger_props: PropHandle<DatePickerTriggerProps>,
    pub label: PropHandle<Option<String>>,
    pub start_value: PropHandle<Option<String>>,
    pub end_value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub is_open: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorFieldHook {
    pub color_field_props: PropHandle<ColorFieldProps>,
    pub color_field_input_props: PropHandle<ColorFieldInputProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub color_space: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorPickerHook {
    pub color_picker_props: PropHandle<ColorPickerProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorAreaHook {
    pub color_area_props: PropHandle<ColorAreaProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub x_channel: PropHandle<Option<String>>,
    pub y_channel: PropHandle<Option<String>>,
    pub x_value: PropHandle<f64>,
    pub y_value: PropHandle<f64>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorSliderHook {
    pub color_slider_props: PropHandle<ColorRangeProps>,
    pub label: PropHandle<Option<String>>,
    pub channel: PropHandle<Option<String>>,
    pub value_number: PropHandle<f64>,
    pub min_value: PropHandle<f64>,
    pub max_value: PropHandle<f64>,
    pub step_value: PropHandle<f64>,
    pub value_percent: PropHandle<f64>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorWheelHook {
    pub color_wheel_props: PropHandle<ColorRangeProps>,
    pub label: PropHandle<Option<String>>,
    pub channel: PropHandle<Option<String>>,
    pub value_number: PropHandle<f64>,
    pub min_value: PropHandle<f64>,
    pub max_value: PropHandle<f64>,
    pub step_value: PropHandle<f64>,
    pub value_percent: PropHandle<f64>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorSwatchPickerHook {
    pub color_swatch_picker_props: PropHandle<ColorSwatchPickerProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorSwatchPickerItemHook {
    pub color_swatch_picker_item_props: PropHandle<ColorSwatchPickerItemProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColorSwatchHook {
    pub color_swatch_props: PropHandle<ColorSwatchProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorThumbHook {
    pub color_thumb_props: PropHandle<ColorThumbProps>,
    pub value: PropHandle<Option<String>>,
    pub x_value: PropHandle<f64>,
    pub y_value: PropHandle<f64>,
    pub is_pressed: PropHandle<bool>,
    pub is_dragging: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboBoxHook {
    pub combo_box_props: PropHandle<ComboBoxProps>,
    pub combo_box_input_props: PropHandle<ComboBoxInputProps>,
    pub combo_box_trigger_props: PropHandle<SelectionInputTriggerProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub input_value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub is_open: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutocompleteHook {
    pub autocomplete_props: PropHandle<AutocompleteProps>,
    pub autocomplete_input_props: PropHandle<ComboBoxInputProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub input_value: PropHandle<Option<String>>,
    pub placeholder: PropHandle<Option<String>>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectHook {
    pub select_props: PropHandle<SelectProps>,
    pub select_trigger_props: PropHandle<SelectionInputTriggerProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub placeholder: PropHandle<Option<String>>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub is_open: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectDisplayHook {
    pub select_value_props: PropHandle<SelectValueProps>,
    pub value: PropHandle<Option<String>>,
    pub display_value: PropHandle<Option<String>>,
    pub is_placeholder: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComboBoxDisplayHook {
    pub combo_box_value_props: PropHandle<SelectValueProps>,
    pub value: PropHandle<Option<String>>,
    pub display_value: PropHandle<Option<String>>,
    pub is_placeholder: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OverlayHook {
    pub overlay_props: PropHandle<OverlayProps>,
    pub overlay_trigger_props: PropHandle<OverlayTriggerProps>,
    pub is_open: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverlayPositionHook {
    pub overlay_position_props: PropHandle<OverlayPositionProps>,
    pub arrow_props: PropHandle<OverlayArrowProps>,
    pub placement: PropHandle<crate::overlay_position::OverlayPlacement>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionHook {
    pub selection_props: PropHandle<SelectionProps>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub escape_key_behavior: PropHandle<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeHook {
    pub tree_props: PropHandle<TreeProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub selected_keys: PropHandle<Selection>,
    pub expanded_keys: PropHandle<BTreeSet<CollectionKey>>,
    pub selection_mode: PropHandle<String>,
    pub selection_behavior: PropHandle<String>,
    pub disabled_behavior: PropHandle<String>,
    pub escape_key_behavior: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeItemHook {
    pub tree_item_props: PropHandle<TreeItemProps>,
    pub value: PropHandle<Option<String>>,
    pub text_value: PropHandle<Option<String>>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_expanded: PropHandle<Option<bool>>,
    pub has_child_items: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisclosureHook {
    pub disclosure_props: PropHandle<DisclosureProps>,
    pub disclosure_trigger_props: PropHandle<DisclosureTriggerProps>,
    pub disclosure_panel_props: PropHandle<DisclosurePanelProps>,
    pub is_expanded: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DisclosureGroupHook {
    pub disclosure_group_props: PropHandle<DisclosureGroupProps>,
    pub label: PropHandle<Option<String>>,
    pub expanded_keys: PropHandle<Option<String>>,
    pub allows_multiple_expanded: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangeHook {
    pub range_props: PropHandle<RangeProps>,
    pub range_input_props: PropHandle<RangeInputProps>,
    pub value_number: PropHandle<f64>,
    pub min_value: PropHandle<f64>,
    pub max_value: PropHandle<f64>,
    pub step_value: PropHandle<f64>,
    pub value_percent: PropHandle<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastHook {
    pub toast_props: PropHandle<ToastProps>,
    pub title: PropHandle<Option<String>>,
    pub description: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ToastRegionHook {
    pub toast_region_props: PropHandle<ToastRegionProps>,
    pub label: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NumberFieldHook {
    pub number_field_props: PropHandle<NumberFieldProps>,
    pub number_field_input_props: PropHandle<NumberFieldInputProps>,
    pub increment_button_props: PropHandle<NumberFieldButtonProps>,
    pub decrement_button_props: PropHandle<NumberFieldButtonProps>,
    pub label: PropHandle<Option<String>>,
    pub value_number: PropHandle<f64>,
    pub placeholder: PropHandle<Option<String>>,
    pub min_value: PropHandle<f64>,
    pub max_value: PropHandle<f64>,
    pub step_value: PropHandle<f64>,
    pub format_options: PropHandle<NumberFormatOptions>,
    pub format_style: PropHandle<NumberFormatStyle>,
    pub value_percent: PropHandle<f64>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
    pub is_wheel_disabled: PropHandle<bool>,
    pub can_increment: PropHandle<bool>,
    pub can_decrement: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderTrackHook {
    pub slider_track_props: PropHandle<SliderTrackProps>,
    pub orientation: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliderFillHook {
    pub slider_fill_props: PropHandle<SliderFillProps>,
    pub orientation: PropHandle<String>,
    pub value_number: PropHandle<f64>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SliderOutputHook {
    pub slider_output_props: PropHandle<SliderOutputProps>,
    pub label: PropHandle<Option<String>>,
    pub value: PropHandle<Option<String>>,
    pub value_number: PropHandle<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleHook {
    pub toggle_props: PropHandle<ToggleProps>,
    pub is_selected: PropHandle<bool>,
    pub is_checked: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchHook {
    pub switch_props: PropHandle<SwitchProps>,
    pub is_checked: PropHandle<bool>,
    pub is_selected: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
    pub is_required: PropHandle<bool>,
    pub is_invalid: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleButtonHook {
    pub toggle_button_props: PropHandle<ToggleButtonProps>,
    pub is_selected: PropHandle<bool>,
    pub is_pressed: PropHandle<bool>,
    pub is_disabled: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToggleButtonGroupHook {
    pub toggle_button_group_props: PropHandle<ToggleButtonGroupProps>,
    pub label: PropHandle<Option<String>>,
    pub selected_value: PropHandle<Option<String>>,
    pub orientation: PropHandle<String>,
    pub selection_mode: PropHandle<String>,
    pub is_disabled: PropHandle<bool>,
    pub is_read_only: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextFieldHook {
    pub input_props: PropHandle<TextInputProps>,
    pub field_props: PropHandle<TextFieldProps>,
    pub value: PropHandle<Option<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BindingAlias {
    State(String),
    Props(String),
    Derived(String),
    Context(String),
    Resource(String),
    Action(String),
}

impl BindingAlias {
    fn resolve(&self, suffix: &[String]) -> Option<String> {
        if let Self::Action(id) = self {
            return suffix.is_empty().then(|| format!("{id:?}"));
        }

        let (root, path) = match self {
            Self::State(path) => ("state", path),
            Self::Props(path) => ("props", path),
            Self::Derived(path) => ("derived", path),
            Self::Context(path) => ("context", path),
            Self::Resource(path) => ("resource", path),
            Self::Action(_) => unreachable!("action aliases are handled above"),
        };
        if suffix.is_empty() {
            Some(format!("{root}.{path}"))
        } else {
            Some(format!("{root}.{path}.{}", suffix.join(".")))
        }
    }
}

pub struct ComponentCx<S> {
    frame_id: String,
    registrations: Vec<ComponentRegistration<S>>,
    aliases: BTreeMap<String, BindingAlias>,
    id_counter: usize,
}

impl<S: 'static> ComponentCx<S> {
    pub fn new(frame_id: impl Into<String>) -> Self {
        Self {
            frame_id: frame_id.into(),
            registrations: Vec::new(),
            aliases: BTreeMap::new(),
            id_counter: 0,
        }
    }

    pub fn compile<F>(frame_id: impl Into<String>, render: F) -> GuiResult<RsxComponent<S>>
    where
        F: FnOnce(&mut Self) -> RSX,
    {
        let mut cx = Self::new(frame_id);
        let view = render(&mut cx);
        cx.into_component(view)
    }

    /// Compiles a component without installing the default design-system
    /// component registry.
    pub fn compile_bare<F>(frame_id: impl Into<String>, render: F) -> GuiResult<RsxComponent<S>>
    where
        F: FnOnce(&mut Self) -> RSX,
    {
        let mut cx = Self::new(frame_id);
        let view = render(&mut cx);
        cx.into_component_bare(view)
    }

    /// Compiles a component against an explicitly supplied component registry.
    pub fn compile_with_registry<F>(
        frame_id: impl Into<String>,
        registry: ComponentRegistry,
        render: F,
    ) -> GuiResult<RsxComponent<S>>
    where
        F: FnOnce(&mut Self) -> RSX,
    {
        let mut cx = Self::new(frame_id);
        let view = render(&mut cx);
        cx.into_component_with_registry(view, registry)
    }

    pub fn use_ref<T>(&mut self, initial: T) -> RefHandle<T>
    where
        T: Send + 'static,
    {
        RefHandle::new(initial)
    }

    pub fn use_imperative_handle<T, F>(&mut self, target: RefHandle<Option<T>>, create: F)
    where
        T: Send + 'static,
        F: Fn(&mut S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.use_layout_effect_with_cleanup(move |state| {
            target.set(Some(create(state)?))?;
            let cleanup_target = target.clone();
            Ok(move |_state: &mut S| cleanup_target.set(None))
        });
    }

    pub fn use_id(&mut self) -> String {
        let id = stable_hook_id(&self.frame_id, self.id_counter);
        self.id_counter += 1;
        id
    }

    pub fn use_effect_event<F>(&mut self, event: F) -> EffectEventHandle<S>
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        EffectEventHandle::new(event)
    }

    pub fn use_state<T, F>(&mut self, path: impl Into<String>, selector: F) -> StateHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::State(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_state::<T, F>(hook_path, selector))
        }));
        StateHandle::new(path)
    }

    pub fn use_state_result<T, F>(&mut self, path: impl Into<String>, selector: F) -> StateHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::State(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_state_result::<T, F>(hook_path, selector))
        }));
        StateHandle::new(path)
    }

    pub fn use_selector<T, F>(&mut self, path: impl Into<String>, selector: F) -> SelectorHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_state(path, selector)
    }

    pub fn use_selector_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> SelectorHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.use_state_result(path, selector)
    }

    pub fn use_prop<T, F>(&mut self, path: impl Into<String>, selector: F) -> PropHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Props(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_prop::<T, F>(hook_path, selector))
        }));
        PropHandle::new(path)
    }

    pub fn use_prop_result<T, F>(&mut self, path: impl Into<String>, selector: F) -> PropHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Props(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_prop_result::<T, F>(hook_path, selector))
        }));
        PropHandle::new(path)
    }

    pub fn use_derived<T, F>(&mut self, path: impl Into<String>, selector: F) -> DerivedHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Derived(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_derived::<T, F>(hook_path, selector))
        }));
        DerivedHandle::new(path)
    }

    pub fn use_derived_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> DerivedHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Derived(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_derived_result::<T, F>(hook_path, selector))
        }));
        DerivedHandle::new(path)
    }

    pub fn use_memo<T, F>(&mut self, path: impl Into<String>, selector: F) -> DerivedHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_derived(path, selector)
    }

    pub fn use_sync_external_store<T>(
        &mut self,
        path: impl Into<String>,
        store: SyncExternalStore<T>,
    ) -> DerivedHandle<T>
    where
        T: Clone + Send + Serialize + 'static,
    {
        self.use_derived_result(path, move |_state| store.snapshot())
    }

    pub fn use_optimistic<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> OptimisticHandle<T>
    where
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_optimistic_result(path, move |state| Ok(selector(state)))
    }

    pub fn use_optimistic_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> OptimisticHandle<T>
    where
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let handle = OptimisticHandle::new();
        let render_handle = handle.clone();
        self.use_derived_result(path, move |state| {
            render_handle.set_base(selector(state)?)?;
            render_handle.current()
        });
        handle
    }

    pub fn use_deferred_value<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> DerivedHandle<T>
    where
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_deferred_value_result(path, move |state| Ok(selector(state)))
    }

    pub fn use_deferred_value_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> DerivedHandle<T>
    where
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        let selector: Arc<dyn Fn(&S) -> GuiResult<T> + Send + Sync> = Arc::new(selector);
        let deferred_value = Arc::new(Mutex::new(None::<T>));
        let render_selector = Arc::clone(&selector);
        let render_deferred_value = Arc::clone(&deferred_value);
        let effect_selector = Arc::clone(&selector);
        let effect_deferred_value = Arc::clone(&deferred_value);
        let handle = self.use_derived_result(path, move |state| {
            let current = render_selector(state)?;
            let deferred = render_deferred_value.lock().map_err(|_| {
                GuiError::invalid_tree("RSX deferred value lock was poisoned while reading")
            })?;
            Ok(deferred.clone().unwrap_or(current))
        });
        self.use_effect(move |state| {
            let current = effect_selector(state)?;
            let mut deferred = effect_deferred_value.lock().map_err(|_| {
                GuiError::invalid_tree("RSX deferred value lock was poisoned while writing")
            })?;
            *deferred = Some(current);
            Ok(())
        });
        handle
    }

    pub fn use_debug_value<T, F>(&mut self, label: impl Into<String>, selector: F)
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let label = label.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_debug_value::<T, F>(label, selector))
        }));
    }

    pub fn use_debug_value_result<T, F>(&mut self, label: impl Into<String>, selector: F)
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let label = label.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_debug_value_result::<T, F>(label, selector))
        }));
    }

    pub fn use_context<T, F>(&mut self, path: impl Into<String>, selector: F) -> ContextHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Context(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_context::<T, F>(hook_path, selector))
        }));
        ContextHandle::new(path)
    }

    pub fn use_context_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> ContextHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Context(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_context_result::<T, F>(hook_path, selector))
        }));
        ContextHandle::new(path)
    }

    pub fn use_resource<T, F>(&mut self, path: impl Into<String>, selector: F) -> ResourceHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> RsxResource<T> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Resource(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_resource::<T, F>(hook_path, selector))
        }));
        ResourceHandle::new(path)
    }

    pub fn use_resource_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> ResourceHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<RsxResource<T>> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Resource(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_resource_result::<T, F>(hook_path, selector))
        }));
        ResourceHandle::new(path)
    }

    pub fn use_action_state<D, F>(
        &mut self,
        path: impl Into<String>,
        action: impl Into<String>,
        reducer: F,
    ) -> ActionStateHandle<D, String>
    where
        D: Clone + Send + Serialize + 'static,
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<D> + Send + Sync + 'static,
    {
        let action = action.into();
        let handle = ActionStateHandle::new(action.clone());
        let selector_handle = handle.clone();
        self.use_derived_result(path, move |_state| selector_handle.snapshot());
        let reducer_handle = handle.clone();
        self.use_reducer(action, move |state, invocation| {
            reducer_handle.set_pending(true)?;
            match reducer(state, invocation) {
                Ok(data) => reducer_handle.set_data(data),
                Err(error) => reducer_handle.set_error(error.to_string()),
            }
        });
        handle
    }

    pub fn use_form_status<D, E>(
        &mut self,
        path: impl Into<String>,
        action_state: ActionStateHandle<D, E>,
    ) -> DerivedHandle<FormStatusSnapshot<D, E>>
    where
        D: Clone + Send + Serialize + 'static,
        E: Clone + Send + Serialize + 'static,
    {
        self.use_derived_result(path, move |_state| action_state.form_status())
    }

    pub fn use_reactive<T, F>(&mut self, path: impl Into<String>, selector: F) -> ReactiveHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let handle = self.use_state(path, selector);
        ReactiveHandle::new(handle.path().to_string())
    }

    pub fn use_reactive_result<T, F>(
        &mut self,
        path: impl Into<String>,
        selector: F,
    ) -> ReactiveHandle<T>
    where
        T: Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let handle = self.use_state_result(path, selector);
        ReactiveHandle::new(handle.path().to_string())
    }

    pub fn use_reducer<F>(&mut self, id: impl Into<String>, reducer: F) -> ActionHandle
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
        self.register_alias(&id, BindingAlias::Action(id.clone()));
        let action_id = id.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_reducer(action_id, reducer))
        }));
        ActionHandle::new(id)
    }

    pub fn use_value_reducer<T, F>(&mut self, id: impl Into<String>, reducer: F) -> ActionHandle
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
        self.register_alias(&id, BindingAlias::Action(id.clone()));
        let action_id = id.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_value_reducer::<T, F>(action_id, reducer))
        }));
        ActionHandle::new(id)
    }

    pub fn use_payload_reducer<T, F>(&mut self, id: impl Into<String>, reducer: F) -> ActionHandle
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
        self.register_alias(&id, BindingAlias::Action(id.clone()));
        let action_id = id.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_payload_reducer::<T, F>(action_id, reducer))
        }));
        ActionHandle::new(id)
    }

    pub fn use_callback<F>(&mut self, id: impl Into<String>, callback: F) -> ActionHandle
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_reducer(id, callback)
    }

    pub fn use_action_disabled<F>(&mut self, id: impl Into<String>, selector: F)
    where
        F: Fn(&S) -> bool + Send + Sync + 'static,
    {
        let id = id.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_action_disabled(id, selector))
        }));
    }

    pub fn use_action_disabled_result<F>(&mut self, id: impl Into<String>, selector: F)
    where
        F: Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    {
        let id = id.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_action_disabled_result(id, selector))
        }));
    }

    pub fn use_action_enabled<F>(&mut self, id: impl Into<String>, selector: F)
    where
        F: Fn(&S) -> bool + Send + Sync + 'static,
    {
        let id = id.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_action_enabled(id, selector))
        }));
    }

    pub fn use_action_enabled_result<F>(&mut self, id: impl Into<String>, selector: F)
    where
        F: Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    {
        let id = id.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_action_enabled_result(id, selector))
        }));
    }

    pub fn use_mount<F>(&mut self, hook: F)
    where
        F: Fn(&mut S) + Send + Sync + 'static,
    {
        self.registrations
            .push(Box::new(move |component| Ok(component.use_mount(hook))));
    }

    pub fn use_mount_result<F>(&mut self, hook: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_mount_result(hook))
        }));
    }

    pub fn use_unmount<F>(&mut self, hook: F)
    where
        F: Fn(&mut S) + Send + Sync + 'static,
    {
        self.registrations
            .push(Box::new(move |component| Ok(component.use_unmount(hook))));
    }

    pub fn use_unmount_result<F>(&mut self, hook: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_unmount_result(hook))
        }));
    }

    pub fn use_effect<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations
            .push(Box::new(move |component| Ok(component.use_effect(effect))));
    }

    pub fn use_effect_once<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_effect_once(effect))
        }));
    }

    pub fn use_effect_with_deps<T, D, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_effect_with_deps::<T, D, F>(deps, effect))
        }));
    }

    pub fn use_effect_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_effect_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_effect_once_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_effect_once_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_effect_with_deps_and_cleanup<T, D, C, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_effect_with_deps_and_cleanup::<T, D, C, F>(deps, effect))
        }));
    }

    pub fn use_layout_effect<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect(effect))
        }));
    }

    pub fn use_layout_effect_once<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect_once(effect))
        }));
    }

    pub fn use_layout_effect_with_deps<T, D, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect_with_deps::<T, D, F>(deps, effect))
        }));
    }

    pub fn use_layout_effect_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_layout_effect_once_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect_once_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_layout_effect_with_deps_and_cleanup<T, D, C, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_layout_effect_with_deps_and_cleanup::<T, D, C, F>(deps, effect))
        }));
    }

    pub fn use_insertion_effect<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect(effect))
        }));
    }

    pub fn use_insertion_effect_once<F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect_once(effect))
        }));
    }

    pub fn use_insertion_effect_with_deps<T, D, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect_with_deps::<T, D, F>(deps, effect))
        }));
    }

    pub fn use_insertion_effect_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_insertion_effect_once_with_cleanup<C, F>(&mut self, effect: F)
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect_once_with_cleanup::<C, F>(effect))
        }));
    }

    pub fn use_insertion_effect_with_deps_and_cleanup<T, D, C, F>(&mut self, deps: D, effect: F)
    where
        T: Serialize + 'static,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_insertion_effect_with_deps_and_cleanup::<T, D, C, F>(deps, effect))
        }));
    }

    pub fn use_action_effect<F>(&mut self, action: impl Into<String>, effect: F)
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action = action.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_action_effect(action, effect))
        }));
    }

    pub fn use_value_effect<T, F>(&mut self, action: impl Into<String>, effect: F)
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action = action.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_value_effect::<T, F>(action, effect))
        }));
    }

    pub fn use_payload_effect<T, F>(&mut self, action: impl Into<String>, effect: F)
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action = action.into();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_payload_effect::<T, F>(action, effect))
        }));
    }

    pub fn use_press<F>(&mut self, selector: F) -> PressHook
    where
        F: Fn(&S) -> UsePressProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UsePressProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("pressProps", move |state| {
            press_value_part((props_selector)(state), "pressProps")
        });
        let pressed_selector = Arc::clone(&selector);
        self.use_prop_value("isPressed", move |state| {
            press_value_part((pressed_selector)(state), "isPressed")
        });

        self.register_alias("pressProps", BindingAlias::Props("pressProps".to_string()));
        self.register_alias("isPressed", BindingAlias::Props("isPressed".to_string()));

        PressHook {
            press_props: PropHandle::new("pressProps"),
            is_pressed: PropHandle::new("isPressed"),
        }
    }

    pub fn use_link<F>(&mut self, selector: F) -> LinkHook
    where
        F: Fn(&S) -> UseLinkProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseLinkProps + Send + Sync> = Arc::new(selector);
        for path in ["linkProps", "href", "isDisabled", "isPressed"] {
            self.use_semantic_part(path, &selector, link_value_part);
        }
        self.register_prop_aliases(&["linkProps", "href", "isDisabled", "isPressed"]);

        LinkHook {
            link_props: PropHandle::new("linkProps"),
            href: PropHandle::new("href"),
            is_disabled: PropHandle::new("isDisabled"),
            is_pressed: PropHandle::new("isPressed"),
        }
    }

    pub fn use_hover<F>(&mut self, selector: F) -> HoverHook
    where
        F: Fn(&S) -> UseHoverProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseHoverProps + Send + Sync> = Arc::new(selector);
        for path in ["hoverProps", "isHovered"] {
            self.use_semantic_part(path, &selector, hover_value_part);
        }
        self.register_prop_aliases(&["hoverProps", "isHovered"]);

        HoverHook {
            hover_props: PropHandle::new("hoverProps"),
            is_hovered: PropHandle::new("isHovered"),
        }
    }

    pub fn use_keyboard_interaction<F>(&mut self, selector: F) -> KeyboardInteractionHook
    where
        F: Fn(&S) -> UseKeyboardInteractionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseKeyboardInteractionProps + Send + Sync> =
            Arc::new(selector);
        for path in ["keyboardInteractionProps", "isKeyboardActive"] {
            self.use_semantic_part(path, &selector, keyboard_interaction_value_part);
        }
        self.register_prop_aliases(&["keyboardInteractionProps", "isKeyboardActive"]);

        KeyboardInteractionHook {
            keyboard_interaction_props: PropHandle::new("keyboardInteractionProps"),
            is_keyboard_active: PropHandle::new("isKeyboardActive"),
        }
    }

    pub fn use_clipboard<F>(&mut self, selector: F) -> ClipboardHook
    where
        F: Fn(&S) -> UseClipboardProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseClipboardProps + Send + Sync> = Arc::new(selector);
        for path in ["clipboardProps", "isClipboardDisabled"] {
            self.use_semantic_part(path, &selector, clipboard_value_part);
        }
        self.register_prop_aliases(&["clipboardProps", "isClipboardDisabled"]);

        ClipboardHook {
            clipboard_props: PropHandle::new("clipboardProps"),
            is_clipboard_disabled: PropHandle::new("isClipboardDisabled"),
        }
    }

    pub fn use_long_press<F>(&mut self, selector: F) -> LongPressHook
    where
        F: Fn(&S) -> UseLongPressProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseLongPressProps + Send + Sync> = Arc::new(selector);
        for path in ["longPressProps", "isPressed", "isLongPressed"] {
            self.use_semantic_part(path, &selector, long_press_value_part);
        }
        self.register_prop_aliases(&["longPressProps", "isPressed", "isLongPressed"]);

        LongPressHook {
            long_press_props: PropHandle::new("longPressProps"),
            is_pressed: PropHandle::new("isPressed"),
            is_long_pressed: PropHandle::new("isLongPressed"),
        }
    }

    pub fn use_move<F>(&mut self, selector: F) -> MoveHook
    where
        F: Fn(&S) -> UseMoveProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseMoveProps + Send + Sync> = Arc::new(selector);
        for path in ["moveProps", "isMoving", "xDelta", "yDelta"] {
            self.use_semantic_part(path, &selector, move_value_part);
        }
        self.register_prop_aliases(&["moveProps", "isMoving", "xDelta", "yDelta"]);

        MoveHook {
            move_props: PropHandle::new("moveProps"),
            is_moving: PropHandle::new("isMoving"),
            x_delta: PropHandle::new("xDelta"),
            y_delta: PropHandle::new("yDelta"),
        }
    }

    pub fn use_button<F>(&mut self, selector: F) -> ButtonHook
    where
        F: Fn(&S) -> UseButtonProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseButtonProps + Send + Sync> = Arc::new(selector);
        for path in ["buttonProps", "pressProps", "isPressed"] {
            self.use_semantic_part(path, &selector, button_value_part);
        }
        self.register_prop_aliases(&["buttonProps", "pressProps", "isPressed"]);

        ButtonHook {
            button_props: PropHandle::new("buttonProps"),
            press_props: PropHandle::new("pressProps"),
            is_pressed: PropHandle::new("isPressed"),
        }
    }

    pub fn use_file_trigger<F>(&mut self, selector: F) -> FileTriggerHook
    where
        F: Fn(&S) -> UseFileTriggerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFileTriggerProps + Send + Sync> = Arc::new(selector);
        for path in [
            "fileTriggerProps",
            "acceptedFileTypes",
            "allowsMultiple",
            "isDisabled",
            "isPressed",
        ] {
            self.use_semantic_part(path, &selector, file_trigger_value_part);
        }
        self.register_prop_aliases(&[
            "fileTriggerProps",
            "acceptedFileTypes",
            "allowsMultiple",
            "isDisabled",
            "isPressed",
        ]);

        FileTriggerHook {
            file_trigger_props: PropHandle::new("fileTriggerProps"),
            accepted_file_types: PropHandle::new("acceptedFileTypes"),
            allows_multiple: PropHandle::new("allowsMultiple"),
            is_disabled: PropHandle::new("isDisabled"),
            is_pressed: PropHandle::new("isPressed"),
        }
    }

    pub fn use_drop_zone<F>(&mut self, selector: F) -> DropZoneHook
    where
        F: Fn(&S) -> UseDropZoneProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDropZoneProps + Send + Sync> = Arc::new(selector);
        for path in ["dropZoneProps", "label", "isDisabled"] {
            self.use_semantic_part(path, &selector, drop_zone_value_part);
        }
        self.register_prop_aliases(&["dropZoneProps", "label", "isDisabled"]);

        DropZoneHook {
            drop_zone_props: PropHandle::new("dropZoneProps"),
            label: PropHandle::new("label"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_drag<F>(&mut self, selector: F) -> DragHook
    where
        F: Fn(&S) -> UseDragProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDragProps + Send + Sync> = Arc::new(selector);
        for path in ["dragProps", "dragButtonProps", "isDragging"] {
            self.use_semantic_part(path, &selector, drag_value_part);
        }
        self.register_prop_aliases(&["dragProps", "dragButtonProps", "isDragging"]);

        DragHook {
            drag_props: PropHandle::new("dragProps"),
            drag_button_props: PropHandle::new("dragButtonProps"),
            is_dragging: PropHandle::new("isDragging"),
        }
    }

    pub fn use_drop<F>(&mut self, selector: F) -> DropHook
    where
        F: Fn(&S) -> UseDropProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDropProps + Send + Sync> = Arc::new(selector);
        for path in [
            "dropProps",
            "dropButtonProps",
            "label",
            "isDisabled",
            "isDropTarget",
        ] {
            self.use_semantic_part(path, &selector, drop_value_part);
        }
        self.register_prop_aliases(&[
            "dropProps",
            "dropButtonProps",
            "label",
            "isDisabled",
            "isDropTarget",
        ]);

        DropHook {
            drop_props: PropHandle::new("dropProps"),
            drop_button_props: PropHandle::new("dropButtonProps"),
            label: PropHandle::new("label"),
            is_disabled: PropHandle::new("isDisabled"),
            is_drop_target: PropHandle::new("isDropTarget"),
        }
    }

    pub fn use_group<F>(&mut self, selector: F) -> GroupHook
    where
        F: Fn(&S) -> UseGroupProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseGroupProps + Send + Sync> = Arc::new(selector);
        for path in [
            "groupProps",
            "label",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
            "isHovered",
            "isFocused",
            "isFocusVisible",
            "isFocusWithin",
        ] {
            self.use_semantic_part(path, &selector, group_value_part);
        }
        self.register_prop_aliases(&[
            "groupProps",
            "label",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
            "isHovered",
            "isFocused",
            "isFocusVisible",
            "isFocusWithin",
        ]);

        GroupHook {
            group_props: PropHandle::new("groupProps"),
            label: PropHandle::new("label"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
            is_hovered: PropHandle::new("isHovered"),
            is_focused: PropHandle::new("isFocused"),
            is_focus_visible: PropHandle::new("isFocusVisible"),
            is_focus_within: PropHandle::new("isFocusWithin"),
        }
    }

    pub fn use_virtualizer<F>(&mut self, selector: F) -> VirtualizerHook
    where
        F: Fn(&S) -> UseVirtualizerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseVirtualizerProps + Send + Sync> = Arc::new(selector);
        for path in [
            "virtualizerProps",
            "label",
            "layout",
            "orientation",
            "itemCount",
            "estimatedItemSize",
            "visibleStart",
            "visibleEnd",
            "overscan",
            "gap",
            "padding",
            "isScrolling",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, virtualizer_value_part);
        }
        self.register_prop_aliases(&[
            "virtualizerProps",
            "label",
            "layout",
            "orientation",
            "itemCount",
            "estimatedItemSize",
            "visibleStart",
            "visibleEnd",
            "overscan",
            "gap",
            "padding",
            "isScrolling",
            "isDisabled",
        ]);

        VirtualizerHook {
            virtualizer_props: PropHandle::new("virtualizerProps"),
            label: PropHandle::new("label"),
            layout: PropHandle::new("layout"),
            orientation: PropHandle::new("orientation"),
            item_count: PropHandle::new("itemCount"),
            estimated_item_size: PropHandle::new("estimatedItemSize"),
            visible_start: PropHandle::new("visibleStart"),
            visible_end: PropHandle::new("visibleEnd"),
            overscan: PropHandle::new("overscan"),
            gap: PropHandle::new("gap"),
            padding: PropHandle::new("padding"),
            is_scrolling: PropHandle::new("isScrolling"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    /// React Aria-compatible alias for [`Self::use_focusable`].
    pub fn use_focus<F>(&mut self, selector: F) -> FocusableHook
    where
        F: Fn(&S) -> UseFocusableProps + Send + Sync + 'static,
    {
        self.use_focusable(selector)
    }

    pub fn use_focusable<F>(&mut self, selector: F) -> FocusableHook
    where
        F: Fn(&S) -> UseFocusableProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFocusableProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("focusProps", move |state| {
            focusable_value_part((props_selector)(state), "focusProps")
        });
        let focused_selector = Arc::clone(&selector);
        self.use_prop_value("isFocused", move |state| {
            focusable_value_part((focused_selector)(state), "isFocused")
        });

        self.register_alias("focusProps", BindingAlias::Props("focusProps".to_string()));
        self.register_alias("isFocused", BindingAlias::Props("isFocused".to_string()));

        FocusableHook {
            focus_props: PropHandle::new("focusProps"),
            is_focused: PropHandle::new("isFocused"),
        }
    }

    pub fn use_focus_within<F>(&mut self, selector: F) -> FocusWithinHook
    where
        F: Fn(&S) -> UseFocusWithinProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFocusWithinProps + Send + Sync> = Arc::new(selector);
        for path in ["focusWithinProps", "isFocusWithin"] {
            self.use_semantic_part(path, &selector, focus_within_value_part);
        }
        self.register_prop_aliases(&["focusWithinProps", "isFocusWithin"]);

        FocusWithinHook {
            focus_within_props: PropHandle::new("focusWithinProps"),
            is_focus_within: PropHandle::new("isFocusWithin"),
        }
    }

    pub fn use_form<F>(&mut self, selector: F) -> FormHook
    where
        F: Fn(&S) -> UseFormProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFormProps + Send + Sync> = Arc::new(selector);
        for path in [
            "formProps",
            "label",
            "validationBehavior",
            "isDisabled",
            "isInvalid",
            "noValidate",
        ] {
            self.use_semantic_part(path, &selector, form_value_part);
        }
        self.register_prop_aliases(&[
            "formProps",
            "label",
            "validationBehavior",
            "isDisabled",
            "isInvalid",
            "noValidate",
        ]);

        FormHook {
            form_props: PropHandle::new("formProps"),
            label: PropHandle::new("label"),
            validation_behavior: PropHandle::new("validationBehavior"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
            no_validate: PropHandle::new("noValidate"),
        }
    }

    pub fn use_breadcrumbs<F>(&mut self, selector: F) -> BreadcrumbsHook
    where
        F: Fn(&S) -> UseBreadcrumbsProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseBreadcrumbsProps + Send + Sync> = Arc::new(selector);
        for path in ["breadcrumbsProps", "label"] {
            self.use_semantic_part(path, &selector, breadcrumbs_value_part);
        }
        self.register_prop_aliases(&["breadcrumbsProps", "label"]);

        BreadcrumbsHook {
            breadcrumbs_props: PropHandle::new("breadcrumbsProps"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_landmark<F>(&mut self, selector: F) -> LandmarkHook
    where
        F: Fn(&S) -> UseLandmarkProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseLandmarkProps + Send + Sync> = Arc::new(selector);
        for path in ["landmarkProps", "landmarkKind", "label"] {
            self.use_semantic_part(path, &selector, landmark_value_part);
        }
        self.register_prop_aliases(&["landmarkProps", "landmarkKind", "label"]);

        LandmarkHook {
            landmark_props: PropHandle::new("landmarkProps"),
            landmark_kind: PropHandle::new("landmarkKind"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_focus_ring<F>(&mut self, selector: F) -> FocusRingHook
    where
        F: Fn(&S) -> UseFocusRingProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFocusRingProps + Send + Sync> = Arc::new(selector);
        for path in [
            "focusRingProps",
            "isFocused",
            "isFocusVisible",
            "isFocusWithin",
        ] {
            self.use_semantic_part(path, &selector, focus_ring_value_part);
        }
        self.register_prop_aliases(&[
            "focusRingProps",
            "isFocused",
            "isFocusVisible",
            "isFocusWithin",
        ]);

        FocusRingHook {
            focus_ring_props: PropHandle::new("focusRingProps"),
            is_focused: PropHandle::new("isFocused"),
            is_focus_visible: PropHandle::new("isFocusVisible"),
            is_focus_within: PropHandle::new("isFocusWithin"),
        }
    }

    pub fn use_focus_scope<F>(&mut self, selector: F) -> FocusScopeHook
    where
        F: Fn(&S) -> UseFocusScopeProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFocusScopeProps + Send + Sync> = Arc::new(selector);
        for path in [
            "focusScopeProps",
            "contain",
            "restoreFocus",
            "autoFocus",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, focus_scope_value_part);
        }
        self.register_prop_aliases(&[
            "focusScopeProps",
            "contain",
            "restoreFocus",
            "autoFocus",
            "isDisabled",
        ]);

        FocusScopeHook {
            focus_scope_props: PropHandle::new("focusScopeProps"),
            contain: PropHandle::new("contain"),
            restore_focus: PropHandle::new("restoreFocus"),
            auto_focus: PropHandle::new("autoFocus"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_field<F>(&mut self, selector: F) -> FieldHook
    where
        F: Fn(&S) -> UseFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseFieldProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("fieldProps", move |state| {
            field_value_part((props_selector)(state), "fieldProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            field_value_part((label_selector)(state), "label")
        });

        self.register_alias("fieldProps", BindingAlias::Props("fieldProps".to_string()));
        self.register_alias("label", BindingAlias::Props("label".to_string()));

        FieldHook {
            field_props: PropHandle::new("fieldProps"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_checkbox<F>(&mut self, selector: F) -> CheckboxHook
    where
        F: Fn(&S) -> UseCheckboxProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCheckboxProps + Send + Sync> = Arc::new(selector);
        for path in [
            "checkboxProps",
            "value",
            "isChecked",
            "isSelected",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, checkbox_value_part);
        }
        self.register_prop_aliases(&[
            "checkboxProps",
            "value",
            "isChecked",
            "isSelected",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        CheckboxHook {
            checkbox_props: PropHandle::new("checkboxProps"),
            value: PropHandle::new("value"),
            is_checked: PropHandle::new("isChecked"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_checkbox_group<F>(&mut self, selector: F) -> CheckboxGroupHook
    where
        F: Fn(&S) -> UseCheckboxGroupProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCheckboxGroupProps + Send + Sync> = Arc::new(selector);
        for path in [
            "checkboxGroupProps",
            "label",
            "selectedValue",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, checkbox_group_value_part);
        }
        self.register_prop_aliases(&[
            "checkboxGroupProps",
            "label",
            "selectedValue",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        CheckboxGroupHook {
            checkbox_group_props: PropHandle::new("checkboxGroupProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_separator<F>(&mut self, selector: F) -> SeparatorHook
    where
        F: Fn(&S) -> UseSeparatorProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSeparatorProps + Send + Sync> = Arc::new(selector);
        for path in ["separatorProps", "orientation"] {
            self.use_semantic_part(path, &selector, separator_value_part);
        }
        self.register_prop_aliases(&["separatorProps", "orientation"]);

        SeparatorHook {
            separator_props: PropHandle::new("separatorProps"),
            orientation: PropHandle::new("orientation"),
        }
    }

    pub fn use_toolbar<F>(&mut self, selector: F) -> ToolbarHook
    where
        F: Fn(&S) -> UseToolbarProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToolbarProps + Send + Sync> = Arc::new(selector);
        for path in ["toolbarProps", "label", "orientation", "isDisabled"] {
            self.use_semantic_part(path, &selector, toolbar_value_part);
        }
        self.register_prop_aliases(&["toolbarProps", "label", "orientation", "isDisabled"]);

        ToolbarHook {
            toolbar_props: PropHandle::new("toolbarProps"),
            label: PropHandle::new("label"),
            orientation: PropHandle::new("orientation"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_drop_indicator<F>(&mut self, selector: F) -> DropIndicatorHook
    where
        F: Fn(&S) -> UseDropIndicatorProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDropIndicatorProps + Send + Sync> = Arc::new(selector);
        for path in ["dropIndicatorProps", "orientation", "isTarget"] {
            self.use_semantic_part(path, &selector, drop_indicator_value_part);
        }
        self.register_prop_aliases(&["dropIndicatorProps", "orientation", "isTarget"]);

        DropIndicatorHook {
            drop_indicator_props: PropHandle::new("dropIndicatorProps"),
            orientation: PropHandle::new("orientation"),
            is_target: PropHandle::new("isTarget"),
        }
    }

    pub fn use_selection_indicator<F>(&mut self, selector: F) -> SelectionIndicatorHook
    where
        F: Fn(&S) -> UseSelectionIndicatorProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSelectionIndicatorProps + Send + Sync> =
            Arc::new(selector);
        for path in ["selectionIndicatorProps", "label", "isSelected"] {
            self.use_semantic_part(path, &selector, selection_indicator_value_part);
        }
        self.register_prop_aliases(&["selectionIndicatorProps", "label", "isSelected"]);

        SelectionIndicatorHook {
            selection_indicator_props: PropHandle::new("selectionIndicatorProps"),
            label: PropHandle::new("label"),
            is_selected: PropHandle::new("isSelected"),
        }
    }

    pub fn use_i18n<F>(&mut self, selector: F) -> I18nHook
    where
        F: Fn(&S) -> UseI18nProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseI18nProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("i18nProps", move |state| {
            i18n_value_part((props_selector)(state), "i18nProps")
        });
        let locale_selector = Arc::clone(&selector);
        self.use_prop_value("locale", move |state| {
            i18n_value_part((locale_selector)(state), "locale")
        });
        let direction_selector = Arc::clone(&selector);
        self.use_prop_value("direction", move |state| {
            i18n_value_part((direction_selector)(state), "direction")
        });
        let rtl_selector = Arc::clone(&selector);
        self.use_prop_value("isRtl", move |state| {
            i18n_value_part((rtl_selector)(state), "isRtl")
        });

        self.register_alias("i18nProps", BindingAlias::Props("i18nProps".to_string()));
        self.register_alias("locale", BindingAlias::Props("locale".to_string()));
        self.register_alias("direction", BindingAlias::Props("direction".to_string()));
        self.register_alias("isRtl", BindingAlias::Props("isRtl".to_string()));

        I18nHook {
            i18n_props: PropHandle::new("i18nProps"),
            locale: PropHandle::new("locale"),
            direction: PropHandle::new("direction"),
            is_rtl: PropHandle::new("isRtl"),
        }
    }

    pub fn use_overlay<F>(&mut self, selector: F) -> OverlayHook
    where
        F: Fn(&S) -> UseOverlayProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseOverlayProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("overlayProps", move |state| {
            overlay_value_part((props_selector)(state), "overlayProps")
        });
        let trigger_props_selector = Arc::clone(&selector);
        self.use_prop_value("overlayTriggerProps", move |state| {
            overlay_value_part((trigger_props_selector)(state), "overlayTriggerProps")
        });
        let open_selector = Arc::clone(&selector);
        self.use_prop_value("isOpen", move |state| {
            overlay_value_part((open_selector)(state), "isOpen")
        });

        self.register_alias(
            "overlayProps",
            BindingAlias::Props("overlayProps".to_string()),
        );
        self.register_alias(
            "overlayTriggerProps",
            BindingAlias::Props("overlayTriggerProps".to_string()),
        );
        self.register_alias("isOpen", BindingAlias::Props("isOpen".to_string()));

        OverlayHook {
            overlay_props: PropHandle::new("overlayProps"),
            overlay_trigger_props: PropHandle::new("overlayTriggerProps"),
            is_open: PropHandle::new("isOpen"),
        }
    }

    pub fn use_overlay_position<F>(&mut self, selector: F) -> OverlayPositionHook
    where
        F: Fn(&S) -> UseOverlayPositionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseOverlayPositionProps + Send + Sync> = Arc::new(selector);
        for path in ["overlayPositionProps", "arrowProps", "placement"] {
            self.use_semantic_part(path, &selector, overlay_position_value_part);
        }
        self.register_prop_aliases(&["overlayPositionProps", "arrowProps", "placement"]);

        OverlayPositionHook {
            overlay_position_props: PropHandle::new("overlayPositionProps"),
            arrow_props: PropHandle::new("arrowProps"),
            placement: PropHandle::new("placement"),
        }
    }

    pub fn use_menu<F>(&mut self, selector: F) -> MenuHook
    where
        F: Fn(&S) -> UseMenuProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseMenuProps + Send + Sync> = Arc::new(selector);
        for path in [
            "menuProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
        ] {
            self.use_semantic_part(path, &selector, menu_value_part);
        }
        self.register_prop_aliases(&[
            "menuProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
        ]);

        MenuHook {
            menu_props: PropHandle::new("menuProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            is_disabled: PropHandle::new("menuProps.disabled"),
            is_read_only: PropHandle::new("menuProps.readOnly"),
        }
    }

    pub fn use_collection_item<F>(&mut self, selector: F) -> CollectionItemHook
    where
        F: Fn(&S) -> UseCollectionItemProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCollectionItemProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("collectionItemProps", move |state| {
            collection_item_value_part((props_selector)(state), "collectionItemProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("value", move |state| {
            collection_item_value_part((value_selector)(state), "value")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            collection_item_value_part((text_value_selector)(state), "textValue")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            collection_item_value_part((selected_selector)(state), "isSelected")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            collection_item_value_part((disabled_selector)(state), "isDisabled")
        });
        let expanded_selector = Arc::clone(&selector);
        self.use_prop_value("isExpanded", move |state| {
            collection_item_value_part((expanded_selector)(state), "isExpanded")
        });

        self.register_alias(
            "collectionItemProps",
            BindingAlias::Props("collectionItemProps".to_string()),
        );
        self.register_alias("value", BindingAlias::Props("value".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));
        self.register_alias("isExpanded", BindingAlias::Props("isExpanded".to_string()));

        CollectionItemHook {
            collection_item_props: PropHandle::new("collectionItemProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
            is_expanded: PropHandle::new("isExpanded"),
        }
    }

    pub fn use_collection<F>(&mut self, selector: F) -> CollectionHook
    where
        F: Fn(&S) -> UseCollectionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCollectionProps + Send + Sync> = Arc::new(selector);
        for path in [
            "collectionProps",
            "label",
            "itemCount",
            "isEmpty",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, collection_value_part);
        }
        self.register_prop_aliases(&[
            "collectionProps",
            "label",
            "itemCount",
            "isEmpty",
            "isDisabled",
        ]);

        CollectionHook {
            collection_props: PropHandle::new("collectionProps"),
            label: PropHandle::new("label"),
            item_count: PropHandle::new("itemCount"),
            is_empty: PropHandle::new("isEmpty"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_collection_section<F>(&mut self, selector: F) -> CollectionSectionHook
    where
        F: Fn(&S) -> UseCollectionSectionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCollectionSectionProps + Send + Sync> =
            Arc::new(selector);
        for path in [
            "collectionSectionProps",
            "label",
            "collectionKind",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, collection_section_value_part);
        }
        self.register_prop_aliases(&[
            "collectionSectionProps",
            "label",
            "collectionKind",
            "isDisabled",
        ]);

        CollectionSectionHook {
            collection_section_props: PropHandle::new("collectionSectionProps"),
            label: PropHandle::new("label"),
            collection_kind: PropHandle::new("collectionKind"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_load_more_item<F>(&mut self, selector: F) -> LoadMoreItemHook
    where
        F: Fn(&S) -> UseLoadMoreItemProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseLoadMoreItemProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("loadMoreItemProps", move |state| {
            load_more_item_value_part((props_selector)(state), "loadMoreItemProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            load_more_item_value_part((label_selector)(state), "label")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            load_more_item_value_part((text_value_selector)(state), "textValue")
        });
        let action_value_selector = Arc::clone(&selector);
        self.use_prop_value("actionValue", move |state| {
            load_more_item_value_part((action_value_selector)(state), "actionValue")
        });
        let action_payload_selector = Arc::clone(&selector);
        self.use_prop_value("actionPayload", move |state| {
            load_more_item_value_part((action_payload_selector)(state), "actionPayload")
        });
        let loading_selector = Arc::clone(&selector);
        self.use_prop_value("isLoading", move |state| {
            load_more_item_value_part((loading_selector)(state), "isLoading")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            load_more_item_value_part((disabled_selector)(state), "isDisabled")
        });

        self.register_alias(
            "loadMoreItemProps",
            BindingAlias::Props("loadMoreItemProps".to_string()),
        );
        self.register_alias("label", BindingAlias::Props("label".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));
        self.register_alias(
            "actionValue",
            BindingAlias::Props("actionValue".to_string()),
        );
        self.register_alias(
            "actionPayload",
            BindingAlias::Props("actionPayload".to_string()),
        );
        self.register_alias("isLoading", BindingAlias::Props("isLoading".to_string()));
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));

        LoadMoreItemHook {
            load_more_item_props: PropHandle::new("loadMoreItemProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
            action_value: PropHandle::new("actionValue"),
            action_payload: PropHandle::new("actionPayload"),
            is_loading: PropHandle::new("isLoading"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_radio_group<F>(&mut self, selector: F) -> RadioGroupHook
    where
        F: Fn(&S) -> UseRadioGroupProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseRadioGroupProps + Send + Sync> = Arc::new(selector);
        for path in [
            "radioGroupProps",
            "label",
            "selectedValue",
            "selectionMode",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, radio_group_value_part);
        }
        self.register_prop_aliases(&[
            "radioGroupProps",
            "label",
            "selectedValue",
            "selectionMode",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        RadioGroupHook {
            radio_group_props: PropHandle::new("radioGroupProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selection_mode: PropHandle::new("selectionMode"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_radio<F>(&mut self, selector: F) -> RadioHook
    where
        F: Fn(&S) -> UseRadioProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseRadioProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("radioProps", move |state| {
            radio_value_part((props_selector)(state), "radioProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("value", move |state| {
            radio_value_part((value_selector)(state), "value")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            radio_value_part((text_value_selector)(state), "textValue")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            radio_value_part((selected_selector)(state), "isSelected")
        });
        let checked_selector = Arc::clone(&selector);
        self.use_prop_value("isChecked", move |state| {
            radio_value_part((checked_selector)(state), "isChecked")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            radio_value_part((disabled_selector)(state), "isDisabled")
        });

        self.register_alias("radioProps", BindingAlias::Props("radioProps".to_string()));
        self.register_alias("value", BindingAlias::Props("value".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("isChecked", BindingAlias::Props("isChecked".to_string()));
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));

        RadioHook {
            radio_props: PropHandle::new("radioProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_checked: PropHandle::new("isChecked"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_tab<F>(&mut self, selector: F) -> TabHook
    where
        F: Fn(&S) -> UseTabProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTabProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tabProps", move |state| {
            tab_value_part((props_selector)(state), "tabProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("value", move |state| {
            tab_value_part((value_selector)(state), "value")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            tab_value_part((text_value_selector)(state), "textValue")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            tab_value_part((selected_selector)(state), "isSelected")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            tab_value_part((disabled_selector)(state), "isDisabled")
        });

        self.register_alias("tabProps", BindingAlias::Props("tabProps".to_string()));
        self.register_alias("value", BindingAlias::Props("value".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));

        TabHook {
            tab_props: PropHandle::new("tabProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_tab_list<F>(&mut self, selector: F) -> TabListHook
    where
        F: Fn(&S) -> UseTabListProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTabListProps + Send + Sync> = Arc::new(selector);
        for path in ["tabListProps", "label", "orientation", "isDisabled"] {
            self.use_semantic_part(path, &selector, tab_list_value_part);
        }
        self.register_prop_aliases(&["tabListProps", "label", "orientation", "isDisabled"]);

        TabListHook {
            tab_list_props: PropHandle::new("tabListProps"),
            label: PropHandle::new("label"),
            orientation: PropHandle::new("orientation"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_tab_panel<F>(&mut self, selector: F) -> TabPanelHook
    where
        F: Fn(&S) -> UseTabPanelProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTabPanelProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tabPanelProps", move |state| {
            tab_panel_value_part((props_selector)(state), "tabPanelProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("value", move |state| {
            tab_panel_value_part((value_selector)(state), "value")
        });

        self.register_alias(
            "tabPanelProps",
            BindingAlias::Props("tabPanelProps".to_string()),
        );
        self.register_alias("value", BindingAlias::Props("value".to_string()));

        TabPanelHook {
            tab_panel_props: PropHandle::new("tabPanelProps"),
            value: PropHandle::new("value"),
        }
    }

    pub fn use_table<F>(&mut self, selector: F) -> TableHook
    where
        F: Fn(&S) -> UseTableProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tableProps", move |state| {
            table_value_part((props_selector)(state), "tableProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            table_value_part((label_selector)(state), "label")
        });

        self.register_alias("tableProps", BindingAlias::Props("tableProps".to_string()));
        self.register_alias("label", BindingAlias::Props("label".to_string()));

        TableHook {
            table_props: PropHandle::new("tableProps"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_table_section<F>(&mut self, selector: F) -> TableSectionHook
    where
        F: Fn(&S) -> UseTableSectionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableSectionProps + Send + Sync> = Arc::new(selector);
        for path in ["tableSectionProps", "sectionKind", "label"] {
            self.use_semantic_part(path, &selector, table_section_value_part);
        }
        self.register_prop_aliases(&["tableSectionProps", "sectionKind", "label"]);

        TableSectionHook {
            table_section_props: PropHandle::new("tableSectionProps"),
            section_kind: PropHandle::new("sectionKind"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_table_row<F>(&mut self, selector: F) -> TableRowHook
    where
        F: Fn(&S) -> UseTableRowProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableRowProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tableRowProps", move |state| {
            table_row_value_part((props_selector)(state), "tableRowProps")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            table_row_value_part((selected_selector)(state), "isSelected")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            table_row_value_part((disabled_selector)(state), "isDisabled")
        });

        self.register_alias(
            "tableRowProps",
            BindingAlias::Props("tableRowProps".to_string()),
        );
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));

        TableRowHook {
            table_row_props: PropHandle::new("tableRowProps"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_table_cell<F>(&mut self, selector: F) -> TableCellHook
    where
        F: Fn(&S) -> UseTableCellProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableCellProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tableCellProps", move |state| {
            table_cell_value_part((props_selector)(state), "tableCellProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            table_cell_value_part((label_selector)(state), "label")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            table_cell_value_part((text_value_selector)(state), "textValue")
        });

        self.register_alias(
            "tableCellProps",
            BindingAlias::Props("tableCellProps".to_string()),
        );
        self.register_alias("label", BindingAlias::Props("label".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));

        TableCellHook {
            table_cell_props: PropHandle::new("tableCellProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_table_column<F>(&mut self, selector: F) -> TableColumnHook
    where
        F: Fn(&S) -> UseTableColumnProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableColumnProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tableColumnProps", move |state| {
            table_column_value_part((props_selector)(state), "tableColumnProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            table_column_value_part((label_selector)(state), "label")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            table_column_value_part((text_value_selector)(state), "textValue")
        });

        self.register_alias(
            "tableColumnProps",
            BindingAlias::Props("tableColumnProps".to_string()),
        );
        self.register_alias("label", BindingAlias::Props("label".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));

        TableColumnHook {
            table_column_props: PropHandle::new("tableColumnProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_table_caption<F>(&mut self, selector: F) -> TableCaptionHook
    where
        F: Fn(&S) -> UseTableCaptionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTableCaptionProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("tableCaptionProps", move |state| {
            table_caption_value_part((props_selector)(state), "tableCaptionProps")
        });
        let label_selector = Arc::clone(&selector);
        self.use_prop_value("label", move |state| {
            table_caption_value_part((label_selector)(state), "label")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            table_caption_value_part((text_value_selector)(state), "textValue")
        });

        self.register_alias(
            "tableCaptionProps",
            BindingAlias::Props("tableCaptionProps".to_string()),
        );
        self.register_alias("label", BindingAlias::Props("label".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));

        TableCaptionHook {
            table_caption_props: PropHandle::new("tableCaptionProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_text<F>(&mut self, selector: F) -> TextHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["textProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, text_value_part);
        }
        self.register_prop_aliases(&["textProps", "label", "textValue"]);

        TextHook {
            text_props: PropHandle::new("textProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_label<F>(&mut self, selector: F) -> LabelHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["labelProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, label_value_part);
        }
        self.register_prop_aliases(&["labelProps", "label", "textValue"]);

        LabelHook {
            label_props: PropHandle::new("labelProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_description<F>(&mut self, selector: F) -> DescriptionHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["descriptionProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, description_value_part);
        }
        self.register_prop_aliases(&["descriptionProps", "label", "textValue"]);

        DescriptionHook {
            description_props: PropHandle::new("descriptionProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_field_error<F>(&mut self, selector: F) -> FieldErrorHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["fieldErrorProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, field_error_value_part);
        }
        self.register_prop_aliases(&["fieldErrorProps", "label", "textValue"]);

        FieldErrorHook {
            field_error_props: PropHandle::new("fieldErrorProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_legend<F>(&mut self, selector: F) -> LegendHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["legendProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, legend_value_part);
        }
        self.register_prop_aliases(&["legendProps", "label", "textValue"]);

        LegendHook {
            legend_props: PropHandle::new("legendProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_visually_hidden<F>(&mut self, selector: F) -> VisuallyHiddenHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["visuallyHiddenProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, visually_hidden_value_part);
        }
        self.register_prop_aliases(&["visuallyHiddenProps", "label", "textValue"]);

        VisuallyHiddenHook {
            visually_hidden_props: PropHandle::new("visuallyHiddenProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_keyboard<F>(&mut self, selector: F) -> KeyboardHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["keyboardProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, keyboard_value_part);
        }
        self.register_prop_aliases(&["keyboardProps", "label", "textValue"]);

        KeyboardHook {
            keyboard_props: PropHandle::new("keyboardProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_list_box_header<F>(&mut self, selector: F) -> ListBoxHeaderHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["listBoxHeaderProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, list_box_header_value_part);
        }
        self.register_prop_aliases(&["listBoxHeaderProps", "label", "textValue"]);

        ListBoxHeaderHook {
            list_box_header_props: PropHandle::new("listBoxHeaderProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_grid_list_header<F>(&mut self, selector: F) -> GridListHeaderHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["gridListHeaderProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, grid_list_header_value_part);
        }
        self.register_prop_aliases(&["gridListHeaderProps", "label", "textValue"]);

        GridListHeaderHook {
            grid_list_header_props: PropHandle::new("gridListHeaderProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_tree_header<F>(&mut self, selector: F) -> TreeHeaderHook
    where
        F: Fn(&S) -> UseTextProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextProps + Send + Sync> = Arc::new(selector);
        for path in ["treeHeaderProps", "label", "textValue"] {
            self.use_semantic_part(path, &selector, tree_header_value_part);
        }
        self.register_prop_aliases(&["treeHeaderProps", "label", "textValue"]);

        TreeHeaderHook {
            tree_header_props: PropHandle::new("treeHeaderProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_heading<F>(&mut self, selector: F) -> HeadingHook
    where
        F: Fn(&S) -> UseHeadingProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseHeadingProps + Send + Sync> = Arc::new(selector);
        for path in ["headingProps", "label", "textValue", "level"] {
            self.use_semantic_part(path, &selector, heading_value_part);
        }
        self.register_prop_aliases(&["headingProps", "label", "textValue", "level"]);

        HeadingHook {
            heading_props: PropHandle::new("headingProps"),
            label: PropHandle::new("label"),
            text_value: PropHandle::new("textValue"),
            level: PropHandle::new("level"),
        }
    }

    pub fn use_date_field<F>(&mut self, selector: F) -> DateFieldHook
    where
        F: Fn(&S) -> UseDateFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDateFieldProps + Send + Sync> = Arc::new(selector);
        for path in [
            "dateFieldProps",
            "dateFieldInputProps",
            "label",
            "value",
            "placeholder",
            "granularity",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, date_field_value_part);
        }
        self.register_prop_aliases(&[
            "dateFieldProps",
            "dateFieldInputProps",
            "label",
            "value",
            "placeholder",
            "granularity",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        DateFieldHook {
            date_field_props: PropHandle::new("dateFieldProps"),
            date_field_input_props: PropHandle::new("dateFieldInputProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            placeholder: PropHandle::new("placeholder"),
            granularity: PropHandle::new("granularity"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_time_field<F>(&mut self, selector: F) -> TimeFieldHook
    where
        F: Fn(&S) -> UseTimeFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTimeFieldProps + Send + Sync> = Arc::new(selector);
        for path in [
            "timeFieldProps",
            "timeFieldInputProps",
            "label",
            "value",
            "placeholder",
            "granularity",
            "hourCycle",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, time_field_value_part);
        }
        self.register_prop_aliases(&[
            "timeFieldProps",
            "timeFieldInputProps",
            "label",
            "value",
            "placeholder",
            "granularity",
            "hourCycle",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        TimeFieldHook {
            time_field_props: PropHandle::new("timeFieldProps"),
            time_field_input_props: PropHandle::new("timeFieldInputProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            placeholder: PropHandle::new("placeholder"),
            granularity: PropHandle::new("granularity"),
            hour_cycle: PropHandle::new("hourCycle"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_date_input<F>(&mut self, selector: F) -> DateInputHook
    where
        F: Fn(&S) -> UseDateInputProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDateInputProps + Send + Sync> = Arc::new(selector);
        for path in [
            "dateInputProps",
            "label",
            "value",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, date_input_value_part);
        }
        self.register_prop_aliases(&[
            "dateInputProps",
            "label",
            "value",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ]);

        DateInputHook {
            date_input_props: PropHandle::new("dateInputProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_date_segment<F>(&mut self, selector: F) -> DateSegmentHook
    where
        F: Fn(&S) -> UseDateSegmentProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDateSegmentProps + Send + Sync> = Arc::new(selector);
        for path in [
            "dateSegmentProps",
            "segmentType",
            "value",
            "textValue",
            "placeholder",
            "isPlaceholder",
            "isDisabled",
            "isInvalid",
        ] {
            self.use_semantic_part(path, &selector, date_segment_value_part);
        }
        self.register_prop_aliases(&[
            "dateSegmentProps",
            "segmentType",
            "value",
            "textValue",
            "placeholder",
            "isPlaceholder",
            "isDisabled",
            "isInvalid",
        ]);

        DateSegmentHook {
            date_segment_props: PropHandle::new("dateSegmentProps"),
            segment_type: PropHandle::new("segmentType"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            placeholder: PropHandle::new("placeholder"),
            is_placeholder: PropHandle::new("isPlaceholder"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
        }
    }

    pub fn use_calendar<F>(&mut self, selector: F) -> CalendarHook
    where
        F: Fn(&S) -> UseCalendarProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCalendarProps + Send + Sync> = Arc::new(selector);
        for path in [
            "calendarProps",
            "label",
            "value",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, calendar_value_part);
        }
        self.register_prop_aliases(&[
            "calendarProps",
            "label",
            "value",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ]);

        CalendarHook {
            calendar_props: PropHandle::new("calendarProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_range_calendar<F>(&mut self, selector: F) -> RangeCalendarHook
    where
        F: Fn(&S) -> UseRangeCalendarProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseRangeCalendarProps + Send + Sync> = Arc::new(selector);
        for path in [
            "rangeCalendarProps",
            "label",
            "startValue",
            "endValue",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, range_calendar_value_part);
        }
        self.register_prop_aliases(&[
            "rangeCalendarProps",
            "label",
            "startValue",
            "endValue",
            "isDisabled",
            "isInvalid",
            "isReadOnly",
        ]);

        RangeCalendarHook {
            range_calendar_props: PropHandle::new("rangeCalendarProps"),
            label: PropHandle::new("label"),
            start_value: PropHandle::new("startValue"),
            end_value: PropHandle::new("endValue"),
            is_disabled: PropHandle::new("isDisabled"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_calendar_cell<F>(&mut self, selector: F) -> CalendarCellHook
    where
        F: Fn(&S) -> UseCalendarCellProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseCalendarCellProps + Send + Sync> = Arc::new(selector);
        for path in [
            "calendarCellProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
            "isUnavailable",
            "isOutsideMonth",
            "isToday",
            "isPressed",
        ] {
            self.use_semantic_part(path, &selector, calendar_cell_value_part);
        }
        self.register_prop_aliases(&[
            "calendarCellProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
            "isUnavailable",
            "isOutsideMonth",
            "isToday",
            "isPressed",
        ]);

        CalendarCellHook {
            calendar_cell_props: PropHandle::new("calendarCellProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
            is_unavailable: PropHandle::new("isUnavailable"),
            is_outside_month: PropHandle::new("isOutsideMonth"),
            is_today: PropHandle::new("isToday"),
            is_pressed: PropHandle::new("isPressed"),
        }
    }

    pub fn use_date_picker<F>(&mut self, selector: F) -> DatePickerHook
    where
        F: Fn(&S) -> UseDatePickerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDatePickerProps + Send + Sync> = Arc::new(selector);
        for path in [
            "datePickerProps",
            "datePickerInputProps",
            "datePickerTriggerProps",
            "label",
            "value",
            "placeholder",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, date_picker_value_part);
        }
        self.register_prop_aliases(&[
            "datePickerProps",
            "datePickerInputProps",
            "datePickerTriggerProps",
            "label",
            "value",
            "placeholder",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        DatePickerHook {
            date_picker_props: PropHandle::new("datePickerProps"),
            date_picker_input_props: PropHandle::new("datePickerInputProps"),
            date_picker_trigger_props: PropHandle::new("datePickerTriggerProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            placeholder: PropHandle::new("placeholder"),
            is_open: PropHandle::new("isOpen"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_date_range_picker<F>(&mut self, selector: F) -> DateRangePickerHook
    where
        F: Fn(&S) -> UseDateRangePickerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDateRangePickerProps + Send + Sync> = Arc::new(selector);
        for path in [
            "dateRangePickerProps",
            "dateRangePickerStartInputProps",
            "dateRangePickerEndInputProps",
            "dateRangePickerTriggerProps",
            "label",
            "startValue",
            "endValue",
            "placeholder",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, date_range_picker_value_part);
        }
        self.register_prop_aliases(&[
            "dateRangePickerProps",
            "dateRangePickerStartInputProps",
            "dateRangePickerEndInputProps",
            "dateRangePickerTriggerProps",
            "label",
            "startValue",
            "endValue",
            "placeholder",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        DateRangePickerHook {
            date_range_picker_props: PropHandle::new("dateRangePickerProps"),
            date_range_picker_start_input_props: PropHandle::new("dateRangePickerStartInputProps"),
            date_range_picker_end_input_props: PropHandle::new("dateRangePickerEndInputProps"),
            date_range_picker_trigger_props: PropHandle::new("dateRangePickerTriggerProps"),
            label: PropHandle::new("label"),
            start_value: PropHandle::new("startValue"),
            end_value: PropHandle::new("endValue"),
            placeholder: PropHandle::new("placeholder"),
            is_open: PropHandle::new("isOpen"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_field<F>(&mut self, selector: F) -> ColorFieldHook
    where
        F: Fn(&S) -> UseColorFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorFieldProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorFieldProps",
            "colorFieldInputProps",
            "label",
            "value",
            "placeholder",
            "colorSpace",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_field_value_part);
        }
        self.register_prop_aliases(&[
            "colorFieldProps",
            "colorFieldInputProps",
            "label",
            "value",
            "placeholder",
            "colorSpace",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        ColorFieldHook {
            color_field_props: PropHandle::new("colorFieldProps"),
            color_field_input_props: PropHandle::new("colorFieldInputProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            placeholder: PropHandle::new("placeholder"),
            color_space: PropHandle::new("colorSpace"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_picker<F>(&mut self, selector: F) -> ColorPickerHook
    where
        F: Fn(&S) -> UseColorPickerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorPickerProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorPickerProps",
            "label",
            "value",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_picker_value_part);
        }
        self.register_prop_aliases(&[
            "colorPickerProps",
            "label",
            "value",
            "isDisabled",
            "isReadOnly",
        ]);

        ColorPickerHook {
            color_picker_props: PropHandle::new("colorPickerProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_area<F>(&mut self, selector: F) -> ColorAreaHook
    where
        F: Fn(&S) -> UseColorAreaProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorAreaProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorAreaProps",
            "label",
            "value",
            "xChannel",
            "yChannel",
            "xValue",
            "yValue",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_area_value_part);
        }
        self.register_prop_aliases(&[
            "colorAreaProps",
            "label",
            "value",
            "xChannel",
            "yChannel",
            "xValue",
            "yValue",
            "isDisabled",
            "isReadOnly",
        ]);

        ColorAreaHook {
            color_area_props: PropHandle::new("colorAreaProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            x_channel: PropHandle::new("xChannel"),
            y_channel: PropHandle::new("yChannel"),
            x_value: PropHandle::new("xValue"),
            y_value: PropHandle::new("yValue"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_slider<F>(&mut self, selector: F) -> ColorSliderHook
    where
        F: Fn(&S) -> UseColorRangeProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorRangeProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorSliderProps",
            "label",
            "channel",
            "valueNumber",
            "minValue",
            "maxValue",
            "stepValue",
            "valuePercent",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_slider_value_part);
        }
        self.register_prop_aliases(&[
            "colorSliderProps",
            "label",
            "channel",
            "valueNumber",
            "minValue",
            "maxValue",
            "stepValue",
            "valuePercent",
            "isDisabled",
            "isReadOnly",
        ]);

        ColorSliderHook {
            color_slider_props: PropHandle::new("colorSliderProps"),
            label: PropHandle::new("label"),
            channel: PropHandle::new("channel"),
            value_number: PropHandle::new("valueNumber"),
            min_value: PropHandle::new("minValue"),
            max_value: PropHandle::new("maxValue"),
            step_value: PropHandle::new("stepValue"),
            value_percent: PropHandle::new("valuePercent"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_wheel<F>(&mut self, selector: F) -> ColorWheelHook
    where
        F: Fn(&S) -> UseColorRangeProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorRangeProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorWheelProps",
            "label",
            "channel",
            "valueNumber",
            "minValue",
            "maxValue",
            "stepValue",
            "valuePercent",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_wheel_value_part);
        }
        self.register_prop_aliases(&[
            "colorWheelProps",
            "label",
            "channel",
            "valueNumber",
            "minValue",
            "maxValue",
            "stepValue",
            "valuePercent",
            "isDisabled",
            "isReadOnly",
        ]);

        ColorWheelHook {
            color_wheel_props: PropHandle::new("colorWheelProps"),
            label: PropHandle::new("label"),
            channel: PropHandle::new("channel"),
            value_number: PropHandle::new("valueNumber"),
            min_value: PropHandle::new("minValue"),
            max_value: PropHandle::new("maxValue"),
            step_value: PropHandle::new("stepValue"),
            value_percent: PropHandle::new("valuePercent"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_swatch_picker<F>(&mut self, selector: F) -> ColorSwatchPickerHook
    where
        F: Fn(&S) -> UseColorSwatchPickerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorSwatchPickerProps + Send + Sync> =
            Arc::new(selector);
        for path in [
            "colorSwatchPickerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, color_swatch_picker_value_part);
        }
        self.register_prop_aliases(&[
            "colorSwatchPickerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isDisabled",
            "isReadOnly",
        ]);

        ColorSwatchPickerHook {
            color_swatch_picker_props: PropHandle::new("colorSwatchPickerProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_color_swatch_picker_item<F>(&mut self, selector: F) -> ColorSwatchPickerItemHook
    where
        F: Fn(&S) -> UseColorSwatchPickerItemProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorSwatchPickerItemProps + Send + Sync> =
            Arc::new(selector);
        for path in [
            "colorSwatchPickerItemProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, color_swatch_picker_item_value_part);
        }
        self.register_prop_aliases(&[
            "colorSwatchPickerItemProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
        ]);

        ColorSwatchPickerItemHook {
            color_swatch_picker_item_props: PropHandle::new("colorSwatchPickerItemProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_color_swatch<F>(&mut self, selector: F) -> ColorSwatchHook
    where
        F: Fn(&S) -> UseColorSwatchProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorSwatchProps + Send + Sync> = Arc::new(selector);
        for path in ["colorSwatchProps", "label", "value", "isDisabled"] {
            self.use_semantic_part(path, &selector, color_swatch_value_part);
        }
        self.register_prop_aliases(&["colorSwatchProps", "label", "value", "isDisabled"]);

        ColorSwatchHook {
            color_swatch_props: PropHandle::new("colorSwatchProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_color_thumb<F>(&mut self, selector: F) -> ColorThumbHook
    where
        F: Fn(&S) -> UseColorThumbProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseColorThumbProps + Send + Sync> = Arc::new(selector);
        for path in [
            "colorThumbProps",
            "value",
            "xValue",
            "yValue",
            "isPressed",
            "isDragging",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, color_thumb_value_part);
        }
        self.register_prop_aliases(&[
            "colorThumbProps",
            "value",
            "xValue",
            "yValue",
            "isPressed",
            "isDragging",
            "isDisabled",
        ]);

        ColorThumbHook {
            color_thumb_props: PropHandle::new("colorThumbProps"),
            value: PropHandle::new("value"),
            x_value: PropHandle::new("xValue"),
            y_value: PropHandle::new("yValue"),
            is_pressed: PropHandle::new("isPressed"),
            is_dragging: PropHandle::new("isDragging"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_combo_box<F>(&mut self, selector: F) -> ComboBoxHook
    where
        F: Fn(&S) -> UseComboBoxProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseComboBoxProps + Send + Sync> = Arc::new(selector);
        for path in [
            "comboBoxProps",
            "comboBoxInputProps",
            "comboBoxTriggerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "inputValue",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, combo_box_value_part);
        }
        self.register_prop_aliases(&[
            "comboBoxProps",
            "comboBoxInputProps",
            "comboBoxTriggerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "inputValue",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        ComboBoxHook {
            combo_box_props: PropHandle::new("comboBoxProps"),
            combo_box_input_props: PropHandle::new("comboBoxInputProps"),
            combo_box_trigger_props: PropHandle::new("comboBoxTriggerProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            input_value: PropHandle::new("inputValue"),
            placeholder: PropHandle::new("placeholder"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            is_open: PropHandle::new("isOpen"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_autocomplete<F>(&mut self, selector: F) -> AutocompleteHook
    where
        F: Fn(&S) -> UseAutocompleteProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseAutocompleteProps + Send + Sync> = Arc::new(selector);
        for path in [
            "autocompleteProps",
            "autocompleteInputProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "inputValue",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, autocomplete_value_part);
        }
        self.register_prop_aliases(&[
            "autocompleteProps",
            "autocompleteInputProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "inputValue",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        AutocompleteHook {
            autocomplete_props: PropHandle::new("autocompleteProps"),
            autocomplete_input_props: PropHandle::new("autocompleteInputProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            input_value: PropHandle::new("inputValue"),
            placeholder: PropHandle::new("placeholder"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_select<F>(&mut self, selector: F) -> SelectHook
    where
        F: Fn(&S) -> UseSelectProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSelectProps + Send + Sync> = Arc::new(selector);
        for path in [
            "selectProps",
            "selectTriggerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, select_value_part);
        }
        self.register_prop_aliases(&[
            "selectProps",
            "selectTriggerProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "placeholder",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "isOpen",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        SelectHook {
            select_props: PropHandle::new("selectProps"),
            select_trigger_props: PropHandle::new("selectTriggerProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            placeholder: PropHandle::new("placeholder"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            is_open: PropHandle::new("isOpen"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_select_display<F>(&mut self, selector: F) -> SelectDisplayHook
    where
        F: Fn(&S) -> UseSelectDisplayProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSelectDisplayProps + Send + Sync> = Arc::new(selector);
        for path in ["selectValueProps", "value", "displayValue", "isPlaceholder"] {
            self.use_semantic_part(path, &selector, select_display_value_part);
        }
        self.register_prop_aliases(&["selectValueProps", "value", "displayValue", "isPlaceholder"]);

        SelectDisplayHook {
            select_value_props: PropHandle::new("selectValueProps"),
            value: PropHandle::new("value"),
            display_value: PropHandle::new("displayValue"),
            is_placeholder: PropHandle::new("isPlaceholder"),
        }
    }

    pub fn use_combo_box_display<F>(&mut self, selector: F) -> ComboBoxDisplayHook
    where
        F: Fn(&S) -> UseComboBoxDisplayProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseComboBoxDisplayProps + Send + Sync> = Arc::new(selector);
        for path in [
            "comboBoxValueProps",
            "value",
            "displayValue",
            "isPlaceholder",
        ] {
            self.use_semantic_part(path, &selector, combo_box_display_value_part);
        }
        self.register_prop_aliases(&[
            "comboBoxValueProps",
            "value",
            "displayValue",
            "isPlaceholder",
        ]);

        ComboBoxDisplayHook {
            combo_box_value_props: PropHandle::new("comboBoxValueProps"),
            value: PropHandle::new("value"),
            display_value: PropHandle::new("displayValue"),
            is_placeholder: PropHandle::new("isPlaceholder"),
        }
    }

    pub fn use_menu_item<F>(&mut self, selector: F) -> MenuItemHook
    where
        F: Fn(&S) -> UseMenuItemProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseMenuItemProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("menuItemProps", move |state| {
            menu_item_value_part((props_selector)(state), "menuItemProps")
        });
        let disabled_selector = Arc::clone(&selector);
        self.use_prop_value("isDisabled", move |state| {
            menu_item_value_part((disabled_selector)(state), "isDisabled")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            menu_item_value_part((selected_selector)(state), "isSelected")
        });
        let text_value_selector = Arc::clone(&selector);
        self.use_prop_value("textValue", move |state| {
            menu_item_value_part((text_value_selector)(state), "textValue")
        });

        self.register_alias(
            "menuItemProps",
            BindingAlias::Props("menuItemProps".to_string()),
        );
        self.register_alias("isDisabled", BindingAlias::Props("isDisabled".to_string()));
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("textValue", BindingAlias::Props("textValue".to_string()));

        MenuItemHook {
            menu_item_props: PropHandle::new("menuItemProps"),
            is_disabled: PropHandle::new("isDisabled"),
            is_selected: PropHandle::new("isSelected"),
            text_value: PropHandle::new("textValue"),
        }
    }

    pub fn use_submenu_trigger<F>(&mut self, selector: F) -> SubmenuTriggerHook
    where
        F: Fn(&S) -> UseSubmenuTriggerProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSubmenuTriggerProps + Send + Sync> = Arc::new(selector);
        for path in ["submenuTriggerProps", "isDisabled", "isPressed", "isOpen"] {
            self.use_semantic_part(path, &selector, submenu_trigger_value_part);
        }
        self.register_prop_aliases(&["submenuTriggerProps", "isDisabled", "isPressed", "isOpen"]);

        SubmenuTriggerHook {
            submenu_trigger_props: PropHandle::new("submenuTriggerProps"),
            is_disabled: PropHandle::new("isDisabled"),
            is_pressed: PropHandle::new("isPressed"),
            is_open: PropHandle::new("isOpen"),
        }
    }

    pub fn use_selection<F>(&mut self, selector: F) -> SelectionHook
    where
        F: Fn(&S) -> UseSelectionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSelectionProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("selectionProps", move |state| {
            selection_value_part((props_selector)(state), "selectionProps")
        });
        let selected_value_selector = Arc::clone(&selector);
        self.use_prop_value("selectedValue", move |state| {
            selection_value_part((selected_value_selector)(state), "selectedValue")
        });
        let selected_keys_selector = Arc::clone(&selector);
        self.use_prop_value("selectedKeys", move |state| {
            selection_value_part((selected_keys_selector)(state), "selectedKeys")
        });
        let selection_mode_selector = Arc::clone(&selector);
        self.use_prop_value("selectionMode", move |state| {
            selection_value_part((selection_mode_selector)(state), "selectionMode")
        });
        let selection_behavior_selector = Arc::clone(&selector);
        self.use_prop_value("selectionBehavior", move |state| {
            selection_value_part((selection_behavior_selector)(state), "selectionBehavior")
        });
        let disabled_behavior_selector = Arc::clone(&selector);
        self.use_prop_value("disabledBehavior", move |state| {
            selection_value_part((disabled_behavior_selector)(state), "disabledBehavior")
        });
        let escape_key_behavior_selector = Arc::clone(&selector);
        self.use_prop_value("escapeKeyBehavior", move |state| {
            selection_value_part((escape_key_behavior_selector)(state), "escapeKeyBehavior")
        });

        self.register_alias(
            "selectionProps",
            BindingAlias::Props("selectionProps".to_string()),
        );
        self.register_alias(
            "selectedValue",
            BindingAlias::Props("selectedValue".to_string()),
        );
        self.register_alias(
            "selectedKeys",
            BindingAlias::Props("selectedKeys".to_string()),
        );
        self.register_alias(
            "selectionMode",
            BindingAlias::Props("selectionMode".to_string()),
        );
        self.register_alias(
            "selectionBehavior",
            BindingAlias::Props("selectionBehavior".to_string()),
        );
        self.register_alias(
            "disabledBehavior",
            BindingAlias::Props("disabledBehavior".to_string()),
        );
        self.register_alias(
            "escapeKeyBehavior",
            BindingAlias::Props("escapeKeyBehavior".to_string()),
        );

        SelectionHook {
            selection_props: PropHandle::new("selectionProps"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            escape_key_behavior: PropHandle::new("escapeKeyBehavior"),
        }
    }

    pub fn use_tree<F>(&mut self, selector: F) -> TreeHook
    where
        F: Fn(&S) -> UseTreeProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTreeProps + Send + Sync> = Arc::new(selector);
        for path in [
            "treeProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "expandedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "escapeKeyBehavior",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, tree_value_part);
        }
        self.register_prop_aliases(&[
            "treeProps",
            "label",
            "selectedValue",
            "selectedKeys",
            "expandedKeys",
            "selectionMode",
            "selectionBehavior",
            "disabledBehavior",
            "escapeKeyBehavior",
            "isDisabled",
            "isReadOnly",
        ]);

        TreeHook {
            tree_props: PropHandle::new("treeProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            selected_keys: PropHandle::new("selectedKeys"),
            expanded_keys: PropHandle::new("expandedKeys"),
            selection_mode: PropHandle::new("selectionMode"),
            selection_behavior: PropHandle::new("selectionBehavior"),
            disabled_behavior: PropHandle::new("disabledBehavior"),
            escape_key_behavior: PropHandle::new("escapeKeyBehavior"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_tree_item<F>(&mut self, selector: F) -> TreeItemHook
    where
        F: Fn(&S) -> UseTreeItemProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTreeItemProps + Send + Sync> = Arc::new(selector);
        for path in [
            "treeItemProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
            "isExpanded",
            "hasChildItems",
        ] {
            self.use_semantic_part(path, &selector, tree_item_value_part);
        }
        self.register_prop_aliases(&[
            "treeItemProps",
            "value",
            "textValue",
            "isSelected",
            "isDisabled",
            "isExpanded",
            "hasChildItems",
        ]);

        TreeItemHook {
            tree_item_props: PropHandle::new("treeItemProps"),
            value: PropHandle::new("value"),
            text_value: PropHandle::new("textValue"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
            is_expanded: PropHandle::new("isExpanded"),
            has_child_items: PropHandle::new("hasChildItems"),
        }
    }

    pub fn use_disclosure_group<F>(&mut self, selector: F) -> DisclosureGroupHook
    where
        F: Fn(&S) -> UseDisclosureGroupProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDisclosureGroupProps + Send + Sync> = Arc::new(selector);
        for path in [
            "disclosureGroupProps",
            "label",
            "expandedKeys",
            "allowsMultipleExpanded",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, disclosure_group_value_part);
        }
        self.register_prop_aliases(&[
            "disclosureGroupProps",
            "label",
            "expandedKeys",
            "allowsMultipleExpanded",
            "isDisabled",
        ]);

        DisclosureGroupHook {
            disclosure_group_props: PropHandle::new("disclosureGroupProps"),
            label: PropHandle::new("label"),
            expanded_keys: PropHandle::new("expandedKeys"),
            allows_multiple_expanded: PropHandle::new("allowsMultipleExpanded"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_disclosure<F>(&mut self, selector: F) -> DisclosureHook
    where
        F: Fn(&S) -> UseDisclosureProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseDisclosureProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("disclosureProps", move |state| {
            disclosure_value_part((props_selector)(state), "disclosureProps")
        });
        let trigger_props_selector = Arc::clone(&selector);
        self.use_prop_value("disclosureTriggerProps", move |state| {
            disclosure_value_part((trigger_props_selector)(state), "disclosureTriggerProps")
        });
        let panel_props_selector = Arc::clone(&selector);
        self.use_prop_value("disclosurePanelProps", move |state| {
            disclosure_value_part((panel_props_selector)(state), "disclosurePanelProps")
        });
        let expanded_selector = Arc::clone(&selector);
        self.use_prop_value("isExpanded", move |state| {
            disclosure_value_part((expanded_selector)(state), "isExpanded")
        });

        self.register_alias(
            "disclosureProps",
            BindingAlias::Props("disclosureProps".to_string()),
        );
        self.register_alias(
            "disclosureTriggerProps",
            BindingAlias::Props("disclosureTriggerProps".to_string()),
        );
        self.register_alias(
            "disclosurePanelProps",
            BindingAlias::Props("disclosurePanelProps".to_string()),
        );
        self.register_alias("isExpanded", BindingAlias::Props("isExpanded".to_string()));

        DisclosureHook {
            disclosure_props: PropHandle::new("disclosureProps"),
            disclosure_trigger_props: PropHandle::new("disclosureTriggerProps"),
            disclosure_panel_props: PropHandle::new("disclosurePanelProps"),
            is_expanded: PropHandle::new("isExpanded"),
        }
    }

    pub fn use_range<F>(&mut self, selector: F) -> RangeHook
    where
        F: Fn(&S) -> UseRangeProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseRangeProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("rangeProps", move |state| {
            range_value_part((props_selector)(state), "rangeProps")
        });
        let input_props_selector = Arc::clone(&selector);
        self.use_prop_value("rangeInputProps", move |state| {
            range_value_part((input_props_selector)(state), "rangeInputProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("valueNumber", move |state| {
            range_value_part((value_selector)(state), "valueNumber")
        });
        let min_selector = Arc::clone(&selector);
        self.use_prop_value("minValue", move |state| {
            range_value_part((min_selector)(state), "minValue")
        });
        let max_selector = Arc::clone(&selector);
        self.use_prop_value("maxValue", move |state| {
            range_value_part((max_selector)(state), "maxValue")
        });
        let step_selector = Arc::clone(&selector);
        self.use_prop_value("stepValue", move |state| {
            range_value_part((step_selector)(state), "stepValue")
        });
        let percent_selector = Arc::clone(&selector);
        self.use_prop_value("valuePercent", move |state| {
            range_value_part((percent_selector)(state), "valuePercent")
        });

        self.register_alias("rangeProps", BindingAlias::Props("rangeProps".to_string()));
        self.register_alias(
            "rangeInputProps",
            BindingAlias::Props("rangeInputProps".to_string()),
        );
        self.register_alias(
            "valueNumber",
            BindingAlias::Props("valueNumber".to_string()),
        );
        self.register_alias("minValue", BindingAlias::Props("minValue".to_string()));
        self.register_alias("maxValue", BindingAlias::Props("maxValue".to_string()));
        self.register_alias("stepValue", BindingAlias::Props("stepValue".to_string()));
        self.register_alias(
            "valuePercent",
            BindingAlias::Props("valuePercent".to_string()),
        );

        RangeHook {
            range_props: PropHandle::new("rangeProps"),
            range_input_props: PropHandle::new("rangeInputProps"),
            value_number: PropHandle::new("valueNumber"),
            min_value: PropHandle::new("minValue"),
            max_value: PropHandle::new("maxValue"),
            step_value: PropHandle::new("stepValue"),
            value_percent: PropHandle::new("valuePercent"),
        }
    }

    pub fn use_toast<F>(&mut self, selector: F) -> ToastHook
    where
        F: Fn(&S) -> UseToastProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToastProps + Send + Sync> = Arc::new(selector);
        for path in ["toastProps", "title", "description"] {
            self.use_semantic_part(path, &selector, toast_value_part);
        }
        self.register_prop_aliases(&["toastProps", "title", "description"]);

        ToastHook {
            toast_props: PropHandle::new("toastProps"),
            title: PropHandle::new("title"),
            description: PropHandle::new("description"),
        }
    }

    pub fn use_toast_region<F>(&mut self, selector: F) -> ToastRegionHook
    where
        F: Fn(&S) -> UseToastRegionProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToastRegionProps + Send + Sync> = Arc::new(selector);
        for path in ["toastRegionProps", "label"] {
            self.use_semantic_part(path, &selector, toast_region_value_part);
        }
        self.register_prop_aliases(&["toastRegionProps", "label"]);

        ToastRegionHook {
            toast_region_props: PropHandle::new("toastRegionProps"),
            label: PropHandle::new("label"),
        }
    }

    pub fn use_number_field<F>(&mut self, selector: F) -> NumberFieldHook
    where
        F: Fn(&S) -> UseNumberFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseNumberFieldProps + Send + Sync> = Arc::new(selector);
        for path in [
            "numberFieldProps",
            "numberFieldInputProps",
            "incrementButtonProps",
            "decrementButtonProps",
            "label",
            "valueNumber",
            "placeholder",
            "minValue",
            "maxValue",
            "stepValue",
            "formatOptions",
            "formatStyle",
            "valuePercent",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
            "isWheelDisabled",
            "canIncrement",
            "canDecrement",
        ] {
            self.use_semantic_part(path, &selector, number_field_value_part);
        }
        self.register_prop_aliases(&[
            "numberFieldProps",
            "numberFieldInputProps",
            "incrementButtonProps",
            "decrementButtonProps",
            "label",
            "valueNumber",
            "placeholder",
            "minValue",
            "maxValue",
            "stepValue",
            "formatOptions",
            "formatStyle",
            "valuePercent",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
            "isWheelDisabled",
            "canIncrement",
            "canDecrement",
        ]);

        NumberFieldHook {
            number_field_props: PropHandle::new("numberFieldProps"),
            number_field_input_props: PropHandle::new("numberFieldInputProps"),
            increment_button_props: PropHandle::new("incrementButtonProps"),
            decrement_button_props: PropHandle::new("decrementButtonProps"),
            label: PropHandle::new("label"),
            value_number: PropHandle::new("valueNumber"),
            placeholder: PropHandle::new("placeholder"),
            min_value: PropHandle::new("minValue"),
            max_value: PropHandle::new("maxValue"),
            step_value: PropHandle::new("stepValue"),
            format_options: PropHandle::new("formatOptions"),
            format_style: PropHandle::new("formatStyle"),
            value_percent: PropHandle::new("valuePercent"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
            is_wheel_disabled: PropHandle::new("isWheelDisabled"),
            can_increment: PropHandle::new("canIncrement"),
            can_decrement: PropHandle::new("canDecrement"),
        }
    }

    pub fn use_slider_track<F>(&mut self, selector: F) -> SliderTrackHook
    where
        F: Fn(&S) -> UseSliderTrackProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSliderTrackProps + Send + Sync> = Arc::new(selector);
        for path in ["sliderTrackProps", "orientation", "isDisabled"] {
            self.use_semantic_part(path, &selector, slider_track_value_part);
        }
        self.register_prop_aliases(&["sliderTrackProps", "orientation", "isDisabled"]);

        SliderTrackHook {
            slider_track_props: PropHandle::new("sliderTrackProps"),
            orientation: PropHandle::new("orientation"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_slider_fill<F>(&mut self, selector: F) -> SliderFillHook
    where
        F: Fn(&S) -> UseSliderFillProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSliderFillProps + Send + Sync> = Arc::new(selector);
        for path in [
            "sliderFillProps",
            "orientation",
            "valueNumber",
            "isDisabled",
        ] {
            self.use_semantic_part(path, &selector, slider_fill_value_part);
        }
        self.register_prop_aliases(&[
            "sliderFillProps",
            "orientation",
            "valueNumber",
            "isDisabled",
        ]);

        SliderFillHook {
            slider_fill_props: PropHandle::new("sliderFillProps"),
            orientation: PropHandle::new("orientation"),
            value_number: PropHandle::new("valueNumber"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_slider_output<F>(&mut self, selector: F) -> SliderOutputHook
    where
        F: Fn(&S) -> UseSliderOutputProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSliderOutputProps + Send + Sync> = Arc::new(selector);
        for path in ["sliderOutputProps", "label", "value", "valueNumber"] {
            self.use_semantic_part(path, &selector, slider_output_value_part);
        }
        self.register_prop_aliases(&["sliderOutputProps", "label", "value", "valueNumber"]);

        SliderOutputHook {
            slider_output_props: PropHandle::new("sliderOutputProps"),
            label: PropHandle::new("label"),
            value: PropHandle::new("value"),
            value_number: PropHandle::new("valueNumber"),
        }
    }

    pub fn use_toggle<F>(&mut self, selector: F) -> ToggleHook
    where
        F: Fn(&S) -> UseToggleProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToggleProps + Send + Sync> = Arc::new(selector);
        let props_selector = Arc::clone(&selector);
        self.use_prop_value("toggleProps", move |state| {
            toggle_value_part((props_selector)(state), "toggleProps")
        });
        let selected_selector = Arc::clone(&selector);
        self.use_prop_value("isSelected", move |state| {
            toggle_value_part((selected_selector)(state), "isSelected")
        });
        let checked_selector = Arc::clone(&selector);
        self.use_prop_value("isChecked", move |state| {
            toggle_value_part((checked_selector)(state), "isChecked")
        });

        self.register_alias(
            "toggleProps",
            BindingAlias::Props("toggleProps".to_string()),
        );
        self.register_alias("isSelected", BindingAlias::Props("isSelected".to_string()));
        self.register_alias("isChecked", BindingAlias::Props("isChecked".to_string()));

        ToggleHook {
            toggle_props: PropHandle::new("toggleProps"),
            is_selected: PropHandle::new("isSelected"),
            is_checked: PropHandle::new("isChecked"),
        }
    }

    pub fn use_switch<F>(&mut self, selector: F) -> SwitchHook
    where
        F: Fn(&S) -> UseSwitchProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseSwitchProps + Send + Sync> = Arc::new(selector);
        for path in [
            "switchProps",
            "isChecked",
            "isSelected",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, switch_value_part);
        }
        self.register_prop_aliases(&[
            "switchProps",
            "isChecked",
            "isSelected",
            "isDisabled",
            "isRequired",
            "isInvalid",
            "isReadOnly",
        ]);

        SwitchHook {
            switch_props: PropHandle::new("switchProps"),
            is_checked: PropHandle::new("isChecked"),
            is_selected: PropHandle::new("isSelected"),
            is_disabled: PropHandle::new("isDisabled"),
            is_required: PropHandle::new("isRequired"),
            is_invalid: PropHandle::new("isInvalid"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_toggle_button<F>(&mut self, selector: F) -> ToggleButtonHook
    where
        F: Fn(&S) -> UseToggleButtonProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToggleButtonProps + Send + Sync> = Arc::new(selector);
        for path in ["toggleButtonProps", "isSelected", "isPressed", "isDisabled"] {
            self.use_semantic_part(path, &selector, toggle_button_value_part);
        }
        self.register_prop_aliases(&["toggleButtonProps", "isSelected", "isPressed", "isDisabled"]);

        ToggleButtonHook {
            toggle_button_props: PropHandle::new("toggleButtonProps"),
            is_selected: PropHandle::new("isSelected"),
            is_pressed: PropHandle::new("isPressed"),
            is_disabled: PropHandle::new("isDisabled"),
        }
    }

    pub fn use_toggle_button_group<F>(&mut self, selector: F) -> ToggleButtonGroupHook
    where
        F: Fn(&S) -> UseToggleButtonGroupProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseToggleButtonGroupProps + Send + Sync> =
            Arc::new(selector);
        for path in [
            "toggleButtonGroupProps",
            "label",
            "selectedValue",
            "orientation",
            "selectionMode",
            "isDisabled",
            "isReadOnly",
        ] {
            self.use_semantic_part(path, &selector, toggle_button_group_value_part);
        }
        self.register_prop_aliases(&[
            "toggleButtonGroupProps",
            "label",
            "selectedValue",
            "orientation",
            "selectionMode",
            "isDisabled",
            "isReadOnly",
        ]);

        ToggleButtonGroupHook {
            toggle_button_group_props: PropHandle::new("toggleButtonGroupProps"),
            label: PropHandle::new("label"),
            selected_value: PropHandle::new("selectedValue"),
            orientation: PropHandle::new("orientation"),
            selection_mode: PropHandle::new("selectionMode"),
            is_disabled: PropHandle::new("isDisabled"),
            is_read_only: PropHandle::new("isReadOnly"),
        }
    }

    pub fn use_text_field<F>(&mut self, selector: F) -> TextFieldHook
    where
        F: Fn(&S) -> UseTextFieldProps + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> UseTextFieldProps + Send + Sync> = Arc::new(selector);
        let input_props_selector = Arc::clone(&selector);
        self.use_prop_value("inputProps", move |state| {
            text_field_value_part((input_props_selector)(state), "inputProps")
        });
        let field_props_selector = Arc::clone(&selector);
        self.use_prop_value("fieldProps", move |state| {
            text_field_value_part((field_props_selector)(state), "fieldProps")
        });
        let value_selector = Arc::clone(&selector);
        self.use_prop_value("value", move |state| {
            text_field_value_part((value_selector)(state), "value")
        });

        self.register_alias("inputProps", BindingAlias::Props("inputProps".to_string()));
        self.register_alias("fieldProps", BindingAlias::Props("fieldProps".to_string()));
        self.register_alias("value", BindingAlias::Props("value".to_string()));

        TextFieldHook {
            input_props: PropHandle::new("inputProps"),
            field_props: PropHandle::new("fieldProps"),
            value: PropHandle::new("value"),
        }
    }

    pub fn into_component(self, view: RSX) -> GuiResult<RsxComponent<S>> {
        self.into_component_with_defaults(view)
    }

    fn into_component_with_defaults(self, view: RSX) -> GuiResult<RsxComponent<S>> {
        let source = rewrite_registered_bindings(view.as_source(), &self.aliases);
        let mut component = RsxComponent::from_source(self.frame_id, "component-cx.rsx", &source)?;
        for registration in self.registrations {
            component = registration(component)?;
        }
        Ok(component)
    }

    pub fn into_component_bare(self, view: RSX) -> GuiResult<RsxComponent<S>> {
        self.into_component_with_registry(view, ComponentRegistry::new())
    }

    pub fn into_component_with_registry(
        self,
        view: RSX,
        registry: ComponentRegistry,
    ) -> GuiResult<RsxComponent<S>> {
        let source = rewrite_registered_bindings(view.as_source(), &self.aliases);
        let mut component = RsxComponent::from_source_with_registry(
            self.frame_id,
            "component-cx.rsx",
            &source,
            registry,
        )?;
        for registration in self.registrations {
            component = registration(component)?;
        }
        Ok(component)
    }

    fn use_prop_value<F>(&mut self, path: impl Into<String>, selector: F) -> PropHandle<JsonValue>
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        let path = path.into();
        self.register_alias(&path, BindingAlias::Props(path.clone()));
        let hook_path = path.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_prop_value(hook_path, selector))
        }));
        PropHandle::new(path)
    }

    fn use_semantic_part<P>(
        &mut self,
        path: &'static str,
        selector: &Arc<dyn Fn(&S) -> P + Send + Sync>,
        value_part: fn(P, &str) -> GuiResult<JsonValue>,
    ) -> PropHandle<JsonValue>
    where
        P: 'static,
    {
        let part_selector = Arc::clone(selector);
        self.use_prop_value(path, move |state| value_part((part_selector)(state), path))
    }

    fn register_prop_aliases(&mut self, paths: &[&str]) {
        for path in paths {
            self.register_alias(path, BindingAlias::Props((*path).to_string()));
        }
    }

    fn register_alias(&mut self, path: &str, alias: BindingAlias) {
        if let Some(name) = path.rsplit('.').next().filter(|name| !name.is_empty()) {
            self.aliases.entry(name.to_string()).or_insert(alias);
        }
    }
}

fn stable_hook_id(frame_id: &str, index: usize) -> String {
    let mut stem = String::new();
    for ch in frame_id.chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
            stem.push(ch);
        } else if !stem.ends_with('-') {
            stem.push('-');
        }
    }
    let stem = stem.trim_matches('-');
    if stem.is_empty() {
        format!("rsx-{index}")
    } else {
        format!("rsx-{stem}-{index}")
    }
}

fn press_value_part(props: UsePressProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_press_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn button_value_part(props: UseButtonProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_button_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn link_value_part(props: UseLinkProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_link_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn hover_value_part(props: UseHoverProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_hover_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn keyboard_interaction_value_part(
    props: UseKeyboardInteractionProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_keyboard_interaction_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn clipboard_value_part(props: UseClipboardProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_clipboard_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn long_press_value_part(props: UseLongPressProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_long_press_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn move_value_part(props: UseMoveProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_move_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn file_trigger_value_part(props: UseFileTriggerProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_file_trigger_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn drop_zone_value_part(props: UseDropZoneProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_drop_zone_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn drag_value_part(props: UseDragProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_drag_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn drop_value_part(props: UseDropProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_drop_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn group_value_part(props: UseGroupProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_group_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn virtualizer_value_part(props: UseVirtualizerProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_virtualizer_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn focusable_value_part(props: UseFocusableProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_focusable_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn focus_within_value_part(props: UseFocusWithinProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_focus_within_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn form_value_part(props: UseFormProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_form_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn breadcrumbs_value_part(props: UseBreadcrumbsProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_breadcrumbs_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn landmark_value_part(props: UseLandmarkProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_landmark_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn focus_ring_value_part(props: UseFocusRingProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_focus_ring_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn focus_scope_value_part(props: UseFocusScopeProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_focus_scope_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn field_value_part(props: UseFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn checkbox_value_part(props: UseCheckboxProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_checkbox_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn checkbox_group_value_part(props: UseCheckboxGroupProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_checkbox_group_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn separator_value_part(props: UseSeparatorProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_separator_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toolbar_value_part(props: UseToolbarProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_toolbar_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn drop_indicator_value_part(props: UseDropIndicatorProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_drop_indicator_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn selection_indicator_value_part(
    props: UseSelectionIndicatorProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_selection_indicator_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn i18n_value_part(props: UseI18nProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_i18n_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn collection_item_value_part(props: UseCollectionItemProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_collection_item_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn collection_value_part(props: UseCollectionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_collection_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn collection_section_value_part(
    props: UseCollectionSectionProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_collection_section_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn load_more_item_value_part(props: UseLoadMoreItemProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_load_more_item_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn menu_value_part(props: UseMenuProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_menu_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn menu_item_value_part(props: UseMenuItemProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_menu_item_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn submenu_trigger_value_part(props: UseSubmenuTriggerProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_submenu_trigger_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn radio_value_part(props: UseRadioProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_radio_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn radio_group_value_part(props: UseRadioGroupProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_radio_group_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tab_value_part(props: UseTabProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tab_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tab_list_value_part(props: UseTabListProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tab_list_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tab_panel_value_part(props: UseTabPanelProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tab_panel_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_value_part(props: UseTableProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_section_value_part(props: UseTableSectionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_section_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_row_value_part(props: UseTableRowProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_row_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_cell_value_part(props: UseTableCellProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_cell_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_column_value_part(props: UseTableColumnProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_column_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn table_caption_value_part(props: UseTableCaptionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_table_caption_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn text_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_text_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn label_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_label_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn description_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_description_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn field_error_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_field_error_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn legend_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_legend_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn visually_hidden_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_visually_hidden_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn keyboard_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_keyboard_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn list_box_header_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_list_box_header_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn grid_list_header_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_grid_list_header_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tree_header_value_part(props: UseTextProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tree_header_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn heading_value_part(props: UseHeadingProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_heading_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn date_field_value_part(props: UseDateFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_date_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn time_field_value_part(props: UseTimeFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_time_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn date_input_value_part(props: UseDateInputProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_date_input_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn date_segment_value_part(props: UseDateSegmentProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_date_segment_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn calendar_value_part(props: UseCalendarProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_calendar_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn range_calendar_value_part(props: UseRangeCalendarProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_range_calendar_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn calendar_cell_value_part(props: UseCalendarCellProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_calendar_cell_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn date_picker_value_part(props: UseDatePickerProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_date_picker_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn date_range_picker_value_part(
    props: UseDateRangePickerProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_date_range_picker_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_field_value_part(props: UseColorFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_picker_value_part(props: UseColorPickerProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_picker_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_area_value_part(props: UseColorAreaProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_area_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_slider_value_part(props: UseColorRangeProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_slider_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_wheel_value_part(props: UseColorRangeProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_wheel_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_swatch_picker_value_part(
    props: UseColorSwatchPickerProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_color_swatch_picker_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_swatch_picker_item_value_part(
    props: UseColorSwatchPickerItemProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_color_swatch_picker_item_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_swatch_value_part(props: UseColorSwatchProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_swatch_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn color_thumb_value_part(props: UseColorThumbProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_color_thumb_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn combo_box_value_part(props: UseComboBoxProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_combo_box_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn autocomplete_value_part(props: UseAutocompleteProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_autocomplete_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn select_value_part(props: UseSelectProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_select_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn select_display_value_part(props: UseSelectDisplayProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_select_display_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn combo_box_display_value_part(
    props: UseComboBoxDisplayProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_combo_box_display_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn overlay_value_part(props: UseOverlayProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_overlay_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn overlay_position_value_part(props: UseOverlayPositionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_overlay_position_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn selection_value_part(props: UseSelectionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_selection_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tree_value_part(props: UseTreeProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tree_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn tree_item_value_part(props: UseTreeItemProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_tree_item_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn disclosure_value_part(props: UseDisclosureProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_disclosure_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn disclosure_group_value_part(props: UseDisclosureGroupProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_disclosure_group_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn range_value_part(props: UseRangeProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_range_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toast_value_part(props: UseToastProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_toast_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toast_region_value_part(props: UseToastRegionProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_toast_region_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn number_field_value_part(props: UseNumberFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_number_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn slider_track_value_part(props: UseSliderTrackProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_slider_track_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn slider_fill_value_part(props: UseSliderFillProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_slider_fill_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn slider_output_value_part(props: UseSliderOutputProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_slider_output_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toggle_value_part(props: UseToggleProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_toggle_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn switch_value_part(props: UseSwitchProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_switch_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toggle_button_value_part(props: UseToggleButtonProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_toggle_button_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn toggle_button_group_value_part(
    props: UseToggleButtonGroupProps,
    part: &str,
) -> GuiResult<JsonValue> {
    let value = use_toggle_button_group_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn text_field_value_part(props: UseTextFieldProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_text_field_value(props)?;
    Ok(value.get(part).cloned().unwrap_or(JsonValue::Null))
}

fn rewrite_registered_bindings(source: &str, aliases: &BTreeMap<String, BindingAlias>) -> String {
    if aliases.is_empty() {
        return source.to_string();
    }

    let mut output = String::with_capacity(source.len());
    let mut index = 0;
    let mut quote = None;
    let chars = source.char_indices().collect::<Vec<_>>();
    let mut cursor = 0;

    while cursor < chars.len() {
        let (byte_index, ch) = chars[cursor];
        if let Some(quote_ch) = quote {
            output.push_str(&source[index..byte_index + ch.len_utf8()]);
            index = byte_index + ch.len_utf8();
            if ch == quote_ch {
                quote = None;
            }
            cursor += 1;
            continue;
        }

        match ch {
            '"' | '\'' | '`' => {
                output.push_str(&source[index..byte_index + ch.len_utf8()]);
                index = byte_index + ch.len_utf8();
                quote = Some(ch);
                cursor += 1;
            }
            '{' => {
                if let Some(end) = find_expression_end(source, byte_index) {
                    output.push_str(&source[index..byte_index]);
                    let expression = &source[byte_index + 1..end];
                    if let Some(rewritten) = rewrite_expression(expression, aliases) {
                        output.push('{');
                        output.push_str(&rewritten);
                        output.push('}');
                    } else {
                        output.push_str(&source[byte_index..=end]);
                    }
                    index = end + 1;
                    cursor = chars
                        .iter()
                        .position(|(position, _)| *position >= index)
                        .unwrap_or(chars.len());
                } else {
                    output.push_str(&source[index..byte_index + ch.len_utf8()]);
                    index = byte_index + ch.len_utf8();
                    cursor += 1;
                }
            }
            _ => {
                output.push_str(&source[index..byte_index + ch.len_utf8()]);
                index = byte_index + ch.len_utf8();
                cursor += 1;
            }
        }
    }

    output.push_str(&source[index..]);
    output
}

fn find_expression_end(source: &str, start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    let mut index = start;

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if let Some(quote_ch) = quote {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote_ch {
                quote = None;
            }
            index += ch.len_utf8();
            continue;
        }

        match ch {
            '"' | '\'' | '`' => quote = Some(ch),
            '{' => depth += 1,
            '}' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
        index += ch.len_utf8();
    }
    None
}

fn rewrite_expression(
    expression: &str,
    aliases: &BTreeMap<String, BindingAlias>,
) -> Option<String> {
    let trimmed = expression.trim();
    let (spread, path) = trimmed
        .strip_prefix("...")
        .map(|path| (true, path.trim()))
        .unwrap_or((false, trimmed));
    let segments = member_segments(path)?;
    let alias = aliases.get(segments.first()?)?;
    let rewritten = alias.resolve(&segments[1..])?;
    if spread {
        Some(format!("...{rewritten}"))
    } else {
        Some(rewritten)
    }
}

fn member_segments(expression: &str) -> Option<Vec<String>> {
    let segments = expression
        .split('.')
        .map(str::trim)
        .map(str::to_string)
        .collect::<Vec<_>>();
    if segments.is_empty() || segments.iter().any(|segment| !is_valid_identifier(segment)) {
        None
    } else {
        Some(segments)
    }
}

fn is_valid_identifier(value: &str) -> bool {
    let mut chars = value.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    (first == '_' || first.is_ascii_alphabetic())
        && chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
}

#[macro_export]
macro_rules! rsx {
    ($($tokens:tt)+) => {
        {
            $crate::__a3s_rsx_touch_handles!($($tokens)+);
            $crate::RSX::source(stringify!($($tokens)+))
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __a3s_rsx_touch_handles {
    () => {};
    ({ $handle:ident } $($rest:tt)*) => {
        let _ = &$handle;
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ state . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ props . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ derived . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ context . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ resource . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ $handle:ident . $($inner:tt)+ } $($rest:tt)*) => {
        let _ = &$handle;
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... state . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... props . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... derived . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... context . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... resource . $($inner:tt)+ } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... $handle:ident . $($inner:tt)+ } $($rest:tt)*) => {
        let _ = &$handle;
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ ... $handle:ident } $($rest:tt)*) => {
        let _ = &$handle;
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ({ $($inner:tt)* } $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
    ($token:tt $($rest:tt)*) => {
        $crate::__a3s_rsx_touch_handles!($($rest)*);
    };
}
