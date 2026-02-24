use std::collections::HashMap;

/// A GraphQL API resource (v1).
#[derive(Debug, Clone)]
pub struct GraphqlApi {
    pub api_id: String,
    pub name: String,
    pub authentication_type: String,
    pub arn: String,
    pub uris: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    pub xray_enabled: bool,
    pub additional_authentication_provider: Option<serde_json::Value>,
    pub lambda_authorizer_config: Option<serde_json::Value>,
    pub user_pool_config: Option<serde_json::Value>,
    pub enhanced_metrics_config: Option<serde_json::Value>,
}

/// An Event API resource (v2).
#[derive(Debug, Clone)]
pub struct Api {
    pub api_id: String,
    pub name: String,
    pub api_arn: String,
    pub created: f64,
    pub tags: HashMap<String, String>,
    pub owner_contact: Option<String>,
}

/// An API cache resource.
#[derive(Debug, Clone)]
pub struct ApiCacheEntry {
    pub api_id: String,
    pub api_caching_behavior: String,
    pub r#type: String,
    pub ttl: i64,
    pub at_rest_encryption_enabled: bool,
    pub transit_encryption_enabled: bool,
    pub status: String,
    pub health_metrics_config: Option<String>,
}

/// An API key resource.
#[derive(Debug, Clone)]
pub struct ApiKeyEntry {
    pub id: String,
    pub api_id: String,
    pub description: Option<String>,
    pub expires: i64,
    pub deletes: i64,
}

/// A channel namespace resource (v2).
#[derive(Debug, Clone)]
pub struct ChannelNamespaceEntry {
    pub api_id: String,
    pub name: String,
    pub channel_namespace_arn: String,
    pub created: f64,
    pub last_modified: f64,
    pub tags: HashMap<String, String>,
}

/// A GraphQL type resource.
#[derive(Debug, Clone)]
pub struct TypeEntry {
    pub api_id: String,
    pub name: String,
    pub definition: Option<String>,
    pub format: String,
    pub arn: String,
}

/// Schema creation status for a GraphQL API.
#[derive(Debug, Clone)]
pub struct SchemaStatus {
    pub status: String,
    pub details: Option<String>,
}
