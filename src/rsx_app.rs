use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::{Arc, Mutex};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::app::NativeRuntimeApp;
use crate::compiler::{
    CompiledBinding, CompiledBindingSource, CompiledProps, CompiledRsxNode, ComponentClassVariants,
};
use crate::error::{GuiError, GuiResult};
use crate::event::ActionInvocation;
use crate::host::NativeHost;
use crate::platform::PlatformAdapter;
use crate::protocol::{NativeProtocolApp, UiAction, UiFrame, WindowOptions};

mod component_cx;
mod hooks;

pub use component_cx::{
    ActionHandle, ActionStateHandle, ActionStateSnapshot, AutocompleteHook, BreadcrumbsHook,
    ButtonHook, CalendarCellHook, CalendarHook, CheckboxGroupHook, ClipboardHook, CollectionHook,
    CollectionItemHook, CollectionSectionHook, ColorAreaHook, ColorFieldHook, ColorPickerHook,
    ColorSliderHook, ColorSwatchHook, ColorSwatchPickerHook, ColorSwatchPickerItemHook,
    ColorThumbHook, ColorWheelHook, ComboBoxDisplayHook, ComboBoxHook, ComponentCx, ContextHandle,
    DateFieldHook, DateInputHook, DatePickerHook, DateRangePickerHook, DateSegmentHook,
    DerivedHandle, DisclosureGroupHook, DisclosureHook, DragHook, DropHook, DropIndicatorHook,
    DropZoneHook, EffectEventHandle, FieldErrorHook, FieldHook, FileTriggerHook, FocusRingHook,
    FocusScopeHook, FocusWithinHook, FocusableHook, FormHook, FormStatusSnapshot,
    GridListHeaderHook, GroupHook, HeadingHook, HoverHook, I18nHook, KeyboardHook,
    KeyboardInteractionHook, LabelHook, LegendHook, LinkHook, ListBoxHeaderHook, LoadMoreItemHook,
    LongPressHook, MenuHook, MenuItemHook, MoveHook, NumberFieldHook, OptimisticHandle,
    OverlayHook, PressHook, PropHandle, RadioGroupHook, RadioHook, RangeCalendarHook, RangeHook,
    ReactiveHandle, RefHandle, ResourceHandle, SelectDisplayHook, SelectHook, SelectionHook,
    SelectionIndicatorHook, SelectorHandle, SeparatorHook, SliderFillHook, SliderOutputHook,
    SliderTrackHook, StateHandle, SyncExternalStore, SyncExternalStoreSubscription, TabHook,
    TabListHook, TabPanelHook, TableCaptionHook, TableCellHook, TableColumnHook, TableHook,
    TableRowHook, TableSectionHook, TextFieldHook, TextHook, TimeFieldHook, ToggleButtonGroupHook,
    ToggleButtonHook, ToggleHook, ToolbarHook, TreeHeaderHook, TreeHook, TreeItemHook,
    VirtualizerHook, VisuallyHiddenHook, RSX,
};

use hooks::{
    decode_action_payload, decode_action_value, insert_scope_value, RsxActionDisabledHook,
    RsxActionHook, RsxDebugValueHook, RsxEffectHook, RsxMountHook, RsxRenderEffectHook,
    RsxRenderEffectRuntime, RsxRouteContextHook, RsxRouteEffectHook, RsxUnmountHook, RsxValueHook,
};

#[derive(Debug, Clone)]
pub struct RsxTemplate {
    root: CompiledRsxNode,
    source_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RsxComponentContract {
    required_props: BTreeSet<String>,
    optional_props: BTreeSet<String>,
    default_props: BTreeMap<String, JsonValue>,
    allow_extra_props: bool,
}

/// A reusable set of compiled RSX component definitions.
///
/// Cloning a registry is inexpensive: its templates, contracts, and class
/// variants are held in shared immutable maps. A component only detaches the
/// map it needs when an application-specific definition is registered.
#[derive(Debug, Clone, Default)]
pub struct ComponentRegistry {
    templates: Arc<BTreeMap<String, CompiledRsxNode>>,
    contracts: Arc<BTreeMap<String, RsxComponentContract>>,
    variants: Arc<BTreeMap<String, ComponentClassVariants>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn len(&self) -> usize {
        self.templates.len()
    }

    pub fn is_empty(&self) -> bool {
        self.templates.is_empty()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.templates.contains_key(name)
    }

    fn templates(&self) -> &BTreeMap<String, CompiledRsxNode> {
        &self.templates
    }

    fn contracts(&self) -> &BTreeMap<String, RsxComponentContract> {
        &self.contracts
    }

    fn variants(&self) -> &BTreeMap<String, ComponentClassVariants> {
        &self.variants
    }

    fn templates_mut(&mut self) -> &mut BTreeMap<String, CompiledRsxNode> {
        Arc::make_mut(&mut self.templates)
    }

    fn contracts_mut(&mut self) -> &mut BTreeMap<String, RsxComponentContract> {
        Arc::make_mut(&mut self.contracts)
    }

    fn variants_mut(&mut self) -> &mut BTreeMap<String, ComponentClassVariants> {
        Arc::make_mut(&mut self.variants)
    }

    #[cfg(test)]
    pub(crate) fn shares_compiled_definitions_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.templates, &other.templates)
            && Arc::ptr_eq(&self.contracts, &other.contracts)
            && Arc::ptr_eq(&self.variants, &other.variants)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RsxResource<T> {
    Idle,
    Loading,
    Ready(T),
    Failed(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RsxDebugValue {
    pub label: String,
    pub value: JsonValue,
}

impl<T> RsxResource<T> {
    pub fn idle() -> Self {
        Self::Idle
    }

    pub fn loading() -> Self {
        Self::Loading
    }

    pub fn ready(data: T) -> Self {
        Self::Ready(data)
    }

    pub fn failed(error: impl Into<String>) -> Self {
        Self::Failed(error.into())
    }

    pub fn error(error: impl Into<String>) -> Self {
        Self::failed(error)
    }
}

impl<T: Serialize> RsxResource<T> {
    fn into_scope_value(self, path: &str) -> GuiResult<JsonValue> {
        let mut scope = JsonMap::new();
        let (status, data, error) = match self {
            Self::Idle => ("idle", JsonValue::Null, None),
            Self::Loading => ("loading", JsonValue::Null, None),
            Self::Ready(data) => {
                let data = serde_json::to_value(data).map_err(|error| {
                    GuiError::invalid_tree(format!(
                        "RSX resource hook {path:?} data did not serialize: {error}"
                    ))
                })?;
                ("ready", data, None)
            }
            Self::Failed(error) => ("error", JsonValue::Null, Some(error)),
        };
        let is_idle = status == "idle";
        let is_loading = status == "loading";
        let is_ready = status == "ready";
        let is_error = status == "error";
        let error = error.map(JsonValue::String).unwrap_or(JsonValue::Null);

        scope.insert("status".to_string(), JsonValue::String(status.to_string()));
        scope.insert("data".to_string(), data);
        scope.insert("error".to_string(), error.clone());
        scope.insert("message".to_string(), error);
        scope.insert("isIdle".to_string(), JsonValue::Bool(is_idle));
        scope.insert("isLoading".to_string(), JsonValue::Bool(is_loading));
        scope.insert("isReady".to_string(), JsonValue::Bool(is_ready));
        scope.insert("isError".to_string(), JsonValue::Bool(is_error));
        scope.insert("hasData".to_string(), JsonValue::Bool(is_ready));
        scope.insert("hasError".to_string(), JsonValue::Bool(is_error));
        Ok(JsonValue::Object(scope))
    }
}

impl Default for RsxComponentContract {
    fn default() -> Self {
        Self::new()
    }
}

impl RsxComponentContract {
    pub fn new() -> Self {
        Self {
            required_props: BTreeSet::new(),
            optional_props: BTreeSet::new(),
            default_props: BTreeMap::new(),
            allow_extra_props: false,
        }
    }

    pub fn open() -> Self {
        Self {
            required_props: BTreeSet::new(),
            optional_props: BTreeSet::new(),
            default_props: BTreeMap::new(),
            allow_extra_props: true,
        }
    }

    pub fn required<I, P>(mut self, props: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<String>,
    {
        self.required_props.extend(
            props
                .into_iter()
                .map(Into::into)
                .map(canonical_component_prop_name),
        );
        self
    }

    pub fn optional<I, P>(mut self, props: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<String>,
    {
        self.optional_props.extend(
            props
                .into_iter()
                .map(Into::into)
                .map(canonical_component_prop_name),
        );
        self
    }

    pub fn allow_extra_props(mut self) -> Self {
        self.allow_extra_props = true;
        self
    }

    pub fn default_prop<T: Serialize>(self, prop: impl Into<String>, value: T) -> GuiResult<Self> {
        let value = serde_json::to_value(value).map_err(|error| {
            GuiError::invalid_tree(format!(
                "RSX component default prop did not serialize: {error}"
            ))
        })?;
        self.default_prop_value(prop, value)
    }

    pub fn default_prop_value(
        mut self,
        prop: impl Into<String>,
        value: JsonValue,
    ) -> GuiResult<Self> {
        let prop = canonical_component_prop_name(prop.into());
        validate_component_prop_name("contract", &prop)?;
        self.required_props.remove(&prop);
        self.optional_props.insert(prop.clone());
        self.default_props.insert(prop, value);
        Ok(self)
    }

    fn validate(&self, component: &str) -> GuiResult<()> {
        for prop in self
            .required_props
            .iter()
            .chain(self.optional_props.iter())
            .chain(self.default_props.keys())
        {
            validate_component_prop_name(component, prop)?;
        }
        if let Some(prop) = self
            .required_props
            .intersection(&self.optional_props)
            .next()
        {
            return Err(GuiError::invalid_tree(format!(
                "RSX component {component:?} prop {prop:?} cannot be both required and optional"
            )));
        }
        let mut props = CompiledProps::default();
        props.apply_default_props(&self.default_props)?;
        Ok(())
    }

    fn validate_invocation(
        &self,
        component: &str,
        invocation_props: &CompiledProps,
    ) -> GuiResult<()> {
        self.validate(component)?;
        let supplied = component_invocation_prop_names(invocation_props);
        for prop in &self.required_props {
            if !supplied.contains(prop) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX component {component:?} requires prop {prop:?}"
                )));
            }
        }

        if self.allow_extra_props {
            return Ok(());
        }

        if !invocation_props.spreads.is_empty() {
            return Err(GuiError::invalid_tree(format!(
                "RSX component {component:?} uses spread props, but its prop contract is closed"
            )));
        }

        let allowed = self
            .required_props
            .union(&self.optional_props)
            .cloned()
            .collect::<BTreeSet<_>>();
        for prop in supplied {
            if prop == "slot" {
                continue;
            }
            if !allowed.contains(&prop) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX component {component:?} received unknown prop {prop:?}"
                )));
            }
        }
        Ok(())
    }

    fn covers_binding_path(&self, path: &[String]) -> bool {
        let default_props = self.default_props.keys().cloned().collect();
        prop_set_covers_path(&self.required_props, path)
            || prop_set_covers_path(&self.optional_props, path)
            || prop_set_covers_path(&default_props, path)
    }

    fn default_props(&self) -> &BTreeMap<String, JsonValue> {
        &self.default_props
    }
}

impl RsxTemplate {
    pub fn parse(source: &str) -> GuiResult<Self> {
        Self::parse_source("inline.rsx", source)
    }

    pub fn parse_source(source_name: impl AsRef<str>, source: &str) -> GuiResult<Self> {
        let source_name = normalize_rsx_source_name(source_name);
        Self::from_named_root(
            source_name.clone(),
            crate::rsx::parse_rsx_source(&source_name, source)?,
        )
    }

    pub fn from_file(path: impl AsRef<Path>) -> GuiResult<Self> {
        let path = path.as_ref();
        let source = std::fs::read_to_string(path).map_err(|error| {
            GuiError::invalid_tree(format!(
                "failed to read RSX template {:?}: {error}",
                path.display()
            ))
        })?;
        Self::parse_source(path.display().to_string(), &source)
    }

    pub fn from_root(root: CompiledRsxNode) -> GuiResult<Self> {
        root.validate()?;
        Ok(Self {
            root,
            source_name: None,
        })
    }

    pub fn from_named_root(source_name: impl AsRef<str>, root: CompiledRsxNode) -> GuiResult<Self> {
        let source_name = normalize_rsx_source_name(source_name);
        root.validate()?;
        Ok(Self {
            root,
            source_name: Some(source_name),
        })
    }

    pub fn root(&self) -> &CompiledRsxNode {
        &self.root
    }

    pub fn source_name(&self) -> Option<&str> {
        self.source_name.as_deref()
    }

    pub fn render_with_state(
        &self,
        frame_id: impl Into<String>,
        state: &JsonValue,
    ) -> GuiResult<UiFrame> {
        let mut scope = JsonMap::new();
        scope.insert("state".to_string(), state.clone());
        scope.insert("props".to_string(), JsonValue::Object(JsonMap::new()));
        scope.insert("derived".to_string(), JsonValue::Object(JsonMap::new()));
        scope.insert("context".to_string(), JsonValue::Object(JsonMap::new()));
        scope.insert("resource".to_string(), JsonValue::Object(JsonMap::new()));
        self.render_with_scope(frame_id, &JsonValue::Object(scope))
    }

    pub fn render_with_scope(
        &self,
        frame_id: impl Into<String>,
        scope: &JsonValue,
    ) -> GuiResult<UiFrame> {
        self.render_with_scope_parts(frame_id, scope, None, None)
    }

    pub fn render_with_scope_parts(
        &self,
        frame_id: impl Into<String>,
        scope: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
    ) -> GuiResult<UiFrame> {
        self.render_with_scope_parts_and_components(
            frame_id,
            scope,
            actions,
            window,
            &BTreeMap::new(),
        )
    }

    pub fn render_with_scope_parts_and_components(
        &self,
        frame_id: impl Into<String>,
        scope: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
        components: &BTreeMap<String, CompiledRsxNode>,
    ) -> GuiResult<UiFrame> {
        self.render_with_scope_parts_and_component_defaults(
            frame_id,
            scope,
            actions,
            window,
            components,
            &BTreeMap::new(),
        )
    }

    pub fn render_with_scope_parts_and_component_defaults(
        &self,
        frame_id: impl Into<String>,
        scope: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
    ) -> GuiResult<UiFrame> {
        self.render_with_scope_parts_and_component_options(
            frame_id,
            scope,
            actions,
            window,
            components,
            component_defaults,
            &BTreeMap::new(),
        )
    }

    pub fn render_with_scope_parts_and_component_options(
        &self,
        frame_id: impl Into<String>,
        scope: &JsonValue,
        actions: Option<Vec<UiAction>>,
        window: Option<WindowOptions>,
        components: &BTreeMap<String, CompiledRsxNode>,
        component_defaults: &BTreeMap<String, BTreeMap<String, JsonValue>>,
        component_variants: &BTreeMap<String, ComponentClassVariants>,
    ) -> GuiResult<UiFrame> {
        let root = self
            .root
            .resolve_bindings_with_component_options(
                scope,
                components,
                component_defaults,
                component_variants,
            )
            .map_err(|error| self.with_source_context(error))?;
        UiFrame::from_compiled_parts(frame_id, root, actions, window)
            .map_err(|error| self.with_source_context(error))
    }

    fn with_source_context(&self, error: GuiError) -> GuiError {
        with_optional_source_context(self.source_name.as_deref(), error)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RsxActionTransition<'a, S> {
    before: &'a S,
    invocation: &'a ActionInvocation,
}

impl<'a, S> RsxActionTransition<'a, S> {
    pub fn new(before: &'a S, invocation: &'a ActionInvocation) -> Self {
        Self { before, invocation }
    }

    pub fn before(&self) -> &'a S {
        self.before
    }

    pub fn invocation(&self) -> &'a ActionInvocation {
        self.invocation
    }

    pub fn action(&self) -> &'a str {
        &self.invocation.action
    }

    pub fn value(&self) -> Option<&'a str> {
        self.invocation.value()
    }

    pub fn payload_json(&self) -> GuiResult<Option<JsonValue>> {
        self.invocation.payload_json()
    }

    pub fn payload<T>(&self) -> GuiResult<Option<T>>
    where
        T: DeserializeOwned,
    {
        self.invocation.payload()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RsxRouteTransition<'a> {
    from: &'a str,
    to: &'a str,
    invocation: &'a ActionInvocation,
}

impl<'a> RsxRouteTransition<'a> {
    pub fn new(from: &'a str, to: &'a str, invocation: &'a ActionInvocation) -> Self {
        Self {
            from,
            to,
            invocation,
        }
    }

    pub fn from(&self) -> &'a str {
        self.from
    }

    pub fn to(&self) -> &'a str {
        self.to
    }

    pub fn invocation(&self) -> &'a ActionInvocation {
        self.invocation
    }

    pub fn action(&self) -> &'a str {
        &self.invocation.action
    }

    pub fn value(&self) -> Option<&'a str> {
        self.invocation.value()
    }
}

pub struct RsxComponent<S> {
    frame_id: String,
    template: RsxTemplate,
    state_hooks: Vec<RsxValueHook<S>>,
    prop_hooks: Vec<RsxValueHook<S>>,
    derived_hooks: Vec<RsxValueHook<S>>,
    context_hooks: Vec<RsxValueHook<S>>,
    resource_hooks: Vec<RsxValueHook<S>>,
    component_registry: ComponentRegistry,
    action_hooks: BTreeMap<String, RsxActionHook<S>>,
    duplicate_actions: BTreeSet<String>,
    action_disabled_hooks: BTreeMap<String, RsxActionDisabledHook<S>>,
    duplicate_action_disabled_hooks: BTreeSet<String>,
    mount_hooks: Vec<RsxMountHook<S>>,
    unmount_hooks: Vec<RsxUnmountHook<S>>,
    action_effect_hooks: Vec<RsxEffectHook<S>>,
    render_effect_hooks: Vec<RsxRenderEffectHook<S>>,
    debug_hooks: Vec<RsxDebugValueHook<S>>,
    window: Option<WindowOptions>,
}

pub struct RsxRouter<S> {
    layout: Option<RsxComponent<S>>,
    routes: BTreeMap<String, RsxComponent<S>>,
    route_selector: Box<dyn Fn(&S) -> GuiResult<String> + Send + Sync>,
    default_route: Option<String>,
    route_context_hooks: Vec<RsxRouteContextHook<S>>,
    route_effect_hooks: Vec<RsxRouteEffectHook<S>>,
}

struct RsxRouterRenderEffectRuntime<S> {
    layout: Option<RsxRenderEffectRuntime<S>>,
    routes: BTreeMap<String, RsxRenderEffectRuntime<S>>,
    active_route: Option<String>,
}

impl<S> RsxComponent<S> {
    pub fn new(frame_id: impl Into<String>, source: &str) -> GuiResult<Self> {
        Self::from_template_with_default_components(frame_id, RsxTemplate::parse(source)?)
    }

    /// Creates an RSX component without installing the default design-system
    /// components.
    pub fn new_bare(frame_id: impl Into<String>, source: &str) -> GuiResult<Self> {
        Ok(Self::from_template_bare(
            frame_id,
            RsxTemplate::parse(source)?,
        ))
    }

    /// Creates an RSX component with an explicitly supplied component registry.
    pub fn new_with_registry(
        frame_id: impl Into<String>,
        source: &str,
        registry: ComponentRegistry,
    ) -> GuiResult<Self> {
        Ok(Self::from_template_with_registry(
            frame_id,
            RsxTemplate::parse(source)?,
            registry,
        ))
    }

    pub fn from_source(
        frame_id: impl Into<String>,
        source_name: impl AsRef<str>,
        source: &str,
    ) -> GuiResult<Self> {
        Self::from_template_with_default_components(
            frame_id,
            RsxTemplate::parse_source(source_name, source)?,
        )
    }

    pub fn from_source_bare(
        frame_id: impl Into<String>,
        source_name: impl AsRef<str>,
        source: &str,
    ) -> GuiResult<Self> {
        Ok(Self::from_template_bare(
            frame_id,
            RsxTemplate::parse_source(source_name, source)?,
        ))
    }

    pub fn from_source_with_registry(
        frame_id: impl Into<String>,
        source_name: impl AsRef<str>,
        source: &str,
        registry: ComponentRegistry,
    ) -> GuiResult<Self> {
        Ok(Self::from_template_with_registry(
            frame_id,
            RsxTemplate::parse_source(source_name, source)?,
            registry,
        ))
    }

    pub fn from_file(frame_id: impl Into<String>, path: impl AsRef<Path>) -> GuiResult<Self> {
        Self::from_template_with_default_components(frame_id, RsxTemplate::from_file(path)?)
    }

    pub fn from_file_bare(frame_id: impl Into<String>, path: impl AsRef<Path>) -> GuiResult<Self> {
        Ok(Self::from_template_bare(
            frame_id,
            RsxTemplate::from_file(path)?,
        ))
    }

    pub fn from_file_with_registry(
        frame_id: impl Into<String>,
        path: impl AsRef<Path>,
        registry: ComponentRegistry,
    ) -> GuiResult<Self> {
        Ok(Self::from_template_with_registry(
            frame_id,
            RsxTemplate::from_file(path)?,
            registry,
        ))
    }

    pub fn from_template(frame_id: impl Into<String>, template: RsxTemplate) -> GuiResult<Self> {
        Self::from_template_with_default_components(frame_id, template)
    }

    pub fn from_template_bare(frame_id: impl Into<String>, template: RsxTemplate) -> Self {
        Self::from_template_with_registry(frame_id, template, ComponentRegistry::new())
    }

    pub fn from_template_with_registry(
        frame_id: impl Into<String>,
        template: RsxTemplate,
        component_registry: ComponentRegistry,
    ) -> Self {
        Self {
            frame_id: frame_id.into(),
            template,
            state_hooks: Vec::new(),
            prop_hooks: Vec::new(),
            derived_hooks: Vec::new(),
            context_hooks: Vec::new(),
            resource_hooks: Vec::new(),
            component_registry,
            action_hooks: BTreeMap::new(),
            duplicate_actions: BTreeSet::new(),
            action_disabled_hooks: BTreeMap::new(),
            duplicate_action_disabled_hooks: BTreeSet::new(),
            mount_hooks: Vec::new(),
            unmount_hooks: Vec::new(),
            action_effect_hooks: Vec::new(),
            render_effect_hooks: Vec::new(),
            debug_hooks: Vec::new(),
            window: None,
        }
    }

    fn from_template_with_default_components(
        frame_id: impl Into<String>,
        template: RsxTemplate,
    ) -> GuiResult<Self> {
        Ok(Self::from_template_with_registry(
            frame_id,
            template,
            crate::default_components::registry()?,
        ))
    }

    pub fn template(&self) -> &RsxTemplate {
        &self.template
    }

    pub fn component_registry(&self) -> &ComponentRegistry {
        &self.component_registry
    }

    #[cfg(feature = "design-system")]
    pub(crate) fn into_component_registry(self) -> ComponentRegistry {
        self.component_registry
    }

    pub fn with_window(mut self, window: WindowOptions) -> Self {
        self.window = Some(window);
        self
    }

    pub fn register<F>(self, registration: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        registration(self)
    }

    pub fn try_register<F>(self, registration: F) -> GuiResult<Self>
    where
        F: FnOnce(Self) -> GuiResult<Self>,
    {
        registration(self)
    }

    pub fn use_hooks<F>(self, hooks: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        hooks(self)
    }

    pub fn try_use_hooks<F>(self, hooks: F) -> GuiResult<Self>
    where
        F: FnOnce(Self) -> GuiResult<Self>,
    {
        hooks(self)
    }

    pub fn use_component(mut self, name: impl Into<String>, source: &str) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        let template = RsxTemplate::parse(source)?;
        self.component_registry
            .templates_mut()
            .insert(name, template.root().clone());
        Ok(self)
    }

    pub fn use_component_source(
        mut self,
        name: impl Into<String>,
        source_name: impl AsRef<str>,
        source: &str,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        let template = RsxTemplate::parse_source(source_name, source)?;
        self.component_registry
            .templates_mut()
            .insert(name, template.root().clone());
        Ok(self)
    }

    pub fn use_component_file(
        self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
    ) -> GuiResult<Self> {
        self.use_template_component(name, RsxTemplate::from_file(path)?)
    }

    pub fn use_component_with_contract(
        mut self,
        name: impl Into<String>,
        source: &str,
        contract: RsxComponentContract,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        contract.validate(&name)?;
        let template = RsxTemplate::parse(source)?;
        self.component_registry
            .templates_mut()
            .insert(name.clone(), template.root().clone());
        self.component_registry
            .contracts_mut()
            .insert(name, contract);
        Ok(self)
    }

    pub fn use_component_source_with_contract(
        mut self,
        name: impl Into<String>,
        source_name: impl AsRef<str>,
        source: &str,
        contract: RsxComponentContract,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        contract.validate(&name)?;
        let template = RsxTemplate::parse_source(source_name, source)?;
        self.component_registry
            .templates_mut()
            .insert(name.clone(), template.root().clone());
        self.component_registry
            .contracts_mut()
            .insert(name, contract);
        Ok(self)
    }

    pub fn use_component_file_with_contract(
        self,
        name: impl Into<String>,
        path: impl AsRef<Path>,
        contract: RsxComponentContract,
    ) -> GuiResult<Self> {
        self.use_template_component_with_contract(name, RsxTemplate::from_file(path)?, contract)
    }

    pub fn use_template_component(
        mut self,
        name: impl Into<String>,
        template: RsxTemplate,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        self.component_registry
            .templates_mut()
            .insert(name, template.root().clone());
        Ok(self)
    }

    pub fn use_template_component_with_contract(
        mut self,
        name: impl Into<String>,
        template: RsxTemplate,
        contract: RsxComponentContract,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        self.validate_new_component_name(&name)?;
        contract.validate(&name)?;
        self.component_registry
            .templates_mut()
            .insert(name.clone(), template.root().clone());
        self.component_registry
            .contracts_mut()
            .insert(name, contract);
        Ok(self)
    }

    pub fn use_component_class_variants(
        mut self,
        name: impl Into<String>,
        variants: ComponentClassVariants,
    ) -> GuiResult<Self> {
        let name = name.into();
        validate_component_name(&name)?;
        if !self.component_registry.templates().contains_key(&name) {
            return Err(GuiError::invalid_tree(format!(
                "RSX component {name:?} needs a registered template before class variants can be attached"
            )));
        }
        if self.component_registry.variants().contains_key(&name) {
            return Err(GuiError::invalid_tree(format!(
                "RSX component {name:?} class variants were registered more than once"
            )));
        }
        self.component_registry
            .variants_mut()
            .insert(name, variants);
        Ok(self)
    }

    pub fn use_state<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.state_hooks
            .push(RsxValueHook::serializing("state", path, selector));
        self
    }

    pub fn use_state_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.state_hooks
            .push(RsxValueHook::serializing_result("state", path, selector));
        self
    }

    pub fn use_state_value<F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.state_hooks.push(RsxValueHook::value(path, selector));
        self
    }

    pub fn use_selector<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_state(path, selector)
    }

    pub fn use_selector_result<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.use_state_result(path, selector)
    }

    pub fn use_selector_value<F>(self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.use_state_value(path, selector)
    }

    pub fn use_prop<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.prop_hooks
            .push(RsxValueHook::serializing("prop", path, selector));
        self
    }

    pub fn use_prop_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.prop_hooks
            .push(RsxValueHook::serializing_result("prop", path, selector));
        self
    }

    pub fn use_prop_value<F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.prop_hooks.push(RsxValueHook::value(path, selector));
        self
    }

    pub fn use_derived<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.derived_hooks
            .push(RsxValueHook::serializing("derived", path, selector));
        self
    }

    pub fn use_derived_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.derived_hooks
            .push(RsxValueHook::serializing_result("derived", path, selector));
        self
    }

    pub fn use_derived_value<F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.derived_hooks.push(RsxValueHook::value(path, selector));
        self
    }

    pub fn use_context<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.context_hooks
            .push(RsxValueHook::serializing("context", path, selector));
        self
    }

    pub fn use_context_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.context_hooks
            .push(RsxValueHook::serializing_result("context", path, selector));
        self
    }

    pub fn use_context_value<F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.context_hooks.push(RsxValueHook::value(path, selector));
        self
    }

    pub fn use_resource<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> RsxResource<T> + Send + Sync + 'static,
    {
        self.use_resource_result(path, move |state| Ok(selector(state)))
    }

    pub fn use_resource_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<RsxResource<T>> + Send + Sync + 'static,
    {
        let path = path.into();
        let context = path.clone();
        self.resource_hooks
            .push(RsxValueHook::value(path, move |state| {
                selector(state)?.into_scope_value(&context)
            }));
        self
    }

    pub fn use_memo<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_derived(path, selector)
    }

    pub fn use_memo_result<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.use_derived_result(path, selector)
    }

    pub fn use_memo_value<F>(self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.use_derived_value(path, selector)
    }

    pub fn use_sync_external_store<T>(
        self,
        path: impl Into<String>,
        store: SyncExternalStore<T>,
    ) -> Self
    where
        T: Clone + Send + Serialize + 'static,
    {
        self.use_derived_result(path, move |_state| store.snapshot())
    }

    pub fn use_deferred_value<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        S: 'static,
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_deferred_value_result(path, move |state| Ok(selector(state)))
    }

    pub fn use_deferred_value_result<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        S: 'static,
        T: Clone + Send + Serialize + 'static,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let selector: Arc<dyn Fn(&S) -> GuiResult<T> + Send + Sync> = Arc::new(selector);
        let deferred_value = Arc::new(Mutex::new(None::<T>));
        let render_selector = Arc::clone(&selector);
        let render_deferred_value = Arc::clone(&deferred_value);
        let effect_selector = Arc::clone(&selector);
        let effect_deferred_value = Arc::clone(&deferred_value);
        self.use_derived_result(path, move |state| {
            let current = render_selector(state)?;
            let deferred = render_deferred_value.lock().map_err(|_| {
                GuiError::invalid_tree("RSX deferred value lock was poisoned while reading")
            })?;
            Ok(deferred.clone().unwrap_or(current))
        })
        .use_effect(move |state| {
            let current = effect_selector(state)?;
            let mut deferred = effect_deferred_value.lock().map_err(|_| {
                GuiError::invalid_tree("RSX deferred value lock was poisoned while writing")
            })?;
            *deferred = Some(current);
            Ok(())
        })
    }

    pub fn use_debug_value<T, F>(mut self, label: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.debug_hooks
            .push(RsxDebugValueHook::serializing(label, selector));
        self
    }

    pub fn use_debug_value_result<T, F>(mut self, label: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.debug_hooks
            .push(RsxDebugValueHook::serializing_result(label, selector));
        self
    }

    pub fn use_reactive<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        self.use_state(path, selector)
    }

    pub fn use_reactive_result<T, F>(self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.use_state_result(path, selector)
    }

    pub fn use_reactive_value<F>(self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.use_state_value(path, selector)
    }

    pub fn use_field<T, F, R>(
        self,
        path: impl Into<String>,
        action: impl Into<String>,
        selector: F,
        reducer: R,
    ) -> Self
    where
        T: DeserializeOwned + Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_field(path, action, None::<String>, selector, reducer)
    }

    pub fn use_labeled_field<T, F, R>(
        self,
        path: impl Into<String>,
        action: impl Into<String>,
        label: Option<impl Into<String>>,
        selector: F,
        reducer: R,
    ) -> Self
    where
        T: DeserializeOwned + Serialize + 'static,
        F: Fn(&S) -> T + Send + Sync + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_state::<T, F>(path, selector)
            .use_labeled_value_reducer::<T, R>(action, label, reducer)
    }

    pub fn use_mount<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut S) + Send + Sync + 'static,
    {
        self.mount_hooks.push(RsxMountHook::new(hook));
        self
    }

    pub fn use_mount_result<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.mount_hooks.push(RsxMountHook::result(hook));
        self
    }

    pub fn use_unmount<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut S) + Send + Sync + 'static,
    {
        self.unmount_hooks.push(RsxUnmountHook::new(hook));
        self
    }

    pub fn use_unmount_result<F>(mut self, hook: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.unmount_hooks.push(RsxUnmountHook::result(hook));
        self
    }

    pub fn use_effect<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::always(effect));
        self
    }

    pub fn use_effect_once<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::once(effect));
        self
    }

    pub fn use_effect_with_deps<T, D, F>(mut self, deps: D, effect: F) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::with_deps::<T, F, D>(deps, effect));
        self
    }

    pub fn use_effect_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::always_with_cleanup(effect));
        self
    }

    pub fn use_effect_once_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::once_with_cleanup(effect));
        self
    }

    pub fn use_effect_with_deps_and_cleanup<T, D, C, F>(mut self, deps: D, effect: F) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::with_deps_and_cleanup::<T, F, D, C>(
                deps, effect,
            ));
        self
    }

    pub fn use_layout_effect<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout(effect));
        self
    }

    pub fn use_layout_effect_once<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout_once(effect));
        self
    }

    pub fn use_layout_effect_with_deps<T, D, F>(mut self, deps: D, effect: F) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout_with_deps::<T, F, D>(
                deps, effect,
            ));
        self
    }

    pub fn use_layout_effect_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout_with_cleanup(effect));
        self
    }

    pub fn use_layout_effect_once_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout_once_with_cleanup(effect));
        self
    }

    pub fn use_layout_effect_with_deps_and_cleanup<T, D, C, F>(mut self, deps: D, effect: F) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::layout_with_deps_and_cleanup::<
                T,
                F,
                D,
                C,
            >(deps, effect));
        self
    }

    pub fn use_insertion_effect<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion(effect));
        self
    }

    pub fn use_insertion_effect_once<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion_once(effect));
        self
    }

    pub fn use_insertion_effect_with_deps<T, D, F>(mut self, deps: D, effect: F) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion_with_deps::<T, F, D>(
                deps, effect,
            ));
        self
    }

    pub fn use_insertion_effect_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion_with_cleanup(effect));
        self
    }

    pub fn use_insertion_effect_once_with_cleanup<C, F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion_once_with_cleanup(effect));
        self
    }

    pub fn use_insertion_effect_with_deps_and_cleanup<T, D, C, F>(
        mut self,
        deps: D,
        effect: F,
    ) -> Self
    where
        T: Serialize,
        D: Fn(&S) -> T + Send + Sync + 'static,
        F: Fn(&mut S) -> GuiResult<C> + Send + Sync + 'static,
        C: Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.render_effect_hooks
            .push(RsxRenderEffectHook::insertion_with_deps_and_cleanup::<
                T,
                F,
                D,
                C,
            >(deps, effect));
        self
    }

    pub fn use_action_effect<F>(mut self, action: impl Into<String>, effect: F) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.action_effect_hooks
            .push(RsxEffectHook::for_action(action, effect));
        self
    }

    pub fn use_value_effect<T, F>(self, action: impl Into<String>, effect: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action_id = action.into();
        let context = action_id.clone();
        self.use_action_effect(action_id, move |state, invocation| {
            effect(state, decode_action_value::<T>(invocation, &context)?)
        })
    }

    pub fn use_payload_effect<T, F>(self, action: impl Into<String>, effect: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action_id = action.into();
        let context = action_id.clone();
        self.use_action_effect(action_id, move |state, invocation| {
            effect(state, decode_action_payload::<T>(invocation, &context)?)
        })
    }

    pub fn use_transition_effect<F>(mut self, effect: F) -> Self
    where
        S: Clone + 'static,
        F: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.action_effect_hooks
            .push(RsxEffectHook::transition_global(effect));
        self
    }

    pub fn use_action_transition_effect<F>(mut self, action: impl Into<String>, effect: F) -> Self
    where
        S: Clone + 'static,
        F: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.action_effect_hooks
            .push(RsxEffectHook::transition_for_action(action, effect));
        self
    }

    pub fn use_value_transition_effect<T, F>(self, action: impl Into<String>, effect: F) -> Self
    where
        S: Clone + 'static,
        T: DeserializeOwned + 'static,
        F: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        let action_id = action.into();
        let context = action_id.clone();
        self.use_action_transition_effect(action_id, move |state, transition| {
            effect(
                state,
                transition,
                decode_action_value::<T>(transition.invocation(), &context)?,
            )
        })
    }

    pub fn use_payload_transition_effect<T, F>(self, action: impl Into<String>, effect: F) -> Self
    where
        S: Clone + 'static,
        T: DeserializeOwned + 'static,
        F: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        let action_id = action.into();
        let context = action_id.clone();
        self.use_action_transition_effect(action_id, move |state, transition| {
            effect(
                state,
                transition,
                decode_action_payload::<T>(transition.invocation(), &context)?,
            )
        })
    }

    pub fn use_transition_reducer<R, E>(self, id: impl Into<String>, reducer: R, effect: E) -> Self
    where
        S: Clone,
        R: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_transition_reducer(id, None::<String>, reducer, effect)
    }

    pub fn use_labeled_transition_reducer<R, E>(
        mut self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: R,
        effect: E,
    ) -> Self
    where
        S: Clone,
        R: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
        if self.action_hooks.contains_key(&id) {
            self.duplicate_actions.insert(id.clone());
        }
        self.action_hooks.insert(
            id,
            RsxActionHook::new(label.map(Into::into), move |state: &mut S, invocation| {
                let before = state.clone();
                reducer(state, invocation)?;
                let transition = RsxActionTransition::new(&before, invocation);
                effect(state, &transition)
            }),
        );
        self
    }

    pub fn use_value_transition_reducer<T, R, E>(
        self,
        id: impl Into<String>,
        reducer: R,
        effect: E,
    ) -> Self
    where
        S: Clone,
        T: DeserializeOwned + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        self.use_labeled_value_transition_reducer(id, None::<String>, reducer, effect)
    }

    pub fn use_labeled_value_transition_reducer<T, R, E>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: R,
        effect: E,
    ) -> Self
    where
        S: Clone,
        T: DeserializeOwned + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        let action_id = id.into();
        let reducer_context = action_id.clone();
        let effect_context = action_id.clone();
        self.use_labeled_transition_reducer(
            action_id,
            label,
            move |state, invocation| {
                reducer(
                    state,
                    decode_action_value::<T>(invocation, &reducer_context)?,
                )
            },
            move |state, transition| {
                effect(
                    state,
                    transition,
                    decode_action_value::<T>(transition.invocation(), &effect_context)?,
                )
            },
        )
    }

    pub fn use_payload_transition_reducer<T, R, E>(
        self,
        id: impl Into<String>,
        reducer: R,
        effect: E,
    ) -> Self
    where
        S: Clone,
        T: DeserializeOwned + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        self.use_labeled_payload_transition_reducer(id, None::<String>, reducer, effect)
    }

    pub fn use_labeled_payload_transition_reducer<T, R, E>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: R,
        effect: E,
    ) -> Self
    where
        S: Clone,
        T: DeserializeOwned + 'static,
        R: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
        E: for<'a> Fn(&mut S, &RsxActionTransition<'a, S>, T) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    {
        let action_id = id.into();
        let reducer_context = action_id.clone();
        let effect_context = action_id.clone();
        self.use_labeled_transition_reducer(
            action_id,
            label,
            move |state, invocation| {
                reducer(
                    state,
                    decode_action_payload::<T>(invocation, &reducer_context)?,
                )
            },
            move |state, transition| {
                effect(
                    state,
                    transition,
                    decode_action_payload::<T>(transition.invocation(), &effect_context)?,
                )
            },
        )
    }

    pub fn use_action<F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_action(id, None::<String>, reducer)
    }

    pub fn use_labeled_action<F>(
        mut self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
        if self.action_hooks.contains_key(&id) {
            self.duplicate_actions.insert(id.clone());
        }
        self.action_hooks
            .insert(id, RsxActionHook::new(label.map(Into::into), reducer));
        self
    }

    pub fn use_action_disabled<F>(self, id: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> bool + Send + Sync + 'static,
    {
        self.use_action_disabled_result(id, move |state| Ok(selector(state)))
    }

    pub fn use_action_disabled_result<F>(mut self, id: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    {
        let id = id.into();
        if self.action_disabled_hooks.contains_key(&id) {
            self.duplicate_action_disabled_hooks.insert(id.clone());
        }
        self.action_disabled_hooks
            .insert(id.clone(), RsxActionDisabledHook::disabled(id, selector));
        self
    }

    pub fn use_action_enabled<F>(self, id: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> bool + Send + Sync + 'static,
    {
        self.use_action_enabled_result(id, move |state| Ok(selector(state)))
    }

    pub fn use_action_enabled_result<F>(mut self, id: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    {
        let id = id.into();
        if self.action_disabled_hooks.contains_key(&id) {
            self.duplicate_action_disabled_hooks.insert(id.clone());
        }
        self.action_disabled_hooks
            .insert(id.clone(), RsxActionDisabledHook::enabled(id, selector));
        self
    }

    pub fn use_value_action<T, F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_value_action(id, None::<String>, reducer)
    }

    pub fn use_labeled_value_action<T, F>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action_id = id.into();
        let context = action_id.clone();
        self.use_labeled_action(action_id, label, move |state, invocation| {
            reducer(state, decode_action_value::<T>(invocation, &context)?)
        })
    }

    pub fn use_payload_action<T, F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_payload_action(id, None::<String>, reducer)
    }

    pub fn use_labeled_payload_action<T, F>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        let action_id = id.into();
        let context = action_id.clone();
        self.use_labeled_action(action_id, label, move |state, invocation| {
            reducer(state, decode_action_payload::<T>(invocation, &context)?)
        })
    }

    pub fn use_callback<F>(self, id: impl Into<String>, callback: F) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_reducer(id, callback)
    }

    pub fn use_reducer<F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_action(id, reducer)
    }

    pub fn use_labeled_reducer<F>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_action(id, label, reducer)
    }

    pub fn use_value_reducer<T, F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_value_action(id, reducer)
    }

    pub fn use_labeled_value_reducer<T, F>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_value_action(id, label, reducer)
    }

    pub fn use_payload_reducer<T, F>(self, id: impl Into<String>, reducer: F) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_payload_action(id, reducer)
    }

    pub fn use_labeled_payload_reducer<T, F>(
        self,
        id: impl Into<String>,
        label: Option<impl Into<String>>,
        reducer: F,
    ) -> Self
    where
        T: DeserializeOwned + 'static,
        F: Fn(&mut S, T) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.use_labeled_payload_action(id, label, reducer)
    }

    pub fn scope(&self, state: &S) -> GuiResult<JsonValue> {
        self.scope_with_context_scope(state, JsonMap::new())
    }

    fn scope_with_context_scope(
        &self,
        state: &S,
        mut context_scope: JsonMap<String, JsonValue>,
    ) -> GuiResult<JsonValue> {
        let mut state_scope = JsonMap::new();
        for hook in &self.state_hooks {
            insert_scope_value(&mut state_scope, &hook.path, (hook.selector)(state)?)?;
        }

        let mut props_scope = JsonMap::new();
        for hook in &self.prop_hooks {
            insert_scope_value(&mut props_scope, &hook.path, (hook.selector)(state)?)?;
        }

        let mut derived_scope = JsonMap::new();
        for hook in &self.derived_hooks {
            insert_scope_value(&mut derived_scope, &hook.path, (hook.selector)(state)?)?;
        }

        for hook in &self.context_hooks {
            insert_scope_value(&mut context_scope, &hook.path, (hook.selector)(state)?)?;
        }

        let mut resource_scope = JsonMap::new();
        for hook in &self.resource_hooks {
            insert_scope_value(&mut resource_scope, &hook.path, (hook.selector)(state)?)?;
        }

        let mut scope = JsonMap::new();
        scope.insert("state".to_string(), JsonValue::Object(state_scope));
        scope.insert("props".to_string(), JsonValue::Object(props_scope));
        scope.insert("derived".to_string(), JsonValue::Object(derived_scope));
        scope.insert("context".to_string(), JsonValue::Object(context_scope));
        scope.insert("resource".to_string(), JsonValue::Object(resource_scope));
        Ok(JsonValue::Object(scope))
    }

    pub fn validate(&self) -> GuiResult<()> {
        self.validate_with_context_paths(&[])
    }

    pub fn debug_values(&self, state: &S) -> GuiResult<Vec<RsxDebugValue>> {
        self.debug_hooks
            .iter()
            .map(|hook| hook.select(state))
            .collect()
    }

    fn validate_with_context_paths(&self, context_paths: &[Vec<String>]) -> GuiResult<()> {
        self.validate_value_hook_paths("state", &self.state_hooks)?;
        self.validate_value_hook_paths("props", &self.prop_hooks)?;
        self.validate_value_hook_paths("derived", &self.derived_hooks)?;
        self.validate_value_hook_paths("context", &self.context_hooks)?;
        self.validate_value_hook_paths("resource", &self.resource_hooks)?;
        self.validate_action_hook_ids()?;
        self.validate_unique_action_hooks()?;
        self.validate_action_disabled_hooks()?;
        self.validate_action_effect_hooks()?;
        self.validate_component_contracts()?;
        self.validate_template_bindings(context_paths)?;
        self.validate_static_template_actions()?;
        if let Some(window) = &self.window {
            window.validate()?;
        }
        Ok(())
    }

    pub fn render(&self, state: &S) -> GuiResult<UiFrame> {
        self.render_with_context_scope(state, JsonMap::new(), &[])
    }

    fn render_with_context_scope(
        &self,
        state: &S,
        context_scope: JsonMap<String, JsonValue>,
        context_paths: &[Vec<String>],
    ) -> GuiResult<UiFrame> {
        self.validate_with_context_paths(context_paths)?;
        let scope = self.scope_with_context_scope(state, context_scope)?;
        let component_defaults = self.component_default_props();
        let component_variants = self.component_registry.variants();
        let mut frame = self
            .template
            .render_with_scope_parts_and_component_options(
                &self.frame_id,
                &scope,
                None,
                self.window.clone(),
                self.component_registry.templates(),
                &component_defaults,
                component_variants,
            )?;
        self.validate_inferred_actions(&frame.actions)?;
        frame.actions = self.registered_actions(state, &frame.actions)?;
        frame.validate()?;
        Ok(frame)
    }

    pub fn reduce(&self, state: &mut S, invocation: &ActionInvocation) -> GuiResult<()> {
        self.validate()?;
        let Some(action) = self.action_hooks.get(&invocation.action) else {
            return Err(GuiError::host(format!(
                "RSX action {:?} has no reducer hook",
                invocation.action
            )));
        };
        if self.action_is_disabled(state, &invocation.action)? {
            return Err(GuiError::host(format!(
                "RSX action {:?} is disabled",
                invocation.action
            )));
        }
        let effect_captures = self
            .action_effect_hooks
            .iter()
            .map(|effect| effect.capture_if_matches(state, invocation))
            .collect::<Vec<_>>();
        action.reduce(state, invocation)?;
        for (effect, capture) in self.action_effect_hooks.iter().zip(effect_captures.iter()) {
            effect.run_if_matches(state, invocation, capture.as_ref())?;
        }
        Ok(())
    }

    pub fn mount(&self, state: &mut S) -> GuiResult<()> {
        self.validate()?;
        self.run_mount_hooks(state)
    }

    pub fn unmount(&self, state: &mut S) -> GuiResult<()> {
        self.validate()?;
        self.run_unmount_hooks(state)
    }

    pub fn into_protocol_app<A>(
        self,
        adapter: A,
        mut state: S,
    ) -> NativeProtocolApp<
        A,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        let mount_error = self.run_mount_hooks(&mut state).err();
        self.into_protocol_app_mounted(adapter, state, mount_error)
    }

    fn into_protocol_app_mounted<A>(
        self,
        adapter: A,
        state: S,
        mount_error: Option<GuiError>,
    ) -> NativeProtocolApp<
        A,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        let component = Arc::new(self);
        let render_component = Arc::clone(&component);
        let reduce_component = Arc::clone(&component);
        let effect_component = Arc::clone(&component);
        let cleanup_component = component;
        let effect_runtime = Arc::new(Mutex::new(effect_component.new_render_effect_runtime()));
        let render_effect_runtime = Arc::clone(&effect_runtime);
        let cleanup_effect_runtime = effect_runtime;
        NativeProtocolApp::new(
            adapter,
            state,
            move |state| {
                if let Some(error) = mount_error.as_ref() {
                    return Err(error.clone());
                }
                render_component.render(state)
            },
            move |state, invocation| reduce_component.reduce(state, invocation),
        )
        .with_render_effect(move |state| {
            let mut runtime = render_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            effect_component.run_render_effect_hooks(state, &mut runtime)
        })
        .with_cleanup_effect(move |state| {
            let mut runtime = cleanup_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            cleanup_component.cleanup_render_effect_hooks(state, &mut runtime)
        })
    }

    pub fn try_into_protocol_app<A>(
        self,
        adapter: A,
        state: S,
    ) -> GuiResult<
        NativeProtocolApp<
            A,
            S,
            impl Fn(&S) -> GuiResult<UiFrame>,
            impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
        >,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        self.validate()?;
        let mut state = state;
        self.run_mount_hooks(&mut state)?;
        Ok(self.into_protocol_app_mounted(adapter, state, None))
    }

    pub fn into_runtime_app<H>(
        self,
        host: H,
        mut state: S,
    ) -> NativeRuntimeApp<
        H,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        let mount_error = self.run_mount_hooks(&mut state).err();
        self.into_runtime_app_mounted(host, state, mount_error)
    }

    fn into_runtime_app_mounted<H>(
        self,
        host: H,
        state: S,
        mount_error: Option<GuiError>,
    ) -> NativeRuntimeApp<
        H,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        let component = Arc::new(self);
        let render_component = Arc::clone(&component);
        let reduce_component = Arc::clone(&component);
        let effect_component = Arc::clone(&component);
        let cleanup_component = component;
        let effect_runtime = Arc::new(Mutex::new(effect_component.new_render_effect_runtime()));
        let render_effect_runtime = Arc::clone(&effect_runtime);
        let cleanup_effect_runtime = effect_runtime;
        NativeRuntimeApp::new(
            host,
            state,
            move |state| {
                if let Some(error) = mount_error.as_ref() {
                    return Err(error.clone());
                }
                render_component.render(state)
            },
            move |state, invocation| reduce_component.reduce(state, invocation),
        )
        .with_render_effect(move |state| {
            let mut runtime = render_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            effect_component.run_render_effect_hooks(state, &mut runtime)
        })
        .with_cleanup_effect(move |state| {
            let mut runtime = cleanup_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            cleanup_component.cleanup_render_effect_hooks(state, &mut runtime)
        })
    }

    pub fn try_into_runtime_app<H>(
        self,
        host: H,
        state: S,
    ) -> GuiResult<
        NativeRuntimeApp<
            H,
            S,
            impl Fn(&S) -> GuiResult<UiFrame>,
            impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
        >,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        self.validate()?;
        let mut state = state;
        self.run_mount_hooks(&mut state)?;
        Ok(self.into_runtime_app_mounted(host, state, None))
    }

    fn run_mount_hooks(&self, state: &mut S) -> GuiResult<()> {
        for hook in &self.mount_hooks {
            hook.run(state)?;
        }
        Ok(())
    }

    fn run_unmount_hooks(&self, state: &mut S) -> GuiResult<()> {
        for hook in &self.unmount_hooks {
            hook.run(state)?;
        }
        Ok(())
    }

    fn new_render_effect_runtime(&self) -> RsxRenderEffectRuntime<S> {
        RsxRenderEffectRuntime::new(self.render_effect_hooks.len())
    }

    fn run_render_effect_hooks(
        &self,
        state: &mut S,
        runtime: &mut RsxRenderEffectRuntime<S>,
    ) -> GuiResult<()> {
        runtime.run(&self.render_effect_hooks, state)
    }

    fn cleanup_render_effect_hooks(
        &self,
        state: &mut S,
        runtime: &mut RsxRenderEffectRuntime<S>,
    ) -> GuiResult<()> {
        runtime.cleanup(&self.render_effect_hooks, state)
    }

    fn validate_value_hook_paths(&self, kind: &str, hooks: &[RsxValueHook<S>]) -> GuiResult<()> {
        let mut paths = BTreeSet::new();
        for hook in hooks {
            validate_hook_path(kind, &hook.path)?;
            if !paths.insert(hook.path.as_str()) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX {kind} hook path {:?} was registered more than once",
                    hook.path
                )));
            }
        }
        Ok(())
    }

    fn validate_action_hook_ids(&self) -> GuiResult<()> {
        if self.action_hooks.keys().any(|id| id.is_empty()) {
            return Err(GuiError::invalid_tree(
                "RSX action hook id must be non-empty",
            ));
        }
        Ok(())
    }

    fn validate_unique_action_hooks(&self) -> GuiResult<()> {
        if self.duplicate_actions.is_empty() {
            return Ok(());
        }

        let actions = self
            .duplicate_actions
            .iter()
            .map(|action| format!("{action:?}"))
            .collect::<Vec<_>>()
            .join(", ");
        let verb = if self.duplicate_actions.len() == 1 {
            "was"
        } else {
            "were"
        };

        Err(GuiError::invalid_tree(format!(
            "RSX action hook {actions} {verb} registered more than once"
        )))
    }

    fn validate_action_disabled_hooks(&self) -> GuiResult<()> {
        if !self.duplicate_action_disabled_hooks.is_empty() {
            let actions = self
                .duplicate_action_disabled_hooks
                .iter()
                .map(|action| format!("{action:?}"))
                .collect::<Vec<_>>()
                .join(", ");
            let verb = if self.duplicate_action_disabled_hooks.len() == 1 {
                "was"
            } else {
                "were"
            };

            return Err(GuiError::invalid_tree(format!(
                "RSX action disabled hook {actions} {verb} registered more than once"
            )));
        }

        for hook in self.action_disabled_hooks.values() {
            let action = hook.action.as_str();
            if action.is_empty() {
                return Err(GuiError::invalid_tree(
                    "RSX action disabled hook target must be a non-empty action id",
                ));
            }
            if !self.action_hooks.contains_key(action) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX action disabled hook {action:?} has no reducer hook; add use_action or use_reducer for that action"
                )));
            }
        }
        Ok(())
    }

    fn validate_action_effect_hooks(&self) -> GuiResult<()> {
        for effect in &self.action_effect_hooks {
            let Some(action) = effect.action() else {
                continue;
            };
            if action.is_empty() {
                return Err(GuiError::invalid_tree(
                    "RSX action effect target must be a non-empty action id",
                ));
            }
            if !self.action_hooks.contains_key(action) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX action effect {action:?} has no reducer hook; add use_action or use_reducer for that action"
                )));
            }
        }
        Ok(())
    }

    fn validate_component_contracts(&self) -> GuiResult<()> {
        self.validate_component_contracts_in_node(self.template.root(), &mut Vec::new())
    }

    fn validate_component_contracts_in_node(
        &self,
        node: &CompiledRsxNode,
        component_stack: &mut Vec<String>,
    ) -> GuiResult<()> {
        let CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } = node
        else {
            return Ok(());
        };

        if let Some(contract) = self.component_registry.contracts().get(tag) {
            contract.validate_invocation(tag, props)?;
        }

        if let Some(component) = self.component_registry.templates().get(tag) {
            if component_stack.iter().any(|name| name == tag) {
                let mut cycle = component_stack.clone();
                cycle.push(tag.clone());
                return Err(GuiError::invalid_tree(format!(
                    "RSX component cycle detected: {}",
                    cycle.join(" -> ")
                )));
            }
            component_stack.push(tag.clone());
            self.validate_component_contracts_in_node(component, component_stack)?;
            component_stack.pop();
        }

        for child in children {
            self.validate_component_contracts_in_node(child, component_stack)?;
        }
        Ok(())
    }

    fn validate_template_bindings(&self, context_paths: &[Vec<String>]) -> GuiResult<()> {
        let coverage = BindingHookCoverage::new(
            &self.state_hooks,
            &self.prop_hooks,
            &self.derived_hooks,
            &self.context_hooks,
            &self.resource_hooks,
            context_paths,
        );
        self.validate_bindings_in_node(self.template.root(), None, &coverage, &mut Vec::new())
    }

    fn validate_bindings_in_node(
        &self,
        node: &CompiledRsxNode,
        component_props: Option<&str>,
        coverage: &BindingHookCoverage,
        component_stack: &mut Vec<String>,
    ) -> GuiResult<()> {
        let CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } = node
        else {
            return Ok(());
        };

        self.validate_props_bindings(props, component_props, coverage)?;

        if let Some(component) = self.component_registry.templates().get(tag) {
            if component_stack.iter().any(|name| name == tag) {
                let mut cycle = component_stack.clone();
                cycle.push(tag.clone());
                return Err(GuiError::invalid_tree(format!(
                    "RSX component cycle detected: {}",
                    cycle.join(" -> ")
                )));
            }
            component_stack.push(tag.clone());
            self.validate_bindings_in_node(component, Some(tag), coverage, component_stack)?;
            component_stack.pop();
        }

        if matches!(tag.as_str(), "Show" | "When") {
            return Ok(());
        }

        for child in children {
            self.validate_bindings_in_node(child, component_props, coverage, component_stack)?;
        }
        Ok(())
    }

    fn validate_props_bindings(
        &self,
        props: &CompiledProps,
        component_props: Option<&str>,
        coverage: &BindingHookCoverage,
    ) -> GuiResult<()> {
        for binding in props.spreads.iter().chain(props.bindings.values()) {
            self.validate_binding(binding, component_props, coverage)?;
        }
        Ok(())
    }

    fn validate_binding(
        &self,
        binding: &CompiledBinding,
        component_props: Option<&str>,
        coverage: &BindingHookCoverage,
    ) -> GuiResult<()> {
        match binding.source {
            CompiledBindingSource::State => coverage.validate_hook_binding("state", binding),
            CompiledBindingSource::Derived => coverage.validate_hook_binding("derived", binding),
            CompiledBindingSource::Context => coverage.validate_hook_binding("context", binding),
            CompiledBindingSource::Resource => coverage.validate_hook_binding("resource", binding),
            CompiledBindingSource::Props => {
                if let Some(component) = component_props {
                    self.validate_component_prop_binding(component, binding)
                } else {
                    coverage.validate_hook_binding("props", binding)
                }
            }
            CompiledBindingSource::Local => Ok(()),
        }
    }

    fn validate_component_prop_binding(
        &self,
        component: &str,
        binding: &CompiledBinding,
    ) -> GuiResult<()> {
        let Some(contract) = self.component_registry.contracts().get(component) else {
            return Ok(());
        };
        if contract.covers_binding_path(&binding.path) {
            Ok(())
        } else {
            Err(GuiError::invalid_tree(format!(
                "RSX component {component:?} template binding {} is not declared in its prop contract",
                binding.display_path()
            )))
        }
    }

    fn validate_static_template_actions(&self) -> GuiResult<()> {
        let mut actions = BTreeSet::new();
        collect_static_action_ids(
            self.template.root(),
            self.component_registry.templates(),
            &mut Vec::new(),
            &mut actions,
        )?;
        if let Some(on_close) = self
            .window
            .as_ref()
            .and_then(|window| window.on_close.as_deref())
            .filter(|action| !action.is_empty())
        {
            actions.insert(on_close.to_string());
        }
        for action in actions {
            if !self.action_hooks.contains_key(&action) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX action {action:?} has no reducer hook; add use_action or use_reducer"
                )));
            }
        }
        Ok(())
    }

    fn validate_inferred_actions(&self, actions: &[UiAction]) -> GuiResult<()> {
        for action in actions {
            if !self.action_hooks.contains_key(&action.id) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX action {:?} has no reducer hook; add use_action or use_reducer",
                    action.id
                )));
            }
        }
        Ok(())
    }

    fn registered_actions(
        &self,
        state: &S,
        inferred_actions: &[UiAction],
    ) -> GuiResult<Vec<UiAction>> {
        let inferred_labels = inferred_actions
            .iter()
            .filter_map(|action| {
                action
                    .label
                    .as_ref()
                    .map(|label| (action.id.as_str(), label))
            })
            .collect::<BTreeMap<_, _>>();

        let mut actions = Vec::new();
        for (id, action) in &self.action_hooks {
            actions.push(UiAction {
                id: id.clone(),
                disabled: self.action_is_disabled(state, id)?,
                label: action.label().map(ToOwned::to_owned).or_else(|| {
                    inferred_labels
                        .get(id.as_str())
                        .map(|label| (*label).clone())
                }),
            });
        }
        Ok(actions)
    }

    fn action_is_disabled(&self, state: &S, action: &str) -> GuiResult<bool> {
        self.action_disabled_hooks
            .get(action)
            .map(|hook| hook.is_disabled(state))
            .unwrap_or(Ok(false))
    }

    fn validate_new_component_name(&self, name: &str) -> GuiResult<()> {
        if self.component_registry.templates().contains_key(name) {
            return Err(GuiError::invalid_tree(format!(
                "RSX component {name:?} was registered more than once"
            )));
        }
        Ok(())
    }

    fn component_default_props(&self) -> BTreeMap<String, BTreeMap<String, JsonValue>> {
        self.component_registry
            .contracts()
            .iter()
            .filter(|(_, contract)| !contract.default_props().is_empty())
            .map(|(component, contract)| (component.clone(), contract.default_props().clone()))
            .collect()
    }
}

impl<S> RsxRouter<S> {
    pub fn new<F>(route_selector: F) -> Self
    where
        F: Fn(&S) -> String + Send + Sync + 'static,
    {
        Self::new_result(move |state| Ok(route_selector(state)))
    }

    pub fn new_result<F>(route_selector: F) -> Self
    where
        F: Fn(&S) -> GuiResult<String> + Send + Sync + 'static,
    {
        Self {
            layout: None,
            routes: BTreeMap::new(),
            route_selector: Box::new(route_selector),
            default_route: None,
            route_context_hooks: Vec::new(),
            route_effect_hooks: Vec::new(),
        }
    }

    pub fn layout(mut self, component: RsxComponent<S>) -> Self {
        self.layout = Some(component);
        self
    }

    pub fn route(mut self, id: impl Into<String>, component: RsxComponent<S>) -> GuiResult<Self> {
        let id = id.into();
        validate_route_id(&id)?;
        if self.routes.contains_key(&id) {
            return Err(GuiError::invalid_tree(format!(
                "RSX route {id:?} was registered more than once"
            )));
        }
        self.routes.insert(id, component);
        Ok(self)
    }

    pub fn default_route(mut self, id: impl Into<String>) -> Self {
        self.default_route = Some(id.into());
        self
    }

    pub fn use_routes<F>(self, routes: F) -> Self
    where
        F: FnOnce(Self) -> Self,
    {
        routes(self)
    }

    pub fn try_use_routes<F>(self, routes: F) -> GuiResult<Self>
    where
        F: FnOnce(Self) -> GuiResult<Self>,
    {
        routes(self)
    }

    pub fn use_route_context<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S, &str) -> T + Send + Sync + 'static,
    {
        self.route_context_hooks
            .push(RsxRouteContextHook::serializing(path, selector));
        self
    }

    pub fn use_route_context_result<T, F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S, &str) -> GuiResult<T> + Send + Sync + 'static,
    {
        self.route_context_hooks
            .push(RsxRouteContextHook::serializing_result(path, selector));
        self
    }

    pub fn use_route_context_value<F>(mut self, path: impl Into<String>, selector: F) -> Self
    where
        F: Fn(&S, &str) -> GuiResult<JsonValue> + Send + Sync + 'static,
    {
        self.route_context_hooks
            .push(RsxRouteContextHook::value(path, selector));
        self
    }

    pub fn use_route_effect<F>(mut self, effect: F) -> Self
    where
        F: Fn(&mut S, &str, &str, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.route_effect_hooks
            .push(RsxRouteEffectHook::new(move |state, transition| {
                effect(
                    state,
                    transition.from(),
                    transition.to(),
                    transition.invocation(),
                )
            }));
        self
    }

    pub fn use_route_transition_effect<F>(mut self, effect: F) -> Self
    where
        F: for<'a> Fn(&mut S, &RsxRouteTransition<'a>) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.route_effect_hooks
            .push(RsxRouteEffectHook::new(effect));
        self
    }

    pub fn validate(&self) -> GuiResult<()> {
        if self.routes.is_empty() {
            return Err(GuiError::invalid_tree(
                "RSX router needs at least one registered route",
            ));
        }
        self.validate_route_context_hooks()?;
        let route_context_paths = self.route_context_paths();
        if let Some(layout) = &self.layout {
            layout.validate_with_context_paths(&route_context_paths)?;
            validate_route_layout_outlet(layout.template.root())?;
        }
        for (id, component) in &self.routes {
            validate_route_id(id)?;
            component.validate_with_context_paths(&route_context_paths)?;
            self.validate_layout_route_action_collisions(id, component)?;
        }
        if let Some(default_route) = &self.default_route {
            validate_route_id(default_route)?;
            if !self.routes.contains_key(default_route) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX router default route {default_route:?} is not registered"
                )));
            }
        }
        Ok(())
    }

    pub fn render(&self, state: &S) -> GuiResult<UiFrame> {
        self.validate()?;
        let route = self.active_route_id(state)?;
        let context_scope = self.route_context_scope(state, &route)?;
        let route_context_paths = self.route_context_paths();
        let route_frame = self
            .routes
            .get(&route)
            .expect("active route should be registered")
            .render_with_context_scope(state, context_scope, &route_context_paths)?;

        let Some(layout) = &self.layout else {
            return Ok(route_frame);
        };

        let layout_context_scope = self.route_context_scope(state, &route)?;
        let layout_frame =
            layout.render_with_context_scope(state, layout_context_scope, &route_context_paths)?;
        let root = splice_route_layout_outlet(layout_frame.root, route_frame.root)?;
        let actions = merge_router_actions(layout_frame.actions, route_frame.actions)?;
        let window = layout_frame.window.or(route_frame.window);
        UiFrame::from_compiled_parts(layout_frame.frame_id, root, Some(actions), window)
    }

    pub fn reduce(&self, state: &mut S, invocation: &ActionInvocation) -> GuiResult<()> {
        self.validate()?;
        let before = self.active_route_id(state)?;
        if self.layout_handles_action(&invocation.action) {
            self.layout
                .as_ref()
                .expect("layout should be registered")
                .reduce(state, invocation)?;
        } else {
            self.routes
                .get(&before)
                .expect("active route should be registered")
                .reduce(state, invocation)?;
        }
        let after = self.active_route_id(state)?;
        if after != before {
            self.run_route_change_hooks(state, &before, &after, invocation)?;
        }
        Ok(())
    }

    pub fn into_protocol_app<A>(
        self,
        adapter: A,
        mut state: S,
    ) -> NativeProtocolApp<
        A,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        let mount_error = self.mount_initial_components(&mut state).err();
        self.into_protocol_app_mounted(adapter, state, mount_error)
    }

    fn into_protocol_app_mounted<A>(
        self,
        adapter: A,
        state: S,
        mount_error: Option<GuiError>,
    ) -> NativeProtocolApp<
        A,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        let router = Arc::new(self);
        let render_router = Arc::clone(&router);
        let reduce_router = Arc::clone(&router);
        let effect_router = Arc::clone(&router);
        let cleanup_router = router;
        let effect_runtime = Arc::new(Mutex::new(effect_router.new_render_effect_runtime()));
        let render_effect_runtime = Arc::clone(&effect_runtime);
        let cleanup_effect_runtime = effect_runtime;
        NativeProtocolApp::new(
            adapter,
            state,
            move |state| {
                if let Some(error) = mount_error.as_ref() {
                    return Err(error.clone());
                }
                render_router.render(state)
            },
            move |state, invocation| reduce_router.reduce(state, invocation),
        )
        .with_render_effect(move |state| {
            let mut runtime = render_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            effect_router.run_render_effect_hooks(state, &mut runtime)
        })
        .with_cleanup_effect(move |state| {
            let mut runtime = cleanup_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            cleanup_router.cleanup_render_effect_hooks(state, &mut runtime)
        })
    }

    pub fn try_into_protocol_app<A>(
        self,
        adapter: A,
        state: S,
    ) -> GuiResult<
        NativeProtocolApp<
            A,
            S,
            impl Fn(&S) -> GuiResult<UiFrame>,
            impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
        >,
    >
    where
        A: PlatformAdapter,
        S: 'static,
    {
        self.validate()?;
        let mut state = state;
        self.mount_initial_components(&mut state)?;
        Ok(self.into_protocol_app_mounted(adapter, state, None))
    }

    pub fn into_runtime_app<H>(
        self,
        host: H,
        mut state: S,
    ) -> NativeRuntimeApp<
        H,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        let mount_error = self.mount_initial_components(&mut state).err();
        self.into_runtime_app_mounted(host, state, mount_error)
    }

    fn into_runtime_app_mounted<H>(
        self,
        host: H,
        state: S,
        mount_error: Option<GuiError>,
    ) -> NativeRuntimeApp<
        H,
        S,
        impl Fn(&S) -> GuiResult<UiFrame>,
        impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        let router = Arc::new(self);
        let render_router = Arc::clone(&router);
        let reduce_router = Arc::clone(&router);
        let effect_router = Arc::clone(&router);
        let cleanup_router = router;
        let effect_runtime = Arc::new(Mutex::new(effect_router.new_render_effect_runtime()));
        let render_effect_runtime = Arc::clone(&effect_runtime);
        let cleanup_effect_runtime = effect_runtime;
        NativeRuntimeApp::new(
            host,
            state,
            move |state| {
                if let Some(error) = mount_error.as_ref() {
                    return Err(error.clone());
                }
                render_router.render(state)
            },
            move |state, invocation| reduce_router.reduce(state, invocation),
        )
        .with_render_effect(move |state| {
            let mut runtime = render_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            effect_router.run_render_effect_hooks(state, &mut runtime)
        })
        .with_cleanup_effect(move |state| {
            let mut runtime = cleanup_effect_runtime.lock().map_err(|_| {
                GuiError::invalid_tree("RSX render effect runtime lock was poisoned")
            })?;
            cleanup_router.cleanup_render_effect_hooks(state, &mut runtime)
        })
    }

    pub fn try_into_runtime_app<H>(
        self,
        host: H,
        state: S,
    ) -> GuiResult<
        NativeRuntimeApp<
            H,
            S,
            impl Fn(&S) -> GuiResult<UiFrame>,
            impl FnMut(&mut S, &ActionInvocation) -> GuiResult<()>,
        >,
    >
    where
        H: NativeHost,
        S: 'static,
    {
        self.validate()?;
        let mut state = state;
        self.mount_initial_components(&mut state)?;
        Ok(self.into_runtime_app_mounted(host, state, None))
    }

    fn mount_initial_components(&self, state: &mut S) -> GuiResult<()> {
        self.validate()?;
        if let Some(layout) = &self.layout {
            layout.run_mount_hooks(state)?;
        }
        let route = self.active_route_id(state)?;
        self.routes
            .get(&route)
            .expect("active route should be registered")
            .run_mount_hooks(state)
    }

    fn run_route_change_hooks(
        &self,
        state: &mut S,
        before: &str,
        after: &str,
        invocation: &ActionInvocation,
    ) -> GuiResult<()> {
        self.routes
            .get(before)
            .expect("active route should be registered")
            .run_unmount_hooks(state)?;
        self.routes
            .get(after)
            .expect("active route should be registered")
            .run_mount_hooks(state)?;
        self.run_route_effect_hooks(state, before, after, invocation)
    }

    fn new_render_effect_runtime(&self) -> RsxRouterRenderEffectRuntime<S> {
        RsxRouterRenderEffectRuntime {
            layout: self
                .layout
                .as_ref()
                .map(RsxComponent::new_render_effect_runtime),
            routes: self
                .routes
                .iter()
                .map(|(route, component)| (route.clone(), component.new_render_effect_runtime()))
                .collect(),
            active_route: None,
        }
    }

    fn run_render_effect_hooks(
        &self,
        state: &mut S,
        runtime: &mut RsxRouterRenderEffectRuntime<S>,
    ) -> GuiResult<()> {
        let route = self.active_route_id(state)?;
        if let Some(previous) = runtime.active_route.clone() {
            if previous != route {
                self.cleanup_route_render_effect_hooks(state, &previous, runtime)?;
            }
        }
        runtime.active_route = Some(route.clone());

        if let Some(layout) = &self.layout {
            let layout_runtime = runtime.layout.as_mut().ok_or_else(|| {
                GuiError::invalid_tree("RSX router layout effect runtime was not initialized")
            })?;
            layout.run_render_effect_hooks(state, layout_runtime)?;
        }

        let route_component = self
            .routes
            .get(&route)
            .expect("active route should be registered");
        let route_runtime = runtime.routes.get_mut(&route).ok_or_else(|| {
            GuiError::invalid_tree(format!(
                "RSX route {route:?} effect runtime was not initialized"
            ))
        })?;
        route_component.run_render_effect_hooks(state, route_runtime)
    }

    fn cleanup_render_effect_hooks(
        &self,
        state: &mut S,
        runtime: &mut RsxRouterRenderEffectRuntime<S>,
    ) -> GuiResult<()> {
        if let Some(route) = runtime.active_route.clone() {
            self.cleanup_route_render_effect_hooks(state, &route, runtime)?;
        }
        runtime.active_route = None;
        if let Some(layout) = &self.layout {
            if let Some(layout_runtime) = runtime.layout.as_mut() {
                layout.cleanup_render_effect_hooks(state, layout_runtime)?;
                *layout_runtime = layout.new_render_effect_runtime();
            }
        }
        Ok(())
    }

    fn cleanup_route_render_effect_hooks(
        &self,
        state: &mut S,
        route: &str,
        runtime: &mut RsxRouterRenderEffectRuntime<S>,
    ) -> GuiResult<()> {
        let Some(route_component) = self.routes.get(route) else {
            return Ok(());
        };
        if let Some(route_runtime) = runtime.routes.get_mut(route) {
            route_component.cleanup_render_effect_hooks(state, route_runtime)?;
            *route_runtime = route_component.new_render_effect_runtime();
        }
        Ok(())
    }

    fn layout_handles_action(&self, action: &str) -> bool {
        self.layout
            .as_ref()
            .is_some_and(|layout| layout.action_hooks.contains_key(action))
    }

    fn validate_layout_route_action_collisions(
        &self,
        route: &str,
        component: &RsxComponent<S>,
    ) -> GuiResult<()> {
        let Some(layout) = &self.layout else {
            return Ok(());
        };
        let collisions = layout
            .action_hooks
            .keys()
            .filter(|action| component.action_hooks.contains_key(*action))
            .map(|action| format!("{action:?}"))
            .collect::<Vec<_>>();
        if collisions.is_empty() {
            return Ok(());
        }
        Err(GuiError::invalid_tree(format!(
            "RSX router layout and route {route:?} both register action {}",
            collisions.join(", ")
        )))
    }

    fn route_context_scope(&self, state: &S, route: &str) -> GuiResult<JsonMap<String, JsonValue>> {
        let mut context = JsonMap::new();
        insert_scope_value(
            &mut context,
            "route.id",
            JsonValue::String(route.to_string()),
        )?;
        for hook in &self.route_context_hooks {
            insert_scope_value(
                &mut context,
                &route_context_scope_path(&hook.path),
                hook.select(state, route)?,
            )?;
        }
        Ok(context)
    }

    fn route_context_paths(&self) -> Vec<Vec<String>> {
        let mut paths = vec![vec!["route".to_string(), "id".to_string()]];
        paths.extend(
            self.route_context_hooks
                .iter()
                .map(|hook| route_context_segments(&hook.path)),
        );
        paths
    }

    fn validate_route_context_hooks(&self) -> GuiResult<()> {
        let reserved_id = vec!["id".to_string()];
        let mut registered = Vec::<Vec<String>>::new();
        for hook in &self.route_context_hooks {
            validate_hook_path("route context", &hook.path)?;
            let path = path_segments(&hook.path);
            if path.first().is_some_and(|segment| segment == "route") {
                return Err(GuiError::invalid_tree(format!(
                    "RSX route context path {:?} is relative to context.route; use {:?} instead",
                    hook.path,
                    path[1..].join(".")
                )));
            }
            if path_segments_cover(&reserved_id, &path) || path_segments_cover(&path, &reserved_id)
            {
                return Err(GuiError::invalid_tree(format!(
                    "RSX route context path {:?} conflicts with reserved route id at context.route.id",
                    hook.path
                )));
            }
            if let Some(existing) = registered.iter().find(|registered| {
                path_segments_cover(registered, &path) || path_segments_cover(&path, registered)
            }) {
                return Err(GuiError::invalid_tree(format!(
                    "RSX route context path {:?} conflicts with {:?}",
                    hook.path,
                    existing.join(".")
                )));
            }
            registered.push(path);
        }
        Ok(())
    }

    fn run_route_effect_hooks(
        &self,
        state: &mut S,
        from: &str,
        to: &str,
        invocation: &ActionInvocation,
    ) -> GuiResult<()> {
        let transition = RsxRouteTransition::new(from, to, invocation);
        for hook in &self.route_effect_hooks {
            hook.run(state, &transition)?;
        }
        Ok(())
    }

    fn active_route_id(&self, state: &S) -> GuiResult<String> {
        let route = (self.route_selector)(state)?;
        let route = if route.trim().is_empty() {
            self.default_route.as_deref().unwrap_or(route.as_str())
        } else {
            route.as_str()
        };
        if self.routes.contains_key(route) {
            return Ok(route.to_string());
        }
        if let Some(default_route) = self.default_route.as_deref() {
            if self.routes.contains_key(default_route) {
                return Ok(default_route.to_string());
            }
            return Err(GuiError::invalid_tree(format!(
                "RSX router default route {default_route:?} is not registered"
            )));
        }
        Err(GuiError::invalid_tree(format!(
            "RSX route {route:?} is not registered"
        )))
    }
}

fn validate_route_layout_outlet(root: &CompiledRsxNode) -> GuiResult<()> {
    let count = count_route_layout_outlets(root);
    match count {
        1 => Ok(()),
        0 => Err(GuiError::invalid_tree(
            "RSX router layout needs exactly one <Slot /> or <Slot name=\"route\" /> outlet",
        )),
        _ => Err(GuiError::invalid_tree(
            "RSX router layout cannot contain more than one route outlet",
        )),
    }
}

fn count_route_layout_outlets(node: &CompiledRsxNode) -> usize {
    match node {
        CompiledRsxNode::Text { .. } => 0,
        CompiledRsxNode::Element {
            tag,
            props,
            children,
            ..
        } => {
            let current = usize::from(is_route_layout_outlet(tag, props));
            current
                + children
                    .iter()
                    .map(count_route_layout_outlets)
                    .sum::<usize>()
        }
    }
}

fn splice_route_layout_outlet(
    layout_root: CompiledRsxNode,
    route_root: CompiledRsxNode,
) -> GuiResult<CompiledRsxNode> {
    let mut replaced = false;
    let mut nodes = splice_route_layout_outlet_node(layout_root, &route_root, &mut replaced)?;
    if !replaced {
        return Err(GuiError::invalid_tree(
            "RSX router layout needs exactly one <Slot /> or <Slot name=\"route\" /> outlet",
        ));
    }
    match nodes.len() {
        0 => Err(GuiError::invalid_tree(
            "RSX router layout route outlet resolved to no nodes",
        )),
        1 => Ok(nodes.remove(0)),
        _ => Ok(CompiledRsxNode::Element {
            key: "layout".to_string(),
            tag: "Fragment".to_string(),
            import_source: None,
            props: CompiledProps::default(),
            children: nodes,
        }),
    }
}

fn splice_route_layout_outlet_node(
    node: CompiledRsxNode,
    route_root: &CompiledRsxNode,
    replaced: &mut bool,
) -> GuiResult<Vec<CompiledRsxNode>> {
    match node {
        CompiledRsxNode::Text { .. } => Ok(vec![node]),
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => {
            if is_route_layout_outlet(&tag, &props) {
                if *replaced {
                    return Err(GuiError::invalid_tree(
                        "RSX router layout cannot contain more than one route outlet",
                    ));
                }
                *replaced = true;
                return Ok(route_layout_outlet_nodes(route_root.clone(), &key));
            }

            let children = children
                .into_iter()
                .map(|child| splice_route_layout_outlet_node(child, route_root, replaced))
                .collect::<GuiResult<Vec<_>>>()?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            Ok(vec![CompiledRsxNode::Element {
                key,
                tag,
                import_source,
                props,
                children,
            }])
        }
    }
}

fn is_route_layout_outlet(tag: &str, props: &CompiledProps) -> bool {
    if !matches!(tag, "Slot" | "slot") {
        return false;
    }
    props
        .name
        .as_deref()
        .is_none_or(|name| name.is_empty() || name == "route")
}

fn route_layout_outlet_nodes(route_root: CompiledRsxNode, prefix: &str) -> Vec<CompiledRsxNode> {
    match route_root {
        CompiledRsxNode::Element { tag, children, .. } if tag == "Fragment" => children
            .into_iter()
            .map(|child| prefix_compiled_node_keys(child, prefix))
            .collect(),
        node => vec![prefix_compiled_node_keys(node, prefix)],
    }
}

fn prefix_compiled_node_keys(node: CompiledRsxNode, prefix: &str) -> CompiledRsxNode {
    match node {
        CompiledRsxNode::Text { key, value } => CompiledRsxNode::Text {
            key: format!("{prefix}-{key}"),
            value,
        },
        CompiledRsxNode::Element {
            key,
            tag,
            import_source,
            props,
            children,
        } => CompiledRsxNode::Element {
            key: format!("{prefix}-{key}"),
            tag,
            import_source,
            props,
            children: children
                .into_iter()
                .map(|child| prefix_compiled_node_keys(child, prefix))
                .collect(),
        },
    }
}

fn merge_router_actions(
    layout_actions: Vec<UiAction>,
    route_actions: Vec<UiAction>,
) -> GuiResult<Vec<UiAction>> {
    let mut seen = BTreeSet::new();
    let mut actions = Vec::new();
    for action in layout_actions.into_iter().chain(route_actions) {
        if !seen.insert(action.id.clone()) {
            return Err(GuiError::invalid_tree(format!(
                "RSX router layout and active route both registered action {:?}",
                action.id
            )));
        }
        actions.push(action);
    }
    Ok(actions)
}

fn validate_hook_path(kind: &str, path: &str) -> GuiResult<()> {
    if path.split('.').any(|segment| segment.trim().is_empty()) {
        return Err(GuiError::invalid_tree(format!(
            "RSX {kind} hook path {path:?} must contain non-empty dot-separated segments"
        )));
    }
    Ok(())
}

fn validate_route_id(id: &str) -> GuiResult<()> {
    if id.trim().is_empty() || id.chars().any(char::is_whitespace) {
        return Err(GuiError::invalid_tree(format!(
            "RSX route id {id:?} must be non-empty and contain no whitespace"
        )));
    }
    Ok(())
}

fn normalize_rsx_source_name(source_name: impl AsRef<str>) -> String {
    let source_name = source_name.as_ref().trim();
    if source_name.is_empty() {
        "inline.rsx".to_string()
    } else {
        source_name.to_string()
    }
}

fn with_optional_source_context(source_name: Option<&str>, error: GuiError) -> GuiError {
    let Some(source_name) = source_name else {
        return error;
    };

    match error {
        GuiError::InvalidTree { message } if message.contains(source_name) => {
            GuiError::invalid_tree(message)
        }
        GuiError::InvalidTree { message } => {
            GuiError::invalid_tree(format!("RSX source {source_name:?}: {message}"))
        }
        other => other,
    }
}

struct BindingHookCoverage {
    state: Vec<Vec<String>>,
    props: Vec<Vec<String>>,
    derived: Vec<Vec<String>>,
    context: Vec<Vec<String>>,
    resource: Vec<Vec<String>>,
}

impl BindingHookCoverage {
    fn new<S>(
        state: &[RsxValueHook<S>],
        props: &[RsxValueHook<S>],
        derived: &[RsxValueHook<S>],
        context: &[RsxValueHook<S>],
        resource: &[RsxValueHook<S>],
        extra_context: &[Vec<String>],
    ) -> Self {
        let mut context = hook_paths(context);
        context.extend(extra_context.iter().cloned());
        Self {
            state: hook_paths(state),
            props: hook_paths(props),
            derived: hook_paths(derived),
            context,
            resource: hook_paths(resource),
        }
    }

    fn validate_hook_binding(&self, kind: &str, binding: &CompiledBinding) -> GuiResult<()> {
        let paths = match kind {
            "state" => &self.state,
            "props" => &self.props,
            "derived" => &self.derived,
            "context" => &self.context,
            "resource" => &self.resource,
            _ => {
                return Err(GuiError::invalid_tree(format!(
                    "RSX binding kind {kind:?} is not supported"
                )))
            }
        };
        if paths
            .iter()
            .any(|hook_path| path_segments_cover(hook_path, &binding.path))
        {
            return Ok(());
        }

        Err(GuiError::invalid_tree(format!(
            "RSX binding {} has no {kind} hook; add {} for {:?}",
            binding.display_path(),
            hook_registration_hint(kind),
            binding.path.join(".")
        )))
    }
}

fn hook_paths<S>(hooks: &[RsxValueHook<S>]) -> Vec<Vec<String>> {
    hooks.iter().map(|hook| path_segments(&hook.path)).collect()
}

fn prop_set_covers_path(props: &BTreeSet<String>, path: &[String]) -> bool {
    if path.is_empty() {
        return false;
    }
    let root = canonical_component_prop_name(path[0].clone());
    if props.contains(&root) {
        return true;
    }
    props
        .iter()
        .map(|prop| path_segments(prop))
        .any(|prop_path| path_segments_cover(&prop_path, path))
}

fn path_segments(path: &str) -> Vec<String> {
    path.split('.').map(str::to_string).collect()
}

fn route_context_scope_path(path: &str) -> String {
    format!("route.{path}")
}

fn route_context_segments(path: &str) -> Vec<String> {
    let mut segments = vec!["route".to_string()];
    segments.extend(path_segments(path));
    segments
}

fn path_segments_cover(registered: &[String], binding: &[String]) -> bool {
    !registered.is_empty()
        && registered.len() <= binding.len()
        && registered
            .iter()
            .zip(binding.iter())
            .all(|(registered, binding)| registered == binding)
}

fn hook_registration_hint(kind: &str) -> &'static str {
    match kind {
        "state" => {
            "use_selector, use_state, use_selector_result, use_state_result, use_selector_value, or use_state_value"
        }
        "props" => "use_prop, use_prop_result, or use_prop_value",
        "derived" => {
            "use_derived, use_derived_result, use_memo, use_memo_result, or use_derived_value"
        }
        "context" => "use_context, use_context_result, or use_context_value",
        "resource" => "use_resource or use_resource_result",
        _ => "a matching hook",
    }
}

fn validate_component_prop_name(component: &str, prop: &str) -> GuiResult<()> {
    if prop.trim().is_empty() || prop.chars().any(char::is_whitespace) {
        return Err(GuiError::invalid_tree(format!(
            "RSX component {component:?} prop name {prop:?} must be non-empty and contain no whitespace"
        )));
    }
    Ok(())
}

fn component_invocation_prop_names(props: &CompiledProps) -> BTreeSet<String> {
    let mut names = BTreeSet::new();

    names.extend(props.explicit_props.iter().cloned());
    names.extend(
        props
            .bindings
            .keys()
            .cloned()
            .map(canonical_component_prop_name),
    );
    names.extend(
        props
            .attributes
            .keys()
            .cloned()
            .map(canonical_component_prop_name),
    );
    names.extend(
        props
            .events
            .keys()
            .cloned()
            .map(canonical_component_prop_name),
    );

    insert_optional_prop(&mut names, "label", props.label.as_ref());
    insert_optional_prop(&mut names, "textValue", props.text_value.as_ref());
    insert_optional_prop(&mut names, "value", props.value.as_ref());
    insert_optional_prop(&mut names, "placeholder", props.placeholder.as_ref());
    insert_optional_prop(&mut names, "action", props.action.as_ref());
    insert_optional_prop(&mut names, "aria-label", props.aria_label.as_ref());
    insert_optional_prop(&mut names, "id", props.id.as_ref());
    insert_optional_prop(&mut names, "name", props.name.as_ref());
    insert_optional_prop(&mut names, "form", props.form.as_ref());
    insert_optional_prop(&mut names, "type", props.input_type.as_ref());
    insert_optional_prop(&mut names, "className", props.class_name.as_ref());

    if !props.style.is_empty() {
        names.insert("style".to_string());
    }
    if props.is_disabled {
        names.insert("isDisabled".to_string());
    }
    if props.is_required {
        names.insert("isRequired".to_string());
    }
    if props.is_invalid {
        names.insert("isInvalid".to_string());
    }
    if props.is_read_only {
        names.insert("isReadOnly".to_string());
    }
    if props.is_selected {
        names.insert("isSelected".to_string());
    }
    if props.is_checked.is_some() {
        names.insert("isChecked".to_string());
    }
    if props.is_expanded.is_some() {
        names.insert("isExpanded".to_string());
    }
    if props.orientation.is_some() {
        names.insert("orientation".to_string());
    }
    if props.min_value.is_some() {
        names.insert("minValue".to_string());
    }
    if props.max_value.is_some() {
        names.insert("maxValue".to_string());
    }
    if props.step_value.is_some() {
        names.insert("stepValue".to_string());
    }
    if props.value_number.is_some() {
        names.insert("valueNumber".to_string());
    }

    names
}

fn insert_optional_prop(names: &mut BTreeSet<String>, prop: &str, value: Option<&String>) {
    if value.is_some() {
        names.insert(canonical_component_prop_name(prop.to_string()));
    }
}

fn canonical_component_prop_name(prop: impl Into<String>) -> String {
    let prop = prop.into();
    match prop.as_str() {
        "class" | "className" => "className".to_string(),
        "aria-label" | "ariaLabel" => "aria-label".to_string(),
        "disabled" | "isDisabled" => "isDisabled".to_string(),
        "required" | "isRequired" => "isRequired".to_string(),
        "invalid" | "isInvalid" => "isInvalid".to_string(),
        "readOnly" | "readonly" | "isReadOnly" => "isReadOnly".to_string(),
        "selected" | "isSelected" => "isSelected".to_string(),
        "checked" | "isChecked" => "isChecked".to_string(),
        "expanded" | "isExpanded" => "isExpanded".to_string(),
        "min" | "minValue" => "minValue".to_string(),
        "max" | "maxValue" => "maxValue".to_string(),
        "step" | "stepValue" => "stepValue".to_string(),
        "type" | "inputType" => "inputType".to_string(),
        "onclick" => "onClick".to_string(),
        "onpress" => "onPress".to_string(),
        "onpressstart" => "onPressStart".to_string(),
        "onpressend" => "onPressEnd".to_string(),
        "onpressup" => "onPressUp".to_string(),
        "onchange" => "onChange".to_string(),
        "oninput" => "onInput".to_string(),
        "onselectionchange" => "onSelectionChange".to_string(),
        "onfocus" => "onFocus".to_string(),
        "onblur" => "onBlur".to_string(),
        "onfocuschange" => "onFocusChange".to_string(),
        "onfocuswithin" => "onFocusWithin".to_string(),
        "onblurwithin" => "onBlurWithin".to_string(),
        "onfocuswithinchange" => "onFocusWithinChange".to_string(),
        "ontoggle" => "onToggle".to_string(),
        "onexpandedchange" => "onExpandedChange".to_string(),
        "onhoverstart" => "onHoverStart".to_string(),
        "onhoverend" => "onHoverEnd".to_string(),
        "onhoverchange" => "onHoverChange".to_string(),
        "onkeydown" => "onKeyDown".to_string(),
        "onkeyup" => "onKeyUp".to_string(),
        "oncopy" => "onCopy".to_string(),
        "oncut" => "onCut".to_string(),
        "onpaste" => "onPaste".to_string(),
        _ => prop,
    }
}

fn collect_static_action_ids(
    node: &CompiledRsxNode,
    components: &BTreeMap<String, CompiledRsxNode>,
    component_stack: &mut Vec<String>,
    actions: &mut BTreeSet<String>,
) -> GuiResult<()> {
    let CompiledRsxNode::Element {
        tag,
        props,
        children,
        ..
    } = node
    else {
        return Ok(());
    };

    actions.extend(props.events.values().filter(|id| !id.is_empty()).cloned());

    if let Some(component) = components.get(tag) {
        if component_stack.iter().any(|name| name == tag) {
            let mut cycle = component_stack.clone();
            cycle.push(tag.clone());
            return Err(GuiError::invalid_tree(format!(
                "RSX component cycle detected: {}",
                cycle.join(" -> ")
            )));
        }
        component_stack.push(tag.clone());
        collect_static_action_ids(component, components, component_stack, actions)?;
        component_stack.pop();
    }

    for child in children {
        collect_static_action_ids(child, components, component_stack, actions)?;
    }

    Ok(())
}

fn validate_component_name(name: &str) -> GuiResult<()> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(GuiError::invalid_tree(
            "RSX component name must not be empty",
        ));
    };
    if !first.is_ascii_uppercase()
        || !chars.all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
        || matches!(name, "For" | "Each" | "Show" | "When" | "Slot" | "Fragment")
    {
        return Err(GuiError::invalid_tree(format!(
            "RSX component name {name:?} must be a PascalCase identifier that does not shadow native controls"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
