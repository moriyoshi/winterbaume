use std::collections::HashMap;

use serde_json::Value;

use crate::Diagnostic;
use crate::model::{State, StateMachine, StateType};

/// Parse raw JSON into an internal `StateMachine` model, collecting
/// diagnostics for structural problems encountered during parsing.
pub fn parse_state_machine(
    root: &Value,
    pointer: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> StateMachine {
    let obj = match root.as_object() {
        Some(o) => o,
        None => {
            diagnostics.push(Diagnostic {
                code: "INVALID_JSON",
                message: "Top-level value is not a JSON object".into(),
                location: Some(pointer.to_string()),
            });
            return StateMachine {
                start_at: None,
                states: HashMap::new(),
                pointer: pointer.to_string(),
            };
        }
    };

    let start_at = obj
        .get("StartAt")
        .and_then(|v| v.as_str())
        .map(String::from);
    if start_at.is_none() {
        diagnostics.push(Diagnostic {
            code: "MISSING_START_AT",
            message: format!("State machine at `{pointer}` is missing required field \"StartAt\""),
            location: Some(pointer.to_string()),
        });
    }

    let mut states = HashMap::new();

    if let Some(Value::Object(states_obj)) = obj.get("States") {
        for (name, state_val) in states_obj {
            let state_pointer = format!("{pointer}/States/{name}");
            let state = parse_state(name, state_val, &state_pointer, diagnostics);
            states.insert(name.clone(), state);
        }
    }

    StateMachine {
        start_at,
        states,
        pointer: pointer.to_string(),
    }
}

fn parse_state(
    name: &str,
    value: &Value,
    pointer: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> State {
    let obj = value.as_object();

    let type_str = obj.and_then(|o| o.get("Type")).and_then(|v| v.as_str());
    let state_type = match type_str {
        Some(t) => match t.parse::<StateType>() {
            Ok(st) => Some(st),
            Err(_) => {
                diagnostics.push(Diagnostic {
                    code: "INVALID_TYPE",
                    message: format!("State \"{name}\" has unknown Type \"{t}\""),
                    location: Some(pointer.to_string()),
                });
                None
            }
        },
        None => {
            diagnostics.push(Diagnostic {
                code: "MISSING_TYPE",
                message: format!("State \"{name}\" is missing required field \"Type\""),
                location: Some(pointer.to_string()),
            });
            None
        }
    };

    let resource = obj
        .and_then(|o| o.get("Resource"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let next = obj
        .and_then(|o| o.get("Next"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let end = obj
        .and_then(|o| o.get("End"))
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let default = obj
        .and_then(|o| o.get("Default"))
        .and_then(|v| v.as_str())
        .map(String::from);

    // Parse Choice targets from Choices array
    let mut choice_targets = Vec::new();
    let mut has_choices = false;
    if let Some(Value::Array(choices)) = obj.and_then(|o| o.get("Choices")) {
        has_choices = !choices.is_empty();
        for choice in choices {
            if let Some(target) = choice.get("Next").and_then(|v| v.as_str()) {
                choice_targets.push(target.to_string());
            }
        }
    }

    let has_item_processor = obj
        .map(|o| o.contains_key("Iterator") || o.contains_key("ItemProcessor"))
        .unwrap_or(false);

    State {
        name: name.to_string(),
        state_type,
        resource,
        next,
        end,
        choice_targets,
        default,
        has_choices,
        has_item_processor,
        pointer: pointer.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_minimal_state_machine() {
        let json: Value = serde_json::from_str(
            r#"{
                "StartAt": "Hello",
                "States": {
                    "Hello": {
                        "Type": "Pass",
                        "End": true
                    }
                }
            }"#,
        )
        .unwrap();

        let mut diags = Vec::new();
        let sm = parse_state_machine(&json, "", &mut diags);

        assert!(diags.is_empty(), "expected no diagnostics, got: {diags:?}");
        assert_eq!(sm.start_at.as_deref(), Some("Hello"));
        assert_eq!(sm.states.len(), 1);
        assert_eq!(sm.states["Hello"].state_type, Some(StateType::Pass));
        assert!(sm.states["Hello"].end);
    }

    #[test]
    fn parse_missing_start_at() {
        let json: Value = serde_json::from_str(
            r#"{
                "States": {
                    "Hello": { "Type": "Pass", "End": true }
                }
            }"#,
        )
        .unwrap();

        let mut diags = Vec::new();
        parse_state_machine(&json, "", &mut diags);

        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "MISSING_START_AT");
    }

    #[test]
    fn parse_invalid_type() {
        let json: Value = serde_json::from_str(
            r#"{
                "StartAt": "Hello",
                "States": {
                    "Hello": { "Type": "DoSomething", "End": true }
                }
            }"#,
        )
        .unwrap();

        let mut diags = Vec::new();
        parse_state_machine(&json, "", &mut diags);

        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "INVALID_TYPE");
    }

    #[test]
    fn parse_non_object_root() {
        let json: Value = serde_json::from_str(r#""not an object""#).unwrap();

        let mut diags = Vec::new();
        parse_state_machine(&json, "", &mut diags);

        assert_eq!(diags.len(), 1);
        assert_eq!(diags[0].code, "INVALID_JSON");
    }
}
