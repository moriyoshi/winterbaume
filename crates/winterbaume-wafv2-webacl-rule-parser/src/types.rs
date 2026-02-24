/// A single WAFv2 rule with its statement tree.
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub priority: i32,
    pub statement: Statement,
    /// Pass-through: action JSON is not interpreted by the parser.
    pub action: Option<serde_json::Value>,
    /// Pass-through: override action JSON.
    pub override_action: Option<serde_json::Value>,
    /// Pass-through: visibility config JSON.
    pub visibility_config: serde_json::Value,
    pub rule_labels: Vec<String>,
}

/// Recursive statement AST — a proper enum, unlike the wire format's struct-of-optionals.
#[derive(Debug, Clone)]
pub enum Statement {
    ByteMatch {
        field_to_match: FieldToMatch,
        positional_constraint: String,
        search_string: String,
        text_transformations: Vec<TextTransformation>,
    },
    SqliMatch {
        field_to_match: FieldToMatch,
        sensitivity_level: Option<String>,
        text_transformations: Vec<TextTransformation>,
    },
    XssMatch {
        field_to_match: FieldToMatch,
        text_transformations: Vec<TextTransformation>,
    },
    SizeConstraint {
        field_to_match: FieldToMatch,
        comparison_operator: String,
        size: i64,
        text_transformations: Vec<TextTransformation>,
    },
    RegexMatch {
        field_to_match: FieldToMatch,
        regex_string: String,
        text_transformations: Vec<TextTransformation>,
    },
    RegexPatternSetReference {
        arn: String,
        field_to_match: FieldToMatch,
        text_transformations: Vec<TextTransformation>,
    },
    IpSetReference {
        arn: String,
        forwarded_ip_config: Option<serde_json::Value>,
    },
    GeoMatch {
        country_codes: Vec<String>,
        forwarded_ip_config: Option<serde_json::Value>,
    },
    AsnMatch {
        asn_list: Vec<i64>,
        forwarded_ip_config: Option<serde_json::Value>,
    },
    LabelMatch {
        scope: String,
        key: String,
    },
    ManagedRuleGroup {
        vendor_name: String,
        name: String,
        version: Option<String>,
        excluded_rules: Vec<String>,
        scope_down_statement: Option<Box<Statement>>,
        /// Pass-through: rule action overrides, managed rule group configs.
        extra: serde_json::Value,
    },
    RuleGroupReference {
        arn: String,
        excluded_rules: Vec<String>,
    },
    RateBased {
        limit: i64,
        aggregate_key_type: String,
        scope_down_statement: Option<Box<Statement>>,
        /// Pass-through: custom keys, forwarded IP config, evaluation window.
        extra: serde_json::Value,
    },
    And {
        statements: Vec<Statement>,
    },
    Or {
        statements: Vec<Statement>,
    },
    Not {
        statement: Box<Statement>,
    },
}

/// Which part of the request to inspect — kept as pass-through JSON since
/// the WCU calculator does not need to inspect its internals.
#[derive(Debug, Clone)]
pub struct FieldToMatch(pub serde_json::Value);

#[derive(Debug, Clone)]
pub struct TextTransformation {
    pub priority: i32,
    pub r#type: String,
}
