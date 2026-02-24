//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-secretsmanager

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSecretValueRequest {
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
    #[serde(rename = "SecretIdList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Filter {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Values")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchGetSecretValueResponse {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<APIErrorType>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecretValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_values: Option<Vec<SecretValueEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct APIErrorType {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretValueEntry {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecretBinary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "SecretString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelRotateSecretRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelRotateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSecretRequest {
    #[serde(rename = "AddReplicaRegions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_replica_regions: Option<Vec<ReplicaRegionType>>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForceOverwriteReplicaSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_overwrite_replica_secret: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SecretBinary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "SecretString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicaRegionType {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tag {
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
pub struct CreateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<Vec<ReplicationStatusType>>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicationStatusType {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecretRequest {
    #[serde(rename = "ForceDeleteWithoutRecovery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_delete_without_recovery: Option<bool>,
    #[serde(rename = "RecoveryWindowInDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_window_in_days: Option<i64>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "DeletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecretRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DeletedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExternalSecretRotationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_metadata: Option<Vec<ExternalSecretRotationMetadataItem>>,
    #[serde(rename = "ExternalSecretRotationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_role_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    #[serde(rename = "LastChangedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<f64>,
    #[serde(rename = "LastRotatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextRotationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<f64>,
    #[serde(rename = "OwningService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_service: Option<String>,
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<String>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<Vec<ReplicationStatusType>>,
    #[serde(rename = "RotationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_a_r_n: Option<String>,
    #[serde(rename = "RotationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "VersionIdsToStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_ids_to_stages: Option<std::collections::HashMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExternalSecretRotationMetadataItem {
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
pub struct RotationRulesType {
    #[serde(rename = "AutomaticallyAfterDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatically_after_days: Option<i64>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    #[serde(rename = "ScheduleExpression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_expression: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRandomPasswordRequest {
    #[serde(rename = "ExcludeCharacters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_characters: Option<String>,
    #[serde(rename = "ExcludeLowercase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_lowercase: Option<bool>,
    #[serde(rename = "ExcludeNumbers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_numbers: Option<bool>,
    #[serde(rename = "ExcludePunctuation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_punctuation: Option<bool>,
    #[serde(rename = "ExcludeUppercase")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_uppercase: Option<bool>,
    #[serde(rename = "IncludeSpace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_space: Option<bool>,
    #[serde(rename = "PasswordLength")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_length: Option<i64>,
    #[serde(rename = "RequireEachIncludedType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_each_included_type: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetRandomPasswordResponse {
    #[serde(rename = "RandomPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub random_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecretValueRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stage: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetSecretValueResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SecretBinary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "SecretString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecretVersionIdsRequest {
    #[serde(rename = "IncludeDeprecated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_deprecated: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecretVersionIdsResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Versions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<SecretVersionsListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretVersionsListEntry {
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "KmsKeyIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_ids: Option<Vec<String>>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecretsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<Filter>>,
    #[serde(rename = "IncludePlannedDeletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_planned_deletion: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SortBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSecretsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SecretList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_list: Option<Vec<SecretListEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecretListEntry {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<f64>,
    #[serde(rename = "DeletedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_date: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ExternalSecretRotationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_metadata: Option<Vec<ExternalSecretRotationMetadataItem>>,
    #[serde(rename = "ExternalSecretRotationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_role_arn: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LastAccessedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_date: Option<f64>,
    #[serde(rename = "LastChangedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_changed_date: Option<f64>,
    #[serde(rename = "LastRotatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_rotated_date: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NextRotationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_rotation_date: Option<f64>,
    #[serde(rename = "OwningService")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owning_service: Option<String>,
    #[serde(rename = "PrimaryRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_region: Option<String>,
    #[serde(rename = "RotationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_enabled: Option<bool>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_a_r_n: Option<String>,
    #[serde(rename = "RotationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "SecretVersionsToStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_versions_to_stages: Option<std::collections::HashMap<String, Vec<String>>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyRequest {
    #[serde(rename = "BlockPublicPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_public_policy: Option<bool>,
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutResourcePolicyResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSecretValueRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "RotationToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_token: Option<String>,
    #[serde(rename = "SecretBinary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "SecretString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PutSecretValueResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
    #[serde(rename = "VersionStages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_stages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionsFromReplicationRequest {
    #[serde(rename = "RemoveReplicaRegions")]
    #[serde(default)]
    pub remove_replica_regions: Vec<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveRegionsFromReplicationResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<Vec<ReplicationStatusType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateSecretToRegionsRequest {
    #[serde(rename = "AddReplicaRegions")]
    #[serde(default)]
    pub add_replica_regions: Vec<ReplicaRegionType>,
    #[serde(rename = "ForceOverwriteReplicaSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_overwrite_replica_secret: Option<bool>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReplicateSecretToRegionsResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "ReplicationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_status: Option<Vec<ReplicationStatusType>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreSecretRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateSecretRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ExternalSecretRotationMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_metadata: Option<Vec<ExternalSecretRotationMetadataItem>>,
    #[serde(rename = "ExternalSecretRotationRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_secret_rotation_role_arn: Option<String>,
    #[serde(rename = "RotateImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotate_immediately: Option<bool>,
    #[serde(rename = "RotationLambdaARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_lambda_a_r_n: Option<String>,
    #[serde(rename = "RotationRules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation_rules: Option<RotationRulesType>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RotateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationToReplicaRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StopReplicationToReplicaResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: Vec<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecretRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SecretBinary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_binary: Option<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "SecretString")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_string: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecretResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "VersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecretVersionStageRequest {
    #[serde(rename = "MoveToVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub move_to_version_id: Option<String>,
    #[serde(rename = "RemoveFromVersionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_from_version_id: Option<String>,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    pub secret_id: String,
    #[serde(rename = "VersionStage")]
    #[serde(default)]
    pub version_stage: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSecretVersionStageResponse {
    #[serde(rename = "ARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a_r_n: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateResourcePolicyRequest {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    pub resource_policy: String,
    #[serde(rename = "SecretId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidateResourcePolicyResponse {
    #[serde(rename = "PolicyValidationPassed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_validation_passed: Option<bool>,
    #[serde(rename = "ValidationErrors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationErrorsEntry>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidationErrorsEntry {
    #[serde(rename = "CheckName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_name: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
