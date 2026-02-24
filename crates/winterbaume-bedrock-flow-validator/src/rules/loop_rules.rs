use winterbaume_bedrock_flow_parser::{FlowDefinition, FlowNodeConfiguration, FlowNodeType};

use crate::{Diagnostic, DiagnosticDetails, FlowValidationType, Severity};

/// Forbidden node types inside a Loop sub-definition.
const LOOP_FORBIDDEN_TYPES: &[FlowNodeType] = &[
    FlowNodeType::Input,
    FlowNodeType::Output,
    FlowNodeType::Loop,
    FlowNodeType::Iterator,
    FlowNodeType::Collector,
];

pub fn check(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    for node in &def.nodes {
        if node.node_type != FlowNodeType::Loop {
            continue;
        }
        if let Some(FlowNodeConfiguration::Loop(ref cfg)) = node.configuration {
            check_loop_body(&node.name, &cfg.definition, diagnostics);
        }
    }
}

fn check_loop_body(loop_node_name: &str, body: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    // Count LoopInput and LoopController nodes.
    let loop_inputs: Vec<&str> = body
        .nodes
        .iter()
        .filter(|n| n.node_type == FlowNodeType::LoopInput)
        .map(|n| n.name.as_str())
        .collect();
    let loop_controllers: Vec<&str> = body
        .nodes
        .iter()
        .filter(|n| n.node_type == FlowNodeType::LoopController)
        .map(|n| n.name.as_str())
        .collect();

    if loop_inputs.is_empty() {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MissingLoopInputNode,
            severity: Severity::Error,
            message: format!(
                "Loop node `{}` is missing a LoopInput node in its definition",
                loop_node_name
            ),
            details: DiagnosticDetails::MissingLoopInputNode {
                loop_node: Some(loop_node_name.to_string()),
            },
        });
    }
    if loop_inputs.len() > 1 {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MultipleLoopInputNodes,
            severity: Severity::Error,
            message: format!(
                "Loop node `{}` has {} LoopInput nodes (expected 1)",
                loop_node_name,
                loop_inputs.len()
            ),
            details: DiagnosticDetails::MultipleLoopInputNodes {
                loop_node: Some(loop_node_name.to_string()),
            },
        });
    }

    if loop_controllers.is_empty() {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MissingLoopControllerNode,
            severity: Severity::Error,
            message: format!(
                "Loop node `{}` is missing a LoopController node in its definition",
                loop_node_name
            ),
            details: DiagnosticDetails::MissingLoopControllerNode {
                loop_node: Some(loop_node_name.to_string()),
            },
        });
    }
    if loop_controllers.len() > 1 {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MultipleLoopControllerNodes,
            severity: Severity::Error,
            message: format!(
                "Loop node `{}` has {} LoopController nodes (expected 1)",
                loop_node_name,
                loop_controllers.len()
            ),
            details: DiagnosticDetails::MultipleLoopControllerNodes {
                loop_node: Some(loop_node_name.to_string()),
            },
        });
    }

    // Check for forbidden node types.
    for inner_node in &body.nodes {
        if LOOP_FORBIDDEN_TYPES.contains(&inner_node.node_type) {
            diagnostics.push(Diagnostic {
                code: FlowValidationType::LoopIncompatibleNodeType,
                severity: Severity::Error,
                message: format!(
                    "Node `{}` of type {} is not allowed inside loop `{}`",
                    inner_node.name, inner_node.node_type, loop_node_name
                ),
                details: DiagnosticDetails::LoopIncompatibleNodeType {
                    node: Some(loop_node_name.to_string()),
                    incompatible_node_name: Some(inner_node.name.clone()),
                    incompatible_node_type: Some(inner_node.node_type.to_string()),
                },
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use winterbaume_bedrock_flow_parser::parse_flow_definition;

    use super::*;

    fn loop_json_with_body(body_nodes: serde_json::Value) -> serde_json::Value {
        serde_json::json!({
            "nodes": [{
                "name": "MyLoop",
                "type": "Loop",
                "configuration": {
                    "loop": {
                        "definition": {
                            "nodes": body_nodes,
                            "connections": []
                        }
                    }
                }
            }]
        })
    }

    #[test]
    fn valid_loop_body() {
        let json = loop_json_with_body(serde_json::json!([
            { "name": "LI", "type": "LoopInput" },
            { "name": "LC", "type": "LoopController", "configuration": {
                "loopController": { "continueCondition": { "name": "go", "expression": "true" } }
            }}
        ]));
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(diags.is_empty(), "got: {diags:?}");
    }

    #[test]
    fn missing_both() {
        let json = loop_json_with_body(serde_json::json!([]));
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        let codes: Vec<_> = diags.iter().map(|d| d.code).collect();
        assert!(codes.contains(&FlowValidationType::MissingLoopInputNode));
        assert!(codes.contains(&FlowValidationType::MissingLoopControllerNode));
    }

    #[test]
    fn multiple_loop_inputs() {
        let json = loop_json_with_body(serde_json::json!([
            { "name": "LI1", "type": "LoopInput" },
            { "name": "LI2", "type": "LoopInput" },
            { "name": "LC", "type": "LoopController", "configuration": {
                "loopController": { "continueCondition": { "name": "go", "expression": "true" } }
            }}
        ]));
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MultipleLoopInputNodes)
        );
    }

    #[test]
    fn forbidden_node_type_inside_loop() {
        let json = loop_json_with_body(serde_json::json!([
            { "name": "LI", "type": "LoopInput" },
            { "name": "LC", "type": "LoopController", "configuration": {
                "loopController": { "continueCondition": { "name": "go", "expression": "true" } }
            }},
            { "name": "Bad", "type": "Input" }
        ]));
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::LoopIncompatibleNodeType)
        );
    }
}
