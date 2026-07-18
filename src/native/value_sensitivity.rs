use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Classification applied to text values at process and serialization boundaries.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ValueSensitivity {
    #[default]
    Public,
    Sensitive,
}

impl ValueSensitivity {
    pub fn from_input_type(input_type: Option<&str>) -> Self {
        if input_type.is_some_and(|value| value.trim().eq_ignore_ascii_case("password")) {
            Self::Sensitive
        } else {
            Self::Public
        }
    }

    pub fn is_public(&self) -> bool {
        *self == Self::Public
    }

    pub fn is_sensitive(self) -> bool {
        self == Self::Sensitive
    }

    pub fn redact(self, value: Option<&str>) -> Option<&str> {
        if self.is_sensitive() {
            None
        } else {
            value
        }
    }

    /// Removes metadata fields that can duplicate a control's live value.
    ///
    /// HTML attributes are preserved as metadata for compatibility, so a
    /// password can otherwise escape through `value`, `defaultValue`, ARIA,
    /// or data-value aliases even after the typed value field is redacted.
    pub(crate) fn redact_metadata(self, metadata: &mut BTreeMap<String, String>) {
        if self.is_sensitive() {
            metadata.retain(|name, _| !metadata_key_carries_value(name));
        }
    }

    /// Removes values that must never enter retained state or diagnostic output.
    ///
    /// Unlike wire redaction, this also strips credentials such as CSP nonces
    /// regardless of the control's value sensitivity.
    pub(crate) fn redact_metadata_for_diagnostics(self, metadata: &mut BTreeMap<String, String>) {
        self.redact_metadata(metadata);
        metadata.retain(|name, _| !metadata_key_carries_credential(name));
    }
}

fn metadata_key_carries_value(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "value"
            | "defaultvalue"
            | "default-value"
            | "default_value"
            | "aria-value"
            | "aria-valuenow"
            | "aria-valuetext"
            | "aria_value"
            | "aria_valuenow"
            | "aria_valuetext"
            | "data-value"
            | "data_value"
            | "data-a3s-value"
            | "data_a3s_value"
    )
}

fn metadata_key_carries_credential(name: &str) -> bool {
    let compact = name
        .trim()
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .map(|character| character.to_ascii_lowercase())
        .collect::<String>();
    compact.ends_with("nonce")
        || compact.ends_with("password")
        || compact.ends_with("secret")
        || compact.ends_with("token")
        || compact.ends_with("apikey")
        || compact.ends_with("credential")
        || compact.ends_with("credentials")
        || matches!(
            compact.as_str(),
            "authorization" | "proxyauthorization" | "cookie" | "setcookie"
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_input_types_are_sensitive_case_insensitively() {
        assert_eq!(
            ValueSensitivity::from_input_type(Some(" PASSWORD ")),
            ValueSensitivity::Sensitive
        );
        assert_eq!(
            ValueSensitivity::from_input_type(Some("text")),
            ValueSensitivity::Public
        );
        assert_eq!(
            ValueSensitivity::from_input_type(None),
            ValueSensitivity::Public
        );
    }

    #[test]
    fn redaction_only_removes_sensitive_values() {
        assert_eq!(
            ValueSensitivity::Public.redact(Some("visible")),
            Some("visible")
        );
        assert_eq!(ValueSensitivity::Sensitive.redact(Some("secret")), None);
        assert_eq!(ValueSensitivity::Sensitive.redact(None), None);
    }

    #[test]
    fn sensitive_metadata_redaction_keeps_structure_and_removes_value_aliases() {
        let mut metadata = BTreeMap::from([
            ("type".to_string(), "password".to_string()),
            ("value".to_string(), "secret".to_string()),
            ("defaultValue".to_string(), "secret".to_string()),
            ("aria-valuetext".to_string(), "secret".to_string()),
            ("data-a3s-value".to_string(), "secret".to_string()),
        ]);

        ValueSensitivity::Sensitive.redact_metadata(&mut metadata);

        assert_eq!(metadata.get("type").map(String::as_str), Some("password"));
        assert_eq!(metadata.len(), 1);
    }

    #[test]
    fn diagnostic_metadata_redaction_always_removes_credentials() {
        let mut metadata = BTreeMap::from([
            ("class".to_string(), "module".to_string()),
            ("nonce".to_string(), "csp-secret".to_string()),
            ("data-access-token".to_string(), "bearer-secret".to_string()),
        ]);

        ValueSensitivity::Public.redact_metadata_for_diagnostics(&mut metadata);

        assert_eq!(metadata.get("class").map(String::as_str), Some("module"));
        assert_eq!(metadata.len(), 1);
    }
}
