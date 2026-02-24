/// A tag on an RDS resource.
#[derive(Debug, Clone, Default)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// Represents a DB instance.
#[derive(Debug, Clone)]
pub struct DbInstance {
    pub identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: String,
    pub status: String,
    pub master_username: Option<String>,
    pub db_name: Option<String>,
    pub endpoint_address: Option<String>,
    pub port: Option<i32>,
    pub multi_az: bool,
    pub storage_type: Option<String>,
    pub allocated_storage: i32,
    pub db_subnet_group_name: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub db_parameter_group_names: Vec<String>,
    pub availability_zone: Option<String>,
    pub publicly_accessible: bool,
    pub auto_minor_version_upgrade: bool,
    pub backup_retention_period: i32,
    pub db_cluster_identifier: Option<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub instance_create_time: Option<String>,
    pub license_model: Option<String>,
    pub iops: Option<i32>,
    pub deletion_protection: bool,
    pub copy_tags_to_snapshot: bool,
    pub monitoring_interval: Option<i32>,
    pub performance_insights_enabled: bool,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub ca_certificate_identifier: Option<String>,
    pub secondary_availability_zone: Option<String>,
    /// IAM roles associated with this DB instance.
    pub associated_roles: Vec<String>,
}

/// Represents a DB cluster.
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
    pub storage_type: Option<String>,
    pub allocated_storage: Option<i32>,
    pub backup_retention_period: i32,
    pub deletion_protection: bool,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_cluster_parameter_group: Option<String>,
    pub engine_mode: Option<String>,
    pub copy_tags_to_snapshot: bool,
    pub members: Vec<DbClusterMember>,
    /// IAM roles associated with this DB cluster.
    pub associated_roles: Vec<String>,
}

/// DB cluster member.
#[derive(Debug, Clone)]
pub struct DbClusterMember {
    pub db_instance_identifier: String,
    pub is_cluster_writer: bool,
    pub db_cluster_parameter_group_status: String,
    pub promotion_tier: Option<i32>,
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
}

/// Represents a DB cluster parameter group.
#[derive(Debug, Clone)]
pub struct DbClusterParameterGroup {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents a DB snapshot.
#[derive(Debug, Clone)]
pub struct DbSnapshot {
    pub identifier: String,
    pub db_instance_identifier: String,
    pub engine: String,
    pub engine_version: Option<String>,
    pub allocated_storage: i32,
    pub status: String,
    pub port: Option<i32>,
    pub availability_zone: Option<String>,
    pub vpc_id: Option<String>,
    pub instance_create_time: Option<String>,
    pub master_username: Option<String>,
    pub snapshot_type: String,
    pub iops: Option<i32>,
    pub option_group_name: Option<String>,
    pub percent_progress: i32,
    pub source_region: Option<String>,
    pub source_db_snapshot_identifier: Option<String>,
    pub storage_type: Option<String>,
    pub tde_credential_arn: Option<String>,
    pub encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_snapshot_arn: String,
    pub timezone: Option<String>,
    pub db_instance_automated_backups_arn: Option<String>,
    pub snapshot_create_time: Option<String>,
    pub tags: Vec<Tag>,
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
    pub source_db_cluster_snapshot_arn: Option<String>,
    pub availability_zones: Vec<String>,
    pub snapshot_create_time: Option<String>,
    pub tags: Vec<Tag>,
    pub storage_type: Option<String>,
}

/// Represents a DB security group.
#[derive(Debug, Clone)]
pub struct DbSecurityGroup {
    pub name: String,
    pub description: String,
    pub vpc_id: Option<String>,
    pub ec2_security_groups: Vec<EC2SecurityGroup>,
    pub ip_ranges: Vec<IPRange>,
    pub arn: String,
    pub tags: Vec<Tag>,
    pub owner_id: String,
}

#[derive(Debug, Clone)]
pub struct EC2SecurityGroup {
    pub status: String,
    pub ec2_security_group_name: Option<String>,
    pub ec2_security_group_id: Option<String>,
    pub ec2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct IPRange {
    pub status: String,
    pub cidrip: String,
}

/// Represents an event subscription.
#[derive(Debug, Clone)]
pub struct EventSubscription {
    pub subscription_name: String,
    pub sns_topic_arn: String,
    pub source_type: Option<String>,
    pub source_ids: Vec<String>,
    pub event_categories: Vec<String>,
    pub enabled: bool,
    pub status: String,
    pub arn: String,
    pub customer_aws_id: String,
    pub subscription_creation_time: String,
}

/// Represents an option group.
#[derive(Debug, Clone)]
pub struct OptionGroup {
    pub name: String,
    pub engine_name: String,
    pub major_engine_version: String,
    pub description: String,
    pub allows_vpc_and_non_vpc_instance_memberships: bool,
    pub vpc_id: Option<String>,
    pub arn: String,
    pub tags: Vec<Tag>,
}

/// Represents an export task.
#[derive(Debug, Clone)]
pub struct ExportTask {
    pub export_task_identifier: String,
    pub source_arn: String,
    pub export_only: Vec<String>,
    pub source_type: Option<String>,
    pub snapshot_time: Option<String>,
    pub task_start_time: Option<String>,
    pub task_end_time: Option<String>,
    pub s3_bucket: String,
    pub s3_prefix: Option<String>,
    pub iam_role_arn: String,
    pub kms_key_id: String,
    pub status: String,
    pub percent_progress: i32,
    pub total_extracted_data_in_gb: Option<i32>,
    pub failure_cause: Option<String>,
    pub warning_message: Option<String>,
}

/// Represents a global cluster.
#[derive(Debug, Clone)]
pub struct GlobalCluster {
    pub global_cluster_identifier: String,
    pub global_cluster_resource_id: String,
    pub global_cluster_arn: String,
    pub status: String,
    pub engine: Option<String>,
    pub engine_version: Option<String>,
    pub database_name: Option<String>,
    pub storage_encrypted: bool,
    pub deletion_protection: bool,
}

/// Represents a DB proxy.
#[derive(Debug, Clone)]
pub struct DbProxy {
    pub db_proxy_name: String,
    pub db_proxy_arn: String,
    pub status: String,
    pub engine_family: String,
    pub vpc_id: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub vpc_subnet_ids: Vec<String>,
    pub endpoint: String,
    pub require_tls: bool,
    pub idle_client_timeout: i32,
    pub debug_logging: bool,
    pub role_arn: String,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
    pub tags: Vec<Tag>,
    pub targets: Vec<DbProxyTarget>,
}

#[derive(Debug, Clone)]
pub struct DbProxyTarget {
    pub target_arn: String,
    pub endpoint: Option<String>,
    pub tracked_cluster_id: Option<String>,
    pub rds_resource_id: Option<String>,
    pub port: Option<i32>,
    pub type_: Option<String>,
    pub role: Option<String>,
    pub target_health_status: Option<String>,
}

/// Represents a blue/green deployment.
#[derive(Debug, Clone)]
pub struct BlueGreenDeployment {
    pub blue_green_deployment_identifier: String,
    pub blue_green_deployment_name: String,
    pub source: Option<String>,
    pub target: Option<String>,
    pub status: String,
    pub status_details: Option<String>,
    pub create_time: Option<String>,
    pub delete_time: Option<String>,
    pub tags: Vec<Tag>,
}

/// Represents a DB proxy target group.
#[derive(Debug, Clone)]
pub struct DbProxyTargetGroup {
    pub target_group_name: String,
    pub db_proxy_name: String,
    pub target_group_arn: String,
    pub is_default: bool,
    pub status: String,
    pub connection_pool_config: Option<String>,
    pub created_date: Option<String>,
    pub updated_date: Option<String>,
}

/// Represents a DB shard group.
#[derive(Debug, Clone)]
pub struct DbShardGroup {
    pub db_shard_group_identifier: String,
    pub db_shard_group_resource_id: Option<String>,
    pub db_cluster_identifier: String,
    pub max_acu: f64,
    pub min_acu: Option<f64>,
    pub publicly_accessible: bool,
    pub status: String,
    pub endpoint: Option<String>,
    pub db_shard_group_arn: Option<String>,
    pub tag_list: Vec<Tag>,
}

/// Represents a DB cluster endpoint.
#[derive(Debug, Clone)]
pub struct DbClusterEndpoint {
    pub db_cluster_endpoint_identifier: String,
    pub db_cluster_identifier: String,
    pub db_cluster_endpoint_arn: String,
    pub endpoint: String,
    pub status: String,
    pub endpoint_type: String,
    pub custom_endpoint_type: Option<String>,
    pub static_members: Vec<String>,
    pub excluded_members: Vec<String>,
}

/// Represents a DB proxy endpoint.
#[derive(Debug, Clone)]
pub struct DbProxyEndpoint {
    pub db_proxy_endpoint_name: String,
    pub db_proxy_endpoint_arn: String,
    pub db_proxy_name: String,
    pub status: String,
    pub vpc_id: Option<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub vpc_subnet_ids: Vec<String>,
    pub endpoint: String,
    pub is_default: bool,
    pub target_role: String,
    pub created_date: Option<String>,
}
