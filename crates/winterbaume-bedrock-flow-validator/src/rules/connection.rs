use std::collections::{HashMap, HashSet};

use winterbaume_bedrock_flow_parser::{
    FlowConnectionType, FlowDefinition, FlowNodeConfiguration, FlowNodeType,
};

use crate::{Diagnostic, DiagnosticDetails, FlowValidationType, Severity};

pub fn check(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let node_map: HashMap<&str, usize> = def
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.as_str(), i))
        .collect();

    check_connection_endpoints(def, &node_map, diagnostics);
    check_duplicate_connections(def, diagnostics);
    check_multiple_input_connections(def, &node_map, diagnostics);
    check_condition_connections(def, &node_map, diagnostics);
}

fn check_connection_endpoints(
    def: &FlowDefinition,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for conn in &def.connections {
        let source_idx = match node_map.get(conn.source.as_str()) {
            Some(&idx) => Some(idx),
            None => {
                diagnostics.push(Diagnostic {
                    code: FlowValidationType::UnknownConnectionSource,
                    severity: Severity::Error,
                    message: format!(
                        "Connection `{}` references unknown source node `{}`",
                        conn.name, conn.source
                    ),
                    details: DiagnosticDetails::UnknownConnectionSource {
                        connection: Some(conn.name.clone()),
                    },
                });
                None
            }
        };

        let target_idx = match node_map.get(conn.target.as_str()) {
            Some(&idx) => Some(idx),
            None => {
                diagnostics.push(Diagnostic {
                    code: FlowValidationType::UnknownConnectionTarget,
                    severity: Severity::Error,
                    message: format!(
                        "Connection `{}` references unknown target node `{}`",
                        conn.name, conn.target
                    ),
                    details: DiagnosticDetails::UnknownConnectionTarget {
                        connection: Some(conn.name.clone()),
                    },
                });
                None
            }
        };

        // Check source output and target input names for Data connections.
        if let FlowConnectionType::Data {
            ref source_output,
            ref target_input,
        } = conn.connection_type
        {
            if let Some(si) = source_idx {
                let source_node = &def.nodes[si];
                if !source_node.outputs.iter().any(|o| o.name == *source_output) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::UnknownConnectionSourceOutput,
                        severity: Severity::Error,
                        message: format!(
                            "Connection `{}` references unknown output `{}` on source node `{}`",
                            conn.name, source_output, conn.source
                        ),
                        details: DiagnosticDetails::UnknownConnectionSourceOutput {
                            connection: Some(conn.name.clone()),
                        },
                    });
                }
            }

            if let Some(ti) = target_idx {
                let target_node = &def.nodes[ti];
                if !target_node.inputs.iter().any(|i| i.name == *target_input) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::UnknownConnectionTargetInput,
                        severity: Severity::Error,
                        message: format!(
                            "Connection `{}` references unknown input `{}` on target node `{}`",
                            conn.name, target_input, conn.target
                        ),
                        details: DiagnosticDetails::UnknownConnectionTargetInput {
                            connection: Some(conn.name.clone()),
                        },
                    });
                }
            }
        }
    }
}

fn check_duplicate_connections(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let mut seen: HashSet<(&str, &str, Option<(&str, &str)>)> = HashSet::new();
    for conn in &def.connections {
        let key = match &conn.connection_type {
            FlowConnectionType::Data {
                source_output,
                target_input,
            } => (
                conn.source.as_str(),
                conn.target.as_str(),
                Some((source_output.as_str(), target_input.as_str())),
            ),
            FlowConnectionType::Conditional { condition } => (
                conn.source.as_str(),
                conn.target.as_str(),
                Some((condition.as_str(), "")),
            ),
        };
        if !seen.insert(key) {
            diagnostics.push(Diagnostic {
                code: FlowValidationType::DuplicateConnections,
                severity: Severity::Error,
                message: format!(
                    "Duplicate connection between `{}` and `{}`",
                    conn.source, conn.target
                ),
                details: DiagnosticDetails::DuplicateConnections {
                    source: Some(conn.source.clone()),
                    target: Some(conn.target.clone()),
                },
            });
        }
    }
}

fn check_multiple_input_connections(
    def: &FlowDefinition,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // Count how many Data connections target each (node, input) pair.
    let mut counts: HashMap<(&str, &str), u32> = HashMap::new();
    for conn in &def.connections {
        if let FlowConnectionType::Data {
            ref target_input, ..
        } = conn.connection_type
        {
            if node_map.contains_key(conn.target.as_str()) {
                *counts
                    .entry((conn.target.as_str(), target_input.as_str()))
                    .or_insert(0) += 1;
            }
        }
    }
    for ((node, input), count) in &counts {
        if *count > 1 {
            diagnostics.push(Diagnostic {
                code: FlowValidationType::MultipleNodeInputConnections,
                severity: Severity::Error,
                message: format!(
                    "Input `{input}` on node `{node}` has {count} incoming data connections"
                ),
                details: DiagnosticDetails::MultipleNodeInputConnections {
                    node: Some(node.to_string()),
                    input: Some(input.to_string()),
                },
            });
        }
    }
}

fn check_condition_connections(
    def: &FlowDefinition,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    // For Conditional connections, check that:
    // 1. The source node is a Condition node.
    // 2. The condition name matches one of the node's defined conditions.
    // For Condition nodes, check that all conditions have outgoing connections.
    let mut condition_connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for conn in &def.connections {
        if let FlowConnectionType::Conditional { ref condition } = conn.connection_type {
            if let Some(&si) = node_map.get(conn.source.as_str()) {
                let source_node = &def.nodes[si];
                if let Some(FlowNodeConfiguration::Condition(ref cfg)) = source_node.configuration {
                    if !cfg.conditions.iter().any(|c| c.name == *condition) {
                        diagnostics.push(Diagnostic {
                            code: FlowValidationType::UnknownConnectionCondition,
                            severity: Severity::Error,
                            message: format!(
                                "Connection `{}` references unknown condition `{}` on node `{}`",
                                conn.name, condition, conn.source
                            ),
                            details: DiagnosticDetails::UnknownConnectionCondition {
                                connection: Some(conn.name.clone()),
                            },
                        });
                    }
                    condition_connections
                        .entry(conn.source.as_str())
                        .or_default()
                        .insert(condition.as_str());
                }
            }
        }
    }

    // Check all conditions in Condition nodes have outgoing connections.
    for node in &def.nodes {
        if node.node_type != FlowNodeType::Condition {
            continue;
        }
        if let Some(FlowNodeConfiguration::Condition(ref cfg)) = node.configuration {
            let connected = condition_connections
                .get(node.name.as_str())
                .cloned()
                .unwrap_or_default();
            for cond in &cfg.conditions {
                if !connected.contains(cond.name.as_str()) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::UnsatisfiedConnectionConditions,
                        severity: Severity::Error,
                        message: format!(
                            "Condition `{}` on node `{}` has no outgoing connection",
                            cond.name, node.name
                        ),
                        details: DiagnosticDetails::UnsatisfiedConnectionConditions {
                            connection: Some(cond.name.clone()),
                        },
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use winterbaume_bedrock_flow_parser::parse_flow_definition;

    use super::*;

    #[test]
    fn valid_connections() {
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
    fn detects_unknown_source() {
        let json = serde_json::json!({
            "nodes": [{ "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }],
            "connections": [{ "name": "c1", "source": "Ghost", "target": "Out", "type": "Data",
                "configuration": { "data": { "sourceOutput": "x", "targetInput": "document" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnknownConnectionSource)
        );
    }

    #[test]
    fn detects_unknown_source_output() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [{ "name": "c1", "source": "In", "target": "Out", "type": "Data",
                "configuration": { "data": { "sourceOutput": "nonexistent", "targetInput": "document" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnknownConnectionSourceOutput)
        );
    }

    #[test]
    fn detects_duplicate() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "String", "expression": "$.data" }] }
            ],
            "connections": [
                { "name": "c1", "source": "In", "target": "Out", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } },
                { "name": "c2", "source": "In", "target": "Out", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } }
            ]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::DuplicateConnections)
        );
    }

    #[test]
    fn detects_unknown_condition() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Cond", "type": "Condition", "configuration": {
                    "condition": { "conditions": [{ "name": "yes", "expression": "true" }, { "name": "default" }] }
                } },
                { "name": "Out", "type": "Output" }
            ],
            "connections": [{ "name": "c1", "source": "Cond", "target": "Out", "type": "Conditional",
                "configuration": { "conditional": { "condition": "nonexistent" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnknownConnectionCondition)
        );
    }

    #[test]
    fn detects_unsatisfied_conditions() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Cond", "type": "Condition", "configuration": {
                    "condition": { "conditions": [
                        { "name": "yes", "expression": "true" },
                        { "name": "default" }
                    ] }
                } },
                { "name": "Out", "type": "Output" }
            ],
            "connections": [{ "name": "c1", "source": "Cond", "target": "Out", "type": "Conditional",
                "configuration": { "conditional": { "condition": "yes" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        // "default" condition has no outgoing connection.
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::UnsatisfiedConnectionConditions)
        );
    }
}
