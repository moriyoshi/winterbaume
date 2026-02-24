use serde_json::Value;

use crate::error::ParseError;
use crate::types::*;

/// Parse a Bedrock Agent flow definition JSON value into a typed [`FlowDefinition`].
pub fn parse_flow_definition(value: &Value) -> Result<FlowDefinition, ParseError> {
    let obj = value.as_object().ok_or_else(|| ParseError::TypeError {
        field: "definition".into(),
        expected: "object".into(),
        actual: json_type_name(value).into(),
    })?;

    let nodes = match obj.get("nodes") {
        Some(v) => {
            let arr = v.as_array().ok_or_else(|| ParseError::TypeError {
                field: "nodes".into(),
                expected: "array".into(),
                actual: json_type_name(v).into(),
            })?;
            arr.iter().map(parse_node).collect::<Result<Vec<_>, _>>()?
        }
        None => vec![],
    };

    let connections = match obj.get("connections") {
        Some(v) => {
            let arr = v.as_array().ok_or_else(|| ParseError::TypeError {
                field: "connections".into(),
                expected: "array".into(),
                actual: json_type_name(v).into(),
            })?;
            arr.iter()
                .map(parse_connection)
                .collect::<Result<Vec<_>, _>>()?
        }
        None => vec![],
    };

    Ok(FlowDefinition { nodes, connections })
}

fn parse_node(v: &Value) -> Result<FlowNode, ParseError> {
    let name = str_field(v, "name", "FlowNode")?.to_owned();
    let type_str = str_field(v, "type", &format!("FlowNode `{name}`"))?;
    let node_type = parse_node_type(type_str)?;

    let configuration = match v.get("configuration") {
        Some(cfg) if !cfg.is_null() => Some(parse_node_configuration(&node_type, cfg, &name)?),
        _ => None,
    };

    let inputs = match v.get("inputs") {
        Some(arr_val) => {
            let arr = arr_val.as_array().ok_or_else(|| ParseError::TypeError {
                field: "inputs".into(),
                expected: "array".into(),
                actual: json_type_name(arr_val).into(),
            })?;
            arr.iter()
                .map(|i| parse_node_input(i, &name))
                .collect::<Result<Vec<_>, _>>()?
        }
        None => vec![],
    };

    let outputs = match v.get("outputs") {
        Some(arr_val) => {
            let arr = arr_val.as_array().ok_or_else(|| ParseError::TypeError {
                field: "outputs".into(),
                expected: "array".into(),
                actual: json_type_name(arr_val).into(),
            })?;
            arr.iter()
                .map(|o| parse_node_output(o, &name))
                .collect::<Result<Vec<_>, _>>()?
        }
        None => vec![],
    };

    Ok(FlowNode {
        name,
        node_type,
        configuration,
        inputs,
        outputs,
    })
}

fn parse_node_type(s: &str) -> Result<FlowNodeType, ParseError> {
    match s {
        "Input" => Ok(FlowNodeType::Input),
        "Output" => Ok(FlowNodeType::Output),
        "Condition" => Ok(FlowNodeType::Condition),
        "KnowledgeBase" => Ok(FlowNodeType::KnowledgeBase),
        "Prompt" => Ok(FlowNodeType::Prompt),
        "LambdaFunction" => Ok(FlowNodeType::LambdaFunction),
        "Lex" => Ok(FlowNodeType::Lex),
        "Agent" => Ok(FlowNodeType::Agent),
        "Storage" => Ok(FlowNodeType::Storage),
        "Retrieval" => Ok(FlowNodeType::Retrieval),
        "Iterator" => Ok(FlowNodeType::Iterator),
        "Collector" => Ok(FlowNodeType::Collector),
        "InlineCode" => Ok(FlowNodeType::InlineCode),
        "Loop" => Ok(FlowNodeType::Loop),
        "LoopInput" => Ok(FlowNodeType::LoopInput),
        "LoopController" => Ok(FlowNodeType::LoopController),
        other => Err(ParseError::UnknownNodeType(other.to_owned())),
    }
}

fn parse_node_configuration(
    node_type: &FlowNodeType,
    v: &Value,
    node_name: &str,
) -> Result<FlowNodeConfiguration, ParseError> {
    let ctx = || format!("FlowNode `{node_name}` configuration");

    match node_type {
        FlowNodeType::Input => Ok(FlowNodeConfiguration::Input),
        FlowNodeType::Output => Ok(FlowNodeConfiguration::Output),
        FlowNodeType::Iterator => Ok(FlowNodeConfiguration::Iterator),
        FlowNodeType::Collector => Ok(FlowNodeConfiguration::Collector),
        FlowNodeType::LoopInput => Ok(FlowNodeConfiguration::LoopInput),

        FlowNodeType::Condition => {
            let inner = v.get("condition").ok_or_else(|| ParseError::MissingField {
                field: "condition".into(),
                context: ctx(),
            })?;
            let conditions = parse_conditions_array(inner, node_name)?;
            Ok(FlowNodeConfiguration::Condition(ConditionConfig {
                conditions,
            }))
        }

        FlowNodeType::KnowledgeBase => {
            let inner = v
                .get("knowledgeBase")
                .ok_or_else(|| ParseError::MissingField {
                    field: "knowledgeBase".into(),
                    context: ctx(),
                })?;
            let knowledge_base_id = str_field(inner, "knowledgeBaseId", &ctx())?.to_owned();
            let model_id = inner
                .get("modelId")
                .and_then(|v| v.as_str())
                .map(String::from);
            Ok(FlowNodeConfiguration::KnowledgeBase(KnowledgeBaseConfig {
                knowledge_base_id,
                model_id,
                extra: inner.clone(),
            }))
        }

        FlowNodeType::Prompt => {
            let inner = v.get("prompt").ok_or_else(|| ParseError::MissingField {
                field: "prompt".into(),
                context: ctx(),
            })?;
            let source_cfg =
                inner
                    .get("sourceConfiguration")
                    .ok_or_else(|| ParseError::MissingField {
                        field: "sourceConfiguration".into(),
                        context: ctx(),
                    })?;
            let source = parse_prompt_source(source_cfg, node_name)?;
            Ok(FlowNodeConfiguration::Prompt(PromptConfig {
                source,
                extra: inner.clone(),
            }))
        }

        FlowNodeType::LambdaFunction => {
            let inner = v
                .get("lambdaFunction")
                .ok_or_else(|| ParseError::MissingField {
                    field: "lambdaFunction".into(),
                    context: ctx(),
                })?;
            let lambda_arn = str_field(inner, "lambdaArn", &ctx())?.to_owned();
            Ok(FlowNodeConfiguration::LambdaFunction(
                LambdaFunctionConfig { lambda_arn },
            ))
        }

        FlowNodeType::Lex => {
            let inner = v.get("lex").ok_or_else(|| ParseError::MissingField {
                field: "lex".into(),
                context: ctx(),
            })?;
            let bot_alias_arn = str_field(inner, "botAliasArn", &ctx())?.to_owned();
            let locale_id = str_field(inner, "localeId", &ctx())?.to_owned();
            Ok(FlowNodeConfiguration::Lex(LexConfig {
                bot_alias_arn,
                locale_id,
            }))
        }

        FlowNodeType::Agent => {
            let inner = v.get("agent").ok_or_else(|| ParseError::MissingField {
                field: "agent".into(),
                context: ctx(),
            })?;
            let agent_alias_arn = str_field(inner, "agentAliasArn", &ctx())?.to_owned();
            Ok(FlowNodeConfiguration::Agent(AgentConfig {
                agent_alias_arn,
            }))
        }

        FlowNodeType::Storage => {
            let inner = v.get("storage").ok_or_else(|| ParseError::MissingField {
                field: "storage".into(),
                context: ctx(),
            })?;
            let bucket_name = extract_s3_bucket(inner, &ctx())?;
            Ok(FlowNodeConfiguration::Storage(StorageConfig {
                bucket_name,
            }))
        }

        FlowNodeType::Retrieval => {
            let inner = v.get("retrieval").ok_or_else(|| ParseError::MissingField {
                field: "retrieval".into(),
                context: ctx(),
            })?;
            let bucket_name = extract_s3_bucket(inner, &ctx())?;
            Ok(FlowNodeConfiguration::Retrieval(RetrievalConfig {
                bucket_name,
            }))
        }

        FlowNodeType::InlineCode => {
            let inner = v
                .get("inlineCode")
                .ok_or_else(|| ParseError::MissingField {
                    field: "inlineCode".into(),
                    context: ctx(),
                })?;
            let code = str_field(inner, "code", &ctx())?.to_owned();
            let language = str_field(inner, "language", &ctx())?.to_owned();
            Ok(FlowNodeConfiguration::InlineCode(InlineCodeConfig {
                code,
                language,
            }))
        }

        FlowNodeType::Loop => {
            let inner = v.get("loop").ok_or_else(|| ParseError::MissingField {
                field: "loop".into(),
                context: ctx(),
            })?;
            let def_val = inner
                .get("definition")
                .ok_or_else(|| ParseError::MissingField {
                    field: "definition".into(),
                    context: ctx(),
                })?;
            let definition = parse_flow_definition(def_val)?;
            Ok(FlowNodeConfiguration::Loop(LoopConfig {
                definition: Box::new(definition),
            }))
        }

        FlowNodeType::LoopController => {
            let inner = v
                .get("loopController")
                .ok_or_else(|| ParseError::MissingField {
                    field: "loopController".into(),
                    context: ctx(),
                })?;
            let cond_val =
                inner
                    .get("continueCondition")
                    .ok_or_else(|| ParseError::MissingField {
                        field: "continueCondition".into(),
                        context: ctx(),
                    })?;
            let continue_condition = parse_condition(cond_val, node_name)?;
            let max_iterations = inner
                .get("maxIterations")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32);
            Ok(FlowNodeConfiguration::LoopController(
                LoopControllerConfig {
                    continue_condition,
                    max_iterations,
                },
            ))
        }
    }
}

fn parse_prompt_source(v: &Value, node_name: &str) -> Result<PromptSource, ParseError> {
    let ctx = format!("FlowNode `{node_name}` sourceConfiguration");
    if let Some(inline) = v.get("inline") {
        let model_id = str_field(inline, "modelId", &ctx)?.to_owned();
        let template_type = str_field(inline, "templateType", &ctx)?.to_owned();
        return Ok(PromptSource::Inline {
            model_id,
            template_type,
            extra: inline.clone(),
        });
    }
    if let Some(resource) = v.get("resource") {
        let prompt_arn = str_field(resource, "promptArn", &ctx)?.to_owned();
        return Ok(PromptSource::Resource { prompt_arn });
    }
    Err(ParseError::MissingField {
        field: "inline or resource".into(),
        context: ctx,
    })
}

fn parse_conditions_array(v: &Value, node_name: &str) -> Result<Vec<FlowCondition>, ParseError> {
    let conditions_val = v
        .get("conditions")
        .ok_or_else(|| ParseError::MissingField {
            field: "conditions".into(),
            context: format!("FlowNode `{node_name}` condition configuration"),
        })?;
    let arr = conditions_val
        .as_array()
        .ok_or_else(|| ParseError::TypeError {
            field: "conditions".into(),
            expected: "array".into(),
            actual: json_type_name(conditions_val).into(),
        })?;
    arr.iter().map(|c| parse_condition(c, node_name)).collect()
}

fn parse_condition(v: &Value, node_name: &str) -> Result<FlowCondition, ParseError> {
    let name = str_field(v, "name", &format!("FlowCondition in `{node_name}`"))?.to_owned();
    let expression = v
        .get("expression")
        .and_then(|e| e.as_str())
        .map(String::from);
    Ok(FlowCondition { name, expression })
}

fn extract_s3_bucket(v: &Value, ctx: &str) -> Result<String, ParseError> {
    let svc = v
        .get("serviceConfiguration")
        .ok_or_else(|| ParseError::MissingField {
            field: "serviceConfiguration".into(),
            context: ctx.to_owned(),
        })?;
    let s3 = svc.get("s3").ok_or_else(|| ParseError::MissingField {
        field: "serviceConfiguration.s3".into(),
        context: ctx.to_owned(),
    })?;
    Ok(str_field(s3, "bucketName", ctx)?.to_owned())
}

fn parse_node_input(v: &Value, node_name: &str) -> Result<FlowNodeInput, ParseError> {
    let ctx = format!("FlowNodeInput in `{node_name}`");
    let name = str_field(v, "name", &ctx)?.to_owned();
    let type_str = str_field(v, "type", &ctx)?;
    let data_type = parse_data_type(type_str)?;
    let expression = str_field(v, "expression", &ctx)?.to_owned();
    let category = v.get("category").and_then(|c| c.as_str()).map(String::from);
    Ok(FlowNodeInput {
        name,
        data_type,
        expression,
        category,
    })
}

fn parse_node_output(v: &Value, node_name: &str) -> Result<FlowNodeOutput, ParseError> {
    let ctx = format!("FlowNodeOutput in `{node_name}`");
    let name = str_field(v, "name", &ctx)?.to_owned();
    let type_str = str_field(v, "type", &ctx)?;
    let data_type = parse_data_type(type_str)?;
    Ok(FlowNodeOutput { name, data_type })
}

fn parse_connection(v: &Value) -> Result<FlowConnection, ParseError> {
    let name = str_field(v, "name", "FlowConnection")?.to_owned();
    let source = str_field(v, "source", &format!("FlowConnection `{name}`"))?.to_owned();
    let target = str_field(v, "target", &format!("FlowConnection `{name}`"))?.to_owned();
    let type_str = str_field(v, "type", &format!("FlowConnection `{name}`"))?;
    let connection_type = parse_connection_type(v, type_str, &name)?;

    Ok(FlowConnection {
        name,
        source,
        target,
        connection_type,
    })
}

fn parse_connection_type(
    v: &Value,
    type_str: &str,
    conn_name: &str,
) -> Result<FlowConnectionType, ParseError> {
    let ctx = format!("FlowConnection `{conn_name}`");
    match type_str {
        "Data" => {
            let cfg = v
                .get("configuration")
                .ok_or_else(|| ParseError::MissingField {
                    field: "configuration".into(),
                    context: ctx.clone(),
                })?;
            let data = cfg.get("data").ok_or_else(|| ParseError::MissingField {
                field: "configuration.data".into(),
                context: ctx.clone(),
            })?;
            let source_output = str_field(data, "sourceOutput", &ctx)?.to_owned();
            let target_input = str_field(data, "targetInput", &ctx)?.to_owned();
            Ok(FlowConnectionType::Data {
                source_output,
                target_input,
            })
        }
        "Conditional" => {
            let cfg = v
                .get("configuration")
                .ok_or_else(|| ParseError::MissingField {
                    field: "configuration".into(),
                    context: ctx.clone(),
                })?;
            let cond = cfg
                .get("conditional")
                .ok_or_else(|| ParseError::MissingField {
                    field: "configuration.conditional".into(),
                    context: ctx.clone(),
                })?;
            let condition = str_field(cond, "condition", &ctx)?.to_owned();
            Ok(FlowConnectionType::Conditional { condition })
        }
        other => Err(ParseError::UnknownConnectionType(other.to_owned())),
    }
}

fn parse_data_type(s: &str) -> Result<FlowDataType, ParseError> {
    match s {
        "String" => Ok(FlowDataType::String),
        "Number" => Ok(FlowDataType::Number),
        "Boolean" => Ok(FlowDataType::Boolean),
        "Object" => Ok(FlowDataType::Object),
        "Array" => Ok(FlowDataType::Array),
        other => Err(ParseError::UnknownDataType(other.to_owned())),
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────

fn str_field<'a>(v: &'a Value, field: &str, context: &str) -> Result<&'a str, ParseError> {
    v.get(field)
        .and_then(|f| f.as_str())
        .ok_or_else(|| ParseError::MissingField {
            field: field.into(),
            context: context.into(),
        })
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_flow_json() -> Value {
        serde_json::json!({
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
            "connections": [
                {
                    "name": "start_to_end",
                    "source": "Start",
                    "target": "End",
                    "type": "Data",
                    "configuration": {
                        "data": { "sourceOutput": "document", "targetInput": "document" }
                    }
                }
            ]
        })
    }

    #[test]
    fn parse_minimal_flow() {
        let def = parse_flow_definition(&minimal_flow_json()).unwrap();
        assert_eq!(def.nodes.len(), 2);
        assert_eq!(def.connections.len(), 1);

        assert_eq!(def.nodes[0].name, "Start");
        assert_eq!(def.nodes[0].node_type, FlowNodeType::Input);
        assert_eq!(def.nodes[0].outputs.len(), 1);
        assert!(def.nodes[0].inputs.is_empty());

        assert_eq!(def.nodes[1].name, "End");
        assert_eq!(def.nodes[1].node_type, FlowNodeType::Output);
        assert_eq!(def.nodes[1].inputs.len(), 1);
        assert_eq!(def.nodes[1].inputs[0].expression, "$.data");

        let conn = &def.connections[0];
        assert_eq!(conn.source, "Start");
        assert_eq!(conn.target, "End");
        match &conn.connection_type {
            FlowConnectionType::Data {
                source_output,
                target_input,
            } => {
                assert_eq!(source_output, "document");
                assert_eq!(target_input, "document");
            }
            _ => panic!("expected Data connection"),
        }
    }

    #[test]
    fn parse_empty_definition() {
        let def = parse_flow_definition(&serde_json::json!({})).unwrap();
        assert!(def.nodes.is_empty());
        assert!(def.connections.is_empty());
    }

    #[test]
    fn error_on_non_object() {
        let err = parse_flow_definition(&serde_json::json!("string")).unwrap_err();
        assert!(matches!(err, ParseError::TypeError { .. }));
    }

    #[test]
    fn error_on_unknown_node_type() {
        let json = serde_json::json!({
            "nodes": [{ "name": "N", "type": "Bogus" }]
        });
        let err = parse_flow_definition(&json).unwrap_err();
        assert!(matches!(err, ParseError::UnknownNodeType(ref s) if s == "Bogus"));
    }

    #[test]
    fn error_on_missing_node_name() {
        let json = serde_json::json!({
            "nodes": [{ "type": "Input" }]
        });
        let err = parse_flow_definition(&json).unwrap_err();
        assert!(matches!(err, ParseError::MissingField { ref field, .. } if field == "name"));
    }

    #[test]
    fn error_on_unknown_data_type() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "N",
                "type": "Input",
                "outputs": [{ "name": "out", "type": "Float" }]
            }]
        });
        let err = parse_flow_definition(&json).unwrap_err();
        assert!(matches!(err, ParseError::UnknownDataType(ref s) if s == "Float"));
    }

    #[test]
    fn error_on_unknown_connection_type() {
        let json = serde_json::json!({
            "connections": [{
                "name": "c",
                "source": "A",
                "target": "B",
                "type": "Magic"
            }]
        });
        let err = parse_flow_definition(&json).unwrap_err();
        assert!(matches!(err, ParseError::UnknownConnectionType(ref s) if s == "Magic"));
    }

    #[test]
    fn error_on_data_connection_missing_config() {
        let json = serde_json::json!({
            "connections": [{
                "name": "c",
                "source": "A",
                "target": "B",
                "type": "Data"
            }]
        });
        let err = parse_flow_definition(&json).unwrap_err();
        assert!(matches!(err, ParseError::MissingField { .. }));
    }

    #[test]
    fn parse_condition_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "Cond",
                "type": "Condition",
                "configuration": {
                    "condition": {
                        "conditions": [
                            { "name": "is_big", "expression": "$.size > 100" },
                            { "name": "default" }
                        ]
                    }
                },
                "inputs": [{ "name": "size", "type": "Number", "expression": "$.data.size" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        let node = &def.nodes[0];
        assert_eq!(node.node_type, FlowNodeType::Condition);
        match &node.configuration {
            Some(FlowNodeConfiguration::Condition(cfg)) => {
                assert_eq!(cfg.conditions.len(), 2);
                assert_eq!(cfg.conditions[0].name, "is_big");
                assert!(cfg.conditions[0].expression.is_some());
                assert_eq!(cfg.conditions[1].name, "default");
                assert!(cfg.conditions[1].expression.is_none());
            }
            other => panic!("expected Condition config, got {other:?}"),
        }
    }

    #[test]
    fn parse_conditional_connection() {
        let json = serde_json::json!({
            "connections": [{
                "name": "branch",
                "source": "Cond",
                "target": "Next",
                "type": "Conditional",
                "configuration": {
                    "conditional": { "condition": "is_big" }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.connections[0].connection_type {
            FlowConnectionType::Conditional { condition } => {
                assert_eq!(condition, "is_big");
            }
            _ => panic!("expected Conditional connection"),
        }
    }

    #[test]
    fn parse_lambda_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "Lambda",
                "type": "LambdaFunction",
                "configuration": {
                    "lambdaFunction": { "lambdaArn": "arn:aws:lambda:us-east-1:123:function:my-fn" }
                },
                "inputs": [{ "name": "codeHookInput", "type": "String", "expression": "$.data" }],
                "outputs": [{ "name": "functionResponse", "type": "String" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::LambdaFunction(cfg)) => {
                assert_eq!(
                    cfg.lambda_arn,
                    "arn:aws:lambda:us-east-1:123:function:my-fn"
                );
            }
            other => panic!("expected LambdaFunction config, got {other:?}"),
        }
    }

    #[test]
    fn parse_prompt_inline_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "MyPrompt",
                "type": "Prompt",
                "configuration": {
                    "prompt": {
                        "sourceConfiguration": {
                            "inline": {
                                "modelId": "anthropic.claude-3-haiku-20240307-v1:0",
                                "templateType": "TEXT",
                                "templateConfiguration": {
                                    "text": { "text": "Hello {{name}}" }
                                }
                            }
                        }
                    }
                },
                "inputs": [{ "name": "name", "type": "String", "expression": "$.data.name" }],
                "outputs": [{ "name": "modelCompletion", "type": "String" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Prompt(cfg)) => match &cfg.source {
                PromptSource::Inline {
                    model_id,
                    template_type,
                    ..
                } => {
                    assert_eq!(model_id, "anthropic.claude-3-haiku-20240307-v1:0");
                    assert_eq!(template_type, "TEXT");
                }
                other => panic!("expected Inline source, got {other:?}"),
            },
            other => panic!("expected Prompt config, got {other:?}"),
        }
    }

    #[test]
    fn parse_prompt_resource_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "MyPrompt",
                "type": "Prompt",
                "configuration": {
                    "prompt": {
                        "sourceConfiguration": {
                            "resource": {
                                "promptArn": "arn:aws:bedrock:us-east-1:123:prompt/ABCDEF"
                            }
                        }
                    }
                },
                "outputs": [{ "name": "modelCompletion", "type": "String" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Prompt(cfg)) => match &cfg.source {
                PromptSource::Resource { prompt_arn } => {
                    assert_eq!(prompt_arn, "arn:aws:bedrock:us-east-1:123:prompt/ABCDEF");
                }
                other => panic!("expected Resource source, got {other:?}"),
            },
            other => panic!("expected Prompt config, got {other:?}"),
        }
    }

    #[test]
    fn parse_knowledge_base_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "KB",
                "type": "KnowledgeBase",
                "configuration": {
                    "knowledgeBase": {
                        "knowledgeBaseId": "ABCDEF1234",
                        "modelId": "anthropic.claude-v2"
                    }
                },
                "inputs": [{ "name": "retrievalQuery", "type": "String", "expression": "$.data" }],
                "outputs": [{ "name": "outputText", "type": "String" }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::KnowledgeBase(cfg)) => {
                assert_eq!(cfg.knowledge_base_id, "ABCDEF1234");
                assert_eq!(cfg.model_id.as_deref(), Some("anthropic.claude-v2"));
            }
            other => panic!("expected KnowledgeBase config, got {other:?}"),
        }
    }

    #[test]
    fn parse_lex_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "LexBot",
                "type": "Lex",
                "configuration": {
                    "lex": {
                        "botAliasArn": "arn:aws:lex:us-east-1:123:bot-alias/BOT/ALIAS",
                        "localeId": "en_US"
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Lex(cfg)) => {
                assert_eq!(cfg.locale_id, "en_US");
            }
            other => panic!("expected Lex config, got {other:?}"),
        }
    }

    #[test]
    fn parse_agent_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "MyAgent",
                "type": "Agent",
                "configuration": {
                    "agent": {
                        "agentAliasArn": "arn:aws:bedrock:us-east-1:123:agent-alias/AGENT/ALIAS"
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Agent(cfg)) => {
                assert_eq!(
                    cfg.agent_alias_arn,
                    "arn:aws:bedrock:us-east-1:123:agent-alias/AGENT/ALIAS"
                );
            }
            other => panic!("expected Agent config, got {other:?}"),
        }
    }

    #[test]
    fn parse_storage_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "S3Store",
                "type": "Storage",
                "configuration": {
                    "storage": {
                        "serviceConfiguration": {
                            "s3": { "bucketName": "my-bucket" }
                        }
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Storage(cfg)) => {
                assert_eq!(cfg.bucket_name, "my-bucket");
            }
            other => panic!("expected Storage config, got {other:?}"),
        }
    }

    #[test]
    fn parse_retrieval_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "S3Get",
                "type": "Retrieval",
                "configuration": {
                    "retrieval": {
                        "serviceConfiguration": {
                            "s3": { "bucketName": "my-bucket" }
                        }
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Retrieval(cfg)) => {
                assert_eq!(cfg.bucket_name, "my-bucket");
            }
            other => panic!("expected Retrieval config, got {other:?}"),
        }
    }

    #[test]
    fn parse_inline_code_node() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "Code",
                "type": "InlineCode",
                "configuration": {
                    "inlineCode": {
                        "code": "x + 1",
                        "language": "Python_3"
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::InlineCode(cfg)) => {
                assert_eq!(cfg.code, "x + 1");
                assert_eq!(cfg.language, "Python_3");
            }
            other => panic!("expected InlineCode config, got {other:?}"),
        }
    }

    #[test]
    fn parse_iterator_collector_nodes() {
        let json = serde_json::json!({
            "nodes": [
                { "name": "Iter", "type": "Iterator" },
                { "name": "Coll", "type": "Collector" }
            ]
        });
        let def = parse_flow_definition(&json).unwrap();
        assert_eq!(def.nodes[0].node_type, FlowNodeType::Iterator);
        assert!(def.nodes[0].configuration.is_none());
        assert_eq!(def.nodes[1].node_type, FlowNodeType::Collector);
    }

    #[test]
    fn parse_loop_with_sub_definition() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "MyLoop",
                "type": "Loop",
                "configuration": {
                    "loop": {
                        "definition": {
                            "nodes": [
                                { "name": "LI", "type": "LoopInput" },
                                {
                                    "name": "LC",
                                    "type": "LoopController",
                                    "configuration": {
                                        "loopController": {
                                            "continueCondition": {
                                                "name": "keep_going",
                                                "expression": "$.count < 10"
                                            },
                                            "maxIterations": 50
                                        }
                                    }
                                }
                            ],
                            "connections": []
                        }
                    }
                }
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        match &def.nodes[0].configuration {
            Some(FlowNodeConfiguration::Loop(cfg)) => {
                assert_eq!(cfg.definition.nodes.len(), 2);
                assert_eq!(cfg.definition.nodes[0].node_type, FlowNodeType::LoopInput);
                match &cfg.definition.nodes[1].configuration {
                    Some(FlowNodeConfiguration::LoopController(lc)) => {
                        assert_eq!(lc.continue_condition.name, "keep_going");
                        assert_eq!(lc.max_iterations, Some(50));
                    }
                    other => panic!("expected LoopController config, got {other:?}"),
                }
            }
            other => panic!("expected Loop config, got {other:?}"),
        }
    }

    #[test]
    fn parse_node_input_with_category() {
        let json = serde_json::json!({
            "nodes": [{
                "name": "LC",
                "type": "LoopController",
                "configuration": {
                    "loopController": {
                        "continueCondition": { "name": "cond", "expression": "true" }
                    }
                },
                "inputs": [{
                    "name": "check",
                    "type": "Boolean",
                    "expression": "$.done",
                    "category": "LoopCondition"
                }]
            }]
        });
        let def = parse_flow_definition(&json).unwrap();
        assert_eq!(
            def.nodes[0].inputs[0].category.as_deref(),
            Some("LoopCondition")
        );
    }

    #[test]
    fn parse_all_data_types() {
        for (type_str, expected) in [
            ("String", FlowDataType::String),
            ("Number", FlowDataType::Number),
            ("Boolean", FlowDataType::Boolean),
            ("Object", FlowDataType::Object),
            ("Array", FlowDataType::Array),
        ] {
            let json = serde_json::json!({
                "nodes": [{
                    "name": "N",
                    "type": "Input",
                    "outputs": [{ "name": "out", "type": type_str }]
                }]
            });
            let def = parse_flow_definition(&json).unwrap();
            assert_eq!(def.nodes[0].outputs[0].data_type, expected);
        }
    }

    #[test]
    fn parse_all_node_types() {
        let types = [
            "Input",
            "Output",
            "Condition",
            "KnowledgeBase",
            "Prompt",
            "LambdaFunction",
            "Lex",
            "Agent",
            "Storage",
            "Retrieval",
            "Iterator",
            "Collector",
            "InlineCode",
            "Loop",
            "LoopInput",
            "LoopController",
        ];
        for t in types {
            assert!(parse_node_type(t).is_ok(), "failed to parse node type: {t}");
        }
    }
}
