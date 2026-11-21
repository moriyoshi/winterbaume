//! Validation of a resource model against a hand-transcribed subset of its
//! CloudFormation registry schema.
//!
//! Cloud Control validates `DesiredState`, and the result of applying an
//! `UpdateResource` patch to the current read model, before calling the
//! resource handler. A failure is a synchronous `ValidationException` whose
//! message lists every violation, in the wording of the JSON Schema validator
//! the CloudFormation registry uses:
//!
//! ```text
//! Model validation failed (#/A/B: required key [C] not found
//! #/A: extraneous key [D] is not permitted)
//! ```

use serde_json::Value;

/// The subset of JSON Schema that the transcribed schemas need.
pub(crate) enum Schema {
    String,
    Boolean,
    Integer,
    Number,
    /// A string that must match `pattern`, which is an anchored alternation
    /// of `values`.
    Enum {
        pattern: &'static str,
        values: &'static [&'static str],
    },
    Array(&'static Schema),
    /// An object with `additionalProperties: false`.
    Object {
        properties: &'static [(&'static str, Schema)],
        required: &'static [&'static str],
    },
}

/// Validate `model`, returning the `Model validation failed (...)` message on
/// failure.
pub(crate) fn validate(schema: &Schema, model: &Value) -> Result<(), String> {
    let mut errors = Vec::new();
    check(schema, model, "#", &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(format!("Model validation failed ({})", errors.join("\n")))
    }
}

fn type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(n) if n.is_f64() => "Number",
        Value::Number(_) => "Integer",
        Value::String(_) => "String",
        Value::Array(_) => "JSONArray",
        Value::Object(_) => "JSONObject",
    }
}

fn check(schema: &Schema, value: &Value, path: &str, errors: &mut Vec<String>) {
    let expected = match schema {
        Schema::String | Schema::Enum { .. } if value.is_string() => None,
        Schema::String | Schema::Enum { .. } => Some("String"),
        Schema::Boolean if value.is_boolean() => None,
        Schema::Boolean => Some("Boolean"),
        Schema::Integer if value.is_i64() || value.is_u64() => None,
        Schema::Integer => Some("Integer"),
        Schema::Number if value.is_number() => None,
        Schema::Number => Some("Number"),
        Schema::Array(_) if value.is_array() => None,
        Schema::Array(_) => Some("JSONArray"),
        Schema::Object { .. } if value.is_object() => None,
        Schema::Object { .. } => Some("JSONObject"),
    };
    if let Some(expected) = expected {
        errors.push(format!(
            "{path}: expected type: {expected}, found: {}",
            type_name(value)
        ));
        return;
    }
    match schema {
        Schema::Enum { pattern, values } => {
            let s = value.as_str().unwrap();
            if !values.contains(&s) {
                errors.push(format!(
                    "{path}: string [{s}] does not match pattern {pattern}"
                ));
            }
        }
        Schema::Array(items) => {
            for (i, item) in value.as_array().unwrap().iter().enumerate() {
                check(items, item, &format!("{path}/{i}"), errors);
            }
        }
        Schema::Object {
            properties,
            required,
        } => {
            let object = value.as_object().unwrap();
            for key in *required {
                if !object.contains_key(*key) {
                    errors.push(format!("{path}: required key [{key}] not found"));
                }
            }
            for (key, child) in object {
                match properties.iter().find(|(name, _)| name == key) {
                    Some((_, schema)) => check(schema, child, &format!("{path}/{key}"), errors),
                    None => errors.push(format!("{path}: extraneous key [{key}] is not permitted")),
                }
            }
        }
        Schema::String | Schema::Boolean | Schema::Integer | Schema::Number => {}
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::{Schema, validate};

    const SCHEMA: Schema = Schema::Object {
        properties: &[
            (
                "Frame",
                Schema::Object {
                    properties: &[
                        ("Override", Schema::Boolean),
                        (
                            "FrameOption",
                            Schema::Enum {
                                pattern: "^(DENY|SAMEORIGIN)$",
                                values: &["DENY", "SAMEORIGIN"],
                            },
                        ),
                    ],
                    required: &["Override", "FrameOption"],
                },
            ),
            ("Items", Schema::Array(&Schema::Integer)),
        ],
        required: &["Frame"],
    };

    #[test]
    fn reports_every_violation() {
        assert!(
            validate(
                &SCHEMA,
                &json!({"Frame": {"Override": true, "FrameOption": "DENY"}})
            )
            .is_ok()
        );
        let message = validate(
            &SCHEMA,
            &json!({"Frame": {"FrameOption": "ALLOW"}, "Items": [1, "x"], "Extra": 1}),
        )
        .unwrap_err();
        let errors = message
            .strip_prefix("Model validation failed (")
            .and_then(|m| m.strip_suffix(')'))
            .unwrap();
        let mut errors: Vec<_> = errors.lines().collect();
        errors.sort_unstable();
        assert_eq!(
            errors,
            [
                "#/Frame/FrameOption: string [ALLOW] does not match pattern ^(DENY|SAMEORIGIN)$",
                "#/Frame: required key [Override] not found",
                "#/Items/1: expected type: Integer, found: String",
                "#: extraneous key [Extra] is not permitted",
            ]
        );
    }
}
