//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-glacier

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AbortMultipartUploadInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AbortVaultLockInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AddTagsToVaultInput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArchiveCreationOutput {
    #[serde(rename = "archiveId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteMultipartUploadInput {
    #[serde(rename = "archiveSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CompleteVaultLockInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVaultInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVaultOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteArchiveInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVaultAccessPolicyInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVaultInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVaultNotificationsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVaultInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVaultOutput {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "LastInventoryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_inventory_date: Option<String>,
    #[serde(rename = "NumberOfArchives")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_archives: Option<i64>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "VaultARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_a_r_n: Option<String>,
    #[serde(rename = "VaultName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataRetrievalPolicyInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetDataRetrievalPolicyOutput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<DataRetrievalPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRetrievalPolicy {
    #[serde(rename = "Rules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<DataRetrievalRule>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRetrievalRule {
    #[serde(rename = "BytesPerHour")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_per_hour: Option<i64>,
    #[serde(rename = "Strategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobOutputInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobOutputOutput {
    #[serde(rename = "acceptRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_ranges: Option<String>,
    #[serde(rename = "archiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(rename = "contentRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_range: Option<String>,
    #[serde(rename = "contentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultAccessPolicyInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultAccessPolicyOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultAccessPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultAccessPolicy {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultLockInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultLockOutput {
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultNotificationsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetVaultNotificationsOutput {
    #[serde(rename = "vaultNotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_notification_config: Option<VaultNotificationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultNotificationConfig {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
    #[serde(rename = "SNSTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_topic: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlacierJobDescription {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "ArchiveId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    #[serde(rename = "ArchiveSHA256TreeHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_s_h_a256_tree_hash: Option<String>,
    #[serde(rename = "ArchiveSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_size_in_bytes: Option<i64>,
    #[serde(rename = "Completed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(rename = "CompletionDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_date: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "InventoryRetrievalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_retrieval_parameters: Option<InventoryRetrievalJobDescription>,
    #[serde(rename = "InventorySizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_size_in_bytes: Option<i64>,
    #[serde(rename = "JobDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_description: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobOutputPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output_path: Option<String>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    #[serde(rename = "RetrievalByteRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_byte_range: Option<String>,
    #[serde(rename = "SHA256TreeHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_h_a256_tree_hash: Option<String>,
    #[serde(rename = "SNSTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_topic: Option<String>,
    #[serde(rename = "SelectParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_parameters: Option<SelectParameters>,
    #[serde(rename = "StatusCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "VaultARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryRetrievalJobDescription {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputLocation {
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3Location>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(rename = "AccessControlList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control_list: Option<Vec<Grant>>,
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "CannedACL")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canned_a_c_l: Option<String>,
    #[serde(rename = "Encryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption: Option<Encryption>,
    #[serde(rename = "Prefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "StorageClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_class: Option<String>,
    #[serde(rename = "Tagging")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagging: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "UserMetadata")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Grant {
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<Grantee>,
    #[serde(rename = "Permission")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Grantee {
    #[serde(rename = "DisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "EmailAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
    #[serde(rename = "ID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_d: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "URI")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_r_i: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Encryption {
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "KMSContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_context: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelectParameters {
    #[serde(rename = "Expression")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[serde(rename = "ExpressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression_type: Option<String>,
    #[serde(rename = "InputSerialization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_serialization: Option<InputSerialization>,
    #[serde(rename = "OutputSerialization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_serialization: Option<OutputSerialization>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InputSerialization {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CSVInput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CSVInput {
    #[serde(rename = "Comments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "FieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "FileHeaderInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_header_info: Option<String>,
    #[serde(rename = "QuoteCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    #[serde(rename = "RecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutputSerialization {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csv: Option<CSVOutput>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CSVOutput {
    #[serde(rename = "FieldDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_delimiter: Option<String>,
    #[serde(rename = "QuoteCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_character: Option<String>,
    #[serde(rename = "QuoteEscapeCharacter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_escape_character: Option<String>,
    #[serde(rename = "QuoteFields")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_fields: Option<String>,
    #[serde(rename = "RecordDelimiter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_delimiter: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateJobInput {
    #[serde(rename = "jobParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<JobParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobParameters {
    #[serde(rename = "ArchiveId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_id: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "InventoryRetrievalParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inventory_retrieval_parameters: Option<InventoryRetrievalJobInput>,
    #[serde(rename = "OutputLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_location: Option<OutputLocation>,
    #[serde(rename = "RetrievalByteRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_byte_range: Option<String>,
    #[serde(rename = "SNSTopic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_n_s_topic: Option<String>,
    #[serde(rename = "SelectParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub select_parameters: Option<SelectParameters>,
    #[serde(rename = "Tier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InventoryRetrievalJobInput {
    #[serde(rename = "EndDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "Limit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateJobOutput {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobOutputPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_output_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateMultipartUploadInput {
    #[serde(rename = "archiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    #[serde(rename = "partSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateMultipartUploadOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "uploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateVaultLockInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultLockPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VaultLockPolicy {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InitiateVaultLockOutput {
    #[serde(rename = "lockId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lock_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsOutput {
    #[serde(rename = "JobList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_list: Option<Vec<GlacierJobDescription>>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultipartUploadsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListMultipartUploadsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "UploadsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploads_list: Option<Vec<UploadListElement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadListElement {
    #[serde(rename = "ArchiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "MultipartUploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    #[serde(rename = "PartSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size_in_bytes: Option<i64>,
    #[serde(rename = "VaultARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPartsOutput {
    #[serde(rename = "ArchiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    #[serde(rename = "CreationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MultipartUploadId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipart_upload_id: Option<String>,
    #[serde(rename = "PartSizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_size_in_bytes: Option<i64>,
    #[serde(rename = "Parts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parts: Option<Vec<PartListElement>>,
    #[serde(rename = "VaultARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartListElement {
    #[serde(rename = "RangeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range_in_bytes: Option<String>,
    #[serde(rename = "SHA256TreeHash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s_h_a256_tree_hash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedCapacityInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListProvisionedCapacityOutput {
    #[serde(rename = "ProvisionedCapacityList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_capacity_list: Option<Vec<ProvisionedCapacityDescription>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionedCapacityDescription {
    #[serde(rename = "CapacityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_id: Option<String>,
    #[serde(rename = "ExpirationDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "StartDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForVaultInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForVaultOutput {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVaultsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListVaultsOutput {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "VaultList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_list: Option<Vec<DescribeVaultOutput>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseProvisionedCapacityInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PurchaseProvisionedCapacityOutput {
    #[serde(rename = "capacityId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveTagsFromVaultInput {
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetDataRetrievalPolicyInput {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<DataRetrievalPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetVaultAccessPolicyInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<VaultAccessPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SetVaultNotificationsInput {
    #[serde(rename = "vaultNotificationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vault_notification_config: Option<VaultNotificationConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadArchiveInput {
    #[serde(rename = "archiveDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadMultipartPartInput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UploadMultipartPartOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
}
