//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appfabric

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetUserAccessTasksRequest {
    #[serde(rename = "appBundleIdentifier")]
    #[serde(default)]
    pub app_bundle_identifier: String,
    #[serde(rename = "taskIdList")]
    #[serde(default)]
    pub task_id_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetUserAccessTasksResponse {
    #[serde(rename = "userAccessResultsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_results_list: Option<Vec<UserAccessResultItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAccessResultItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "resultStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_status: Option<String>,
    #[serde(rename = "taskError")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_error: Option<TaskError>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "tenantDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_display_name: Option<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "userFirstName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_first_name: Option<String>,
    #[serde(rename = "userFullName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_full_name: Option<String>,
    #[serde(rename = "userId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "userLastName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_last_name: Option<String>,
    #[serde(rename = "userStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskError {
    #[serde(rename = "errorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "errorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectAppAuthorizationRequest {
    #[serde(rename = "authRequest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_request: Option<AuthRequest>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthRequest {
    #[serde(default)]
    pub code: String,
    #[serde(rename = "redirectUri")]
    #[serde(default)]
    pub redirect_uri: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectAppAuthorizationResponse {
    #[serde(rename = "appAuthorizationSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization_summary: Option<AppAuthorizationSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppAuthorizationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "appAuthorizationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization_arn: Option<String>,
    #[serde(rename = "appBundleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tenant {
    #[serde(rename = "tenantDisplayName")]
    #[serde(default)]
    pub tenant_display_name: String,
    #[serde(rename = "tenantIdentifier")]
    #[serde(default)]
    pub tenant_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppAuthorizationRequest {
    #[serde(default)]
    pub app: String,
    #[serde(rename = "authType")]
    #[serde(default)]
    pub auth_type: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub credential: Credential,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(default)]
    pub tenant: Tenant,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Credential {
    #[serde(rename = "apiKeyCredential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_credential: Option<ApiKeyCredential>,
    #[serde(rename = "oauth2Credential")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth2_credential: Option<Oauth2Credential>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiKeyCredential {
    #[serde(rename = "apiKey")]
    #[serde(default)]
    pub api_key: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Oauth2Credential {
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
    #[serde(rename = "clientSecret")]
    #[serde(default)]
    pub client_secret: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
    #[serde(default)]
    pub key: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppAuthorizationResponse {
    #[serde(rename = "appAuthorization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization: Option<AppAuthorization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppAuthorization {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "appAuthorizationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization_arn: Option<String>,
    #[serde(rename = "appBundleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle_arn: Option<String>,
    #[serde(rename = "authType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(rename = "authUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppBundleRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "customerManagedKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAppBundleResponse {
    #[serde(rename = "appBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle: Option<AppBundle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppBundle {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "customerManagedKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_managed_key_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionDestinationRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    pub destination_configuration: DestinationConfiguration,
    #[serde(rename = "processingConfiguration")]
    #[serde(default)]
    pub processing_configuration: ProcessingConfiguration,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DestinationConfiguration {
    #[serde(rename = "auditLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log: Option<AuditLogDestinationConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditLogDestinationConfiguration {
    #[serde(default)]
    pub destination: Destination,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Destination {
    #[serde(rename = "firehoseStream")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firehose_stream: Option<FirehoseStream>,
    #[serde(rename = "s3Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<S3Bucket>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirehoseStream {
    #[serde(rename = "streamName")]
    #[serde(default)]
    pub stream_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Bucket {
    #[serde(rename = "bucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProcessingConfiguration {
    #[serde(rename = "auditLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log: Option<AuditLogProcessingConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditLogProcessingConfiguration {
    #[serde(default)]
    pub format: String,
    #[serde(default)]
    pub schema: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionDestinationResponse {
    #[serde(rename = "ingestionDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_destination: Option<IngestionDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionDestination {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_configuration: Option<DestinationConfiguration>,
    #[serde(rename = "ingestionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_arn: Option<String>,
    #[serde(rename = "processingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_configuration: Option<ProcessingConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionRequest {
    #[serde(default)]
    pub app: String,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "ingestionType")]
    #[serde(default)]
    pub ingestion_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    pub tenant_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIngestionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion: Option<Ingestion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ingestion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(rename = "appBundleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "ingestionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppAuthorizationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppAuthorizationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppBundleRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteAppBundleResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIngestionDestinationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIngestionDestinationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIngestionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIngestionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppAuthorizationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppAuthorizationResponse {
    #[serde(rename = "appAuthorization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization: Option<AppAuthorization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppBundleRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetAppBundleResponse {
    #[serde(rename = "appBundle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle: Option<AppBundle>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionDestinationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionDestinationResponse {
    #[serde(rename = "ingestionDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_destination: Option<IngestionDestination>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIngestionResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion: Option<Ingestion>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAuthorizationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppAuthorizationsResponse {
    #[serde(rename = "appAuthorizationSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization_summary_list: Option<Vec<AppAuthorizationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppBundlesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListAppBundlesResponse {
    #[serde(rename = "appBundleSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_bundle_summary_list: Option<Vec<AppBundleSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppBundleSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionDestinationsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionDestinationsResponse {
    #[serde(rename = "ingestionDestinations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_destinations: Option<Vec<IngestionDestinationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionDestinationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListIngestionsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestions: Option<Vec<IngestionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IngestionSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartIngestionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartIngestionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUserAccessTasksRequest {
    #[serde(rename = "appBundleIdentifier")]
    #[serde(default)]
    pub app_bundle_identifier: String,
    #[serde(default)]
    pub email: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartUserAccessTasksResponse {
    #[serde(rename = "userAccessTasksList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_access_tasks_list: Option<Vec<UserAccessTaskItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserAccessTaskItem {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<TaskError>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "tenantId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopIngestionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopIngestionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppAuthorizationRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credential: Option<Credential>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant: Option<Tenant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateAppAuthorizationResponse {
    #[serde(rename = "appAuthorization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_authorization: Option<AppAuthorization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIngestionDestinationRequest {
    #[serde(rename = "destinationConfiguration")]
    #[serde(default)]
    pub destination_configuration: DestinationConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIngestionDestinationResponse {
    #[serde(rename = "ingestionDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingestion_destination: Option<IngestionDestination>,
}
