use std::collections::{HashMap, HashSet, VecDeque};

use winterbaume_bedrock_flow_parser::{FlowDefinition, FlowNodeType};

use crate::{Diagnostic, DiagnosticDetails, FlowValidationType, Severity};

pub fn check(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    check_starting_nodes(def, diagnostics);
    check_ending_nodes(def, diagnostics);
    check_reachability(def, diagnostics);
    check_cycles(def, diagnostics);
}

fn check_starting_nodes(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let has_input = def.nodes.iter().any(|n| n.node_type == FlowNodeType::Input);
    if !has_input {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MissingStartingNodes,
            severity: Severity::Error,
            message: "Flow is missing an Input node".into(),
            details: DiagnosticDetails::MissingStartingNodes,
        });
    }
}

fn check_ending_nodes(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let has_output = def
        .nodes
        .iter()
        .any(|n| n.node_type == FlowNodeType::Output);
    if !has_output {
        diagnostics.push(Diagnostic {
            code: FlowValidationType::MissingEndingNodes,
            severity: Severity::Error,
            message: "Flow is missing an Output node".into(),
            details: DiagnosticDetails::MissingEndingNodes,
        });
    }
}

fn check_reachability(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let node_names: HashSet<&str> = def.nodes.iter().map(|n| n.name.as_str()).collect();
    if node_names.is_empty() {
        return;
    }

    // Build adjacency list from connections.
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
    for conn in &def.connections {
        if node_names.contains(conn.source.as_str()) && node_names.contains(conn.target.as_str()) {
            adj.entry(conn.source.as_str())
                .or_default()
                .push(conn.target.as_str());
        }
    }

    // BFS from all Input nodes.
    let mut visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&str> = VecDeque::new();
    for node in &def.nodes {
        if node.node_type == FlowNodeType::Input {
            visited.insert(&node.name);
            queue.push_back(&node.name);
        }
    }
    while let Some(current) = queue.pop_front() {
        if let Some(neighbors) = adj.get(current) {
            for &next in neighbors {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    for node in &def.nodes {
        if node.node_type != FlowNodeType::Input && !visited.contains(node.name.as_str()) {
            diagnostics.push(Diagnostic {
                code: FlowValidationType::UnreachableNode,
                severity: Severity::Warning,
                message: format!("Node `{}` is not reachable from any Input node", node.name),
                details: DiagnosticDetails::UnreachableNode {
                    node: Some(node.name.clone()),
                },
            });
        }
    }
}

fn check_cycles(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let node_names: HashSet<&str> = def.nodes.iter().map(|n| n.name.as_str()).collect();

    // Build adjacency list.
    let mut adj: HashMap<&str, Vec<(&str, &str)>> = HashMap::new(); // source -> [(target, conn_name)]
    for conn in &def.connections {
        if node_names.contains(conn.source.as_str()) && node_names.contains(conn.target.as_str()) {
            adj.entry(conn.source.as_str())
                .or_default()
                .push((conn.target.as_str(), conn.name.as_str()));
        }
    }

    // DFS-based cycle detection using colouring (White/Grey/Black).
    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Grey,
        Black,
    }

    let mut color: HashMap<&str, Color> = node_names.iter().map(|&n| (n, Color::White)).collect();
    let mut reported = false;

    fn dfs<'a>(
        node: &'a str,
        adj: &HashMap<&'a str, Vec<(&'a str, &'a str)>>,
        color: &mut HashMap<&'a str, Color>,
        diagnostics: &mut Vec<Diagnostic>,
        reported: &mut bool,
    ) {
        color.insert(node, Color::Grey);
        if let Some(neighbors) = adj.get(node) {
            for &(next, conn_name) in neighbors {
                if *reported {
                    return;
                }
                match color.get(next) {
                    Some(Color::Grey) => {
                        // Back edge — cycle detected.
                        diagnostics.push(Diagnostic {
                            code: FlowValidationType::CyclicConnection,
                            severity: Severity::Error,
                            message: format!(
                                "Connection `{conn_name}` creates a cycle in the flow"
                            ),
                            details: DiagnosticDetails::CyclicConnection {
                                connection: Some(conn_name.to_string()),
                            },
                        });
                        *reported = true;
                        return;
                    }
                    Some(Color::White) => {
                        dfs(next, adj, color, diagnostics, reported);
                    }
                    _ => {}
                }
            }
        }
        color.insert(node, Color::Black);
    }

    for &node in &node_names {
        if reported {
            break;
        }
        if color.get(node) == Some(&Color::White) {
            dfs(node, &adj, &mut color, diagnostics, &mut reported);
        }
    }
}

#[cfg(test)]
mod tests {
    use winterbaume_bedrock_flow_parser::parse_flow_definition;

    use super::*;

    #[test]
    fn no_issues_on_valid_flow() {
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
    fn detects_missing_input() {
        let json = serde_json::json!({ "nodes": [{ "name": "Out", "type": "Output" }] });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MissingStartingNodes)
        );
    }

    #[test]
    fn detects_missing_output() {
        let json = serde_json::json!({ "nodes": [{ "name": "In", "type": "Input" }] });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::MissingEndingNodes)
        );
    }

    #[test]
    fn detects_unreachable() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input" },
                { "name": "Out", "type": "Output" },
                { "name": "Island", "type": "Prompt", "configuration": {
                    "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } }
                } }
            ],
            "connections": [{ "name": "c1", "source": "In", "target": "Out", "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        let unreachable: Vec<_> = diags
            .iter()
            .filter(|d| d.code == FlowValidationType::UnreachableNode)
            .collect();
        assert_eq!(unreachable.len(), 1);
    }

    #[test]
    fn detects_cycle() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input", "outputs": [{ "name": "document", "type": "String" }] },
                { "name": "A", "type": "Prompt", "configuration": {
                    "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } } },
                    "inputs": [{ "name": "x", "type": "String", "expression": "$.data" }],
                    "outputs": [{ "name": "modelCompletion", "type": "String" }] },
                { "name": "B", "type": "Prompt", "configuration": {
                    "prompt": { "sourceConfiguration": { "resource": { "promptArn": "arn:x" } } } },
                    "inputs": [{ "name": "x", "type": "String", "expression": "$.data" }],
                    "outputs": [{ "name": "modelCompletion", "type": "String" }] },
                { "name": "Out", "type": "Output" }
            ],
            "connections": [
                { "name": "c0", "source": "In", "target": "A", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "document", "targetInput": "x" } } },
                { "name": "c1", "source": "A", "target": "B", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "modelCompletion", "targetInput": "x" } } },
                { "name": "c2", "source": "B", "target": "A", "type": "Data",
                  "configuration": { "data": { "sourceOutput": "modelCompletion", "targetInput": "x" } } }
            ]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::CyclicConnection)
        );
    }
}
