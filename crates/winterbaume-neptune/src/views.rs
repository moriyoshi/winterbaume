//! Serde-compatible view types for Neptune state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::NeptuneService;
use crate::state::NeptuneState;
use crate::types::{
    DbCluster, DbClusterEndpoint, DbClusterMember, DbClusterParameterGroup, DbClusterSnapshot,
    DbInstance, DbParameterGroup, DbSubnetGroup, EventSubscription, GlobalCluster, Parameter,
    ServerlessV2ScalingConfiguration, SnapshotAttribute, Tag,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NeptuneStateView {
    #[serde(default)]
    pub db_clusters: HashMap<String, DbClusterView>,
    #[serde(default)]
    pub db_instances: HashMap<String, DbInstanceView>,
    #[serde(default)]
    pub db_subnet_groups: HashMap<String, DbSubnetGroupView>,
    #[serde(default)]
    pub db_parameter_groups: HashMap<String, DbParameterGroupView>,
    #[serde(default)]
    pub db_cluster_parameter_groups: HashMap<String, DbClusterParameterGroupView>,
    #[serde(default)]
    pub db_cluster_snapshots: HashMap<String, DbClusterSnapshotView>,
    #[serde(default)]
    pub global_clusters: HashMap<String, GlobalClusterView>,
    #[serde(default)]
    pub db_cluster_endpoints: HashMap<String, DbClusterEndpointView>,
    #[serde(default)]
    pub event_subscriptions: HashMap<String, EventSubscriptionView>,
    #[serde(default)]
    pub snapshot_attributes: HashMap<String, Vec<SnapshotAttributeView>>,
    #[serde(default)]
    pub resource_tags: HashMap<String, Vec<TagView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterMemberView {
    pub db_instance_identifier: String,
    pub is_cluster_writer: bool,
    pub db_cluster_parameter_group_status: String,
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerlessV2ScalingConfigurationView {
    pub min_capacity: f64,
    pub max_capacity: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterView {
    pub name: String,
    pub value: String,
    pub apply_method: Option<String>,
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
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub db_cluster_parameter_group: Option<String>,
    pub engine_mode: Option<String>,
    pub copy_tags_to_snapshot: bool,
    pub deletion_protection: bool,
    pub backup_retention_period: i32,
    pub members: Vec<DbClusterMemberView>,
    pub associated_roles: Vec<String>,
    #[serde(default)]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbInstanceView {
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
    pub tags: Vec<TagView>,
    pub instance_create_time: Option<String>,
    pub storage_encrypted: bool,
    pub kms_key_id: Option<String>,
    pub publicly_accessible: bool,
    pub deletion_protection: bool,
    pub db_parameter_group_names: Vec<String>,
    pub multi_az: bool,
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
    #[serde(default)]
    pub parameters: Vec<ParameterView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterParameterGroupView {
    pub name: String,
    pub family: String,
    pub description: String,
    pub arn: String,
    pub tags: Vec<TagView>,
    #[serde(default)]
    pub parameters: Vec<ParameterView>,
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
    pub availability_zones: Vec<String>,
    pub snapshot_create_time: Option<String>,
    pub tags: Vec<TagView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalClusterView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbClusterEndpointView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscriptionView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotAttributeView {
    pub attribute_name: String,
    pub attribute_values: Vec<String>,
}

// -------------------------------------------------------------------------
// Conversions: internal types <-> view types
// -------------------------------------------------------------------------

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

impl From<&ServerlessV2ScalingConfiguration> for ServerlessV2ScalingConfigurationView {
    fn from(c: &ServerlessV2ScalingConfiguration) -> Self {
        Self {
            min_capacity: c.min_capacity,
            max_capacity: c.max_capacity,
        }
    }
}

impl From<ServerlessV2ScalingConfigurationView> for ServerlessV2ScalingConfiguration {
    fn from(v: ServerlessV2ScalingConfigurationView) -> Self {
        Self {
            min_capacity: v.min_capacity,
            max_capacity: v.max_capacity,
        }
    }
}

impl From<&Parameter> for ParameterView {
    fn from(p: &Parameter) -> Self {
        Self {
            name: p.name.clone(),
            value: p.value.clone(),
            apply_method: p.apply_method.clone(),
        }
    }
}

impl From<ParameterView> for Parameter {
    fn from(v: ParameterView) -> Self {
        Self {
            name: v.name,
            value: v.value,
            apply_method: v.apply_method,
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
            storage_encrypted: c.storage_encrypted,
            kms_key_id: c.kms_key_id.clone(),
            db_cluster_parameter_group: c.db_cluster_parameter_group.clone(),
            engine_mode: c.engine_mode.clone(),
            copy_tags_to_snapshot: c.copy_tags_to_snapshot,
            deletion_protection: c.deletion_protection,
            backup_retention_period: c.backup_retention_period,
            members: c.members.iter().map(DbClusterMemberView::from).collect(),
            associated_roles: c.associated_roles.clone(),
            serverless_v2_scaling_configuration: c
                .serverless_v2_scaling_configuration
                .as_ref()
                .map(ServerlessV2ScalingConfigurationView::from),
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
            storage_encrypted: v.storage_encrypted,
            kms_key_id: v.kms_key_id,
            db_cluster_parameter_group: v.db_cluster_parameter_group,
            engine_mode: v.engine_mode,
            copy_tags_to_snapshot: v.copy_tags_to_snapshot,
            deletion_protection: v.deletion_protection,
            backup_retention_period: v.backup_retention_period,
            members: v.members.into_iter().map(DbClusterMember::from).collect(),
            associated_roles: v.associated_roles,
            serverless_v2_scaling_configuration: v
                .serverless_v2_scaling_configuration
                .map(ServerlessV2ScalingConfiguration::from),
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
            endpoint_address: i.endpoint_address.clone(),
            port: i.port,
            db_subnet_group_name: i.db_subnet_group_name.clone(),
            vpc_security_group_ids: i.vpc_security_group_ids.clone(),
            availability_zone: i.availability_zone.clone(),
            auto_minor_version_upgrade: i.auto_minor_version_upgrade,
            backup_retention_period: i.backup_retention_period,
            db_cluster_identifier: i.db_cluster_identifier.clone(),
            arn: i.arn.clone(),
            tags: i.tags.iter().map(TagView::from).collect(),
            instance_create_time: i.instance_create_time.clone(),
            storage_encrypted: i.storage_encrypted,
            kms_key_id: i.kms_key_id.clone(),
            publicly_accessible: i.publicly_accessible,
            deletion_protection: i.deletion_protection,
            db_parameter_group_names: i.db_parameter_group_names.clone(),
            multi_az: i.multi_az,
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
            endpoint_address: v.endpoint_address,
            port: v.port,
            db_subnet_group_name: v.db_subnet_group_name,
            vpc_security_group_ids: v.vpc_security_group_ids,
            availability_zone: v.availability_zone,
            auto_minor_version_upgrade: v.auto_minor_version_upgrade,
            backup_retention_period: v.backup_retention_period,
            db_cluster_identifier: v.db_cluster_identifier,
            arn: v.arn,
            tags: v.tags.into_iter().map(Tag::from).collect(),
            instance_create_time: v.instance_create_time,
            storage_encrypted: v.storage_encrypted,
            kms_key_id: v.kms_key_id,
            publicly_accessible: v.publicly_accessible,
            deletion_protection: v.deletion_protection,
            db_parameter_group_names: v.db_parameter_group_names,
            multi_az: v.multi_az,
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
            parameters: pg.parameters.iter().map(ParameterView::from).collect(),
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
            parameters: v.parameters.into_iter().map(Parameter::from).collect(),
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
            parameters: pg.parameters.iter().map(ParameterView::from).collect(),
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
            parameters: v.parameters.into_iter().map(Parameter::from).collect(),
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
            availability_zones: s.availability_zones.clone(),
            snapshot_create_time: s.snapshot_create_time.clone(),
            tags: s.tags.iter().map(TagView::from).collect(),
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
            availability_zones: v.availability_zones,
            snapshot_create_time: v.snapshot_create_time,
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<&GlobalCluster> for GlobalClusterView {
    fn from(gc: &GlobalCluster) -> Self {
        Self {
            identifier: gc.identifier.clone(),
            engine: gc.engine.clone(),
            engine_version: gc.engine_version.clone(),
            database_name: gc.database_name.clone(),
            deletion_protection: gc.deletion_protection,
            storage_encrypted: gc.storage_encrypted,
            status: gc.status.clone(),
            arn: gc.arn.clone(),
            source_db_cluster_identifier: gc.source_db_cluster_identifier.clone(),
        }
    }
}

impl From<GlobalClusterView> for GlobalCluster {
    fn from(v: GlobalClusterView) -> Self {
        Self {
            identifier: v.identifier,
            engine: v.engine,
            engine_version: v.engine_version,
            database_name: v.database_name,
            deletion_protection: v.deletion_protection,
            storage_encrypted: v.storage_encrypted,
            status: v.status,
            arn: v.arn,
            source_db_cluster_identifier: v.source_db_cluster_identifier,
        }
    }
}

impl From<&DbClusterEndpoint> for DbClusterEndpointView {
    fn from(ep: &DbClusterEndpoint) -> Self {
        Self {
            identifier: ep.identifier.clone(),
            db_cluster_identifier: ep.db_cluster_identifier.clone(),
            endpoint_type: ep.endpoint_type.clone(),
            custom_endpoint_type: ep.custom_endpoint_type.clone(),
            endpoint: ep.endpoint.clone(),
            arn: ep.arn.clone(),
            resource_identifier: ep.resource_identifier.clone(),
            status: ep.status.clone(),
            static_members: ep.static_members.clone(),
            excluded_members: ep.excluded_members.clone(),
        }
    }
}

impl From<DbClusterEndpointView> for DbClusterEndpoint {
    fn from(v: DbClusterEndpointView) -> Self {
        Self {
            identifier: v.identifier,
            db_cluster_identifier: v.db_cluster_identifier,
            endpoint_type: v.endpoint_type,
            custom_endpoint_type: v.custom_endpoint_type,
            endpoint: v.endpoint,
            arn: v.arn,
            resource_identifier: v.resource_identifier,
            status: v.status,
            static_members: v.static_members,
            excluded_members: v.excluded_members,
        }
    }
}

impl From<&EventSubscription> for EventSubscriptionView {
    fn from(s: &EventSubscription) -> Self {
        Self {
            subscription_name: s.subscription_name.clone(),
            sns_topic_arn: s.sns_topic_arn.clone(),
            source_type: s.source_type.clone(),
            enabled: s.enabled,
            event_categories: s.event_categories.clone(),
            source_ids: s.source_ids.clone(),
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
            enabled: v.enabled,
            event_categories: v.event_categories,
            source_ids: v.source_ids,
            status: v.status,
            arn: v.arn,
            customer_aws_id: v.customer_aws_id,
            subscription_creation_time: v.subscription_creation_time,
        }
    }
}

impl From<&SnapshotAttribute> for SnapshotAttributeView {
    fn from(a: &SnapshotAttribute) -> Self {
        Self {
            attribute_name: a.attribute_name.clone(),
            attribute_values: a.attribute_values.clone(),
        }
    }
}

impl From<SnapshotAttributeView> for SnapshotAttribute {
    fn from(v: SnapshotAttributeView) -> Self {
        Self {
            attribute_name: v.attribute_name,
            attribute_values: v.attribute_values,
        }
    }
}

impl From<&NeptuneState> for NeptuneStateView {
    fn from(state: &NeptuneState) -> Self {
        Self {
            db_clusters: state
                .db_clusters
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterView::from(v)))
                .collect(),
            db_instances: state
                .db_instances
                .iter()
                .map(|(k, v)| (k.clone(), DbInstanceView::from(v)))
                .collect(),
            db_subnet_groups: state
                .db_subnet_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbSubnetGroupView::from(v)))
                .collect(),
            db_parameter_groups: state
                .db_parameter_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbParameterGroupView::from(v)))
                .collect(),
            db_cluster_parameter_groups: state
                .db_cluster_parameter_groups
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterParameterGroupView::from(v)))
                .collect(),
            db_cluster_snapshots: state
                .db_cluster_snapshots
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterSnapshotView::from(v)))
                .collect(),
            global_clusters: state
                .global_clusters
                .iter()
                .map(|(k, v)| (k.clone(), GlobalClusterView::from(v)))
                .collect(),
            db_cluster_endpoints: state
                .db_cluster_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), DbClusterEndpointView::from(v)))
                .collect(),
            event_subscriptions: state
                .event_subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), EventSubscriptionView::from(v)))
                .collect(),
            snapshot_attributes: state
                .snapshot_attributes
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(SnapshotAttributeView::from).collect(),
                    )
                })
                .collect(),
            resource_tags: state
                .resource_tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TagView::from).collect()))
                .collect(),
        }
    }
}

impl From<NeptuneStateView> for NeptuneState {
    fn from(view: NeptuneStateView) -> Self {
        Self {
            db_clusters: view
                .db_clusters
                .into_iter()
                .map(|(k, v)| (k, DbCluster::from(v)))
                .collect(),
            db_instances: view
                .db_instances
                .into_iter()
                .map(|(k, v)| (k, DbInstance::from(v)))
                .collect(),
            db_subnet_groups: view
                .db_subnet_groups
                .into_iter()
                .map(|(k, v)| (k, DbSubnetGroup::from(v)))
                .collect(),
            db_parameter_groups: view
                .db_parameter_groups
                .into_iter()
                .map(|(k, v)| (k, DbParameterGroup::from(v)))
                .collect(),
            db_cluster_parameter_groups: view
                .db_cluster_parameter_groups
                .into_iter()
                .map(|(k, v)| (k, DbClusterParameterGroup::from(v)))
                .collect(),
            db_cluster_snapshots: view
                .db_cluster_snapshots
                .into_iter()
                .map(|(k, v)| (k, DbClusterSnapshot::from(v)))
                .collect(),
            global_clusters: view
                .global_clusters
                .into_iter()
                .map(|(k, v)| (k, GlobalCluster::from(v)))
                .collect(),
            db_cluster_endpoints: view
                .db_cluster_endpoints
                .into_iter()
                .map(|(k, v)| (k, DbClusterEndpoint::from(v)))
                .collect(),
            event_subscriptions: view
                .event_subscriptions
                .into_iter()
                .map(|(k, v)| (k, EventSubscription::from(v)))
                .collect(),
            snapshot_attributes: view
                .snapshot_attributes
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(SnapshotAttribute::from).collect()))
                .collect(),
            resource_tags: view
                .resource_tags
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(Tag::from).collect()))
                .collect(),
        }
    }
}

impl StatefulService for NeptuneService {
    type StateView = NeptuneStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        NeptuneStateView::from(&*guard)
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
            *guard = NeptuneState::from(view);
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
            for (k, v) in view.db_clusters {
                guard.db_clusters.insert(k, DbCluster::from(v));
            }
            for (k, v) in view.db_instances {
                guard.db_instances.insert(k, DbInstance::from(v));
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
            for (k, v) in view.db_cluster_snapshots {
                guard
                    .db_cluster_snapshots
                    .insert(k, DbClusterSnapshot::from(v));
            }
            for (k, v) in view.global_clusters {
                guard.global_clusters.insert(k, GlobalCluster::from(v));
            }
            for (k, v) in view.db_cluster_endpoints {
                guard
                    .db_cluster_endpoints
                    .insert(k, DbClusterEndpoint::from(v));
            }
            for (k, v) in view.event_subscriptions {
                guard
                    .event_subscriptions
                    .insert(k, EventSubscription::from(v));
            }
            for (k, v) in view.snapshot_attributes {
                guard
                    .snapshot_attributes
                    .insert(k, v.into_iter().map(SnapshotAttribute::from).collect());
            }
            for (k, v) in view.resource_tags {
                guard
                    .resource_tags
                    .insert(k, v.into_iter().map(Tag::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
