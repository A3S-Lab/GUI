use std::any::Any;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::error::{GuiError, GuiResult};
use crate::event::ActionInvocation;

use super::{RsxActionTransition, RsxRouteTransition};

pub(super) struct RsxValueHook<S> {
    pub(super) path: String,
    pub(super) selector: Box<dyn Fn(&S) -> GuiResult<JsonValue> + Send + Sync>,
}

impl<S> RsxValueHook<S> {
    pub(super) fn serializing<T, F>(
        kind: &'static str,
        path: impl Into<String>,
        selector: F,
    ) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        let context = path.clone();
        Self {
            path,
            selector: Box::new(move |state| {
                serde_json::to_value(selector(state)).map_err(|error| {
                    GuiError::invalid_tree(format!(
                        "RSX {kind} hook {context:?} did not serialize: {error}"
                    ))
                })
            }),
        }
    }

    pub(super) fn serializing_result<T, F>(
        kind: &'static str,
        path: impl Into<String>,
        selector: F,
    ) -> Self
    where
        T: Serialize,
        F: Fn(&S) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        let context = path.clone();
        Self {
            path,
            selector: Box::new(move |state| {
                let value = selector(state)?;
                serde_json::to_value(value).map_err(|error| {
                    GuiError::invalid_tree(format!(
                        "RSX {kind} hook {context:?} did not serialize: {error}"
                    ))
                })
            }),
        }
    }

    pub(super) fn value(
        path: impl Into<String>,
        selector: impl Fn(&S) -> GuiResult<JsonValue> + Send + Sync + 'static,
    ) -> Self {
        Self {
            path: path.into(),
            selector: Box::new(selector),
        }
    }
}

pub(super) struct RsxRouteContextHook<S> {
    pub(super) path: String,
    selector: Box<dyn Fn(&S, &str) -> GuiResult<JsonValue> + Send + Sync>,
}

impl<S> RsxRouteContextHook<S> {
    pub(super) fn serializing<T, F>(path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S, &str) -> T + Send + Sync + 'static,
    {
        let path = path.into();
        let context = path.clone();
        Self {
            path,
            selector: Box::new(move |state, route| {
                serde_json::to_value(selector(state, route)).map_err(|error| {
                    GuiError::invalid_tree(format!(
                        "RSX route context hook {context:?} did not serialize: {error}"
                    ))
                })
            }),
        }
    }

    pub(super) fn serializing_result<T, F>(path: impl Into<String>, selector: F) -> Self
    where
        T: Serialize,
        F: Fn(&S, &str) -> GuiResult<T> + Send + Sync + 'static,
    {
        let path = path.into();
        let context = path.clone();
        Self {
            path,
            selector: Box::new(move |state, route| {
                let value = selector(state, route)?;
                serde_json::to_value(value).map_err(|error| {
                    GuiError::invalid_tree(format!(
                        "RSX route context hook {context:?} did not serialize: {error}"
                    ))
                })
            }),
        }
    }

    pub(super) fn value(
        path: impl Into<String>,
        selector: impl Fn(&S, &str) -> GuiResult<JsonValue> + Send + Sync + 'static,
    ) -> Self {
        Self {
            path: path.into(),
            selector: Box::new(selector),
        }
    }

    pub(super) fn select(&self, state: &S, route: &str) -> GuiResult<JsonValue> {
        (self.selector)(state, route)
    }
}

pub(super) struct RsxActionHook<S> {
    label: Option<String>,
    reducer: Box<dyn Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync>,
}

impl<S> RsxActionHook<S> {
    pub(super) fn new(
        label: Option<String>,
        reducer: impl Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    ) -> Self {
        Self {
            label,
            reducer: Box::new(reducer),
        }
    }

    pub(super) fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    pub(super) fn reduce(&self, state: &mut S, invocation: &ActionInvocation) -> GuiResult<()> {
        (self.reducer)(state, invocation)
    }
}

pub(super) struct RsxActionDisabledHook<S> {
    pub(super) action: String,
    selector: Box<dyn Fn(&S) -> GuiResult<bool> + Send + Sync>,
}

impl<S> RsxActionDisabledHook<S> {
    pub(super) fn disabled(
        action: impl Into<String>,
        selector: impl Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    ) -> Self {
        Self {
            action: action.into(),
            selector: Box::new(selector),
        }
    }

    pub(super) fn enabled(
        action: impl Into<String>,
        selector: impl Fn(&S) -> GuiResult<bool> + Send + Sync + 'static,
    ) -> Self {
        Self::disabled(action, move |state| selector(state).map(|enabled| !enabled))
    }

    pub(super) fn is_disabled(&self, state: &S) -> GuiResult<bool> {
        (self.selector)(state)
    }
}

pub(super) struct RsxEffectCapture {
    before: Box<dyn Any>,
}

enum RsxEffectKind<S> {
    Invocation(Box<dyn Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync>),
    Transition {
        capture: Box<dyn Fn(&S) -> Box<dyn Any> + Send + Sync>,
        effect: Box<dyn Fn(&mut S, &ActionInvocation, &dyn Any) -> GuiResult<()> + Send + Sync>,
    },
}

pub(super) struct RsxEffectHook<S> {
    action: Option<String>,
    kind: RsxEffectKind<S>,
}

impl<S> RsxEffectHook<S> {
    pub(super) fn global(
        effect: impl Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    ) -> Self {
        Self {
            action: None,
            kind: RsxEffectKind::Invocation(Box::new(effect)),
        }
    }

    pub(super) fn for_action(
        action: impl Into<String>,
        effect: impl Fn(&mut S, &ActionInvocation) -> GuiResult<()> + Send + Sync + 'static,
    ) -> Self {
        Self {
            action: Some(action.into()),
            kind: RsxEffectKind::Invocation(Box::new(effect)),
        }
    }

    pub(super) fn transition_global(
        effect: impl for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        S: Clone + 'static,
    {
        Self {
            action: None,
            kind: Self::transition_kind(effect),
        }
    }

    pub(super) fn transition_for_action(
        action: impl Into<String>,
        effect: impl for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    ) -> Self
    where
        S: Clone + 'static,
    {
        Self {
            action: Some(action.into()),
            kind: Self::transition_kind(effect),
        }
    }

    fn transition_kind(
        effect: impl for<'a> Fn(&mut S, &RsxActionTransition<'a, S>) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    ) -> RsxEffectKind<S>
    where
        S: Clone + 'static,
    {
        RsxEffectKind::Transition {
            capture: Box::new(|state| Box::new(state.clone())),
            effect: Box::new(move |state, invocation, before| {
                let Some(before) = before.downcast_ref::<S>() else {
                    return Err(GuiError::invalid_tree(
                        "RSX transition effect snapshot type did not match component state",
                    ));
                };
                let transition = RsxActionTransition::new(before, invocation);
                effect(state, &transition)
            }),
        }
    }

    pub(super) fn capture_if_matches(
        &self,
        state: &S,
        invocation: &ActionInvocation,
    ) -> Option<RsxEffectCapture> {
        if !self.matches(invocation) {
            return None;
        }
        match &self.kind {
            RsxEffectKind::Invocation(_) => None,
            RsxEffectKind::Transition { capture, .. } => Some(RsxEffectCapture {
                before: capture(state),
            }),
        }
    }

    pub(super) fn run_if_matches(
        &self,
        state: &mut S,
        invocation: &ActionInvocation,
        capture: Option<&RsxEffectCapture>,
    ) -> GuiResult<()> {
        if !self.matches(invocation) {
            return Ok(());
        }
        match &self.kind {
            RsxEffectKind::Invocation(effect) => effect(state, invocation),
            RsxEffectKind::Transition { effect, .. } => {
                let Some(capture) = capture else {
                    return Err(GuiError::invalid_tree(
                        "RSX transition effect did not capture a before-state snapshot",
                    ));
                };
                effect(state, invocation, capture.before.as_ref())
            }
        }
    }

    pub(super) fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    fn matches(&self, invocation: &ActionInvocation) -> bool {
        self.action
            .as_ref()
            .is_none_or(|action| action == &invocation.action)
    }
}

pub(super) struct RsxMountHook<S> {
    hook: Box<dyn Fn(&mut S) -> GuiResult<()> + Send + Sync>,
}

impl<S> RsxMountHook<S> {
    pub(super) fn new(hook: impl Fn(&mut S) + Send + Sync + 'static) -> Self {
        Self {
            hook: Box::new(move |state| {
                hook(state);
                Ok(())
            }),
        }
    }

    pub(super) fn result(hook: impl Fn(&mut S) -> GuiResult<()> + Send + Sync + 'static) -> Self {
        Self {
            hook: Box::new(hook),
        }
    }

    pub(super) fn run(&self, state: &mut S) -> GuiResult<()> {
        (self.hook)(state)
    }
}

pub(super) type RsxUnmountHook<S> = RsxMountHook<S>;

pub(super) struct RsxRouteEffectHook<S> {
    effect: Box<dyn for<'a> Fn(&mut S, &RsxRouteTransition<'a>) -> GuiResult<()> + Send + Sync>,
}

impl<S> RsxRouteEffectHook<S> {
    pub(super) fn new(
        effect: impl for<'a> Fn(&mut S, &RsxRouteTransition<'a>) -> GuiResult<()>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self {
            effect: Box::new(effect),
        }
    }

    pub(super) fn run(&self, state: &mut S, transition: &RsxRouteTransition<'_>) -> GuiResult<()> {
        (self.effect)(state, transition)
    }
}

pub(super) fn decode_action_value<T>(invocation: &ActionInvocation, action: &str) -> GuiResult<T>
where
    T: DeserializeOwned,
{
    let Some(raw) = invocation.value() else {
        return Err(GuiError::host(format!(
            "RSX action {action:?} expected value {}",
            std::any::type_name::<T>()
        )));
    };

    serde_json::from_str(raw).or_else(|json_error| {
        serde_json::from_value(JsonValue::String(raw.to_string())).map_err(|string_error| {
            GuiError::host(format!(
                "RSX action {action:?} value did not decode as {}: {json_error}; string fallback failed: {string_error}",
                std::any::type_name::<T>()
            ))
        })
    })
}

pub(super) fn decode_action_payload<T>(invocation: &ActionInvocation, action: &str) -> GuiResult<T>
where
    T: DeserializeOwned,
{
    let Some(payload) = invocation.payload::<T>()? else {
        return Err(GuiError::host(format!(
            "RSX action {action:?} expected payload {}",
            std::any::type_name::<T>()
        )));
    };
    Ok(payload)
}

pub(super) fn insert_scope_value(
    root: &mut JsonMap<String, JsonValue>,
    path: &str,
    value: JsonValue,
) -> GuiResult<()> {
    let segments = path.split('.').collect::<Vec<_>>();
    if segments.is_empty() || segments.iter().any(|segment| segment.trim().is_empty()) {
        return Err(GuiError::invalid_tree(format!(
            "RSX hook path {path:?} must contain non-empty dot-separated segments"
        )));
    }

    let mut object = root;
    for segment in &segments[..segments.len() - 1] {
        let entry = object
            .entry((*segment).to_string())
            .or_insert_with(|| JsonValue::Object(JsonMap::new()));
        let JsonValue::Object(next) = entry else {
            return Err(GuiError::invalid_tree(format!(
                "RSX hook path {path:?} conflicts with scalar segment {segment:?}"
            )));
        };
        object = next;
    }

    let leaf = segments[segments.len() - 1];
    if object.insert(leaf.to_string(), value).is_some() {
        return Err(GuiError::invalid_tree(format!(
            "RSX hook path {path:?} was registered more than once"
        )));
    }
    Ok(())
}
