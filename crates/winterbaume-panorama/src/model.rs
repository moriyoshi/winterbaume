//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-panorama

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveApplicationInstanceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobForDevicesResponse {
    #[serde(rename = "Jobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<Job>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Job {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodeFromTemplateJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NodeFromTemplateJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_from_template_jobs: Option<Vec<NodeFromTemplateJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeFromTemplateJob {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "NodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageImportJobsResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageImportJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_import_jobs: Option<Vec<PackageImportJob>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageImportJob {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
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
pub struct CreatePackageResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "StorageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<StorageLocation>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StorageLocation {
    #[serde(rename = "BinaryPrefixLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_prefix_location: Option<String>,
    #[serde(rename = "Bucket")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    #[serde(rename = "GeneratedPrefixLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_prefix_location: Option<String>,
    #[serde(rename = "ManifestPrefixLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_prefix_location: Option<String>,
    #[serde(rename = "RepoPrefixLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_prefix_location: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalApplicationInstanceNodeInstancesRequest {
    #[serde(rename = "NodeSignals")]
    #[serde(default)]
    pub node_signals: Vec<NodeSignal>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeSignal {
    #[serde(rename = "NodeInstanceId")]
    #[serde(default)]
    pub node_instance_id: String,
    #[serde(rename = "Signal")]
    #[serde(default)]
    pub signal: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationInstanceDetailsResponse {
    #[serde(rename = "ApplicationInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id: Option<String>,
    #[serde(rename = "ApplicationInstanceIdToReplace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id_to_replace: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DefaultRuntimeContextDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime_context_device: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ManifestOverridesPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_overrides_payload: Option<ManifestOverridesPayload>,
    #[serde(rename = "ManifestPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_payload: Option<ManifestPayload>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManifestOverridesPayload {
    #[serde(rename = "PayloadData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ManifestPayload {
    #[serde(rename = "PayloadData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_data: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPackageVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceMetadataRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceResponse {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationInstanceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageVersionResponse {
    #[serde(rename = "IsLatestPatch")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_latest_patch: Option<bool>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "PackageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_arn: Option<String>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_version: Option<String>,
    #[serde(rename = "RegisteredTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered_time: Option<f64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstanceDependenciesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstancesResponse {
    #[serde(rename = "ApplicationInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instances: Option<Vec<ApplicationInstance>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApplicationInstance {
    #[serde(rename = "ApplicationInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DefaultRuntimeContextDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime_context_device: Option<String>,
    #[serde(rename = "DefaultRuntimeContextDeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime_context_device_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RuntimeContextStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_context_states: Option<Vec<ReportedRuntimeContextState>>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReportedRuntimeContextState {
    #[serde(rename = "DesiredState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_state: Option<String>,
    #[serde(rename = "DeviceReportedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_reported_status: Option<String>,
    #[serde(rename = "DeviceReportedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_reported_time: Option<f64>,
    #[serde(rename = "RuntimeContextName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_context_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationInstanceRequest {
    #[serde(rename = "ApplicationInstanceIdToReplace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id_to_replace: Option<String>,
    #[serde(rename = "DefaultRuntimeContextDevice")]
    #[serde(default)]
    pub default_runtime_context_device: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ManifestOverridesPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_overrides_payload: Option<ManifestOverridesPayload>,
    #[serde(rename = "ManifestPayload")]
    #[serde(default)]
    pub manifest_payload: ManifestPayload,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RuntimeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_role_arn: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "ReadAccessPrincipalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_access_principal_arns: Option<Vec<String>>,
    #[serde(rename = "StorageLocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_location: Option<StorageLocation>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "WriteAccessPrincipalArns")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub write_access_principal_arns: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesResponse {
    #[serde(rename = "Devices")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "Brand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "CurrentSoftware")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_software: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceAggregatedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_aggregated_status: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "LatestDeviceJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_device_job: Option<LatestDeviceJob>,
    #[serde(rename = "LeaseExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_expiration_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "ProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatestDeviceJob {
    #[serde(rename = "ImageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_version: Option<String>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstancesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceResponse {
    #[serde(rename = "AlternateSoftwares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_softwares: Option<Vec<AlternateSoftwareMetadata>>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Brand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "CurrentNetworkingStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_networking_status: Option<NetworkStatus>,
    #[serde(rename = "CurrentSoftware")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_software: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "DeviceAggregatedStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_aggregated_status: Option<String>,
    #[serde(rename = "DeviceConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_connection_status: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "LatestAlternateSoftware")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_alternate_software: Option<String>,
    #[serde(rename = "LatestDeviceJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_device_job: Option<LatestDeviceJob>,
    #[serde(rename = "LatestSoftware")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_software: Option<String>,
    #[serde(rename = "LeaseExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_expiration_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NetworkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_configuration: Option<NetworkPayload>,
    #[serde(rename = "ProvisioningStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioning_status: Option<String>,
    #[serde(rename = "SerialNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AlternateSoftwareMetadata {
    #[serde(rename = "Version")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkStatus {
    #[serde(rename = "Ethernet0Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet0_status: Option<EthernetStatus>,
    #[serde(rename = "Ethernet1Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet1_status: Option<EthernetStatus>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "NtpStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_status: Option<NtpStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EthernetStatus {
    #[serde(rename = "ConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[serde(rename = "HwAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_address: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NtpStatus {
    #[serde(rename = "ConnectionStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_status: Option<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "NtpServerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp_server_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkPayload {
    #[serde(rename = "Ethernet0")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet0: Option<EthernetPayload>,
    #[serde(rename = "Ethernet1")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ethernet1: Option<EthernetPayload>,
    #[serde(rename = "Ntp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntp: Option<NtpPayload>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EthernetPayload {
    #[serde(rename = "ConnectionType")]
    #[serde(default)]
    pub connection_type: String,
    #[serde(rename = "StaticIpConnectionInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_ip_connection_info: Option<StaticIpConnectionInfo>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StaticIpConnectionInfo {
    #[serde(rename = "DefaultGateway")]
    #[serde(default)]
    pub default_gateway: String,
    #[serde(rename = "Dns")]
    #[serde(default)]
    pub dns: Vec<String>,
    #[serde(rename = "IpAddress")]
    #[serde(default)]
    pub ip_address: String,
    #[serde(rename = "Mask")]
    #[serde(default)]
    pub mask: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NtpPayload {
    #[serde(rename = "NtpServers")]
    #[serde(default)]
    pub ntp_servers: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterPackageVersionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageImportJobResponse {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "InputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_config: Option<PackageImportJobInputConfig>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<JobResourceTags>>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Output")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<PackageImportJobOutput>,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_config: Option<PackageImportJobOutputConfig>,
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
pub struct PackageImportJobInputConfig {
    #[serde(rename = "PackageVersionInputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_input_config: Option<PackageVersionInputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionInputConfig {
    #[serde(rename = "S3Location")]
    #[serde(default)]
    pub s3_location: S3Location,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3Location {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    pub bucket_name: String,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    pub object_key: String,
    #[serde(rename = "Region")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobResourceTags {
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    pub resource_type: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageImportJobOutput {
    #[serde(rename = "OutputS3Location")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_s3_location: Option<OutPutS3Location>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OutPutS3Location {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ObjectKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_key: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageImportJobOutputConfig {
    #[serde(rename = "PackageVersionOutputConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version_output_config: Option<PackageVersionOutputConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageVersionOutputConfig {
    #[serde(rename = "MarkLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_latest: Option<bool>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    pub package_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageRequest {
    #[serde(rename = "PackageName")]
    #[serde(default)]
    pub package_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeResponse {
    #[serde(rename = "AssetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_name: Option<String>,
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeInterface")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_interface: Option<NodeInterface>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "PackageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_arn: Option<String>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInterface {
    #[serde(rename = "Inputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<NodeInputPort>>,
    #[serde(rename = "Outputs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<NodeOutputPort>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInputPort {
    #[serde(rename = "DefaultValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "MaxConnections")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i32>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeOutputPort {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodeFromTemplateJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RemoveApplicationInstanceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionDeviceResponse {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "Certificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificates: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "IotThingName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iot_thing_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageImportJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateApplicationInstanceResponse {
    #[serde(rename = "ApplicationInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationInstanceDetailsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageImportJobRequest {
    #[serde(rename = "ClientToken")]
    #[serde(default)]
    pub client_token: String,
    #[serde(rename = "InputConfig")]
    #[serde(default)]
    pub input_config: PackageImportJobInputConfig,
    #[serde(rename = "JobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<JobResourceTags>>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    pub job_type: String,
    #[serde(rename = "OutputConfig")]
    #[serde(default)]
    pub output_config: PackageImportJobOutputConfig,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreatePackageImportJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeletePackageResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeFromTemplateJobRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstanceDependenciesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "PackageObjects")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_objects: Option<Vec<PackageObject>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageObject {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceJobResponse {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DeviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_arn: Option<String>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "DeviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "ImageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_version: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobForDevicesRequest {
    #[serde(rename = "DeviceIds")]
    #[serde(default)]
    pub device_ids: Vec<String>,
    #[serde(rename = "DeviceJobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_job_config: Option<DeviceJobConfig>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    pub job_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceJobConfig {
    #[serde(rename = "OTAJobConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub o_t_a_job_config: Option<OTAJobConfig>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OTAJobConfig {
    #[serde(rename = "AllowMajorVersionUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_update: Option<bool>,
    #[serde(rename = "ImageVersion")]
    #[serde(default)]
    pub image_version: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPackageVersionRequest {
    #[serde(rename = "MarkLatest")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_latest: Option<bool>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeFromTemplateJobRequest {
    #[serde(rename = "JobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<JobResourceTags>>,
    #[serde(rename = "NodeDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    #[serde(rename = "NodeName")]
    #[serde(default)]
    pub node_name: String,
    #[serde(rename = "OutputPackageName")]
    #[serde(default)]
    pub output_package_name: String,
    #[serde(rename = "OutputPackageVersion")]
    #[serde(default)]
    pub output_package_version: String,
    #[serde(rename = "TemplateParameters")]
    #[serde(default)]
    pub template_parameters: std::collections::HashMap<String, String>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    pub template_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteDeviceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackageImportJobsRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeApplicationInstanceResponse {
    #[serde(rename = "ApplicationInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id: Option<String>,
    #[serde(rename = "ApplicationInstanceIdToReplace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id_to_replace: Option<String>,
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DefaultRuntimeContextDevice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime_context_device: Option<String>,
    #[serde(rename = "DefaultRuntimeContextDeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_runtime_context_device_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HealthStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health_status: Option<String>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "RuntimeContextStates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_context_states: Option<Vec<ReportedRuntimeContextState>>,
    #[serde(rename = "RuntimeRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNodeFromTemplateJobResponse {
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterPackageVersionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListNodesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Nodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nodes: Option<Vec<Node>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Node {
    #[serde(rename = "Category")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "PackageArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_arn: Option<String>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
    #[serde(rename = "PatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstanceNodeInstancesRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateDeviceMetadataResponse {
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ProvisionDeviceRequest {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "NetworkingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networking_configuration: Option<NetworkPayload>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesJobsResponse {
    #[serde(rename = "DeviceJobs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_jobs: Option<Vec<DeviceJob>>,
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceJob {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "DeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "DeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SignalApplicationInstanceNodeInstancesResponse {
    #[serde(rename = "ApplicationInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_instance_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeNodeFromTemplateJobResponse {
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "JobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "JobTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_tags: Option<Vec<JobResourceTags>>,
    #[serde(rename = "LastUpdatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[serde(rename = "NodeDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_description: Option<String>,
    #[serde(rename = "NodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "OutputPackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_package_name: Option<String>,
    #[serde(rename = "OutputPackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_package_version: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "TemplateParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "TemplateType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListPackagesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "Packages")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packages: Option<Vec<PackageListItem>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PackageListItem {
    #[serde(rename = "Arn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "CreatedTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<f64>,
    #[serde(rename = "PackageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribePackageVersionRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListApplicationInstanceNodeInstancesResponse {
    #[serde(rename = "NextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "NodeInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_instances: Option<Vec<NodeInstance>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeInstance {
    #[serde(rename = "CurrentStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_status: Option<String>,
    #[serde(rename = "NodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(rename = "NodeInstanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_instance_id: Option<String>,
    #[serde(rename = "NodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "PackageName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
    #[serde(rename = "PackagePatchVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_patch_version: Option<String>,
    #[serde(rename = "PackageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_version: Option<String>,
}
