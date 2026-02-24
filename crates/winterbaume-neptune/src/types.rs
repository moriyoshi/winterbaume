/// A tag on a Neptune resource.
#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Serverless v2 scaling configuration for a Neptune cluster.
#[derive(Debug, Clone)]
pub struct ServerlessV2ScalingConfiguration {
    pub min_capacity: f64,
    pub max_capacity: f64,
}

/// A parameter within a parameter group.
#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub value: String,
    pub apply_method: Option<String>,
}

/// Represents a Neptune DB cluster.
#[derive(Debug, Clone)]
pub struct DbCluster {
    pub identifier: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub status: String,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub port: Option<i32>,
    pub master_username: Option<String>,
    pub database_name: Option<String>,
    pub db_subnet_group_name: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub availability_zones: Vec<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub cluster_create_time: Option<String>,
    pub multi_az: bool,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_cluster_parameter_group: Option<String>,
    pub engine_mode: Option<String>,
    pub copy_tags_to_snapshot: bool,
    pub deletion_protection: bool,
    pub backup_retention_period: i32,
    pub members: Vec<DbClusterMember>,
    /// IAM roles associated with this cluster.
    pub associated_roles: Vec<String>,
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
}

/// DB cluster member.
#[derive(Debug, Clone)]
pub struct DbClusterMember {
    pub db_instance_identifier: String,
    pub is_cluster_writer: bool,
    pub db_cluster_parameter_group_status: String,
    pub promotion_tier: Option<i32>,
}

/// Represents a Neptune DB instance.
#[derive(Debug, Clone)]
pub struct DbInstance {
    pub identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: String,
    pub status: String,
    pub endpoint_address: Option<String>,
    pub port: Option<i32>,
    pub db_subnet_group_name: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub availability_zone: Option<String>,
    pub auto_minor_version_upgrade: bool,
    pub backup_retention_period: i32,
    pub db_cluster_identifier: Option<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub instance_create_time: Option<String>,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub publicly_accessible: bool,
    pub deletion_protection: bool,
    pub db_parameter_group_names: Vec<String>,
    pub multi_az: bool,
}

/// Represents a DB subnet group.
#[derive(Debug, Clone)]
pub struct DbSubnetGroup {
    pub name: String,
    pub description: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub status: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a DB parameter group.
#[derive(Debug, Clone)]
pub struct DbParameterGroup {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub parameters: Vec<Parameter>,
}

/// Represents a DB cluster parameter group.
#[derive(Debug, Clone)]
pub struct DbClusterParameterGroup {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub parameters: Vec<Parameter>,
}

/// Represents a global cluster.
#[derive(Debug, Clone)]
pub struct GlobalCluster {
    pub identifier: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub database_name: Option<String>,
    pub deletion_protection: bool,
    pub storage_encrypted: bool,
    pub status: String,
    pub arn: String,
    pub source_db_cluster_identifier: Option<String>,
}

/// Represents a custom DB cluster endpoint.
#[derive(Debug, Clone)]
pub struct DbClusterEndpoint {
    pub identifier: String,
    pub db_cluster_identifier: String,
    pub endpoint_type: String,
    pub custom_endpoint_type: Option<String>,
    pub endpoint: Option<String>,
    pub arn: String,
    pub resource_identifier: String,
    pub status: String,
    pub static_members: Vec<String>,
    pub excluded_members: Vec<String>,
}

/// Represents an event subscription.
#[derive(Debug, Clone)]
pub struct EventSubscription {
    pub subscription_name: String,
    pub sns_topic_arn: String,
    pub source_type: Option<String>,
    pub enabled: bool,
    pub event_categories: Vec<String>,
    pub source_ids: Vec<String>,
    pub status: String,
    pub arn: String,
    pub customer_aws_id: String,
    pub subscription_creation_time: Option<String>,
}

/// Represents a snapshot sharing attribute.
#[derive(Debug, Clone)]
pub struct SnapshotAttribute {
    pub attribute_name: String,
    pub attribute_values: Vec<String>,
}

/// Represents a DB cluster snapshot.
#[derive(Debug, Clone)]
pub struct DbClusterSnapshot {
    pub identifier: String,
    pub db_cluster_identifier: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub allocated_storage: i32,
    pub status: String,
    pub port: Option<i32>,
    pub vpc_id: Option<String>,
    pub cluster_create_time: Option<String>,
    pub master_username: Option<String>,
    pub snapshot_type: String,
    pub percent_progress: i32,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_cluster_snapshot_arn: String,
    pub availability_zones: Vec<String>,
    pub snapshot_create_time: Option<String>,
    pub tags: Vec<Tag>,
}
