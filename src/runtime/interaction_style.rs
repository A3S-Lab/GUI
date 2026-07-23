use std::collections::{BTreeMap, BTreeSet};

use super::GuiRuntime;
use crate::error::{GuiError, GuiResult};
use crate::host::{HostNodeId, NativeHost};
use crate::interaction::InteractionNodeState;
use crate::native::NativeProps;
use crate::style::{variant_segments, PortableStyle};

impl<H: NativeHost> GuiRuntime<H> {
    pub(super) fn invalidate_interaction_style_projections(
        &mut self,
        previous_props: &BTreeMap<HostNodeId, NativeProps>,
    ) {
        let current_props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        self.projected_interaction_styles
            .retain(|node, _| current_props.contains_key(node));
        for (node, props) in &current_props {
            if previous_props.get(node) != Some(props) {
                self.projected_interaction_styles.remove(node);
            }
        }
    }

    pub(super) fn project_all_interaction_styles(&mut self) -> GuiResult<()> {
        let nodes = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .map(|(node, _)| node)
            .collect::<BTreeSet<_>>();
        self.project_interaction_styles(nodes)
    }

    pub(super) fn project_interaction_style_nodes<I>(&mut self, nodes: I) -> GuiResult<()>
    where
        I: IntoIterator<Item = HostNodeId>,
    {
        self.project_interaction_styles(nodes.into_iter().collect())
    }

    fn project_interaction_styles(&mut self, nodes: BTreeSet<HostNodeId>) -> GuiResult<()> {
        if nodes.is_empty() {
            return Ok(());
        }
        let props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        let mut cache_only = Vec::new();
        let mut updates = Vec::new();

        for node in nodes {
            let Some(props) = props.get(&node) else {
                self.projected_interaction_styles.remove(&node);
                continue;
            };
            let source = PortableStyle::from_web(&props.web);
            if source.variant_declarations.is_empty() {
                self.projected_interaction_styles.remove(&node);
                continue;
            }
            let state = self.interaction_style_state(node, props);
            let focus_visible_within =
                state.focus_within && self.interaction_state.input_modality().shows_focus_ring();
            let active = source
                .variant_declarations
                .keys()
                .filter(|variant| variant_is_active(variant, props, &state, focus_visible_within))
                .cloned()
                .collect::<Vec<_>>();
            let resolved = source.resolve_active_variants(active);

            match self.projected_interaction_styles.get(&node) {
                Some(previous) if *previous == resolved => continue,
                Some(_) => updates.push((node, resolved)),
                None if resolved == source => cache_only.push((node, resolved)),
                None => updates.push((node, resolved)),
            }
        }

        if !updates.is_empty() {
            self.host.begin_frame()?;
            for (node, style) in &updates {
                if let Err(error) = self.host.update_portable_style(*node, style) {
                    let rollback = self.host.rollback_frame();
                    return Err(with_rollback_context(error, rollback));
                }
            }
            if let Err(error) = self.host.commit_frame() {
                let rollback = self.host.rollback_frame();
                return Err(with_rollback_context(error, rollback));
            }
        }
        for (node, style) in cache_only.into_iter().chain(updates) {
            self.projected_interaction_styles.insert(node, style);
        }
        Ok(())
    }

    fn interaction_style_state(
        &self,
        node: HostNodeId,
        props: &NativeProps,
    ) -> InteractionNodeState {
        let previous = self.interaction_state.node(node);
        if self.interaction_revisions.get(&node).copied() == Some(self.render_revision) {
            return previous.cloned().unwrap_or_else(|| initial_state(props));
        }

        let mut state = initial_state(props);
        if let Some(previous) = previous {
            state.focused = previous.focused;
            state.focus_visible = previous.focus_visible;
            state.focus_within = previous.focus_within;
            state.pressed = previous.pressed;
            state.long_pressed = previous.long_pressed;
            state.moving = previous.moving;
            state.x_delta = previous.x_delta;
            state.y_delta = previous.y_delta;
            state.hovered = previous.hovered;
        }
        state
    }

    pub(super) fn include_focus_within_style_ancestors(&self, nodes: &mut BTreeSet<HostNodeId>) {
        let changed = nodes.iter().copied().collect::<Vec<_>>();
        let props = self
            .renderer
            .mounted_node_props()
            .into_iter()
            .collect::<BTreeMap<_, _>>();
        for node in changed {
            for ancestor in self.renderer.ancestor_ids(node) {
                if props.get(&ancestor).is_some_and(|props| {
                    PortableStyle::from_web(&props.web)
                        .interaction_requirements()
                        .focus_within
                }) {
                    nodes.insert(ancestor);
                }
            }
        }
    }
}

fn with_rollback_context(error: GuiError, rollback: GuiResult<()>) -> GuiError {
    match rollback {
        Ok(()) => error,
        Err(rollback_error) => GuiError::host(format!(
            "{error}; interaction style rollback also failed: {rollback_error}"
        )),
    }
}

fn initial_state(props: &NativeProps) -> InteractionNodeState {
    InteractionNodeState {
        value: props.value.clone(),
        selected: props.selected,
        checked: props.checked,
        expanded: props.expanded,
        ..InteractionNodeState::default()
    }
}

fn variant_is_active(
    variant: &str,
    props: &NativeProps,
    state: &InteractionNodeState,
    focus_visible_within: bool,
) -> bool {
    variant_segments(variant)
        .into_iter()
        .all(|segment| variant_segment_is_active(segment, props, state, focus_visible_within))
}

fn variant_segment_is_active(
    segment: &str,
    props: &NativeProps,
    state: &InteractionNodeState,
    focus_visible_within: bool,
) -> bool {
    match segment {
        "hover" => state.hovered,
        "active" => state.pressed,
        "focus" => state.focused,
        "focus-visible" => state.focus_visible,
        "focus-within" => state.focus_within,
        "disabled" => props.disabled,
        "enabled" => !props.disabled,
        "checked" => state.checked == Some(true),
        "selected" => state.selected,
        "required" => props.required,
        "optional" => !props.required,
        "invalid" => props.invalid,
        "valid" => !props.invalid,
        "read-only" => props.read_only,
        "read-write" => !props.read_only,
        "open" => state.expanded == Some(true) || props.html_dialog.open == Some(true),
        "placeholder-shown" => props
            .placeholder
            .as_ref()
            .is_some_and(|_| state.value.as_deref().unwrap_or_default().is_empty()),
        "rtl" => props.dir.as_deref() == Some("rtl"),
        "ltr" => props.dir.as_deref() != Some("rtl"),
        _ => data_variant_is_active(segment, props, state, focus_visible_within)
            .or_else(|| aria_variant_is_active(segment, props, state))
            .unwrap_or(false),
    }
}

fn data_variant_is_active(
    segment: &str,
    props: &NativeProps,
    state: &InteractionNodeState,
    focus_visible_within: bool,
) -> Option<bool> {
    let expression = segment
        .strip_prefix("data-[")
        .and_then(|value| value.strip_suffix(']'))?;
    let (name, expected) = expression
        .split_once('=')
        .map_or((expression, None), |(name, value)| (name, Some(value)));
    let actual = data_attribute(name.trim(), props, state, focus_visible_within);
    Some(match expected {
        Some(expected) => actual
            .as_deref()
            .is_some_and(|actual| values_match(actual, expected)),
        None => actual.as_deref().is_some_and(truthy_attribute),
    })
}

fn data_attribute(
    name: &str,
    props: &NativeProps,
    state: &InteractionNodeState,
    focus_visible_within: bool,
) -> Option<String> {
    let runtime = match name {
        "pressed" => Some(state.pressed),
        "long-pressed" => Some(state.long_pressed),
        "moving" => Some(state.moving),
        "hovered" => Some(state.hovered),
        "focused" => Some(state.focused),
        "focus-visible" => Some(state.focus_visible),
        "focus-within" => Some(state.focus_within),
        "focus-visible-within" => Some(focus_visible_within),
        "selected" => Some(state.selected),
        "checked" => state.checked,
        "expanded" | "open" => state.expanded,
        "disabled" => Some(props.disabled),
        "required" => Some(props.required),
        "invalid" => Some(props.invalid),
        "read-only" | "readonly" => Some(props.read_only),
        _ => None,
    };
    if let Some(runtime) = runtime {
        return Some(runtime.to_string());
    }
    let key = format!("data-{name}");
    props
        .web
        .attributes
        .get(&key)
        .or_else(|| props.metadata.get(&key))
        .cloned()
}

fn aria_variant_is_active(
    segment: &str,
    props: &NativeProps,
    state: &InteractionNodeState,
) -> Option<bool> {
    let expression = segment
        .strip_prefix("aria-[")
        .and_then(|value| value.strip_suffix(']'))
        .or_else(|| segment.strip_prefix("aria-"))?;
    let (name, expected) = expression
        .split_once('=')
        .map_or((expression, "true"), |(name, value)| (name, value));
    let actual = match name {
        "disabled" => Some(props.disabled),
        "required" => Some(props.required),
        "invalid" => Some(props.invalid),
        "readonly" | "read-only" => Some(props.read_only),
        "selected" => Some(state.selected),
        "checked" => state.checked,
        "expanded" => state.expanded,
        _ => None,
    };
    Some(actual.is_some_and(|actual| values_match(&actual.to_string(), expected)))
}

fn truthy_attribute(value: &str) -> bool {
    !matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "" | "false" | "0" | "off" | "none"
    )
}

fn values_match(actual: &str, expected: &str) -> bool {
    actual
        .trim()
        .eq_ignore_ascii_case(expected.trim().trim_matches(|ch| matches!(ch, '\'' | '"')))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::WebProps;

    #[test]
    fn compound_variants_require_every_segment() {
        let props = NativeProps::new().web(WebProps::new().attribute("data-variant", "error"));
        let mut state = InteractionNodeState::default();

        assert!(!variant_is_active(
            "data-[variant=error]:focus",
            &props,
            &state,
            false,
        ));
        state.focused = true;
        assert!(variant_is_active(
            "data-[variant=error]:focus",
            &props,
            &state,
            false,
        ));
        assert!(!variant_is_active(
            "dark:data-[variant=error]:focus",
            &props,
            &state,
            false,
        ));
    }
}
