#[derive(Debug, Clone)]
pub struct WebAcl {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub default_action_json: serde_json::Value,
    pub visibility_config_json: serde_json::Value,
    pub rules_json: serde_json::Value,
    pub association_config_json: Option<serde_json::Value>,
    pub custom_response_bodies_json: Option<serde_json::Value>,
    pub captcha_config_json: Option<serde_json::Value>,
    pub challenge_config_json: Option<serde_json::Value>,
    pub token_domains: Option<Vec<String>>,
    pub label_namespace: String,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct IpSet {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub ip_address_version: String,
    pub addresses: Vec<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct RegexPatternSet {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub regular_expressions: Vec<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct RuleGroupData {
    pub name: String,
    pub id: String,
    pub arn: String,
    pub scope: String,
    pub description: String,
    pub lock_token: String,
    pub capacity: i64,
    pub rules_json: serde_json::Value,
    pub visibility_config_json: serde_json::Value,
    pub custom_response_bodies_json: Option<serde_json::Value>,
    pub label_namespace: String,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct LoggingConfigData {
    pub resource_arn: String,
    pub log_destination_configs: Vec<String>,
    pub logging_filter_json: Option<serde_json::Value>,
    pub redacted_fields_json: Option<serde_json::Value>,
    pub log_scope: Option<String>,
    pub log_type: Option<String>,
}

/// Mapping from resource ARN to web ACL ARN
#[derive(Debug, Clone)]
pub struct WebAclAssociation {
    pub resource_arn: String,
    pub web_acl_arn: String,
}

#[derive(Debug, Clone)]
pub struct ApiKeyData {
    pub api_key: String,
    pub scope: String,
    pub token_domains: Vec<String>,
    pub creation_timestamp: f64,
}
