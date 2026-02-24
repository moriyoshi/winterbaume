mod rules;

use winterbaume_bedrock_flow_parser::{FlowDefinition, ParseError};

/// Options controlling validation behaviour.
#[derive(Debug, Clone)]
pub struct ValidateOptions {
    /// Maximum number of diagnostics to return. AWS allows up to 100.
    pub max_diagnostics: usize,
}

impl Default for ValidateOptions {
    fn default() -> Self {
        Self {
            max_diagnostics: 100,
        }
    }
}

/// Severity of a validation diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Warning,
    Error,
}

impl std::fmt::Display for Severity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Warning => "WARNING",
            Self::Error => "ERROR",
        })
    }
}

/// Machine-readable validation type, matching the AWS `FlowValidation.type` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowValidationType {
    CyclicConnection,
    DuplicateConnections,
    DuplicateConditionExpression,
    IncompatibleConnectionDataType,
    MissingConnectionConfiguration,
    MissingDefaultCondition,
    MissingEndingNodes,
    MissingNodeConfiguration,
    MissingNodeInput,
    MissingNodeOutput,
    MissingStartingNodes,
    MismatchedNodeInputType,
    MismatchedNodeOutputType,
    MultipleNodeInputConnections,
    UnfulfilledNodeInput,
    UnknownConnectionCondition,
    UnknownConnectionSource,
    UnknownConnectionSourceOutput,
    UnknownConnectionTarget,
    UnknownConnectionTargetInput,
    UnknownNodeInput,
    UnknownNodeOutput,
    UnreachableNode,
    UnsatisfiedConnectionConditions,
    // Loop-specific
    MissingLoopInputNode,
    MissingLoopControllerNode,
    MultipleLoopInputNodes,
    MultipleLoopControllerNodes,
    LoopIncompatibleNodeType,
    InvalidLoopBoundary,
    // Catch-all
    Unspecified,
}

impl std::fmt::Display for FlowValidationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::CyclicConnection => "CyclicConnection",
            Self::DuplicateConnections => "DuplicateConnections",
            Self::DuplicateConditionExpression => "DuplicateConditionExpression",
            Self::IncompatibleConnectionDataType => "IncompatibleConnectionDataType",
            Self::MissingConnectionConfiguration => "MissingConnectionConfiguration",
            Self::MissingDefaultCondition => "MissingDefaultCondition",
            Self::MissingEndingNodes => "MissingEndingNodes",
            Self::MissingNodeConfiguration => "MissingNodeConfiguration",
            Self::MissingNodeInput => "MissingNodeInput",
            Self::MissingNodeOutput => "MissingNodeOutput",
            Self::MissingStartingNodes => "MissingStartingNodes",
            Self::MismatchedNodeInputType => "MismatchedNodeInputType",
            Self::MismatchedNodeOutputType => "MismatchedNodeOutputType",
            Self::MultipleNodeInputConnections => "MultipleNodeInputConnections",
            Self::UnfulfilledNodeInput => "UnfulfilledNodeInput",
            Self::UnknownConnectionCondition => "UnknownConnectionCondition",
            Self::UnknownConnectionSource => "UnknownConnectionSource",
            Self::UnknownConnectionSourceOutput => "UnknownConnectionSourceOutput",
            Self::UnknownConnectionTarget => "UnknownConnectionTarget",
            Self::UnknownConnectionTargetInput => "UnknownConnectionTargetInput",
            Self::UnknownNodeInput => "UnknownNodeInput",
            Self::UnknownNodeOutput => "UnknownNodeOutput",
            Self::UnreachableNode => "UnreachableNode",
            Self::UnsatisfiedConnectionConditions => "UnsatisfiedConnectionConditions",
            Self::MissingLoopInputNode => "MissingLoopInputNode",
            Self::MissingLoopControllerNode => "MissingLoopControllerNode",
            Self::MultipleLoopInputNodes => "MultipleLoopInputNodes",
            Self::MultipleLoopControllerNodes => "MultipleLoopControllerNodes",
            Self::LoopIncompatibleNodeType => "LoopIncompatibleNodeType",
            Self::InvalidLoopBoundary => "InvalidLoopBoundary",
            Self::Unspecified => "Unspecified",
        })
    }
}

/// A single validation diagnostic.
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub code: FlowValidationType,
    pub severity: Severity,
    pub message: String,
    pub details: DiagnosticDetails,
}

/// Type-specific detail fields matching the AWS `FlowValidationDetails` union.
#[derive(Debug, Clone)]
pub enum DiagnosticDetails {
    CyclicConnection {
        connection: Option<String>,
    },
    DuplicateConnections {
        source: Option<String>,
        target: Option<String>,
    },
    DuplicateConditionExpression {
        node: Option<String>,
        expression: Option<String>,
    },
    IncompatibleConnectionDataType {
        connection: Option<String>,
    },
    MissingConnectionConfiguration {
        connection: Option<String>,
    },
    MissingDefaultCondition {
        node: Option<String>,
    },
    MissingEndingNodes,
    MissingNodeConfiguration {
        node: Option<String>,
    },
    MissingNodeInput {
        node: Option<String>,
        input: Option<String>,
    },
    MissingNodeOutput {
        node: Option<String>,
        output: Option<String>,
    },
    MissingStartingNodes,
    MismatchedNodeInputType {
        node: Option<String>,
        input: Option<String>,
        expected_type: Option<String>,
    },
    MismatchedNodeOutputType {
        node: Option<String>,
        output: Option<String>,
        expected_type: Option<String>,
    },
    MultipleNodeInputConnections {
        node: Option<String>,
        input: Option<String>,
    },
    UnfulfilledNodeInput {
        node: Option<String>,
        input: Option<String>,
    },
    UnknownConnectionCondition {
        connection: Option<String>,
    },
    UnknownConnectionSource {
        connection: Option<String>,
    },
    UnknownConnectionSourceOutput {
        connection: Option<String>,
    },
    UnknownConnectionTarget {
        connection: Option<String>,
    },
    UnknownConnectionTargetInput {
        connection: Option<String>,
    },
    UnknownNodeInput {
        node: Option<String>,
        input: Option<String>,
    },
    UnknownNodeOutput {
        node: Option<String>,
        output: Option<String>,
    },
    UnreachableNode {
        node: Option<String>,
    },
    UnsatisfiedConnectionConditions {
        connection: Option<String>,
    },
    MissingLoopInputNode {
        loop_node: Option<String>,
    },
    MissingLoopControllerNode {
        loop_node: Option<String>,
    },
    MultipleLoopInputNodes {
        loop_node: Option<String>,
    },
    MultipleLoopControllerNodes {
        loop_node: Option<String>,
    },
    LoopIncompatibleNodeType {
        node: Option<String>,
        incompatible_node_name: Option<String>,
        incompatible_node_type: Option<String>,
    },
    InvalidLoopBoundary {
        connection: Option<String>,
        source: Option<String>,
        target: Option<String>,
    },
    Unspecified,
}

/// The result of validating a flow definition.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
    pub truncated: bool,
}

/// Validate a parsed [`FlowDefinition`].
///
/// Returns a [`ValidationResult`] containing any diagnostics found.
pub fn validate(def: &FlowDefinition, options: &ValidateOptions) -> ValidationResult {
    let mut diagnostics = Vec::new();

    rules::graph::check(def, &mut diagnostics);
    rules::connection::check(def, &mut diagnostics);
    rules::node::check(def, &mut diagnostics);
    rules::types::check(def, &mut diagnostics);
    rules::loop_rules::check(def, &mut diagnostics);

    let truncated = diagnostics.len() > options.max_diagnostics;
    if truncated {
        diagnostics.truncate(options.max_diagnostics);
    }

    ValidationResult {
        diagnostics,
        truncated,
    }
}

/// Convenience: parse JSON and validate in one call.
pub fn parse_and_validate(
    value: &serde_json::Value,
    options: &ValidateOptions,
) -> Result<ValidationResult, ParseError> {
    let def = winterbaume_bedrock_flow_parser::parse_flow_definition(value)?;
    Ok(validate(&def, options))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_minimal_flow() {
        let json = serde_json::json!({
            "nodes": [
                {
                    "name": "Start",
                    "type": "Input",
                    "outputs": [{ "name": "document", "type": "String" }]
                },
                {
                    "name": "End",
                    "type": "Output",
                    "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }]
                }
            ],
            "connections": [{
                "name": "c1",
                "source": "Start",
                "target": "End",
                "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } }
            }]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        assert!(
            result.diagnostics.is_empty(),
            "expected no diagnostics, got: {:?}",
            result.diagnostics
        );
    }

    #[test]
    fn missing_input_and_output_nodes() {
        let json = serde_json::json!({
            "nodes": [{ "name": "P", "type": "Prompt", "configuration": {
                "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } }
            }}]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::MissingStartingNodes));
        assert!(codes.contains(&FlowValidationType::MissingEndingNodes));
    }

    #[test]
    fn unknown_connection_source_and_target() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [{
                "name": "bad",
                "source": "Ghost",
                "target": "Phantom",
                "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } }
            }]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::UnknownConnectionSource));
        assert!(codes.contains(&FlowValidationType::UnknownConnectionTarget));
    }

    #[test]
    fn unreachable_node() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] },
                { "name": "Island", "type": "Prompt", "configuration": {
                    "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } }
                }}
            ],
            "connections": [{
                "name": "c1",
                "source": "Start",
                "target": "End",
                "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } }
            }]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let unreachable: Vec<_> = result
            .diagnostics
            .iter()
            .filter(|d| d.code == FlowValidationType::UnreachableNode)
            .collect();
        assert_eq!(unreachable.len(), 1);
    }

    #[test]
    fn duplicate_connections() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [
                { "name": "c1", "source": "Start", "target": "End", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } },
                { "name": "c2", "source": "Start", "target": "End", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } }
            ]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::DuplicateConnections));
    }

    #[test]
    fn missing_default_condition() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "Cond", "type": "Condition",
                  "configuration": { "condition": { "conditions": [
                      { "name": "check", "expression": "$.x > 0" }
                  ] } },
                  "inputs": [{ "name": "x", "type": "Number", "expression": "$.data.x" }]
                },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [
                { "name": "c1", "source": "Start", "target": "Cond", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "x" } } },
                { "name": "c2", "source": "Cond", "target": "End", "type": "Conditional",
                  "configuration": { "conditional": { "condition": "check" } } }
            ]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::MissingDefaultCondition));
    }

    #[test]
    fn truncation() {
        // Many disconnected nodes to generate many diagnostics.
        let nodes: Vec<_> = (0..20)
            .map(|i| serde_json::json!({ "name": format!("N{i}"), "type": "Prompt",
                "configuration": { "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } } }
            }))
            .collect();
        let json = serde_json::json!({ "nodes": nodes });
        let opts = ValidateOptions { max_diagnostics: 3 };
        let result = parse_and_validate(&json, &opts).unwrap();
        assert!(result.truncated);
        assert_eq!(result.diagnostics.len(), 3);
    }

    #[test]
    fn cyclic_connection() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "A", "type": "Prompt",
                  "configuration": { "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } } },
                  "inputs": [{ "name": "input", "type": "String", "expression": "$.data" }],
                  "outputs": [{ "name": "modelCompletion", "type": "String" }] },
                { "name": "B", "type": "Prompt",
                  "configuration": { "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } } },
                  "inputs": [{ "name": "input", "type": "String", "expression": "$.data" }],
                  "outputs": [{ "name": "modelCompletion", "type": "String" }] },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [
                { "name": "c0", "source": "Start", "target": "A", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "input" } } },
                { "name": "c1", "source": "A", "target": "B", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "modelCompletion", "targetInput": "input" } } },
                { "name": "c2", "source": "B", "target": "A", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "modelCompletion", "targetInput": "input" } } },
                { "name": "c3", "source": "B", "target": "End", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "modelCompletion", "targetInput": "document" } } }
            ]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::CyclicConnection));
    }

    #[test]
    fn loop_missing_required_nodes() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Start", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "MyLoop", "type": "Loop",
                  "configuration": { "loop": { "definition": { "nodes": [], "connections": [] } } },
                  "inputs": [{ "name": "loopInput", "type": "String", "expression": "$.data" }],
                  "outputs": [{ "name": "loopOutput", "type": "String" }] },
                { "name": "End", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [
                { "name": "c1", "source": "Start", "target": "MyLoop", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "loopInput" } } },
                { "name": "c2", "source": "MyLoop", "target": "End", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "loopOutput", "targetInput": "document" } } }
            ]
        });
        let result = parse_and_validate(&json, &ValidateOptions::default()).unwrap();
        let codes: Vec<_> = result.diagnostics.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::MissingLoopInputNode));
        assert!(codes.contains(&FlowValidationType::MissingLoopControllerNode));
    }
}
