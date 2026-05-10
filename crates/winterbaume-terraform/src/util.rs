//! Utility functions for Terraform resource attribute extraction.

use std::collections::HashMap;

use crate::error::ConversionError;

/// Extract a required string attribute from a JSON value.
///
/// Returns `ConversionError::MissingAttribute` if the key is missing,
/// null, or not a string.
pub fn require_str<'a>(
    attrs: &'a serde_json::Value,
    key: &str,
    resource_type: &str,
) -> Result<&'a str, ConversionError> {
    attrs
        .get(key)
        .and_then(|v| v.as_str())
        .ok_or_else(|| ConversionError::MissingAttribute {
            resource_type: resource_type.to_string(),
            attribute: key.to_string(),
        })
}

/// Extract an optional string attribute from a JSON value.
pub fn optional_str(attrs: &serde_json::Value, key: &str) -> Option<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
}

/// Extract an optional integer attribute from a JSON value.
pub fn optional_i64(attrs: &serde_json::Value, key: &str) -> Option<i64> {
    attrs.get(key).and_then(|v| v.as_i64())
}

/// Extract an optional boolean attribute from a JSON value.
pub fn optional_bool(attrs: &serde_json::Value, key: &str) -> Option<bool> {
    attrs.get(key).and_then(|v| v.as_bool())
}

/// Extract tags from Terraform resource attributes.
///
/// Looks at both `"tags"` and `"tags_all"` keys and merges them,
/// with `"tags"` taking precedence over `"tags_all"`.
pub fn extract_tags(attrs: &serde_json::Value) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    // tags_all first (lower precedence)
    if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    // tags second (higher precedence, overwrites tags_all)
    if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
        for (k, v) in obj {
            if let Some(s) = v.as_str() {
                tags.insert(k.clone(), s.to_string());
            }
        }
    }
    tags
}

/// Extract the region from resource attributes, falling back to a default.
pub fn extract_region(attrs: &serde_json::Value, default: &str) -> String {
    optional_str(attrs, "region").unwrap_or_else(|| default.to_string())
}

/// Extract the account ID from resource attributes, falling back to a default.
pub fn extract_account_id(attrs: &serde_json::Value, default: &str) -> String {
    optional_str(attrs, "account_id").unwrap_or_else(|| default.to_string())
}

/// Map a `serde_json` deserialize error onto the existing `ConversionError`
/// variants. Returns `MissingAttribute` when the underlying error names a
/// missing field, otherwise `InvalidAttribute`. Used by spec-driven
/// converters that deserialize Terraform attributes into a generated
/// TfModel struct.
pub fn classify_deserialize_error(resource_type: &str, err: serde_json::Error) -> ConversionError {
    let message = err.to_string();
    let prefix = "missing field `";
    if let Some(rest) = message.strip_prefix(prefix)
        && let Some(end) = rest.find('`')
    {
        return ConversionError::MissingAttribute {
            resource_type: resource_type.to_string(),
            attribute: rest[..end].to_string(),
        };
    }
    ConversionError::InvalidAttribute {
        resource_type: resource_type.to_string(),
        attribute: "(payload)".to_string(),
        detail: message,
    }
}
