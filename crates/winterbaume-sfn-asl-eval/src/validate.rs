use crate::parse::parse_state_machine;
use crate::rules::structural::check_structural;
use crate::{Diagnostic, ValidateOptions, ValidationResult};

/// Run all validation phases on a parsed JSON value.
///
/// This is the internal orchestration entry point called by `lib::validate`.
pub fn run_validation(definition_json: &str, options: &ValidateOptions) -> ValidationResult {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    // Phase 0: parse JSON
    let root = match serde_json::from_str::<serde_json::Value>(definition_json) {
        Ok(v) => v,
        Err(e) => {
            return ValidationResult {
                valid: false,
                diagnostics: vec![Diagnostic {
                    code: "INVALID_JSON",
                    message: format!("Failed to parse definition as JSON: {e}"),
                    location: None,
                }],
                truncated: false,
            };
        }
    };

    // Phase 1a: parse into internal model, collecting parse-level diagnostics
    let sm = parse_state_machine(&root, "", &mut diagnostics);

    // Phase 1b: structural rules
    check_structural(&sm, &mut diagnostics);

    // Truncate to max_diagnostics
    let truncated = diagnostics.len() > options.max_diagnostics;
    if truncated {
        diagnostics.truncate(options.max_diagnostics);
    }

    let valid = diagnostics.is_empty();

    ValidationResult {
        valid,
        diagnostics,
        truncated,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_state_machine() {
        let json = r#"{
            "StartAt": "Hello",
            "States": {
                "Hello": {
                    "Type": "Pass",
                    "End": true
                }
            }
        }"#;
        let result = run_validation(json, &ValidateOptions::default());
        assert!(result.valid);
        assert!(result.diagnostics.is_empty());
        assert!(!result.truncated);
    }

    #[test]
    fn invalid_json_input() {
        let result = run_validation("{{not json", &ValidateOptions::default());
        assert!(!result.valid);
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].code, "INVALID_JSON");
    }

    #[test]
    fn truncation_when_max_diagnostics_exceeded() {
        // Build a state machine with many errors: many Task states without
        // Resource and without End/Next.
        let mut states = String::new();
        for i in 0..20 {
            if i > 0 {
                states.push(',');
            }
            states.push_str(&format!(r#""S{i}": {{ "Type": "Task" }}"#));
        }
        let json = format!(r#"{{ "StartAt": "S0", "States": {{ {states} }} }}"#);

        let opts = ValidateOptions { max_diagnostics: 5 };
        let result = run_validation(&json, &opts);
        assert!(!result.valid);
        assert!(result.truncated);
        assert_eq!(result.diagnostics.len(), 5);
    }

    #[test]
    fn multi_state_chain() {
        let json = r#"{
            "StartAt": "A",
            "States": {
                "A": { "Type": "Pass", "Next": "B" },
                "B": { "Type": "Task", "Resource": "arn:aws:lambda:us-east-1:123:function:foo", "End": true }
            }
        }"#;
        let result = run_validation(json, &ValidateOptions::default());
        assert!(result.valid);
        assert!(result.diagnostics.is_empty());
    }

    #[test]
    fn choice_state_valid() {
        let json = r#"{
            "StartAt": "C",
            "States": {
                "C": {
                    "Type": "Choice",
                    "Choices": [
                        { "Variable": "$.x", "NumericEquals": 1, "Next": "Done" }
                    ],
                    "Default": "Done"
                },
                "Done": { "Type": "Succeed" }
            }
        }"#;
        let result = run_validation(json, &ValidateOptions::default());
        assert!(result.valid, "diagnostics: {:?}", result.diagnostics);
    }
}
