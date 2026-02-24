/// An ECS cluster.
#[derive(Debug, Clone)]
pub struct EcsCluster {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub registered_container_instances_count: i32,
    pub running_tasks_count: i32,
    pub capacity_providers: Vec<String>,
    pub default_capacity_provider_strategy: Vec<CapacityProviderStrategyItem>,
    pub tags: Vec<EcsTag>,
}

/// An ECS task definition.
#[derive(Debug, Clone)]
pub struct TaskDefinition {
    pub family: String,
    pub revision: i32,
    pub arn: String,
    pub container_definitions: Vec<ContainerDefinition>,
    pub status: String,
    pub network_mode: String,
    pub task_role_arn: Option<String>,
    pub execution_role_arn: Option<String>,
    pub requires_compatibilities: Vec<String>,
    pub cpu: Option<String>,
    pub memory: Option<String>,
}

/// A container definition within a task definition.
#[derive(Debug, Clone)]
pub struct ContainerDefinition {
    pub name: String,
    pub image: String,
    pub cpu: i32,
    pub memory: Option<i32>,
    pub memory_reservation: Option<i32>,
    pub essential: bool,
    pub environment: Vec<EnvVar>,
    pub log_configuration: Option<LogConfiguration>,
    pub port_mappings: Vec<PortMapping>,
}

/// An environment variable for a container.
#[derive(Debug, Clone)]
pub struct EnvVar {
    pub name: String,
    pub value: String,
}

/// Log configuration for a container.
#[derive(Debug, Clone)]
pub struct LogConfiguration {
    pub log_driver: String,
}

/// A port mapping for a container.
#[derive(Debug, Clone)]
pub struct PortMapping {
    pub container_port: i32,
    pub host_port: i32,
    pub protocol: String,
}

/// An ECS service.
#[derive(Debug, Clone)]
pub struct EcsServiceDef {
    pub name: String,
    pub arn: String,
    pub cluster_arn: String,
    pub task_definition: String,
    pub desired_count: i32,
    pub running_count: i32,
    pub pending_count: i32,
    pub status: String,
    pub scheduling_strategy: String,
    pub launch_type: String,
    pub tags: Vec<EcsTag>,
    pub load_balancers: Vec<EcsLoadBalancer>,
    pub deployment_controller_type: String,
}

/// A tag for ECS resources.
#[derive(Debug, Clone)]
pub struct EcsTag {
    pub key: String,
    pub value: String,
}

/// An ECS capacity provider.
#[derive(Debug, Clone)]
pub struct EcsCapacityProvider {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub auto_scaling_group_arn: String,
    pub tags: Vec<EcsTag>,
}

/// An ECS container instance.
#[derive(Debug, Clone)]
pub struct EcsContainerInstance {
    pub arn: String,
    pub ec2_instance_id: String,
    pub cluster_arn: String,
    pub status: String,
    pub running_tasks_count: i32,
    pub pending_tasks_count: i32,
    pub agent_connected: bool,
    pub version: i64,
    pub tags: Vec<EcsTag>,
}

/// An ECS task (running or stopped).
#[derive(Debug, Clone)]
pub struct EcsTask {
    pub task_arn: String,
    pub task_definition_arn: String,
    pub cluster_arn: String,
    pub container_instance_arn: Option<String>,
    pub last_status: String,
    pub desired_status: String,
    pub started_by: Option<String>,
    pub group: Option<String>,
    pub launch_type: String,
    pub containers: Vec<EcsContainer>,
    pub overrides: Option<serde_json::Value>,
    pub tags: Vec<EcsTag>,
    pub stopped_reason: Option<String>,
}

/// A container within a running task.
#[derive(Debug, Clone)]
pub struct EcsContainer {
    pub container_arn: String,
    pub name: String,
    pub last_status: String,
    pub task_arn: String,
}

/// An ECS task set.
#[derive(Debug, Clone)]
pub struct EcsTaskSet {
    pub id: String,
    pub task_set_arn: String,
    pub service_arn: String,
    pub cluster_arn: String,
    pub task_definition: String,
    pub status: String,
    pub scale_value: f64,
    pub scale_unit: String,
    pub running_count: i32,
    pub pending_count: i32,
    pub launch_type: String,
    pub tags: Vec<EcsTag>,
}

/// An ECS account setting.
#[derive(Debug, Clone)]
pub struct EcsAccountSetting {
    pub name: String,
    pub value: String,
    pub principal_arn: String,
}

/// An ECS attribute.
#[derive(Debug, Clone)]
pub struct EcsAttribute {
    pub name: String,
    pub value: Option<String>,
    pub target_type: Option<String>,
    pub target_id: Option<String>,
}

/// A capacity provider strategy item.
#[derive(Debug, Clone)]
pub struct CapacityProviderStrategyItem {
    pub capacity_provider: String,
    pub weight: i32,
    pub base: i32,
}

/// A load balancer for an ECS service.
#[derive(Debug, Clone)]
pub struct EcsLoadBalancer {
    pub target_group_arn: Option<String>,
    pub load_balancer_name: Option<String>,
    pub container_name: Option<String>,
    pub container_port: Option<i32>,
}

/// Task protection record.
#[derive(Debug, Clone)]
pub struct EcsTaskProtection {
    pub task_arn: String,
    pub protection_enabled: bool,
    pub expiration_date: Option<f64>,
}

/// A cluster setting (name/value pair stored per-cluster).
#[derive(Debug, Clone)]
pub struct EcsClusterSetting {
    pub name: String,
    pub value: String,
}

/// An ECS Express Gateway Service.
#[derive(Debug, Clone)]
pub struct EcsExpressGatewayService {
    pub service_arn: String,
    pub service_name: String,
    pub cluster: String,
    pub created_at: f64,
    pub updated_at: f64,
}

/// Poll endpoint configuration for ECS agent infrastructure.
#[derive(Debug, Clone)]
pub struct EcsPollEndpoint {
    pub endpoint: String,
    pub telemetry_endpoint: String,
}

/// A submitted attachment state change record (ECS agent internal API).
#[derive(Debug, Clone)]
pub struct EcsAttachmentStateChange {
    pub acknowledgment: String,
}

/// A submitted container state change record (ECS agent internal API).
#[derive(Debug, Clone)]
pub struct EcsContainerStateChange {
    pub acknowledgment: String,
}

/// A submitted task state change record (ECS agent internal API).
#[derive(Debug, Clone)]
pub struct EcsTaskStateChange {
    pub acknowledgment: String,
}

/// A service deployment record.
#[derive(Debug, Clone)]
pub struct EcsServiceDeployment {
    pub service_deployment_arn: String,
    pub service_arn: String,
    pub cluster_arn: String,
    pub status: String,
    pub created_at: f64,
    pub updated_at: f64,
}

/// A service revision record.
#[derive(Debug, Clone)]
pub struct EcsServiceRevision {
    pub service_revision_arn: String,
    pub service_arn: String,
    pub cluster_arn: String,
    pub created_at: f64,
}
