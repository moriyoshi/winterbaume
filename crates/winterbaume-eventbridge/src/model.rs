//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-eventbridge

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActivateEventSourceRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelReplayRequest {
    #[serde(rename = "ReplayName")]
    #[serde(default)]
    pub replay_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelReplayResponse {
    #[serde(rename = "ReplayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiDestinationRequest {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    pub connection_arn: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "InvocationEndpoint")]
    #[serde(default)]
    pub invocation_endpoint: String,
    #[serde(rename = "InvocationRateLimitPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApiDestinationResponse {
    #[serde(rename = "ApiDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    #[serde(rename = "ApiDestinationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateArchiveRequest {
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    pub archive_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    pub event_source_arn: String,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "RetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateArchiveResponse {
    #[serde(rename = "ArchiveArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionRequest {
    #[serde(rename = "AuthParameters")]
    #[serde(default)]
    pub auth_parameters: CreateConnectionAuthRequestParameters,
    #[serde(rename = "AuthorizationType")]
    #[serde(default)]
    pub authorization_type: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InvocationConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_connectivity_parameters: Option<ConnectivityResourceParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionAuthRequestParameters {
    #[serde(rename = "ApiKeyAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<CreateConnectionApiKeyAuthRequestParameters>,
    #[serde(rename = "BasicAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<CreateConnectionBasicAuthRequestParameters>,
    #[serde(rename = "ConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_parameters: Option<ConnectivityResourceParameters>,
    #[serde(rename = "InvocationHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    #[serde(rename = "OAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<CreateConnectionOAuthRequestParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionApiKeyAuthRequestParameters {
    #[serde(rename = "ApiKeyName")]
    #[serde(default)]
    pub api_key_name: String,
    #[serde(rename = "ApiKeyValue")]
    #[serde(default)]
    pub api_key_value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionBasicAuthRequestParameters {
    #[serde(rename = "Password")]
    #[serde(default)]
    pub password: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectivityResourceParameters {
    #[serde(rename = "ResourceParameters")]
    #[serde(default)]
    pub resource_parameters: ConnectivityResourceConfigurationArn,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectivityResourceConfigurationArn {
    #[serde(rename = "ResourceConfigurationArn")]
    #[serde(default)]
    pub resource_configuration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionHttpParameters {
    #[serde(rename = "BodyParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_parameters: Option<Vec<ConnectionBodyParameter>>,
    #[serde(rename = "HeaderParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<Vec<ConnectionHeaderParameter>>,
    #[serde(rename = "QueryStringParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<Vec<ConnectionQueryStringParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionBodyParameter {
    #[serde(rename = "IsValueSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionHeaderParameter {
    #[serde(rename = "IsValueSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionQueryStringParameter {
    #[serde(rename = "IsValueSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_value_secret: Option<bool>,
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionOAuthRequestParameters {
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    pub authorization_endpoint: String,
    #[serde(rename = "ClientParameters")]
    #[serde(default)]
    pub client_parameters: CreateConnectionOAuthClientRequestParameters,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    pub http_method: String,
    #[serde(rename = "OAuthHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionOAuthClientRequestParameters {
    #[serde(rename = "ClientID")]
    #[serde(default)]
    pub client_i_d: String,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    pub client_secret: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConnectionResponse {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    pub event_buses: Vec<EndpointEventBus>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    pub routing_config: RoutingConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointEventBus {
    #[serde(rename = "EventBusArn")]
    #[serde(default)]
    pub event_bus_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationConfig {
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RoutingConfig {
    #[serde(rename = "FailoverConfig")]
    #[serde(default)]
    pub failover_config: FailoverConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FailoverConfig {
    #[serde(rename = "Primary")]
    #[serde(default)]
    pub primary: Primary,
    #[serde(rename = "Secondary")]
    #[serde(default)]
    pub secondary: Secondary,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Primary {
    #[serde(rename = "HealthCheck")]
    #[serde(default)]
    pub health_check: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Secondary {
    #[serde(rename = "Route")]
    #[serde(default)]
    pub route: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EndpointEventBus>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<RoutingConfig>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBusRequest {
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_name: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeadLetterConfig {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfig {
    #[serde(rename = "IncludeDetail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_detail: Option<String>,
    #[serde(rename = "Level")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventBusResponse {
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBusArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_arn: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartnerEventSourceRequest {
    #[serde(rename = "Account")]
    #[serde(default)]
    pub account: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePartnerEventSourceResponse {
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeactivateEventSourceRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeauthorizeConnectionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeauthorizeConnectionResponse {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastAuthorizedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiDestinationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApiDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteArchiveRequest {
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    pub archive_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteArchiveResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConnectionResponse {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastAuthorizedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEndpointResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventBusRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePartnerEventSourceRequest {
    #[serde(rename = "Account")]
    #[serde(default)]
    pub account: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteRuleRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApiDestinationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApiDestinationResponse {
    #[serde(rename = "ApiDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    #[serde(rename = "ApiDestinationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "InvocationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    #[serde(rename = "InvocationRateLimitPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i32>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeArchiveRequest {
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    pub archive_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeArchiveResponse {
    #[serde(rename = "ArchiveArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "RetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[serde(rename = "SizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionResponse {
    #[serde(rename = "AuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<ConnectionAuthResponseParameters>,
    #[serde(rename = "AuthorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InvocationConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_connectivity_parameters: Option<DescribeConnectionConnectivityParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LastAuthorizedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_arn: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionAuthResponseParameters {
    #[serde(rename = "ApiKeyAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<ConnectionApiKeyAuthResponseParameters>,
    #[serde(rename = "BasicAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<ConnectionBasicAuthResponseParameters>,
    #[serde(rename = "ConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_parameters: Option<DescribeConnectionConnectivityParameters>,
    #[serde(rename = "InvocationHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    #[serde(rename = "OAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<ConnectionOAuthResponseParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionApiKeyAuthResponseParameters {
    #[serde(rename = "ApiKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionBasicAuthResponseParameters {
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionConnectivityParameters {
    #[serde(rename = "ResourceParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_parameters: Option<DescribeConnectionResourceParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConnectionResourceParameters {
    #[serde(rename = "ResourceAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_association_arn: Option<String>,
    #[serde(rename = "ResourceConfigurationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_configuration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionOAuthResponseParameters {
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "ClientParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_parameters: Option<ConnectionOAuthClientResponseParameters>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "OAuthHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionOAuthClientResponseParameters {
    #[serde(rename = "ClientID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointRequest {
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EndpointEventBus>>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<RoutingConfig>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventBusRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventBusResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventSourceRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeEventSourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePartnerEventSourceRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePartnerEventSourceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplayRequest {
    #[serde(rename = "ReplayName")]
    #[serde(default)]
    pub replay_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReplayResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<ReplayDestination>,
    #[serde(rename = "EventEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<f64>,
    #[serde(rename = "EventLastReplayedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_replayed_time: Option<f64>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "EventStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<f64>,
    #[serde(rename = "ReplayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    #[serde(rename = "ReplayEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_end_time: Option<f64>,
    #[serde(rename = "ReplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_name: Option<String>,
    #[serde(rename = "ReplayStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplayDestination {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "FilterArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeRuleResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisableRuleRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnableRuleRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApiDestinationsRequest {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApiDestinationsResponse {
    #[serde(rename = "ApiDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destinations: Option<Vec<ApiDestination>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiDestination {
    #[serde(rename = "ApiDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    #[serde(rename = "ApiDestinationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "InvocationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    #[serde(rename = "InvocationRateLimitPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i32>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListArchivesRequest {
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListArchivesResponse {
    #[serde(rename = "Archives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archives: Option<Vec<Archive>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Archive {
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_name: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EventCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_count: Option<i64>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "RetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
    #[serde(rename = "SizeBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<i64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectionsRequest {
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<Connection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Connection {
    #[serde(rename = "AuthorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastAuthorizedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsRequest {
    #[serde(rename = "HomeRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_region: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEndpointsResponse {
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Endpoint {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EndpointEventBus>>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<RoutingConfig>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBusesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventBusesResponse {
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EventBus>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventBus {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventSourcesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventSourcesResponse {
    #[serde(rename = "EventSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_sources: Option<Vec<EventSource>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSource {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartnerEventSourceAccountsRequest {
    #[serde(rename = "EventSourceName")]
    #[serde(default)]
    pub event_source_name: String,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartnerEventSourceAccountsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartnerEventSourceAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_source_accounts: Option<Vec<PartnerEventSourceAccount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartnerEventSourceAccount {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartnerEventSourcesRequest {
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    pub name_prefix: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartnerEventSourcesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PartnerEventSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_event_sources: Option<Vec<PartnerEventSource>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartnerEventSource {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplaysRequest {
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListReplaysResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Replays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replays: Option<Vec<Replay>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Replay {
    #[serde(rename = "EventEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_end_time: Option<f64>,
    #[serde(rename = "EventLastReplayedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_last_replayed_time: Option<f64>,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_source_arn: Option<String>,
    #[serde(rename = "EventStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_start_time: Option<f64>,
    #[serde(rename = "ReplayEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_end_time: Option<f64>,
    #[serde(rename = "ReplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_name: Option<String>,
    #[serde(rename = "ReplayStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleNamesByTargetRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRuleNamesByTargetResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RuleNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NamePrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListRulesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<Rule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Rule {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsByRuleRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTargetsByRuleResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Targets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<Target>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Target {
    #[serde(rename = "AppSyncParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_sync_parameters: Option<AppSyncParameters>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "BatchParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_parameters: Option<BatchParameters>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "EcsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_parameters: Option<EcsParameters>,
    #[serde(rename = "HttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_parameters: Option<HttpParameters>,
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: String,
    #[serde(rename = "Input")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(rename = "InputPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_path: Option<String>,
    #[serde(rename = "InputTransformer")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_transformer: Option<InputTransformer>,
    #[serde(rename = "KinesisParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinesis_parameters: Option<KinesisParameters>,
    #[serde(rename = "RedshiftDataParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_data_parameters: Option<RedshiftDataParameters>,
    #[serde(rename = "RetryPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_policy: Option<RetryPolicy>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RunCommandParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_command_parameters: Option<RunCommandParameters>,
    #[serde(rename = "SageMakerPipelineParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sage_maker_pipeline_parameters: Option<SageMakerPipelineParameters>,
    #[serde(rename = "SqsParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqs_parameters: Option<SqsParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppSyncParameters {
    #[serde(rename = "GraphQLOperation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_q_l_operation: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchParameters {
    #[serde(rename = "ArrayProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<BatchArrayProperties>,
    #[serde(rename = "JobDefinition")]
    #[serde(default)]
    pub job_definition: String,
    #[serde(rename = "JobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "RetryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<BatchRetryStrategy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchArrayProperties {
    #[serde(rename = "Size")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchRetryStrategy {
    #[serde(rename = "Attempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsParameters {
    #[serde(rename = "CapacityProviderStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem>>,
    #[serde(rename = "EnableECSManagedTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_e_c_s_managed_tags: Option<bool>,
    #[serde(rename = "EnableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "Group")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(rename = "LaunchType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_type: Option<String>,
    #[serde(rename = "NetworkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "PlacementConstraints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_constraints: Option<Vec<PlacementConstraint>>,
    #[serde(rename = "PlacementStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_strategy: Option<Vec<PlacementStrategy>>,
    #[serde(rename = "PlatformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "PropagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<String>,
    #[serde(rename = "ReferenceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TaskCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_count: Option<i32>,
    #[serde(rename = "TaskDefinitionArn")]
    #[serde(default)]
    pub task_definition_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityProviderStrategyItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<i32>,
    #[serde(rename = "capacityProvider")]
    #[serde(default)]
    pub capacity_provider: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "awsvpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub awsvpc_configuration: Option<AwsVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AwsVpcConfiguration {
    #[serde(rename = "AssignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
    #[serde(rename = "SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    pub subnets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementConstraint {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlacementStrategy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HttpParameters {
    #[serde(rename = "HeaderParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "PathParameterValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path_parameter_values: Option<Vec<String>>,
    #[serde(rename = "QueryStringParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_string_parameters: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputTransformer {
    #[serde(rename = "InputPathsMap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_paths_map: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "InputTemplate")]
    #[serde(default)]
    pub input_template: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KinesisParameters {
    #[serde(rename = "PartitionKeyPath")]
    #[serde(default)]
    pub partition_key_path: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftDataParameters {
    #[serde(rename = "Database")]
    #[serde(default)]
    pub database: String,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "SecretManagerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_manager_arn: Option<String>,
    #[serde(rename = "Sql")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sql: Option<String>,
    #[serde(rename = "Sqls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sqls: Option<Vec<String>>,
    #[serde(rename = "StatementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_name: Option<String>,
    #[serde(rename = "WithEvent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_event: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryPolicy {
    #[serde(rename = "MaximumEventAgeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_event_age_in_seconds: Option<i32>,
    #[serde(rename = "MaximumRetryAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_retry_attempts: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunCommandParameters {
    #[serde(rename = "RunCommandTargets")]
    #[serde(default)]
    pub run_command_targets: Vec<RunCommandTarget>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RunCommandTarget {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerPipelineParameters {
    #[serde(rename = "PipelineParameterList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SageMakerPipelineParameter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SqsParameters {
    #[serde(rename = "MessageGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsRequest {
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<PutEventsRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsRequestEntry {
    #[serde(rename = "Detail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
    #[serde(rename = "TraceHeader")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_header: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsResponse {
    #[serde(rename = "Entries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutEventsResultEntry>>,
    #[serde(rename = "FailedEntryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutEventsResultEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPartnerEventsRequest {
    #[serde(rename = "Entries")]
    #[serde(default)]
    pub entries: Vec<PutPartnerEventsRequestEntry>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPartnerEventsRequestEntry {
    #[serde(rename = "Detail")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "DetailType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail_type: Option<String>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Time")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPartnerEventsResponse {
    #[serde(rename = "Entries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<PutPartnerEventsResultEntry>>,
    #[serde(rename = "FailedEntryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPartnerEventsResultEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutPermissionRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "Condition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Condition {
    #[serde(rename = "Key")]
    #[serde(default)]
    pub key: String,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuleRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutRuleResponse {
    #[serde(rename = "RuleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTargetsRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: String,
    #[serde(rename = "Targets")]
    #[serde(default)]
    pub targets: Vec<Target>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTargetsResponse {
    #[serde(rename = "FailedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<PutTargetsResultEntry>>,
    #[serde(rename = "FailedEntryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutTargetsResultEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemovePermissionRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "RemoveAllPermissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_all_permissions: Option<bool>,
    #[serde(rename = "StatementId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTargetsRequest {
    #[serde(rename = "EventBusName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bus_name: Option<String>,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "Ids")]
    #[serde(default)]
    pub ids: Vec<String>,
    #[serde(rename = "Rule")]
    #[serde(default)]
    pub rule: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTargetsResponse {
    #[serde(rename = "FailedEntries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entries: Option<Vec<RemoveTargetsResultEntry>>,
    #[serde(rename = "FailedEntryCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_entry_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTargetsResultEntry {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "TargetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplayRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Destination")]
    #[serde(default)]
    pub destination: ReplayDestination,
    #[serde(rename = "EventEndTime")]
    #[serde(default)]
    pub event_end_time: f64,
    #[serde(rename = "EventSourceArn")]
    #[serde(default)]
    pub event_source_arn: String,
    #[serde(rename = "EventStartTime")]
    #[serde(default)]
    pub event_start_time: f64,
    #[serde(rename = "ReplayName")]
    #[serde(default)]
    pub replay_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartReplayResponse {
    #[serde(rename = "ReplayArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_arn: Option<String>,
    #[serde(rename = "ReplayStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestEventPatternRequest {
    #[serde(rename = "Event")]
    #[serde(default)]
    pub event: String,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    pub event_pattern: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TestEventPatternResponse {
    #[serde(rename = "Result")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiDestinationRequest {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "InvocationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_endpoint: Option<String>,
    #[serde(rename = "InvocationRateLimitPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_rate_limit_per_second: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApiDestinationResponse {
    #[serde(rename = "ApiDestinationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_arn: Option<String>,
    #[serde(rename = "ApiDestinationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_destination_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateArchiveRequest {
    #[serde(rename = "ArchiveName")]
    #[serde(default)]
    pub archive_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventPattern")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_pattern: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "RetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_days: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateArchiveResponse {
    #[serde(rename = "ArchiveArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_arn: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "StateReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionRequest {
    #[serde(rename = "AuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_parameters: Option<UpdateConnectionAuthRequestParameters>,
    #[serde(rename = "AuthorizationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "InvocationConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_connectivity_parameters: Option<ConnectivityResourceParameters>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionAuthRequestParameters {
    #[serde(rename = "ApiKeyAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_auth_parameters: Option<UpdateConnectionApiKeyAuthRequestParameters>,
    #[serde(rename = "BasicAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_auth_parameters: Option<UpdateConnectionBasicAuthRequestParameters>,
    #[serde(rename = "ConnectivityParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connectivity_parameters: Option<ConnectivityResourceParameters>,
    #[serde(rename = "InvocationHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_http_parameters: Option<ConnectionHttpParameters>,
    #[serde(rename = "OAuthParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_parameters: Option<UpdateConnectionOAuthRequestParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionApiKeyAuthRequestParameters {
    #[serde(rename = "ApiKeyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_name: Option<String>,
    #[serde(rename = "ApiKeyValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionBasicAuthRequestParameters {
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "Username")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionOAuthRequestParameters {
    #[serde(rename = "AuthorizationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_endpoint: Option<String>,
    #[serde(rename = "ClientParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_parameters: Option<UpdateConnectionOAuthClientRequestParameters>,
    #[serde(rename = "HttpMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<String>,
    #[serde(rename = "OAuthHttpParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_auth_http_parameters: Option<ConnectionHttpParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionOAuthClientRequestParameters {
    #[serde(rename = "ClientID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_i_d: Option<String>,
    #[serde(rename = "ClientSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConnectionResponse {
    #[serde(rename = "ConnectionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_arn: Option<String>,
    #[serde(rename = "ConnectionState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_state: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "LastAuthorizedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_authorized_time: Option<f64>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EndpointEventBus>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<RoutingConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEndpointResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "EndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<String>,
    #[serde(rename = "EndpointUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_url: Option<String>,
    #[serde(rename = "EventBuses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_buses: Option<Vec<EndpointEventBus>>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_config: Option<ReplicationConfig>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "RoutingConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_config: Option<RoutingConfig>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBusRequest {
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventBusResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "DeadLetterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dead_letter_config: Option<DeadLetterConfig>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
    #[serde(rename = "LogConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_config: Option<LogConfig>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
