use std::collections::BTreeMap;
use std::marker::PhantomData;
use std::sync::Arc;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value as JsonValue;

use crate::error::GuiResult;
use crate::event::ActionInvocation;
use crate::semantic_ui::{use_press_value, PressProps, UsePressProps};

use super::{RsxComponent, RsxResource};

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
pub struct ButtonHook {
    pub press_props: PropHandle<PressProps>,
    pub is_pressed: PropHandle<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum BindingAlias {
    State(String),
    Props(String),
    Derived(String),
    Context(String),
    Resource(String),
}

impl BindingAlias {
    fn resolve(&self, suffix: &[String]) -> String {
        let (root, path) = match self {
            Self::State(path) => ("state", path),
            Self::Props(path) => ("props", path),
            Self::Derived(path) => ("derived", path),
            Self::Context(path) => ("context", path),
            Self::Resource(path) => ("resource", path),
        };
        if suffix.is_empty() {
            format!("{root}.{path}")
        } else {
            format!("{root}.{path}.{}", suffix.join("."))
        }
    }
}

pub struct ComponentCx<S> {
    frame_id: String,
    registrations: Vec<ComponentRegistration<S>>,
    aliases: BTreeMap<String, BindingAlias>,
}

impl<S: 'static> ComponentCx<S> {
    pub fn new(frame_id: impl Into<String>) -> Self {
        Self {
            frame_id: frame_id.into(),
            registrations: Vec::new(),
            aliases: BTreeMap::new(),
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

    pub(crate) fn compile_bare<F>(
        frame_id: impl Into<String>,
        render: F,
    ) -> GuiResult<RsxComponent<S>>
    where
        F: FnOnce(&mut Self) -> RSX,
    {
        let mut cx = Self::new(frame_id);
        let view = render(&mut cx);
        cx.into_component_bare(view)
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

    pub fn use_reducer<F>(&mut self, id: impl Into<String>, reducer: F) -> ActionHandle
    where
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        let id = id.into();
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
        let action_id = id.clone();
        self.registrations.push(Box::new(move |component| {
            Ok(component.use_payload_reducer::<T, F>(action_id, reducer))
        }));
        ActionHandle::new(id)
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
        F: Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    {
        self.registrations
            .push(Box::new(move |component| Ok(component.use_effect(effect))));
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

    pub fn use_button<F>(&mut self, selector: F) -> ButtonHook
    where
        F: Fn(&S) -> UsePressProps + Send + Sync + 'static,
    {
        let press = self.use_press(selector);
        ButtonHook {
            press_props: press.press_props,
            is_pressed: press.is_pressed,
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

    fn into_component_bare(self, view: RSX) -> GuiResult<RsxComponent<S>> {
        let source = rewrite_registered_bindings(view.as_source(), &self.aliases);
        let mut component =
            RsxComponent::from_source_bare(self.frame_id, "component-cx.rsx", &source)?;
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

    fn register_alias(&mut self, path: &str, alias: BindingAlias) {
        if let Some(name) = path.rsplit('.').next().filter(|name| !name.is_empty()) {
            self.aliases.entry(name.to_string()).or_insert(alias);
        }
    }
}

fn press_value_part(props: UsePressProps, part: &str) -> GuiResult<JsonValue> {
    let value = use_press_value(props)?;
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
    let rewritten = alias.resolve(&segments[1..]);
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
