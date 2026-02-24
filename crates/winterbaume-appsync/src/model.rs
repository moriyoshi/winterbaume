//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appsync

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateApiResponse {
    #[serde(rename = "apiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_association: Option<ApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiAssociation {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "associationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_status: Option<String>,
    #[serde(rename = "deploymentDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_detail: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMergedGraphqlApiRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
    #[serde(rename = "sourceApiAssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_config: Option<SourceApiAssociationConfig>,
    #[serde(rename = "sourceApiIdentifier")]
    #[serde(default)]
    pub source_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceApiAssociationConfig {
    #[serde(rename = "mergeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merge_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateMergedGraphqlApiResponse {
    #[serde(rename = "sourceApiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association: Option<SourceApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceApiAssociation {
    #[serde(rename = "associationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastSuccessfulMergeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_merge_date: Option<f64>,
    #[serde(rename = "mergedApiArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_arn: Option<String>,
    #[serde(rename = "mergedApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_id: Option<String>,
    #[serde(rename = "sourceApiArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_arn: Option<String>,
    #[serde(rename = "sourceApiAssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_config: Option<SourceApiAssociationConfig>,
    #[serde(rename = "sourceApiAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_status: Option<String>,
    #[serde(rename = "sourceApiAssociationStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_status_detail: Option<String>,
    #[serde(rename = "sourceApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSourceGraphqlApiRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
    #[serde(rename = "sourceApiAssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_config: Option<SourceApiAssociationConfig>,
    #[serde(rename = "sourceApiIdentifier")]
    #[serde(default)]
    pub source_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateSourceGraphqlApiResponse {
    #[serde(rename = "sourceApiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association: Option<SourceApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiCacheRequest {
    #[serde(rename = "apiCachingBehavior")]
    #[serde(default)]
    pub api_caching_behavior: String,
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "atRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "healthMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_metrics_config: Option<String>,
    #[serde(rename = "transitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiCacheResponse {
    #[serde(rename = "apiCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_cache: Option<ApiCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiCache {
    #[serde(rename = "apiCachingBehavior")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_caching_behavior: Option<String>,
    #[serde(rename = "atRestEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_rest_encryption_enabled: Option<bool>,
    #[serde(rename = "healthMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_metrics_config: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "transitEncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_enabled: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiKeyResponse {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKey {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletes: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiRequest {
    #[serde(rename = "eventConfig")]
    #[serde(default)]
    pub event_config: EventConfig,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventConfig {
    #[serde(rename = "authProviders")]
    #[serde(default)]
    pub auth_providers: Vec<AuthProvider>,
    #[serde(rename = "connectionAuthModes")]
    #[serde(default)]
    pub connection_auth_modes: Vec<AuthMode>,
    #[serde(rename = "defaultPublishAuthModes")]
    #[serde(default)]
    pub default_publish_auth_modes: Vec<AuthMode>,
    #[serde(rename = "defaultSubscribeAuthModes")]
    #[serde(default)]
    pub default_subscribe_auth_modes: Vec<AuthMode>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<EventLogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthProvider {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "cognitoConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_config: Option<CognitoConfig>,
    #[serde(rename = "lambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "openIDConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_config: Option<OpenIDConnectConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoConfig {
    #[serde(rename = "appIdClientRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "userPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaAuthorizerConfig {
    #[serde(rename = "authorizerResultTtlInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    #[serde(rename = "authorizerUri")]
    #[serde(default)]
    pub authorizer_uri: String,
    #[serde(rename = "identityValidationExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_validation_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenIDConnectConfig {
    #[serde(rename = "authTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_t_t_l: Option<i64>,
    #[serde(rename = "clientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "iatTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat_t_t_l: Option<i64>,
    #[serde(default)]
    pub issuer: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthMode {
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventLogConfig {
    #[serde(rename = "cloudWatchLogsRoleArn")]
    #[serde(default)]
    pub cloud_watch_logs_role_arn: String,
    #[serde(rename = "logLevel")]
    #[serde(default)]
    pub log_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Api>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Api {
    #[serde(rename = "apiArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_arn: Option<String>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "eventConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_config: Option<EventConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "wafWebAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_web_acl_arn: Option<String>,
    #[serde(rename = "xrayEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelNamespaceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "codeHandlers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_handlers: Option<String>,
    #[serde(rename = "handlerConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_configs: Option<HandlerConfigs>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "publishAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_auth_modes: Option<Vec<AuthMode>>,
    #[serde(rename = "subscribeAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_auth_modes: Option<Vec<AuthMode>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HandlerConfigs {
    #[serde(rename = "onPublish")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_publish: Option<HandlerConfig>,
    #[serde(rename = "onSubscribe")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_subscribe: Option<HandlerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HandlerConfig {
    #[serde(default)]
    pub behavior: String,
    #[serde(default)]
    pub integration: Integration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Integration {
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(rename = "lambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaConfig {
    #[serde(rename = "invokeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoke_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateChannelNamespaceResponse {
    #[serde(rename = "channelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_namespace: Option<ChannelNamespace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChannelNamespace {
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "channelNamespaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_namespace_arn: Option<String>,
    #[serde(rename = "codeHandlers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_handlers: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<f64>,
    #[serde(rename = "handlerConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_configs: Option<HandlerConfigs>,
    #[serde(rename = "lastModified")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "publishAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_auth_modes: Option<Vec<AuthMode>>,
    #[serde(rename = "subscribeAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_auth_modes: Option<Vec<AuthMode>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dynamodbConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    #[serde(rename = "elasticsearchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    #[serde(rename = "eventBridgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_config: Option<EventBridgeDataSourceConfig>,
    #[serde(rename = "httpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    #[serde(rename = "lambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "openSearchServiceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_service_config: Option<OpenSearchServiceDataSourceConfig>,
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DynamodbDataSourceConfig {
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "deltaSyncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_sync_config: Option<DeltaSyncConfig>,
    #[serde(rename = "tableName")]
    #[serde(default)]
    pub table_name: String,
    #[serde(rename = "useCallerCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_caller_credentials: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versioned: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeltaSyncConfig {
    #[serde(rename = "baseTableTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_table_t_t_l: Option<i64>,
    #[serde(rename = "deltaSyncTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_sync_table_name: Option<String>,
    #[serde(rename = "deltaSyncTableTTL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta_sync_table_t_t_l: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ElasticsearchDataSourceConfig {
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(default)]
    pub endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBridgeDataSourceConfig {
    #[serde(rename = "eventBusArn")]
    #[serde(default)]
    pub event_bus_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpDataSourceConfig {
    #[serde(rename = "authorizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<AuthorizationConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizationConfig {
    #[serde(rename = "authorizationType")]
    #[serde(default)]
    pub authorization_type: String,
    #[serde(rename = "awsIamConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_iam_config: Option<AwsIamConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsIamConfig {
    #[serde(rename = "signingRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_region: Option<String>,
    #[serde(rename = "signingServiceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signing_service_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaDataSourceConfig {
    #[serde(rename = "lambdaFunctionArn")]
    #[serde(default)]
    pub lambda_function_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenSearchServiceDataSourceConfig {
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(default)]
    pub endpoint: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RelationalDatabaseDataSourceConfig {
    #[serde(rename = "rdsHttpEndpointConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_http_endpoint_config: Option<RdsHttpEndpointConfig>,
    #[serde(rename = "relationalDatabaseSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsHttpEndpointConfig {
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_region: Option<String>,
    #[serde(rename = "awsSecretStoreArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_secret_store_arn: Option<String>,
    #[serde(rename = "databaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "dbClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "dataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dynamodbConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    #[serde(rename = "elasticsearchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    #[serde(rename = "eventBridgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_config: Option<EventBridgeDataSourceConfig>,
    #[serde(rename = "httpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    #[serde(rename = "lambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "openSearchServiceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_service_config: Option<OpenSearchServiceDataSourceConfig>,
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameRequest {
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    pub certificate_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainNameResponse {
    #[serde(rename = "domainNameConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_config: Option<DomainNameConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainNameConfig {
    #[serde(rename = "appsyncDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appsync_domain_name: Option<String>,
    #[serde(rename = "certificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "domainNameArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_arn: Option<String>,
    #[serde(rename = "hostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppSyncRuntime {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "runtimeVersion")]
    #[serde(default)]
    pub runtime_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SyncConfig {
    #[serde(rename = "conflictDetection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_detection: Option<String>,
    #[serde(rename = "conflictHandler")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conflict_handler: Option<String>,
    #[serde(rename = "lambdaConflictHandlerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_conflict_handler_config: Option<LambdaConflictHandlerConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LambdaConflictHandlerConfig {
    #[serde(rename = "lambdaConflictHandlerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_conflict_handler_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFunctionResponse {
    #[serde(rename = "functionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FunctionConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_arn: Option<String>,
    #[serde(rename = "functionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_id: Option<String>,
    #[serde(rename = "functionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGraphqlApiRequest {
    #[serde(rename = "additionalAuthenticationProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_authentication_providers: Option<Vec<AdditionalAuthenticationProvider>>,
    #[serde(rename = "apiType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "enhancedMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_metrics_config: Option<EnhancedMetricsConfig>,
    #[serde(rename = "introspectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_config: Option<String>,
    #[serde(rename = "lambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "mergedApiExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_execution_role_arn: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "openIDConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_config: Option<OpenIDConnectConfig>,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
    #[serde(rename = "queryDepthLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_depth_limit: Option<i32>,
    #[serde(rename = "resolverCountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_count_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "userPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "xrayEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalAuthenticationProvider {
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(rename = "lambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "openIDConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_config: Option<OpenIDConnectConfig>,
    #[serde(rename = "userPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<CognitoUserPoolConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoUserPoolConfig {
    #[serde(rename = "appIdClientRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "userPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnhancedMetricsConfig {
    #[serde(rename = "dataSourceLevelMetricsBehavior")]
    #[serde(default)]
    pub data_source_level_metrics_behavior: String,
    #[serde(rename = "operationLevelMetricsConfig")]
    #[serde(default)]
    pub operation_level_metrics_config: String,
    #[serde(rename = "resolverLevelMetricsBehavior")]
    #[serde(default)]
    pub resolver_level_metrics_behavior: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(rename = "cloudWatchLogsRoleArn")]
    #[serde(default)]
    pub cloud_watch_logs_role_arn: String,
    #[serde(rename = "excludeVerboseContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_verbose_content: Option<bool>,
    #[serde(rename = "fieldLogLevel")]
    #[serde(default)]
    pub field_log_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPoolConfig {
    #[serde(rename = "appIdClientRegex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id_client_regex: Option<String>,
    #[serde(rename = "awsRegion")]
    #[serde(default)]
    pub aws_region: String,
    #[serde(rename = "defaultAction")]
    #[serde(default)]
    pub default_action: String,
    #[serde(rename = "userPoolId")]
    #[serde(default)]
    pub user_pool_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateGraphqlApiResponse {
    #[serde(rename = "graphqlApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GraphqlApi {
    #[serde(rename = "additionalAuthenticationProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_authentication_providers: Option<Vec<AdditionalAuthenticationProvider>>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_id: Option<String>,
    #[serde(rename = "apiType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "enhancedMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_metrics_config: Option<EnhancedMetricsConfig>,
    #[serde(rename = "introspectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_config: Option<String>,
    #[serde(rename = "lambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "mergedApiExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_execution_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "openIDConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_config: Option<OpenIDConnectConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
    #[serde(rename = "queryDepthLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_depth_limit: Option<i32>,
    #[serde(rename = "resolverCountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_count_limit: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uris: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "userPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
    #[serde(rename = "wafWebAclArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waf_web_acl_arn: Option<String>,
    #[serde(rename = "xrayEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "cachingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_config: Option<CachingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(rename = "pipelineConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CachingConfig {
    #[serde(rename = "cachingKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_keys: Option<Vec<String>>,
    #[serde(default)]
    pub ttl: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PipelineConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateResolverResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Resolver {
    #[serde(rename = "cachingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_config: Option<CachingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "fieldName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(rename = "pipelineConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "resolverArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_arn: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypeRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    pub format: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTypeResponse {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Type {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiCacheRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiCacheResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiKeyRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiKeyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelNamespaceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteChannelNamespaceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainNameResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "functionId")]
    #[serde(default)]
    pub function_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFunctionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGraphqlApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteGraphqlApiResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResolverResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypeRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTypeResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateApiRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateApiResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMergedGraphqlApiRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "sourceApiIdentifier")]
    #[serde(default)]
    pub source_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateMergedGraphqlApiResponse {
    #[serde(rename = "sourceApiAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSourceGraphqlApiRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateSourceGraphqlApiResponse {
    #[serde(rename = "sourceApiAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateCodeRequest {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    #[serde(default)]
    pub runtime: AppSyncRuntime,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateCodeResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<EvaluateCodeErrorDetail>,
    #[serde(rename = "evaluationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(rename = "outErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_errors: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateCodeErrorDetail {
    #[serde(rename = "codeErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_errors: Option<Vec<CodeError>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeError {
    #[serde(rename = "errorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<CodeErrorLocation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CodeErrorLocation {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateMappingTemplateRequest {
    #[serde(default)]
    pub context: String,
    #[serde(default)]
    pub template: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateMappingTemplateResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorDetail>,
    #[serde(rename = "evaluationResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluation_result: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Vec<String>>,
    #[serde(rename = "outErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_errors: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlushApiCacheRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FlushApiCacheResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiAssociationRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiAssociationResponse {
    #[serde(rename = "apiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_association: Option<ApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiCacheRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiCacheResponse {
    #[serde(rename = "apiCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_cache: Option<ApiCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApiResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Api>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelNamespaceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetChannelNamespaceResponse {
    #[serde(rename = "channelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_namespace: Option<ChannelNamespace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceIntrospectionRequest {
    #[serde(rename = "includeModelsSDL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_models_s_d_l: Option<bool>,
    #[serde(rename = "introspectionId")]
    #[serde(default)]
    pub introspection_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceIntrospectionResponse {
    #[serde(rename = "introspectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_id: Option<String>,
    #[serde(rename = "introspectionResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_result: Option<DataSourceIntrospectionResult>,
    #[serde(rename = "introspectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_status: Option<String>,
    #[serde(rename = "introspectionStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_status_detail: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceIntrospectionResult {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<DataSourceIntrospectionModel>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceIntrospectionModel {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<DataSourceIntrospectionModelField>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indexes: Option<Vec<DataSourceIntrospectionModelIndex>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "primaryKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<DataSourceIntrospectionModelIndex>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sdl: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceIntrospectionModelField {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<DataSourceIntrospectionModelFieldType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceIntrospectionModelFieldType {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Box<DataSourceIntrospectionModelFieldType>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceIntrospectionModelIndex {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameRequest {
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainNameResponse {
    #[serde(rename = "domainNameConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_config: Option<DomainNameConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "functionId")]
    #[serde(default)]
    pub function_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetFunctionResponse {
    #[serde(rename = "functionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGraphqlApiEnvironmentVariablesRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGraphqlApiEnvironmentVariablesResponse {
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGraphqlApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetGraphqlApiResponse {
    #[serde(rename = "graphqlApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntrospectionSchemaRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "includeDirectives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_directives: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIntrospectionSchemaResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResolverResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaCreationStatusRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSchemaCreationStatusResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSourceApiAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSourceApiAssociationResponse {
    #[serde(rename = "sourceApiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association: Option<SourceApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypeRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetTypeResponse {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApiKeysRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApiKeysResponse {
    #[serde(rename = "apiKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_keys: Option<Vec<ApiKey>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApisRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApisResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apis: Option<Vec<Api>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelNamespacesRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListChannelNamespacesResponse {
    #[serde(rename = "channelNamespaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_namespaces: Option<Vec<ChannelNamespace>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesResponse {
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainNamesRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainNamesResponse {
    #[serde(rename = "domainNameConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_configs: Option<Vec<DomainNameConfig>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListFunctionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<FunctionConfiguration>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGraphqlApisRequest {
    #[serde(rename = "apiType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_type: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListGraphqlApisResponse {
    #[serde(rename = "graphqlApis")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_apis: Option<Vec<GraphqlApi>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolversByFunctionRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "functionId")]
    #[serde(default)]
    pub function_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolversByFunctionResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<Vec<Resolver>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolversRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListResolversResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolvers: Option<Vec<Resolver>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceApiAssociationsRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSourceApiAssociationsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "sourceApiAssociationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_summaries: Option<Vec<SourceApiAssociationSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceApiAssociationSummary {
    #[serde(rename = "associationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_arn: Option<String>,
    #[serde(rename = "associationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mergedApiArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_arn: Option<String>,
    #[serde(rename = "mergedApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_id: Option<String>,
    #[serde(rename = "sourceApiArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_arn: Option<String>,
    #[serde(rename = "sourceApiId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesByAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesByAssociationResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Type>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTypesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<Type>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutGraphqlApiEnvironmentVariablesRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    pub environment_variables: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutGraphqlApiEnvironmentVariablesResponse {
    #[serde(rename = "environmentVariables")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataSourceIntrospectionRequest {
    #[serde(rename = "rdsDataApiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rds_data_api_config: Option<RdsDataApiConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RdsDataApiConfig {
    #[serde(rename = "databaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "secretArn")]
    #[serde(default)]
    pub secret_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDataSourceIntrospectionResponse {
    #[serde(rename = "introspectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_id: Option<String>,
    #[serde(rename = "introspectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_status: Option<String>,
    #[serde(rename = "introspectionStatusDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_status_detail: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaCreationRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    pub definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaCreationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaMergeRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartSchemaMergeResponse {
    #[serde(rename = "sourceApiAssociationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "resourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "tagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiCacheRequest {
    #[serde(rename = "apiCachingBehavior")]
    #[serde(default)]
    pub api_caching_behavior: String,
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "healthMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_metrics_config: Option<String>,
    #[serde(default)]
    pub ttl: i64,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiCacheResponse {
    #[serde(rename = "apiCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_cache: Option<ApiCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiKeyRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<i64>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiKeyResponse {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<ApiKey>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "eventConfig")]
    #[serde(default)]
    pub event_config: EventConfig,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api: Option<Api>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelNamespaceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "codeHandlers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_handlers: Option<String>,
    #[serde(rename = "handlerConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler_configs: Option<HandlerConfigs>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "publishAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_auth_modes: Option<Vec<AuthMode>>,
    #[serde(rename = "subscribeAuthModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_auth_modes: Option<Vec<AuthMode>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateChannelNamespaceResponse {
    #[serde(rename = "channelNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_namespace: Option<ChannelNamespace>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "dynamodbConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamodb_config: Option<DynamodbDataSourceConfig>,
    #[serde(rename = "elasticsearchConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elasticsearch_config: Option<ElasticsearchDataSourceConfig>,
    #[serde(rename = "eventBridgeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_config: Option<EventBridgeDataSourceConfig>,
    #[serde(rename = "httpConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_config: Option<HttpDataSourceConfig>,
    #[serde(rename = "lambdaConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_config: Option<LambdaDataSourceConfig>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "openSearchServiceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_service_config: Option<OpenSearchServiceDataSourceConfig>,
    #[serde(rename = "relationalDatabaseConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relational_database_config: Option<RelationalDatabaseDataSourceConfig>,
    #[serde(rename = "serviceRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role_arn: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceResponse {
    #[serde(rename = "dataSource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source: Option<DataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "domainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainNameResponse {
    #[serde(rename = "domainNameConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name_config: Option<DomainNameConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "functionId")]
    #[serde(default)]
    pub function_id: String,
    #[serde(rename = "functionVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_version: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFunctionResponse {
    #[serde(rename = "functionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_configuration: Option<FunctionConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGraphqlApiRequest {
    #[serde(rename = "additionalAuthenticationProviders")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_authentication_providers: Option<Vec<AdditionalAuthenticationProvider>>,
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "authenticationType")]
    #[serde(default)]
    pub authentication_type: String,
    #[serde(rename = "enhancedMetricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_metrics_config: Option<EnhancedMetricsConfig>,
    #[serde(rename = "introspectionConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub introspection_config: Option<String>,
    #[serde(rename = "lambdaAuthorizerConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lambda_authorizer_config: Option<LambdaAuthorizerConfig>,
    #[serde(rename = "logConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "mergedApiExecutionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merged_api_execution_role_arn: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "openIDConnectConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_i_d_connect_config: Option<OpenIDConnectConfig>,
    #[serde(rename = "ownerContact")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_contact: Option<String>,
    #[serde(rename = "queryDepthLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_depth_limit: Option<i32>,
    #[serde(rename = "resolverCountLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver_count_limit: Option<i32>,
    #[serde(rename = "userPoolConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_config: Option<UserPoolConfig>,
    #[serde(rename = "xrayEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xray_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateGraphqlApiResponse {
    #[serde(rename = "graphqlApi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphql_api: Option<GraphqlApi>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(rename = "cachingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caching_config: Option<CachingConfig>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "dataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "fieldName")]
    #[serde(default)]
    pub field_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "maxBatchSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<i32>,
    #[serde(rename = "metricsConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics_config: Option<String>,
    #[serde(rename = "pipelineConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_config: Option<PipelineConfig>,
    #[serde(rename = "requestMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_mapping_template: Option<String>,
    #[serde(rename = "responseMappingTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_mapping_template: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<AppSyncRuntime>,
    #[serde(rename = "syncConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_config: Option<SyncConfig>,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateResolverResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolver: Option<Resolver>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSourceApiAssociationRequest {
    #[serde(rename = "associationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "mergedApiIdentifier")]
    #[serde(default)]
    pub merged_api_identifier: String,
    #[serde(rename = "sourceApiAssociationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association_config: Option<SourceApiAssociationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSourceApiAssociationResponse {
    #[serde(rename = "sourceApiAssociation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_api_association: Option<SourceApiAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTypeRequest {
    #[serde(rename = "apiId")]
    #[serde(default)]
    pub api_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub definition: Option<String>,
    #[serde(default)]
    pub format: String,
    #[serde(rename = "typeName")]
    #[serde(default)]
    pub type_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateTypeResponse {
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}
