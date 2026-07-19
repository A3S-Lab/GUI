use super::{apply_widget_setter, NativeWidgetConfig, NativeWidgetSetter};
use crate::native::NativeRole;
use crate::platform::types::{NativeBackendKind, NativeWidgetKind};

/// A compact description of an identity change that cannot be applied with
/// setters and therefore requires the renderer to replace the native widget.
#[derive(Debug, Clone, PartialEq)]
pub struct NativeWidgetReplacement {
    pub backend: Option<NativeConfigValueChange<NativeBackendKind>>,
    pub widget_kind: Option<NativeConfigValueChange<NativeWidgetKind>>,
    pub role: Option<NativeConfigValueChange<NativeRole>>,
}

impl NativeWidgetReplacement {
    fn between(before: &NativeWidgetConfig, after: &NativeWidgetConfig) -> Option<Self> {
        let replacement = Self {
            backend: diff_value(&before.backend, &after.backend),
            widget_kind: diff_value(&before.widget_kind, &after.widget_kind),
            role: diff_value(&before.role, &after.role),
        };
        (replacement.backend.is_some()
            || replacement.widget_kind.is_some()
            || replacement.role.is_some())
        .then_some(replacement)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NativeConfigValueChange<T> {
    pub before: T,
    pub after: T,
}

/// Ordered, typed native setter batch.
///
/// This is the only update representation between a resolved config and a
/// platform surface. It intentionally replaces the former field-for-field
/// patch mirror. The compatibility alias [`NativeWidgetConfigPatch`] remains
/// available while callers migrate to the clearer batch name.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct NativeWidgetSetterBatch {
    setters: Vec<NativeWidgetSetter>,
    replacement: Option<NativeWidgetReplacement>,
}

/// Compatibility name for the pre-batch API.
pub type NativeWidgetConfigPatch = NativeWidgetSetterBatch;

impl NativeWidgetSetterBatch {
    pub fn between(before: &NativeWidgetConfig, after: &NativeWidgetConfig) -> Self {
        let setters = after
            .create_setters()
            .into_iter()
            .filter(|setter| setter.differs_from(before))
            .collect();

        Self {
            setters,
            replacement: NativeWidgetReplacement::between(before, after),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.setters.is_empty() && self.replacement.is_none()
    }

    pub fn setters(&self) -> Vec<NativeWidgetSetter> {
        self.setters.clone()
    }

    pub fn as_setters(&self) -> &[NativeWidgetSetter] {
        &self.setters
    }

    pub fn into_setters(self) -> Vec<NativeWidgetSetter> {
        self.setters
    }

    pub fn replacement(&self) -> Option<&NativeWidgetReplacement> {
        self.replacement.as_ref()
    }

    pub fn requires_replacement(&self) -> bool {
        self.replacement.is_some()
    }

    /// Replays the batch into the in-memory resolved config.
    ///
    /// Identity changes are copied as a unit because they are consumed by a
    /// recreate path, never by native setters.
    pub fn replay(&self, config: &mut NativeWidgetConfig) {
        for setter in &self.setters {
            apply_widget_setter(config, setter);
        }
        if let Some(replacement) = &self.replacement {
            if let Some(change) = &replacement.backend {
                config.backend = change.after;
            }
            if let Some(change) = &replacement.widget_kind {
                config.widget_kind = change.after;
            }
            if let Some(change) = &replacement.role {
                config.role = change.after;
            }
        }
    }
}

fn diff_value<T: Clone + PartialEq>(before: &T, after: &T) -> Option<NativeConfigValueChange<T>> {
    (before != after).then(|| NativeConfigValueChange {
        before: before.clone(),
        after: after.clone(),
    })
}
