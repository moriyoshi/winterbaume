//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-mq

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrokerRequest {
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "autoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "brokerName")]
    #[serde(default)]
    pub broker_name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    #[serde(rename = "creatorRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_request_id: Option<String>,
    #[serde(rename = "dataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_mode: Option<String>,
    #[serde(rename = "dataReplicationPrimaryBrokerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_primary_broker_arn: Option<String>,
    #[serde(rename = "deploymentMode")]
    #[serde(default)]
    pub deployment_mode: String,
    #[serde(rename = "encryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<EncryptionOptions>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    pub engine_type: String,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    pub host_instance_type: String,
    #[serde(rename = "ldapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_server_metadata: Option<LdapServerMetadataInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    #[serde(rename = "maintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    pub publicly_accessible: bool,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "storageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<User>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationId {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EncryptionOptions {
    #[serde(rename = "kmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "useAwsOwnedKey")]
    #[serde(default)]
    pub use_aws_owned_key: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LdapServerMetadataInput {
    #[serde(default)]
    pub hosts: Vec<String>,
    #[serde(rename = "roleBase")]
    #[serde(default)]
    pub role_base: String,
    #[serde(rename = "roleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "roleSearchMatching")]
    #[serde(default)]
    pub role_search_matching: String,
    #[serde(rename = "roleSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_search_subtree: Option<bool>,
    #[serde(rename = "serviceAccountPassword")]
    #[serde(default)]
    pub service_account_password: String,
    #[serde(rename = "serviceAccountUsername")]
    #[serde(default)]
    pub service_account_username: String,
    #[serde(rename = "userBase")]
    #[serde(default)]
    pub user_base: String,
    #[serde(rename = "userRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role_name: Option<String>,
    #[serde(rename = "userSearchMatching")]
    #[serde(default)]
    pub user_search_matching: String,
    #[serde(rename = "userSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_search_subtree: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Logs {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WeeklyStartTime {
    #[serde(rename = "dayOfWeek")]
    #[serde(default)]
    pub day_of_week: String,
    #[serde(rename = "timeOfDay")]
    #[serde(default)]
    pub time_of_day: String,
    #[serde(rename = "timeZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "consoleAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "replicationUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_user: Option<bool>,
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBrokerResponse {
    #[serde(rename = "brokerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationRequest {
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    pub engine_type: String,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfigurationRevision {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(rename = "consoleAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "replicationUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_user: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrokerRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBrokerResponse {
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationRequest {
    #[serde(rename = "ConfigurationId")]
    #[serde(default)]
    pub configuration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConfigurationResponse {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteUserResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrokerEngineTypesRequest {
    #[serde(rename = "EngineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
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
pub struct DescribeBrokerEngineTypesResponse {
    #[serde(rename = "brokerEngineTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_engine_types: Option<Vec<BrokerEngineType>>,
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
pub struct BrokerEngineType {
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "engineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_versions: Option<Vec<EngineVersion>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EngineVersion {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrokerInstanceOptionsRequest {
    #[serde(rename = "EngineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "HostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrokerInstanceOptionsResponse {
    #[serde(rename = "brokerInstanceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_instance_options: Option<Vec<BrokerInstanceOption>>,
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
pub struct BrokerInstanceOption {
    #[serde(rename = "availabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<Vec<AvailabilityZone>>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "storageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "supportedDeploymentModes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_deployment_modes: Option<Vec<String>>,
    #[serde(rename = "supportedEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_engine_versions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZone {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrokerRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBrokerResponse {
    #[serde(rename = "actionsRequired")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions_required: Option<Vec<ActionRequired>>,
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "autoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "brokerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(rename = "brokerInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_instances: Option<Vec<BrokerInstance>>,
    #[serde(rename = "brokerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    #[serde(rename = "brokerState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Configurations>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "dataReplicationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_metadata: Option<DataReplicationMetadataOutput>,
    #[serde(rename = "dataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_mode: Option<String>,
    #[serde(rename = "deploymentMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    #[serde(rename = "encryptionOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_options: Option<EncryptionOptions>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "ldapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_server_metadata: Option<LdapServerMetadataOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<LogsSummary>,
    #[serde(rename = "maintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    #[serde(rename = "pendingAuthenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_authentication_strategy: Option<String>,
    #[serde(rename = "pendingDataReplicationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_data_replication_metadata: Option<DataReplicationMetadataOutput>,
    #[serde(rename = "pendingDataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_data_replication_mode: Option<String>,
    #[serde(rename = "pendingEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_engine_version: Option<String>,
    #[serde(rename = "pendingHostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_host_instance_type: Option<String>,
    #[serde(rename = "pendingLdapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_ldap_server_metadata: Option<LdapServerMetadataOutput>,
    #[serde(rename = "pendingSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_security_groups: Option<Vec<String>>,
    #[serde(rename = "publiclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
    #[serde(rename = "storageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "subnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActionRequired {
    #[serde(rename = "actionRequiredCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_required_code: Option<String>,
    #[serde(rename = "actionRequiredInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_required_info: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerInstance {
    #[serde(rename = "consoleURL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_u_r_l: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<String>>,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Configurations {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<ConfigurationId>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<ConfigurationId>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<ConfigurationId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataReplicationMetadataOutput {
    #[serde(rename = "dataReplicationCounterpart")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_counterpart: Option<DataReplicationCounterpart>,
    #[serde(rename = "dataReplicationRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_role: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataReplicationCounterpart {
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LdapServerMetadataOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(rename = "roleBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_base: Option<String>,
    #[serde(rename = "roleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    #[serde(rename = "roleSearchMatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_search_matching: Option<String>,
    #[serde(rename = "roleSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_search_subtree: Option<bool>,
    #[serde(rename = "serviceAccountUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_username: Option<String>,
    #[serde(rename = "userBase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_base: Option<String>,
    #[serde(rename = "userRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_role_name: Option<String>,
    #[serde(rename = "userSearchMatching")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_search_matching: Option<String>,
    #[serde(rename = "userSearchSubtree")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_search_subtree: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogsSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(rename = "auditLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_group: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
    #[serde(rename = "generalLogGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general_log_group: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<PendingLogs>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingLogs {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserSummary {
    #[serde(rename = "pendingChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_change: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRequest {
    #[serde(rename = "ConfigurationId")]
    #[serde(default)]
    pub configuration_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRevisionRequest {
    #[serde(rename = "ConfigurationId")]
    #[serde(default)]
    pub configuration_id: String,
    #[serde(rename = "ConfigurationRevision")]
    #[serde(default)]
    pub configuration_revision: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConfigurationRevisionResponse {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeUserResponse {
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(rename = "consoleAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<UserPendingChanges>,
    #[serde(rename = "replicationUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_user: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UserPendingChanges {
    #[serde(rename = "consoleAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "pendingChange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_change: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListBrokersRequest {
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
pub struct ListBrokersResponse {
    #[serde(rename = "brokerSummaries")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_summaries: Option<Vec<BrokerSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BrokerSummary {
    #[serde(rename = "brokerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_arn: Option<String>,
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(rename = "brokerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_name: Option<String>,
    #[serde(rename = "brokerState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "deploymentMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_mode: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationRevisionsRequest {
    #[serde(rename = "ConfigurationId")]
    #[serde(default)]
    pub configuration_id: String,
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
pub struct ListConfigurationRevisionsResponse {
    #[serde(rename = "configurationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_id: Option<String>,
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
    pub revisions: Option<Vec<ConfigurationRevision>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConfigurationsRequest {
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
pub struct ListConfigurationsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<Vec<Configuration>>,
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
pub struct Configuration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "engineType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_type: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsRequest {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListUsersRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
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
pub struct ListUsersResponse {
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
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
    pub users: Option<Vec<UserSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromoteRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PromoteResponse {
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootBrokerRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RebootBrokerResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerRequest {
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "autoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    #[serde(rename = "dataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_mode: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "ldapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_server_metadata: Option<LdapServerMetadataInput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    #[serde(rename = "maintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateBrokerResponse {
    #[serde(rename = "authenticationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_strategy: Option<String>,
    #[serde(rename = "autoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "brokerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broker_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<ConfigurationId>,
    #[serde(rename = "dataReplicationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_metadata: Option<DataReplicationMetadataOutput>,
    #[serde(rename = "dataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_replication_mode: Option<String>,
    #[serde(rename = "engineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "hostInstanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_instance_type: Option<String>,
    #[serde(rename = "ldapServerMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ldap_server_metadata: Option<LdapServerMetadataOutput>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logs: Option<Logs>,
    #[serde(rename = "maintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_window_start_time: Option<WeeklyStartTime>,
    #[serde(rename = "pendingDataReplicationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_data_replication_metadata: Option<DataReplicationMetadataOutput>,
    #[serde(rename = "pendingDataReplicationMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_data_replication_mode: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationRequest {
    #[serde(rename = "ConfigurationId")]
    #[serde(default)]
    pub configuration_id: String,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConfigurationResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "latestRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<ConfigurationRevision>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<SanitizationWarning>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SanitizationWarning {
    #[serde(rename = "attributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "elementName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    #[serde(rename = "BrokerId")]
    #[serde(default)]
    pub broker_id: String,
    #[serde(rename = "consoleAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_access: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "replicationUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_user: Option<bool>,
    #[serde(rename = "Username")]
    #[serde(default)]
    pub username: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateUserResponse {}
