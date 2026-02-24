use std::collections::HashMap;

use winterbaume_bedrock_flow_parser::{FlowConnectionType, FlowDataType, FlowDefinition};

use crate::{Diagnostic, DiagnosticDetails, FlowValidationType, Severity};

pub fn check(def: &FlowDefinition, diagnostics: &mut Vec<Diagnostic>) {
    let node_map: HashMap<&str, usize> = def
        .nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.name.as_str(), i))
        .collect();

    check_incompatible_connection_types(def, &node_map, diagnostics);
}

fn check_incompatible_connection_types(
    def: &FlowDefinition,
    node_map: &HashMap<&str, usize>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for conn in &def.connections {
        if let FlowConnectionType::Data {
            ref source_output,
            ref target_input,
        } = conn.connection_type
        {
            let source_type = node_map
                .get(conn.source.as_str())
                .and_then(|&i| {
                    def.nodes[i]
                        .outputs
                        .iter()
                        .find(|o| o.name == *source_output)
                })
                .map(|o| o.data_type);

            let target_type = node_map
                .get(conn.target.as_str())
                .and_then(|&i| {
                    def.nodes[i]
                        .inputs
                        .iter()
                        .find(|inp| inp.name == *target_input)
                })
                .map(|inp| inp.data_type);

            if let (Some(src_t), Some(tgt_t)) = (source_type, target_type) {
                if !types_compatible(src_t, tgt_t) {
                    diagnostics.push(Diagnostic {
                        code: FlowValidationType::IncompatibleConnectionDataType,
                        severity: Severity::Error,
                        message: format!(
                            "Connection `{}`: source output type {} is incompatible with target input type {}",
                            conn.name, src_t, tgt_t
                        ),
                        details: DiagnosticDetails::IncompatibleConnectionDataType {
                            connection: Some(conn.name.clone()),
                        },
                    });
                }
            }
        }
    }
}

/// Check whether two flow data types are compatible across a connection.
///
/// AWS is permissive: a String output can flow into a String input, etc. In
/// practice, most "Any"-typed ports accept all types.  We keep strict
/// same-type matching as the baseline; both String and Object are treated as
/// wildcards since the real runtime coerces freely.
fn types_compatible(source: FlowDataType, target: FlowDataType) -> bool {
    // Same type is always compatible.
    if source == target {
        return true;
    }
    // String is a universal donor/receiver in practice (JSON serialisation).
    if source == FlowDataType::String || target == FlowDataType::String {
        return true;
    }
    // Object is also treated as flexible.
    if source == FlowDataType::Object || target == FlowDataType::Object {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use winterbaume_bedrock_flow_parser::parse_flow_definition;

    use super::*;

    #[test]
    fn compatible_same_types() {
        assert!(types_compatible(FlowDataType::Number, FlowDataType::Number));
        assert!(types_compatible(FlowDataType::Array, FlowDataType::Array));
    }

    #[test]
    fn compatible_string_wildcard() {
        assert!(types_compatible(FlowDataType::String, FlowDataType::Number));
        assert!(types_compatible(
            FlowDataType::Boolean,
            FlowDataType::String
        ));
    }

    #[test]
    fn incompatible_number_boolean() {
        assert!(!types_compatible(
            FlowDataType::Number,
            FlowDataType::Boolean
        ));
    }

    #[test]
    fn incompatible_array_number() {
        assert!(!types_compatible(FlowDataType::Array, FlowDataType::Number));
    }

    #[test]
    fn detects_incompatible_connection() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "In", "type": "Input", "outputs": [{ "name": "document", "type": "Array" }] },
                { "name": "Out", "type": "Output", "inputs": [{ "name": "document", "type": "Number", "expression": "$.data" }] }
            ],
            "connections": [{ "name": "c1", "source": "In", "target": "Out", "type": "Data",
                "configuration": { "data": { "sourceOutput": "document", "targetInput": "document" } } }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let mut diags = Vec::new();
        check(&def, &mut diags);
        assert!(
            diags
                .iter()
                .any(|d| d.code == FlowValidationType::IncompatibleConnectionDataType)
        );
    }

    #[test]
    fn no_issue_on_compatible() {
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
        assert!(diags.is_empty());
    }
}
