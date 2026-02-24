pub mod error;
pub mod model;
pub mod parse;
pub mod rules;
mod validate;

/// Options controlling validation behaviour.
#[derive(Debug, Clone)]
pub struct ValidateOptions {
    /// Maximum number of diagnostics to return. Matches AWS default of 10.
    pub max_diagnostics: usize,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            max_diagnostics: 10,
        }
    }
}

/// A single diagnostic produced by the ASL validation engine.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Machine-readable code, e.g. `"NO_SUCH_STATE"`, `"MISSING_RESOURCE_URI"`.
    pub code: &'static str,
    /// Human-readable description of the problem.
    pub message: String,
    /// JSON Pointer to the offending node, if available.
    pub location: Option<String>,
}

/// The result of validating an ASL definition.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// `true` when zero diagnostics were produced.
    pub valid: bool,
    /// Diagnostics up to `max_diagnostics`.
    pub diagnostics: Vec<Diagnostic>,
    /// `true` when more diagnostics were found than `max_diagnostics`.
    pub truncated: bool,
}

/// Validate an ASL (Amazon States Language) definition JSON string.
///
/// Returns a `ValidationResult` containing any diagnostics found.
/// The number of returned diagnostics is bounded by
/// [`ValidateOptions::max_diagnostics`]; when this limit is exceeded,
/// [`ValidationResult::truncated`] is set to `true`.
pub fn validate(definition_json: &str, options: &ValidateOptions) -> ValidationResult {
    validate::run_validation(definition_json, options)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn public_api_valid_machine() {
        let json = r#"{
            "StartAt": "Only",
            "States": {
                "Only": { "Type": "Succeed" }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(result.valid);
        assert!(result.diagnostics.is_empty());
        assert!(!result.truncated);
    }

    #[test]
    fn public_api_invalid_json() {
        let result = validate("not json at all", &ValidateOptions::default());
        assert!(!result.valid);
        assert_eq!(result.diagnostics[0].code, "INVALID_JSON");
    }

    #[test]
    fn public_api_missing_start_at() {
        let json = r#"{
            "States": {
                "A": { "Type": "Succeed" }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_START_AT")
        );
    }

    #[test]
    fn public_api_no_such_state() {
        let json = r#"{
            "StartAt": "Ghost",
            "States": {
                "A": { "Type": "Succeed" }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(result.diagnostics.iter().any(|d| d.code == "NO_SUCH_STATE"));
    }

    #[test]
    fn public_api_missing_resource_uri() {
        let json = r#"{
            "StartAt": "T",
            "States": {
                "T": { "Type": "Task", "End": true }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_RESOURCE_URI")
        );
    }

    #[test]
    fn public_api_missing_iterator() {
        let json = r#"{
            "StartAt": "M",
            "States": {
                "M": { "Type": "Map", "End": true }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_ITERATOR")
        );
    }

    #[test]
    fn public_api_missing_choices() {
        let json = r#"{
            "StartAt": "C",
            "States": {
                "C": { "Type": "Choice" }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_CHOICES")
        );
    }

    #[test]
    fn public_api_missing_end_or_next() {
        let json = r#"{
            "StartAt": "W",
            "States": {
                "W": { "Type": "Wait", "Seconds": 10 }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_END_OR_NEXT")
        );
    }

    #[test]
    fn public_api_invalid_type() {
        let json = r#"{
            "StartAt": "X",
            "States": {
                "X": { "Type": "Bogus", "End": true }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(result.diagnostics.iter().any(|d| d.code == "INVALID_TYPE"));
    }

    #[test]
    fn public_api_map_with_item_processor_passes() {
        let json = r#"{
            "StartAt": "M",
            "States": {
                "M": {
                    "Type": "Map",
                    "ItemProcessor": {
                        "StartAt": "Inner",
                        "States": {
                            "Inner": { "Type": "Succeed" }
                        }
                    },
                    "End": true
                }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(
            !result
                .diagnostics
                .iter()
                .any(|d| d.code == "MISSING_ITERATOR"),
            "Map with ItemProcessor should not produce MISSING_ITERATOR, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn public_api_missing_type() {
        let json = r#"{
            "StartAt": "X",
            "States": {
                "X": { "End": true }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result.diagnostics.iter().any(|d| d.code == "MISSING_TYPE"),
            "expected MISSING_TYPE diagnostic, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn public_api_conflicting_end_and_next() {
        let json = r#"{
            "StartAt": "A",
            "States": {
                "A": { "Type": "Pass", "End": true, "Next": "A" }
            }
        }"#;
        let result = validate(json, &ValidateOptions::default());
        assert!(!result.valid);
        assert!(
            result
                .diagnostics
                .iter()
                .any(|d| d.code == "CONFLICTING_END_AND_NEXT"),
            "expected CONFLICTING_END_AND_NEXT diagnostic, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn public_api_truncation() {
        // Generate enough errors to trigger truncation at max_diagnostics=3.
        let json = r#"{
            "States": {
                "A": { "Type": "Task" },
                "B": { "Type": "Task" },
                "C": { "Type": "Task" },
                "D": { "Type": "Task" }
            }
        }"#;
        let opts = ValidateOptions { max_diagnostics: 3 };
        let result = validate(json, &opts);
        assert!(!result.valid);
        assert!(result.truncated);
        assert_eq!(result.diagnostics.len(), 3);
    }
}
