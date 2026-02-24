//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-neptune

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBParameterGroupResult")]
pub struct DBParameterGroupNameMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultClusterParametersMessage")]
pub struct DescribeEngineDefaultClusterParametersMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterList {
    #[serde(rename = "Filter", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Filter>,
}
impl From<Vec<Filter>> for FilterList {
    fn from(v: Vec<Filter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Filter> for FilterList {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Filter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Filter>,
}

impl From<Vec<Filter>> for XmlFilterList {
    fn from(v: Vec<Filter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Filter> for XmlFilterList {
    fn from_iter<I: IntoIterator<Item = Filter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Filter")]
pub struct Filter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: FilterValueList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct FilterValueList {
    #[serde(rename = "Value", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for FilterValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for FilterValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<String>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlStringList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<String>,
}

impl From<Vec<String>> for XmlStringList {
    fn from(v: Vec<String>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<String> for XmlStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSubnetGroupsMessage")]
pub struct DescribeDBSubnetGroupsMessage {
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterMessage")]
pub struct ModifyDBClusterMessage {
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CloudwatchLogsExportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBInstanceParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_parameter_group_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "NewDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CloudwatchLogsExportConfiguration")]
pub struct CloudwatchLogsExportConfiguration {
    #[serde(rename = "DisableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_log_types: Option<LogTypeList>,
    #[serde(rename = "EnableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_log_types: Option<LogTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LogTypeList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for LogTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for LogTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessV2ScalingConfiguration")]
pub struct ServerlessV2ScalingConfiguration {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcSecurityGroupIdList {
    #[serde(
        rename = "VpcSecurityGroupId",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VpcSecurityGroupIdList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VpcSecurityGroupIdList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterSnapshotResult")]
pub struct DeleteDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshot")]
pub struct DBClusterSnapshot {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterSnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_arn: Option<String>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SourceDBClusterSnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_snapshot_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZones {
    #[serde(
        rename = "AvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AvailabilityZones {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AvailabilityZones {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventSubscriptionsResult")]
pub struct EventSubscriptionsMessage {
    #[serde(rename = "EventSubscriptionsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscriptions_list: Option<EventSubscriptionsList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventSubscriptionsList {
    #[serde(
        rename = "EventSubscription",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EventSubscription>,
}
impl From<Vec<EventSubscription>> for EventSubscriptionsList {
    fn from(v: Vec<EventSubscription>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventSubscription> for EventSubscriptionsList {
    fn from_iter<I: IntoIterator<Item = EventSubscription>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventSubscription>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventSubscriptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventSubscription>,
}

impl From<Vec<EventSubscription>> for XmlEventSubscriptionList {
    fn from(v: Vec<EventSubscription>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventSubscription> for XmlEventSubscriptionList {
    fn from_iter<I: IntoIterator<Item = EventSubscription>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventSubscription")]
pub struct EventSubscription {
    #[serde(rename = "CustSubscriptionId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_subscription_id: Option<String>,
    #[serde(rename = "CustomerAwsId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_aws_id: Option<String>,
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategoriesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_list: Option<EventCategoriesList>,
    #[serde(rename = "EventSubscriptionArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription_arn: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceIdsList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids_list: Option<SourceIdsList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "SubscriptionCreationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_creation_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCategoriesList {
    #[serde(
        rename = "EventCategory",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for EventCategoriesList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for EventCategoriesList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SourceIdsList {
    #[serde(rename = "SourceId", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SourceIdsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SourceIdsList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBParameterGroupMessage")]
pub struct CopyDBParameterGroupMessage {
    #[serde(rename = "SourceDBParameterGroupIdentifier")]
    #[serde(default)]
    pub source_d_b_parameter_group_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBParameterGroupDescription")]
    #[serde(default)]
    pub target_d_b_parameter_group_description: String,
    #[serde(rename = "TargetDBParameterGroupIdentifier")]
    #[serde(default)]
    pub target_d_b_parameter_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagList {
    #[serde(rename = "Tag", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Tag>,
}
impl From<Vec<Tag>> for TagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Tag> for TagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Tag>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTagList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Tag>,
}

impl From<Vec<Tag>> for XmlTagList {
    fn from(v: Vec<Tag>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Tag> for XmlTagList {
    fn from_iter<I: IntoIterator<Item = Tag>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Tag")]
pub struct Tag {
    #[serde(rename = "Key")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotsResult")]
pub struct DBClusterSnapshotMessage {
    #[serde(rename = "DBClusterSnapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshots: Option<DBClusterSnapshotList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterSnapshotList {
    #[serde(
        rename = "DBClusterSnapshot",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterSnapshot>,
}
impl From<Vec<DBClusterSnapshot>> for DBClusterSnapshotList {
    fn from(v: Vec<DBClusterSnapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterSnapshot> for DBClusterSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterSnapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterSnapshot>,
}

impl From<Vec<DBClusterSnapshot>> for XmlDBClusterSnapshotList {
    fn from(v: Vec<DBClusterSnapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterSnapshot> for XmlDBClusterSnapshotList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEventSubscriptionMessage")]
pub struct DeleteEventSubscriptionMessage {
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyPendingMaintenanceActionResult")]
pub struct ApplyPendingMaintenanceActionResult {
    #[serde(rename = "ResourcePendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_pending_maintenance_actions: Option<ResourcePendingMaintenanceActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourcePendingMaintenanceActions")]
pub struct ResourcePendingMaintenanceActions {
    #[serde(rename = "PendingMaintenanceActionDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_action_details: Option<PendingMaintenanceActionDetails>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingMaintenanceActionDetails {
    #[serde(
        rename = "PendingMaintenanceAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<PendingMaintenanceAction>,
}
impl From<Vec<PendingMaintenanceAction>> for PendingMaintenanceActionDetails {
    fn from(v: Vec<PendingMaintenanceAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PendingMaintenanceAction> for PendingMaintenanceActionDetails {
    fn from_iter<I: IntoIterator<Item = PendingMaintenanceAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PendingMaintenanceAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPendingMaintenanceActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PendingMaintenanceAction>,
}

impl From<Vec<PendingMaintenanceAction>> for XmlPendingMaintenanceActionList {
    fn from(v: Vec<PendingMaintenanceAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PendingMaintenanceAction> for XmlPendingMaintenanceActionList {
    fn from_iter<I: IntoIterator<Item = PendingMaintenanceAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingMaintenanceAction")]
pub struct PendingMaintenanceAction {
    #[serde(rename = "Action")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "AutoAppliedAfterDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_applied_after_date: Option<String>,
    #[serde(rename = "CurrentApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_apply_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ForcedApplyDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forced_apply_date: Option<String>,
    #[serde(rename = "OptInStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opt_in_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSubnetGroupMessage")]
pub struct CreateDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    pub d_b_subnet_group_description: String,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: SubnetIdentifierList,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetIdentifierList {
    #[serde(
        rename = "SubnetIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SubnetIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SubnetIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterEndpointResult")]
pub struct DeleteDBClusterEndpointOutput {
    #[serde(rename = "CustomEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_type: Option<String>,
    #[serde(rename = "DBClusterEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_arn: Option<String>,
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterEndpointResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_resource_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct StringList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for StringList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for StringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalClustersMessage")]
pub struct DescribeGlobalClustersMessage {
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterEndpointMessage")]
pub struct ModifyDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterSnapshotMessage")]
pub struct DeleteDBClusterSnapshotMessage {
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventSubscriptionsMessage")]
pub struct DescribeEventSubscriptionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromSnapshotResult")]
pub struct RestoreDBClusterFromSnapshotResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBCluster")]
pub struct DBCluster {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AssociatedRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_roles: Option<DBClusterRoles>,
    #[serde(rename = "AutomaticRestartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_restart_time: Option<String>,
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CloneGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_group_id: Option<String>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "CrossAccountClone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_account_clone: Option<bool>,
    #[serde(rename = "DBClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_arn: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_members: Option<DBClusterMemberList>,
    #[serde(rename = "DBClusterOptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_option_group_memberships: Option<DBClusterOptionGroupMemberships>,
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<String>,
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DbClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_cluster_resource_id: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EarliestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_restorable_time: Option<String>,
    #[serde(rename = "EnabledCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "IOOptimizedNextAllowedModificationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_o_optimized_next_allowed_modification_time: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<ClusterPendingModifiedValues>,
    #[serde(rename = "PercentProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_progress: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ReadReplicaIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_identifiers: Option<ReadReplicaIdentifierList>,
    #[serde(rename = "ReaderEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reader_endpoint: Option<String>,
    #[serde(rename = "ReplicationSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_source_identifier: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfigurationInfo>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterRoles {
    #[serde(
        rename = "DBClusterRole",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterRole>,
}
impl From<Vec<DBClusterRole>> for DBClusterRoles {
    fn from(v: Vec<DBClusterRole>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterRole> for DBClusterRoles {
    fn from_iter<I: IntoIterator<Item = DBClusterRole>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterRole>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterRole>,
}

impl From<Vec<DBClusterRole>> for XmlDBClusterRoleList {
    fn from(v: Vec<DBClusterRole>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterRole> for XmlDBClusterRoleList {
    fn from_iter<I: IntoIterator<Item = DBClusterRole>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterRole")]
pub struct DBClusterRole {
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterMemberList {
    #[serde(
        rename = "DBClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterMember>,
}
impl From<Vec<DBClusterMember>> for DBClusterMemberList {
    fn from(v: Vec<DBClusterMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterMember> for DBClusterMemberList {
    fn from_iter<I: IntoIterator<Item = DBClusterMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterMember>,
}

impl From<Vec<DBClusterMember>> for XmlDBClusterMemberList {
    fn from(v: Vec<DBClusterMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterMember> for XmlDBClusterMemberList {
    fn from_iter<I: IntoIterator<Item = DBClusterMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterMember")]
pub struct DBClusterMember {
    #[serde(rename = "DBClusterParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_status: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "IsClusterWriter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_cluster_writer: Option<bool>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterOptionGroupMemberships {
    #[serde(
        rename = "DBClusterOptionGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterOptionGroupStatus>,
}
impl From<Vec<DBClusterOptionGroupStatus>> for DBClusterOptionGroupMemberships {
    fn from(v: Vec<DBClusterOptionGroupStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterOptionGroupStatus> for DBClusterOptionGroupMemberships {
    fn from_iter<I: IntoIterator<Item = DBClusterOptionGroupStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterOptionGroupStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterOptionGroupStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterOptionGroupStatus>,
}

impl From<Vec<DBClusterOptionGroupStatus>> for XmlDBClusterOptionGroupStatusList {
    fn from(v: Vec<DBClusterOptionGroupStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterOptionGroupStatus> for XmlDBClusterOptionGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBClusterOptionGroupStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterOptionGroupStatus")]
pub struct DBClusterOptionGroupStatus {
    #[serde(rename = "DBClusterOptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterPendingModifiedValues")]
pub struct ClusterPendingModifiedValues {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "PendingCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingCloudwatchLogsExports")]
pub struct PendingCloudwatchLogsExports {
    #[serde(rename = "LogTypesToDisable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_disable: Option<LogTypeList>,
    #[serde(rename = "LogTypesToEnable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_types_to_enable: Option<LogTypeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaIdentifierList {
    #[serde(
        rename = "ReadReplicaIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessV2ScalingConfigurationInfo")]
pub struct ServerlessV2ScalingConfigurationInfo {
    #[serde(rename = "MaxCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_capacity: Option<f64>,
    #[serde(rename = "MinCapacity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_capacity: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcSecurityGroupMembershipList {
    #[serde(
        rename = "VpcSecurityGroupMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<VpcSecurityGroupMembership>,
}
impl From<Vec<VpcSecurityGroupMembership>> for VpcSecurityGroupMembershipList {
    fn from(v: Vec<VpcSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VpcSecurityGroupMembership> for VpcSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = VpcSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VpcSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVpcSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VpcSecurityGroupMembership>,
}

impl From<Vec<VpcSecurityGroupMembership>> for XmlVpcSecurityGroupMembershipList {
    fn from(v: Vec<VpcSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VpcSecurityGroupMembership> for XmlVpcSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = VpcSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcSecurityGroupMembership")]
pub struct VpcSecurityGroupMembership {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "VpcSecurityGroupId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalClusterMessage")]
pub struct FailoverGlobalClusterMessage {
    #[serde(rename = "AllowDataLoss")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_data_loss: Option<bool>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "Switchover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switchover: Option<bool>,
    #[serde(rename = "TargetDbClusterIdentifier")]
    #[serde(default)]
    pub target_db_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSubnetGroupMessage")]
pub struct ModifyDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_description: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: SubnetIdentifierList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEventSubscriptionMessage")]
pub struct CreateEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    pub sns_topic_arn: String,
    #[serde(rename = "SourceIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<SourceIdsList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveRoleFromDBClusterMessage")]
pub struct RemoveRoleFromDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddSourceIdentifierToSubscriptionMessage")]
pub struct AddSourceIdentifierToSubscriptionMessage {
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    pub source_identifier: String,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverGlobalClusterResult")]
pub struct FailoverGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalCluster")]
pub struct GlobalCluster {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "FailoverState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failover_state: Option<FailoverState>,
    #[serde(rename = "GlobalClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_arn: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "GlobalClusterMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_members: Option<GlobalClusterMemberList>,
    #[serde(rename = "GlobalClusterResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_resource_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverState")]
pub struct FailoverState {
    #[serde(rename = "FromDbClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_db_cluster_arn: Option<String>,
    #[serde(rename = "IsDataLossAllowed")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_data_loss_allowed: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "ToDbClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_db_cluster_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalClusterMemberList {
    #[serde(
        rename = "GlobalClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalClusterMember>,
}
impl From<Vec<GlobalClusterMember>> for GlobalClusterMemberList {
    fn from(v: Vec<GlobalClusterMember>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalClusterMember> for GlobalClusterMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalClusterMember>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalClusterMember>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalClusterMemberList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalClusterMember>,
}

impl From<Vec<GlobalClusterMember>> for XmlGlobalClusterMemberList {
    fn from(v: Vec<GlobalClusterMember>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalClusterMember> for XmlGlobalClusterMemberList {
    fn from_iter<I: IntoIterator<Item = GlobalClusterMember>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GlobalClusterMember")]
pub struct GlobalClusterMember {
    #[serde(rename = "DBClusterArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_arn: Option<String>,
    #[serde(rename = "IsWriter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_writer: Option<bool>,
    #[serde(rename = "Readers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readers: Option<ReadersArnList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadersArnList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadersArnList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadersArnList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterSnapshotMessage")]
pub struct CopyDBClusterSnapshotMessage {
    #[serde(rename = "CopyTags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "SourceDBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub source_d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub target_d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalClusterMessage")]
pub struct CreateGlobalClusterMessage {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_d_b_cluster_identifier: Option<String>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClustersResult")]
pub struct DBClusterMessage {
    #[serde(rename = "DBClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_clusters: Option<DBClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterList {
    #[serde(rename = "DBCluster", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBCluster>,
}
impl From<Vec<DBCluster>> for DBClusterList {
    fn from(v: Vec<DBCluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBCluster> for DBClusterList {
    fn from_iter<I: IntoIterator<Item = DBCluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBCluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBCluster>,
}

impl From<Vec<DBCluster>> for XmlDBClusterList {
    fn from(v: Vec<DBCluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBCluster> for XmlDBClusterList {
    fn from_iter<I: IntoIterator<Item = DBCluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBSubnetGroupMessage")]
pub struct DeleteDBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    pub d_b_subnet_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceResult")]
pub struct TagListMessage {
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveSourceIdentifierFromSubscriptionMessage")]
pub struct RemoveSourceIdentifierFromSubscriptionMessage {
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    pub source_identifier: String,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBClusterResult")]
pub struct StopDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBParameterGroupMessage")]
pub struct ResetDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
    #[serde(rename = "ResetAllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_all_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParametersList {
    #[serde(rename = "Parameter", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Parameter>,
}
impl From<Vec<Parameter>> for ParametersList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Parameter> for ParametersList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Parameter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlParameterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Parameter>,
}

impl From<Vec<Parameter>> for XmlParameterList {
    fn from(v: Vec<Parameter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Parameter> for XmlParameterList {
    fn from_iter<I: IntoIterator<Item = Parameter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Parameter")]
pub struct Parameter {
    #[serde(rename = "AllowedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<String>,
    #[serde(rename = "ApplyMethod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_method: Option<String>,
    #[serde(rename = "ApplyType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_type: Option<String>,
    #[serde(rename = "DataType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IsModifiable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_modifiable: Option<bool>,
    #[serde(rename = "MinimumEngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_engine_version: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
    #[serde(rename = "ParameterValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_value: Option<String>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBInstanceMessage")]
pub struct CreateDBInstanceMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    pub d_b_instance_class: String,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupNameList>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSecurityGroupNameList {
    #[serde(
        rename = "DBSecurityGroupName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DBSecurityGroupNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DBSecurityGroupNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateGlobalClusterResult")]
pub struct CreateGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterMessage")]
pub struct CreateDBClusterMessage {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster_identifier: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreSignedUrl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_signed_url: Option<String>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "ReplicationSourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replication_source_identifier: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParametersResult")]
pub struct DBParameterGroupDetails {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterParameterGroupMessage")]
pub struct ModifyDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ParametersList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveTagsFromResourceMessage")]
pub struct RemoveTagsFromResourceMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: KeyList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct KeyList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for KeyList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for KeyList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBSubnetGroupResult")]
pub struct ModifyDBSubnetGroupResult {
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSubnetGroup")]
pub struct DBSubnetGroup {
    #[serde(rename = "DBSubnetGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_arn: Option<String>,
    #[serde(rename = "DBSubnetGroupDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_description: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<SubnetList>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SubnetList {
    #[serde(rename = "Subnet", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Subnet>,
}
impl From<Vec<Subnet>> for SubnetList {
    fn from(v: Vec<Subnet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Subnet> for SubnetList {
    fn from_iter<I: IntoIterator<Item = Subnet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Subnet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSubnetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Subnet>,
}

impl From<Vec<Subnet>> for XmlSubnetList {
    fn from(v: Vec<Subnet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Subnet> for XmlSubnetList {
    fn from_iter<I: IntoIterator<Item = Subnet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Subnet")]
pub struct Subnet {
    #[serde(rename = "SubnetAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_availability_zone: Option<AvailabilityZone>,
    #[serde(rename = "SubnetIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_identifier: Option<String>,
    #[serde(rename = "SubnetStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AvailabilityZone")]
pub struct AvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParametersMessage")]
pub struct DescribeDBParametersMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterSnapshotAttributeResult")]
pub struct ModifyDBClusterSnapshotAttributeResult {
    #[serde(rename = "DBClusterSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshotAttributesResult")]
pub struct DBClusterSnapshotAttributesResult {
    #[serde(rename = "DBClusterSnapshotAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes: Option<DBClusterSnapshotAttributeList>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterSnapshotAttributeList {
    #[serde(
        rename = "DBClusterSnapshotAttribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterSnapshotAttribute>,
}
impl From<Vec<DBClusterSnapshotAttribute>> for DBClusterSnapshotAttributeList {
    fn from(v: Vec<DBClusterSnapshotAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterSnapshotAttribute> for DBClusterSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshotAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterSnapshotAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterSnapshotAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterSnapshotAttribute>,
}

impl From<Vec<DBClusterSnapshotAttribute>> for XmlDBClusterSnapshotAttributeList {
    fn from(v: Vec<DBClusterSnapshotAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterSnapshotAttribute> for XmlDBClusterSnapshotAttributeList {
    fn from_iter<I: IntoIterator<Item = DBClusterSnapshotAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterSnapshotAttribute")]
pub struct DBClusterSnapshotAttribute {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_name: Option<String>,
    #[serde(rename = "AttributeValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_values: Option<AttributeValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeValueList {
    #[serde(
        rename = "AttributeValue",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AttributeValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AttributeValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverDBClusterResult")]
pub struct FailoverDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsResult")]
pub struct EventsMessage {
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<EventList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventList {
    #[serde(rename = "Event", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Event>,
}
impl From<Vec<Event>> for EventList {
    fn from(v: Vec<Event>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Event> for EventList {
    fn from_iter<I: IntoIterator<Item = Event>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Event>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Event>,
}

impl From<Vec<Event>> for XmlEventList {
    fn from(v: Vec<Event>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Event> for XmlEventList {
    fn from_iter<I: IntoIterator<Item = Event>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Event")]
pub struct Event {
    #[serde(rename = "Date")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveFromGlobalClusterResult")]
pub struct RemoveFromGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBInstanceResult")]
pub struct ModifyDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstance")]
pub struct DBInstance {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_arn: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBInstanceStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_status: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DBParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_groups: Option<DBParameterGroupStatusList>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupMembershipList>,
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
    #[serde(rename = "DbInstancePort")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_instance_port: Option<i32>,
    #[serde(rename = "DbiResourceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbi_resource_id: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "DomainMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_memberships: Option<DomainMembershipList>,
    #[serde(rename = "EnabledCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "EnhancedMonitoringResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_monitoring_resource_arn: Option<String>,
    #[serde(rename = "IAMDatabaseAuthenticationEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_database_authentication_enabled: Option<bool>,
    #[serde(rename = "InstanceCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_create_time: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_restorable_time: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "OptionGroupMemberships")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_memberships: Option<OptionGroupMembershipList>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedValues>,
    #[serde(rename = "PerformanceInsightsEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_enabled: Option<bool>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReadReplicaDBClusterIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_cluster_identifiers: Option<ReadReplicaDBClusterIdentifierList>,
    #[serde(rename = "ReadReplicaDBInstanceIdentifiers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_d_b_instance_identifiers: Option<ReadReplicaDBInstanceIdentifierList>,
    #[serde(rename = "ReadReplicaSourceDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_source_d_b_instance_identifier: Option<String>,
    #[serde(rename = "SecondaryAvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_availability_zone: Option<String>,
    #[serde(rename = "StatusInfos")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_infos: Option<DBInstanceStatusInfoList>,
    #[serde(rename = "StorageEncrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_encrypted: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "Timezone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBParameterGroupStatusList {
    #[serde(
        rename = "DBParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBParameterGroupStatus>,
}
impl From<Vec<DBParameterGroupStatus>> for DBParameterGroupStatusList {
    fn from(v: Vec<DBParameterGroupStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBParameterGroupStatus> for DBParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroupStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBParameterGroupStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBParameterGroupStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBParameterGroupStatus>,
}

impl From<Vec<DBParameterGroupStatus>> for XmlDBParameterGroupStatusList {
    fn from(v: Vec<DBParameterGroupStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBParameterGroupStatus> for XmlDBParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroupStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBParameterGroupStatus")]
pub struct DBParameterGroupStatus {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSecurityGroupMembershipList {
    #[serde(
        rename = "DBSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSecurityGroupMembership>,
}
impl From<Vec<DBSecurityGroupMembership>> for DBSecurityGroupMembershipList {
    fn from(v: Vec<DBSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSecurityGroupMembership> for DBSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSecurityGroupMembership>,
}

impl From<Vec<DBSecurityGroupMembership>> for XmlDBSecurityGroupMembershipList {
    fn from(v: Vec<DBSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSecurityGroupMembership> for XmlDBSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = DBSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBSecurityGroupMembership")]
pub struct DBSecurityGroupMembership {
    #[serde(rename = "DBSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DomainMembershipList {
    #[serde(
        rename = "DomainMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DomainMembership>,
}
impl From<Vec<DomainMembership>> for DomainMembershipList {
    fn from(v: Vec<DomainMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DomainMembership> for DomainMembershipList {
    fn from_iter<I: IntoIterator<Item = DomainMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DomainMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDomainMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DomainMembership>,
}

impl From<Vec<DomainMembership>> for XmlDomainMembershipList {
    fn from(v: Vec<DomainMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DomainMembership> for XmlDomainMembershipList {
    fn from_iter<I: IntoIterator<Item = DomainMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DomainMembership")]
pub struct DomainMembership {
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "FQDN")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub f_q_d_n: Option<String>,
    #[serde(rename = "IAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_a_m_role_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Endpoint")]
pub struct Endpoint {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "HostedZoneId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_zone_id: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptionGroupMembershipList {
    #[serde(
        rename = "OptionGroupMembership",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OptionGroupMembership>,
}
impl From<Vec<OptionGroupMembership>> for OptionGroupMembershipList {
    fn from(v: Vec<OptionGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OptionGroupMembership> for OptionGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = OptionGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OptionGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOptionGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OptionGroupMembership>,
}

impl From<Vec<OptionGroupMembership>> for XmlOptionGroupMembershipList {
    fn from(v: Vec<OptionGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OptionGroupMembership> for XmlOptionGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = OptionGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OptionGroupMembership")]
pub struct OptionGroupMembership {
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PendingModifiedValues")]
pub struct PendingModifiedValues {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "PendingCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_cloudwatch_logs_exports: Option<PendingCloudwatchLogsExports>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaDBClusterIdentifierList {
    #[serde(
        rename = "ReadReplicaDBClusterIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaDBClusterIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaDBClusterIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReadReplicaDBInstanceIdentifierList {
    #[serde(
        rename = "ReadReplicaDBInstanceIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ReadReplicaDBInstanceIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ReadReplicaDBInstanceIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceStatusInfoList {
    #[serde(
        rename = "DBInstanceStatusInfo",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBInstanceStatusInfo>,
}
impl From<Vec<DBInstanceStatusInfo>> for DBInstanceStatusInfoList {
    fn from(v: Vec<DBInstanceStatusInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstanceStatusInfo> for DBInstanceStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBInstanceStatusInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstanceStatusInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceStatusInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstanceStatusInfo>,
}

impl From<Vec<DBInstanceStatusInfo>> for XmlDBInstanceStatusInfoList {
    fn from(v: Vec<DBInstanceStatusInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstanceStatusInfo> for XmlDBInstanceStatusInfoList {
    fn from_iter<I: IntoIterator<Item = DBInstanceStatusInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBInstanceStatusInfo")]
pub struct DBInstanceStatusInfo {
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Normal")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normal: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddRoleToDBClusterMessage")]
pub struct AddRoleToDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "FeatureName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_name: Option<String>,
    #[serde(rename = "RoleArn")]
    #[serde(default)]
    pub role_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParameterGroupsMessage")]
pub struct DescribeDBParameterGroupsMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBClusterResult")]
pub struct StartDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParametersResult")]
pub struct DBClusterParameterGroupDetails {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBParameterGroupMessage")]
pub struct ModifyDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ParametersList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableDBInstanceOptionsResult")]
pub struct OrderableDBInstanceOptionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OrderableDBInstanceOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_d_b_instance_options: Option<OrderableDBInstanceOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderableDBInstanceOptionsList {
    #[serde(
        rename = "OrderableDBInstanceOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OrderableDBInstanceOption>,
}
impl From<Vec<OrderableDBInstanceOption>> for OrderableDBInstanceOptionsList {
    fn from(v: Vec<OrderableDBInstanceOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OrderableDBInstanceOption> for OrderableDBInstanceOptionsList {
    fn from_iter<I: IntoIterator<Item = OrderableDBInstanceOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OrderableDBInstanceOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOrderableDBInstanceOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OrderableDBInstanceOption>,
}

impl From<Vec<OrderableDBInstanceOption>> for XmlOrderableDBInstanceOptionList {
    fn from(v: Vec<OrderableDBInstanceOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OrderableDBInstanceOption> for XmlOrderableDBInstanceOptionList {
    fn from_iter<I: IntoIterator<Item = OrderableDBInstanceOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OrderableDBInstanceOption")]
pub struct OrderableDBInstanceOption {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZoneList>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MaxIopsPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops_per_db_instance: Option<i32>,
    #[serde(rename = "MaxIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_iops_per_gib: Option<f64>,
    #[serde(rename = "MaxStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_storage_size: Option<i32>,
    #[serde(rename = "MinIopsPerDbInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops_per_db_instance: Option<i32>,
    #[serde(rename = "MinIopsPerGib")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_iops_per_gib: Option<f64>,
    #[serde(rename = "MinStorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_storage_size: Option<i32>,
    #[serde(rename = "MultiAZCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_capable: Option<bool>,
    #[serde(rename = "ReadReplicaCapable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_replica_capable: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "SupportsEnhancedMonitoring")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_enhanced_monitoring: Option<bool>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
    #[serde(rename = "SupportsIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "SupportsIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_iops: Option<bool>,
    #[serde(rename = "SupportsPerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_performance_insights: Option<bool>,
    #[serde(rename = "SupportsStorageEncryption")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_storage_encryption: Option<bool>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AvailabilityZoneList {
    #[serde(
        rename = "AvailabilityZone",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AvailabilityZone>,
}
impl From<Vec<AvailabilityZone>> for AvailabilityZoneList {
    fn from(v: Vec<AvailabilityZone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AvailabilityZone> for AvailabilityZoneList {
    fn from_iter<I: IntoIterator<Item = AvailabilityZone>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AvailabilityZone>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAvailabilityZoneList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AvailabilityZone>,
}

impl From<Vec<AvailabilityZone>> for XmlAvailabilityZoneList {
    fn from(v: Vec<AvailabilityZone>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AvailabilityZone> for XmlAvailabilityZoneList {
    fn from_iter<I: IntoIterator<Item = AvailabilityZone>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterFromSnapshotMessage")]
pub struct RestoreDBClusterFromSnapshotMessage {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZones>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterEndpointMessage")]
pub struct DeleteDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StartDBClusterMessage")]
pub struct StartDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEventSubscriptionResult")]
pub struct CreateEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEventSubscriptionMessage")]
pub struct ModifyEventSubscriptionMessage {
    #[serde(rename = "Enabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListTagsForResourceMessage")]
pub struct ListTagsForResourceMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableDBInstanceOptionsMessage")]
pub struct DescribeOrderableDBInstanceOptionsMessage {
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    pub engine: String,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Vpc")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParameterGroupsResult")]
pub struct DBClusterParameterGroupsMessage {
    #[serde(rename = "DBClusterParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_groups: Option<DBClusterParameterGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterParameterGroupList {
    #[serde(
        rename = "DBClusterParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterParameterGroup>,
}
impl From<Vec<DBClusterParameterGroup>> for DBClusterParameterGroupList {
    fn from(v: Vec<DBClusterParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterParameterGroup> for DBClusterParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBClusterParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterParameterGroup>,
}

impl From<Vec<DBClusterParameterGroup>> for XmlDBClusterParameterGroupList {
    fn from(v: Vec<DBClusterParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterParameterGroup> for XmlDBClusterParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBClusterParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterParameterGroup")]
pub struct DBClusterParameterGroup {
    #[serde(rename = "DBClusterParameterGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_arn: Option<String>,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterSnapshotResult")]
pub struct CreateDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterResult")]
pub struct CreateDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterSnapshotMessage")]
pub struct CreateDBClusterSnapshotMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeGlobalClustersResult")]
pub struct GlobalClustersMessage {
    #[serde(rename = "GlobalClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_clusters: Option<GlobalClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GlobalClusterList {
    #[serde(
        rename = "GlobalClusterMember",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<GlobalCluster>,
}
impl From<Vec<GlobalCluster>> for GlobalClusterList {
    fn from(v: Vec<GlobalCluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<GlobalCluster> for GlobalClusterList {
    fn from_iter<I: IntoIterator<Item = GlobalCluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<GlobalCluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlGlobalClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<GlobalCluster>,
}

impl From<Vec<GlobalCluster>> for XmlGlobalClusterList {
    fn from(v: Vec<GlobalCluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<GlobalCluster> for XmlGlobalClusterList {
    fn from_iter<I: IntoIterator<Item = GlobalCluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterToPointInTimeResult")]
pub struct RestoreDBClusterToPointInTimeResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBClusterParameterGroupMessage")]
pub struct ResetDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
    #[serde(rename = "ResetAllParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_all_parameters: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBInstanceMessage")]
pub struct ModifyDBInstanceMessage {
    #[serde(rename = "AllocatedStorage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allocated_storage: Option<i32>,
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "ApplyImmediately")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_immediately: Option<bool>,
    #[serde(rename = "AutoMinorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_minor_version_upgrade: Option<bool>,
    #[serde(rename = "BackupRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_retention_period: Option<i32>,
    #[serde(rename = "CACertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_a_certificate_identifier: Option<String>,
    #[serde(rename = "CloudwatchLogsExportConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudwatch_logs_export_configuration: Option<CloudwatchLogsExportConfiguration>,
    #[serde(rename = "CopyTagsToSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_tags_to_snapshot: Option<bool>,
    #[serde(rename = "DBInstanceClass")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_class: Option<String>,
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "DBPortNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_port_number: Option<i32>,
    #[serde(rename = "DBSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_security_groups: Option<DBSecurityGroupNameList>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "Domain")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "DomainIAMRoleName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_i_a_m_role_name: Option<String>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "EnablePerformanceInsights")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_performance_insights: Option<bool>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Iops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops: Option<i32>,
    #[serde(rename = "LicenseModel")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_model: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MonitoringInterval")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_interval: Option<i32>,
    #[serde(rename = "MonitoringRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitoring_role_arn: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NewDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_d_b_instance_identifier: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "PerformanceInsightsKMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub performance_insights_k_m_s_key_id: Option<String>,
    #[serde(rename = "PreferredBackupWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_backup_window: Option<String>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PromotionTier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_tier: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "TdeCredentialArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_arn: Option<String>,
    #[serde(rename = "TdeCredentialPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tde_credential_password: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetDBClusterParameterGroupResult")]
pub struct DBClusterParameterGroupNameMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventsMessage")]
pub struct DescribeEventsMessage {
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_identifier: Option<String>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterEndpointsResult")]
pub struct DBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoints: Option<DBClusterEndpointList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBClusterEndpointList {
    #[serde(
        rename = "DBClusterEndpointList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBClusterEndpoint>,
}
impl From<Vec<DBClusterEndpoint>> for DBClusterEndpointList {
    fn from(v: Vec<DBClusterEndpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBClusterEndpoint> for DBClusterEndpointList {
    fn from_iter<I: IntoIterator<Item = DBClusterEndpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBClusterEndpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBClusterEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBClusterEndpoint>,
}

impl From<Vec<DBClusterEndpoint>> for XmlDBClusterEndpointList {
    fn from(v: Vec<DBClusterEndpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBClusterEndpoint> for XmlDBClusterEndpointList {
    fn from_iter<I: IntoIterator<Item = DBClusterEndpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBClusterEndpoint")]
pub struct DBClusterEndpoint {
    #[serde(rename = "CustomEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_type: Option<String>,
    #[serde(rename = "DBClusterEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_arn: Option<String>,
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterEndpointResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_resource_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBEngineVersionsMessage")]
pub struct DescribeDBEngineVersionsMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "DefaultOnly")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_only: Option<bool>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "ListSupportedCharacterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_supported_character_sets: Option<bool>,
    #[serde(rename = "ListSupportedTimezones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list_supported_timezones: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverGlobalClusterResult")]
pub struct SwitchoverGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalClusterMessage")]
pub struct DeleteGlobalClusterMessage {
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParametersMessage")]
pub struct DescribeDBClusterParametersMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddTagsToResourceMessage")]
pub struct AddTagsToResourceMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBParameterGroupResult")]
pub struct CopyDBParameterGroupResult {
    #[serde(rename = "DBParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group: Option<DBParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBParameterGroup")]
pub struct DBParameterGroup {
    #[serde(rename = "DBParameterGroupArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_arn: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterParameterGroupResult")]
pub struct CreateDBClusterParameterGroupResult {
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterParameterGroupMessage")]
pub struct CopyDBClusterParameterGroupMessage {
    #[serde(rename = "SourceDBClusterParameterGroupIdentifier")]
    #[serde(default)]
    pub source_d_b_cluster_parameter_group_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetDBClusterParameterGroupDescription")]
    #[serde(default)]
    pub target_d_b_cluster_parameter_group_description: String,
    #[serde(rename = "TargetDBClusterParameterGroupIdentifier")]
    #[serde(default)]
    pub target_d_b_cluster_parameter_group_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBInstanceMessage")]
pub struct RebootDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "ForceFailover")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_failover: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeValidDBInstanceModificationsMessage")]
pub struct DescribeValidDBInstanceModificationsMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEventSubscriptionResult")]
pub struct ModifyEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBSubnetGroupResult")]
pub struct CreateDBSubnetGroupResult {
    #[serde(rename = "DBSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group: Option<DBSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ApplyPendingMaintenanceActionMessage")]
pub struct ApplyPendingMaintenanceActionMessage {
    #[serde(rename = "ApplyAction")]
    #[serde(default)]
    pub apply_action: String,
    #[serde(rename = "OptInType")]
    #[serde(default)]
    pub opt_in_type: String,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    pub resource_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterEndpointsMessage")]
pub struct DescribeDBClusterEndpointsMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClustersMessage")]
pub struct DescribeDBClustersMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterParameterGroupResult")]
pub struct CopyDBClusterParameterGroupResult {
    #[serde(rename = "DBClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group: Option<DBClusterParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBSubnetGroupsResult")]
pub struct DBSubnetGroupMessage {
    #[serde(rename = "DBSubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_groups: Option<DBSubnetGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBSubnetGroups {
    #[serde(
        rename = "DBSubnetGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBSubnetGroup>,
}
impl From<Vec<DBSubnetGroup>> for DBSubnetGroups {
    fn from(v: Vec<DBSubnetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBSubnetGroup> for DBSubnetGroups {
    fn from_iter<I: IntoIterator<Item = DBSubnetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBSubnetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBSubnetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBSubnetGroup>,
}

impl From<Vec<DBSubnetGroup>> for XmlDBSubnetGroupList {
    fn from(v: Vec<DBSubnetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBSubnetGroup> for XmlDBSubnetGroupList {
    fn from_iter<I: IntoIterator<Item = DBSubnetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBParameterGroupMessage")]
pub struct CreateDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePendingMaintenanceActionsMessage")]
pub struct DescribePendingMaintenanceActionsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalClusterResult")]
pub struct ModifyGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterEndpointResult")]
pub struct ModifyDBClusterEndpointOutput {
    #[serde(rename = "CustomEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_type: Option<String>,
    #[serde(rename = "DBClusterEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_arn: Option<String>,
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterEndpointResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_resource_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveSourceIdentifierFromSubscriptionResult")]
pub struct RemoveSourceIdentifierFromSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterMessage")]
pub struct DeleteDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SkipFinalSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteGlobalClusterResult")]
pub struct DeleteGlobalClusterResult {
    #[serde(rename = "GlobalCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_cluster: Option<GlobalCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotAttributesResult")]
pub struct DescribeDBClusterSnapshotAttributesResult {
    #[serde(rename = "DBClusterSnapshotAttributesResult")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_attributes_result: Option<DBClusterSnapshotAttributesResult>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaDBClusterMessage")]
pub struct PromoteReadReplicaDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RemoveFromGlobalClusterMessage")]
pub struct RemoveFromGlobalClusterMessage {
    #[serde(rename = "DbClusterIdentifier")]
    #[serde(default)]
    pub db_cluster_identifier: String,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventCategoriesResult")]
pub struct EventCategoriesMessage {
    #[serde(rename = "EventCategoriesMapList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories_map_list: Option<EventCategoriesMapList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventCategoriesMapList {
    #[serde(
        rename = "EventCategoriesMap",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EventCategoriesMap>,
}
impl From<Vec<EventCategoriesMap>> for EventCategoriesMapList {
    fn from(v: Vec<EventCategoriesMap>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventCategoriesMap> for EventCategoriesMapList {
    fn from_iter<I: IntoIterator<Item = EventCategoriesMap>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventCategoriesMap>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventCategoriesMapList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventCategoriesMap>,
}

impl From<Vec<EventCategoriesMap>> for XmlEventCategoriesMapList {
    fn from(v: Vec<EventCategoriesMap>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventCategoriesMap> for XmlEventCategoriesMapList {
    fn from_iter<I: IntoIterator<Item = EventCategoriesMap>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventCategoriesMap")]
pub struct EventCategoriesMap {
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterResult")]
pub struct ModifyDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEventSubscriptionResult")]
pub struct DeleteEventSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotAttributesMessage")]
pub struct DescribeDBClusterSnapshotAttributesMessage {
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyGlobalClusterMessage")]
pub struct ModifyGlobalClusterMessage {
    #[serde(rename = "AllowMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_major_version_upgrade: Option<bool>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "NewGlobalClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_global_cluster_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBParameterGroupMessage")]
pub struct DeleteDBParameterGroupMessage {
    #[serde(rename = "DBParameterGroupName")]
    #[serde(default)]
    pub d_b_parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePendingMaintenanceActionsResult")]
pub struct PendingMaintenanceActionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "PendingMaintenanceActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_maintenance_actions: Option<PendingMaintenanceActions>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingMaintenanceActions {
    #[serde(
        rename = "ResourcePendingMaintenanceActions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ResourcePendingMaintenanceActions>,
}
impl From<Vec<ResourcePendingMaintenanceActions>> for PendingMaintenanceActions {
    fn from(v: Vec<ResourcePendingMaintenanceActions>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ResourcePendingMaintenanceActions> for PendingMaintenanceActions {
    fn from_iter<I: IntoIterator<Item = ResourcePendingMaintenanceActions>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ResourcePendingMaintenanceActions>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlResourcePendingMaintenanceActionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ResourcePendingMaintenanceActions>,
}

impl From<Vec<ResourcePendingMaintenanceActions>> for XmlResourcePendingMaintenanceActionsList {
    fn from(v: Vec<ResourcePendingMaintenanceActions>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ResourcePendingMaintenanceActions> for XmlResourcePendingMaintenanceActionsList {
    fn from_iter<I: IntoIterator<Item = ResourcePendingMaintenanceActions>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterParameterGroupsMessage")]
pub struct DescribeDBClusterParameterGroupsMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBInstanceResult")]
pub struct CreateDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PromoteReadReplicaDBClusterResult")]
pub struct PromoteReadReplicaDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultParametersResult")]
pub struct DescribeEngineDefaultParametersResult {
    #[serde(rename = "EngineDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_defaults: Option<EngineDefaults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EngineDefaults")]
pub struct EngineDefaults {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBInstanceResult")]
pub struct DeleteDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverDBClusterMessage")]
pub struct FailoverDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "TargetDBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_d_b_instance_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootDBInstanceResult")]
pub struct RebootDBInstanceResult {
    #[serde(rename = "DBInstance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance: Option<DBInstance>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterParameterGroupMessage")]
pub struct DeleteDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyDBClusterSnapshotResult")]
pub struct CopyDBClusterSnapshotResult {
    #[serde(rename = "DBClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot: Option<DBClusterSnapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstancesResult")]
pub struct DBInstanceMessage {
    #[serde(rename = "DBInstances")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instances: Option<DBInstanceList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBInstanceList {
    #[serde(rename = "DBInstance", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DBInstance>,
}
impl From<Vec<DBInstance>> for DBInstanceList {
    fn from(v: Vec<DBInstance>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBInstance> for DBInstanceList {
    fn from_iter<I: IntoIterator<Item = DBInstance>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBInstance>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBInstanceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBInstance>,
}

impl From<Vec<DBInstance>> for XmlDBInstanceList {
    fn from(v: Vec<DBInstance>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBInstance> for XmlDBInstanceList {
    fn from_iter<I: IntoIterator<Item = DBInstance>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterParameterGroupMessage")]
pub struct CreateDBClusterParameterGroupMessage {
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    pub d_b_cluster_parameter_group_name: String,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBInstancesMessage")]
pub struct DescribeDBInstancesMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_instance_identifier: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyDBClusterSnapshotAttributeMessage")]
pub struct ModifyDBClusterSnapshotAttributeMessage {
    #[serde(rename = "AttributeName")]
    #[serde(default)]
    pub attribute_name: String,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    pub d_b_cluster_snapshot_identifier: String,
    #[serde(rename = "ValuesToAdd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_add: Option<AttributeValueList>,
    #[serde(rename = "ValuesToRemove")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values_to_remove: Option<AttributeValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "StopDBClusterMessage")]
pub struct StopDBClusterMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeValidDBInstanceModificationsResult")]
pub struct DescribeValidDBInstanceModificationsResult {
    #[serde(rename = "ValidDBInstanceModificationsMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_d_b_instance_modifications_message: Option<ValidDBInstanceModificationsMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidDBInstanceModificationsMessage")]
pub struct ValidDBInstanceModificationsMessage {
    #[serde(rename = "Storage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage: Option<ValidStorageOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidStorageOptionsList {
    #[serde(
        rename = "ValidStorageOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ValidStorageOptions>,
}
impl From<Vec<ValidStorageOptions>> for ValidStorageOptionsList {
    fn from(v: Vec<ValidStorageOptions>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ValidStorageOptions> for ValidStorageOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidStorageOptions>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ValidStorageOptions>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlValidStorageOptionsList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ValidStorageOptions>,
}

impl From<Vec<ValidStorageOptions>> for XmlValidStorageOptionsList {
    fn from(v: Vec<ValidStorageOptions>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ValidStorageOptions> for XmlValidStorageOptionsList {
    fn from_iter<I: IntoIterator<Item = ValidStorageOptions>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ValidStorageOptions")]
pub struct ValidStorageOptions {
    #[serde(rename = "IopsToStorageRatio")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iops_to_storage_ratio: Option<DoubleRangeList>,
    #[serde(rename = "ProvisionedIops")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_iops: Option<RangeList>,
    #[serde(rename = "StorageSize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_size: Option<RangeList>,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DoubleRangeList {
    #[serde(rename = "DoubleRange", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DoubleRange>,
}
impl From<Vec<DoubleRange>> for DoubleRangeList {
    fn from(v: Vec<DoubleRange>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DoubleRange> for DoubleRangeList {
    fn from_iter<I: IntoIterator<Item = DoubleRange>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DoubleRange>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDoubleRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DoubleRange>,
}

impl From<Vec<DoubleRange>> for XmlDoubleRangeList {
    fn from(v: Vec<DoubleRange>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DoubleRange> for XmlDoubleRangeList {
    fn from_iter<I: IntoIterator<Item = DoubleRange>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DoubleRange")]
pub struct DoubleRange {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RangeList {
    #[serde(rename = "Range", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Range>,
}
impl From<Vec<Range>> for RangeList {
    fn from(v: Vec<Range>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Range> for RangeList {
    fn from_iter<I: IntoIterator<Item = Range>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Range>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Range>,
}

impl From<Vec<Range>> for XmlRangeList {
    fn from(v: Vec<Range>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Range> for XmlRangeList {
    fn from_iter<I: IntoIterator<Item = Range>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Range")]
pub struct Range {
    #[serde(rename = "From")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(rename = "Step")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<i32>,
    #[serde(rename = "To")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBParameterGroupResult")]
pub struct CreateDBParameterGroupResult {
    #[serde(rename = "DBParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group: Option<DBParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultParametersMessage")]
pub struct DescribeEngineDefaultParametersMessage {
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    pub d_b_parameter_group_family: String,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventCategoriesMessage")]
pub struct DescribeEventCategoriesMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBParameterGroupsResult")]
pub struct DBParameterGroupsMessage {
    #[serde(rename = "DBParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_groups: Option<DBParameterGroupList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBParameterGroupList {
    #[serde(
        rename = "DBParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBParameterGroup>,
}
impl From<Vec<DBParameterGroup>> for DBParameterGroupList {
    fn from(v: Vec<DBParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBParameterGroup> for DBParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBParameterGroup>,
}

impl From<Vec<DBParameterGroup>> for XmlDBParameterGroupList {
    fn from(v: Vec<DBParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBParameterGroup> for XmlDBParameterGroupList {
    fn from_iter<I: IntoIterator<Item = DBParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBClusterSnapshotsMessage")]
pub struct DescribeDBClusterSnapshotsMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "DBClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<FilterList>,
    #[serde(rename = "IncludePublic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_public: Option<bool>,
    #[serde(rename = "IncludeShared")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_shared: Option<bool>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBClusterResult")]
pub struct DeleteDBClusterResult {
    #[serde(rename = "DBCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster: Option<DBCluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteDBInstanceMessage")]
pub struct DeleteDBInstanceMessage {
    #[serde(rename = "DBInstanceIdentifier")]
    #[serde(default)]
    pub d_b_instance_identifier: String,
    #[serde(rename = "FinalDBSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_d_b_snapshot_identifier: Option<String>,
    #[serde(rename = "SkipFinalSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreDBClusterToPointInTimeMessage")]
pub struct RestoreDBClusterToPointInTimeMessage {
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "DBClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_parameter_group_name: Option<String>,
    #[serde(rename = "DBSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_subnet_group_name: Option<String>,
    #[serde(rename = "DeletionProtection")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deletion_protection: Option<bool>,
    #[serde(rename = "EnableCloudwatchLogsExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cloudwatch_logs_exports: Option<LogTypeList>,
    #[serde(rename = "EnableIAMDatabaseAuthentication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_i_a_m_database_authentication: Option<bool>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "OptionGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_group_name: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "RestoreToTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_to_time: Option<String>,
    #[serde(rename = "RestoreType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_type: Option<String>,
    #[serde(rename = "ServerlessV2ScalingConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    #[serde(rename = "SourceDBClusterIdentifier")]
    #[serde(default)]
    pub source_d_b_cluster_identifier: String,
    #[serde(rename = "StorageType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_type: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UseLatestRestorableTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_latest_restorable_time: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AddSourceIdentifierToSubscriptionResult")]
pub struct AddSourceIdentifierToSubscriptionResult {
    #[serde(rename = "EventSubscription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_subscription: Option<EventSubscription>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEngineDefaultClusterParametersResult")]
pub struct DescribeEngineDefaultClusterParametersResult {
    #[serde(rename = "EngineDefaults")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_defaults: Option<EngineDefaults>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDBEngineVersionsResult")]
pub struct DBEngineVersionMessage {
    #[serde(rename = "DBEngineVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_versions: Option<DBEngineVersionList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DBEngineVersionList {
    #[serde(
        rename = "DBEngineVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DBEngineVersion>,
}
impl From<Vec<DBEngineVersion>> for DBEngineVersionList {
    fn from(v: Vec<DBEngineVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DBEngineVersion> for DBEngineVersionList {
    fn from_iter<I: IntoIterator<Item = DBEngineVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DBEngineVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDBEngineVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DBEngineVersion>,
}

impl From<Vec<DBEngineVersion>> for XmlDBEngineVersionList {
    fn from(v: Vec<DBEngineVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DBEngineVersion> for XmlDBEngineVersionList {
    fn from_iter<I: IntoIterator<Item = DBEngineVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DBEngineVersion")]
pub struct DBEngineVersion {
    #[serde(rename = "DBEngineDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_description: Option<String>,
    #[serde(rename = "DBEngineVersionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_engine_version_description: Option<String>,
    #[serde(rename = "DBParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_parameter_group_family: Option<String>,
    #[serde(rename = "DefaultCharacterSet")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_character_set: Option<CharacterSet>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "ExportableLogTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exportable_log_types: Option<LogTypeList>,
    #[serde(rename = "SupportedCharacterSets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_character_sets: Option<SupportedCharacterSetsList>,
    #[serde(rename = "SupportedTimezones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_timezones: Option<SupportedTimezonesList>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
    #[serde(rename = "SupportsLogExportsToCloudwatchLogs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_log_exports_to_cloudwatch_logs: Option<bool>,
    #[serde(rename = "SupportsReadReplica")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_read_replica: Option<bool>,
    #[serde(rename = "ValidUpgradeTarget")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_upgrade_target: Option<ValidUpgradeTargetList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CharacterSet")]
pub struct CharacterSet {
    #[serde(rename = "CharacterSetDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_description: Option<String>,
    #[serde(rename = "CharacterSetName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub character_set_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedCharacterSetsList {
    #[serde(
        rename = "CharacterSet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CharacterSet>,
}
impl From<Vec<CharacterSet>> for SupportedCharacterSetsList {
    fn from(v: Vec<CharacterSet>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CharacterSet> for SupportedCharacterSetsList {
    fn from_iter<I: IntoIterator<Item = CharacterSet>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CharacterSet>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCharacterSetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CharacterSet>,
}

impl From<Vec<CharacterSet>> for XmlCharacterSetList {
    fn from(v: Vec<CharacterSet>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CharacterSet> for XmlCharacterSetList {
    fn from_iter<I: IntoIterator<Item = CharacterSet>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedTimezonesList {
    #[serde(rename = "Timezone", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Timezone>,
}
impl From<Vec<Timezone>> for SupportedTimezonesList {
    fn from(v: Vec<Timezone>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Timezone> for SupportedTimezonesList {
    fn from_iter<I: IntoIterator<Item = Timezone>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Timezone>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTimezoneList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Timezone>,
}

impl From<Vec<Timezone>> for XmlTimezoneList {
    fn from(v: Vec<Timezone>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Timezone> for XmlTimezoneList {
    fn from_iter<I: IntoIterator<Item = Timezone>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Timezone")]
pub struct Timezone {
    #[serde(rename = "TimezoneName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValidUpgradeTargetList {
    #[serde(
        rename = "UpgradeTarget",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UpgradeTarget>,
}
impl From<Vec<UpgradeTarget>> for ValidUpgradeTargetList {
    fn from(v: Vec<UpgradeTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UpgradeTarget> for ValidUpgradeTargetList {
    fn from_iter<I: IntoIterator<Item = UpgradeTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UpgradeTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUpgradeTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UpgradeTarget>,
}

impl From<Vec<UpgradeTarget>> for XmlUpgradeTargetList {
    fn from(v: Vec<UpgradeTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UpgradeTarget> for XmlUpgradeTargetList {
    fn from_iter<I: IntoIterator<Item = UpgradeTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpgradeTarget")]
pub struct UpgradeTarget {
    #[serde(rename = "AutoUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_upgrade: Option<bool>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Engine")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    #[serde(rename = "EngineVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_version: Option<String>,
    #[serde(rename = "IsMajorVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_major_version_upgrade: Option<bool>,
    #[serde(rename = "SupportsGlobalDatabases")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_global_databases: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterEndpointMessage")]
pub struct CreateDBClusterEndpointMessage {
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    pub d_b_cluster_endpoint_identifier: String,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    pub d_b_cluster_identifier: String,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    pub endpoint_type: String,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SwitchoverGlobalClusterMessage")]
pub struct SwitchoverGlobalClusterMessage {
    #[serde(rename = "GlobalClusterIdentifier")]
    #[serde(default)]
    pub global_cluster_identifier: String,
    #[serde(rename = "TargetDbClusterIdentifier")]
    #[serde(default)]
    pub target_db_cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateDBClusterEndpointResult")]
pub struct CreateDBClusterEndpointOutput {
    #[serde(rename = "CustomEndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_endpoint_type: Option<String>,
    #[serde(rename = "DBClusterEndpointArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_arn: Option<String>,
    #[serde(rename = "DBClusterEndpointIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_identifier: Option<String>,
    #[serde(rename = "DBClusterEndpointResourceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_endpoint_resource_identifier: Option<String>,
    #[serde(rename = "DBClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_cluster_identifier: Option<String>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    #[serde(rename = "EndpointType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_type: Option<String>,
    #[serde(rename = "ExcludedMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_members: Option<StringList>,
    #[serde(rename = "StaticMembers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_members: Option<StringList>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
