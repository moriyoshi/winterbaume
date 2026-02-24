use std::collections::{HashMap, HashSet};

use winterbaume_bedrock_flow_parser::{
    FlowConnectionType, FlowDataType, FlowDefinition, FlowNodeConfiguration, FlowNodeType,
};

use crate::{Diagnostic, DiagnosticDetails, FlowValidationType, Severity};

/// Specification of expected I/O ports for each node type.
struct NodeTypeSpec {
    required_inputs: &'static [(&'static str, FlowDataType)],
    required_outputs: &'static [(&'static str, FlowDataType)],
    allows_extra_inputs: bool,
    requires_configuration: bool,
}

fn spec_for(node_type: &FlowNodeType) -> NodeTypeSpec {
    match node_type {
        FlowNodeType::Input => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[("document", FlowDataType::String)],
            allows_extra_inputs: false,
            requires_configuration: false,
        },
        FlowNodeType::Output => NodeTypeSpec {
            required_inputs: &[("document", FlowDataType::String)],
            required_outputs: &[],
            allows_extra_inputs: false,
            requires_configuration: false,
        },
        FlowNodeType::Condition => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::Prompt => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[("modelCompletion", FlowDataType::String)],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::KnowledgeBase => NodeTypeSpec {
            required_inputs: &[("retrievalQuery", FlowDataType::String)],
            required_outputs: &[("outputText", FlowDataType::String)],
            allows_extra_inputs: false,
            requires_configuration: true,
        },
        FlowNodeType::LambdaFunction => NodeTypeSpec {
            required_inputs: &[("codeHookInput", FlowDataType::String)],
            required_outputs: &[("functionResponse", FlowDataType::String)],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::Lex => NodeTypeSpec {
            required_inputs: &[("inputText", FlowDataType::String)],
            required_outputs: &[("predictedIntent", FlowDataType::String)],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::Agent => NodeTypeSpec {
            required_inputs: &[("agentInputText", FlowDataType::String)],
            required_outputs: &[("agentResponse", FlowDataType::String)],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::Storage => NodeTypeSpec {
            required_inputs: &[
                ("content", FlowDataType::String),
                ("objectKey", FlowDataType::String),
            ],
            required_outputs: &[("s3Uri", FlowDataType::String)],
            allows_extra_inputs: false,
            requires_configuration: true,
        },
        FlowNodeType::Retrieval => NodeTypeSpec {
            required_inputs: &[("objectKey", FlowDataType::String)],
            required_outputs: &[("s3Content", FlowDataType::String)],
            allows_extra_inputs: false,
            requires_configuration: true,
        },
        FlowNodeType::Iterator => NodeTypeSpec {
            required_inputs: &[("array", FlowDataType::Array)],
            required_outputs: &[
                ("arrayItem", FlowDataType::String),
                ("arraySize", FlowDataType::Number),
            ],
            allows_extra_inputs: false,
            requires_configuration: false,
        },
        FlowNodeType::Collector => NodeTypeSpec {
            required_inputs: &[
                ("arrayItem", FlowDataType::String),
                ("arraySize", FlowDataType::Number),
            ],
            required_outputs: &[("collectedArray", FlowDataType::Array)],
            allows_extra_inputs: false,
            requires_configuration: false,
        },
        FlowNodeType::InlineCode => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[("response", FlowDataType::String)],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
        FlowNodeType::Loop => NodeTypeSpec {
            required_inputs: &[("loopInput", FlowDataType::String)],
            required_outputs: &[
                ("loopOutput", FlowDataType::String),
                ("iterationCount", FlowDataType::Number),
            ],
            allows_extra_inputs: false,
            requires_configuration: true,
        },
        FlowNodeType::LoopInput => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[],
            allows_extra_inputs: false,
            requires_configuration: false,
        },
        FlowNodeType::LoopController => NodeTypeSpec {
            required_inputs: &[],
            required_outputs: &[],
            allows_extra_inputs: true,
            requires_configuration: true,
        },
    }
}

pub fn check(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let node_map: HashMap<&str, usize> = def
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.as_str(), i))
        .collect();

    for node in &def.nodes {
        let spec = spec_for(&node.node_type);

        // MissingNodeConfiguration
        if spec.requires_configuration && node.configuration.is_none() {
            diagnostics.push(Diagnostic {
                code: FlowValidationType::MissingNodeConfiguration,
                severity: Severity::Error,
                message: format!(
                    "Node `{}` of type {} requires configuration",
                    node.name, node.node_type
                ),
                details: DiagnosticDetails::MissingNodeConfiguration {
                    node: Some(node.name.clone()),
                },
            });
        }

        // MissingNodeInput
        let input_names: HashSet<&str> = node.inputs.iter().map(|i| i.name.as_str()).collect();
        for &(req_name, _) in spec.required_inputs {
            if !input_names.contains(req_name) {
                diagnostics.push(Diagnostic {
                    code: FlowValidationType::MissingNodeInput,
                    severity: Severity::Error,
                    message: format!(
                        "Node `{}` of type {} is missing required input `{}`",
                        node.name, node.node_type, req_name
                    ),
                    details: DiagnosticDetails::MissingNodeInput {
                        node: Some(node.name.clone()),
                        input: Some(req_name.to_string()),
                    },
                });
            }
        }

        // MissingNodeOutput
        let output_names: HashSet<&str> = node.outputs.iter().map(|o| o.name.as_str()).collect();
        for &(req_name, _) in spec.required_outputs {
            if !output_names.contains(req_name) {
                diagnostics.push(Diagnostic {
                    code: FlowValidationType::MissingNodeOutput,
                    severity: Severity::Error,
                    message: format!(
                        "Node `{}` of type {} is missing required output `{}`",
                        node.name, node.node_type, req_name
                    ),
                    details: DiagnosticDetails::MissingNodeOutput {
                        node: Some(node.name.clone()),
                        output: Some(req_name.to_string()),
                    },
                });
            }
        }

        // UnknownNodeInput (only for node types that don't allow extra inputs)
        if !spec.allows_extra_inputs {
            let valid_inputs: HashSet<&str> =
                spec.required_inputs.iter().map(|&(n, _)| n).collect();
            for input in &node.inputs {
                if !valid_inputs.contains(input.name.as_str()) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::UnknownNodeInput,
                        severity: Severity::Error,
                        message: format!(
                            "Node `{}` of type {} does not accept input `{}`",
                            node.name, node.node_type, input.name
                        ),
                        details: DiagnosticDetails::UnknownNodeInput {
                            node: Some(node.name.clone()),
                            input: Some(input.name.clone()),
                        },
                    });
                }
            }
        }

        // UnknownNodeOutput
        if !spec.required_outputs.is_empty() {
            let valid_outputs: HashSet<&str> =
                spec.required_outputs.iter().map(|&(n, _)| n).collect();
            for output in &node.outputs {
                if !valid_outputs.contains(output.name.as_str()) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::UnknownNodeOutput,
                        severity: Severity::Error,
                        message: format!(
                            "Node `{}` of type {} does not produce output `{}`",
                            node.name, node.node_type, output.name
                        ),
                        details: DiagnosticDetails::UnknownNodeOutput {
                            node: Some(node.name.clone()),
                            output: Some(output.name.clone()),
                        },
                    });
                }
            }
        }

        // MissingDefaultCondition
        if node.node_type == FlowNodeType::Condition {
            if let Some(FlowNodeConfiguration::Condition(ref cfg)) = node.configuration {
                if !cfg.conditions.iter().any(|c| c.name == "default") {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::MissingDefaultCondition,
                        severity: Severity::Error,
                        message: format!(
                            "Condition node `{}` is missing a `default` condition",
                            node.name
                        ),
                        details: DiagnosticDetails::MissingDefaultCondition {
                            node: Some(node.name.clone()),
                        },
                    });
                }

                // DuplicateConditionExpression
                let mut seen_exprs: HashSet<&str> = HashSet::new();
                for cond in &cfg.conditions {
                    if let Some(ref expr) = cond.expression {
                        if !seen_exprs.insert(expr.as_str()) {
                            diagnostics.push(Diagnostic {
                                code: FlowValidationType::DuplicateConditionExpression,
                                severity: Severity::Error,
                                message: format!(
                                    "Condition node `{}` has duplicate expression `{}`",
                                    node.name, expr
                                ),
                                details: DiagnosticDetails::DuplicateConditionExpression {
                                    node: Some(node.name.clone()),
                                    expression: Some(expr.clone()),
                                },
                            });
                        }
                    }
                }
            }
        }
    }

    // UnfulfilledNodeInput: required inputs that have no incoming Data connection.
    check_unfulfilled_inputs(def, &node_map, diagnostics);
}

fn check_unfulfilled_inputs(
    def: &FlowDefinition,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Collect all (target_node, target_input) pairs from Data connections.
    let mut fulfilled: HashSet<(&str, &str)> = HashSet::new();
    for conn in &def.connections {
        if let FlowConnectionType::Data {
            ref target_input, ..
        } = conn.connection_type
        {
            if node_map.contains_key(conn.target.as_str()) {
                fulfilled.insert((conn.target.as_str(), target_input.as_str()));
            }
        }
    }

    for node in &def.nodes {
        // Input nodes have no required incoming connections.
        if node.node_type == FlowNodeType::Input {
            continue;
        }
        let spec = spec_for(&node.node_type);
        for &(req_name, _) in spec.required_inputs {
            if !fulfilled.contains(&(node.name.as_str(), req_name)) {
                diagnostics.push(Diagnostic {
                    code: FlowValidationType::UnfulfilledNodeInput,
                    severity: Severity::Warning,
                    message: format!(
                        "Required input `{}` on node `{}` has no incoming data connection",
                        req_name, node.name
                    ),
                    details: DiagnosticDetails::UnfulfilledNodeInput {
                        node: Some(node.name.clone()),
                        input: Some(req_name.to_string()),
                    },
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use winterbaume_bedrock_flow_parser::parse_flow_definition;

    use super::*;

    #[test]
    fn valid_node() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [{ "name": "c1", "source": "In", "target": "Out", "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn detects_missing_config() {
        let json = serde_json::json!({
            "nodes": [{ "name": "P", "type": "Prompt" }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MissingNodeConfiguration)
        );
    }

    #[test]
    fn detects_missing_required_output() {
        let json = serde_json::json!({
            "nodes": [{ "name": "In", "type": "Input" }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MissingNodeOutput)
        );
    }

    #[test]
    fn detects_unknown_input() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "In",
                "type": "Input",
                "inputs": [{ "name": "bogus", "type": "String", "expression": "$.x" }],
                "outputs": [{ "name": "document", "type": "String" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnknownNodeInput)
        );
    }

    #[test]
    fn detects_missing_default_condition() {
        let json = serde_json::json!({
            "nodes": [{ "name": "C", "type": "Condition", "configuration": {
                "condition": { "conditions": [{ "name": "yes", "expression": "true" }] }
            } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MissingDefaultCondition)
        );
    }

    #[test]
    fn detects_duplicate_condition_expression() {
        let json = serde_json::json!({
            "nodes": [{ "name": "C", "type": "Condition", "configuration": {
                "condition": { "conditions": [
                    { "name": "a", "expression": "$.x > 0" },
                    { "name": "b", "expression": "$.x > 0" },
                    { "name": "default" }
                ] }
            } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::DuplicateConditionExpression)
        );
    }

    #[test]
    fn detects_unfulfilled_input() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnfulfilledNodeInput)
        );
    }
}
