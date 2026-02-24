use std::collections::HashMap;

use serde_json::Value;
use winterbaume_wafv2_webacl_rule_parser::{Rule, Statement, TextTransformation};

use crate::error::WcuError;
use crate::managed;

/// Calculate total WCU for a slice of parsed WAFv2 rules.
///
/// `resolved_rule_groups` maps customer-managed rule group ARNs to their
/// pre-computed WCU capacity.  The handler is responsible for resolving these
/// from state before calling this function.
pub fn calculate_capacity(
    rules: &[Rule],
    resolved_rule_groups: &HashMap<String, u64>,
) -> Result<u64, WcuError> {
    let mut total: u64 = 0;
    for rule in rules {
        total += statement_cost(&rule.statement, resolved_rule_groups)?;
    }
    Ok(total)
}

/// Recursively compute the WCU cost of a single statement.
pub fn statement_cost(
    stmt: &Statement,
    resolved_rule_groups: &HashMap<String, u64>,
) -> Result<u64, WcuError> {
    match stmt {
        // ── Leaf statements ──
        Statement::ByteMatch {
            field_to_match,
            positional_constraint,
            text_transformations,
            ..
        } => {
            let base = byte_match_base_cost(positional_constraint);
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(base * mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::SqliMatch {
            field_to_match,
            text_transformations,
            ..
        } => {
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(20 * mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::XssMatch {
            field_to_match,
            text_transformations,
            ..
        } => {
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(40 * mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::SizeConstraint {
            field_to_match,
            text_transformations,
            ..
        } => {
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::RegexMatch {
            field_to_match,
            text_transformations,
            ..
        } => {
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(3 * mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::RegexPatternSetReference {
            field_to_match,
            text_transformations,
            ..
        } => {
            let mult = field_to_match_base_multiplier(&field_to_match.0);
            Ok(base_with_transformations(25 * mult, text_transformations)
                + field_to_match_additive_surcharge(&field_to_match.0))
        }

        Statement::IpSetReference { .. } => Ok(1),

        Statement::GeoMatch { .. } => Ok(1),

        Statement::AsnMatch { .. } => Ok(1),

        Statement::LabelMatch { .. } => Ok(1),

        // ── Compound statements ──
        Statement::And { statements } | Statement::Or { statements } => {
            let mut sum = 0u64;
            for s in statements {
                sum += statement_cost(s, resolved_rule_groups)?;
            }
            Ok(sum)
        }

        Statement::Not { statement } => statement_cost(statement, resolved_rule_groups),

        Statement::RateBased {
            scope_down_statement,
            extra,
            ..
        } => {
            let base = 2u64;
            let scope_down = match scope_down_statement {
                Some(s) => statement_cost(s, resolved_rule_groups)?,
                None => 0,
            };
            let custom_key_cost = rate_based_custom_key_surcharge(extra);
            Ok(base + scope_down + custom_key_cost)
        }

        // ── Group references ──
        Statement::ManagedRuleGroup {
            vendor_name,
            name,
            scope_down_statement,
            ..
        } => {
            let group_wcu =
                managed::managed_rule_group_capacity(vendor_name, name).ok_or_else(|| {
                    WcuError::UnknownManagedRuleGroup {
                        vendor: vendor_name.clone(),
                        name: name.clone(),
                    }
                })?;
            let scope_down = match scope_down_statement {
                Some(s) => statement_cost(s, resolved_rule_groups)?,
                None => 0,
            };
            Ok(group_wcu + scope_down)
        }

        Statement::RuleGroupReference { arn, .. } => resolved_rule_groups
            .get(arn)
            .copied()
            .ok_or_else(|| WcuError::UnresolvedRuleGroup(arn.clone())),
    }
}

/// ByteMatch base WCU depends on the positional constraint.
///
/// - EXACTLY / STARTS_WITH / ENDS_WITH → 2 WCU
/// - CONTAINS / CONTAINS_WORD → 10 WCU
///
/// Each text transformation then adds 10 WCU (handled by `base_with_transformations`).
fn byte_match_base_cost(positional_constraint: &str) -> u64 {
    match positional_constraint {
        "CONTAINS" | "CONTAINS_WORD" => 10,
        // EXACTLY, STARTS_WITH, ENDS_WITH, and any unknown constraint
        _ => 2,
    }
}

/// For statements with a flat base cost, each text transformation adds 10 WCU.
fn base_with_transformations(base: u64, transforms: &[TextTransformation]) -> u64 {
    let transform_cost = transforms.len() as u64 * 10;
    base + transform_cost
}

/// Base-cost multiplier for certain field types.
///
/// AWS WAF doubles the base cost of a statement when the request component is
/// `JsonBody` (JSON body parsed for inspection).  All other components use a
/// multiplier of 1.
pub fn field_to_match_base_multiplier(field: &Value) -> u64 {
    let obj = match field.as_object() {
        Some(o) => o,
        None => return 1,
    };
    if obj.contains_key("JsonBody") { 2 } else { 1 }
}

/// Additive WCU surcharge for certain field types.
///
/// - `AllQueryParameters` → +10 WCU
/// - All other components → 0 WCU
///
/// Note: `JsonBody` is handled via [`field_to_match_base_multiplier`], not
/// here.  Plain `Body`, `Headers`, and `Cookies` carry no additive surcharge.
pub fn field_to_match_additive_surcharge(field: &Value) -> u64 {
    let obj = match field.as_object() {
        Some(o) => o,
        None => return 0,
    };
    if obj.contains_key("AllQueryParameters") {
        10
    } else {
        0
    }
}

/// Surcharge for RateBased custom keys.
///
/// Each custom key costs +30 WCU.
pub fn rate_based_custom_key_surcharge(extra: &Value) -> u64 {
    let keys = match extra.get("CustomKeys").and_then(|v| v.as_array()) {
        Some(arr) => arr,
        None => return 0,
    };
    keys.len() as u64 * 30
}

#[cfg(test)]
mod tests {
    use winterbaume_wafv2_webacl_rule_parser::{FieldToMatch, TextTransformation};

    use super::*;

    fn dummy_field_to_match() -> FieldToMatch {
        FieldToMatch(serde_json::json!({ "UriPath": {} }))
    }

    #[test]
    fn leaf_costs() {
        let groups = HashMap::new();

        let geo = Statement::GeoMatch {
            country_codes: vec!["US".into()],
            forwarded_ip_config: None,
        };
        assert_eq!(statement_cost(&geo, &groups).unwrap(), 1);

        let ip = Statement::IpSetReference {
            arn: "arn:...".into(),
            forwarded_ip_config: None,
        };
        assert_eq!(statement_cost(&ip, &groups).unwrap(), 1);

        let label = Statement::LabelMatch {
            scope: "LABEL".into(),
            key: "foo".into(),
        };
        assert_eq!(statement_cost(&label, &groups).unwrap(), 1);

        // SizeConstraint with 1 transform on UriPath: base 1 + 1*10 = 11
        let size = Statement::SizeConstraint {
            field_to_match: dummy_field_to_match(),
            comparison_operator: "LT".into(),
            size: 100,
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "NONE".into(),
            }],
        };
        assert_eq!(statement_cost(&size, &groups).unwrap(), 11);

        // SQLi with 1 transform on UriPath: base 20 + 1*10 = 30
        let sqli = Statement::SqliMatch {
            field_to_match: dummy_field_to_match(),
            sensitivity_level: None,
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "URL_DECODE".into(),
            }],
        };
        assert_eq!(statement_cost(&sqli, &groups).unwrap(), 30);

        // XSS with 1 transform on UriPath: base 40 + 1*10 = 50
        let xss = Statement::XssMatch {
            field_to_match: dummy_field_to_match(),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "NONE".into(),
            }],
        };
        assert_eq!(statement_cost(&xss, &groups).unwrap(), 50);
    }

    #[test]
    fn byte_match_exactly_no_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "EXACTLY".into(),
            search_string: "/admin".into(),
            text_transformations: vec![],
        };
        // base 2
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 2);
    }

    #[test]
    fn byte_match_starts_with_no_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "STARTS_WITH".into(),
            search_string: "/admin".into(),
            text_transformations: vec![],
        };
        // base 2
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 2);
    }

    #[test]
    fn byte_match_ends_with_no_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "ENDS_WITH".into(),
            search_string: ".php".into(),
            text_transformations: vec![],
        };
        // base 2
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 2);
    }

    #[test]
    fn byte_match_contains_no_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "CONTAINS".into(),
            search_string: "attack".into(),
            text_transformations: vec![],
        };
        // base 10
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 10);
    }

    #[test]
    fn byte_match_contains_word_no_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "CONTAINS_WORD".into(),
            search_string: "BadBot".into(),
            text_transformations: vec![],
        };
        // base 10
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 10);
    }

    #[test]
    fn byte_match_starts_with_one_transform() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "STARTS_WITH".into(),
            search_string: "/admin".into(),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "LOWERCASE".into(),
            }],
        };
        // base 2 + 1 transform × 10 = 12
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 12);
    }

    #[test]
    fn byte_match_contains_one_transform() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "CONTAINS".into(),
            search_string: "attack".into(),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "CMD_LINE".into(),
            }],
        };
        // base 10 + 1 transform × 10 = 20
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 20);
    }

    #[test]
    fn byte_match_contains_multiple_transforms() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: dummy_field_to_match(),
            positional_constraint: "CONTAINS".into(),
            search_string: "attack".into(),
            text_transformations: vec![
                TextTransformation {
                    priority: 0,
                    r#type: "CMD_LINE".into(),
                },
                TextTransformation {
                    priority: 1,
                    r#type: "URL_DECODE_UNI".into(),
                },
            ],
        };
        // base 10 + 2 transforms × 10 = 30
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 30);
    }

    #[test]
    fn byte_match_exactly_with_json_body() {
        let groups = HashMap::new();
        let bm = Statement::ByteMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            positional_constraint: "EXACTLY".into(),
            search_string: "foo".into(),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "NONE".into(),
            }],
        };
        // base 2, JsonBody doubles base → 4, 1 transform × 10 = 14
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 14);
    }

    #[test]
    fn compound_sums_children() {
        let groups = HashMap::new();
        let and = Statement::And {
            statements: vec![
                Statement::GeoMatch {
                    country_codes: vec!["US".into()],
                    forwarded_ip_config: None,
                },
                Statement::IpSetReference {
                    arn: "arn:...".into(),
                    forwarded_ip_config: None,
                },
            ],
        };
        assert_eq!(statement_cost(&and, &groups).unwrap(), 2);
    }

    #[test]
    fn rate_based_adds_base_cost() {
        let groups = HashMap::new();
        let rate = Statement::RateBased {
            limit: 2000,
            aggregate_key_type: "IP".into(),
            scope_down_statement: Some(Box::new(Statement::GeoMatch {
                country_codes: vec!["US".into()],
                forwarded_ip_config: None,
            })),
            extra: serde_json::Value::Object(Default::default()),
        };
        // 2 (base) + 1 (geo) = 3
        assert_eq!(statement_cost(&rate, &groups).unwrap(), 3);
    }

    #[test]
    fn rate_based_custom_keys() {
        let groups = HashMap::new();
        let rate = Statement::RateBased {
            limit: 2000,
            aggregate_key_type: "CUSTOM_KEYS".into(),
            scope_down_statement: None,
            extra: serde_json::json!({
                "CustomKeys": [
                    { "Header": { "Name": "x-api-key", "TextTransformations": [] } },
                    { "IP": {} }
                ]
            }),
        };
        // 2 (base) + 2*30 (custom keys) = 62
        assert_eq!(statement_cost(&rate, &groups).unwrap(), 62);
    }

    #[test]
    fn managed_rule_group_cost() {
        let groups = HashMap::new();
        let mrg = Statement::ManagedRuleGroup {
            vendor_name: "AWS".into(),
            name: "AWSManagedRulesCommonRuleSet".into(),
            version: None,
            excluded_rules: vec![],
            scope_down_statement: None,
            extra: serde_json::Value::Object(Default::default()),
        };
        assert_eq!(statement_cost(&mrg, &groups).unwrap(), 700);
    }

    #[test]
    fn rule_group_reference_resolved() {
        let mut groups = HashMap::new();
        groups.insert(
            "arn:aws:wafv2:us-east-1:123:regional/rulegroup/my-rg/abc".into(),
            150u64,
        );
        let rg = Statement::RuleGroupReference {
            arn: "arn:aws:wafv2:us-east-1:123:regional/rulegroup/my-rg/abc".into(),
            excluded_rules: vec![],
        };
        assert_eq!(statement_cost(&rg, &groups).unwrap(), 150);
    }

    #[test]
    fn rule_group_reference_unresolved_errors() {
        let groups = HashMap::new();
        let rg = Statement::RuleGroupReference {
            arn: "arn:missing".into(),
            excluded_rules: vec![],
        };
        assert!(statement_cost(&rg, &groups).is_err());
    }

    #[test]
    fn calculate_capacity_multiple_rules() {
        let groups = HashMap::new();
        let rules = vec![
            Rule {
                name: "r1".into(),
                priority: 1,
                statement: Statement::GeoMatch {
                    country_codes: vec!["US".into()],
                    forwarded_ip_config: None,
                },
                action: None,
                override_action: None,
                visibility_config: serde_json::Value::Object(Default::default()),
                rule_labels: vec![],
            },
            Rule {
                name: "r2".into(),
                priority: 2,
                statement: Statement::SqliMatch {
                    field_to_match: FieldToMatch(serde_json::json!({ "Body": {} })),
                    sensitivity_level: None,
                    text_transformations: vec![TextTransformation {
                        priority: 0,
                        r#type: "NONE".into(),
                    }],
                },
                action: None,
                override_action: None,
                visibility_config: serde_json::Value::Object(Default::default()),
                rule_labels: vec![],
            },
        ];
        // 1 (geo) + 20 (sqli base) + 10 (1 transform) = 31
        // Plain Body carries no WCU surcharge per AWS docs.
        assert_eq!(calculate_capacity(&rules, &groups).unwrap(), 31);
    }

    #[test]
    fn extra_transformations_add_cost() {
        let groups = HashMap::new();
        let sqli = Statement::SqliMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "Body": {} })),
            sensitivity_level: None,
            text_transformations: vec![
                TextTransformation {
                    priority: 0,
                    r#type: "URL_DECODE".into(),
                },
                TextTransformation {
                    priority: 1,
                    r#type: "HTML_ENTITY_DECODE".into(),
                },
                TextTransformation {
                    priority: 2,
                    r#type: "COMPRESS_WHITE_SPACE".into(),
                },
            ],
        };
        // 20 (base) + 3 × 10 (transforms) = 50
        // Plain Body carries no WCU surcharge per AWS docs.
        assert_eq!(statement_cost(&sqli, &groups).unwrap(), 50);
    }

    // ── field_to_match_base_multiplier tests ──

    #[test]
    fn json_body_doubles_multiplier() {
        assert_eq!(
            field_to_match_base_multiplier(&serde_json::json!({ "JsonBody": {} })),
            2
        );
    }

    #[test]
    fn body_multiplier_is_one() {
        assert_eq!(
            field_to_match_base_multiplier(&serde_json::json!({ "Body": {} })),
            1
        );
    }

    #[test]
    fn headers_multiplier_is_one() {
        assert_eq!(
            field_to_match_base_multiplier(&serde_json::json!({ "Headers": {} })),
            1
        );
    }

    #[test]
    fn cookies_multiplier_is_one() {
        assert_eq!(
            field_to_match_base_multiplier(&serde_json::json!({ "Cookies": {} })),
            1
        );
    }

    #[test]
    fn uri_path_multiplier_is_one() {
        assert_eq!(
            field_to_match_base_multiplier(&serde_json::json!({ "UriPath": {} })),
            1
        );
    }

    // ── field_to_match_additive_surcharge tests ──

    #[test]
    fn all_query_params_additive_surcharge() {
        assert_eq!(
            field_to_match_additive_surcharge(&serde_json::json!({ "AllQueryParameters": {} })),
            10
        );
    }

    #[test]
    fn json_body_no_additive_surcharge() {
        assert_eq!(
            field_to_match_additive_surcharge(&serde_json::json!({ "JsonBody": {} })),
            0
        );
    }

    #[test]
    fn uri_path_no_additive_surcharge() {
        assert_eq!(
            field_to_match_additive_surcharge(&serde_json::json!({ "UriPath": {} })),
            0
        );
    }

    #[test]
    fn rate_based_no_custom_keys() {
        assert_eq!(rate_based_custom_key_surcharge(&serde_json::json!({})), 0);
    }

    #[test]
    fn rate_based_with_custom_keys_surcharge() {
        let extra = serde_json::json!({
            "CustomKeys": [
                { "Header": { "Name": "x-api-key" } }
            ]
        });
        assert_eq!(rate_based_custom_key_surcharge(&extra), 30);
    }

    // ── Statement-level tests for new field-to-match semantics ──

    #[test]
    fn sqli_with_json_body_doubles_base() {
        let groups = HashMap::new();
        // SqliMatch base=20, JsonBody doubles → 40, no transforms, no additive surcharge = 40
        let sqli = Statement::SqliMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            sensitivity_level: None,
            text_transformations: vec![],
        };
        assert_eq!(statement_cost(&sqli, &groups).unwrap(), 40);
    }

    #[test]
    fn sqli_with_json_body_and_transforms() {
        let groups = HashMap::new();
        // SqliMatch base=20, JsonBody doubles → 40, 2 transforms × 10 = 60
        let sqli = Statement::SqliMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            sensitivity_level: None,
            text_transformations: vec![
                TextTransformation {
                    priority: 0,
                    r#type: "URL_DECODE".into(),
                },
                TextTransformation {
                    priority: 1,
                    r#type: "HTML_ENTITY_DECODE".into(),
                },
            ],
        };
        assert_eq!(statement_cost(&sqli, &groups).unwrap(), 60);
    }

    #[test]
    fn size_constraint_all_query_params() {
        let groups = HashMap::new();
        // SizeConstraint base=1, AllQueryParameters adds 10, no transforms = 11
        let size = Statement::SizeConstraint {
            field_to_match: FieldToMatch(serde_json::json!({ "AllQueryParameters": {} })),
            comparison_operator: "GT".into(),
            size: 500,
            text_transformations: vec![],
        };
        assert_eq!(statement_cost(&size, &groups).unwrap(), 11);
    }

    #[test]
    fn byte_match_contains_all_query_params_with_transform() {
        let groups = HashMap::new();
        // ByteMatch CONTAINS base=10, AllQueryParameters additive=10, 1 transform × 10 = 30
        let bm = Statement::ByteMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "AllQueryParameters": {} })),
            positional_constraint: "CONTAINS".into(),
            search_string: "attack".into(),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "LOWERCASE".into(),
            }],
        };
        assert_eq!(statement_cost(&bm, &groups).unwrap(), 30);
    }

    #[test]
    fn xss_with_json_body_doubles_base() {
        let groups = HashMap::new();
        // XssMatch base=40, JsonBody doubles → 80, 1 transform × 10 = 90
        let xss = Statement::XssMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            text_transformations: vec![TextTransformation {
                priority: 0,
                r#type: "NONE".into(),
            }],
        };
        assert_eq!(statement_cost(&xss, &groups).unwrap(), 90);
    }

    #[test]
    fn regex_match_with_json_body_doubles_base() {
        let groups = HashMap::new();
        // RegexMatch base=3, JsonBody doubles → 6, no transforms = 6
        let rm = Statement::RegexMatch {
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            regex_string: "foo.*".into(),
            text_transformations: vec![],
        };
        assert_eq!(statement_cost(&rm, &groups).unwrap(), 6);
    }

    #[test]
    fn regex_pattern_set_with_json_body_doubles_base() {
        let groups = HashMap::new();
        // RegexPatternSetReference base=25, JsonBody doubles → 50, no transforms = 50
        let rp = Statement::RegexPatternSetReference {
            arn: "arn:aws:wafv2:us-east-1:123:regional/regexpatternset/my-rp/abc".into(),
            field_to_match: FieldToMatch(serde_json::json!({ "JsonBody": {} })),
            text_transformations: vec![],
        };
        assert_eq!(statement_cost(&rp, &groups).unwrap(), 50);
    }
}
