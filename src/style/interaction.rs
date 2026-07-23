use std::collections::BTreeSet;

use super::PortableStyle;

/// Native input streams required to keep stateful style variants current.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct InteractionStyleRequirements {
    pub press: bool,
    pub long_press: bool,
    pub movement: bool,
    pub hover: bool,
    pub keyboard_modality: bool,
    pub focus_within: bool,
}

impl PortableStyle {
    /// Applies the declarations for the supplied active variant keys over the
    /// base style while retaining the original variant table for later state
    /// transitions and native event subscription updates.
    pub fn resolve_active_variants<I, S>(&self, active_variants: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: AsRef<str>,
    {
        let active = active_variants
            .into_iter()
            .map(|variant| variant.as_ref().to_string())
            .collect::<BTreeSet<_>>();
        let mut resolved = self.clone();
        let mut applied = BTreeSet::new();

        for (variant, property) in &self.variant_declaration_order {
            if active.contains(variant) {
                apply_variant_property(self, &mut resolved, variant, property);
                applied.insert((variant.as_str(), property.as_str()));
            }
        }
        // Older serialized styles may not carry declaration order. Resolve any
        // remaining active entries deterministically instead of dropping them.
        for (variant, declarations) in &self.variant_declarations {
            if !active.contains(variant) {
                continue;
            }
            for property in declarations.keys() {
                if !applied.contains(&(variant.as_str(), property.as_str())) {
                    apply_variant_property(self, &mut resolved, variant, property);
                }
            }
        }
        resolved
    }

    pub fn interaction_requirements(&self) -> InteractionStyleRequirements {
        let mut requirements = InteractionStyleRequirements::default();
        for variant in self.variant_declarations.keys() {
            for segment in variant_segments(variant) {
                apply_segment_requirement(segment, &mut requirements);
            }
        }
        requirements
    }
}

fn apply_variant_property(
    source: &PortableStyle,
    resolved: &mut PortableStyle,
    variant: &str,
    property: &str,
) {
    let Some(value) = source
        .variant_declarations
        .get(variant)
        .and_then(|declarations| declarations.get(property))
    else {
        return;
    };
    resolved.apply(property, value);
}

pub(crate) fn variant_segments(variant: &str) -> Vec<&str> {
    let mut segments = Vec::new();
    let mut start = 0;
    let mut depth = 0_u32;
    let mut escaped = false;

    for (index, ch) in variant.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match ch {
            '\\' => escaped = true,
            '[' | '(' => depth = depth.saturating_add(1),
            ']' | ')' => depth = depth.saturating_sub(1),
            ':' if depth == 0 => {
                segments.push(&variant[start..index]);
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }
    segments.push(&variant[start..]);
    segments
}

fn apply_segment_requirement(segment: &str, requirements: &mut InteractionStyleRequirements) {
    match segment {
        "hover" => requirements.hover = true,
        "active" => requirements.press = true,
        "focus-visible" => requirements.keyboard_modality = true,
        "focus-within" => requirements.focus_within = true,
        _ => {
            let Some(attribute) = data_variant_attribute(segment) else {
                return;
            };
            match attribute {
                "pressed" => requirements.press = true,
                "long-pressed" => requirements.long_press = true,
                "moving" => requirements.movement = true,
                "hovered" => requirements.hover = true,
                "focus-visible" => requirements.keyboard_modality = true,
                "focus-within" => requirements.focus_within = true,
                "focus-visible-within" => {
                    requirements.keyboard_modality = true;
                    requirements.focus_within = true;
                }
                _ => {}
            }
        }
    }
}

fn data_variant_attribute(segment: &str) -> Option<&str> {
    let expression = segment
        .strip_prefix("data-[")
        .and_then(|value| value.strip_suffix(']'))?;
    let name = expression
        .split_once('=')
        .map_or(expression, |(name, _)| name)
        .trim();
    (!name.is_empty()).then_some(name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_segments_preserve_colons_inside_arbitrary_selectors() {
        assert_eq!(
            variant_segments("data-[value=a:b]:focus-visible"),
            ["data-[value=a:b]", "focus-visible"]
        );
    }
}
