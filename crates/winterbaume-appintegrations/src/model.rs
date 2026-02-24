//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-appintegrations

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "ApplicationSourceConfig")]
    #[serde(default)]
    pub application_source_config: ApplicationSourceConfig,
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IframeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe_config: Option<IframeConfig>,
    #[serde(rename = "InitializationTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_timeout: Option<i32>,
    #[serde(rename = "IsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    pub namespace: String,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Publications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationConfig {
    #[serde(rename = "ContactHandling")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_handling: Option<ContactHandling>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContactHandling {
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSourceConfig {
    #[serde(rename = "ExternalUrlConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_url_config: Option<ExternalUrlConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalUrlConfig {
    #[serde(rename = "AccessUrl")]
    #[serde(default)]
    pub access_url: String,
    #[serde(rename = "ApprovedOrigins")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_origins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IframeConfig {
    #[serde(rename = "Allow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Vec<String>>,
    #[serde(rename = "Sandbox")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Publication {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Event")]
    #[serde(default)]
    pub event: String,
    #[serde(rename = "Schema")]
    #[serde(default)]
    pub schema: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Subscription {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Event")]
    #[serde(default)]
    pub event: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataIntegrationAssociationRequest {
    #[serde(rename = "ClientAssociationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_association_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DataIntegrationIdentifier")]
    #[serde(default)]
    pub data_integration_identifier: String,
    #[serde(rename = "DestinationURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_u_r_i: Option<String>,
    #[serde(rename = "ExecutionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<ExecutionConfiguration>,
    #[serde(rename = "ObjectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_configuration:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionConfiguration {
    #[serde(rename = "ExecutionMode")]
    #[serde(default)]
    pub execution_mode: String,
    #[serde(rename = "OnDemandConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_configuration: Option<OnDemandConfiguration>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<ScheduleConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OnDemandConfiguration {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    pub start_time: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleConfiguration {
    #[serde(rename = "FirstExecutionFrom")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_execution_from: Option<String>,
    #[serde(rename = "Object")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    pub schedule_expression: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataIntegrationAssociationResponse {
    #[serde(rename = "DataIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integration_arn: Option<String>,
    #[serde(rename = "DataIntegrationAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integration_association_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataIntegrationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FileConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_configuration: Option<FileConfiguration>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    pub kms_key: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ObjectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_configuration:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "ScheduleConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_config: Option<ScheduleConfiguration>,
    #[serde(rename = "SourceURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_u_r_i: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileConfiguration {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Folders")]
    #[serde(default)]
    pub folders: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataIntegrationResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FileConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_configuration: Option<FileConfiguration>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_configuration:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<ScheduleConfiguration>,
    #[serde(rename = "SourceURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_u_r_i: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventIntegrationRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBridgeBus")]
    #[serde(default)]
    pub event_bridge_bus: String,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    pub event_filter: EventFilter,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventFilter {
    #[serde(rename = "Source")]
    #[serde(default)]
    pub source: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateEventIntegrationResponse {
    #[serde(rename = "EventIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataIntegrationRequest {
    #[serde(rename = "DataIntegrationIdentifier")]
    #[serde(default)]
    pub data_integration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataIntegrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventIntegrationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteEventIntegrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRequest {
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationResponse {
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "ApplicationSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_source_config: Option<ApplicationSourceConfig>,
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IframeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe_config: Option<IframeConfig>,
    #[serde(rename = "InitializationTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_timeout: Option<i32>,
    #[serde(rename = "IsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service: Option<bool>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Publications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataIntegrationRequest {
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataIntegrationResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "FileConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_configuration: Option<FileConfiguration>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "KmsKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ObjectConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_configuration:
        Option<std::collections::HashMap<String, std::collections::HashMap<String, Vec<String>>>>,
    #[serde(rename = "ScheduleConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_configuration: Option<ScheduleConfiguration>,
    #[serde(rename = "SourceURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_u_r_i: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventIntegrationRequest {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetEventIntegrationResponse {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBridgeBus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_bus: Option<String>,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "EventIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssociationsRequest {
    #[serde(rename = "ApplicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationAssociationsResponse {
    #[serde(rename = "ApplicationAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_associations: Option<Vec<ApplicationAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationAssociationSummary {
    #[serde(rename = "ApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
    #[serde(rename = "ApplicationAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_association_arn: Option<String>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsResponse {
    #[serde(rename = "Applications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applications: Option<Vec<ApplicationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSummary {
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "IsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service: Option<bool>,
    #[serde(rename = "LastModifiedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Namespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataIntegrationAssociationsRequest {
    #[serde(rename = "DataIntegrationIdentifier")]
    #[serde(default)]
    pub data_integration_identifier: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataIntegrationAssociationsResponse {
    #[serde(rename = "DataIntegrationAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integration_associations: Option<Vec<DataIntegrationAssociationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataIntegrationAssociationSummary {
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "DataIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integration_arn: Option<String>,
    #[serde(rename = "DataIntegrationAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integration_association_arn: Option<String>,
    #[serde(rename = "DestinationURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_u_r_i: Option<String>,
    #[serde(rename = "ExecutionConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_configuration: Option<ExecutionConfiguration>,
    #[serde(rename = "LastExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_execution_status: Option<LastExecutionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LastExecutionStatus {
    #[serde(rename = "ExecutionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataIntegrationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataIntegrationsResponse {
    #[serde(rename = "DataIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_integrations: Option<Vec<DataIntegrationSummary>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataIntegrationSummary {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SourceURI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventIntegrationAssociationsRequest {
    #[serde(rename = "EventIntegrationName")]
    #[serde(default)]
    pub event_integration_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventIntegrationAssociationsResponse {
    #[serde(rename = "EventIntegrationAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_associations: Option<Vec<EventIntegrationAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventIntegrationAssociation {
    #[serde(rename = "ClientAssociationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_association_metadata: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ClientId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "EventBridgeRuleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_rule_name: Option<String>,
    #[serde(rename = "EventIntegrationAssociationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_association_arn: Option<String>,
    #[serde(rename = "EventIntegrationAssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_association_id: Option<String>,
    #[serde(rename = "EventIntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventIntegrationsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListEventIntegrationsResponse {
    #[serde(rename = "EventIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integrations: Option<Vec<EventIntegration>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventIntegration {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EventBridgeBus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_bridge_bus: Option<String>,
    #[serde(rename = "EventFilter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filter: Option<EventFilter>,
    #[serde(rename = "EventIntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_integration_arn: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
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
pub struct UpdateApplicationRequest {
    #[serde(rename = "ApplicationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_config: Option<ApplicationConfig>,
    #[serde(rename = "ApplicationSourceConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_source_config: Option<ApplicationSourceConfig>,
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IframeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe_config: Option<IframeConfig>,
    #[serde(rename = "InitializationTimeout")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initialization_timeout: Option<i32>,
    #[serde(rename = "IsService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_service: Option<bool>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Permissions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(rename = "Publications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publications: Option<Vec<Publication>>,
    #[serde(rename = "Subscriptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<Vec<Subscription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataIntegrationAssociationRequest {
    #[serde(rename = "DataIntegrationAssociationIdentifier")]
    #[serde(default)]
    pub data_integration_association_identifier: String,
    #[serde(rename = "DataIntegrationIdentifier")]
    #[serde(default)]
    pub data_integration_identifier: String,
    #[serde(rename = "ExecutionConfiguration")]
    #[serde(default)]
    pub execution_configuration: ExecutionConfiguration,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataIntegrationAssociationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataIntegrationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Identifier")]
    #[serde(default)]
    pub identifier: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataIntegrationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventIntegrationRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateEventIntegrationResponse {}
