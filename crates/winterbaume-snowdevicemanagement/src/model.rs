//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-snowdevicemanagement

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTaskInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelTaskOutput {
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskInput {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(default)]
    pub command: Command,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    pub targets: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Command {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reboot: Option<Reboot>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlock: Option<Unlock>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Reboot {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Unlock {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateTaskOutput {
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceEc2Input {
    #[serde(rename = "instanceIds")]
    #[serde(default)]
    pub instance_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceEc2Output {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances: Option<Vec<InstanceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<Instance>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Instance {
    #[serde(rename = "amiLaunchIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_launch_index: Option<i32>,
    #[serde(rename = "blockDeviceMappings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_device_mappings: Option<Vec<InstanceBlockDeviceMapping>>,
    #[serde(rename = "cpuOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_options: Option<CpuOptions>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "instanceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "privateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "publicIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip_address: Option<String>,
    #[serde(rename = "rootDeviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_device_name: Option<String>,
    #[serde(rename = "securityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroupIdentifier>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<InstanceState>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceBlockDeviceMapping {
    #[serde(rename = "deviceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ebs: Option<EbsInstanceBlockDevice>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EbsInstanceBlockDevice {
    #[serde(rename = "attachTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_time: Option<f64>,
    #[serde(rename = "deleteOnTermination")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_on_termination: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "volumeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CpuOptions {
    #[serde(rename = "coreCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_count: Option<i32>,
    #[serde(rename = "threadsPerCore")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub threads_per_core: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SecurityGroupIdentifier {
    #[serde(rename = "groupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "groupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InstanceState {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeDeviceOutput {
    #[serde(rename = "associatedWithJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_with_job: Option<String>,
    #[serde(rename = "deviceCapacities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_capacities: Option<Vec<Capacity>>,
    #[serde(rename = "deviceState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_state: Option<String>,
    #[serde(rename = "deviceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "lastReachedOutAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reached_out_at: Option<f64>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "managedDeviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_arn: Option<String>,
    #[serde(rename = "managedDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_id: Option<String>,
    #[serde(rename = "physicalNetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_network_interfaces: Option<Vec<PhysicalNetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub software: Option<SoftwareInformation>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Capacity {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PhysicalNetworkInterface {
    #[serde(rename = "defaultGateway")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_gateway: Option<String>,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(rename = "ipAddressAssignment")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_assignment: Option<String>,
    #[serde(rename = "macAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub netmask: Option<String>,
    #[serde(rename = "physicalConnectorType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_connector_type: Option<String>,
    #[serde(rename = "physicalNetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_network_interface_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SoftwareInformation {
    #[serde(rename = "installState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_state: Option<String>,
    #[serde(rename = "installedVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installed_version: Option<String>,
    #[serde(rename = "installingVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installing_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeExecutionOutput {
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(rename = "managedDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_id: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeTaskOutput {
    #[serde(rename = "completedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<String>>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceResourcesInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDeviceResourcesOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListDevicesOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<DeviceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeviceSummary {
    #[serde(rename = "associatedWithJob")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_with_job: Option<String>,
    #[serde(rename = "managedDeviceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_arn: Option<String>,
    #[serde(rename = "managedDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListExecutionsOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub executions: Option<Vec<ExecutionSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ExecutionSummary {
    #[serde(rename = "executionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(rename = "managedDeviceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_device_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceOutput {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksInput {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTasksOutput {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasks: Option<Vec<TaskSummary>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "taskId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceInput {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceInput {}
