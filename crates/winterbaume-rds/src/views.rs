//! Serde-compatible view types for RDS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::RdsService;
use crate::state::RdsState;
use crate::types::{
    BlueGreenDeployment, DbCluster, DbClusterEndpoint, DbClusterMember, DbClusterParameterGroup,
    DbClusterSnapshot, DbInstance, DbParameterGroup, DbProxy, DbProxyEndpoint, DbProxyTarget,
    DbProxyTargetGroup, DbSecurityGroup, DbShardGroup, DbSnapshot, DbSubnetGroup, EC2SecurityGroup,
    EventSubscription, ExportTask, GlobalCluster, IPRange, OptionGroup, Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RdsStateView {
    #[serde(default)]
    pub db_instances: HashMap<String, DbInstanceView>,
    #[serde(default)]
    pub db_clusters: HashMap<String, DbClusterView>,
    #[serde(default)]
    pub db_subnet_groups: HashMap<String, DbSubnetGroupView>,
    #[serde(default)]
    pub db_parameter_groups: HashMap<String, DbParameterGroupView>,
    #[serde(default)]
    pub db_cluster_parameter_groups: HashMap<String, DbClusterParameterGroupView>,
    #[serde(default)]
    pub db_snapshots: HashMap<String, DbSnapshotView>,
    #[serde(default)]
    pub db_cluster_snapshots: HashMap<String, DbClusterSnapshotView>,
    #[serde(default)]
    pub db_security_groups: HashMap<String, DbSecurityGroupView>,
    #[serde(default)]
    pub event_subscriptions: HashMap<String, EventSubscriptionView>,
    #[serde(default)]
    pub option_groups: HashMap<String, OptionGroupView>,
    #[serde(default)]
    pub global_clusters: HashMap<String, GlobalClusterView>,
    #[serde(default)]
    pub export_tasks: HashMap<String, ExportTaskView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, Vec<TagView>>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub db_proxies: HashMap<String, DbProxyView>,
    /// Keyed by "{proxy_name}/{target_group_name}" — matches state map key.
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub db_proxy_target_groups: HashMap<String, DbProxyTargetGroupView>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub blue_green_deployments: HashMap<String, BlueGreenDeploymentView>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub db_shard_groups: HashMap<String, DbShardGroupView>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub db_cluster_endpoints: HashMap<String, DbClusterEndpointView>,
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub db_proxy_endpoints: HashMap<String, DbProxyEndpointView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbInstanceView {
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
    pub tags: Vec<TagView>,
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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub blue_green_update: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub restore_to_point_in_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub s3_import: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterMemberView {
    pub db_instance_identifier: String,
    pub is_cluster_writer: bool,
    pub db_cluster_parameter_group_status: String,
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterView {
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
    pub tags: Vec<TagView>,
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
    pub members: Vec<DbClusterMemberView>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub restore_to_point_in_time: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub s3_import: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub scaling_configuration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub serverlessv2_scaling_configuration: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbSubnetGroupView {
    pub name: String,
    pub description: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub status: String,
    pub arn: String,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbParameterGroupView {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameter: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterParameterGroupView {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub parameter: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbSnapshotView {
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
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterSnapshotView {
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
    pub tags: Vec<TagView>,
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EC2SecurityGroupView {
    pub status: String,
    pub ec2_security_group_name: Option<String>,
    pub ec2_security_group_id: Option<String>,
    pub ec2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPRangeView {
    pub status: String,
    pub cidrip: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbSecurityGroupView {
    pub name: String,
    pub description: String,
    pub vpc_id: Option<String>,
    pub ec2_security_groups: Vec<EC2SecurityGroupView>,
    pub ip_ranges: Vec<IPRangeView>,
    pub arn: String,
    pub tags: Vec<TagView>,
    pub owner_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscriptionView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionGroupView {
    pub name: String,
    pub engine_name: String,
    pub major_engine_version: String,
    pub description: String,
    pub allows_vpc_and_non_vpc_instance_memberships: bool,
    pub vpc_id: Option<String>,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub option: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalClusterView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbProxyTargetView {
    pub target_arn: String,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub tracked_cluster_id: Option<String>,
    #[serde(default)]
    pub rds_resource_id: Option<String>,
    #[serde(default)]
    pub port: Option<i32>,
    #[serde(default)]
    pub type_: Option<String>,
    #[serde(default)]
    pub role: Option<String>,
    #[serde(default)]
    pub target_health_status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbProxyView {
    pub db_proxy_name: String,
    pub db_proxy_arn: String,
    pub status: String,
    pub engine_family: String,
    #[serde(default)]
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
    pub endpoint: String,
    pub require_tls: bool,
    pub idle_client_timeout: i32,
    pub debug_logging: bool,
    pub role_arn: String,
    #[serde(default)]
    pub created_date: Option<String>,
    #[serde(default)]
    pub updated_date: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
    #[serde(default)]
    pub targets: Vec<DbProxyTargetView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbProxyTargetGroupView {
    pub target_group_name: String,
    pub db_proxy_name: String,
    pub target_group_arn: String,
    pub is_default: bool,
    pub status: String,
    #[serde(default)]
    pub connection_pool_config: Option<String>,
    #[serde(default)]
    pub created_date: Option<String>,
    #[serde(default)]
    pub updated_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlueGreenDeploymentView {
    pub blue_green_deployment_identifier: String,
    pub blue_green_deployment_name: String,
    #[serde(default)]
    pub source: Option<String>,
    #[serde(default)]
    pub target: Option<String>,
    pub status: String,
    #[serde(default)]
    pub status_details: Option<String>,
    #[serde(default)]
    pub create_time: Option<String>,
    #[serde(default)]
    pub delete_time: Option<String>,
    #[serde(default)]
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbShardGroupView {
    pub db_shard_group_identifier: String,
    #[serde(default)]
    pub db_shard_group_resource_id: Option<String>,
    pub db_cluster_identifier: String,
    pub max_acu: f64,
    #[serde(default)]
    pub min_acu: Option<f64>,
    pub publicly_accessible: bool,
    pub status: String,
    #[serde(default)]
    pub endpoint: Option<String>,
    #[serde(default)]
    pub db_shard_group_arn: Option<String>,
    #[serde(default)]
    pub tag_list: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterEndpointView {
    pub db_cluster_endpoint_identifier: String,
    pub db_cluster_identifier: String,
    pub db_cluster_endpoint_arn: String,
    pub endpoint: String,
    pub status: String,
    pub endpoint_type: String,
    #[serde(default)]
    pub custom_endpoint_type: Option<String>,
    #[serde(default)]
    pub static_members: Vec<String>,
    #[serde(default)]
    pub excluded_members: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbProxyEndpointView {
    pub db_proxy_endpoint_name: String,
    pub db_proxy_endpoint_arn: String,
    pub db_proxy_name: String,
    pub status: String,
    #[serde(default)]
    pub vpc_id: Option<String>,
    #[serde(default)]
    pub vpc_security_group_ids: Vec<String>,
    #[serde(default)]
    pub vpc_subnet_ids: Vec<String>,
    pub endpoint: String,
    pub is_default: bool,
    pub target_role: String,
    #[serde(default)]
    pub created_date: Option<String>,
}

// --- From conversions ---

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        Self {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Self {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<&DbInstance> for DbInstanceView {
    fn from(i: &DbInstance) -> Self {
        Self {
            identifier: i.identifier.clone(),
            db_instance_class: i.db_instance_class.clone(),
            engine: i.engine.clone(),
            engine_version: i.engine_version.clone(),
            status: i.status.clone(),
            master_username: i.master_username.clone(),
            db_name: i.db_name.clone(),
            endpoint_address: i.endpoint_address.clone(),
            port: i.port,
            multi_az: i.multi_az,
            storage_type: i.storage_type.clone(),
            allocated_storage: i.allocated_storage,
            db_subnet_group_name: i.db_subnet_group_name.clone(),
            vpc_security_group_ids: i.vpc_security_group_ids.clone(),
            db_parameter_group_names: i.db_parameter_group_names.clone(),
            availability_zone: i.availability_zone.clone(),
            publicly_accessible: i.publicly_accessible,
            auto_minor_version_upgrade: i.auto_minor_version_upgrade,
            backup_retention_period: i.backup_retention_period,
            db_cluster_identifier: i.db_cluster_identifier.clone(),
            arn: i.arn.clone(),
            tags: i.tags.iter().map(TagView::from).collect(),
            instance_create_time: i.instance_create_time.clone(),
            license_model: i.license_model.clone(),
            iops: i.iops,
            deletion_protection: i.deletion_protection,
            copy_tags_to_snapshot: i.copy_tags_to_snapshot,
            monitoring_interval: i.monitoring_interval,
            performance_insights_enabled: i.performance_insights_enabled,
            storage_encrypted: i.storage_encrypted,
            kms_key_id: i.kms_key_id.clone(),
            ca_certificate_identifier: i.ca_certificate_identifier.clone(),
            secondary_availability_zone: i.secondary_availability_zone.clone(),
            blue_green_update: None,
            restore_to_point_in_time: None,
            s3_import: None,
        }
    }
}

impl From<DbInstanceView> for DbInstance {
    fn from(v: DbInstanceView) -> Self {
        Self {
            identifier: v.identifier,
            db_instance_class: v.db_instance_class,
            engine: v.engine,
            engine_version: v.engine_version,
            status: v.status,
            master_username: v.master_username,
            db_name: v.db_name,
            endpoint_address: v.endpoint_address,
            port: v.port,
            multi_az: v.multi_az,
            storage_type: v.storage_type,
            allocated_storage: v.allocated_storage,
            db_subnet_group_name: v.db_subnet_group_name,
            vpc_security_group_ids: v.vpc_security_group_ids,
            db_parameter_group_names: v.db_parameter_group_names,
            availability_zone: v.availability_zone,
            publicly_accessible: v.publicly_accessible,
            auto_minor_version_upgrade: v.auto_minor_version_upgrade,
            backup_retention_period: v.backup_retention_period,
            db_cluster_identifier: v.db_cluster_identifier,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            instance_create_time: v.instance_create_time,
            license_model: v.license_model,
            iops: v.iops,
            deletion_protection: v.deletion_protection,
            copy_tags_to_snapshot: v.copy_tags_to_snapshot,
            monitoring_interval: v.monitoring_interval,
            performance_insights_enabled: v.performance_insights_enabled,
            storage_encrypted: v.storage_encrypted,
            kms_key_id: v.kms_key_id,
            ca_certificate_identifier: v.ca_certificate_identifier,
            secondary_availability_zone: v.secondary_availability_zone,
            associated_roles: Vec::new(),
        }
    }
}

impl From<&DbClusterMember> for DbClusterMemberView {
    fn from(m: &DbClusterMember) -> Self {
        Self {
            db_instance_identifier: m.db_instance_identifier.clone(),
            is_cluster_writer: m.is_cluster_writer,
            db_cluster_parameter_group_status: m.db_cluster_parameter_group_status.clone(),
            promotion_tier: m.promotion_tier,
        }
    }
}

impl From<DbClusterMemberView> for DbClusterMember {
    fn from(v: DbClusterMemberView) -> Self {
        Self {
            db_instance_identifier: v.db_instance_identifier,
            is_cluster_writer: v.is_cluster_writer,
            db_cluster_parameter_group_status: v.db_cluster_parameter_group_status,
            promotion_tier: v.promotion_tier,
        }
    }
}

impl From<&DbCluster> for DbClusterView {
    fn from(c: &DbCluster) -> Self {
        Self {
            identifier: c.identifier.clone(),
            engine: c.engine.clone(),
            engine_version: c.engine_version.clone(),
            status: c.status.clone(),
            endpoint: c.endpoint.clone(),
            reader_endpoint: c.reader_endpoint.clone(),
            port: c.port,
            master_username: c.master_username.clone(),
            database_name: c.database_name.clone(),
            db_subnet_group_name: c.db_subnet_group_name.clone(),
            vpc_security_group_ids: c.vpc_security_group_ids.clone(),
            availability_zones: c.availability_zones.clone(),
            arn: c.arn.clone(),
            tags: c.tags.iter().map(TagView::from).collect(),
            cluster_create_time: c.cluster_create_time.clone(),
            multi_az: c.multi_az,
            storage_type: c.storage_type.clone(),
            allocated_storage: c.allocated_storage,
            backup_retention_period: c.backup_retention_period,
            deletion_protection: c.deletion_protection,
            storage_encrypted: c.storage_encrypted,
            kms_key_id: c.kms_key_id.clone(),
            db_cluster_parameter_group: c.db_cluster_parameter_group.clone(),
            engine_mode: c.engine_mode.clone(),
            copy_tags_to_snapshot: c.copy_tags_to_snapshot,
            members: c.members.iter().map(DbClusterMemberView::from).collect(),
            restore_to_point_in_time: None,
            s3_import: None,
            scaling_configuration: None,
            serverlessv2_scaling_configuration: None,
        }
    }
}

impl From<DbClusterView> for DbCluster {
    fn from(v: DbClusterView) -> Self {
        Self {
            identifier: v.identifier,
            engine: v.engine,
            engine_version: v.engine_version,
            status: v.status,
            endpoint: v.endpoint,
            reader_endpoint: v.reader_endpoint,
            port: v.port,
            master_username: v.master_username,
            database_name: v.database_name,
            db_subnet_group_name: v.db_subnet_group_name,
            vpc_security_group_ids: v.vpc_security_group_ids,
            availability_zones: v.availability_zones,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            cluster_create_time: v.cluster_create_time,
            multi_az: v.multi_az,
            storage_type: v.storage_type,
            allocated_storage: v.allocated_storage,
            backup_retention_period: v.backup_retention_period,
            deletion_protection: v.deletion_protection,
            storage_encrypted: v.storage_encrypted,
            kms_key_id: v.kms_key_id,
            db_cluster_parameter_group: v.db_cluster_parameter_group,
            engine_mode: v.engine_mode,
            copy_tags_to_snapshot: v.copy_tags_to_snapshot,
            members: v.members.into_iter().map(DbClusterMember::from).collect(),
            associated_roles: Vec::new(),
        }
    }
}

impl From<&DbSubnetGroup> for DbSubnetGroupView {
    fn from(sg: &DbSubnetGroup) -> Self {
        Self {
            name: sg.name.clone(),
            description: sg.description.clone(),
            vpc_id: sg.vpc_id.clone(),
            subnet_ids: sg.subnet_ids.clone(),
            status: sg.status.clone(),
            arn: sg.arn.clone(),
            tags: sg.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<DbSubnetGroupView> for DbSubnetGroup {
    fn from(v: DbSubnetGroupView) -> Self {
        Self {
            name: v.name,
            description: v.description,
            vpc_id: v.vpc_id,
            subnet_ids: v.subnet_ids,
            status: v.status,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbParameterGroup> for DbParameterGroupView {
    fn from(pg: &DbParameterGroup) -> Self {
        Self {
            name: pg.name.clone(),
            family: pg.family.clone(),
            description: pg.description.clone(),
            arn: pg.arn.clone(),
            tags: pg.tags.iter().map(TagView::from).collect(),
            parameter: Vec::new(),
        }
    }
}

impl From<DbParameterGroupView> for DbParameterGroup {
    fn from(v: DbParameterGroupView) -> Self {
        Self {
            name: v.name,
            family: v.family,
            description: v.description,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbClusterParameterGroup> for DbClusterParameterGroupView {
    fn from(pg: &DbClusterParameterGroup) -> Self {
        Self {
            name: pg.name.clone(),
            family: pg.family.clone(),
            description: pg.description.clone(),
            arn: pg.arn.clone(),
            tags: pg.tags.iter().map(TagView::from).collect(),
            parameter: Vec::new(),
        }
    }
}

impl From<DbClusterParameterGroupView> for DbClusterParameterGroup {
    fn from(v: DbClusterParameterGroupView) -> Self {
        Self {
            name: v.name,
            family: v.family,
            description: v.description,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbSnapshot> for DbSnapshotView {
    fn from(s: &DbSnapshot) -> Self {
        Self {
            identifier: s.identifier.clone(),
            db_instance_identifier: s.db_instance_identifier.clone(),
            engine: s.engine.clone(),
            engine_version: s.engine_version.clone(),
            allocated_storage: s.allocated_storage,
            status: s.status.clone(),
            port: s.port,
            availability_zone: s.availability_zone.clone(),
            vpc_id: s.vpc_id.clone(),
            instance_create_time: s.instance_create_time.clone(),
            master_username: s.master_username.clone(),
            snapshot_type: s.snapshot_type.clone(),
            iops: s.iops,
            option_group_name: s.option_group_name.clone(),
            percent_progress: s.percent_progress,
            source_region: s.source_region.clone(),
            source_db_snapshot_identifier: s.source_db_snapshot_identifier.clone(),
            storage_type: s.storage_type.clone(),
            tde_credential_arn: s.tde_credential_arn.clone(),
            encrypted: s.encrypted,
            kms_key_id: s.kms_key_id.clone(),
            db_snapshot_arn: s.db_snapshot_arn.clone(),
            timezone: s.timezone.clone(),
            db_instance_automated_backups_arn: s.db_instance_automated_backups_arn.clone(),
            snapshot_create_time: s.snapshot_create_time.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<DbSnapshotView> for DbSnapshot {
    fn from(v: DbSnapshotView) -> Self {
        Self {
            identifier: v.identifier,
            db_instance_identifier: v.db_instance_identifier,
            engine: v.engine,
            engine_version: v.engine_version,
            allocated_storage: v.allocated_storage,
            status: v.status,
            port: v.port,
            availability_zone: v.availability_zone,
            vpc_id: v.vpc_id,
            instance_create_time: v.instance_create_time,
            master_username: v.master_username,
            snapshot_type: v.snapshot_type,
            iops: v.iops,
            option_group_name: v.option_group_name,
            percent_progress: v.percent_progress,
            source_region: v.source_region,
            source_db_snapshot_identifier: v.source_db_snapshot_identifier,
            storage_type: v.storage_type,
            tde_credential_arn: v.tde_credential_arn,
            encrypted: v.encrypted,
            kms_key_id: v.kms_key_id,
            db_snapshot_arn: v.db_snapshot_arn,
            timezone: v.timezone,
            db_instance_automated_backups_arn: v.db_instance_automated_backups_arn,
            snapshot_create_time: v.snapshot_create_time,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbClusterSnapshot> for DbClusterSnapshotView {
    fn from(s: &DbClusterSnapshot) -> Self {
        Self {
            identifier: s.identifier.clone(),
            db_cluster_identifier: s.db_cluster_identifier.clone(),
            engine: s.engine.clone(),
            engine_version: s.engine_version.clone(),
            allocated_storage: s.allocated_storage,
            status: s.status.clone(),
            port: s.port,
            vpc_id: s.vpc_id.clone(),
            cluster_create_time: s.cluster_create_time.clone(),
            master_username: s.master_username.clone(),
            snapshot_type: s.snapshot_type.clone(),
            percent_progress: s.percent_progress,
            storage_encrypted: s.storage_encrypted,
            kms_key_id: s.kms_key_id.clone(),
            db_cluster_snapshot_arn: s.db_cluster_snapshot_arn.clone(),
            source_db_cluster_snapshot_arn: s.source_db_cluster_snapshot_arn.clone(),
            availability_zones: s.availability_zones.clone(),
            snapshot_create_time: s.snapshot_create_time.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
            storage_type: s.storage_type.clone(),
        }
    }
}

impl From<DbClusterSnapshotView> for DbClusterSnapshot {
    fn from(v: DbClusterSnapshotView) -> Self {
        Self {
            identifier: v.identifier,
            db_cluster_identifier: v.db_cluster_identifier,
            engine: v.engine,
            engine_version: v.engine_version,
            allocated_storage: v.allocated_storage,
            status: v.status,
            port: v.port,
            vpc_id: v.vpc_id,
            cluster_create_time: v.cluster_create_time,
            master_username: v.master_username,
            snapshot_type: v.snapshot_type,
            percent_progress: v.percent_progress,
            storage_encrypted: v.storage_encrypted,
            kms_key_id: v.kms_key_id,
            db_cluster_snapshot_arn: v.db_cluster_snapshot_arn,
            source_db_cluster_snapshot_arn: v.source_db_cluster_snapshot_arn,
            availability_zones: v.availability_zones,
            snapshot_create_time: v.snapshot_create_time,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            storage_type: v.storage_type,
        }
    }
}

impl From<&EC2SecurityGroup> for EC2SecurityGroupView {
    fn from(sg: &EC2SecurityGroup) -> Self {
        Self {
            status: sg.status.clone(),
            ec2_security_group_name: sg.ec2_security_group_name.clone(),
            ec2_security_group_id: sg.ec2_security_group_id.clone(),
            ec2_security_group_owner_id: sg.ec2_security_group_owner_id.clone(),
        }
    }
}

impl From<EC2SecurityGroupView> for EC2SecurityGroup {
    fn from(v: EC2SecurityGroupView) -> Self {
        Self {
            status: v.status,
            ec2_security_group_name: v.ec2_security_group_name,
            ec2_security_group_id: v.ec2_security_group_id,
            ec2_security_group_owner_id: v.ec2_security_group_owner_id,
        }
    }
}

impl From<&IPRange> for IPRangeView {
    fn from(r: &IPRange) -> Self {
        Self {
            status: r.status.clone(),
            cidrip: r.cidrip.clone(),
        }
    }
}

impl From<IPRangeView> for IPRange {
    fn from(v: IPRangeView) -> Self {
        Self {
            status: v.status,
            cidrip: v.cidrip,
        }
    }
}

impl From<&DbSecurityGroup> for DbSecurityGroupView {
    fn from(sg: &DbSecurityGroup) -> Self {
        Self {
            name: sg.name.clone(),
            description: sg.description.clone(),
            vpc_id: sg.vpc_id.clone(),
            ec2_security_groups: sg
                .ec2_security_groups
                .iter()
                .map(EC2SecurityGroupView::from)
                .collect(),
            ip_ranges: sg.ip_ranges.iter().map(IPRangeView::from).collect(),
            arn: sg.arn.clone(),
            tags: sg.tags.iter().map(TagView::from).collect(),
            owner_id: sg.owner_id.clone(),
        }
    }
}

impl From<DbSecurityGroupView> for DbSecurityGroup {
    fn from(v: DbSecurityGroupView) -> Self {
        Self {
            name: v.name,
            description: v.description,
            vpc_id: v.vpc_id,
            ec2_security_groups: v
                .ec2_security_groups
                .into_iter()
                .map(EC2SecurityGroup::from)
                .collect(),
            ip_ranges: v.ip_ranges.into_iter().map(IPRange::from).collect(),
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            owner_id: v.owner_id,
        }
    }
}

impl From<&EventSubscription> for EventSubscriptionView {
    fn from(s: &EventSubscription) -> Self {
        Self {
            subscription_name: s.subscription_name.clone(),
            sns_topic_arn: s.sns_topic_arn.clone(),
            source_type: s.source_type.clone(),
            source_ids: s.source_ids.clone(),
            event_categories: s.event_categories.clone(),
            enabled: s.enabled,
            status: s.status.clone(),
            arn: s.arn.clone(),
            customer_aws_id: s.customer_aws_id.clone(),
            subscription_creation_time: s.subscription_creation_time.clone(),
        }
    }
}

impl From<EventSubscriptionView> for EventSubscription {
    fn from(v: EventSubscriptionView) -> Self {
        Self {
            subscription_name: v.subscription_name,
            sns_topic_arn: v.sns_topic_arn,
            source_type: v.source_type,
            source_ids: v.source_ids,
            event_categories: v.event_categories,
            enabled: v.enabled,
            status: v.status,
            arn: v.arn,
            customer_aws_id: v.customer_aws_id,
            subscription_creation_time: v.subscription_creation_time,
        }
    }
}

impl From<&OptionGroup> for OptionGroupView {
    fn from(og: &OptionGroup) -> Self {
        Self {
            name: og.name.clone(),
            engine_name: og.engine_name.clone(),
            major_engine_version: og.major_engine_version.clone(),
            description: og.description.clone(),
            allows_vpc_and_non_vpc_instance_memberships: og
                .allows_vpc_and_non_vpc_instance_memberships,
            vpc_id: og.vpc_id.clone(),
            arn: og.arn.clone(),
            tags: og.tags.iter().map(TagView::from).collect(),
            option: Vec::new(),
        }
    }
}

impl From<OptionGroupView> for OptionGroup {
    fn from(v: OptionGroupView) -> Self {
        Self {
            name: v.name,
            engine_name: v.engine_name,
            major_engine_version: v.major_engine_version,
            description: v.description,
            allows_vpc_and_non_vpc_instance_memberships: v
                .allows_vpc_and_non_vpc_instance_memberships,
            vpc_id: v.vpc_id,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&GlobalCluster> for GlobalClusterView {
    fn from(gc: &GlobalCluster) -> Self {
        Self {
            global_cluster_identifier: gc.global_cluster_identifier.clone(),
            global_cluster_resource_id: gc.global_cluster_resource_id.clone(),
            global_cluster_arn: gc.global_cluster_arn.clone(),
            status: gc.status.clone(),
            engine: gc.engine.clone(),
            engine_version: gc.engine_version.clone(),
            database_name: gc.database_name.clone(),
            storage_encrypted: gc.storage_encrypted,
            deletion_protection: gc.deletion_protection,
        }
    }
}

impl From<GlobalClusterView> for GlobalCluster {
    fn from(v: GlobalClusterView) -> Self {
        Self {
            global_cluster_identifier: v.global_cluster_identifier,
            global_cluster_resource_id: v.global_cluster_resource_id,
            global_cluster_arn: v.global_cluster_arn,
            status: v.status,
            engine: v.engine,
            engine_version: v.engine_version,
            database_name: v.database_name,
            storage_encrypted: v.storage_encrypted,
            deletion_protection: v.deletion_protection,
        }
    }
}

impl From<&ExportTask> for ExportTaskView {
    fn from(t: &ExportTask) -> Self {
        Self {
            export_task_identifier: t.export_task_identifier.clone(),
            source_arn: t.source_arn.clone(),
            export_only: t.export_only.clone(),
            source_type: t.source_type.clone(),
            snapshot_time: t.snapshot_time.clone(),
            task_start_time: t.task_start_time.clone(),
            task_end_time: t.task_end_time.clone(),
            s3_bucket: t.s3_bucket.clone(),
            s3_prefix: t.s3_prefix.clone(),
            iam_role_arn: t.iam_role_arn.clone(),
            kms_key_id: t.kms_key_id.clone(),
            status: t.status.clone(),
            percent_progress: t.percent_progress,
            total_extracted_data_in_gb: t.total_extracted_data_in_gb,
            failure_cause: t.failure_cause.clone(),
            warning_message: t.warning_message.clone(),
        }
    }
}

impl From<ExportTaskView> for ExportTask {
    fn from(v: ExportTaskView) -> Self {
        Self {
            export_task_identifier: v.export_task_identifier,
            source_arn: v.source_arn,
            export_only: v.export_only,
            source_type: v.source_type,
            snapshot_time: v.snapshot_time,
            task_start_time: v.task_start_time,
            task_end_time: v.task_end_time,
            s3_bucket: v.s3_bucket,
            s3_prefix: v.s3_prefix,
            iam_role_arn: v.iam_role_arn,
            kms_key_id: v.kms_key_id,
            status: v.status,
            percent_progress: v.percent_progress,
            total_extracted_data_in_gb: v.total_extracted_data_in_gb,
            failure_cause: v.failure_cause,
            warning_message: v.warning_message,
        }
    }
}

impl From<&DbProxyTarget> for DbProxyTargetView {
    fn from(t: &DbProxyTarget) -> Self {
        Self {
            target_arn: t.target_arn.clone(),
            endpoint: t.endpoint.clone(),
            tracked_cluster_id: t.tracked_cluster_id.clone(),
            rds_resource_id: t.rds_resource_id.clone(),
            port: t.port,
            type_: t.type_.clone(),
            role: t.role.clone(),
            target_health_status: t.target_health_status.clone(),
        }
    }
}

impl From<DbProxyTargetView> for DbProxyTarget {
    fn from(v: DbProxyTargetView) -> Self {
        Self {
            target_arn: v.target_arn,
            endpoint: v.endpoint,
            tracked_cluster_id: v.tracked_cluster_id,
            rds_resource_id: v.rds_resource_id,
            port: v.port,
            type_: v.type_,
            role: v.role,
            target_health_status: v.target_health_status,
        }
    }
}

impl From<&DbProxy> for DbProxyView {
    fn from(p: &DbProxy) -> Self {
        Self {
            db_proxy_name: p.db_proxy_name.clone(),
            db_proxy_arn: p.db_proxy_arn.clone(),
            status: p.status.clone(),
            engine_family: p.engine_family.clone(),
            vpc_id: p.vpc_id.clone(),
            vpc_security_group_ids: p.vpc_security_group_ids.clone(),
            vpc_subnet_ids: p.vpc_subnet_ids.clone(),
            endpoint: p.endpoint.clone(),
            require_tls: p.require_tls,
            idle_client_timeout: p.idle_client_timeout,
            debug_logging: p.debug_logging,
            role_arn: p.role_arn.clone(),
            created_date: p.created_date.clone(),
            updated_date: p.updated_date.clone(),
            tags: p.tags.iter().map(TagView::from).collect(),
            targets: p.targets.iter().map(DbProxyTargetView::from).collect(),
        }
    }
}

impl From<DbProxyView> for DbProxy {
    fn from(v: DbProxyView) -> Self {
        Self {
            db_proxy_name: v.db_proxy_name,
            db_proxy_arn: v.db_proxy_arn,
            status: v.status,
            engine_family: v.engine_family,
            vpc_id: v.vpc_id,
            vpc_security_group_ids: v.vpc_security_group_ids,
            vpc_subnet_ids: v.vpc_subnet_ids,
            endpoint: v.endpoint,
            require_tls: v.require_tls,
            idle_client_timeout: v.idle_client_timeout,
            debug_logging: v.debug_logging,
            role_arn: v.role_arn,
            created_date: v.created_date,
            updated_date: v.updated_date,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            targets: v.targets.into_iter().map(DbProxyTarget::from).collect(),
        }
    }
}

impl From<&DbProxyTargetGroup> for DbProxyTargetGroupView {
    fn from(g: &DbProxyTargetGroup) -> Self {
        Self {
            target_group_name: g.target_group_name.clone(),
            db_proxy_name: g.db_proxy_name.clone(),
            target_group_arn: g.target_group_arn.clone(),
            is_default: g.is_default,
            status: g.status.clone(),
            connection_pool_config: g.connection_pool_config.clone(),
            created_date: g.created_date.clone(),
            updated_date: g.updated_date.clone(),
        }
    }
}

impl From<DbProxyTargetGroupView> for DbProxyTargetGroup {
    fn from(v: DbProxyTargetGroupView) -> Self {
        Self {
            target_group_name: v.target_group_name,
            db_proxy_name: v.db_proxy_name,
            target_group_arn: v.target_group_arn,
            is_default: v.is_default,
            status: v.status,
            connection_pool_config: v.connection_pool_config,
            created_date: v.created_date,
            updated_date: v.updated_date,
        }
    }
}

impl From<&BlueGreenDeployment> for BlueGreenDeploymentView {
    fn from(d: &BlueGreenDeployment) -> Self {
        Self {
            blue_green_deployment_identifier: d.blue_green_deployment_identifier.clone(),
            blue_green_deployment_name: d.blue_green_deployment_name.clone(),
            source: d.source.clone(),
            target: d.target.clone(),
            status: d.status.clone(),
            status_details: d.status_details.clone(),
            create_time: d.create_time.clone(),
            delete_time: d.delete_time.clone(),
            tags: d.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<BlueGreenDeploymentView> for BlueGreenDeployment {
    fn from(v: BlueGreenDeploymentView) -> Self {
        Self {
            blue_green_deployment_identifier: v.blue_green_deployment_identifier,
            blue_green_deployment_name: v.blue_green_deployment_name,
            source: v.source,
            target: v.target,
            status: v.status,
            status_details: v.status_details,
            create_time: v.create_time,
            delete_time: v.delete_time,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbShardGroup> for DbShardGroupView {
    fn from(g: &DbShardGroup) -> Self {
        Self {
            db_shard_group_identifier: g.db_shard_group_identifier.clone(),
            db_shard_group_resource_id: g.db_shard_group_resource_id.clone(),
            db_cluster_identifier: g.db_cluster_identifier.clone(),
            max_acu: g.max_acu,
            min_acu: g.min_acu,
            publicly_accessible: g.publicly_accessible,
            status: g.status.clone(),
            endpoint: g.endpoint.clone(),
            db_shard_group_arn: g.db_shard_group_arn.clone(),
            tag_list: g.tag_list.iter().map(TagView::from).collect(),
        }
    }
}

impl From<DbShardGroupView> for DbShardGroup {
    fn from(v: DbShardGroupView) -> Self {
        Self {
            db_shard_group_identifier: v.db_shard_group_identifier,
            db_shard_group_resource_id: v.db_shard_group_resource_id,
            db_cluster_identifier: v.db_cluster_identifier,
            max_acu: v.max_acu,
            min_acu: v.min_acu,
            publicly_accessible: v.publicly_accessible,
            status: v.status,
            endpoint: v.endpoint,
            db_shard_group_arn: v.db_shard_group_arn,
            tag_list: v.tag_list.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&DbClusterEndpoint> for DbClusterEndpointView {
    fn from(e: &DbClusterEndpoint) -> Self {
        Self {
            db_cluster_endpoint_identifier: e.db_cluster_endpoint_identifier.clone(),
            db_cluster_identifier: e.db_cluster_identifier.clone(),
            db_cluster_endpoint_arn: e.db_cluster_endpoint_arn.clone(),
            endpoint: e.endpoint.clone(),
            status: e.status.clone(),
            endpoint_type: e.endpoint_type.clone(),
            custom_endpoint_type: e.custom_endpoint_type.clone(),
            static_members: e.static_members.clone(),
            excluded_members: e.excluded_members.clone(),
        }
    }
}

impl From<DbClusterEndpointView> for DbClusterEndpoint {
    fn from(v: DbClusterEndpointView) -> Self {
        Self {
            db_cluster_endpoint_identifier: v.db_cluster_endpoint_identifier,
            db_cluster_identifier: v.db_cluster_identifier,
            db_cluster_endpoint_arn: v.db_cluster_endpoint_arn,
            endpoint: v.endpoint,
            status: v.status,
            endpoint_type: v.endpoint_type,
            custom_endpoint_type: v.custom_endpoint_type,
            static_members: v.static_members,
            excluded_members: v.excluded_members,
        }
    }
}

impl From<&DbProxyEndpoint> for DbProxyEndpointView {
    fn from(e: &DbProxyEndpoint) -> Self {
        Self {
            db_proxy_endpoint_name: e.db_proxy_endpoint_name.clone(),
            db_proxy_endpoint_arn: e.db_proxy_endpoint_arn.clone(),
            db_proxy_name: e.db_proxy_name.clone(),
            status: e.status.clone(),
            vpc_id: e.vpc_id.clone(),
            vpc_security_group_ids: e.vpc_security_group_ids.clone(),
            vpc_subnet_ids: e.vpc_subnet_ids.clone(),
            endpoint: e.endpoint.clone(),
            is_default: e.is_default,
            target_role: e.target_role.clone(),
            created_date: e.created_date.clone(),
        }
    }
}

impl From<DbProxyEndpointView> for DbProxyEndpoint {
    fn from(v: DbProxyEndpointView) -> Self {
        Self {
            db_proxy_endpoint_name: v.db_proxy_endpoint_name,
            db_proxy_endpoint_arn: v.db_proxy_endpoint_arn,
            db_proxy_name: v.db_proxy_name,
            status: v.status,
            vpc_id: v.vpc_id,
            vpc_security_group_ids: v.vpc_security_group_ids,
            vpc_subnet_ids: v.vpc_subnet_ids,
            endpoint: v.endpoint,
            is_default: v.is_default,
            target_role: v.target_role,
            created_date: v.created_date,
        }
    }
}

impl From<&RdsState> for RdsStateView {
    fn from(s: &RdsState) -> Self {
        Self {
            db_instances: s
                .db_instances
                .iter()
                .map(|(k, v)| (k.clone(), DbInstanceView::from(v)))
                .collect(),
            db_clusters: s
                .db_clusters
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterView::from(v)))
                .collect(),
            db_subnet_groups: s
                .db_subnet_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbSubnetGroupView::from(v)))
                .collect(),
            db_parameter_groups: s
                .db_parameter_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbParameterGroupView::from(v)))
                .collect(),
            db_cluster_parameter_groups: s
                .db_cluster_parameter_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterParameterGroupView::from(v)))
                .collect(),
            db_snapshots: s
                .db_snapshots
                .iter()
                .map(|(k, v)| (k.clone(), DbSnapshotView::from(v)))
                .collect(),
            db_cluster_snapshots: s
                .db_cluster_snapshots
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterSnapshotView::from(v)))
                .collect(),
            db_security_groups: s
                .db_security_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbSecurityGroupView::from(v)))
                .collect(),
            event_subscriptions: s
                .event_subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), EventSubscriptionView::from(v)))
                .collect(),
            option_groups: s
                .option_groups
                .iter()
                .map(|(k, v)| (k.clone(), OptionGroupView::from(v)))
                .collect(),
            global_clusters: s
                .global_clusters
                .iter()
                .map(|(k, v)| (k.clone(), GlobalClusterView::from(v)))
                .collect(),
            export_tasks: s
                .export_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ExportTaskView::from(v)))
                .collect(),
            resource_tags: s
                .resource_tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TagView::from).collect()))
                .collect(),
            db_proxies: s
                .db_proxies
                .iter()
                .map(|(k, v)| (k.clone(), DbProxyView::from(v)))
                .collect(),
            db_proxy_target_groups: s
                .db_proxy_target_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbProxyTargetGroupView::from(v)))
                .collect(),
            blue_green_deployments: s
                .blue_green_deployments
                .iter()
                .map(|(k, v)| (k.clone(), BlueGreenDeploymentView::from(v)))
                .collect(),
            db_shard_groups: s
                .db_shard_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbShardGroupView::from(v)))
                .collect(),
            db_cluster_endpoints: s
                .db_cluster_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterEndpointView::from(v)))
                .collect(),
            db_proxy_endpoints: s
                .db_proxy_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), DbProxyEndpointView::from(v)))
                .collect(),
        }
    }
}

impl From<RdsStateView> for RdsState {
    fn from(v: RdsStateView) -> Self {
        Self {
            db_instances: v
                .db_instances
                .into_iter()
                .map(|(k, iv)| (k, DbInstance::from(iv)))
                .collect(),
            db_clusters: v
                .db_clusters
                .into_iter()
                .map(|(k, cv)| (k, DbCluster::from(cv)))
                .collect(),
            db_subnet_groups: v
                .db_subnet_groups
                .into_iter()
                .map(|(k, sv)| (k, DbSubnetGroup::from(sv)))
                .collect(),
            db_parameter_groups: v
                .db_parameter_groups
                .into_iter()
                .map(|(k, pv)| (k, DbParameterGroup::from(pv)))
                .collect(),
            db_cluster_parameter_groups: v
                .db_cluster_parameter_groups
                .into_iter()
                .map(|(k, pv)| (k, DbClusterParameterGroup::from(pv)))
                .collect(),
            db_snapshots: v
                .db_snapshots
                .into_iter()
                .map(|(k, sv)| (k, DbSnapshot::from(sv)))
                .collect(),
            db_cluster_snapshots: v
                .db_cluster_snapshots
                .into_iter()
                .map(|(k, sv)| (k, DbClusterSnapshot::from(sv)))
                .collect(),
            db_security_groups: v
                .db_security_groups
                .into_iter()
                .map(|(k, sgv)| (k, DbSecurityGroup::from(sgv)))
                .collect(),
            event_subscriptions: v
                .event_subscriptions
                .into_iter()
                .map(|(k, sv)| (k, EventSubscription::from(sv)))
                .collect(),
            option_groups: v
                .option_groups
                .into_iter()
                .map(|(k, ogv)| (k, OptionGroup::from(ogv)))
                .collect(),
            global_clusters: v
                .global_clusters
                .into_iter()
                .map(|(k, gcv)| (k, GlobalCluster::from(gcv)))
                .collect(),
            export_tasks: v
                .export_tasks
                .into_iter()
                .map(|(k, tv)| (k, ExportTask::from(tv)))
                .collect(),
            resource_tags: v
                .resource_tags
                .into_iter()
                .map(|(k, tags)| (k, tags.into_iter().map(Tag::from).collect()))
                .collect(),
            db_proxies: v
                .db_proxies
                .into_iter()
                .map(|(k, pv)| (k, DbProxy::from(pv)))
                .collect(),
            db_proxy_target_groups: v
                .db_proxy_target_groups
                .into_iter()
                .map(|(k, gv)| (k, DbProxyTargetGroup::from(gv)))
                .collect(),
            blue_green_deployments: v
                .blue_green_deployments
                .into_iter()
                .map(|(k, dv)| (k, BlueGreenDeployment::from(dv)))
                .collect(),
            db_shard_groups: v
                .db_shard_groups
                .into_iter()
                .map(|(k, gv)| (k, DbShardGroup::from(gv)))
                .collect(),
            db_cluster_endpoints: v
                .db_cluster_endpoints
                .into_iter()
                .map(|(k, ev)| (k, DbClusterEndpoint::from(ev)))
                .collect(),
            db_proxy_endpoints: v
                .db_proxy_endpoints
                .into_iter()
                .map(|(k, ev)| (k, DbProxyEndpoint::from(ev)))
                .collect(),
        }
    }
}

impl StatefulService for RdsService {
    type StateView = RdsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        RdsStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = RdsState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in view.db_instances {
                guard.db_instances.insert(k, DbInstance::from(v));
            }
            for (k, v) in view.db_clusters {
                guard.db_clusters.insert(k, DbCluster::from(v));
            }
            for (k, v) in view.db_subnet_groups {
                guard.db_subnet_groups.insert(k, DbSubnetGroup::from(v));
            }
            for (k, v) in view.db_parameter_groups {
                guard
                    .db_parameter_groups
                    .insert(k, DbParameterGroup::from(v));
            }
            for (k, v) in view.db_cluster_parameter_groups {
                guard
                    .db_cluster_parameter_groups
                    .insert(k, DbClusterParameterGroup::from(v));
            }
            for (k, v) in view.db_snapshots {
                guard.db_snapshots.insert(k, DbSnapshot::from(v));
            }
            for (k, v) in view.db_cluster_snapshots {
                guard
                    .db_cluster_snapshots
                    .insert(k, DbClusterSnapshot::from(v));
            }
            for (k, v) in view.db_security_groups {
                guard.db_security_groups.insert(k, DbSecurityGroup::from(v));
            }
            for (k, v) in view.event_subscriptions {
                guard
                    .event_subscriptions
                    .insert(k, EventSubscription::from(v));
            }
            for (k, v) in view.option_groups {
                guard.option_groups.insert(k, OptionGroup::from(v));
            }
            for (k, v) in view.global_clusters {
                guard.global_clusters.insert(k, GlobalCluster::from(v));
            }
            for (k, v) in view.export_tasks {
                guard.export_tasks.insert(k, ExportTask::from(v));
            }
            for (k, v) in view.resource_tags {
                guard
                    .resource_tags
                    .insert(k, v.into_iter().map(Tag::from).collect());
            }
            for (k, v) in view.db_proxies {
                guard.db_proxies.insert(k, DbProxy::from(v));
            }
            for (k, v) in view.db_proxy_target_groups {
                guard
                    .db_proxy_target_groups
                    .insert(k, DbProxyTargetGroup::from(v));
            }
            for (k, v) in view.blue_green_deployments {
                guard
                    .blue_green_deployments
                    .insert(k, BlueGreenDeployment::from(v));
            }
            for (k, v) in view.db_shard_groups {
                guard.db_shard_groups.insert(k, DbShardGroup::from(v));
            }
            for (k, v) in view.db_cluster_endpoints {
                guard
                    .db_cluster_endpoints
                    .insert(k, DbClusterEndpoint::from(v));
            }
            for (k, v) in view.db_proxy_endpoints {
                guard.db_proxy_endpoints.insert(k, DbProxyEndpoint::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
