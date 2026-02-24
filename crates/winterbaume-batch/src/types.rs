use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// A Batch job queue.
#[derive(Debug, Clone)]
pub struct JobQueue {
    pub job_queue_name: String,
    pub job_queue_arn: String,
    pub state: String,
    pub status: String,
    pub status_reason: String,
    pub priority: i32,
    pub compute_environment_order: Vec<ComputeEnvironmentOrder>,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
    pub scheduling_policy_arn: Option<String>,
}

/// Compute environment ordering within a job queue.
#[derive(Debug, Clone)]
pub struct ComputeEnvironmentOrder {
    pub order: i32,
    pub compute_environment: String,
}

/// A Batch job definition.
#[derive(Debug, Clone)]
pub struct JobDefinition {
    pub job_definition_name: String,
    pub job_definition_arn: String,
    pub revision: i32,
    pub status: String,
    pub job_definition_type: String,
    pub container_properties: Option<ContainerProperties>,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// Container properties for a job definition.
#[derive(Debug, Clone)]
pub struct ContainerProperties {
    pub image: String,
    pub command: Vec<String>,
    pub resource_requirements: Vec<ResourceRequirement>,
}

/// A resource requirement (VCPU, MEMORY, GPU).
#[derive(Debug, Clone)]
pub struct ResourceRequirement {
    pub resource_type: String,
    pub value: String,
}

/// A Batch compute environment.
#[derive(Debug, Clone)]
pub struct ComputeEnvironment {
    pub compute_environment_name: String,
    pub compute_environment_arn: String,
    pub ce_type: String,
    pub state: String,
    pub status: String,
    pub status_reason: String,
    pub service_role: Option<String>,
    pub tags: HashMap<String, String>,
    pub created_at: DateTime<Utc>,
}

/// A Batch scheduling policy.
#[derive(Debug, Clone)]
pub struct SchedulingPolicy {
    pub name: String,
    pub arn: String,
    pub fairshare_policy: Option<FairsharePolicy>,
    pub tags: HashMap<String, String>,
}

/// Fairshare policy for a scheduling policy.
#[derive(Debug, Clone)]
pub struct FairsharePolicy {
    pub compute_reservation: Option<i32>,
    pub share_decay_seconds: Option<i32>,
    pub share_distribution: Vec<ShareAttributes>,
}

/// Share attributes within a fairshare policy.
#[derive(Debug, Clone)]
pub struct ShareAttributes {
    pub share_identifier: String,
    pub weight_factor: Option<f32>,
}

/// A consumable resource (like GPU/CPU quota).
#[derive(Debug, Clone)]
pub struct ConsumableResource {
    pub consumable_resource_name: String,
    pub consumable_resource_arn: String,
    pub total_quantity: i64,
    pub in_use_quantity: i64,
    pub resource_type: Option<String>,
    pub created_at: i64,
    pub tags: HashMap<String, String>,
}

/// A service environment for running services.
#[derive(Debug, Clone)]
pub struct ServiceEnvironment {
    pub service_environment_name: String,
    pub service_environment_arn: String,
    pub service_environment_type: String,
    pub state: String,
    pub status: String,
    pub tags: HashMap<String, String>,
    pub created_at: i64,
}

/// A service job submitted to a service environment.
#[derive(Debug, Clone)]
pub struct ServiceJob {
    pub job_id: String,
    pub job_arn: String,
    pub job_name: String,
    pub job_queue: String,
    pub status: String,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub stopped_at: Option<i64>,
    pub is_terminated: bool,
    pub tags: HashMap<String, String>,
}

/// A Batch job.
#[derive(Debug, Clone)]
pub struct Job {
    pub job_id: String,
    pub job_name: String,
    pub job_arn: String,
    pub job_queue: String,
    pub job_definition: String,
    pub status: String,
    pub status_reason: Option<String>,
    pub created_at: i64,
    pub started_at: i64,
    pub tags: HashMap<String, String>,
}
