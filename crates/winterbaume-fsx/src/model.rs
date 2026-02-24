//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-fsx

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFileSystemAliasesRequest {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociateFileSystemAliasesResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Alias {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataRepositoryTaskRequest {
    #[serde(rename = "TaskId")]
    #[serde(default)]
    pub task_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelDataRepositoryTaskResponse {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopyBackupRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SourceBackupId")]
    #[serde(default)]
    pub source_backup_id: String,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
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
pub struct CopyBackupResponse {
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Backup {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DirectoryInformation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory_information: Option<ActiveDirectoryBackupAttributes>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<BackupFailureDetails>,
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i32>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "SourceBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_id: Option<String>,
    #[serde(rename = "SourceBackupRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_backup_region: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Volume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ActiveDirectoryBackupAttributes {
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BackupFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystem {
    #[serde(rename = "AdministrativeActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FileSystemFailureDetails>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "FileSystemType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type: Option<String>,
    #[serde(rename = "FileSystemTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<LustreFileSystemConfiguration>,
    #[serde(rename = "NetworkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<OntapFileSystemConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<OpenZFSFileSystemConfiguration>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<WindowsFileSystemConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdministrativeAction {
    #[serde(rename = "AdministrativeActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_action_type: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<AdministrativeActionFailureDetails>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<i32>,
    #[serde(rename = "RemainingTransferBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_transfer_bytes: Option<i64>,
    #[serde(rename = "RequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetFileSystemValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_file_system_values: Option<Box<FileSystem>>,
    #[serde(rename = "TargetSnapshotValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_snapshot_values: Option<Box<Snapshot>>,
    #[serde(rename = "TargetVolumeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_volume_values: Option<Box<Volume>>,
    #[serde(rename = "TotalTransferBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_transfer_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AdministrativeActionFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Snapshot {
    #[serde(rename = "AdministrativeActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LifecycleTransitionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition_reason: Option<LifecycleTransitionReason>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LifecycleTransitionReason {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Volume {
    #[serde(rename = "AdministrativeActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LifecycleTransitionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition_reason: Option<LifecycleTransitionReason>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<OntapVolumeConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<OpenZFSVolumeConfiguration>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OntapVolumeConfiguration {
    #[serde(rename = "AggregateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_configuration: Option<AggregateConfiguration>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "FlexCacheEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flex_cache_endpoint_type: Option<String>,
    #[serde(rename = "JunctionPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub junction_path: Option<String>,
    #[serde(rename = "OntapVolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_volume_type: Option<String>,
    #[serde(rename = "SecurityStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_style: Option<String>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "SizeInMegabytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_megabytes: Option<i32>,
    #[serde(rename = "SnaplockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaplock_configuration: Option<SnaplockConfiguration>,
    #[serde(rename = "SnapshotPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_policy: Option<String>,
    #[serde(rename = "StorageEfficiencyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_efficiency_enabled: Option<bool>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_id: Option<String>,
    #[serde(rename = "StorageVirtualMachineRoot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_root: Option<bool>,
    #[serde(rename = "TieringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_policy: Option<TieringPolicy>,
    #[serde(rename = "UUID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_u_i_d: Option<String>,
    #[serde(rename = "VolumeStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_style: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AggregateConfiguration {
    #[serde(rename = "Aggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<Vec<String>>,
    #[serde(rename = "TotalConstituents")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_constituents: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnaplockConfiguration {
    #[serde(rename = "AuditLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_volume: Option<bool>,
    #[serde(rename = "AutocommitPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocommit_period: Option<AutocommitPeriod>,
    #[serde(rename = "PrivilegedDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_delete: Option<String>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<SnaplockRetentionPeriod>,
    #[serde(rename = "SnaplockType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaplock_type: Option<String>,
    #[serde(rename = "VolumeAppendModeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_append_mode_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutocommitPeriod {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnaplockRetentionPeriod {
    #[serde(rename = "DefaultRetention")]
    #[serde(default)]
    pub default_retention: RetentionPeriod,
    #[serde(rename = "MaximumRetention")]
    #[serde(default)]
    pub maximum_retention: RetentionPeriod,
    #[serde(rename = "MinimumRetention")]
    #[serde(default)]
    pub minimum_retention: RetentionPeriod,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetentionPeriod {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TieringPolicy {
    #[serde(rename = "CoolingPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cooling_period: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSVolumeConfiguration {
    #[serde(rename = "CopyStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_strategy: Option<String>,
    #[serde(rename = "CopyTagsToSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshots: Option<bool>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "DeleteClonedVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_cloned_volumes: Option<bool>,
    #[serde(rename = "DeleteIntermediateData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_intermediate_data: Option<bool>,
    #[serde(rename = "DeleteIntermediateSnaphots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_intermediate_snaphots: Option<bool>,
    #[serde(rename = "DestinationSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_snapshot: Option<String>,
    #[serde(rename = "NfsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_exports: Option<Vec<OpenZFSNfsExport>>,
    #[serde(rename = "OriginSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_snapshot: Option<OpenZFSOriginSnapshotConfiguration>,
    #[serde(rename = "ParentVolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_volume_id: Option<String>,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "RecordSizeKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_size_ki_b: Option<i32>,
    #[serde(rename = "RestoreToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_to_snapshot: Option<String>,
    #[serde(rename = "SourceSnapshotARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_a_r_n: Option<String>,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_quota_gi_b: Option<i32>,
    #[serde(rename = "StorageCapacityReservationGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_reservation_gi_b: Option<i32>,
    #[serde(rename = "UserAndGroupQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_and_group_quotas: Option<Vec<OpenZFSUserOrGroupQuota>>,
    #[serde(rename = "VolumePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSNfsExport {
    #[serde(rename = "ClientConfigurations")]
    #[serde(default)]
    pub client_configurations: Vec<OpenZFSClientConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSClientConfiguration {
    #[serde(rename = "Clients")]
    #[serde(default)]
    pub clients: String,
    #[serde(rename = "Options")]
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSOriginSnapshotConfiguration {
    #[serde(rename = "CopyStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_strategy: Option<String>,
    #[serde(rename = "SnapshotARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_a_r_n: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSUserOrGroupQuota {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i32,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    #[serde(default)]
    pub storage_capacity_quota_gi_b: i32,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LustreFileSystemConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "DataReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_read_cache_configuration: Option<LustreReadCacheConfiguration>,
    #[serde(rename = "DataRepositoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_configuration: Option<DataRepositoryConfiguration>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DriveCacheType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_cache_type: Option<String>,
    #[serde(rename = "EfaEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efa_enabled: Option<bool>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LustreLogConfiguration>,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<FileSystemLustreMetadataConfiguration>,
    #[serde(rename = "MountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_name: Option<String>,
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i32>,
    #[serde(rename = "RootSquashConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_squash_configuration: Option<LustreRootSquashConfiguration>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LustreReadCacheConfiguration {
    #[serde(rename = "SizeGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gi_b: Option<i32>,
    #[serde(rename = "SizingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizing_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryConfiguration {
    #[serde(rename = "AutoImportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "ExportPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<DataRepositoryFailureDetails>,
    #[serde(rename = "ImportPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i32>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LustreLogConfiguration {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Level")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemLustreMetadataConfiguration {
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LustreRootSquashConfiguration {
    #[serde(rename = "NoSquashNids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_squash_nids: Option<Vec<String>>,
    #[serde(rename = "RootSquash")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_squash: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OntapFileSystemConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpAddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip_address_range: Option<String>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<FileSystemEndpoints>,
    #[serde(rename = "FsxAdminPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_admin_password: Option<String>,
    #[serde(rename = "HAPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_a_pairs: Option<i32>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "RouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "ThroughputCapacityPerHAPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity_per_h_a_pair: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DiskIopsConfiguration {
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i64>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemEndpoints {
    #[serde(rename = "Intercluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercluster: Option<FileSystemEndpoint>,
    #[serde(rename = "Management")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<FileSystemEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileSystemEndpoint {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "IpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Ipv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSFileSystemConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "CopyTagsToVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_volumes: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip_address: Option<String>,
    #[serde(rename = "EndpointIpAddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip_address_range: Option<String>,
    #[serde(rename = "EndpointIpv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address: Option<String>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "ReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_cache_configuration: Option<OpenZFSReadCacheConfiguration>,
    #[serde(rename = "RootVolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_id: Option<String>,
    #[serde(rename = "RouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSReadCacheConfiguration {
    #[serde(rename = "SizeGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_gi_b: Option<i32>,
    #[serde(rename = "SizingMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizing_mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowsFileSystemConfiguration {
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    #[serde(rename = "AuditLogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_configuration: Option<WindowsAuditLogConfiguration>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "FsrmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsrm_configuration: Option<WindowsFsrmConfiguration>,
    #[serde(rename = "MaintenanceOperationsInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_operations_in_progress: Option<Vec<String>>,
    #[serde(rename = "PreferredFileServerIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_file_server_ip: Option<String>,
    #[serde(rename = "PreferredFileServerIpv6")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_file_server_ipv6: Option<String>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "RemoteAdministrationEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_administration_endpoint: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryAttributes>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowsAuditLogConfiguration {
    #[serde(rename = "AuditLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_destination: Option<String>,
    #[serde(rename = "FileAccessAuditLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_access_audit_log_level: Option<String>,
    #[serde(rename = "FileShareAccessAuditLogLevel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_share_access_audit_log_level: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowsFsrmConfiguration {
    #[serde(rename = "EventLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_log_destination: Option<String>,
    #[serde(rename = "FsrmServiceEnabled")]
    #[serde(default)]
    pub fsrm_service_enabled: bool,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedActiveDirectoryAttributes {
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "DomainJoinServiceAccountSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_service_account_secret: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopySnapshotAndUpdateVolumeRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CopyStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_strategy: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "SourceSnapshotARN")]
    #[serde(default)]
    pub source_snapshot_a_r_n: String,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CopySnapshotAndUpdateVolumeResponse {
    #[serde(rename = "AdministrativeActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAndAttachS3AccessPointRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<CreateAndAttachS3AccessPointOntapConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<CreateAndAttachS3AccessPointOpenZFSConfiguration>,
    #[serde(rename = "S3AccessPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_point: Option<CreateAndAttachS3AccessPointS3Configuration>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAndAttachS3AccessPointOntapConfiguration {
    #[serde(rename = "FileSystemIdentity")]
    #[serde(default)]
    pub file_system_identity: OntapFileSystemIdentity,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OntapFileSystemIdentity {
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "UnixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unix_user: Option<OntapUnixFileSystemUser>,
    #[serde(rename = "WindowsUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_user: Option<OntapWindowsFileSystemUser>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OntapUnixFileSystemUser {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OntapWindowsFileSystemUser {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAndAttachS3AccessPointOpenZFSConfiguration {
    #[serde(rename = "FileSystemIdentity")]
    #[serde(default)]
    pub file_system_identity: OpenZFSFileSystemIdentity,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSFileSystemIdentity {
    #[serde(rename = "PosixUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posix_user: Option<OpenZFSPosixFileSystemUser>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSPosixFileSystemUser {
    #[serde(rename = "Gid")]
    #[serde(default)]
    pub gid: i64,
    #[serde(rename = "SecondaryGids")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_gids: Option<Vec<i64>>,
    #[serde(rename = "Uid")]
    #[serde(default)]
    pub uid: i64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAndAttachS3AccessPointS3Configuration {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<S3AccessPointVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointVpcConfiguration {
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAndAttachS3AccessPointResponse {
    #[serde(rename = "S3AccessPointAttachment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_point_attachment: Option<S3AccessPointAttachment>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointAttachment {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LifecycleTransitionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition_reason: Option<LifecycleTransitionReason>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<S3AccessPointOntapConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<S3AccessPointOpenZFSConfiguration>,
    #[serde(rename = "S3AccessPoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_point: Option<S3AccessPoint>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointOntapConfiguration {
    #[serde(rename = "FileSystemIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_identity: Option<OntapFileSystemIdentity>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointOpenZFSConfiguration {
    #[serde(rename = "FileSystemIdentity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_identity: Option<OpenZFSFileSystemIdentity>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPoint {
    #[serde(rename = "Alias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "VpcConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_configuration: Option<S3AccessPointVpcConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateBackupResponse {
    #[serde(rename = "Backup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup: Option<Backup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataRepositoryAssociationRequest {
    #[serde(rename = "BatchImportMetaDataOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_import_meta_data_on_create: Option<bool>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DataRepositoryPath")]
    #[serde(default)]
    pub data_repository_path: String,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "FileSystemPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i32>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3DataRepositoryConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3DataRepositoryConfiguration {
    #[serde(rename = "AutoExportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_export_policy: Option<AutoExportPolicy>,
    #[serde(rename = "AutoImportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<AutoImportPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoExportPolicy {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AutoImportPolicy {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataRepositoryAssociationResponse {
    #[serde(rename = "Association")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<DataRepositoryAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryAssociation {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "BatchImportMetaDataOnCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_import_meta_data_on_create: Option<bool>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DataRepositoryPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_path: Option<String>,
    #[serde(rename = "DataRepositorySubdirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_subdirectories: Option<Vec<String>>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<DataRepositoryFailureDetails>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_id: Option<String>,
    #[serde(rename = "FileCachePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_path: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "FileSystemPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_path: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i32>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "NFS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_f_s: Option<NFSDataRepositoryConfiguration>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3DataRepositoryConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NFSDataRepositoryConfiguration {
    #[serde(rename = "AutoExportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_export_policy: Option<AutoExportPolicy>,
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataRepositoryTaskRequest {
    #[serde(rename = "CapacityToRelease")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_to_release: Option<i64>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "Paths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(rename = "ReleaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_configuration: Option<ReleaseConfiguration>,
    #[serde(rename = "Report")]
    #[serde(default)]
    pub report: CompletionReport,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReleaseConfiguration {
    #[serde(rename = "DurationSinceLastAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_since_last_access: Option<DurationSinceLastAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DurationSinceLastAccess {
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
pub struct CompletionReport {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "Format")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(rename = "Path")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Scope")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateDataRepositoryTaskResponse {
    #[serde(rename = "DataRepositoryTask")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_task: Option<DataRepositoryTask>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryTask {
    #[serde(rename = "CapacityToRelease")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_to_release: Option<i64>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<f64>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<DataRepositoryTaskFailureDetails>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_id: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "Paths")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
    #[serde(rename = "ReleaseConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_configuration: Option<ReleaseConfiguration>,
    #[serde(rename = "Report")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub report: Option<CompletionReport>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DataRepositoryTaskStatus>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "TaskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryTaskFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryTaskStatus {
    #[serde(rename = "FailedCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "ReleasedCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_capacity: Option<i64>,
    #[serde(rename = "SucceededCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_count: Option<i64>,
    #[serde(rename = "TotalCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileCacheRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "CopyTagsToDataRepositoryAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_data_repository_associations: Option<bool>,
    #[serde(rename = "DataRepositoryAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_associations: Option<Vec<FileCacheDataRepositoryAssociation>>,
    #[serde(rename = "FileCacheType")]
    #[serde(default)]
    pub file_cache_type: String,
    #[serde(rename = "FileCacheTypeVersion")]
    #[serde(default)]
    pub file_cache_type_version: String,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileCacheLustreConfiguration>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    pub storage_capacity: i32,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheDataRepositoryAssociation {
    #[serde(rename = "DataRepositoryPath")]
    #[serde(default)]
    pub data_repository_path: String,
    #[serde(rename = "DataRepositorySubdirectories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_subdirectories: Option<Vec<String>>,
    #[serde(rename = "FileCachePath")]
    #[serde(default)]
    pub file_cache_path: String,
    #[serde(rename = "NFS")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n_f_s: Option<FileCacheNFSConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheNFSConfiguration {
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "Version")]
    #[serde(default)]
    pub version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileCacheLustreConfiguration {
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    pub deployment_type: String,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    pub metadata_configuration: FileCacheLustreMetadataConfiguration,
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(default)]
    pub per_unit_storage_throughput: i32,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheLustreMetadataConfiguration {
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    pub storage_capacity: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileCacheResponse {
    #[serde(rename = "FileCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache: Option<FileCacheCreating>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheCreating {
    #[serde(rename = "CopyTagsToDataRepositoryAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_data_repository_associations: Option<bool>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "DataRepositoryAssociationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_association_ids: Option<Vec<String>>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FileCacheFailureDetails>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_id: Option<String>,
    #[serde(rename = "FileCacheType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_type: Option<String>,
    #[serde(rename = "FileCacheTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_type_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<FileCacheLustreConfiguration>,
    #[serde(rename = "NetworkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheFailureDetails {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCacheLustreConfiguration {
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LustreLogConfiguration>,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<FileCacheLustreMetadataConfiguration>,
    #[serde(rename = "MountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_name: Option<String>,
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemFromBackupRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileSystemLustreConfiguration>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<CreateFileSystemOpenZFSConfiguration>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemLustreConfiguration {
    #[serde(rename = "AutoImportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "DataReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_read_cache_configuration: Option<LustreReadCacheConfiguration>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DriveCacheType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_cache_type: Option<String>,
    #[serde(rename = "EfaEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efa_enabled: Option<bool>,
    #[serde(rename = "ExportPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_path: Option<String>,
    #[serde(rename = "ImportPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_path: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i32>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LustreLogCreateConfiguration>,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<CreateFileSystemLustreMetadataConfiguration>,
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i32>,
    #[serde(rename = "RootSquashConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_squash_configuration: Option<LustreRootSquashConfiguration>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LustreLogCreateConfiguration {
    #[serde(rename = "Destination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    #[serde(rename = "Level")]
    #[serde(default)]
    pub level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemLustreMetadataConfiguration {
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    pub mode: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemOpenZFSConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "CopyTagsToVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_volumes: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    pub deployment_type: String,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpAddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip_address_range: Option<String>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "ReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_cache_configuration: Option<OpenZFSReadCacheConfiguration>,
    #[serde(rename = "RootVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_configuration: Option<OpenZFSCreateRootVolumeConfiguration>,
    #[serde(rename = "RouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    pub throughput_capacity: i32,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OpenZFSCreateRootVolumeConfiguration {
    #[serde(rename = "CopyTagsToSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshots: Option<bool>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "NfsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_exports: Option<Vec<OpenZFSNfsExport>>,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "RecordSizeKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_size_ki_b: Option<i32>,
    #[serde(rename = "UserAndGroupQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_and_group_quotas: Option<Vec<OpenZFSUserOrGroupQuota>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemWindowsConfiguration {
    #[serde(rename = "ActiveDirectoryId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_id: Option<String>,
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(rename = "AuditLogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_configuration: Option<WindowsAuditLogCreateConfiguration>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment_type: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "FsrmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsrm_configuration: Option<WindowsFsrmConfiguration>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfiguration>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    pub throughput_capacity: i32,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct WindowsAuditLogCreateConfiguration {
    #[serde(rename = "AuditLogDestination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_destination: Option<String>,
    #[serde(rename = "FileAccessAuditLogLevel")]
    #[serde(default)]
    pub file_access_audit_log_level: String,
    #[serde(rename = "FileShareAccessAuditLogLevel")]
    #[serde(default)]
    pub file_share_access_audit_log_level: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedActiveDirectoryConfiguration {
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    pub dns_ips: Vec<String>,
    #[serde(rename = "DomainJoinServiceAccountSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_service_account_secret: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    pub domain_name: String,
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemFromBackupResponse {
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemType")]
    #[serde(default)]
    pub file_system_type: String,
    #[serde(rename = "FileSystemTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<CreateFileSystemLustreConfiguration>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<CreateFileSystemOntapConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<CreateFileSystemOpenZFSConfiguration>,
    #[serde(rename = "SecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<CreateFileSystemWindowsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemOntapConfiguration {
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DeploymentType")]
    #[serde(default)]
    pub deployment_type: String,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpAddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ip_address_range: Option<String>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "FsxAdminPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_admin_password: Option<String>,
    #[serde(rename = "HAPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_a_pairs: Option<i32>,
    #[serde(rename = "PreferredSubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_subnet_id: Option<String>,
    #[serde(rename = "RouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "ThroughputCapacityPerHAPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity_per_h_a_pair: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateFileSystemResponse {
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnapshotResponse {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStorageVirtualMachineRequest {
    #[serde(rename = "ActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_configuration: Option<CreateSvmActiveDirectoryConfiguration>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "RootVolumeSecurityStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_security_style: Option<String>,
    #[serde(rename = "SvmAdminPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svm_admin_password: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSvmActiveDirectoryConfiguration {
    #[serde(rename = "NetBiosName")]
    #[serde(default)]
    pub net_bios_name: String,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateStorageVirtualMachineResponse {
    #[serde(rename = "StorageVirtualMachine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine: Option<StorageVirtualMachine>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageVirtualMachine {
    #[serde(rename = "ActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_configuration: Option<SvmActiveDirectoryConfiguration>,
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "Endpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<SvmEndpoints>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LifecycleTransitionReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle_transition_reason: Option<LifecycleTransitionReason>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "RootVolumeSecurityStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_volume_security_style: Option<String>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_id: Option<String>,
    #[serde(rename = "Subtype")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "UUID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub u_u_i_d: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SvmActiveDirectoryConfiguration {
    #[serde(rename = "NetBiosName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_bios_name: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration: Option<SelfManagedActiveDirectoryAttributes>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SvmEndpoints {
    #[serde(rename = "Iscsi")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iscsi: Option<SvmEndpoint>,
    #[serde(rename = "Management")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub management: Option<SvmEndpoint>,
    #[serde(rename = "Nfs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs: Option<SvmEndpoint>,
    #[serde(rename = "Smb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smb: Option<SvmEndpoint>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SvmEndpoint {
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "IpAddresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_addresses: Option<Vec<String>>,
    #[serde(rename = "Ipv6Addresses")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_addresses: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVolumeFromBackupRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<CreateOntapVolumeConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOntapVolumeConfiguration {
    #[serde(rename = "AggregateConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_configuration: Option<CreateAggregateConfiguration>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "JunctionPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub junction_path: Option<String>,
    #[serde(rename = "OntapVolumeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_volume_type: Option<String>,
    #[serde(rename = "SecurityStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_style: Option<String>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "SizeInMegabytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_megabytes: Option<i32>,
    #[serde(rename = "SnaplockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaplock_configuration: Option<CreateSnaplockConfiguration>,
    #[serde(rename = "SnapshotPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_policy: Option<String>,
    #[serde(rename = "StorageEfficiencyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_efficiency_enabled: Option<bool>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    pub storage_virtual_machine_id: String,
    #[serde(rename = "TieringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_policy: Option<TieringPolicy>,
    #[serde(rename = "VolumeStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_style: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateAggregateConfiguration {
    #[serde(rename = "Aggregates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<Vec<String>>,
    #[serde(rename = "ConstituentsPerAggregate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub constituents_per_aggregate: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSnaplockConfiguration {
    #[serde(rename = "AuditLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_volume: Option<bool>,
    #[serde(rename = "AutocommitPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocommit_period: Option<AutocommitPeriod>,
    #[serde(rename = "PrivilegedDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_delete: Option<String>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<SnaplockRetentionPeriod>,
    #[serde(rename = "SnaplockType")]
    #[serde(default)]
    pub snaplock_type: String,
    #[serde(rename = "VolumeAppendModeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_append_mode_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVolumeFromBackupResponse {
    #[serde(rename = "Volume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVolumeRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<CreateOntapVolumeConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<CreateOpenZFSVolumeConfiguration>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
    #[serde(rename = "VolumeType")]
    #[serde(default)]
    pub volume_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpenZFSVolumeConfiguration {
    #[serde(rename = "CopyTagsToSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshots: Option<bool>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "NfsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_exports: Option<Vec<OpenZFSNfsExport>>,
    #[serde(rename = "OriginSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_snapshot: Option<CreateOpenZFSOriginSnapshotConfiguration>,
    #[serde(rename = "ParentVolumeId")]
    #[serde(default)]
    pub parent_volume_id: String,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "RecordSizeKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_size_ki_b: Option<i32>,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_quota_gi_b: Option<i32>,
    #[serde(rename = "StorageCapacityReservationGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_reservation_gi_b: Option<i32>,
    #[serde(rename = "UserAndGroupQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_and_group_quotas: Option<Vec<OpenZFSUserOrGroupQuota>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateOpenZFSOriginSnapshotConfiguration {
    #[serde(rename = "CopyStrategy")]
    #[serde(default)]
    pub copy_strategy: String,
    #[serde(rename = "SnapshotARN")]
    #[serde(default)]
    pub snapshot_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateVolumeResponse {
    #[serde(rename = "Volume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupRequest {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    pub backup_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteBackupResponse {
    #[serde(rename = "BackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataRepositoryAssociationRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "DeleteDataInFileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_data_in_file_system: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDataRepositoryAssociationResponse {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_id: Option<String>,
    #[serde(rename = "DeleteDataInFileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_data_in_file_system: Option<bool>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileCacheRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    pub file_cache_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileCacheResponse {
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<DeleteFileSystemLustreConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<DeleteFileSystemOpenZFSConfiguration>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<DeleteFileSystemWindowsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemLustreConfiguration {
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    #[serde(rename = "SkipFinalBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemOpenZFSConfiguration {
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "SkipFinalBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemWindowsConfiguration {
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    #[serde(rename = "SkipFinalBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemResponse {
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_response: Option<DeleteFileSystemLustreResponse>,
    #[serde(rename = "OpenZFSResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_response: Option<DeleteFileSystemOpenZFSResponse>,
    #[serde(rename = "WindowsResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_response: Option<DeleteFileSystemWindowsResponse>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemLustreResponse {
    #[serde(rename = "FinalBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemOpenZFSResponse {
    #[serde(rename = "FinalBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteFileSystemWindowsResponse {
    #[serde(rename = "FinalBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSnapshotResponse {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStorageVirtualMachineRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    pub storage_virtual_machine_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteStorageVirtualMachineResponse {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVolumeRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<DeleteVolumeOntapConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<DeleteVolumeOpenZFSConfiguration>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVolumeOntapConfiguration {
    #[serde(rename = "BypassSnaplockEnterpriseRetention")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bypass_snaplock_enterprise_retention: Option<bool>,
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
    #[serde(rename = "SkipFinalBackup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_backup: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVolumeOpenZFSConfiguration {
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVolumeResponse {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "OntapResponse")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_response: Option<DeleteVolumeOntapResponse>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteVolumeOntapResponse {
    #[serde(rename = "FinalBackupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_id: Option<String>,
    #[serde(rename = "FinalBackupTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_backup_tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeBackupsRequest {
    #[serde(rename = "BackupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_ids: Option<Vec<String>>,
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
pub struct DescribeBackupsResponse {
    #[serde(rename = "Backups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backups: Option<Vec<Backup>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataRepositoryAssociationsRequest {
    #[serde(rename = "AssociationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association_ids: Option<Vec<String>>,
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
pub struct DescribeDataRepositoryAssociationsResponse {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<DataRepositoryAssociation>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDataRepositoryTasksRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<DataRepositoryTaskFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "TaskIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataRepositoryTaskFilter {
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
pub struct DescribeDataRepositoryTasksResponse {
    #[serde(rename = "DataRepositoryTasks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_tasks: Option<Vec<DataRepositoryTask>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileCachesRequest {
    #[serde(rename = "FileCacheIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_ids: Option<Vec<String>>,
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
pub struct DescribeFileCachesResponse {
    #[serde(rename = "FileCaches")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_caches: Option<Vec<FileCache>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FileCache {
    #[serde(rename = "CreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time: Option<f64>,
    #[serde(rename = "DNSName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_n_s_name: Option<String>,
    #[serde(rename = "DataRepositoryAssociationIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_repository_association_ids: Option<Vec<String>>,
    #[serde(rename = "FailureDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<FileCacheFailureDetails>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_id: Option<String>,
    #[serde(rename = "FileCacheType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_type: Option<String>,
    #[serde(rename = "FileCacheTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache_type_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<FileCacheLustreConfiguration>,
    #[serde(rename = "NetworkInterfaceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_ids: Option<Vec<String>>,
    #[serde(rename = "OwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_a_r_n: Option<String>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_ids: Option<Vec<String>>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileSystemAliasesRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
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
pub struct DescribeFileSystemAliasesResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeFileSystemsRequest {
    #[serde(rename = "FileSystemIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_ids: Option<Vec<String>>,
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
pub struct DescribeFileSystemsResponse {
    #[serde(rename = "FileSystems")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_systems: Option<Vec<FileSystem>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeS3AccessPointAttachmentsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<S3AccessPointAttachmentsFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "Names")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub names: Option<Vec<String>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessPointAttachmentsFilter {
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
pub struct DescribeS3AccessPointAttachmentsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "S3AccessPointAttachments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_point_attachments: Option<Vec<S3AccessPointAttachment>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSharedVpcConfigurationRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSharedVpcConfigurationResponse {
    #[serde(rename = "EnableFsxRouteTableUpdatesFromParticipantAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_fsx_route_table_updates_from_participant_accounts: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSnapshotsRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<SnapshotFilter>>,
    #[serde(rename = "IncludeShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "SnapshotIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotFilter {
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
pub struct DescribeSnapshotsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Snapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<Vec<Snapshot>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeStorageVirtualMachinesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<StorageVirtualMachineFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageVirtualMachineIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageVirtualMachineFilter {
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
pub struct DescribeStorageVirtualMachinesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "StorageVirtualMachines")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machines: Option<Vec<StorageVirtualMachine>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeVolumesRequest {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<VolumeFilter>>,
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "VolumeIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_ids: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VolumeFilter {
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
pub struct DescribeVolumesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Volumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachAndDeleteS3AccessPointRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DetachAndDeleteS3AccessPointResponse {
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFileSystemAliasesRequest {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DisassociateFileSystemAliasesResponse {
    #[serde(rename = "Aliases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<Alias>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {
    #[serde(rename = "MaxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "ResourceARN")]
    #[serde(default)]
    pub resource_a_r_n: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReleaseFileSystemNfsV3LocksRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReleaseFileSystemNfsV3LocksResponse {
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreVolumeFromSnapshotRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Options")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestoreVolumeFromSnapshotResponse {
    #[serde(rename = "AdministrativeActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub administrative_actions: Option<Vec<AdministrativeAction>>,
    #[serde(rename = "Lifecycle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifecycle: Option<String>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMisconfiguredStateRecoveryRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StartMisconfiguredStateRecoveryResponse {
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
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
pub struct UpdateDataRepositoryAssociationRequest {
    #[serde(rename = "AssociationId")]
    #[serde(default)]
    pub association_id: String,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "ImportedFileChunkSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imported_file_chunk_size: Option<i32>,
    #[serde(rename = "S3")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3: Option<S3DataRepositoryConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDataRepositoryAssociationResponse {
    #[serde(rename = "Association")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub association: Option<DataRepositoryAssociation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileCacheRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileCacheId")]
    #[serde(default)]
    pub file_cache_id: String,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<UpdateFileCacheLustreConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileCacheLustreConfiguration {
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileCacheResponse {
    #[serde(rename = "FileCache")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_cache: Option<FileCache>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "FileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "FileSystemTypeVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_type_version: Option<String>,
    #[serde(rename = "LustreConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lustre_configuration: Option<UpdateFileSystemLustreConfiguration>,
    #[serde(rename = "NetworkType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_type: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<UpdateFileSystemOntapConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<UpdateFileSystemOpenZFSConfiguration>,
    #[serde(rename = "StorageCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "WindowsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub windows_configuration: Option<UpdateFileSystemWindowsConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemLustreConfiguration {
    #[serde(rename = "AutoImportPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_import_policy: Option<String>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "DataReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_read_cache_configuration: Option<LustreReadCacheConfiguration>,
    #[serde(rename = "LogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LustreLogCreateConfiguration>,
    #[serde(rename = "MetadataConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata_configuration: Option<UpdateFileSystemLustreMetadataConfiguration>,
    #[serde(rename = "PerUnitStorageThroughput")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_unit_storage_throughput: Option<i32>,
    #[serde(rename = "RootSquashConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_squash_configuration: Option<LustreRootSquashConfiguration>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemLustreMetadataConfiguration {
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemOntapConfiguration {
    #[serde(rename = "AddRouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_route_table_ids: Option<Vec<String>>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "FsxAdminPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsx_admin_password: Option<String>,
    #[serde(rename = "HAPairs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h_a_pairs: Option<i32>,
    #[serde(rename = "RemoveRouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "ThroughputCapacityPerHAPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity_per_h_a_pair: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemOpenZFSConfiguration {
    #[serde(rename = "AddRouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_route_table_ids: Option<Vec<String>>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "CopyTagsToVolumes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_volumes: Option<bool>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "EndpointIpv6AddressRange")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_ipv6_address_range: Option<String>,
    #[serde(rename = "ReadCacheConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_cache_configuration: Option<OpenZFSReadCacheConfiguration>,
    #[serde(rename = "RemoveRouteTableIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_route_table_ids: Option<Vec<String>>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemWindowsConfiguration {
    #[serde(rename = "AuditLogConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_configuration: Option<WindowsAuditLogCreateConfiguration>,
    #[serde(rename = "AutomaticBackupRetentionDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_backup_retention_days: Option<i32>,
    #[serde(rename = "DailyAutomaticBackupStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_automatic_backup_start_time: Option<String>,
    #[serde(rename = "DiskIopsConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disk_iops_configuration: Option<DiskIopsConfiguration>,
    #[serde(rename = "FsrmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsrm_configuration: Option<WindowsFsrmConfiguration>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfigurationUpdates>,
    #[serde(rename = "ThroughputCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub throughput_capacity: Option<i32>,
    #[serde(rename = "WeeklyMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SelfManagedActiveDirectoryConfigurationUpdates {
    #[serde(rename = "DnsIps")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_ips: Option<Vec<String>>,
    #[serde(rename = "DomainJoinServiceAccountSecret")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_join_service_account_secret: Option<String>,
    #[serde(rename = "DomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(rename = "FileSystemAdministratorsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system_administrators_group: Option<String>,
    #[serde(rename = "OrganizationalUnitDistinguishedName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizational_unit_distinguished_name: Option<String>,
    #[serde(rename = "Password")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "UserName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateFileSystemResponse {
    #[serde(rename = "FileSystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_system: Option<FileSystem>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSharedVpcConfigurationRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "EnableFsxRouteTableUpdatesFromParticipantAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_fsx_route_table_updates_from_participant_accounts: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSharedVpcConfigurationResponse {
    #[serde(rename = "EnableFsxRouteTableUpdatesFromParticipantAccounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_fsx_route_table_updates_from_participant_accounts: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSnapshotRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "SnapshotId")]
    #[serde(default)]
    pub snapshot_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSnapshotResponse {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStorageVirtualMachineRequest {
    #[serde(rename = "ActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_directory_configuration: Option<UpdateSvmActiveDirectoryConfiguration>,
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "StorageVirtualMachineId")]
    #[serde(default)]
    pub storage_virtual_machine_id: String,
    #[serde(rename = "SvmAdminPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svm_admin_password: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSvmActiveDirectoryConfiguration {
    #[serde(rename = "NetBiosName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_bios_name: Option<String>,
    #[serde(rename = "SelfManagedActiveDirectoryConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub self_managed_active_directory_configuration:
        Option<SelfManagedActiveDirectoryConfigurationUpdates>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateStorageVirtualMachineResponse {
    #[serde(rename = "StorageVirtualMachine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_virtual_machine: Option<StorageVirtualMachine>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVolumeRequest {
    #[serde(rename = "ClientRequestToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_request_token: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "OntapConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ontap_configuration: Option<UpdateOntapVolumeConfiguration>,
    #[serde(rename = "OpenZFSConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_z_f_s_configuration: Option<UpdateOpenZFSVolumeConfiguration>,
    #[serde(rename = "VolumeId")]
    #[serde(default)]
    pub volume_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOntapVolumeConfiguration {
    #[serde(rename = "CopyTagsToBackups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_backups: Option<bool>,
    #[serde(rename = "JunctionPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub junction_path: Option<String>,
    #[serde(rename = "SecurityStyle")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_style: Option<String>,
    #[serde(rename = "SizeInBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<i64>,
    #[serde(rename = "SizeInMegabytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_in_megabytes: Option<i32>,
    #[serde(rename = "SnaplockConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snaplock_configuration: Option<UpdateSnaplockConfiguration>,
    #[serde(rename = "SnapshotPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_policy: Option<String>,
    #[serde(rename = "StorageEfficiencyEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_efficiency_enabled: Option<bool>,
    #[serde(rename = "TieringPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiering_policy: Option<TieringPolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSnaplockConfiguration {
    #[serde(rename = "AuditLogVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_log_volume: Option<bool>,
    #[serde(rename = "AutocommitPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autocommit_period: Option<AutocommitPeriod>,
    #[serde(rename = "PrivilegedDelete")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged_delete: Option<String>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<SnaplockRetentionPeriod>,
    #[serde(rename = "VolumeAppendModeEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_append_mode_enabled: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateOpenZFSVolumeConfiguration {
    #[serde(rename = "DataCompressionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_compression_type: Option<String>,
    #[serde(rename = "NfsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nfs_exports: Option<Vec<OpenZFSNfsExport>>,
    #[serde(rename = "ReadOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "RecordSizeKiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_size_ki_b: Option<i32>,
    #[serde(rename = "StorageCapacityQuotaGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_quota_gi_b: Option<i32>,
    #[serde(rename = "StorageCapacityReservationGiB")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_capacity_reservation_gi_b: Option<i32>,
    #[serde(rename = "UserAndGroupQuotas")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_and_group_quotas: Option<Vec<OpenZFSUserOrGroupQuota>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateVolumeResponse {
    #[serde(rename = "Volume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume: Option<Volume>,
}
