use serde_json::Value;

use crate::error::ParseError;
use crate::types::*;

/// Parse a WAFv2 `Rules` JSON array into typed [`Rule`] structs.
pub fn parse_rules(rules: &Value) -> Result<Vec<Rule>, ParseError> {
    let arr = rules.as_array().ok_or_else(|| ParseError::TypeError {
        field: "Rules".into(),
        expected: "array".into(),
        actual: json_type_name(rules).into(),
    })?;
    arr.iter().map(parse_rule).collect()
}

fn parse_rule(v: &Value) -> Result<Rule, ParseError> {
    let name = str_field(v, "Name", "Rule")?.to_owned();
    let priority = v.get("Priority").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
    let statement =
        parse_statement(v.get("Statement").ok_or_else(|| ParseError::MissingField {
            field: "Statement".into(),
            context: format!("Rule `{name}`"),
        })?)?;
    let action = v.get("Action").cloned();
    let override_action = v.get("OverrideAction").cloned();
    let visibility_config = v
        .get("VisibilityConfig")
        .cloned()
        .unwrap_or(Value::Object(Default::default()));
    let rule_labels = v
        .get("RuleLabels")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|l| l.get("Name").and_then(|n| n.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(Rule {
        name,
        priority,
        statement,
        action,
        override_action,
        visibility_config,
        rule_labels,
    })
}

/// Parse a single Statement JSON object into the typed [`Statement`] enum.
pub fn parse_statement(v: &Value) -> Result<Statement, ParseError> {
    let obj = v.as_object().ok_or_else(|| ParseError::TypeError {
        field: "Statement".into(),
        expected: "object".into(),
        actual: json_type_name(v).into(),
    })?;

    // The wire format is a struct-of-optionals: exactly one key should be present.
    // We check them in order and return the first match.

    if let Some(inner) = obj.get("ByteMatchStatement") {
        return parse_byte_match(inner);
    }
    if let Some(inner) = obj.get("SqliMatchStatement") {
        return parse_sqli_match(inner);
    }
    if let Some(inner) = obj.get("XssMatchStatement") {
        return parse_xss_match(inner);
    }
    if let Some(inner) = obj.get("SizeConstraintStatement") {
        return parse_size_constraint(inner);
    }
    if let Some(inner) = obj.get("RegexMatchStatement") {
        return parse_regex_match(inner);
    }
    if let Some(inner) = obj.get("RegexPatternSetReferenceStatement") {
        return parse_regex_pattern_set_ref(inner);
    }
    if let Some(inner) = obj.get("IPSetReferenceStatement") {
        return parse_ip_set_ref(inner);
    }
    if let Some(inner) = obj.get("GeoMatchStatement") {
        return parse_geo_match(inner);
    }
    if let Some(inner) = obj.get("AsnMatchStatement") {
        return parse_asn_match(inner);
    }
    if let Some(inner) = obj.get("LabelMatchStatement") {
        return parse_label_match(inner);
    }
    if let Some(inner) = obj.get("ManagedRuleGroupStatement") {
        return parse_managed_rule_group(inner);
    }
    if let Some(inner) = obj.get("RuleGroupReferenceStatement") {
        return parse_rule_group_ref(inner);
    }
    if let Some(inner) = obj.get("RateBasedStatement") {
        return parse_rate_based(inner);
    }
    if let Some(inner) = obj.get("AndStatement") {
        return parse_and(inner);
    }
    if let Some(inner) = obj.get("OrStatement") {
        return parse_or(inner);
    }
    if let Some(inner) = obj.get("NotStatement") {
        return parse_not(inner);
    }

    Err(ParseError::EmptyStatement)
}

// ── Leaf statement parsers ──

fn parse_byte_match(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::ByteMatch {
        field_to_match: extract_field_to_match(v, "ByteMatchStatement")?,
        positional_constraint: str_field(v, "PositionalConstraint", "ByteMatchStatement")?
            .to_owned(),
        search_string: str_field(v, "SearchString", "ByteMatchStatement")?.to_owned(),
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_sqli_match(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::SqliMatch {
        field_to_match: extract_field_to_match(v, "SqliMatchStatement")?,
        sensitivity_level: v
            .get("SensitivityLevel")
            .and_then(|s| s.as_str())
            .map(String::from),
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_xss_match(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::XssMatch {
        field_to_match: extract_field_to_match(v, "XssMatchStatement")?,
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_size_constraint(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::SizeConstraint {
        field_to_match: extract_field_to_match(v, "SizeConstraintStatement")?,
        comparison_operator: str_field(v, "ComparisonOperator", "SizeConstraintStatement")?
            .to_owned(),
        size: v.get("Size").and_then(|s| s.as_i64()).unwrap_or(0),
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_regex_match(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::RegexMatch {
        field_to_match: extract_field_to_match(v, "RegexMatchStatement")?,
        regex_string: str_field(v, "RegexString", "RegexMatchStatement")?.to_owned(),
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_regex_pattern_set_ref(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::RegexPatternSetReference {
        arn: str_field(v, "ARN", "RegexPatternSetReferenceStatement")?.to_owned(),
        field_to_match: extract_field_to_match(v, "RegexPatternSetReferenceStatement")?,
        text_transformations: extract_text_transformations(v),
    })
}

fn parse_ip_set_ref(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::IpSetReference {
        arn: str_field(v, "ARN", "IPSetReferenceStatement")?.to_owned(),
        forwarded_ip_config: v.get("IPSetForwardedIPConfig").cloned(),
    })
}

fn parse_geo_match(v: &Value) -> Result<Statement, ParseError> {
    let country_codes = v
        .get("CountryCodes")
        .and_then(|c| c.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|c| c.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    Ok(Statement::GeoMatch {
        country_codes,
        forwarded_ip_config: v.get("ForwardedIPConfig").cloned(),
    })
}

fn parse_asn_match(v: &Value) -> Result<Statement, ParseError> {
    let asn_list = v
        .get("AsnList")
        .and_then(|a| a.as_array())
        .map(|arr| arr.iter().filter_map(|a| a.as_i64()).collect())
        .unwrap_or_default();
    Ok(Statement::AsnMatch {
        asn_list,
        forwarded_ip_config: v.get("ForwardedIPConfig").cloned(),
    })
}

fn parse_label_match(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::LabelMatch {
        scope: str_field(v, "Scope", "LabelMatchStatement")?.to_owned(),
        key: str_field(v, "Key", "LabelMatchStatement")?.to_owned(),
    })
}

// ── Compound / group statement parsers ──

fn parse_managed_rule_group(v: &Value) -> Result<Statement, ParseError> {
    let scope_down = match v.get("ScopeDownStatement") {
        Some(s) => Some(Box::new(parse_statement(s)?)),
        None => None,
    };
    let excluded_rules = extract_excluded_rules(v);
    // Build an extra value with everything the calculator doesn't need but we preserve.
    let mut extra = v.clone();
    // Remove fields we already extracted to avoid duplication.
    if let Some(obj) = extra.as_object_mut() {
        obj.remove("Name");
        obj.remove("VendorName");
        obj.remove("Version");
        obj.remove("ExcludedRules");
        obj.remove("ScopeDownStatement");
    }
    Ok(Statement::ManagedRuleGroup {
        vendor_name: str_field(v, "VendorName", "ManagedRuleGroupStatement")?.to_owned(),
        name: str_field(v, "Name", "ManagedRuleGroupStatement")?.to_owned(),
        version: v.get("Version").and_then(|s| s.as_str()).map(String::from),
        excluded_rules,
        scope_down_statement: scope_down,
        extra,
    })
}

fn parse_rule_group_ref(v: &Value) -> Result<Statement, ParseError> {
    Ok(Statement::RuleGroupReference {
        arn: str_field(v, "ARN", "RuleGroupReferenceStatement")?.to_owned(),
        excluded_rules: extract_excluded_rules(v),
    })
}

fn parse_rate_based(v: &Value) -> Result<Statement, ParseError> {
    let scope_down = match v.get("ScopeDownStatement") {
        Some(s) => Some(Box::new(parse_statement(s)?)),
        None => None,
    };
    let mut extra = v.clone();
    if let Some(obj) = extra.as_object_mut() {
        obj.remove("Limit");
        obj.remove("AggregateKeyType");
        obj.remove("ScopeDownStatement");
    }
    Ok(Statement::RateBased {
        limit: v.get("Limit").and_then(|l| l.as_i64()).unwrap_or(0),
        aggregate_key_type: v
            .get("AggregateKeyType")
            .and_then(|s| s.as_str())
            .unwrap_or("IP")
            .to_owned(),
        scope_down_statement: scope_down,
        extra,
    })
}

fn parse_and(v: &Value) -> Result<Statement, ParseError> {
    let stmts = parse_nested_statements(v, "AndStatement")?;
    Ok(Statement::And { statements: stmts })
}

fn parse_or(v: &Value) -> Result<Statement, ParseError> {
    let stmts = parse_nested_statements(v, "OrStatement")?;
    Ok(Statement::Or { statements: stmts })
}

fn parse_not(v: &Value) -> Result<Statement, ParseError> {
    let inner = v.get("Statement").ok_or_else(|| ParseError::MissingField {
        field: "Statement".into(),
        context: "NotStatement".into(),
    })?;
    Ok(Statement::Not {
        statement: Box::new(parse_statement(inner)?),
    })
}

// ── Helpers ──

fn parse_nested_statements(v: &Value, context: &str) -> Result<Vec<Statement>, ParseError> {
    let arr = v
        .get("Statements")
        .and_then(|s| s.as_array())
        .ok_or_else(|| ParseError::MissingField {
            field: "Statements".into(),
            context: context.into(),
        })?;
    arr.iter().map(parse_statement).collect()
}

fn extract_field_to_match(v: &Value, context: &str) -> Result<FieldToMatch, ParseError> {
    let ftm = v
        .get("FieldToMatch")
        .ok_or_else(|| ParseError::MissingField {
            field: "FieldToMatch".into(),
            context: context.into(),
        })?;
    Ok(FieldToMatch(ftm.clone()))
}

fn extract_text_transformations(v: &Value) -> Vec<TextTransformation> {
    v.get("TextTransformations")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .map(|t| TextTransformation {
                    priority: t.get("Priority").and_then(|p| p.as_i64()).unwrap_or(0) as i32,
                    r#type: t
                        .get("Type")
                        .and_then(|s| s.as_str())
                        .unwrap_or("NONE")
                        .to_owned(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn extract_excluded_rules(v: &Value) -> Vec<String> {
    v.get("ExcludedRules")
        .and_then(|e| e.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|r| r.get("Name").and_then(|n| n.as_str()).map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

fn str_field<'a>(v: &'a Value, field: &str, context: &str) -> Result<&'a str, ParseError> {
    v.get(field)
        .and_then(|s| s.as_str())
        .ok_or_else(|| ParseError::MissingField {
            field: field.into(),
            context: context.into(),
        })
}

fn json_type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_geo_match_rule() {
        let rules = json!([{
            "Name": "geo-block",
            "Priority": 1,
            "Statement": {
                "GeoMatchStatement": {
                    "CountryCodes": ["US", "NL"]
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {
                "SampledRequestsEnabled": false,
                "CloudWatchMetricsEnabled": false,
                "MetricName": "geo"
            }
        }]);
        let parsed = parse_rules(&rules).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].name, "geo-block");
        assert_eq!(parsed[0].priority, 1);
        match &parsed[0].statement {
            Statement::GeoMatch { country_codes, .. } => {
                assert_eq!(country_codes, &["US", "NL"]);
            }
            other => panic!("expected GeoMatch, got {other:?}"),
        }
    }

    #[test]
    fn parse_size_constraint_rule() {
        let rules = json!([{
            "Name": "size-check",
            "Priority": 5,
            "Statement": {
                "SizeConstraintStatement": {
                    "ComparisonOperator": "LT",
                    "FieldToMatch": { "QueryString": {} },
                    "Size": 50,
                    "TextTransformations": [
                        { "Priority": 2, "Type": "CMD_LINE" }
                    ]
                }
            },
            "Action": { "Count": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        assert_eq!(parsed.len(), 1);
        match &parsed[0].statement {
            Statement::SizeConstraint {
                comparison_operator,
                size,
                text_transformations,
                ..
            } => {
                assert_eq!(comparison_operator, "LT");
                assert_eq!(*size, 50);
                assert_eq!(text_transformations.len(), 1);
                assert_eq!(text_transformations[0].r#type, "CMD_LINE");
            }
            other => panic!("expected SizeConstraint, got {other:?}"),
        }
    }

    #[test]
    fn parse_and_compound() {
        let rules = json!([{
            "Name": "compound",
            "Priority": 1,
            "Statement": {
                "AndStatement": {
                    "Statements": [
                        { "GeoMatchStatement": { "CountryCodes": ["US"] } },
                        { "IPSetReferenceStatement": { "ARN": "arn:aws:wafv2:us-east-1:123456789012:regional/ipset/my-ipset/abc123" } }
                    ]
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::And { statements } => {
                assert_eq!(statements.len(), 2);
                assert!(matches!(&statements[0], Statement::GeoMatch { .. }));
                assert!(matches!(&statements[1], Statement::IpSetReference { .. }));
            }
            other => panic!("expected And, got {other:?}"),
        }
    }

    #[test]
    fn parse_not_statement() {
        let rules = json!([{
            "Name": "not-rule",
            "Priority": 1,
            "Statement": {
                "NotStatement": {
                    "Statement": {
                        "GeoMatchStatement": { "CountryCodes": ["CN"] }
                    }
                }
            },
            "Action": { "Allow": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::Not { statement } => {
                assert!(matches!(statement.as_ref(), Statement::GeoMatch { .. }));
            }
            other => panic!("expected Not, got {other:?}"),
        }
    }

    #[test]
    fn parse_rate_based_with_scope_down() {
        let rules = json!([{
            "Name": "rate-limit",
            "Priority": 1,
            "Statement": {
                "RateBasedStatement": {
                    "Limit": 2000,
                    "AggregateKeyType": "IP",
                    "ScopeDownStatement": {
                        "GeoMatchStatement": { "CountryCodes": ["US"] }
                    }
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::RateBased {
                limit,
                aggregate_key_type,
                scope_down_statement,
                ..
            } => {
                assert_eq!(*limit, 2000);
                assert_eq!(aggregate_key_type, "IP");
                assert!(scope_down_statement.is_some());
                assert!(matches!(
                    scope_down_statement.as_deref().unwrap(),
                    Statement::GeoMatch { .. }
                ));
            }
            other => panic!("expected RateBased, got {other:?}"),
        }
    }

    #[test]
    fn parse_managed_rule_group() {
        let rules = json!([{
            "Name": "aws-managed",
            "Priority": 1,
            "Statement": {
                "ManagedRuleGroupStatement": {
                    "VendorName": "AWS",
                    "Name": "AWSManagedRulesCommonRuleSet",
                    "ExcludedRules": [{ "Name": "SizeRestrictions_BODY" }]
                }
            },
            "OverrideAction": { "None": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::ManagedRuleGroup {
                vendor_name,
                name,
                excluded_rules,
                ..
            } => {
                assert_eq!(vendor_name, "AWS");
                assert_eq!(name, "AWSManagedRulesCommonRuleSet");
                assert_eq!(excluded_rules, &["SizeRestrictions_BODY"]);
            }
            other => panic!("expected ManagedRuleGroup, got {other:?}"),
        }
    }

    #[test]
    fn parse_byte_match() {
        let rules = json!([{
            "Name": "byte-match",
            "Priority": 1,
            "Statement": {
                "ByteMatchStatement": {
                    "FieldToMatch": { "UriPath": {} },
                    "PositionalConstraint": "STARTS_WITH",
                    "SearchString": "/admin",
                    "TextTransformations": [
                        { "Priority": 0, "Type": "NONE" }
                    ]
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::ByteMatch {
                positional_constraint,
                search_string,
                text_transformations,
                ..
            } => {
                assert_eq!(positional_constraint, "STARTS_WITH");
                assert_eq!(search_string, "/admin");
                assert_eq!(text_transformations.len(), 1);
                assert_eq!(text_transformations[0].r#type, "NONE");
            }
            other => panic!("expected ByteMatch, got {other:?}"),
        }
    }

    #[test]
    fn parse_sqli_match() {
        let rules = json!([{
            "Name": "sqli",
            "Priority": 1,
            "Statement": {
                "SqliMatchStatement": {
                    "FieldToMatch": { "Body": {} },
                    "SensitivityLevel": "HIGH",
                    "TextTransformations": [
                        { "Priority": 0, "Type": "URL_DECODE" }
                    ]
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::SqliMatch {
                sensitivity_level,
                text_transformations,
                ..
            } => {
                assert_eq!(sensitivity_level.as_deref(), Some("HIGH"));
                assert_eq!(text_transformations[0].r#type, "URL_DECODE");
            }
            other => panic!("expected SqliMatch, got {other:?}"),
        }
    }

    #[test]
    fn parse_label_match() {
        let rules = json!([{
            "Name": "label",
            "Priority": 1,
            "Statement": {
                "LabelMatchStatement": {
                    "Scope": "LABEL",
                    "Key": "awswaf:managed:aws:bot-control:bot"
                }
            },
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let parsed = parse_rules(&rules).unwrap();
        match &parsed[0].statement {
            Statement::LabelMatch { scope, key } => {
                assert_eq!(scope, "LABEL");
                assert_eq!(key, "awswaf:managed:aws:bot-control:bot");
            }
            other => panic!("expected LabelMatch, got {other:?}"),
        }
    }

    #[test]
    fn empty_statement_errors() {
        let rules = json!([{
            "Name": "bad",
            "Priority": 1,
            "Statement": {},
            "Action": { "Block": {} },
            "VisibilityConfig": {}
        }]);
        let err = parse_rules(&rules).unwrap_err();
        assert!(matches!(err, ParseError::EmptyStatement));
    }
}
