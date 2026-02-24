//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-batch

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobDefinitionsResponse {
    #[serde(rename = "jobDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<JobDefinition>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDefinition {
    #[serde(rename = "consumableResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties: Option<ConsumableResourceProperties>,
    #[serde(rename = "containerOrchestrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_orchestration_type: Option<String>,
    #[serde(rename = "containerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    #[serde(rename = "ecsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties: Option<EcsProperties>,
    #[serde(rename = "eksProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties: Option<EksProperties>,
    #[serde(rename = "jobDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_arn: Option<String>,
    #[serde(rename = "jobDefinitionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    #[serde(rename = "nodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "platformCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<String>>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    #[serde(rename = "schedulingPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumableResourceProperties {
    #[serde(rename = "consumableResourceList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_list: Option<Vec<ConsumableResourceRequirement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumableResourceRequirement {
    #[serde(rename = "consumableResource")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "fargatePlatformConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "jobRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValuePair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EphemeralStorage {
    #[serde(rename = "sizeInGiB")]
    #[serde(default)]
    pub size_in_gi_b: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FargatePlatformConfiguration {
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LinuxParameters {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<Device>>,
    #[serde(rename = "initProcessEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_process_enabled: Option<bool>,
    #[serde(rename = "maxSwap")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_swap: Option<i32>,
    #[serde(rename = "sharedMemorySize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_memory_size: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swappiness: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpfs: Option<Vec<Tmpfs>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Device {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "hostPath")]
    #[serde(default)]
    pub host_path: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Tmpfs {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    pub container_path: String,
    #[serde(rename = "mountOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_options: Option<Vec<String>>,
    #[serde(default)]
    pub size: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogConfiguration {
    #[serde(rename = "logDriver")]
    #[serde(default)]
    pub log_driver: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "secretOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_options: Option<Vec<Secret>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Secret {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "valueFrom")]
    #[serde(default)]
    pub value_from: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct MountPoint {
    #[serde(rename = "containerPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_path: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "sourceVolume")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_volume: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkConfiguration {
    #[serde(rename = "assignPublicIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assign_public_ip: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RepositoryCredentials {
    #[serde(rename = "credentialsParameter")]
    #[serde(default)]
    pub credentials_parameter: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ResourceRequirement {
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(default)]
    pub value: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RuntimePlatform {
    #[serde(rename = "cpuArchitecture")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu_architecture: Option<String>,
    #[serde(rename = "operatingSystemFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system_family: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ulimit {
    #[serde(rename = "hardLimit")]
    #[serde(default)]
    pub hard_limit: i32,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "softLimit")]
    #[serde(default)]
    pub soft_limit: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Volume {
    #[serde(rename = "efsVolumeConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efs_volume_configuration: Option<EFSVolumeConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host: Option<Host>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EFSVolumeConfiguration {
    #[serde(rename = "authorizationConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_config: Option<EFSAuthorizationConfig>,
    #[serde(rename = "fileSystemId")]
    #[serde(default)]
    pub file_system_id: String,
    #[serde(rename = "rootDirectory")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_directory: Option<String>,
    #[serde(rename = "transitEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption: Option<String>,
    #[serde(rename = "transitEncryptionPort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transit_encryption_port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EFSAuthorizationConfig {
    #[serde(rename = "accessPointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_point_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Host {
    #[serde(rename = "sourcePath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsProperties {
    #[serde(rename = "taskProperties")]
    #[serde(default)]
    pub task_properties: Vec<EcsTaskProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsTaskProperties {
    #[serde(default)]
    pub containers: Vec<TaskContainerProperties>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "ipcMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "pidMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskContainerProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskContainerDependency>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "firelensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    #[serde(default)]
    pub image: String,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskContainerDependency {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(rename = "containerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FirelensConfiguration {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksProperties {
    #[serde(rename = "podProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_properties: Option<EksPodProperties>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPodProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<EksContainer>>,
    #[serde(rename = "dnsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    #[serde(rename = "hostNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    #[serde(rename = "imagePullSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<ImagePullSecret>>,
    #[serde(rename = "initContainers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<EksContainer>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EksMetadata>,
    #[serde(rename = "serviceAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    #[serde(rename = "shareProcessNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<EksVolume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainer {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<EksContainerEnvironmentVariable>>,
    #[serde(default)]
    pub image: String,
    #[serde(rename = "imagePullPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EksContainerResourceRequirements>,
    #[serde(rename = "securityContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<EksContainerSecurityContext>,
    #[serde(rename = "volumeMounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<EksContainerVolumeMount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerEnvironmentVariable {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerResourceRequirements {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requests: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerSecurityContext {
    #[serde(rename = "allowPrivilegeEscalation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_privilege_escalation: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "readOnlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only_root_filesystem: Option<bool>,
    #[serde(rename = "runAsGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_group: Option<i64>,
    #[serde(rename = "runAsNonRoot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_non_root: Option<bool>,
    #[serde(rename = "runAsUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_as_user: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerVolumeMount {
    #[serde(rename = "mountPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_path: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "subPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImagePullSecret {
    #[serde(default)]
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksMetadata {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksVolume {
    #[serde(rename = "emptyDir")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub empty_dir: Option<EksEmptyDir>,
    #[serde(rename = "hostPath")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_path: Option<EksHostPath>,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "persistentVolumeClaim")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_volume_claim: Option<EksPersistentVolumeClaim>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<EksSecret>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksEmptyDir {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium: Option<String>,
    #[serde(rename = "sizeLimit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksHostPath {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPersistentVolumeClaim {
    #[serde(rename = "claimName")]
    #[serde(default)]
    pub claim_name: String,
    #[serde(rename = "readOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksSecret {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    #[serde(rename = "secretName")]
    #[serde(default)]
    pub secret_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeProperties {
    #[serde(rename = "mainNode")]
    #[serde(default)]
    pub main_node: i32,
    #[serde(rename = "nodeRangeProperties")]
    #[serde(default)]
    pub node_range_properties: Vec<NodeRangeProperty>,
    #[serde(rename = "numNodes")]
    #[serde(default)]
    pub num_nodes: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeRangeProperty {
    #[serde(rename = "consumableResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties: Option<ConsumableResourceProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerProperties>,
    #[serde(rename = "ecsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties: Option<EcsProperties>,
    #[serde(rename = "eksProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties: Option<EksProperties>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(rename = "targetNodes")]
    #[serde(default)]
    pub target_nodes: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RetryStrategy {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i32>,
    #[serde(rename = "evaluateOnExit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_on_exit: Option<Vec<EvaluateOnExit>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EvaluateOnExit {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "onExitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_exit_code: Option<String>,
    #[serde(rename = "onReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_reason: Option<String>,
    #[serde(rename = "onStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobTimeout {
    #[serde(rename = "attemptDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComputeEnvironmentRequest {
    #[serde(rename = "computeEnvironmentName")]
    #[serde(default)]
    pub compute_environment_name: String,
    #[serde(rename = "computeResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "eksConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_configuration: Option<EksConfiguration>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "unmanagedvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmanagedv_cpus: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeResource {
    #[serde(rename = "allocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "bidPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_percentage: Option<i32>,
    #[serde(rename = "desiredvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i32>,
    #[serde(rename = "ec2Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_configuration: Option<Vec<Ec2Configuration>>,
    #[serde(rename = "ec2KeyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_pair: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "instanceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(rename = "launchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "maxvCpus")]
    #[serde(default)]
    pub maxv_cpus: i32,
    #[serde(rename = "minvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i32>,
    #[serde(rename = "placementGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    #[serde(rename = "scalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<ComputeScalingPolicy>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(rename = "spotIamFleetRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spot_iam_fleet_role: Option<String>,
    #[serde(default)]
    pub subnets: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Ec2Configuration {
    #[serde(rename = "imageIdOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id_override: Option<String>,
    #[serde(rename = "imageKubernetesVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_kubernetes_version: Option<String>,
    #[serde(rename = "imageType")]
    #[serde(default)]
    pub image_type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchTemplateSpecification {
    #[serde(rename = "launchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "launchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<LaunchTemplateSpecificationOverride>>,
    #[serde(rename = "userdataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdata_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LaunchTemplateSpecificationOverride {
    #[serde(rename = "launchTemplateId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_id: Option<String>,
    #[serde(rename = "launchTemplateName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template_name: Option<String>,
    #[serde(rename = "targetInstanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_instance_types: Option<Vec<String>>,
    #[serde(rename = "userdataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userdata_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeScalingPolicy {
    #[serde(rename = "minScaleDownDelayMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_scale_down_delay_minutes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksConfiguration {
    #[serde(rename = "eksClusterArn")]
    #[serde(default)]
    pub eks_cluster_arn: String,
    #[serde(rename = "kubernetesNamespace")]
    #[serde(default)]
    pub kubernetes_namespace: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobQueuesRequest {
    #[serde(rename = "jobQueues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<String>>,
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
pub struct DeleteJobQueueRequest {
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    pub job_queue: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateSchedulingPolicyRequest {
    #[serde(default)]
    pub arn: String,
    #[serde(rename = "fairsharePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairshare_policy: Option<FairsharePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FairsharePolicy {
    #[serde(rename = "computeReservation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_reservation: Option<i32>,
    #[serde(rename = "shareDecaySeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_decay_seconds: Option<i32>,
    #[serde(rename = "shareDistribution")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_distribution: Option<Vec<ShareAttributes>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ShareAttributes {
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    pub share_identifier: String,
    #[serde(rename = "weightFactor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_factor: Option<f32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceEnvironmentsRequest {
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsRequest {
    #[serde(rename = "arrayJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_job_id: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<KeyValuesPair>>,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(rename = "maxResults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    #[serde(rename = "multiNodeJobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_node_job_id: Option<String>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyValuesPair {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComputeEnvironmentsRequest {
    #[serde(rename = "computeEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<String>>,
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
pub struct ListSchedulingPoliciesRequest {
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
pub struct CancelJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceEnvironmentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobQueueResponse {
    #[serde(rename = "jobQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    #[serde(rename = "jobQueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitServiceJobRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    pub job_queue: String,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<ServiceJobRetryStrategy>,
    #[serde(rename = "schedulingPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i32>,
    #[serde(rename = "serviceJobType")]
    #[serde(default)]
    pub service_job_type: String,
    #[serde(rename = "serviceRequestPayload")]
    #[serde(default)]
    pub service_request_payload: String,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<ServiceJobTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobRetryStrategy {
    #[serde(default)]
    pub attempts: i32,
    #[serde(rename = "evaluateOnExit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evaluate_on_exit: Option<Vec<ServiceJobEvaluateOnExit>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobEvaluateOnExit {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "onStatusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobTimeout {
    #[serde(rename = "attemptDurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempt_duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchedulingPolicyRequest {
    #[serde(rename = "fairsharePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairshare_policy: Option<FairsharePolicy>,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobsResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<JobDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDetail {
    #[serde(rename = "arrayProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesDetail>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<AttemptDetail>>,
    #[serde(rename = "consumableResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties: Option<ConsumableResourceProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerDetail>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    #[serde(rename = "ecsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties: Option<EcsPropertiesDetail>,
    #[serde(rename = "eksAttempts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_attempts: Option<Vec<EksAttemptDetail>>,
    #[serde(rename = "eksProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties: Option<EksPropertiesDetail>,
    #[serde(rename = "isCancelled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cancelled: Option<bool>,
    #[serde(rename = "isTerminated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminated: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    #[serde(rename = "nodeDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_details: Option<NodeDetails>,
    #[serde(rename = "nodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "platformCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<String>>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[serde(rename = "schedulingPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i32>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArrayPropertiesDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "statusSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "statusSummaryLastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary_last_updated_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttemptDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<AttemptContainerDetail>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    #[serde(rename = "taskProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_properties: Option<Vec<AttemptEcsTaskDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttemptContainerDetail {
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterface {
    #[serde(rename = "attachmentId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    #[serde(rename = "ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "privateIpv4Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ipv4_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttemptEcsTaskDetails {
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<AttemptTaskContainerDetails>>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttemptTaskContainerDetails {
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "fargatePlatformConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fargate_platform_configuration: Option<FargatePlatformConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(rename = "jobRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_role_arn: Option<String>,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobDependency {
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsPropertiesDetail {
    #[serde(rename = "taskProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_properties: Option<Vec<EcsTaskDetails>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsTaskDetails {
    #[serde(rename = "containerInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_instance_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<TaskContainerDetails>>,
    #[serde(rename = "enableExecuteCommand")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_execute_command: Option<bool>,
    #[serde(rename = "ephemeralStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ephemeral_storage: Option<EphemeralStorage>,
    #[serde(rename = "executionRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_role_arn: Option<String>,
    #[serde(rename = "ipcMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipc_mode: Option<String>,
    #[serde(rename = "networkConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_configuration: Option<NetworkConfiguration>,
    #[serde(rename = "pidMode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pid_mode: Option<String>,
    #[serde(rename = "platformVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,
    #[serde(rename = "runtimePlatform")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_platform: Option<RuntimePlatform>,
    #[serde(rename = "taskArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_arn: Option<String>,
    #[serde(rename = "taskRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_role_arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<Volume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskContainerDetails {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<TaskContainerDependency>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub essential: Option<bool>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(rename = "firelensConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firelens_configuration: Option<FirelensConfiguration>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "linuxParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux_parameters: Option<LinuxParameters>,
    #[serde(rename = "logConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_configuration: Option<LogConfiguration>,
    #[serde(rename = "logStreamName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_stream_name: Option<String>,
    #[serde(rename = "mountPoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mount_points: Option<Vec<MountPoint>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "networkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privileged: Option<bool>,
    #[serde(rename = "readonlyRootFilesystem")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly_root_filesystem: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "repositoryCredentials")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository_credentials: Option<RepositoryCredentials>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Vec<Secret>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ulimits: Option<Vec<Ulimit>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksAttemptDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<EksAttemptContainerDetail>>,
    #[serde(rename = "eksClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_cluster_arn: Option<String>,
    #[serde(rename = "initContainers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<EksAttemptContainerDetail>>,
    #[serde(rename = "nodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "podName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    #[serde(rename = "podNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_namespace: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksAttemptContainerDetail {
    #[serde(rename = "containerID")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_i_d: Option<String>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPropertiesDetail {
    #[serde(rename = "podProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_properties: Option<EksPodPropertiesDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPodPropertiesDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<EksContainerDetail>>,
    #[serde(rename = "dnsPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dns_policy: Option<String>,
    #[serde(rename = "hostNetwork")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub host_network: Option<bool>,
    #[serde(rename = "imagePullSecrets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_secrets: Option<Vec<ImagePullSecret>>,
    #[serde(rename = "initContainers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<EksContainerDetail>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EksMetadata>,
    #[serde(rename = "nodeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[serde(rename = "podName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    #[serde(rename = "serviceAccountName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_account_name: Option<String>,
    #[serde(rename = "shareProcessNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_process_namespace: Option<bool>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volumes: Option<Vec<EksVolume>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<EksContainerEnvironmentVariable>>,
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "imagePullPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_pull_policy: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EksContainerResourceRequirements>,
    #[serde(rename = "securityContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_context: Option<EksContainerSecurityContext>,
    #[serde(rename = "volumeMounts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<Vec<EksContainerVolumeMount>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeDetails {
    #[serde(rename = "isMainNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    #[serde(rename = "nodeIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConsumableResourceRequest {
    #[serde(rename = "consumableResource")]
    #[serde(default)]
    pub consumable_resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSchedulingPoliciesRequest {
    #[serde(default)]
    pub arns: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobQueueSnapshotRequest {
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    pub job_queue: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConsumableResourcesRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<KeyValuesPair>>,
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
pub struct RegisterJobDefinitionResponse {
    #[serde(rename = "jobDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_arn: Option<String>,
    #[serde(rename = "jobDefinitionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateServiceJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceRequest {
    #[serde(default)]
    pub tags: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConsumableResourceRequest {
    #[serde(rename = "clientToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    #[serde(rename = "consumableResource")]
    #[serde(default)]
    pub consumable_resource: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceEnvironmentRequest {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<Vec<CapacityLimit>>,
    #[serde(rename = "serviceEnvironment")]
    #[serde(default)]
    pub service_environment: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CapacityLimit {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(rename = "maxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CancelJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceEnvironmentResponse {
    #[serde(rename = "serviceEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeSchedulingPoliciesResponse {
    #[serde(rename = "schedulingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policies: Option<Vec<SchedulingPolicyDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulingPolicyDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(rename = "fairsharePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairshare_policy: Option<FairsharePolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConsumableResourceRequest {
    #[serde(rename = "consumableResource")]
    #[serde(default)]
    pub consumable_resource: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceJobResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<Vec<ServiceJobAttemptDetail>>,
    #[serde(rename = "capacityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_usage: Option<Vec<ServiceJobCapacityUsageDetail>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "isTerminated")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_terminated: Option<bool>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    #[serde(rename = "latestAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<LatestServiceJobAttempt>,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<ServiceJobRetryStrategy>,
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i64>,
    #[serde(rename = "schedulingPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i32>,
    #[serde(rename = "serviceJobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_job_type: Option<String>,
    #[serde(rename = "serviceRequestPayload")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_request_payload: Option<String>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "timeoutConfig")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout_config: Option<ServiceJobTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobAttemptDetail {
    #[serde(rename = "serviceResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<ServiceResourceId>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceResourceId {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobCapacityUsageDetail {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LatestServiceJobAttempt {
    #[serde(rename = "serviceResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_resource_id: Option<ServiceResourceId>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceJobsResponse {
    #[serde(rename = "jobSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary_list: Option<Vec<ServiceJobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobSummary {
    #[serde(rename = "capacityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_usage: Option<Vec<ServiceJobCapacityUsageSummary>>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "latestAttempt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<LatestServiceJobAttempt>,
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i64>,
    #[serde(rename = "serviceJobType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_job_type: Option<String>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceJobCapacityUsageSummary {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RegisterJobDefinitionRequest {
    #[serde(rename = "consumableResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties: Option<ConsumableResourceProperties>,
    #[serde(rename = "containerProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_properties: Option<ContainerProperties>,
    #[serde(rename = "ecsProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties: Option<EcsProperties>,
    #[serde(rename = "eksProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties: Option<EksProperties>,
    #[serde(rename = "jobDefinitionName")]
    #[serde(default)]
    pub job_definition_name: String,
    #[serde(rename = "nodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodeProperties>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "platformCapabilities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_capabilities: Option<Vec<String>>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[serde(rename = "schedulingPriority")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
    #[serde(rename = "type")]
    #[serde(default)]
    pub r#type: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComputeEnvironmentRequest {
    #[serde(rename = "computeEnvironment")]
    #[serde(default)]
    pub compute_environment: String,
    #[serde(rename = "computeResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResourceUpdate>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "unmanagedvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmanagedv_cpus: Option<i32>,
    #[serde(rename = "updatePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_policy: Option<UpdatePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeResourceUpdate {
    #[serde(rename = "allocationStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocation_strategy: Option<String>,
    #[serde(rename = "bidPercentage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bid_percentage: Option<i32>,
    #[serde(rename = "desiredvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desiredv_cpus: Option<i32>,
    #[serde(rename = "ec2Configuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_configuration: Option<Vec<Ec2Configuration>>,
    #[serde(rename = "ec2KeyPair")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ec2_key_pair: Option<String>,
    #[serde(rename = "imageId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
    #[serde(rename = "instanceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_role: Option<String>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(rename = "launchTemplate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_template: Option<LaunchTemplateSpecification>,
    #[serde(rename = "maxvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxv_cpus: Option<i32>,
    #[serde(rename = "minvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minv_cpus: Option<i32>,
    #[serde(rename = "placementGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placement_group: Option<String>,
    #[serde(rename = "scalingPolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scaling_policy: Option<ComputeScalingPolicy>,
    #[serde(rename = "securityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_group_ids: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "updateToLatestImageVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_to_latest_image_version: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdatePolicy {
    #[serde(rename = "jobExecutionTimeoutMinutes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_execution_timeout_minutes: Option<i64>,
    #[serde(rename = "terminateJobsOnUpdate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminate_jobs_on_update: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListSchedulingPoliciesResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "schedulingPolicies")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policies: Option<Vec<SchedulingPolicyListingDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SchedulingPolicyListingDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobDefinitionsRequest {
    #[serde(rename = "jobDefinitionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_name: Option<String>,
    #[serde(rename = "jobDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definitions: Option<Vec<String>>,
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
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobQueuesResponse {
    #[serde(rename = "jobQueues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queues: Option<Vec<JobQueueDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobQueueDetail {
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    #[serde(rename = "jobQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    #[serde(rename = "jobQueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
    #[serde(rename = "jobQueueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_type: Option<String>,
    #[serde(rename = "jobStateTimeLimitActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state_time_limit_actions: Option<Vec<JobStateTimeLimitAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "schedulingPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_order: Option<Vec<ServiceEnvironmentOrder>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeEnvironmentOrder {
    #[serde(rename = "computeEnvironment")]
    #[serde(default)]
    pub compute_environment: String,
    #[serde(default)]
    pub order: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobStateTimeLimitAction {
    #[serde(default)]
    pub action: String,
    #[serde(rename = "maxTimeSeconds")]
    #[serde(default)]
    pub max_time_seconds: i32,
    #[serde(default)]
    pub reason: String,
    #[serde(default)]
    pub state: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceEnvironmentOrder {
    #[serde(default)]
    pub order: i32,
    #[serde(rename = "serviceEnvironment")]
    #[serde(default)]
    pub service_environment: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListServiceJobsRequest {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<KeyValuesPair>>,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
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
pub struct DeregisterJobDefinitionRequest {
    #[serde(rename = "jobDefinition")]
    #[serde(default)]
    pub job_definition: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobQueueResponse {
    #[serde(rename = "jobQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    #[serde(rename = "jobQueueName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateServiceEnvironmentRequest {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    pub capacity_limits: Vec<CapacityLimit>,
    #[serde(rename = "serviceEnvironmentName")]
    #[serde(default)]
    pub service_environment_name: String,
    #[serde(rename = "serviceEnvironmentType")]
    #[serde(default)]
    pub service_environment_type: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsByConsumableResourceRequest {
    #[serde(rename = "consumableResource")]
    #[serde(default)]
    pub consumable_resource: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<KeyValuesPair>>,
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
pub struct UpdateSchedulingPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeConsumableResourceResponse {
    #[serde(rename = "availableQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_quantity: Option<i64>,
    #[serde(rename = "consumableResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_arn: Option<String>,
    #[serde(rename = "consumableResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_name: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "inUseQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_quantity: Option<i64>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "totalQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListTagsForResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateSchedulingPolicyResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arn: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteComputeEnvironmentRequest {
    #[serde(rename = "computeEnvironment")]
    #[serde(default)]
    pub compute_environment: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceRequest {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateComputeEnvironmentResponse {
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    #[serde(rename = "computeEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateServiceJobResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsResponse {
    #[serde(rename = "jobSummaryList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_summary_list: Option<Vec<JobSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobSummary {
    #[serde(rename = "arrayProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayPropertiesSummary>,
    #[serde(rename = "capacityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_usage: Option<Vec<JobCapacityUsageSummary>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<ContainerSummary>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobDefinition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "nodeProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_properties: Option<NodePropertiesSummary>,
    #[serde(rename = "scheduledAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<i64>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(rename = "stoppedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArrayPropertiesSummary {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(rename = "statusSummary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary: Option<std::collections::HashMap<String, i32>>,
    #[serde(rename = "statusSummaryLastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_summary_last_updated_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct JobCapacityUsageSummary {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerSummary {
    #[serde(rename = "exitCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodePropertiesSummary {
    #[serde(rename = "isMainNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_main_node: Option<bool>,
    #[serde(rename = "nodeIndex")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_index: Option<i32>,
    #[serde(rename = "numNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeComputeEnvironmentsResponse {
    #[serde(rename = "computeEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environments: Option<Vec<ComputeEnvironmentDetail>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ComputeEnvironmentDetail {
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    #[serde(rename = "computeEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
    #[serde(rename = "computeResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_resources: Option<ComputeResource>,
    #[serde(rename = "containerOrchestrationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_orchestration_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<String>,
    #[serde(rename = "ecsClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_cluster_arn: Option<String>,
    #[serde(rename = "eksConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_configuration: Option<EksConfiguration>,
    #[serde(rename = "serviceRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_role: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "unmanagedvCpus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unmanagedv_cpus: Option<i32>,
    #[serde(rename = "updatePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_policy: Option<UpdatePolicy>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeJobsRequest {
    #[serde(default)]
    pub jobs: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsByConsumableResourceResponse {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<ListJobsByConsumableResourceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListJobsByConsumableResourceSummary {
    #[serde(rename = "consumableResourceProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties: Option<ConsumableResourceProperties>,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobDefinitionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_definition_arn: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
    #[serde(rename = "jobQueueArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_arn: Option<String>,
    #[serde(rename = "jobStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(rename = "startedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    #[serde(rename = "statusReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateServiceEnvironmentResponse {
    #[serde(rename = "serviceEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchedulingPolicyResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteConsumableResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceEnvironmentsResponse {
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[serde(rename = "serviceEnvironments")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environments: Option<Vec<ServiceEnvironmentDetail>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceEnvironmentDetail {
    #[serde(rename = "capacityLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_limits: Option<Vec<CapacityLimit>>,
    #[serde(rename = "serviceEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_name: Option<String>,
    #[serde(rename = "serviceEnvironmentType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitServiceJobResponse {
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
    #[serde(rename = "jobId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(rename = "jobName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateJobQueueRequest {
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    #[serde(rename = "jobQueueName")]
    #[serde(default)]
    pub job_queue_name: String,
    #[serde(rename = "jobQueueType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_queue_type: Option<String>,
    #[serde(rename = "jobStateTimeLimitActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state_time_limit_actions: Option<Vec<JobStateTimeLimitAction>>,
    #[serde(default)]
    pub priority: i32,
    #[serde(rename = "schedulingPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_order: Option<Vec<ServiceEnvironmentOrder>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TerminateJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
    #[serde(default)]
    pub reason: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteComputeEnvironmentResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteSchedulingPolicyRequest {
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteServiceEnvironmentRequest {
    #[serde(rename = "serviceEnvironment")]
    #[serde(default)]
    pub service_environment: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConsumableResourceResponse {
    #[serde(rename = "consumableResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_arn: Option<String>,
    #[serde(rename = "consumableResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeregisterJobDefinitionResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubmitJobRequest {
    #[serde(rename = "arrayProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array_properties: Option<ArrayProperties>,
    #[serde(rename = "consumableResourcePropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties_override: Option<ConsumableResourceProperties>,
    #[serde(rename = "containerOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    #[serde(rename = "dependsOn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depends_on: Option<Vec<JobDependency>>,
    #[serde(rename = "ecsPropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties_override: Option<EcsPropertiesOverride>,
    #[serde(rename = "eksPropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties_override: Option<EksPropertiesOverride>,
    #[serde(rename = "jobDefinition")]
    #[serde(default)]
    pub job_definition: String,
    #[serde(rename = "jobName")]
    #[serde(default)]
    pub job_name: String,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    pub job_queue: String,
    #[serde(rename = "nodeOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_overrides: Option<NodeOverrides>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "propagateTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub propagate_tags: Option<bool>,
    #[serde(rename = "retryStrategy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry_strategy: Option<RetryStrategy>,
    #[serde(rename = "schedulingPriorityOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_priority_override: Option<i32>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<JobTimeout>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ArrayProperties {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ContainerOverrides {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(rename = "instanceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<i32>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vcpus: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EcsPropertiesOverride {
    #[serde(rename = "taskProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_properties: Option<Vec<TaskPropertiesOverride>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskPropertiesOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<TaskContainerOverrides>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaskContainerOverrides {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Vec<KeyValuePair>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "resourceRequirements")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_requirements: Option<Vec<ResourceRequirement>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPropertiesOverride {
    #[serde(rename = "podProperties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_properties: Option<EksPodPropertiesOverride>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksPodPropertiesOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub containers: Option<Vec<EksContainerOverride>>,
    #[serde(rename = "initContainers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub init_containers: Option<Vec<EksContainerOverride>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<EksMetadata>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EksContainerOverride {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<Vec<String>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env: Option<Vec<EksContainerEnvironmentVariable>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<EksContainerResourceRequirements>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeOverrides {
    #[serde(rename = "nodePropertyOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_property_overrides: Option<Vec<NodePropertyOverride>>,
    #[serde(rename = "numNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_nodes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodePropertyOverride {
    #[serde(rename = "consumableResourcePropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_properties_override: Option<ConsumableResourceProperties>,
    #[serde(rename = "containerOverrides")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_overrides: Option<ContainerOverrides>,
    #[serde(rename = "ecsPropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecs_properties_override: Option<EcsPropertiesOverride>,
    #[serde(rename = "eksPropertiesOverride")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eks_properties_override: Option<EksPropertiesOverride>,
    #[serde(rename = "instanceTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_types: Option<Vec<String>>,
    #[serde(rename = "targetNodes")]
    #[serde(default)]
    pub target_nodes: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UntagResourceResponse {}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateComputeEnvironmentResponse {
    #[serde(rename = "computeEnvironmentArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_arn: Option<String>,
    #[serde(rename = "computeEnvironmentName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeServiceJobRequest {
    #[serde(rename = "jobId")]
    #[serde(default)]
    pub job_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateJobQueueRequest {
    #[serde(rename = "computeEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_order: Option<Vec<ComputeEnvironmentOrder>>,
    #[serde(rename = "jobQueue")]
    #[serde(default)]
    pub job_queue: String,
    #[serde(rename = "jobStateTimeLimitActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_state_time_limit_actions: Option<Vec<JobStateTimeLimitAction>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(rename = "schedulingPolicyArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduling_policy_arn: Option<String>,
    #[serde(rename = "serviceEnvironmentOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_environment_order: Option<Vec<ServiceEnvironmentOrder>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetJobQueueSnapshotResponse {
    #[serde(rename = "frontOfQueue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_of_queue: Option<FrontOfQueueDetail>,
    #[serde(rename = "queueUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_utilization: Option<QueueSnapshotUtilizationDetail>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrontOfQueueDetail {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jobs: Option<Vec<FrontOfQueueJobSummary>>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FrontOfQueueJobSummary {
    #[serde(rename = "earliestTimeAtPosition")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_time_at_position: Option<i64>,
    #[serde(rename = "jobArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueSnapshotUtilizationDetail {
    #[serde(rename = "fairshareUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fairshare_utilization: Option<FairshareUtilizationDetail>,
    #[serde(rename = "lastUpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<i64>,
    #[serde(rename = "totalCapacityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_capacity_usage: Option<Vec<QueueSnapshotCapacityUsage>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FairshareUtilizationDetail {
    #[serde(rename = "activeShareCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_share_count: Option<i64>,
    #[serde(rename = "topCapacityUtilization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_capacity_utilization: Option<Vec<FairshareCapacityUtilization>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FairshareCapacityUtilization {
    #[serde(rename = "capacityUsage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_usage: Option<Vec<FairshareCapacityUsage>>,
    #[serde(rename = "shareIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FairshareCapacityUsage {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct QueueSnapshotCapacityUsage {
    #[serde(rename = "capacityUnit")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity_unit: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateConsumableResourceRequest {
    #[serde(rename = "consumableResourceName")]
    #[serde(default)]
    pub consumable_resource_name: String,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "totalQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ListConsumableResourcesResponse {
    #[serde(rename = "consumableResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resources: Option<Vec<ConsumableResourceSummary>>,
    #[serde(rename = "nextToken")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumableResourceSummary {
    #[serde(rename = "consumableResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_arn: Option<String>,
    #[serde(rename = "consumableResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_name: Option<String>,
    #[serde(rename = "inUseQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_use_quantity: Option<i64>,
    #[serde(rename = "resourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "totalQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateConsumableResourceResponse {
    #[serde(rename = "consumableResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_arn: Option<String>,
    #[serde(rename = "consumableResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumable_resource_name: Option<String>,
    #[serde(rename = "totalQuantity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_quantity: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteJobQueueResponse {}
