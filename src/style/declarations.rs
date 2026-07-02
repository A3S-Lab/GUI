use super::*;

impl PortableStyle {
    pub(super) fn record_declaration(&mut self, property: &str, value: &str) {
        if property.starts_with("--") {
            self.custom_properties
                .insert(property.to_string(), value.to_string());
        } else {
            self.declarations
                .insert(property.to_string(), value.to_string());
        }
    }

    pub(super) fn record_variant_declaration(
        &mut self,
        variant: &str,
        property: String,
        value: String,
    ) {
        self.variant_declarations
            .entry(variant.to_string())
            .or_default()
            .insert(property, value);
    }
}
