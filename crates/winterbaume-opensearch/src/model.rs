//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-opensearch

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInboundConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AcceptInboundConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<InboundConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundConnection {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "ConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<InboundConnectionStatus>,
    #[serde(rename = "LocalDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_domain_info: Option<DomainInformationContainer>,
    #[serde(rename = "RemoteDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_info: Option<DomainInformationContainer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundConnectionStatus {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainInformationContainer {
    #[serde(rename = "AWSDomainInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_w_s_domain_information: Option<AWSDomainInformation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AWSDomainInformation {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddDataSourceRequest {
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    pub data_source_type: DataSourceType,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceType {
    #[serde(rename = "S3GlueDataCatalog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_glue_data_catalog: Option<S3GlueDataCatalog>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3GlueDataCatalog {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddDataSourceResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddDirectQueryDataSourceRequest {
    #[serde(rename = "DataSourceAccessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_access_policy: Option<String>,
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    pub data_source_type: DirectQueryDataSourceType,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OpenSearchArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_arns: Option<Vec<String>>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectQueryDataSourceType {
    #[serde(rename = "CloudWatchLog")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_log: Option<CloudWatchDirectQueryDataSource>,
    #[serde(rename = "Prometheus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prometheus: Option<PrometheusDirectQueryDataSource>,
    #[serde(rename = "SecurityLake")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_lake: Option<SecurityLakeDirectQueryDataSource>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CloudWatchDirectQueryDataSource {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PrometheusDirectQueryDataSource {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
    #[serde(rename = "WorkspaceArn")]
    #[serde(default)]
    pub workspace_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityLakeDirectQueryDataSource {
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
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
pub struct AddDirectQueryDataSourceResponse {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
    #[serde(rename = "TagList")]
    #[serde(default)]
    pub tag_list: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePackageRequest {
    #[serde(rename = "AssociationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_configuration: Option<PackageAssociationConfiguration>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
    #[serde(rename = "PrerequisitePackageIDList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_package_i_d_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageAssociationConfiguration {
    #[serde(rename = "KeyStoreAccessOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_store_access_option: Option<KeyStoreAccessOption>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyStoreAccessOption {
    #[serde(rename = "KeyAccessRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_access_role_arn: Option<String>,
    #[serde(rename = "KeyStoreAccessEnabled")]
    #[serde(default)]
    pub key_store_access_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePackageResponse {
    #[serde(rename = "DomainPackageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details: Option<DomainPackageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainPackageDetails {
    #[serde(rename = "AssociationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_configuration: Option<PackageAssociationConfiguration>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainPackageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_status: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_i_d: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PrerequisitePackageIDList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_package_i_d_list: Option<Vec<String>>,
    #[serde(rename = "ReferencePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ErrorDetails {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "ErrorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePackagesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "PackageList")]
    #[serde(default)]
    pub package_list: Vec<PackageDetailsForAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageDetailsForAssociation {
    #[serde(rename = "AssociationConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_configuration: Option<PackageAssociationConfiguration>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
    #[serde(rename = "PrerequisitePackageIDList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prerequisite_package_i_d_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatePackagesResponse {
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizeVpcEndpointAccessRequest {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "ServiceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_options: Option<ServiceOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceOptions {
    #[serde(rename = "SupportedRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_regions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizeVpcEndpointAccessResponse {
    #[serde(rename = "AuthorizedPrincipal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_principal: Option<AuthorizedPrincipal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizedPrincipal {
    #[serde(rename = "Principal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[serde(rename = "PrincipalType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub principal_type: Option<String>,
    #[serde(rename = "ServiceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_options: Option<ServiceOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDomainConfigChangeRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDomainConfigChangeResponse {
    #[serde(rename = "CancelledChangeIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_change_ids: Option<Vec<String>>,
    #[serde(rename = "CancelledChangeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_change_properties: Option<Vec<CancelledChangeProperty>>,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelledChangeProperty {
    #[serde(rename = "ActiveValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_value: Option<String>,
    #[serde(rename = "CancelledValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancelled_value: Option<String>,
    #[serde(rename = "PropertyName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelServiceSoftwareUpdateRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelServiceSoftwareUpdateResponse {
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceSoftwareOptions {
    #[serde(rename = "AutomatedUpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_update_date: Option<f64>,
    #[serde(rename = "Cancellable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    #[serde(rename = "OptionalDeployment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_deployment: Option<bool>,
    #[serde(rename = "UpdateAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_available: Option<bool>,
    #[serde(rename = "UpdateStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationRequest {
    #[serde(rename = "appConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_configs: Option<Vec<AppConfig>>,
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<IamIdentityCenterOptionsInput>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSource {
    #[serde(rename = "dataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(rename = "dataSourceDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_description: Option<String>,
    #[serde(rename = "iamRoleForDataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_for_data_source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamIdentityCenterOptionsInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "iamIdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_instance_arn: Option<String>,
    #[serde(rename = "iamRoleForIdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_for_identity_center_application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationResponse {
    #[serde(rename = "appConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_configs: Option<Vec<AppConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<IamIdentityCenterOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamIdentityCenterOptions {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "iamIdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_application_arn: Option<String>,
    #[serde(rename = "iamIdentityCenterInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_instance_arn: Option<String>,
    #[serde(rename = "iamRoleForIdentityCenterApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_for_identity_center_application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainRequest {
    #[serde(rename = "AIMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_i_m_l_options: Option<AIMLOptionsInput>,
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
    #[serde(rename = "AutoTuneOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_options: Option<AutoTuneOptionsInput>,
    #[serde(rename = "ClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<ClusterConfig>,
    #[serde(rename = "CognitoOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    #[serde(rename = "DeploymentStrategyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_options: Option<DeploymentStrategyOptions>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "EBSOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_options: Option<EBSOptions>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<String>,
    #[serde(rename = "IdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_options: Option<IdentityCenterOptionsInput>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    #[serde(rename = "OffPeakWindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,
    #[serde(rename = "SnapshotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    #[serde(rename = "SoftwareUpdateOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_options: Option<SoftwareUpdateOptions>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
    #[serde(rename = "VPCOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_options: Option<VPCOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AIMLOptionsInput {
    #[serde(rename = "NaturalLanguageQueryGenerationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_language_query_generation_options:
        Option<NaturalLanguageQueryGenerationOptionsInput>,
    #[serde(rename = "S3VectorsEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_vectors_engine: Option<S3VectorsEngine>,
    #[serde(rename = "ServerlessVectorAcceleration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_vector_acceleration: Option<ServerlessVectorAcceleration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NaturalLanguageQueryGenerationOptionsInput {
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3VectorsEngine {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServerlessVectorAcceleration {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedSecurityOptionsInput {
    #[serde(rename = "AnonymousAuthEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_auth_enabled: Option<bool>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IAMFederationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_federation_options: Option<IAMFederationOptionsInput>,
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,
    #[serde(rename = "JWTOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_w_t_options: Option<JWTOptionsInput>,
    #[serde(rename = "MasterUserOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_options: Option<MasterUserOptions>,
    #[serde(rename = "SAMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_options: Option<SAMLOptionsInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IAMFederationOptionsInput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JWTOptionsInput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "JwksUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_url: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MasterUserOptions {
    #[serde(rename = "MasterUserARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_a_r_n: Option<String>,
    #[serde(rename = "MasterUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAMLOptionsInput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Idp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<SAMLIdp>,
    #[serde(rename = "MasterBackendRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_backend_role: Option<String>,
    #[serde(rename = "MasterUserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_name: Option<String>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SessionTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_minutes: Option<i32>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAMLIdp {
    #[serde(rename = "EntityId")]
    #[serde(default)]
    pub entity_id: String,
    #[serde(rename = "MetadataContent")]
    #[serde(default)]
    pub metadata_content: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneOptionsInput {
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "MaintenanceSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedules: Option<Vec<AutoTuneMaintenanceSchedule>>,
    #[serde(rename = "UseOffPeakWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_off_peak_window: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneMaintenanceSchedule {
    #[serde(rename = "CronExpressionForRecurrence")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cron_expression_for_recurrence: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<Duration>,
    #[serde(rename = "StartAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Duration {
    #[serde(rename = "Unit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterConfig {
    #[serde(rename = "ColdStorageOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_storage_options: Option<ColdStorageOptions>,
    #[serde(rename = "DedicatedMasterCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_count: Option<i32>,
    #[serde(rename = "DedicatedMasterEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_enabled: Option<bool>,
    #[serde(rename = "DedicatedMasterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master_type: Option<String>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "MultiAZWithStandbyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_with_standby_enabled: Option<bool>,
    #[serde(rename = "NodeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_options: Option<Vec<NodeOption>>,
    #[serde(rename = "WarmCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_count: Option<i32>,
    #[serde(rename = "WarmEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_enabled: Option<bool>,
    #[serde(rename = "WarmType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_type: Option<String>,
    #[serde(rename = "ZoneAwarenessConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_config: Option<ZoneAwarenessConfig>,
    #[serde(rename = "ZoneAwarenessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_awareness_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ColdStorageOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeOption {
    #[serde(rename = "NodeConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_config: Option<NodeConfig>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeConfig {
    #[serde(rename = "Count")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ZoneAwarenessConfig {
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IdentityPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_pool_id: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "UserPoolId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_pool_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentStrategyOptions {
    #[serde(rename = "DeploymentStrategy")]
    #[serde(default)]
    pub deployment_strategy: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainEndpointOptions {
    #[serde(rename = "CustomEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint: Option<String>,
    #[serde(rename = "CustomEndpointCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_certificate_arn: Option<String>,
    #[serde(rename = "CustomEndpointEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_enabled: Option<bool>,
    #[serde(rename = "EnforceHTTPS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enforce_h_t_t_p_s: Option<bool>,
    #[serde(rename = "TLSSecurityPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_l_s_security_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSOptions {
    #[serde(rename = "EBSEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_enabled: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "Throughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput: Option<i32>,
    #[serde(rename = "VolumeSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_size: Option<i32>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAtRestOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterOptionsInput {
    #[serde(rename = "EnabledAPIAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_a_p_i_access: Option<bool>,
    #[serde(rename = "IdentityCenterInstanceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_a_r_n: Option<String>,
    #[serde(rename = "IdentityCenterInstanceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_region: Option<String>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogPublishingOption {
    #[serde(rename = "CloudWatchLogsLogGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_watch_logs_log_group_arn: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeToNodeEncryptionOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OffPeakWindowOptions {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "OffPeakWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window: Option<OffPeakWindow>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OffPeakWindow {
    #[serde(rename = "WindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_start_time: Option<WindowStartTime>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowStartTime {
    #[serde(rename = "Hours")]
    #[serde(default)]
    pub hours: i64,
    #[serde(rename = "Minutes")]
    #[serde(default)]
    pub minutes: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotOptions {
    #[serde(rename = "AutomatedSnapshotStartHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_start_hour: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwareUpdateOptions {
    #[serde(rename = "AutoSoftwareUpdateEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_software_update_enabled: Option<bool>,
    #[serde(rename = "UseLatestServiceSoftwareForBlueGreen")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_service_software_for_blue_green: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCOptions {
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDomainResponse {
    #[serde(rename = "DomainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<DomainStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainStatus {
    #[serde(rename = "AIMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_i_m_l_options: Option<AIMLOptionsOutput>,
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptions>,
    #[serde(rename = "AutoTuneOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_options: Option<AutoTuneOptionsOutput>,
    #[serde(rename = "ChangeProgressDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_details: Option<ChangeProgressDetails>,
    #[serde(rename = "ClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<ClusterConfig>,
    #[serde(rename = "CognitoOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    #[serde(rename = "Created")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<bool>,
    #[serde(rename = "Deleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    #[serde(rename = "DeploymentStrategyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_options: Option<DeploymentStrategyOptions>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    #[serde(rename = "DomainEndpointV2HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_v2_hosted_zone_id: Option<String>,
    #[serde(rename = "DomainId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_id: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "DomainProcessingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_processing_status: Option<String>,
    #[serde(rename = "EBSOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_options: Option<EBSOptions>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointV2")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_v2: Option<String>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<String>,
    #[serde(rename = "IdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_options: Option<IdentityCenterOptions>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,
    #[serde(rename = "ModifyingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifying_properties: Option<Vec<ModifyingProperties>>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    #[serde(rename = "OffPeakWindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,
    #[serde(rename = "Processing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing: Option<bool>,
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
    #[serde(rename = "SnapshotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    #[serde(rename = "SoftwareUpdateOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_options: Option<SoftwareUpdateOptions>,
    #[serde(rename = "UpgradeProcessing")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_processing: Option<bool>,
    #[serde(rename = "VPCOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_options: Option<VPCDerivedInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AIMLOptionsOutput {
    #[serde(rename = "NaturalLanguageQueryGenerationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub natural_language_query_generation_options:
        Option<NaturalLanguageQueryGenerationOptionsOutput>,
    #[serde(rename = "S3VectorsEngine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_vectors_engine: Option<S3VectorsEngine>,
    #[serde(rename = "ServerlessVectorAcceleration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_vector_acceleration: Option<ServerlessVectorAcceleration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NaturalLanguageQueryGenerationOptionsOutput {
    #[serde(rename = "CurrentState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedSecurityOptions {
    #[serde(rename = "AnonymousAuthDisableDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_auth_disable_date: Option<f64>,
    #[serde(rename = "AnonymousAuthEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymous_auth_enabled: Option<bool>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "IAMFederationOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_federation_options: Option<IAMFederationOptionsOutput>,
    #[serde(rename = "InternalUserDatabaseEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_user_database_enabled: Option<bool>,
    #[serde(rename = "JWTOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub j_w_t_options: Option<JWTOptionsOutput>,
    #[serde(rename = "SAMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_a_m_l_options: Option<SAMLOptionsOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IAMFederationOptionsOutput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JWTOptionsOutput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "JwksUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jwks_url: Option<String>,
    #[serde(rename = "PublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SAMLOptionsOutput {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "Idp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idp: Option<SAMLIdp>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SessionTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_timeout_minutes: Option<i32>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneOptionsOutput {
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UseOffPeakWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_off_peak_window: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeProgressDetails {
    #[serde(rename = "ChangeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    #[serde(rename = "ConfigChangeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_change_status: Option<String>,
    #[serde(rename = "InitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterOptions {
    #[serde(rename = "EnabledAPIAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_a_p_i_access: Option<bool>,
    #[serde(rename = "IdentityCenterApplicationARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_application_a_r_n: Option<String>,
    #[serde(rename = "IdentityCenterInstanceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_a_r_n: Option<String>,
    #[serde(rename = "IdentityCenterInstanceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_instance_region: Option<String>,
    #[serde(rename = "IdentityStoreId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_store_id: Option<String>,
    #[serde(rename = "RolesKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles_key: Option<String>,
    #[serde(rename = "SubjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ModifyingProperties {
    #[serde(rename = "ActiveValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_value: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PendingValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_value: Option<String>,
    #[serde(rename = "ValueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCDerivedInfo {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VPCId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "IndexSchema")]
    #[serde(default)]
    pub index_schema: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateIndexResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutboundConnectionRequest {
    #[serde(rename = "ConnectionAlias")]
    #[serde(default)]
    pub connection_alias: String,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<ConnectionProperties>,
    #[serde(rename = "LocalDomainInfo")]
    #[serde(default)]
    pub local_domain_info: DomainInformationContainer,
    #[serde(rename = "RemoteDomainInfo")]
    #[serde(default)]
    pub remote_domain_info: DomainInformationContainer,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConnectionProperties {
    #[serde(rename = "CrossClusterSearch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_cluster_search: Option<CrossClusterSearchConnectionProperties>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CrossClusterSearchConnectionProperties {
    #[serde(rename = "SkipUnavailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_unavailable: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOutboundConnectionResponse {
    #[serde(rename = "ConnectionAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_alias: Option<String>,
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<ConnectionProperties>,
    #[serde(rename = "ConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<OutboundConnectionStatus>,
    #[serde(rename = "LocalDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_domain_info: Option<DomainInformationContainer>,
    #[serde(rename = "RemoteDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_info: Option<DomainInformationContainer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundConnectionStatus {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageRequest {
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "PackageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_configuration: Option<PackageConfiguration>,
    #[serde(rename = "PackageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    #[serde(rename = "PackageEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_encryption_options: Option<PackageEncryptionOptions>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "PackageSource")]
    #[serde(default)]
    pub package_source: PackageSource,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    pub package_type: String,
    #[serde(rename = "PackageVendingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_vending_options: Option<PackageVendingOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageConfiguration {
    #[serde(rename = "ConfigurationRequirement")]
    #[serde(default)]
    pub configuration_requirement: String,
    #[serde(rename = "LicenseFilepath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_filepath: Option<String>,
    #[serde(rename = "LicenseRequirement")]
    #[serde(default)]
    pub license_requirement: String,
    #[serde(rename = "RequiresRestartForConfigurationUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_restart_for_configuration_update: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageEncryptionOptions {
    #[serde(rename = "EncryptionEnabled")]
    #[serde(default)]
    pub encryption_enabled: bool,
    #[serde(rename = "KmsKeyIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageSource {
    #[serde(rename = "S3BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket_name: Option<String>,
    #[serde(rename = "S3Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVendingOptions {
    #[serde(rename = "VendingEnabled")]
    #[serde(default)]
    pub vending_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageResponse {
    #[serde(rename = "PackageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageDetails {
    #[serde(rename = "AllowListedUserList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_listed_user_list: Option<Vec<String>>,
    #[serde(rename = "AvailablePackageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_package_configuration: Option<PackageConfiguration>,
    #[serde(rename = "AvailablePackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_package_version: Option<String>,
    #[serde(rename = "AvailablePluginProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_plugin_properties: Option<PluginProperties>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "ErrorDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_details: Option<ErrorDetails>,
    #[serde(rename = "LastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "PackageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    #[serde(rename = "PackageEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_encryption_options: Option<PackageEncryptionOptions>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_i_d: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackageOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_owner: Option<String>,
    #[serde(rename = "PackageStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_status: Option<String>,
    #[serde(rename = "PackageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_type: Option<String>,
    #[serde(rename = "PackageVendingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_vending_options: Option<PackageVendingOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PluginProperties {
    #[serde(rename = "ClassName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub class_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "UncompressedSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncompressed_size_in_bytes: Option<i64>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "DomainArn")]
    #[serde(default)]
    pub domain_arn: String,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    pub vpc_options: VPCOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVpcEndpointResponse {
    #[serde(rename = "VpcEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<VpcEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpoint {
    #[serde(rename = "DomainArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcEndpointOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner: Option<String>,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_options: Option<VPCDerivedInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteApplicationResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataSourceResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDirectQueryDataSourceRequest {
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDomainResponse {
    #[serde(rename = "DomainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<DomainStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInboundConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteInboundConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<InboundConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteIndexResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutboundConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteOutboundConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<OutboundConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutboundConnection {
    #[serde(rename = "ConnectionAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_alias: Option<String>,
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<String>,
    #[serde(rename = "ConnectionMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_mode: Option<String>,
    #[serde(rename = "ConnectionProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_properties: Option<ConnectionProperties>,
    #[serde(rename = "ConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<OutboundConnectionStatus>,
    #[serde(rename = "LocalDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_domain_info: Option<DomainInformationContainer>,
    #[serde(rename = "RemoteDomainInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_domain_info: Option<DomainInformationContainer>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageRequest {
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageResponse {
    #[serde(rename = "PackageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointRequest {
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    pub vpc_endpoint_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVpcEndpointResponse {
    #[serde(rename = "VpcEndpointSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_summary: Option<VpcEndpointSummary>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointSummary {
    #[serde(rename = "DomainArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcEndpointOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_owner: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterCapabilityRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterCapabilityResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainAutoTunesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
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
pub struct DescribeDomainAutoTunesResponse {
    #[serde(rename = "AutoTunes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tunes: Option<Vec<AutoTune>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTune {
    #[serde(rename = "AutoTuneDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_details: Option<AutoTuneDetails>,
    #[serde(rename = "AutoTuneType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneDetails {
    #[serde(rename = "ScheduledAutoTuneDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_auto_tune_details: Option<ScheduledAutoTuneDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledAutoTuneDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<f64>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainChangeProgressRequest {
    #[serde(rename = "ChangeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainChangeProgressResponse {
    #[serde(rename = "ChangeProgressStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_status: Option<ChangeProgressStatusDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeProgressStatusDetails {
    #[serde(rename = "ChangeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_id: Option<String>,
    #[serde(rename = "ChangeProgressStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_stages: Option<Vec<ChangeProgressStage>>,
    #[serde(rename = "CompletedProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_properties: Option<Vec<String>>,
    #[serde(rename = "ConfigChangeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_change_status: Option<String>,
    #[serde(rename = "InitiatedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiated_by: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "PendingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_properties: Option<Vec<String>>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalNumberOfStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_number_of_stages: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ChangeProgressStage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainConfigRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainConfigResponse {
    #[serde(rename = "DomainConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config: Option<DomainConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainConfig {
    #[serde(rename = "AIMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_i_m_l_options: Option<AIMLOptionsStatus>,
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<AccessPoliciesStatus>,
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<AdvancedOptionsStatus>,
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsStatus>,
    #[serde(rename = "AutoTuneOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_options: Option<AutoTuneOptionsStatus>,
    #[serde(rename = "ChangeProgressDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_details: Option<ChangeProgressDetails>,
    #[serde(rename = "ClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<ClusterConfigStatus>,
    #[serde(rename = "CognitoOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptionsStatus>,
    #[serde(rename = "DeploymentStrategyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_options: Option<DeploymentStrategyOptionsStatus>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptionsStatus>,
    #[serde(rename = "EBSOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_options: Option<EBSOptionsStatus>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptionsStatus>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<VersionStatus>,
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<IPAddressTypeStatus>,
    #[serde(rename = "IdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_options: Option<IdentityCenterOptionsStatus>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<LogPublishingOptionsStatus>,
    #[serde(rename = "ModifyingProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifying_properties: Option<Vec<ModifyingProperties>>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptionsStatus>,
    #[serde(rename = "OffPeakWindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window_options: Option<OffPeakWindowOptionsStatus>,
    #[serde(rename = "SnapshotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptionsStatus>,
    #[serde(rename = "SoftwareUpdateOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_options: Option<SoftwareUpdateOptionsStatus>,
    #[serde(rename = "VPCOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_options: Option<VPCDerivedInfoStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AIMLOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AIMLOptionsOutput>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionStatus {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "PendingDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion: Option<bool>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
    #[serde(rename = "UpdateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccessPoliciesStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdvancedSecurityOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AdvancedSecurityOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<AutoTuneOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AutoTuneStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneOptions {
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "MaintenanceSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_schedules: Option<Vec<AutoTuneMaintenanceSchedule>>,
    #[serde(rename = "RollbackOnDisable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_on_disable: Option<String>,
    #[serde(rename = "UseOffPeakWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_off_peak_window: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoTuneStatus {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<f64>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "PendingDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_deletion: Option<bool>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<f64>,
    #[serde(rename = "UpdateVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_version: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterConfigStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<ClusterConfig>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CognitoOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CognitoOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeploymentStrategyOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DeploymentStrategyOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainEndpointOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<DomainEndpointOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EBSOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<EBSOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionAtRestOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VersionStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPAddressTypeStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IdentityCenterOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<IdentityCenterOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogPublishingOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, LogPublishingOption>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeToNodeEncryptionOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<NodeToNodeEncryptionOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OffPeakWindowOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OffPeakWindowOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SnapshotOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwareUpdateOptionsStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<SoftwareUpdateOptions>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VPCDerivedInfoStatus {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<VPCDerivedInfo>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptionStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainHealthRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainHealthResponse {
    #[serde(rename = "ActiveAvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_availability_zone_count: Option<String>,
    #[serde(rename = "AvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_count: Option<String>,
    #[serde(rename = "ClusterHealth")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_health: Option<String>,
    #[serde(rename = "DataNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_node_count: Option<String>,
    #[serde(rename = "DedicatedMaster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dedicated_master: Option<bool>,
    #[serde(rename = "DomainState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_state: Option<String>,
    #[serde(rename = "EnvironmentInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_information: Option<Vec<EnvironmentInfo>>,
    #[serde(rename = "MasterEligibleNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_eligible_node_count: Option<String>,
    #[serde(rename = "MasterNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_node: Option<String>,
    #[serde(rename = "StandByAvailabilityZoneCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stand_by_availability_zone_count: Option<String>,
    #[serde(rename = "TotalShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shards: Option<String>,
    #[serde(rename = "TotalUnAssignedShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_un_assigned_shards: Option<String>,
    #[serde(rename = "WarmNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_node_count: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EnvironmentInfo {
    #[serde(rename = "AvailabilityZoneInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_information: Option<Vec<AvailabilityZoneInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneInfo {
    #[serde(rename = "AvailabilityZoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_name: Option<String>,
    #[serde(rename = "AvailableDataNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_data_node_count: Option<String>,
    #[serde(rename = "ConfiguredDataNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configured_data_node_count: Option<String>,
    #[serde(rename = "TotalShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_shards: Option<String>,
    #[serde(rename = "TotalUnAssignedShards")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_un_assigned_shards: Option<String>,
    #[serde(rename = "ZoneStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zone_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainNodesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainNodesResponse {
    #[serde(rename = "DomainNodesStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_nodes_status_list: Option<Vec<DomainNodesStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainNodesStatus {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_status: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "StorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<String>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "StorageVolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainResponse {
    #[serde(rename = "DomainStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status: Option<DomainStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainsRequest {
    #[serde(rename = "DomainNames")]
    #[serde(default)]
    pub domain_names: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDomainsResponse {
    #[serde(rename = "DomainStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_status_list: Option<Vec<DomainStatus>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDryRunProgressRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "LoadDryRunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_dry_run_config: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDryRunProgressResponse {
    #[serde(rename = "DryRunConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_config: Option<DomainStatus>,
    #[serde(rename = "DryRunProgressStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_progress_status: Option<DryRunProgressStatus>,
    #[serde(rename = "DryRunResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_results: Option<DryRunResults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DryRunProgressStatus {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "DryRunId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_id: Option<String>,
    #[serde(rename = "DryRunStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_status: Option<String>,
    #[serde(rename = "UpdateDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_date: Option<String>,
    #[serde(rename = "ValidationFailures")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_failures: Option<Vec<ValidationFailure>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationFailure {
    #[serde(rename = "Code")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DryRunResults {
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInboundConnectionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInboundConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<InboundConnection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightDetailsRequest {
    #[serde(rename = "Entity")]
    #[serde(default)]
    pub entity: InsightEntity,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    pub insight_id: String,
    #[serde(rename = "ShowHtmlContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_html_content: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightEntity {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInsightDetailsResponse {
    #[serde(rename = "Fields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<InsightField>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightField {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceTypeLimitsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    pub engine_version: String,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    pub instance_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeInstanceTypeLimitsResponse {
    #[serde(rename = "LimitsByRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits_by_role: Option<std::collections::HashMap<String, Limits>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Limits {
    #[serde(rename = "AdditionalLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_limits: Option<Vec<AdditionalLimit>>,
    #[serde(rename = "InstanceLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_limits: Option<InstanceLimits>,
    #[serde(rename = "StorageTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_types: Option<Vec<StorageType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdditionalLimit {
    #[serde(rename = "LimitName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
    #[serde(rename = "LimitValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceLimits {
    #[serde(rename = "InstanceCountLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count_limits: Option<InstanceCountLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceCountLimits {
    #[serde(rename = "MaximumInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_instance_count: Option<i32>,
    #[serde(rename = "MinimumInstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_instance_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageType {
    #[serde(rename = "StorageSubTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_sub_type_name: Option<String>,
    #[serde(rename = "StorageTypeLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type_limits: Option<Vec<StorageTypeLimit>>,
    #[serde(rename = "StorageTypeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageTypeLimit {
    #[serde(rename = "LimitName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_name: Option<String>,
    #[serde(rename = "LimitValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeOutboundConnectionsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
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
pub struct DescribeOutboundConnectionsResponse {
    #[serde(rename = "Connections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<OutboundConnection>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackagesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DescribePackagesFilter>>,
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
pub struct DescribePackagesFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackagesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details_list: Option<Vec<PackageDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedInstanceOfferingsRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedInstanceOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedInstanceOfferingsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedInstanceOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_offerings: Option<Vec<ReservedInstanceOffering>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedInstanceOffering {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    #[serde(rename = "ReservedInstanceOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_offering_id: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurringCharge {
    #[serde(rename = "RecurringChargeAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_amount: Option<f64>,
    #[serde(rename = "RecurringChargeFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedInstancesRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeReservedInstancesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ReservedInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instances: Option<Vec<ReservedInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedInstance {
    #[serde(rename = "BillingSubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_subscription_id: Option<i64>,
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "PaymentOption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_option: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<Vec<RecurringCharge>>,
    #[serde(rename = "ReservationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[serde(rename = "ReservedInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_id: Option<String>,
    #[serde(rename = "ReservedInstanceOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_offering_id: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcEndpointsRequest {
    #[serde(rename = "VpcEndpointIds")]
    #[serde(default)]
    pub vpc_endpoint_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVpcEndpointsResponse {
    #[serde(rename = "VpcEndpointErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_errors: Option<Vec<VpcEndpointError>>,
    #[serde(rename = "VpcEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<Vec<VpcEndpoint>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DissociatePackageRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DissociatePackageResponse {
    #[serde(rename = "DomainPackageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details: Option<DomainPackageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DissociatePackagesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "PackageList")]
    #[serde(default)]
    pub package_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DissociatePackagesResponse {
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationRequest {
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetApplicationResponse {
    #[serde(rename = "appConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_configs: Option<Vec<AppConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<IamIdentityCenterOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "kmsKeyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_arn: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapabilityRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCapabilityResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "capabilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_config: Option<CapabilityExtendedResponseConfig>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<CapabilityFailure>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityExtendedResponseConfig {
    #[serde(rename = "aiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_config: Option<AIConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AIConfig {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityFailure {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompatibleVersionsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetCompatibleVersionsResponse {
    #[serde(rename = "CompatibleVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatible_versions: Option<Vec<CompatibleVersionsMap>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompatibleVersionsMap {
    #[serde(rename = "SourceVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_version: Option<String>,
    #[serde(rename = "TargetVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataSourceResponse {
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<DataSourceType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultApplicationSettingRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDefaultApplicationSettingResponse {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectQueryDataSourceRequest {
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDirectQueryDataSourceResponse {
    #[serde(rename = "DataSourceAccessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_access_policy: Option<String>,
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<DirectQueryDataSourceType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OpenSearchArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainMaintenanceStatusRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "MaintenanceId")]
    #[serde(default)]
    pub maintenance_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDomainMaintenanceStatusResponse {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetIndexResponse {
    #[serde(rename = "IndexSchema")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index_schema: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionHistoryRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetPackageVersionHistoryResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_i_d: Option<String>,
    #[serde(rename = "PackageVersionHistoryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_history_list: Option<Vec<PackageVersionHistory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionHistory {
    #[serde(rename = "CommitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "PackageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_configuration: Option<PackageConfiguration>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PluginProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin_properties: Option<PluginProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUpgradeHistoryRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
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
pub struct GetUpgradeHistoryResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "UpgradeHistories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_histories: Option<Vec<UpgradeHistory>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeHistory {
    #[serde(rename = "StartTimestamp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_timestamp: Option<f64>,
    #[serde(rename = "StepsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steps_list: Option<Vec<UpgradeStepItem>>,
    #[serde(rename = "UpgradeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_name: Option<String>,
    #[serde(rename = "UpgradeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeStepItem {
    #[serde(rename = "Issues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issues: Option<Vec<String>>,
    #[serde(rename = "ProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<f64>,
    #[serde(rename = "UpgradeStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<String>,
    #[serde(rename = "UpgradeStepStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUpgradeStatusRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetUpgradeStatusResponse {
    #[serde(rename = "StepStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step_status: Option<String>,
    #[serde(rename = "UpgradeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_name: Option<String>,
    #[serde(rename = "UpgradeStep")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_step: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsRequest {
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
    pub statuses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationsResponse {
    #[serde(rename = "ApplicationSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_summaries: Option<Vec<ApplicationSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDataSourcesResponse {
    #[serde(rename = "DataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSourceDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataSourceDetails {
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<DataSourceType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDirectQueryDataSourcesRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDirectQueryDataSourcesResponse {
    #[serde(rename = "DirectQueryDataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_query_data_sources: Option<Vec<DirectQueryDataSource>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DirectQueryDataSource {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_name: Option<String>,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_type: Option<DirectQueryDataSourceType>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OpenSearchArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_arns: Option<Vec<String>>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainMaintenancesRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainMaintenancesResponse {
    #[serde(rename = "DomainMaintenances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_maintenances: Option<Vec<DomainMaintenanceDetails>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainMaintenanceDetails {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "MaintenanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_id: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainNamesRequest {
    #[serde(rename = "EngineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainNamesResponse {
    #[serde(rename = "DomainNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<DomainInfo>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainInfo {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "EngineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsForPackageRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDomainsForPackageResponse {
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsRequest {
    #[serde(rename = "Entity")]
    #[serde(default)]
    pub entity: InsightEntity,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    #[serde(rename = "TimeRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_range: Option<InsightTimeRange>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InsightTimeRange {
    #[serde(rename = "From")]
    #[serde(default)]
    pub from: i64,
    #[serde(rename = "To")]
    #[serde(default)]
    pub to: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInsightsResponse {
    #[serde(rename = "Insights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insights: Option<Vec<Insight>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Insight {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "InsightId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insight_id: Option<String>,
    #[serde(rename = "IsExperimental")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_experimental: Option<bool>,
    #[serde(rename = "Priority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "UpdateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceTypeDetailsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    pub engine_version: String,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "RetrieveAZs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieve_a_zs: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListInstanceTypeDetailsResponse {
    #[serde(rename = "InstanceTypeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type_details: Option<Vec<InstanceTypeDetails>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceTypeDetails {
    #[serde(rename = "AdvancedSecurityEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_enabled: Option<bool>,
    #[serde(rename = "AppLogsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_logs_enabled: Option<bool>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<String>>,
    #[serde(rename = "CognitoEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_enabled: Option<bool>,
    #[serde(rename = "EncryptionEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_enabled: Option<bool>,
    #[serde(rename = "InstanceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<Vec<String>>,
    #[serde(rename = "InstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "WarmEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warm_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesForDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
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
pub struct ListPackagesForDomainResponse {
    #[serde(rename = "DomainPackageDetailsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_package_details_list: Option<Vec<DomainPackageDetails>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListScheduledActionsRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
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
pub struct ListScheduledActionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ScheduledActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<Vec<ScheduledAction>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledAction {
    #[serde(rename = "Cancellable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellable: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "Mandatory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandatory: Option<bool>,
    #[serde(rename = "ScheduledBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_by: Option<String>,
    #[serde(rename = "ScheduledTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_time: Option<i64>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVersionsRequest {
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
pub struct ListVersionsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointAccessRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointAccessResponse {
    #[serde(rename = "AuthorizedPrincipalList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_principal_list: Option<Vec<AuthorizedPrincipal>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsForDomainRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsForDomainResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcEndpointSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_summary_list: Option<Vec<VpcEndpointSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsRequest {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVpcEndpointsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VpcEndpointSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_summary_list: Option<Vec<VpcEndpointSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseReservedInstanceOfferingRequest {
    #[serde(rename = "InstanceCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_count: Option<i32>,
    #[serde(rename = "ReservationName")]
    #[serde(default)]
    pub reservation_name: String,
    #[serde(rename = "ReservedInstanceOfferingId")]
    #[serde(default)]
    pub reserved_instance_offering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseReservedInstanceOfferingResponse {
    #[serde(rename = "ReservationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_name: Option<String>,
    #[serde(rename = "ReservedInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDefaultApplicationSettingRequest {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    pub application_arn: String,
    #[serde(rename = "setAsDefault")]
    #[serde(default)]
    pub set_as_default: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutDefaultApplicationSettingResponse {
    #[serde(rename = "applicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCapabilityRequest {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    pub application_id: String,
    #[serde(rename = "capabilityConfig")]
    #[serde(default)]
    pub capability_config: CapabilityBaseRequestConfig,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    pub capability_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityBaseRequestConfig {
    #[serde(rename = "aiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_config: Option<AIConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterCapabilityResponse {
    #[serde(rename = "applicationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<String>,
    #[serde(rename = "capabilityConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_config: Option<CapabilityBaseResponseConfig>,
    #[serde(rename = "capabilityName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapabilityBaseResponseConfig {
    #[serde(rename = "aiConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ai_config: Option<AIConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInboundConnectionRequest {
    #[serde(rename = "ConnectionId")]
    #[serde(default)]
    pub connection_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RejectInboundConnectionResponse {
    #[serde(rename = "Connection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<InboundConnection>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsRequest {
    #[serde(rename = "ARN")]
    #[serde(default)]
    pub a_r_n: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeVpcEndpointAccessRequest {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Service")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "ServiceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_options: Option<ServiceOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevokeVpcEndpointAccessResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackServiceSoftwareUpdateRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackServiceSoftwareUpdateResponse {
    #[serde(rename = "RollbackServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_service_software_options: Option<RollbackServiceSoftwareOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RollbackServiceSoftwareOptions {
    #[serde(rename = "CurrentVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "NewVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_version: Option<String>,
    #[serde(rename = "RollbackAvailable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rollback_available: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainMaintenanceRequest {
    #[serde(rename = "Action")]
    #[serde(default)]
    pub action: String,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartDomainMaintenanceResponse {
    #[serde(rename = "MaintenanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartServiceSoftwareUpdateRequest {
    #[serde(rename = "DesiredStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_start_time: Option<i64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "ScheduleAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartServiceSoftwareUpdateResponse {
    #[serde(rename = "ServiceSoftwareOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_software_options: Option<ServiceSoftwareOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationRequest {
    #[serde(rename = "appConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_configs: Option<Vec<AppConfig>>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(default)]
    pub id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateApplicationResponse {
    #[serde(rename = "appConfigs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_configs: Option<Vec<AppConfig>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "dataSources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_sources: Option<Vec<DataSource>>,
    #[serde(rename = "iamIdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_identity_center_options: Option<IamIdentityCenterOptions>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceRequest {
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    pub data_source_type: DataSourceType,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataSourceResponse {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectQueryDataSourceRequest {
    #[serde(rename = "DataSourceAccessPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_access_policy: Option<String>,
    #[serde(rename = "DataSourceName")]
    #[serde(default)]
    pub data_source_name: String,
    #[serde(rename = "DataSourceType")]
    #[serde(default)]
    pub data_source_type: DirectQueryDataSourceType,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "OpenSearchArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_search_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDirectQueryDataSourceResponse {
    #[serde(rename = "DataSourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainConfigRequest {
    #[serde(rename = "AIMLOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_i_m_l_options: Option<AIMLOptionsInput>,
    #[serde(rename = "AccessPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_policies: Option<String>,
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "AdvancedSecurityOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_security_options: Option<AdvancedSecurityOptionsInput>,
    #[serde(rename = "AutoTuneOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_tune_options: Option<AutoTuneOptions>,
    #[serde(rename = "ClusterConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_config: Option<ClusterConfig>,
    #[serde(rename = "CognitoOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cognito_options: Option<CognitoOptions>,
    #[serde(rename = "DeploymentStrategyOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_strategy_options: Option<DeploymentStrategyOptions>,
    #[serde(rename = "DomainEndpointOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_endpoint_options: Option<DomainEndpointOptions>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "DryRunMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_mode: Option<String>,
    #[serde(rename = "EBSOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_b_s_options: Option<EBSOptions>,
    #[serde(rename = "EncryptionAtRestOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_at_rest_options: Option<EncryptionAtRestOptions>,
    #[serde(rename = "IPAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_address_type: Option<String>,
    #[serde(rename = "IdentityCenterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_center_options: Option<IdentityCenterOptionsInput>,
    #[serde(rename = "LogPublishingOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_publishing_options: Option<std::collections::HashMap<String, LogPublishingOption>>,
    #[serde(rename = "NodeToNodeEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_to_node_encryption_options: Option<NodeToNodeEncryptionOptions>,
    #[serde(rename = "OffPeakWindowOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_peak_window_options: Option<OffPeakWindowOptions>,
    #[serde(rename = "SnapshotOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_options: Option<SnapshotOptions>,
    #[serde(rename = "SoftwareUpdateOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software_update_options: Option<SoftwareUpdateOptions>,
    #[serde(rename = "VPCOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v_p_c_options: Option<VPCOptions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDomainConfigResponse {
    #[serde(rename = "DomainConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_config: Option<DomainConfig>,
    #[serde(rename = "DryRunProgressStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_progress_status: Option<DryRunProgressStatus>,
    #[serde(rename = "DryRunResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run_results: Option<DryRunResults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexRequest {
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "IndexName")]
    #[serde(default)]
    pub index_name: String,
    #[serde(rename = "IndexSchema")]
    #[serde(default)]
    pub index_schema: serde_json::Value,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateIndexResponse {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageRequest {
    #[serde(rename = "CommitMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit_message: Option<String>,
    #[serde(rename = "PackageConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_configuration: Option<PackageConfiguration>,
    #[serde(rename = "PackageDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_description: Option<String>,
    #[serde(rename = "PackageEncryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_encryption_options: Option<PackageEncryptionOptions>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
    #[serde(rename = "PackageSource")]
    #[serde(default)]
    pub package_source: PackageSource,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageResponse {
    #[serde(rename = "PackageDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_details: Option<PackageDetails>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageScopeRequest {
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: String,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    pub package_i_d: String,
    #[serde(rename = "PackageUserList")]
    #[serde(default)]
    pub package_user_list: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePackageScopeResponse {
    #[serde(rename = "Operation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "PackageID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_i_d: Option<String>,
    #[serde(rename = "PackageUserList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_user_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledActionRequest {
    #[serde(rename = "ActionID")]
    #[serde(default)]
    pub action_i_d: String,
    #[serde(rename = "ActionType")]
    #[serde(default)]
    pub action_type: String,
    #[serde(rename = "DesiredStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_start_time: Option<i64>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "ScheduleAt")]
    #[serde(default)]
    pub schedule_at: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateScheduledActionResponse {
    #[serde(rename = "ScheduledAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action: Option<ScheduledAction>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcEndpointRequest {
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    pub vpc_endpoint_id: String,
    #[serde(rename = "VpcOptions")]
    #[serde(default)]
    pub vpc_options: VPCOptions,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVpcEndpointResponse {
    #[serde(rename = "VpcEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<VpcEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeDomainRequest {
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "PerformCheckOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_check_only: Option<bool>,
    #[serde(rename = "TargetVersion")]
    #[serde(default)]
    pub target_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpgradeDomainResponse {
    #[serde(rename = "AdvancedOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ChangeProgressDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub change_progress_details: Option<ChangeProgressDetails>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "PerformCheckOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perform_check_only: Option<bool>,
    #[serde(rename = "TargetVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_version: Option<String>,
    #[serde(rename = "UpgradeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upgrade_id: Option<String>,
}
