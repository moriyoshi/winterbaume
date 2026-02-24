//! Auto-generated types from Smithy models.
//! Do not edit manually. Regenerate with:
//!   smithy-codegen gen-serializers winterbaume-redshift

#![allow(non_camel_case_types, clippy::upper_case_acronyms, dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterVersionsResult")]
pub struct ClusterVersionsMessage {
    #[serde(rename = "ClusterVersions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_versions: Option<ClusterVersionList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterVersionList {
    #[serde(
        rename = "ClusterVersion",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterVersion>,
}
impl From<Vec<ClusterVersion>> for ClusterVersionList {
    fn from(v: Vec<ClusterVersion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterVersion> for ClusterVersionList {
    fn from_iter<I: IntoIterator<Item = ClusterVersion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterVersion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterVersionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterVersion>,
}

impl From<Vec<ClusterVersion>> for XmlClusterVersionList {
    fn from(v: Vec<ClusterVersion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterVersion> for XmlClusterVersionList {
    fn from_iter<I: IntoIterator<Item = ClusterVersion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterVersion")]
pub struct ClusterVersion {
    #[serde(rename = "ClusterParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_family: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PartnerIntegrationInputMessage")]
pub struct PartnerIntegrationInputMessage {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartnerName")]
    #[serde(default)]
    pub partner_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCustomDomainAssociationsMessage")]
pub struct DescribeCustomDomainAssociationsMessage {
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_arn: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
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
#[serde(rename = "DescribeStorageResult")]
pub struct CustomerStorageMessage {
    #[serde(rename = "TotalBackupSizeInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_backup_size_in_mega_bytes: Option<f64>,
    #[serde(rename = "TotalProvisionedStorageInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_provisioned_storage_in_mega_bytes: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResizeClusterResult")]
pub struct ResizeClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Cluster")]
pub struct Cluster {
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    #[serde(rename = "AquaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration: Option<AquaConfiguration>,
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneRelocationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_relocation_status: Option<String>,
    #[serde(rename = "CatalogArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_arn: Option<String>,
    #[serde(rename = "ClusterAvailabilityStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_availability_status: Option<String>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterNamespaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_namespace_arn: Option<String>,
    #[serde(rename = "ClusterNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_nodes: Option<ClusterNodesList>,
    #[serde(rename = "ClusterParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_groups: Option<ClusterParameterGroupStatusList>,
    #[serde(rename = "ClusterPublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_public_key: Option<String>,
    #[serde(rename = "ClusterRevisionNumber")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_revision_number: Option<String>,
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<ClusterSecurityGroupMembershipList>,
    #[serde(rename = "ClusterSnapshotCopyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_snapshot_copy_status: Option<ClusterSnapshotCopyStatus>,
    #[serde(rename = "ClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_arn: Option<String>,
    #[serde(rename = "CustomDomainCertificateExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_expiry_date: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DataTransferProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_progress: Option<DataTransferProgress>,
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<String>,
    #[serde(rename = "DeferredMaintenanceWindows")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred_maintenance_windows: Option<DeferredMaintenanceWindowsList>,
    #[serde(rename = "ElasticIpStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip_status: Option<ElasticIpStatus>,
    #[serde(rename = "ElasticResizeNumberOfNodeOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_resize_number_of_node_options: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "Endpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<Endpoint>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "ExpectedNextSnapshotScheduleTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time: Option<String>,
    #[serde(rename = "ExpectedNextSnapshotScheduleTimeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_next_snapshot_schedule_time_status: Option<String>,
    #[serde(rename = "ExtraComputeForAutomaticOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_compute_for_automatic_optimization: Option<String>,
    #[serde(rename = "HsmStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_status: Option<HsmStatus>,
    #[serde(rename = "IamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<ClusterIamRoleList>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LakehouseRegistrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_registration_status: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterPasswordSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_arn: Option<String>,
    #[serde(rename = "MasterPasswordSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "ModifyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_status: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<String>,
    #[serde(rename = "MultiAZSecondary")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z_secondary: Option<SecondaryClusterInfo>,
    #[serde(rename = "NextMaintenanceWindowStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_maintenance_window_start_time: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "PendingActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_actions: Option<PendingActionsList>,
    #[serde(rename = "PendingModifiedValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_modified_values: Option<PendingModifiedValues>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "ReservedNodeExchangeStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_exchange_status: Option<ReservedNodeExchangeStatus>,
    #[serde(rename = "ResizeInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_info: Option<ResizeInfo>,
    #[serde(rename = "RestoreStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_status: Option<RestoreStatus>,
    #[serde(rename = "SnapshotScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_identifier: Option<String>,
    #[serde(rename = "SnapshotScheduleState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_state: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TotalStorageCapacityInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_storage_capacity_in_mega_bytes: Option<i64>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AquaConfiguration")]
pub struct AquaConfiguration {
    #[serde(rename = "AquaConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration_status: Option<String>,
    #[serde(rename = "AquaStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterNodesList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ClusterNode>,
}
impl From<Vec<ClusterNode>> for ClusterNodesList {
    fn from(v: Vec<ClusterNode>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterNode> for ClusterNodesList {
    fn from_iter<I: IntoIterator<Item = ClusterNode>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterNode>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterNodeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterNode>,
}

impl From<Vec<ClusterNode>> for XmlClusterNodeList {
    fn from(v: Vec<ClusterNode>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterNode> for XmlClusterNodeList {
    fn from_iter<I: IntoIterator<Item = ClusterNode>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterNode")]
pub struct ClusterNode {
    #[serde(rename = "NodeRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[serde(rename = "PrivateIPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_i_p_address: Option<String>,
    #[serde(rename = "PublicIPAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_i_p_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterParameterGroupStatusList {
    #[serde(
        rename = "ClusterParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterParameterGroupStatus>,
}
impl From<Vec<ClusterParameterGroupStatus>> for ClusterParameterGroupStatusList {
    fn from(v: Vec<ClusterParameterGroupStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterParameterGroupStatus> for ClusterParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterGroupStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterParameterGroupStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterParameterGroupStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterParameterGroupStatus>,
}

impl From<Vec<ClusterParameterGroupStatus>> for XmlClusterParameterGroupStatusList {
    fn from(v: Vec<ClusterParameterGroupStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterParameterGroupStatus> for XmlClusterParameterGroupStatusList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterGroupStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterParameterGroupStatus")]
pub struct ClusterParameterGroupStatus {
    #[serde(rename = "ClusterParameterStatusList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_status_list: Option<ClusterParameterStatusList>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterParameterStatusList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ClusterParameterStatus>,
}
impl From<Vec<ClusterParameterStatus>> for ClusterParameterStatusList {
    fn from(v: Vec<ClusterParameterStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterParameterStatus> for ClusterParameterStatusList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterParameterStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterParameterStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterParameterStatus>,
}

impl From<Vec<ClusterParameterStatus>> for XmlClusterParameterStatusList {
    fn from(v: Vec<ClusterParameterStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterParameterStatus> for XmlClusterParameterStatusList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterParameterStatus")]
pub struct ClusterParameterStatus {
    #[serde(rename = "ParameterApplyErrorDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_error_description: Option<String>,
    #[serde(rename = "ParameterApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_apply_status: Option<String>,
    #[serde(rename = "ParameterName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSecurityGroupMembershipList {
    #[serde(
        rename = "ClusterSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterSecurityGroupMembership>,
}
impl From<Vec<ClusterSecurityGroupMembership>> for ClusterSecurityGroupMembershipList {
    fn from(v: Vec<ClusterSecurityGroupMembership>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterSecurityGroupMembership> for ClusterSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = ClusterSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterSecurityGroupMembership>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterSecurityGroupMembershipList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterSecurityGroupMembership>,
}

impl From<Vec<ClusterSecurityGroupMembership>> for XmlClusterSecurityGroupMembershipList {
    fn from(v: Vec<ClusterSecurityGroupMembership>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterSecurityGroupMembership> for XmlClusterSecurityGroupMembershipList {
    fn from_iter<I: IntoIterator<Item = ClusterSecurityGroupMembership>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterSecurityGroupMembership")]
pub struct ClusterSecurityGroupMembership {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterSnapshotCopyStatus")]
pub struct ClusterSnapshotCopyStatus {
    #[serde(rename = "DestinationRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_region: Option<String>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i64>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DataTransferProgress")]
pub struct DataTransferProgress {
    #[serde(rename = "CurrentRateInMegaBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_rate_in_mega_bytes_per_second: Option<f64>,
    #[serde(rename = "DataTransferredInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transferred_in_mega_bytes: Option<i64>,
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    #[serde(rename = "EstimatedTimeToCompletionInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TotalDataInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_data_in_mega_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeferredMaintenanceWindowsList {
    #[serde(
        rename = "DeferredMaintenanceWindow",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DeferredMaintenanceWindow>,
}
impl From<Vec<DeferredMaintenanceWindow>> for DeferredMaintenanceWindowsList {
    fn from(v: Vec<DeferredMaintenanceWindow>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DeferredMaintenanceWindow> for DeferredMaintenanceWindowsList {
    fn from_iter<I: IntoIterator<Item = DeferredMaintenanceWindow>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DeferredMaintenanceWindow>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDeferredMaintenanceWindowList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DeferredMaintenanceWindow>,
}

impl From<Vec<DeferredMaintenanceWindow>> for XmlDeferredMaintenanceWindowList {
    fn from(v: Vec<DeferredMaintenanceWindow>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DeferredMaintenanceWindow> for XmlDeferredMaintenanceWindowList {
    fn from_iter<I: IntoIterator<Item = DeferredMaintenanceWindow>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeferredMaintenanceWindow")]
pub struct DeferredMaintenanceWindow {
    #[serde(rename = "DeferMaintenanceEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_end_time: Option<String>,
    #[serde(rename = "DeferMaintenanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_identifier: Option<String>,
    #[serde(rename = "DeferMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ElasticIpStatus")]
pub struct ElasticIpStatus {
    #[serde(rename = "ElasticIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
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
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "VpcEndpoints")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoints: Option<VpcEndpointsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcEndpointsList {
    #[serde(rename = "VpcEndpoint", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<VpcEndpoint>,
}
impl From<Vec<VpcEndpoint>> for VpcEndpointsList {
    fn from(v: Vec<VpcEndpoint>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<VpcEndpoint> for VpcEndpointsList {
    fn from_iter<I: IntoIterator<Item = VpcEndpoint>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<VpcEndpoint>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlVpcEndpointList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<VpcEndpoint>,
}

impl From<Vec<VpcEndpoint>> for XmlVpcEndpointList {
    fn from(v: Vec<VpcEndpoint>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<VpcEndpoint> for XmlVpcEndpointList {
    fn from_iter<I: IntoIterator<Item = VpcEndpoint>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "VpcEndpoint")]
pub struct VpcEndpoint {
    #[serde(rename = "NetworkInterfaces")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<NetworkInterfaceList>,
    #[serde(rename = "VpcEndpointId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint_id: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NetworkInterfaceList {
    #[serde(
        rename = "NetworkInterface",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NetworkInterface>,
}
impl From<Vec<NetworkInterface>> for NetworkInterfaceList {
    fn from(v: Vec<NetworkInterface>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NetworkInterface> for NetworkInterfaceList {
    fn from_iter<I: IntoIterator<Item = NetworkInterface>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NetworkInterface>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNetworkInterfaceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NetworkInterface>,
}

impl From<Vec<NetworkInterface>> for XmlNetworkInterfaceList {
    fn from(v: Vec<NetworkInterface>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NetworkInterface> for XmlNetworkInterfaceList {
    fn from_iter<I: IntoIterator<Item = NetworkInterface>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NetworkInterface")]
pub struct NetworkInterface {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "Ipv6Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv6_address: Option<String>,
    #[serde(rename = "NetworkInterfaceId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    #[serde(rename = "PrivateIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    #[serde(rename = "SubnetId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HsmStatus")]
pub struct HsmStatus {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterIamRoleList {
    #[serde(
        rename = "ClusterIamRole",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterIamRole>,
}
impl From<Vec<ClusterIamRole>> for ClusterIamRoleList {
    fn from(v: Vec<ClusterIamRole>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterIamRole> for ClusterIamRoleList {
    fn from_iter<I: IntoIterator<Item = ClusterIamRole>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterIamRole>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterIamRoleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterIamRole>,
}

impl From<Vec<ClusterIamRole>> for XmlClusterIamRoleList {
    fn from(v: Vec<ClusterIamRole>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterIamRole> for XmlClusterIamRoleList {
    fn from_iter<I: IntoIterator<Item = ClusterIamRole>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterIamRole")]
pub struct ClusterIamRole {
    #[serde(rename = "ApplyStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apply_status: Option<String>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SecondaryClusterInfo")]
pub struct SecondaryClusterInfo {
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "ClusterNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_nodes: Option<ClusterNodesList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PendingActionsList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for PendingActionsList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for PendingActionsList {
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
#[serde(rename = "PendingModifiedValues")]
pub struct PendingModifiedValues {
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "EncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encryption_type: Option<String>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedNodeExchangeStatus")]
pub struct ReservedNodeExchangeStatus {
    #[serde(rename = "RequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<String>,
    #[serde(rename = "ReservedNodeExchangeRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_exchange_request_id: Option<String>,
    #[serde(rename = "SourceReservedNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reserved_node_count: Option<i32>,
    #[serde(rename = "SourceReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reserved_node_id: Option<String>,
    #[serde(rename = "SourceReservedNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reserved_node_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetReservedNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_count: Option<i32>,
    #[serde(rename = "TargetReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_offering_id: Option<String>,
    #[serde(rename = "TargetReservedNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResizeInfo")]
pub struct ResizeInfo {
    #[serde(rename = "AllowCancelResize")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_cancel_resize: Option<bool>,
    #[serde(rename = "ResizeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreStatus")]
pub struct RestoreStatus {
    #[serde(rename = "CurrentRestoreRateInMegaBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_restore_rate_in_mega_bytes_per_second: Option<f64>,
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    #[serde(rename = "EstimatedTimeToCompletionInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    #[serde(rename = "ProgressInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_mega_bytes: Option<i64>,
    #[serde(rename = "SnapshotSizeInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_size_in_mega_bytes: Option<i64>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
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
pub struct VpcSecurityGroupMembershipList {
    #[serde(
        rename = "VpcSecurityGroup",
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
#[serde(rename = "EnableLoggingMessage")]
pub struct EnableLoggingMessage {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "LogDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination_type: Option<String>,
    #[serde(rename = "LogExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_exports: Option<LogTypeList>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
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
#[serde(rename = "AcceptReservedNodeExchangeResult")]
pub struct AcceptReservedNodeExchangeOutputMessage {
    #[serde(rename = "ExchangedReservedNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchanged_reserved_node: Option<ReservedNode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedNode")]
pub struct ReservedNode {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "NodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<RecurringChargeList>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_id: Option<String>,
    #[serde(rename = "ReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offering_id: Option<String>,
    #[serde(rename = "ReservedNodeOfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offering_type: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecurringChargeList {
    #[serde(
        rename = "RecurringCharge",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RecurringCharge>,
}
impl From<Vec<RecurringCharge>> for RecurringChargeList {
    fn from(v: Vec<RecurringCharge>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecurringCharge> for RecurringChargeList {
    fn from_iter<I: IntoIterator<Item = RecurringCharge>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecurringCharge>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecurringChargeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecurringCharge>,
}

impl From<Vec<RecurringCharge>> for XmlRecurringChargeList {
    fn from(v: Vec<RecurringCharge>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecurringCharge> for XmlRecurringChargeList {
    fn from_iter<I: IntoIterator<Item = RecurringCharge>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecurringCharge")]
pub struct RecurringCharge {
    #[serde(rename = "RecurringChargeAmount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_amount: Option<f64>,
    #[serde(rename = "RecurringChargeFrequency")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charge_frequency: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterParameterGroupMessage")]
pub struct ModifyClusterParameterGroupMessage {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    pub parameters: ParametersList,
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
#[serde(rename = "ResetClusterParameterGroupMessage")]
pub struct ResetClusterParameterGroupMessage {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
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
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
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
#[serde(rename = "AuthorizeClusterSecurityGroupIngressMessage")]
pub struct AuthorizeClusterSecurityGroupIngressMessage {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    pub cluster_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteSnapshotScheduleMessage")]
pub struct DeleteSnapshotScheduleMessage {
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    pub schedule_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterParameterGroupMessage")]
pub struct DeleteClusterParameterGroupMessage {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUsageLimitsResult")]
pub struct UsageLimitList {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "UsageLimits")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limits: Option<UsageLimits>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UsageLimits {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<UsageLimit>,
}
impl From<Vec<UsageLimit>> for UsageLimits {
    fn from(v: Vec<UsageLimit>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UsageLimit> for UsageLimits {
    fn from_iter<I: IntoIterator<Item = UsageLimit>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UsageLimit>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUsageLimitList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UsageLimit>,
}

impl From<Vec<UsageLimit>> for XmlUsageLimitList {
    fn from(v: Vec<UsageLimit>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UsageLimit> for XmlUsageLimitList {
    fn from_iter<I: IntoIterator<Item = UsageLimit>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUsageLimitResult")]
pub struct UsageLimit {
    #[serde(rename = "Amount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "BreachAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breach_action: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "FeatureType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[serde(rename = "LimitType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_type: Option<String>,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "UsageLimitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesResult")]
pub struct DescribeDataSharesResult {
    #[serde(rename = "DataShares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_shares: Option<DataShareList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataShareList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DataShare>,
}
impl From<Vec<DataShare>> for DataShareList {
    fn from(v: Vec<DataShare>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DataShare> for DataShareList {
    fn from_iter<I: IntoIterator<Item = DataShare>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DataShare>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDataShareList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DataShare>,
}

impl From<Vec<DataShare>> for XmlDataShareList {
    fn from(v: Vec<DataShare>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DataShare> for XmlDataShareList {
    fn from_iter<I: IntoIterator<Item = DataShare>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RejectDataShareResult")]
pub struct DataShare {
    #[serde(rename = "AllowPubliclyAccessibleConsumers")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_publicly_accessible_consumers: Option<bool>,
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_share_arn: Option<String>,
    #[serde(rename = "DataShareAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_share_associations: Option<DataShareAssociationList>,
    #[serde(rename = "DataShareType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_share_type: Option<String>,
    #[serde(rename = "ManagedBy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<String>,
    #[serde(rename = "ProducerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DataShareAssociationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<DataShareAssociation>,
}
impl From<Vec<DataShareAssociation>> for DataShareAssociationList {
    fn from(v: Vec<DataShareAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DataShareAssociation> for DataShareAssociationList {
    fn from_iter<I: IntoIterator<Item = DataShareAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DataShareAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDataShareAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DataShareAssociation>,
}

impl From<Vec<DataShareAssociation>> for XmlDataShareAssociationList {
    fn from(v: Vec<DataShareAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DataShareAssociation> for XmlDataShareAssociationList {
    fn from_iter<I: IntoIterator<Item = DataShareAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DataShareAssociation")]
pub struct DataShareAssociation {
    #[serde(rename = "ConsumerAcceptedWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_accepted_writes: Option<bool>,
    #[serde(rename = "ConsumerIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_identifier: Option<String>,
    #[serde(rename = "ConsumerRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_region: Option<String>,
    #[serde(rename = "CreatedDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "ProducerAllowedWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_allowed_writes: Option<bool>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusChangeDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_change_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetClusterCredentialsResult")]
pub struct ClusterCredentials {
    #[serde(rename = "DbPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
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
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
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
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
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
#[serde(rename = "DescribeTagsResult")]
pub struct TaggedResourceListMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TaggedResources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tagged_resources: Option<TaggedResourceList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TaggedResourceList {
    #[serde(
        rename = "TaggedResource",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TaggedResource>,
}
impl From<Vec<TaggedResource>> for TaggedResourceList {
    fn from(v: Vec<TaggedResource>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TaggedResource> for TaggedResourceList {
    fn from_iter<I: IntoIterator<Item = TaggedResource>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TaggedResource>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTaggedResourceList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TaggedResource>,
}

impl From<Vec<TaggedResource>> for XmlTaggedResourceList {
    fn from(v: Vec<TaggedResource>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TaggedResource> for XmlTaggedResourceList {
    fn from_iter<I: IntoIterator<Item = TaggedResource>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TaggedResource")]
pub struct TaggedResource {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "Tag")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<Tag>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodeOfferingsResult")]
pub struct ReservedNodeOfferingsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedNodeOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offerings: Option<ReservedNodeOfferingList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNodeOfferingList {
    #[serde(
        rename = "ReservedNodeOffering",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedNodeOffering>,
}
impl From<Vec<ReservedNodeOffering>> for ReservedNodeOfferingList {
    fn from(v: Vec<ReservedNodeOffering>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedNodeOffering> for ReservedNodeOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeOffering>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedNodeOffering>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedNodeOfferingList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedNodeOffering>,
}

impl From<Vec<ReservedNodeOffering>> for XmlReservedNodeOfferingList {
    fn from(v: Vec<ReservedNodeOffering>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedNodeOffering> for XmlReservedNodeOfferingList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeOffering>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedNodeOffering")]
pub struct ReservedNodeOffering {
    #[serde(rename = "CurrencyCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(rename = "Duration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    #[serde(rename = "FixedPrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_price: Option<f64>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "OfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offering_type: Option<String>,
    #[serde(rename = "RecurringCharges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring_charges: Option<RecurringChargeList>,
    #[serde(rename = "ReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offering_id: Option<String>,
    #[serde(rename = "ReservedNodeOfferingType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offering_type: Option<String>,
    #[serde(rename = "UsagePrice")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_price: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteHsmConfigurationMessage")]
pub struct DeleteHsmConfigurationMessage {
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    pub hsm_configuration_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableSnapshotCopyMessage")]
pub struct DisableSnapshotCopyMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReservedNodeExchangeConfigurationOptionsResult")]
pub struct GetReservedNodeExchangeConfigurationOptionsOutputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedNodeConfigurationOptionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_configuration_option_list: Option<ReservedNodeConfigurationOptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNodeConfigurationOptionList {
    #[serde(
        rename = "ReservedNodeConfigurationOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedNodeConfigurationOption>,
}
impl From<Vec<ReservedNodeConfigurationOption>> for ReservedNodeConfigurationOptionList {
    fn from(v: Vec<ReservedNodeConfigurationOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedNodeConfigurationOption> for ReservedNodeConfigurationOptionList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeConfigurationOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedNodeConfigurationOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedNodeConfigurationOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedNodeConfigurationOption>,
}

impl From<Vec<ReservedNodeConfigurationOption>> for XmlReservedNodeConfigurationOptionList {
    fn from(v: Vec<ReservedNodeConfigurationOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedNodeConfigurationOption> for XmlReservedNodeConfigurationOptionList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeConfigurationOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReservedNodeConfigurationOption")]
pub struct ReservedNodeConfigurationOption {
    #[serde(rename = "SourceReservedNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_reserved_node: Option<ReservedNode>,
    #[serde(rename = "TargetReservedNodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_count: Option<i32>,
    #[serde(rename = "TargetReservedNodeOffering")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_offering: Option<ReservedNodeOffering>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterParameterGroupResult")]
pub struct CreateClusterParameterGroupResult {
    #[serde(rename = "ClusterParameterGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group: Option<ClusterParameterGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterParameterGroup")]
pub struct ClusterParameterGroup {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_family: Option<String>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEventSubscriptionMessage")]
pub struct DeleteEventSubscriptionMessage {
    #[serde(rename = "SubscriptionName")]
    #[serde(default)]
    pub subscription_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSnapshotsMessage")]
pub struct DescribeClusterSnapshotsMessage {
    #[serde(rename = "ClusterExists")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_exists: Option<bool>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SortingEntities")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sorting_entities: Option<SnapshotSortingEntityList>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotSortingEntityList {
    #[serde(
        rename = "SnapshotSortingEntity",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SnapshotSortingEntity>,
}
impl From<Vec<SnapshotSortingEntity>> for SnapshotSortingEntityList {
    fn from(v: Vec<SnapshotSortingEntity>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SnapshotSortingEntity> for SnapshotSortingEntityList {
    fn from_iter<I: IntoIterator<Item = SnapshotSortingEntity>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SnapshotSortingEntity>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotSortingEntityList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SnapshotSortingEntity>,
}

impl From<Vec<SnapshotSortingEntity>> for XmlSnapshotSortingEntityList {
    fn from(v: Vec<SnapshotSortingEntity>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SnapshotSortingEntity> for XmlSnapshotSortingEntityList {
    fn from_iter<I: IntoIterator<Item = SnapshotSortingEntity>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SnapshotSortingEntity")]
pub struct SnapshotSortingEntity {
    #[serde(rename = "Attribute")]
    #[serde(default)]
    pub attribute: String,
    #[serde(rename = "SortOrder")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagKeyList {
    #[serde(rename = "TagKey", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagKeyList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagKeyList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TagValueList {
    #[serde(rename = "TagValue", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for TagValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for TagValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeEndpointAccessMessage")]
pub struct AuthorizeEndpointAccessMessage {
    #[serde(rename = "Account")]
    #[serde(default)]
    pub account: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "VpcIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ids: Option<VpcIdentifierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct VpcIdentifierList {
    #[serde(
        rename = "VpcIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for VpcIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for VpcIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetClusterCredentialsWithIAMResult")]
pub struct ClusterExtendedCredentials {
    #[serde(rename = "DbPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_password: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_user: Option<String>,
    #[serde(rename = "Expiration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "NextRefreshTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_refresh_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeCustomDomainAssociationsResult")]
pub struct CustomDomainAssociationsMessage {
    #[serde(rename = "Associations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associations: Option<AssociationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociationList {
    #[serde(rename = "Association", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Association>,
}
impl From<Vec<Association>> for AssociationList {
    fn from(v: Vec<Association>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Association> for AssociationList {
    fn from_iter<I: IntoIterator<Item = Association>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Association>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Association>,
}

impl From<Vec<Association>> for XmlAssociationList {
    fn from(v: Vec<Association>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Association> for XmlAssociationList {
    fn from_iter<I: IntoIterator<Item = Association>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Association")]
pub struct Association {
    #[serde(rename = "CertificateAssociations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_associations: Option<CertificateAssociationList>,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_arn: Option<String>,
    #[serde(rename = "CustomDomainCertificateExpiryDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_expiry_date: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CertificateAssociationList {
    #[serde(
        rename = "CertificateAssociation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<CertificateAssociation>,
}
impl From<Vec<CertificateAssociation>> for CertificateAssociationList {
    fn from(v: Vec<CertificateAssociation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<CertificateAssociation> for CertificateAssociationList {
    fn from_iter<I: IntoIterator<Item = CertificateAssociation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<CertificateAssociation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlCertificateAssociationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<CertificateAssociation>,
}

impl From<Vec<CertificateAssociation>> for XmlCertificateAssociationList {
    fn from(v: Vec<CertificateAssociation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<CertificateAssociation> for XmlCertificateAssociationList {
    fn from_iter<I: IntoIterator<Item = CertificateAssociation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CertificateAssociation")]
pub struct CertificateAssociation {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeIntegrationsResult")]
pub struct IntegrationsMessage {
    #[serde(rename = "Integrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrations: Option<IntegrationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationList {
    #[serde(rename = "Integration", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Integration>,
}
impl From<Vec<Integration>> for IntegrationList {
    fn from(v: Vec<Integration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Integration> for IntegrationList {
    fn from_iter<I: IntoIterator<Item = Integration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Integration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIntegrationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Integration>,
}

impl From<Vec<Integration>> for XmlIntegrationList {
    fn from(v: Vec<Integration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Integration> for XmlIntegrationList {
    fn from_iter<I: IntoIterator<Item = Integration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyIntegrationResult")]
pub struct Integration {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<IntegrationErrorList>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IntegrationErrorList {
    #[serde(
        rename = "IntegrationError",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<IntegrationError>,
}
impl From<Vec<IntegrationError>> for IntegrationErrorList {
    fn from(v: Vec<IntegrationError>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<IntegrationError> for IntegrationErrorList {
    fn from_iter<I: IntoIterator<Item = IntegrationError>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<IntegrationError>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIntegrationErrorList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<IntegrationError>,
}

impl From<Vec<IntegrationError>> for XmlIntegrationErrorList {
    fn from(v: Vec<IntegrationError>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<IntegrationError> for XmlIntegrationErrorList {
    fn from_iter<I: IntoIterator<Item = IntegrationError>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IntegrationError")]
pub struct IntegrationError {
    #[serde(rename = "ErrorCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(rename = "ErrorMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodesMessage")]
pub struct DescribeReservedNodesMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterParametersResult")]
pub struct ClusterParameterGroupDetails {
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
    #[serde(rename = "Events")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<EventInfoMapList>,
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EventInfoMapList {
    #[serde(
        rename = "EventInfoMap",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EventInfoMap>,
}
impl From<Vec<EventInfoMap>> for EventInfoMapList {
    fn from(v: Vec<EventInfoMap>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EventInfoMap> for EventInfoMapList {
    fn from_iter<I: IntoIterator<Item = EventInfoMap>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EventInfoMap>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEventInfoMapList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EventInfoMap>,
}

impl From<Vec<EventInfoMap>> for XmlEventInfoMapList {
    fn from(v: Vec<EventInfoMap>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EventInfoMap> for XmlEventInfoMapList {
    fn from_iter<I: IntoIterator<Item = EventInfoMap>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EventInfoMap")]
pub struct EventInfoMap {
    #[serde(rename = "EventCategories")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_categories: Option<EventCategoriesList>,
    #[serde(rename = "EventDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_description: Option<String>,
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReservedNodeExchangeOfferingsResult")]
pub struct GetReservedNodeExchangeOfferingsOutputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedNodeOfferings")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offerings: Option<ReservedNodeOfferingList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterResult")]
pub struct ModifyClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeSnapshotAccessResult")]
pub struct AuthorizeSnapshotAccessResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Snapshot")]
pub struct Snapshot {
    #[serde(rename = "AccountsWithRestoreAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts_with_restore_access: Option<AccountsWithRestoreAccessList>,
    #[serde(rename = "ActualIncrementalBackupSizeInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_incremental_backup_size_in_mega_bytes: Option<f64>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "BackupProgressInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backup_progress_in_mega_bytes: Option<f64>,
    #[serde(rename = "ClusterCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_create_time: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "CurrentBackupRateInMegaBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_backup_rate_in_mega_bytes_per_second: Option<f64>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "EncryptedWithHSM")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_with_h_s_m: Option<bool>,
    #[serde(rename = "EngineFullVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engine_full_version: Option<String>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "EstimatedSecondsToCompletion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_seconds_to_completion: Option<i64>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManualSnapshotRemainingDays")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_remaining_days: Option<i32>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterPasswordSecretArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_arn: Option<String>,
    #[serde(rename = "MasterPasswordSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_username: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "RestorableNodeTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restorable_node_types: Option<RestorableNodeTypeList>,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_create_time: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
    #[serde(rename = "SnapshotRetentionStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_retention_start_time: Option<String>,
    #[serde(rename = "SnapshotType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_type: Option<String>,
    #[serde(rename = "SourceRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
    #[serde(rename = "TotalBackupSizeInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_backup_size_in_mega_bytes: Option<f64>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AccountsWithRestoreAccessList {
    #[serde(
        rename = "AccountWithRestoreAccess",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AccountWithRestoreAccess>,
}
impl From<Vec<AccountWithRestoreAccess>> for AccountsWithRestoreAccessList {
    fn from(v: Vec<AccountWithRestoreAccess>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccountWithRestoreAccess> for AccountsWithRestoreAccessList {
    fn from_iter<I: IntoIterator<Item = AccountWithRestoreAccess>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccountWithRestoreAccess>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccountWithRestoreAccessList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccountWithRestoreAccess>,
}

impl From<Vec<AccountWithRestoreAccess>> for XmlAccountWithRestoreAccessList {
    fn from(v: Vec<AccountWithRestoreAccess>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccountWithRestoreAccess> for XmlAccountWithRestoreAccessList {
    fn from_iter<I: IntoIterator<Item = AccountWithRestoreAccess>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountWithRestoreAccess")]
pub struct AccountWithRestoreAccess {
    #[serde(rename = "AccountAlias")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<String>,
    #[serde(rename = "AccountId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RestorableNodeTypeList {
    #[serde(rename = "NodeType", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for RestorableNodeTypeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for RestorableNodeTypeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteCustomDomainAssociationMessage")]
pub struct DeleteCustomDomainAssociationMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    pub custom_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodeOfferingsMessage")]
pub struct DescribeReservedNodeOfferingsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyAquaConfigurationResult")]
pub struct ModifyAquaOutputMessage {
    #[serde(rename = "AquaConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration: Option<AquaConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterParameterGroupsResult")]
pub struct ClusterParameterGroupsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ParameterGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_groups: Option<ParameterGroupList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ParameterGroupList {
    #[serde(
        rename = "ClusterParameterGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterParameterGroup>,
}
impl From<Vec<ClusterParameterGroup>> for ParameterGroupList {
    fn from(v: Vec<ClusterParameterGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterParameterGroup> for ParameterGroupList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterParameterGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterParameterGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterParameterGroup>,
}

impl From<Vec<ClusterParameterGroup>> for XmlClusterParameterGroupList {
    fn from(v: Vec<ClusterParameterGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterParameterGroup> for XmlClusterParameterGroupList {
    fn from_iter<I: IntoIterator<Item = ClusterParameterGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesMessage")]
pub struct DescribeDataSharesMessage {
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_share_arn: Option<String>,
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
#[serde(rename = "ModifyClusterDbRevisionResult")]
pub struct ModifyClusterDbRevisionResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAuthenticationProfilesResult")]
pub struct DescribeAuthenticationProfilesResult {
    #[serde(rename = "AuthenticationProfiles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profiles: Option<AuthenticationProfileList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthenticationProfileList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AuthenticationProfile>,
}
impl From<Vec<AuthenticationProfile>> for AuthenticationProfileList {
    fn from(v: Vec<AuthenticationProfile>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AuthenticationProfile> for AuthenticationProfileList {
    fn from_iter<I: IntoIterator<Item = AuthenticationProfile>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AuthenticationProfile>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAuthenticationProfileList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AuthenticationProfile>,
}

impl From<Vec<AuthenticationProfile>> for XmlAuthenticationProfileList {
    fn from(v: Vec<AuthenticationProfile>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AuthenticationProfile> for XmlAuthenticationProfileList {
    fn from_iter<I: IntoIterator<Item = AuthenticationProfile>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthenticationProfile")]
pub struct AuthenticationProfile {
    #[serde(rename = "AuthenticationProfileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_content: Option<String>,
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCustomDomainAssociationMessage")]
pub struct ModifyCustomDomainAssociationMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    pub custom_domain_certificate_arn: String,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    pub custom_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesForProducerResult")]
pub struct DescribeDataSharesForProducerResult {
    #[serde(rename = "DataShares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_shares: Option<DataShareList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeSnapshotAccessMessage")]
pub struct RevokeSnapshotAccessMessage {
    #[serde(rename = "AccountWithRestoreAccess")]
    #[serde(default)]
    pub account_with_restore_access: String,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResizeClusterMessage")]
pub struct ResizeClusterMessage {
    #[serde(rename = "Classic")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classic: Option<bool>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_id: Option<String>,
    #[serde(rename = "TargetReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_offering_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterVersionsMessage")]
pub struct DescribeClusterVersionsMessage {
    #[serde(rename = "ClusterParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_family: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
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
#[serde(rename = "DescribeClusterTracksMessage")]
pub struct DescribeClusterTracksMessage {
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
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
#[serde(rename = "DescribeClustersMessage")]
pub struct DescribeClustersMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeRedshiftIdcApplicationsResult")]
pub struct DescribeRedshiftIdcApplicationsResult {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "RedshiftIdcApplications")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_applications: Option<RedshiftIdcApplicationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftIdcApplicationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RedshiftIdcApplication>,
}
impl From<Vec<RedshiftIdcApplication>> for RedshiftIdcApplicationList {
    fn from(v: Vec<RedshiftIdcApplication>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RedshiftIdcApplication> for RedshiftIdcApplicationList {
    fn from_iter<I: IntoIterator<Item = RedshiftIdcApplication>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RedshiftIdcApplication>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRedshiftIdcApplicationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RedshiftIdcApplication>,
}

impl From<Vec<RedshiftIdcApplication>> for XmlRedshiftIdcApplicationList {
    fn from(v: Vec<RedshiftIdcApplication>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RedshiftIdcApplication> for XmlRedshiftIdcApplicationList {
    fn from_iter<I: IntoIterator<Item = RedshiftIdcApplication>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RedshiftIdcApplication")]
pub struct RedshiftIdcApplication {
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "AuthorizedTokenIssuerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_token_issuer_list: Option<AuthorizedTokenIssuerList>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IdcDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_display_name: Option<String>,
    #[serde(rename = "IdcInstanceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_instance_arn: Option<String>,
    #[serde(rename = "IdcManagedApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_managed_application_arn: Option<String>,
    #[serde(rename = "IdcOnboardStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_onboard_status: Option<String>,
    #[serde(rename = "IdentityNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_namespace: Option<String>,
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application_arn: Option<String>,
    #[serde(rename = "RedshiftIdcApplicationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application_name: Option<String>,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<ServiceIntegrationList>,
    #[serde(rename = "SsoTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_tag_keys: Option<TagKeyList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizedTokenIssuerList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<AuthorizedTokenIssuer>,
}
impl From<Vec<AuthorizedTokenIssuer>> for AuthorizedTokenIssuerList {
    fn from(v: Vec<AuthorizedTokenIssuer>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AuthorizedTokenIssuer> for AuthorizedTokenIssuerList {
    fn from_iter<I: IntoIterator<Item = AuthorizedTokenIssuer>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AuthorizedTokenIssuer>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAuthorizedTokenIssuerList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AuthorizedTokenIssuer>,
}

impl From<Vec<AuthorizedTokenIssuer>> for XmlAuthorizedTokenIssuerList {
    fn from(v: Vec<AuthorizedTokenIssuer>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AuthorizedTokenIssuer> for XmlAuthorizedTokenIssuerList {
    fn from_iter<I: IntoIterator<Item = AuthorizedTokenIssuer>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizedTokenIssuer")]
pub struct AuthorizedTokenIssuer {
    #[serde(rename = "AuthorizedAudiencesList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_audiences_list: Option<AuthorizedAudienceList>,
    #[serde(rename = "TrustedTokenIssuerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusted_token_issuer_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuthorizedAudienceList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AuthorizedAudienceList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AuthorizedAudienceList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ServiceIntegrationList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<ServiceIntegrationsUnion>,
}
impl From<Vec<ServiceIntegrationsUnion>> for ServiceIntegrationList {
    fn from(v: Vec<ServiceIntegrationsUnion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ServiceIntegrationsUnion> for ServiceIntegrationList {
    fn from_iter<I: IntoIterator<Item = ServiceIntegrationsUnion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ServiceIntegrationsUnion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlServiceIntegrationsUnionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ServiceIntegrationsUnion>,
}

impl From<Vec<ServiceIntegrationsUnion>> for XmlServiceIntegrationsUnionList {
    fn from(v: Vec<ServiceIntegrationsUnion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ServiceIntegrationsUnion> for XmlServiceIntegrationsUnionList {
    fn from_iter<I: IntoIterator<Item = ServiceIntegrationsUnion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServiceIntegrationsUnion")]
pub struct ServiceIntegrationsUnion {
    #[serde(rename = "LakeFormation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation: Option<LakeFormationServiceIntegrations>,
    #[serde(rename = "Redshift")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift: Option<RedshiftServiceIntegrations>,
    #[serde(rename = "S3AccessGrants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_access_grants: Option<S3AccessGrantsServiceIntegrations>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct LakeFormationServiceIntegrations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<LakeFormationScopeUnion>,
}
impl From<Vec<LakeFormationScopeUnion>> for LakeFormationServiceIntegrations {
    fn from(v: Vec<LakeFormationScopeUnion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<LakeFormationScopeUnion> for LakeFormationServiceIntegrations {
    fn from_iter<I: IntoIterator<Item = LakeFormationScopeUnion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<LakeFormationScopeUnion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlLakeFormationScopeUnionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<LakeFormationScopeUnion>,
}

impl From<Vec<LakeFormationScopeUnion>> for XmlLakeFormationScopeUnionList {
    fn from(v: Vec<LakeFormationScopeUnion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<LakeFormationScopeUnion> for XmlLakeFormationScopeUnionList {
    fn from_iter<I: IntoIterator<Item = LakeFormationScopeUnion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LakeFormationScopeUnion")]
pub struct LakeFormationScopeUnion {
    #[serde(rename = "LakeFormationQuery")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lake_formation_query: Option<LakeFormationQuery>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "LakeFormationQuery")]
pub struct LakeFormationQuery {
    #[serde(rename = "Authorization")]
    #[serde(default)]
    pub authorization: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RedshiftServiceIntegrations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<RedshiftScopeUnion>,
}
impl From<Vec<RedshiftScopeUnion>> for RedshiftServiceIntegrations {
    fn from(v: Vec<RedshiftScopeUnion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RedshiftScopeUnion> for RedshiftServiceIntegrations {
    fn from_iter<I: IntoIterator<Item = RedshiftScopeUnion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RedshiftScopeUnion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRedshiftScopeUnionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RedshiftScopeUnion>,
}

impl From<Vec<RedshiftScopeUnion>> for XmlRedshiftScopeUnionList {
    fn from(v: Vec<RedshiftScopeUnion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RedshiftScopeUnion> for XmlRedshiftScopeUnionList {
    fn from_iter<I: IntoIterator<Item = RedshiftScopeUnion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RedshiftScopeUnion")]
pub struct RedshiftScopeUnion {
    #[serde(rename = "Connect")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect: Option<Connect>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Connect")]
pub struct Connect {
    #[serde(rename = "Authorization")]
    #[serde(default)]
    pub authorization: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct S3AccessGrantsServiceIntegrations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<S3AccessGrantsScopeUnion>,
}
impl From<Vec<S3AccessGrantsScopeUnion>> for S3AccessGrantsServiceIntegrations {
    fn from(v: Vec<S3AccessGrantsScopeUnion>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<S3AccessGrantsScopeUnion> for S3AccessGrantsServiceIntegrations {
    fn from_iter<I: IntoIterator<Item = S3AccessGrantsScopeUnion>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<S3AccessGrantsScopeUnion>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlS3AccessGrantsScopeUnionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<S3AccessGrantsScopeUnion>,
}

impl From<Vec<S3AccessGrantsScopeUnion>> for XmlS3AccessGrantsScopeUnionList {
    fn from(v: Vec<S3AccessGrantsScopeUnion>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<S3AccessGrantsScopeUnion> for XmlS3AccessGrantsScopeUnionList {
    fn from_iter<I: IntoIterator<Item = S3AccessGrantsScopeUnion>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "S3AccessGrantsScopeUnion")]
pub struct S3AccessGrantsScopeUnion {
    #[serde(rename = "ReadWriteAccess")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_write_access: Option<ReadWriteAccess>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReadWriteAccess")]
pub struct ReadWriteAccess {
    #[serde(rename = "Authorization")]
    #[serde(default)]
    pub authorization: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScheduledActionsResult")]
pub struct ScheduledActionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ScheduledActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_actions: Option<ScheduledActionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledActionList {
    #[serde(
        rename = "ScheduledAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ScheduledAction>,
}
impl From<Vec<ScheduledAction>> for ScheduledActionList {
    fn from(v: Vec<ScheduledAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScheduledAction> for ScheduledActionList {
    fn from_iter<I: IntoIterator<Item = ScheduledAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScheduledAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScheduledActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScheduledAction>,
}

impl From<Vec<ScheduledAction>> for XmlScheduledActionList {
    fn from(v: Vec<ScheduledAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScheduledAction> for XmlScheduledActionList {
    fn from_iter<I: IntoIterator<Item = ScheduledAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyScheduledActionResult")]
pub struct ScheduledAction {
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "NextInvocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invocations: Option<ScheduledActionTimeList>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduledActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_description: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "State")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "TargetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_action: Option<ScheduledActionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledActionTimeList {
    #[serde(
        rename = "ScheduledActionTime",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ScheduledActionTimeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ScheduledActionTimeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScheduledActionType")]
pub struct ScheduledActionType {
    #[serde(rename = "PauseCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pause_cluster: Option<PauseClusterMessage>,
    #[serde(rename = "ResizeCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_cluster: Option<ResizeClusterMessage>,
    #[serde(rename = "ResumeCluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_cluster: Option<ResumeClusterMessage>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PauseClusterMessage")]
pub struct PauseClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResumeClusterMessage")]
pub struct ResumeClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteScheduledActionMessage")]
pub struct DeleteScheduledActionMessage {
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventSubscriptionsMessage")]
pub struct DescribeEventSubscriptionsMessage {
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
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyScheduledActionMessage")]
pub struct ModifyScheduledActionMessage {
    #[serde(rename = "Enable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role: Option<String>,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,
    #[serde(rename = "ScheduledActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_description: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TargetAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_action: Option<ScheduledActionType>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterNamespaceInputMessage")]
pub struct RegisterNamespaceInputMessage {
    #[serde(rename = "ConsumerIdentifiers")]
    #[serde(default)]
    pub consumer_identifiers: ConsumerIdentifierList,
    #[serde(rename = "NamespaceIdentifier")]
    #[serde(default)]
    pub namespace_identifier: NamespaceIdentifierUnion,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConsumerIdentifierList {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ConsumerIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ConsumerIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NamespaceIdentifierUnion")]
pub struct NamespaceIdentifierUnion {
    #[serde(rename = "ProvisionedIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provisioned_identifier: Option<ProvisionedIdentifier>,
    #[serde(rename = "ServerlessIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serverless_identifier: Option<ServerlessIdentifier>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ProvisionedIdentifier")]
pub struct ProvisionedIdentifier {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ServerlessIdentifier")]
pub struct ServerlessIdentifier {
    #[serde(rename = "NamespaceIdentifier")]
    #[serde(default)]
    pub namespace_identifier: String,
    #[serde(rename = "WorkgroupIdentifier")]
    #[serde(default)]
    pub workgroup_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverPrimaryComputeResult")]
pub struct FailoverPrimaryComputeResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTableRestoreStatusResult")]
pub struct TableRestoreStatusMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "TableRestoreStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_restore_status_details: Option<TableRestoreStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TableRestoreStatusList {
    #[serde(
        rename = "TableRestoreStatus",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<TableRestoreStatus>,
}
impl From<Vec<TableRestoreStatus>> for TableRestoreStatusList {
    fn from(v: Vec<TableRestoreStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<TableRestoreStatus> for TableRestoreStatusList {
    fn from_iter<I: IntoIterator<Item = TableRestoreStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<TableRestoreStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlTableRestoreStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<TableRestoreStatus>,
}

impl From<Vec<TableRestoreStatus>> for XmlTableRestoreStatusList {
    fn from(v: Vec<TableRestoreStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<TableRestoreStatus> for XmlTableRestoreStatusList {
    fn from_iter<I: IntoIterator<Item = TableRestoreStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "TableRestoreStatus")]
pub struct TableRestoreStatus {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "NewTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_table_name: Option<String>,
    #[serde(rename = "ProgressInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_mega_bytes: Option<i64>,
    #[serde(rename = "RequestTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_time: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
    #[serde(rename = "SourceDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_database_name: Option<String>,
    #[serde(rename = "SourceSchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_schema_name: Option<String>,
    #[serde(rename = "SourceTableName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_table_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TableRestoreRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_restore_request_id: Option<String>,
    #[serde(rename = "TargetDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[serde(rename = "TargetSchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_schema_name: Option<String>,
    #[serde(rename = "TotalDataInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_data_in_mega_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterIamRolesResult")]
pub struct ModifyClusterIamRolesResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAuthenticationProfilesMessage")]
pub struct DescribeAuthenticationProfilesMessage {
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableClusterOptionsResult")]
pub struct OrderableClusterOptionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "OrderableClusterOptions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderable_cluster_options: Option<OrderableClusterOptionsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OrderableClusterOptionsList {
    #[serde(
        rename = "OrderableClusterOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<OrderableClusterOption>,
}
impl From<Vec<OrderableClusterOption>> for OrderableClusterOptionsList {
    fn from(v: Vec<OrderableClusterOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<OrderableClusterOption> for OrderableClusterOptionsList {
    fn from_iter<I: IntoIterator<Item = OrderableClusterOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<OrderableClusterOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlOrderableClusterOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<OrderableClusterOption>,
}

impl From<Vec<OrderableClusterOption>> for XmlOrderableClusterOptionList {
    fn from(v: Vec<OrderableClusterOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<OrderableClusterOption> for XmlOrderableClusterOptionList {
    fn from_iter<I: IntoIterator<Item = OrderableClusterOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "OrderableClusterOption")]
pub struct OrderableClusterOption {
    #[serde(rename = "AvailabilityZones")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zones: Option<AvailabilityZoneList>,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
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
#[serde(rename = "AvailabilityZone")]
pub struct AvailabilityZone {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "SupportedPlatforms")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_platforms: Option<SupportedPlatformsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedPlatformsList {
    #[serde(
        rename = "SupportedPlatform",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SupportedPlatform>,
}
impl From<Vec<SupportedPlatform>> for SupportedPlatformsList {
    fn from(v: Vec<SupportedPlatform>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SupportedPlatform> for SupportedPlatformsList {
    fn from_iter<I: IntoIterator<Item = SupportedPlatform>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SupportedPlatform>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSupportedPlatformList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SupportedPlatform>,
}

impl From<Vec<SupportedPlatform>> for XmlSupportedPlatformList {
    fn from(v: Vec<SupportedPlatform>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SupportedPlatform> for XmlSupportedPlatformList {
    fn from_iter<I: IntoIterator<Item = SupportedPlatform>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SupportedPlatform")]
pub struct SupportedPlatform {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyCustomDomainAssociationResult")]
pub struct ModifyCustomDomainAssociationResult {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CustomDomainCertExpiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_cert_expiry_time: Option<String>,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_arn: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PauseClusterResult")]
pub struct PauseClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterResult")]
pub struct CreateClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateTagsMessage")]
pub struct CreateTagsMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    pub tags: TagList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRecommendationsMessage")]
pub struct ListRecommendationsMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NamespaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteEndpointAccessMessage")]
pub struct DeleteEndpointAccessMessage {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreFromClusterSnapshotResult")]
pub struct RestoreFromClusterSnapshotResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifySnapshotCopyRetentionPeriodResult")]
pub struct ModifySnapshotCopyRetentionPeriodResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterNamespaceInputMessage")]
pub struct DeregisterNamespaceInputMessage {
    #[serde(rename = "ConsumerIdentifiers")]
    #[serde(default)]
    pub consumer_identifiers: ConsumerIdentifierList,
    #[serde(rename = "NamespaceIdentifier")]
    #[serde(default)]
    pub namespace_identifier: NamespaceIdentifierUnion,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSubnetGroupsMessage")]
pub struct DescribeClusterSubnetGroupsMessage {
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodeExchangeStatusInputMessage")]
pub struct DescribeReservedNodeExchangeStatusInputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReservedNodeExchangeRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_exchange_request_id: Option<String>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterSnapshotMessage")]
pub struct CreateClusterSnapshotMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyClusterSnapshotResult")]
pub struct CopyClusterSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutResourcePolicyResult")]
pub struct PutResourcePolicyResult {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResourcePolicy")]
pub struct ResourcePolicy {
    #[serde(rename = "Policy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyRedshiftIdcApplicationMessage")]
pub struct ModifyRedshiftIdcApplicationMessage {
    #[serde(rename = "AuthorizedTokenIssuerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_token_issuer_list: Option<AuthorizedTokenIssuerList>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(rename = "IdcDisplayName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idc_display_name: Option<String>,
    #[serde(rename = "IdentityNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_namespace: Option<String>,
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    pub redshift_idc_application_arn: String,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<ServiceIntegrationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreTableFromClusterSnapshotResult")]
pub struct RestoreTableFromClusterSnapshotResult {
    #[serde(rename = "TableRestoreStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_restore_status: Option<TableRestoreStatus>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSnapshotsResult")]
pub struct SnapshotMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Snapshots")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshots: Option<SnapshotList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotList {
    #[serde(rename = "Snapshot", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Snapshot>,
}
impl From<Vec<Snapshot>> for SnapshotList {
    fn from(v: Vec<Snapshot>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Snapshot> for SnapshotList {
    fn from_iter<I: IntoIterator<Item = Snapshot>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Snapshot>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Snapshot>,
}

impl From<Vec<Snapshot>> for XmlSnapshotList {
    fn from(v: Vec<Snapshot>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Snapshot> for XmlSnapshotList {
    fn from_iter<I: IntoIterator<Item = Snapshot>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAuthenticationProfileResult")]
pub struct DeleteAuthenticationProfileResult {
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesForConsumerMessage")]
pub struct DescribeDataSharesForConsumerMessage {
    #[serde(rename = "ConsumerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyAuthenticationProfileResult")]
pub struct ModifyAuthenticationProfileResult {
    #[serde(rename = "AuthenticationProfileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_content: Option<String>,
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeClusterSecurityGroupIngressResult")]
pub struct AuthorizeClusterSecurityGroupIngressResult {
    #[serde(rename = "ClusterSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterSecurityGroup")]
pub struct ClusterSecurityGroup {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "EC2SecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_groups: Option<EC2SecurityGroupList>,
    #[serde(rename = "IPRanges")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i_p_ranges: Option<IPRangeList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EC2SecurityGroupList {
    #[serde(
        rename = "EC2SecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<EC2SecurityGroup>,
}
impl From<Vec<EC2SecurityGroup>> for EC2SecurityGroupList {
    fn from(v: Vec<EC2SecurityGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EC2SecurityGroup> for EC2SecurityGroupList {
    fn from_iter<I: IntoIterator<Item = EC2SecurityGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EC2SecurityGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEC2SecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EC2SecurityGroup>,
}

impl From<Vec<EC2SecurityGroup>> for XmlEC2SecurityGroupList {
    fn from(v: Vec<EC2SecurityGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EC2SecurityGroup> for XmlEC2SecurityGroupList {
    fn from_iter<I: IntoIterator<Item = EC2SecurityGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EC2SecurityGroup")]
pub struct EC2SecurityGroup {
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IPRangeList {
    #[serde(rename = "IPRange", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<IPRange>,
}
impl From<Vec<IPRange>> for IPRangeList {
    fn from(v: Vec<IPRange>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<IPRange> for IPRangeList {
    fn from_iter<I: IntoIterator<Item = IPRange>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<IPRange>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlIPRangeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<IPRange>,
}

impl From<Vec<IPRange>> for XmlIPRangeList {
    fn from(v: Vec<IPRange>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<IPRange> for XmlIPRangeList {
    fn from_iter<I: IntoIterator<Item = IPRange>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "IPRange")]
pub struct IPRange {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterSecurityGroupMessage")]
pub struct DeleteClusterSecurityGroupMessage {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    pub cluster_security_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterIamRolesMessage")]
pub struct ModifyClusterIamRolesMessage {
    #[serde(rename = "AddIamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_iam_roles: Option<IamRoleArnList>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<String>,
    #[serde(rename = "RemoveIamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_iam_roles: Option<IamRoleArnList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct IamRoleArnList {
    #[serde(rename = "IamRoleArn", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for IamRoleArnList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for IamRoleArnList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyIntegrationMessage")]
pub struct ModifyIntegrationMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    pub integration_arn: String,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedNodeOfferingResult")]
pub struct PurchaseReservedNodeOfferingResult {
    #[serde(rename = "ReservedNode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node: Option<ReservedNode>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeClusterSecurityGroupIngressMessage")]
pub struct RevokeClusterSecurityGroupIngressMessage {
    #[serde(rename = "CIDRIP")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_i_d_r_i_p: Option<String>,
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    pub cluster_security_group_name: String,
    #[serde(rename = "EC2SecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_name: Option<String>,
    #[serde(rename = "EC2SecurityGroupOwnerId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_c2_security_group_owner_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterDbRevisionsMessage")]
pub struct DescribeClusterDbRevisionsMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
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
#[serde(rename = "CreateClusterSecurityGroupResult")]
pub struct CreateClusterSecurityGroupResult {
    #[serde(rename = "ClusterSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterMessage")]
pub struct ModifyClusterMessage {
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneRelocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_relocation: Option<bool>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "ClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_name: Option<String>,
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<ClusterSecurityGroupNameList>,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "ElasticIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "ExtraComputeForAutomaticOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_compute_for_automatic_optimization: Option<bool>,
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManageMasterPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_password: Option<bool>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterPasswordSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NewClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_cluster_identifier: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSecurityGroupNameList {
    #[serde(
        rename = "ClusterSecurityGroupName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ClusterSecurityGroupNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ClusterSecurityGroupNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
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
#[serde(rename = "CreateHsmConfigurationResult")]
pub struct CreateHsmConfigurationResult {
    #[serde(rename = "HsmConfiguration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration: Option<HsmConfiguration>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HsmConfiguration")]
pub struct HsmConfiguration {
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "HsmIpAddress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_ip_address: Option<String>,
    #[serde(rename = "HsmPartitionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_partition_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSnapshotCopyGrantResult")]
pub struct CreateSnapshotCopyGrantResult {
    #[serde(rename = "SnapshotCopyGrant")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant: Option<SnapshotCopyGrant>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SnapshotCopyGrant")]
pub struct SnapshotCopyGrant {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeScheduledActionsMessage")]
pub struct DescribeScheduledActionsMessage {
    #[serde(rename = "Active")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ScheduledActionFilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_name: Option<String>,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TargetActionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_action_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledActionFilterList {
    #[serde(
        rename = "ScheduledActionFilter",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ScheduledActionFilter>,
}
impl From<Vec<ScheduledActionFilter>> for ScheduledActionFilterList {
    fn from(v: Vec<ScheduledActionFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ScheduledActionFilter> for ScheduledActionFilterList {
    fn from_iter<I: IntoIterator<Item = ScheduledActionFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ScheduledActionFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlScheduledActionFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ScheduledActionFilter>,
}

impl From<Vec<ScheduledActionFilter>> for XmlScheduledActionFilterList {
    fn from(v: Vec<ScheduledActionFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ScheduledActionFilter> for XmlScheduledActionFilterList {
    fn from_iter<I: IntoIterator<Item = ScheduledActionFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ScheduledActionFilter")]
pub struct ScheduledActionFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: ValueStringList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ValueStringList {
    #[serde(rename = "item", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ValueStringList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ValueStringList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisassociateDataShareConsumerMessage")]
pub struct DisassociateDataShareConsumerMessage {
    #[serde(rename = "ConsumerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_arn: Option<String>,
    #[serde(rename = "ConsumerRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_region: Option<String>,
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    pub data_share_arn: String,
    #[serde(rename = "DisassociateEntireAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_entire_account: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateIntegrationMessage")]
pub struct CreateIntegrationMessage {
    #[serde(rename = "AdditionalEncryptionContext")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_encryption_context: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "IntegrationName")]
    #[serde(default)]
    pub integration_name: String,
    #[serde(rename = "KMSKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_m_s_key_id: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    pub source_arn: String,
    #[serde(rename = "TagList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_list: Option<TagList>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    pub target_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RotateEncryptionKeyResult")]
pub struct RotateEncryptionKeyResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetReservedNodeExchangeConfigurationOptionsInputMessage")]
pub struct GetReservedNodeExchangeConfigurationOptionsInputMessage {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    pub action_type: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdatePartnerStatusResult")]
pub struct PartnerIntegrationOutputMessage {
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "PartnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAuthenticationProfileResult")]
pub struct CreateAuthenticationProfileResult {
    #[serde(rename = "AuthenticationProfileContent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_content: Option<String>,
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_profile_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterSubnetGroupMessage")]
pub struct CreateClusterSubnetGroupMessage {
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    pub cluster_subnet_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
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
#[serde(rename = "DescribeTagsMessage")]
pub struct DescribeTagsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(rename = "ResourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeRedshiftIdcApplicationsMessage")]
pub struct DescribeRedshiftIdcApplicationsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEndpointAccessResult")]
pub struct EndpointAccess {
    #[serde(rename = "Address")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "EndpointCreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_create_time: Option<String>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "EndpointStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_status: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_name: Option<String>,
    #[serde(rename = "VpcEndpoint")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_endpoint: Option<VpcEndpoint>,
    #[serde(rename = "VpcSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_groups: Option<VpcSecurityGroupMembershipList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchModifyClusterSnapshotsResult")]
pub struct BatchModifyClusterSnapshotsOutputMessage {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchSnapshotOperationErrors>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<SnapshotIdentifierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSnapshotOperationErrors {
    #[serde(
        rename = "SnapshotErrorMessage",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SnapshotErrorMessage>,
}
impl From<Vec<SnapshotErrorMessage>> for BatchSnapshotOperationErrors {
    fn from(v: Vec<SnapshotErrorMessage>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SnapshotErrorMessage> for BatchSnapshotOperationErrors {
    fn from_iter<I: IntoIterator<Item = SnapshotErrorMessage>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SnapshotErrorMessage>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotErrorMessageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SnapshotErrorMessage>,
}

impl From<Vec<SnapshotErrorMessage>> for XmlSnapshotErrorMessageList {
    fn from(v: Vec<SnapshotErrorMessage>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SnapshotErrorMessage> for XmlSnapshotErrorMessageList {
    fn from_iter<I: IntoIterator<Item = SnapshotErrorMessage>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SnapshotErrorMessage")]
pub struct SnapshotErrorMessage {
    #[serde(rename = "FailureCode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(rename = "FailureReason")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotIdentifierList {
    #[serde(rename = "String", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for SnapshotIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for SnapshotIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterSnapshotMessage")]
pub struct DeleteClusterSnapshotMessage {
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountAttributesMessage")]
pub struct DescribeAccountAttributesMessage {
    #[serde(rename = "AttributeNames")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_names: Option<AttributeNameList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeNameList {
    #[serde(
        rename = "AttributeName",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for AttributeNameList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for AttributeNameList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEndpointAccessResult")]
pub struct EndpointAccessList {
    #[serde(rename = "EndpointAccessList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_access_list: Option<EndpointAccesses>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointAccesses {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EndpointAccess>,
}
impl From<Vec<EndpointAccess>> for EndpointAccesses {
    fn from(v: Vec<EndpointAccess>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EndpointAccess> for EndpointAccesses {
    fn from_iter<I: IntoIterator<Item = EndpointAccess>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EndpointAccess>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEndpointAccessList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EndpointAccess>,
}

impl From<Vec<EndpointAccess>> for XmlEndpointAccessList {
    fn from(v: Vec<EndpointAccess>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EndpointAccess> for XmlEndpointAccessList {
    fn from_iter<I: IntoIterator<Item = EndpointAccess>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLakehouseConfigurationResult")]
pub struct LakehouseConfiguration {
    #[serde(rename = "CatalogArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_arn: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "LakehouseIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_idc_application_arn: Option<String>,
    #[serde(rename = "LakehouseRegistrationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_registration_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterSnapshotMessage")]
pub struct ModifyClusterSnapshotMessage {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreTableFromClusterSnapshotMessage")]
pub struct RestoreTableFromClusterSnapshotMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "EnableCaseSensitiveIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_case_sensitive_identifier: Option<bool>,
    #[serde(rename = "NewTableName")]
    #[serde(default)]
    pub new_table_name: String,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    pub snapshot_identifier: String,
    #[serde(rename = "SourceDatabaseName")]
    #[serde(default)]
    pub source_database_name: String,
    #[serde(rename = "SourceSchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_schema_name: Option<String>,
    #[serde(rename = "SourceTableName")]
    #[serde(default)]
    pub source_table_name: String,
    #[serde(rename = "TargetDatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_database_name: Option<String>,
    #[serde(rename = "TargetSchemaName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_schema_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotCopyGrantsMessage")]
pub struct DescribeSnapshotCopyGrantsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRedshiftIdcApplicationMessage")]
pub struct CreateRedshiftIdcApplicationMessage {
    #[serde(rename = "ApplicationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "AuthorizedTokenIssuerList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_token_issuer_list: Option<AuthorizedTokenIssuerList>,
    #[serde(rename = "IamRoleArn")]
    #[serde(default)]
    pub iam_role_arn: String,
    #[serde(rename = "IdcDisplayName")]
    #[serde(default)]
    pub idc_display_name: String,
    #[serde(rename = "IdcInstanceArn")]
    #[serde(default)]
    pub idc_instance_arn: String,
    #[serde(rename = "IdentityNamespace")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_namespace: Option<String>,
    #[serde(rename = "RedshiftIdcApplicationName")]
    #[serde(default)]
    pub redshift_idc_application_name: String,
    #[serde(rename = "ServiceIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_integrations: Option<ServiceIntegrationList>,
    #[serde(rename = "SsoTagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_tag_keys: Option<TagKeyList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteUsageLimitMessage")]
pub struct DeleteUsageLimitMessage {
    #[serde(rename = "UsageLimitId")]
    #[serde(default)]
    pub usage_limit_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEndpointAuthorizationResult")]
pub struct EndpointAuthorizationList {
    #[serde(rename = "EndpointAuthorizationList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_authorization_list: Option<EndpointAuthorizations>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EndpointAuthorizations {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<EndpointAuthorization>,
}
impl From<Vec<EndpointAuthorization>> for EndpointAuthorizations {
    fn from(v: Vec<EndpointAuthorization>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<EndpointAuthorization> for EndpointAuthorizations {
    fn from_iter<I: IntoIterator<Item = EndpointAuthorization>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<EndpointAuthorization>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlEndpointAuthorizationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<EndpointAuthorization>,
}

impl From<Vec<EndpointAuthorization>> for XmlEndpointAuthorizationList {
    fn from(v: Vec<EndpointAuthorization>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<EndpointAuthorization> for XmlEndpointAuthorizationList {
    fn from_iter<I: IntoIterator<Item = EndpointAuthorization>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeEndpointAccessResult")]
pub struct EndpointAuthorization {
    #[serde(rename = "AllowedAllVPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_all_v_p_cs: Option<bool>,
    #[serde(rename = "AllowedVPCs")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_v_p_cs: Option<VpcIdentifierList>,
    #[serde(rename = "AuthorizeTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorize_time: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ClusterStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_status: Option<String>,
    #[serde(rename = "EndpointCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_count: Option<i32>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<String>,
    #[serde(rename = "Grantor")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantor: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ResetClusterParameterGroupResult")]
pub struct ClusterParameterGroupNameMessage {
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "ParameterGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ListRecommendationsResult")]
pub struct ListRecommendationsResult {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "Recommendations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<RecommendationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendationList {
    #[serde(
        rename = "Recommendation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<Recommendation>,
}
impl From<Vec<Recommendation>> for RecommendationList {
    fn from(v: Vec<Recommendation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Recommendation> for RecommendationList {
    fn from_iter<I: IntoIterator<Item = Recommendation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Recommendation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecommendationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Recommendation>,
}

impl From<Vec<Recommendation>> for XmlRecommendationList {
    fn from(v: Vec<Recommendation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Recommendation> for XmlRecommendationList {
    fn from_iter<I: IntoIterator<Item = Recommendation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "Recommendation")]
pub struct Recommendation {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "ImpactRanking")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact_ranking: Option<String>,
    #[serde(rename = "NamespaceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace_arn: Option<String>,
    #[serde(rename = "Observation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observation: Option<String>,
    #[serde(rename = "RecommendationText")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_text: Option<String>,
    #[serde(rename = "RecommendationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    #[serde(rename = "RecommendedActions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<RecommendedActionList>,
    #[serde(rename = "ReferenceLinks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_links: Option<ReferenceLinkList>,
    #[serde(rename = "Title")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RecommendedActionList {
    #[serde(
        rename = "RecommendedAction",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RecommendedAction>,
}
impl From<Vec<RecommendedAction>> for RecommendedActionList {
    fn from(v: Vec<RecommendedAction>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RecommendedAction> for RecommendedActionList {
    fn from_iter<I: IntoIterator<Item = RecommendedAction>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RecommendedAction>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRecommendedActionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RecommendedAction>,
}

impl From<Vec<RecommendedAction>> for XmlRecommendedActionList {
    fn from(v: Vec<RecommendedAction>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RecommendedAction> for XmlRecommendedActionList {
    fn from_iter<I: IntoIterator<Item = RecommendedAction>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RecommendedAction")]
pub struct RecommendedAction {
    #[serde(rename = "Command")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(rename = "Database")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "Type")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReferenceLinkList {
    #[serde(
        rename = "ReferenceLink",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReferenceLink>,
}
impl From<Vec<ReferenceLink>> for ReferenceLinkList {
    fn from(v: Vec<ReferenceLink>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReferenceLink> for ReferenceLinkList {
    fn from_iter<I: IntoIterator<Item = ReferenceLink>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReferenceLink>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReferenceLinkList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReferenceLink>,
}

impl From<Vec<ReferenceLink>> for XmlReferenceLinkList {
    fn from(v: Vec<ReferenceLink>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReferenceLink> for XmlReferenceLinkList {
    fn from_iter<I: IntoIterator<Item = ReferenceLink>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ReferenceLink")]
pub struct ReferenceLink {
    #[serde(rename = "Link")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    #[serde(rename = "Text")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHsmClientCertificateMessage")]
pub struct CreateHsmClientCertificateMessage {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    pub hsm_client_certificate_identifier: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSnapshotCopyGrantMessage")]
pub struct CreateSnapshotCopyGrantMessage {
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    pub snapshot_copy_grant_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterSubnetGroupResult")]
pub struct CreateClusterSubnetGroupResult {
    #[serde(rename = "ClusterSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group: Option<ClusterSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterSubnetGroup")]
pub struct ClusterSubnetGroup {
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SubnetGroupStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_group_status: Option<String>,
    #[serde(rename = "Subnets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnets: Option<SubnetList>,
    #[serde(rename = "SupportedClusterIpAddressTypes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_cluster_ip_address_types: Option<ValueStringList>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
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
#[serde(rename = "DeleteSnapshotCopyGrantMessage")]
pub struct DeleteSnapshotCopyGrantMessage {
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    pub snapshot_copy_grant_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeUsageLimitsMessage")]
pub struct DescribeUsageLimitsMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "FeatureType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature_type: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
    #[serde(rename = "UsageLimitId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_limit_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableSnapshotCopyResult")]
pub struct DisableSnapshotCopyResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateAuthenticationProfileMessage")]
pub struct CreateAuthenticationProfileMessage {
    #[serde(rename = "AuthenticationProfileContent")]
    #[serde(default)]
    pub authentication_profile_content: String,
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    pub authentication_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeIntegrationsMessage")]
pub struct DescribeIntegrationsMessage {
    #[serde(rename = "Filters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<DescribeIntegrationsFilterList>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
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
pub struct DescribeIntegrationsFilterList {
    #[serde(
        rename = "DescribeIntegrationsFilter",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DescribeIntegrationsFilter>,
}
impl From<Vec<DescribeIntegrationsFilter>> for DescribeIntegrationsFilterList {
    fn from(v: Vec<DescribeIntegrationsFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DescribeIntegrationsFilter> for DescribeIntegrationsFilterList {
    fn from_iter<I: IntoIterator<Item = DescribeIntegrationsFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DescribeIntegrationsFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDescribeIntegrationsFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DescribeIntegrationsFilter>,
}

impl From<Vec<DescribeIntegrationsFilter>> for XmlDescribeIntegrationsFilterList {
    fn from(v: Vec<DescribeIntegrationsFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DescribeIntegrationsFilter> for XmlDescribeIntegrationsFilterList {
    fn from_iter<I: IntoIterator<Item = DescribeIntegrationsFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeIntegrationsFilter")]
pub struct DescribeIntegrationsFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "Values")]
    #[serde(default)]
    pub values: DescribeIntegrationsFilterValueList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DescribeIntegrationsFilterValueList {
    #[serde(rename = "Value", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DescribeIntegrationsFilterValueList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DescribeIntegrationsFilterValueList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifySnapshotCopyRetentionPeriodMessage")]
pub struct ModifySnapshotCopyRetentionPeriodMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "Manual")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual: Option<bool>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    pub retention_period: i32,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeSnapshotAccessResult")]
pub struct RevokeSnapshotAccessResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterMaintenanceMessage")]
pub struct ModifyClusterMaintenanceMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DeferMaintenance")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance: Option<bool>,
    #[serde(rename = "DeferMaintenanceDuration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_duration: Option<i32>,
    #[serde(rename = "DeferMaintenanceEndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_end_time: Option<String>,
    #[serde(rename = "DeferMaintenanceIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_identifier: Option<String>,
    #[serde(rename = "DeferMaintenanceStartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub defer_maintenance_start_time: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteResourcePolicyMessage")]
pub struct DeleteResourcePolicyMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyResult")]
pub struct GetResourcePolicyResult {
    #[serde(rename = "ResourcePolicy")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_policy: Option<ResourcePolicy>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootClusterResult")]
pub struct RebootClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "FailoverPrimaryComputeInputMessage")]
pub struct FailoverPrimaryComputeInputMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityCenterAuthTokenRequest")]
pub struct GetIdentityCenterAuthTokenRequest {
    #[serde(rename = "ClusterIds")]
    #[serde(default)]
    pub cluster_ids: ClusterIdentifierList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterIdentifierList {
    #[serde(
        rename = "ClusterIdentifier",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ClusterIdentifierList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ClusterIdentifierList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInboundIntegrationsMessage")]
pub struct DescribeInboundIntegrationsMessage {
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeHsmClientCertificatesMessage")]
pub struct DescribeHsmClientCertificatesMessage {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateRedshiftIdcApplicationResult")]
pub struct CreateRedshiftIdcApplicationResult {
    #[serde(rename = "RedshiftIdcApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application: Option<RedshiftIdcApplication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetResourcePolicyMessage")]
pub struct GetResourcePolicyMessage {
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterMessage")]
pub struct DeleteClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "FinalClusterSnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_cluster_snapshot_identifier: Option<String>,
    #[serde(rename = "FinalClusterSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_cluster_snapshot_retention_period: Option<i32>,
    #[serde(rename = "SkipFinalClusterSnapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_final_cluster_snapshot: Option<bool>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSubnetGroupsResult")]
pub struct ClusterSubnetGroupMessage {
    #[serde(rename = "ClusterSubnetGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_groups: Option<ClusterSubnetGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSubnetGroups {
    #[serde(
        rename = "ClusterSubnetGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterSubnetGroup>,
}
impl From<Vec<ClusterSubnetGroup>> for ClusterSubnetGroups {
    fn from(v: Vec<ClusterSubnetGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterSubnetGroup> for ClusterSubnetGroups {
    fn from_iter<I: IntoIterator<Item = ClusterSubnetGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterSubnetGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterSubnetGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterSubnetGroup>,
}

impl From<Vec<ClusterSubnetGroup>> for XmlClusterSubnetGroupList {
    fn from(v: Vec<ClusterSubnetGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterSubnetGroup> for XmlClusterSubnetGroupList {
    fn from_iter<I: IntoIterator<Item = ClusterSubnetGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeregisterNamespaceResult")]
pub struct DeregisterNamespaceOutputMessage {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeauthorizeDataShareMessage")]
pub struct DeauthorizeDataShareMessage {
    #[serde(rename = "ConsumerIdentifier")]
    #[serde(default)]
    pub consumer_identifier: String,
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    pub data_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateSnapshotScheduleMessage")]
pub struct CreateSnapshotScheduleMessage {
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "NextInvocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invocations: Option<i32>,
    #[serde(rename = "ScheduleDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_definitions: Option<ScheduleDefinitionList>,
    #[serde(rename = "ScheduleDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_description: Option<String>,
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduleDefinitionList {
    #[serde(
        rename = "ScheduleDefinition",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ScheduleDefinitionList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ScheduleDefinitionList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHsmClientCertificateResult")]
pub struct CreateHsmClientCertificateResult {
    #[serde(rename = "HsmClientCertificate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate: Option<HsmClientCertificate>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "HsmClientCertificate")]
pub struct HsmClientCertificate {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmClientCertificatePublicKey")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_public_key: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSecurityGroupsMessage")]
pub struct DescribeClusterSecurityGroupsMessage {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDefaultClusterParametersResult")]
pub struct DescribeDefaultClusterParametersResult {
    #[serde(rename = "DefaultClusterParameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_cluster_parameters: Option<DefaultClusterParameters>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DefaultClusterParameters")]
pub struct DefaultClusterParameters {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_family: Option<String>,
    #[serde(rename = "Parameters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ParametersList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePartnersResult")]
pub struct DescribePartnersOutputMessage {
    #[serde(rename = "PartnerIntegrationInfoList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_integration_info_list: Option<PartnerIntegrationInfoList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PartnerIntegrationInfoList {
    #[serde(
        rename = "PartnerIntegrationInfo",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<PartnerIntegrationInfo>,
}
impl From<Vec<PartnerIntegrationInfo>> for PartnerIntegrationInfoList {
    fn from(v: Vec<PartnerIntegrationInfo>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<PartnerIntegrationInfo> for PartnerIntegrationInfoList {
    fn from_iter<I: IntoIterator<Item = PartnerIntegrationInfo>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<PartnerIntegrationInfo>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlPartnerIntegrationInfoList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<PartnerIntegrationInfo>,
}

impl From<Vec<PartnerIntegrationInfo>> for XmlPartnerIntegrationInfoList {
    fn from(v: Vec<PartnerIntegrationInfo>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<PartnerIntegrationInfo> for XmlPartnerIntegrationInfoList {
    fn from_iter<I: IntoIterator<Item = PartnerIntegrationInfo>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PartnerIntegrationInfo")]
pub struct PartnerIntegrationInfo {
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "PartnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
    #[serde(rename = "UpdatedAt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribePartnersInputMessage")]
pub struct DescribePartnersInputMessage {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_name: Option<String>,
    #[serde(rename = "PartnerName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeOrderableClusterOptionsMessage")]
pub struct DescribeOrderableClusterOptionsMessage {
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodesResult")]
pub struct ReservedNodesMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_nodes: Option<ReservedNodeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNodeList {
    #[serde(
        rename = "ReservedNode",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedNode>,
}
impl From<Vec<ReservedNode>> for ReservedNodeList {
    fn from(v: Vec<ReservedNode>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedNode> for ReservedNodeList {
    fn from_iter<I: IntoIterator<Item = ReservedNode>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedNode>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedNodeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedNode>,
}

impl From<Vec<ReservedNode>> for XmlReservedNodeList {
    fn from(v: Vec<ReservedNode>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedNode> for XmlReservedNodeList {
    fn from_iter<I: IntoIterator<Item = ReservedNode>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteTagsMessage")]
pub struct DeleteTagsMessage {
    #[serde(rename = "ResourceName")]
    #[serde(default)]
    pub resource_name: String,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    pub tag_keys: TagKeyList,
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
#[serde(rename = "DeleteRedshiftIdcApplicationMessage")]
pub struct DeleteRedshiftIdcApplicationMessage {
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    pub redshift_idc_application_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableSnapshotCopyResult")]
pub struct EnableSnapshotCopyResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotSchedulesMessage")]
pub struct DescribeSnapshotSchedulesMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_identifier: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesForConsumerResult")]
pub struct DescribeDataSharesForConsumerResult {
    #[serde(rename = "DataShares")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_shares: Option<DataShareList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableSnapshotCopyMessage")]
pub struct EnableSnapshotCopyMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DestinationRegion")]
    #[serde(default)]
    pub destination_region: String,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "RetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retention_period: Option<i32>,
    #[serde(rename = "SnapshotCopyGrantName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grant_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifySnapshotScheduleMessage")]
pub struct ModifySnapshotScheduleMessage {
    #[serde(rename = "ScheduleDefinitions")]
    #[serde(default)]
    pub schedule_definitions: ScheduleDefinitionList,
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    pub schedule_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDeleteClusterSnapshotsRequest")]
pub struct BatchDeleteClusterSnapshotsRequest {
    #[serde(rename = "Identifiers")]
    #[serde(default)]
    pub identifiers: DeleteClusterSnapshotMessageList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DeleteClusterSnapshotMessageList {
    #[serde(
        rename = "DeleteClusterSnapshotMessage",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<DeleteClusterSnapshotMessage>,
}
impl From<Vec<DeleteClusterSnapshotMessage>> for DeleteClusterSnapshotMessageList {
    fn from(v: Vec<DeleteClusterSnapshotMessage>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<DeleteClusterSnapshotMessage> for DeleteClusterSnapshotMessageList {
    fn from_iter<I: IntoIterator<Item = DeleteClusterSnapshotMessage>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<DeleteClusterSnapshotMessage>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlDeleteClusterSnapshotMessageList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<DeleteClusterSnapshotMessage>,
}

impl From<Vec<DeleteClusterSnapshotMessage>> for XmlDeleteClusterSnapshotMessageList {
    fn from(v: Vec<DeleteClusterSnapshotMessage>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<DeleteClusterSnapshotMessage> for XmlDeleteClusterSnapshotMessageList {
    fn from_iter<I: IntoIterator<Item = DeleteClusterSnapshotMessage>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RegisterNamespaceResult")]
pub struct RegisterNamespaceOutputMessage {
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CancelResizeMessage")]
pub struct CancelResizeMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteAuthenticationProfileMessage")]
pub struct DeleteAuthenticationProfileMessage {
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    pub authentication_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEndpointAuthorizationMessage")]
pub struct DescribeEndpointAuthorizationMessage {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Grantee")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grantee: Option<bool>,
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
#[serde(rename = "GetReservedNodeExchangeOfferingsInputMessage")]
pub struct GetReservedNodeExchangeOfferingsInputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    pub reserved_node_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeHsmConfigurationsMessage")]
pub struct DescribeHsmConfigurationsMessage {
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchDeleteClusterSnapshotsResult")]
pub struct BatchDeleteClusterSnapshotsResult {
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<BatchSnapshotOperationErrorList>,
    #[serde(rename = "Resources")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<SnapshotIdentifierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct BatchSnapshotOperationErrorList {
    #[serde(
        rename = "SnapshotErrorMessage",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SnapshotErrorMessage>,
}
impl From<Vec<SnapshotErrorMessage>> for BatchSnapshotOperationErrorList {
    fn from(v: Vec<SnapshotErrorMessage>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SnapshotErrorMessage> for BatchSnapshotOperationErrorList {
    fn from_iter<I: IntoIterator<Item = SnapshotErrorMessage>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDefaultClusterParametersMessage")]
pub struct DescribeDefaultClusterParametersMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    pub parameter_group_family: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterSnapshotResult")]
pub struct DeleteClusterSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeHsmClientCertificatesResult")]
pub struct HsmClientCertificateMessage {
    #[serde(rename = "HsmClientCertificates")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificates: Option<HsmClientCertificateList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HsmClientCertificateList {
    #[serde(
        rename = "HsmClientCertificate",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<HsmClientCertificate>,
}
impl From<Vec<HsmClientCertificate>> for HsmClientCertificateList {
    fn from(v: Vec<HsmClientCertificate>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HsmClientCertificate> for HsmClientCertificateList {
    fn from_iter<I: IntoIterator<Item = HsmClientCertificate>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HsmClientCertificate>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHsmClientCertificateList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HsmClientCertificate>,
}

impl From<Vec<HsmClientCertificate>> for XmlHsmClientCertificateList {
    fn from(v: Vec<HsmClientCertificate>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HsmClientCertificate> for XmlHsmClientCertificateList {
    fn from_iter<I: IntoIterator<Item = HsmClientCertificate>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DisableLoggingMessage")]
pub struct DisableLoggingMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClustersResult")]
pub struct ClustersMessage {
    #[serde(rename = "Clusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clusters: Option<ClusterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterList {
    #[serde(rename = "Cluster", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Cluster>,
}
impl From<Vec<Cluster>> for ClusterList {
    fn from(v: Vec<Cluster>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<Cluster> for ClusterList {
    fn from_iter<I: IntoIterator<Item = Cluster>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<Cluster>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<Cluster>,
}

impl From<Vec<Cluster>> for XmlClusterList {
    fn from(v: Vec<Cluster>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<Cluster> for XmlClusterList {
    fn from_iter<I: IntoIterator<Item = Cluster>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCustomDomainAssociationResult")]
pub struct CreateCustomDomainAssociationResult {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CustomDomainCertExpiryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_cert_expiry_time: Option<String>,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_certificate_arn: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeTableRestoreStatusMessage")]
pub struct DescribeTableRestoreStatusMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "TableRestoreRequestId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_restore_request_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RotateEncryptionKeyMessage")]
pub struct RotateEncryptionKeyMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterMessage")]
pub struct CreateClusterMessage {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    #[serde(rename = "AquaConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration_status: Option<String>,
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneRelocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_relocation: Option<bool>,
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "ClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_name: Option<String>,
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<ClusterSecurityGroupNameList>,
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "ClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_type: Option<String>,
    #[serde(rename = "ClusterVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "DBName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_b_name: Option<String>,
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<String>,
    #[serde(rename = "ElasticIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "ExtraComputeForAutomaticOptimization")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_compute_for_automatic_optimization: Option<bool>,
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "IamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<IamRoleArnList>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "LoadSampleData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub load_sample_data: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManageMasterPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_password: Option<bool>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterPasswordSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_kms_key_id: Option<String>,
    #[serde(rename = "MasterUserPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_user_password: Option<String>,
    #[serde(rename = "MasterUsername")]
    #[serde(default)]
    pub master_username: String,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    pub node_type: String,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application_arn: Option<String>,
    #[serde(rename = "SnapshotScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_identifier: Option<String>,
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
#[serde(rename = "ResumeClusterResult")]
pub struct ResumeClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PurchaseReservedNodeOfferingMessage")]
pub struct PurchaseReservedNodeOfferingMessage {
    #[serde(rename = "NodeCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_count: Option<i32>,
    #[serde(rename = "ReservedNodeOfferingId")]
    #[serde(default)]
    pub reserved_node_offering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateUsageLimitMessage")]
pub struct CreateUsageLimitMessage {
    #[serde(rename = "Amount")]
    #[serde(default)]
    pub amount: i64,
    #[serde(rename = "BreachAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breach_action: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "FeatureType")]
    #[serde(default)]
    pub feature_type: String,
    #[serde(rename = "LimitType")]
    #[serde(default)]
    pub limit_type: String,
    #[serde(rename = "Period")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetClusterCredentialsMessage")]
pub struct GetClusterCredentialsMessage {
    #[serde(rename = "AutoCreate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_create: Option<bool>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "DbGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_groups: Option<DbGroupList>,
    #[serde(rename = "DbName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[serde(rename = "DbUser")]
    #[serde(default)]
    pub db_user: String,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct DbGroupList {
    #[serde(rename = "DbGroup", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for DbGroupList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for DbGroupList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeDataShareMessage")]
pub struct AuthorizeDataShareMessage {
    #[serde(rename = "AllowWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_writes: Option<bool>,
    #[serde(rename = "ConsumerIdentifier")]
    #[serde(default)]
    pub consumer_identifier: String,
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    pub data_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeNodeConfigurationOptionsResult")]
pub struct NodeConfigurationOptionsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "NodeConfigurationOptionList")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_configuration_option_list: Option<NodeConfigurationOptionList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeConfigurationOptionList {
    #[serde(
        rename = "NodeConfigurationOption",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeConfigurationOption>,
}
impl From<Vec<NodeConfigurationOption>> for NodeConfigurationOptionList {
    fn from(v: Vec<NodeConfigurationOption>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeConfigurationOption> for NodeConfigurationOptionList {
    fn from_iter<I: IntoIterator<Item = NodeConfigurationOption>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeConfigurationOption>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeConfigurationOptionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeConfigurationOption>,
}

impl From<Vec<NodeConfigurationOption>> for XmlNodeConfigurationOptionList {
    fn from(v: Vec<NodeConfigurationOption>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeConfigurationOption> for XmlNodeConfigurationOptionList {
    fn from_iter<I: IntoIterator<Item = NodeConfigurationOption>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeConfigurationOption")]
pub struct NodeConfigurationOption {
    #[serde(rename = "EstimatedDiskUtilizationPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_disk_utilization_percent: Option<f64>,
    #[serde(rename = "Mode")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotCopyGrantsResult")]
pub struct SnapshotCopyGrantMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SnapshotCopyGrants")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_copy_grants: Option<SnapshotCopyGrantList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotCopyGrantList {
    #[serde(
        rename = "SnapshotCopyGrant",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SnapshotCopyGrant>,
}
impl From<Vec<SnapshotCopyGrant>> for SnapshotCopyGrantList {
    fn from(v: Vec<SnapshotCopyGrant>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SnapshotCopyGrant> for SnapshotCopyGrantList {
    fn from_iter<I: IntoIterator<Item = SnapshotCopyGrant>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SnapshotCopyGrant>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotCopyGrantList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SnapshotCopyGrant>,
}

impl From<Vec<SnapshotCopyGrant>> for XmlSnapshotCopyGrantList {
    fn from(v: Vec<SnapshotCopyGrant>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SnapshotCopyGrant> for XmlSnapshotCopyGrantList {
    fn from_iter<I: IntoIterator<Item = SnapshotCopyGrant>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeClusterSecurityGroupIngressResult")]
pub struct RevokeClusterSecurityGroupIngressResult {
    #[serde(rename = "ClusterSecurityGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_group: Option<ClusterSecurityGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetClusterCredentialsWithIAMMessage")]
pub struct GetClusterCredentialsWithIAMMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_domain_name: Option<String>,
    #[serde(rename = "DbName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_name: Option<String>,
    #[serde(rename = "DurationSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CopyClusterSnapshotMessage")]
pub struct CopyClusterSnapshotMessage {
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "SourceSnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SourceSnapshotIdentifier")]
    #[serde(default)]
    pub source_snapshot_identifier: String,
    #[serde(rename = "TargetSnapshotIdentifier")]
    #[serde(default)]
    pub target_snapshot_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeResizeMessage")]
pub struct DescribeResizeMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AssociateDataShareConsumerMessage")]
pub struct AssociateDataShareConsumerMessage {
    #[serde(rename = "AllowWrites")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_writes: Option<bool>,
    #[serde(rename = "AssociateEntireAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associate_entire_account: Option<bool>,
    #[serde(rename = "ConsumerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_arn: Option<String>,
    #[serde(rename = "ConsumerRegion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_region: Option<String>,
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    pub data_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeInboundIntegrationsResult")]
pub struct InboundIntegrationsMessage {
    #[serde(rename = "InboundIntegrations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_integrations: Option<InboundIntegrationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct InboundIntegrationList {
    #[serde(
        rename = "InboundIntegration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<InboundIntegration>,
}
impl From<Vec<InboundIntegration>> for InboundIntegrationList {
    fn from(v: Vec<InboundIntegration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<InboundIntegration> for InboundIntegrationList {
    fn from_iter<I: IntoIterator<Item = InboundIntegration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<InboundIntegration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlInboundIntegrationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<InboundIntegration>,
}

impl From<Vec<InboundIntegration>> for XmlInboundIntegrationList {
    fn from(v: Vec<InboundIntegration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<InboundIntegration> for XmlInboundIntegrationList {
    fn from_iter<I: IntoIterator<Item = InboundIntegration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "InboundIntegration")]
pub struct InboundIntegration {
    #[serde(rename = "CreateTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(rename = "Errors")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<IntegrationErrorList>,
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_arn: Option<String>,
    #[serde(rename = "SourceArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_arn: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterSubnetGroupResult")]
pub struct ModifyClusterSubnetGroupResult {
    #[serde(rename = "ClusterSubnetGroup")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group: Option<ClusterSubnetGroup>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateScheduledActionMessage")]
pub struct CreateScheduledActionMessage {
    #[serde(rename = "Enable")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
    #[serde(rename = "EndTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(rename = "IamRole")]
    #[serde(default)]
    pub iam_role: String,
    #[serde(rename = "Schedule")]
    #[serde(default)]
    pub schedule: String,
    #[serde(rename = "ScheduledActionDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_action_description: Option<String>,
    #[serde(rename = "ScheduledActionName")]
    #[serde(default)]
    pub scheduled_action_name: String,
    #[serde(rename = "StartTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "TargetAction")]
    #[serde(default)]
    pub target_action: ScheduledActionType,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeSnapshotSchedulesResult")]
pub struct DescribeSnapshotSchedulesOutputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "SnapshotSchedules")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedules: Option<SnapshotScheduleList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SnapshotScheduleList {
    #[serde(
        rename = "SnapshotSchedule",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SnapshotSchedule>,
}
impl From<Vec<SnapshotSchedule>> for SnapshotScheduleList {
    fn from(v: Vec<SnapshotSchedule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SnapshotSchedule> for SnapshotScheduleList {
    fn from_iter<I: IntoIterator<Item = SnapshotSchedule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SnapshotSchedule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSnapshotScheduleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SnapshotSchedule>,
}

impl From<Vec<SnapshotSchedule>> for XmlSnapshotScheduleList {
    fn from(v: Vec<SnapshotSchedule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SnapshotSchedule> for XmlSnapshotScheduleList {
    fn from_iter<I: IntoIterator<Item = SnapshotSchedule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifySnapshotScheduleResult")]
pub struct SnapshotSchedule {
    #[serde(rename = "AssociatedClusterCount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_cluster_count: Option<i32>,
    #[serde(rename = "AssociatedClusters")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub associated_clusters: Option<AssociatedClusterList>,
    #[serde(rename = "NextInvocations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invocations: Option<ScheduledSnapshotTimeList>,
    #[serde(rename = "ScheduleDefinitions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_definitions: Option<ScheduleDefinitionList>,
    #[serde(rename = "ScheduleDescription")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_description: Option<String>,
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_identifier: Option<String>,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AssociatedClusterList {
    #[serde(
        rename = "ClusterAssociatedToSchedule",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterAssociatedToSchedule>,
}
impl From<Vec<ClusterAssociatedToSchedule>> for AssociatedClusterList {
    fn from(v: Vec<ClusterAssociatedToSchedule>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterAssociatedToSchedule> for AssociatedClusterList {
    fn from_iter<I: IntoIterator<Item = ClusterAssociatedToSchedule>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterAssociatedToSchedule>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterAssociatedToScheduleList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterAssociatedToSchedule>,
}

impl From<Vec<ClusterAssociatedToSchedule>> for XmlClusterAssociatedToScheduleList {
    fn from(v: Vec<ClusterAssociatedToSchedule>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterAssociatedToSchedule> for XmlClusterAssociatedToScheduleList {
    fn from_iter<I: IntoIterator<Item = ClusterAssociatedToSchedule>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterAssociatedToSchedule")]
pub struct ClusterAssociatedToSchedule {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "ScheduleAssociationState")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_association_state: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ScheduledSnapshotTimeList {
    #[serde(
        rename = "SnapshotTime",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ScheduledSnapshotTimeList {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ScheduledSnapshotTimeList {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AcceptReservedNodeExchangeInputMessage")]
pub struct AcceptReservedNodeExchangeInputMessage {
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    pub reserved_node_id: String,
    #[serde(rename = "TargetReservedNodeOfferingId")]
    #[serde(default)]
    pub target_reserved_node_offering_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterTracksResult")]
pub struct TrackListMessage {
    #[serde(rename = "MaintenanceTracks")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_tracks: Option<TrackList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct TrackList {
    #[serde(
        rename = "MaintenanceTrack",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<MaintenanceTrack>,
}
impl From<Vec<MaintenanceTrack>> for TrackList {
    fn from(v: Vec<MaintenanceTrack>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<MaintenanceTrack> for TrackList {
    fn from_iter<I: IntoIterator<Item = MaintenanceTrack>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<MaintenanceTrack>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlMaintenanceTrackList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<MaintenanceTrack>,
}

impl From<Vec<MaintenanceTrack>> for XmlMaintenanceTrackList {
    fn from(v: Vec<MaintenanceTrack>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<MaintenanceTrack> for XmlMaintenanceTrackList {
    fn from_iter<I: IntoIterator<Item = MaintenanceTrack>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "MaintenanceTrack")]
pub struct MaintenanceTrack {
    #[serde(rename = "DatabaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_version: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "UpdateTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_targets: Option<EligibleTracksToUpdateList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct EligibleTracksToUpdateList {
    #[serde(
        rename = "UpdateTarget",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<UpdateTarget>,
}
impl From<Vec<UpdateTarget>> for EligibleTracksToUpdateList {
    fn from(v: Vec<UpdateTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<UpdateTarget> for EligibleTracksToUpdateList {
    fn from_iter<I: IntoIterator<Item = UpdateTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<UpdateTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlUpdateTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<UpdateTarget>,
}

impl From<Vec<UpdateTarget>> for XmlUpdateTargetList {
    fn from(v: Vec<UpdateTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<UpdateTarget> for XmlUpdateTargetList {
    fn from_iter<I: IntoIterator<Item = UpdateTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdateTarget")]
pub struct UpdateTarget {
    #[serde(rename = "DatabaseVersion")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_version: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "SupportedOperations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_operations: Option<SupportedOperationList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SupportedOperationList {
    #[serde(
        rename = "SupportedOperation",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<SupportedOperation>,
}
impl From<Vec<SupportedOperation>> for SupportedOperationList {
    fn from(v: Vec<SupportedOperation>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<SupportedOperation> for SupportedOperationList {
    fn from_iter<I: IntoIterator<Item = SupportedOperation>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<SupportedOperation>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlSupportedOperationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<SupportedOperation>,
}

impl From<Vec<SupportedOperation>> for XmlSupportedOperationList {
    fn from(v: Vec<SupportedOperation>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<SupportedOperation> for XmlSupportedOperationList {
    fn from_iter<I: IntoIterator<Item = SupportedOperation>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "SupportedOperation")]
pub struct SupportedOperation {
    #[serde(rename = "OperationName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeDataSharesForProducerMessage")]
pub struct DescribeDataSharesForProducerMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ProducerArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer_arn: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterSubnetGroupMessage")]
pub struct ModifyClusterSubnetGroupMessage {
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    pub cluster_subnet_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "SubnetIds")]
    #[serde(default)]
    pub subnet_ids: SubnetIdentifierList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterParameterGroupMessage")]
pub struct CreateClusterParameterGroupMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "ParameterGroupFamily")]
    #[serde(default)]
    pub parameter_group_family: String,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "UpdatePartnerStatusInputMessage")]
pub struct UpdatePartnerStatusInputMessage {
    #[serde(rename = "AccountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DatabaseName")]
    #[serde(default)]
    pub database_name: String,
    #[serde(rename = "PartnerName")]
    #[serde(default)]
    pub partner_name: String,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: String,
    #[serde(rename = "StatusMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeLoggingStatusMessage")]
pub struct DescribeLoggingStatusMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeNodeConfigurationOptionsMessage")]
pub struct DescribeNodeConfigurationOptionsMessage {
    #[serde(rename = "ActionType")]
    #[serde(default)]
    pub action_type: String,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Filter")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<NodeConfigurationOptionsFilterList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NodeConfigurationOptionsFilterList {
    #[serde(
        rename = "NodeConfigurationOptionsFilter",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<NodeConfigurationOptionsFilter>,
}
impl From<Vec<NodeConfigurationOptionsFilter>> for NodeConfigurationOptionsFilterList {
    fn from(v: Vec<NodeConfigurationOptionsFilter>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<NodeConfigurationOptionsFilter> for NodeConfigurationOptionsFilterList {
    fn from_iter<I: IntoIterator<Item = NodeConfigurationOptionsFilter>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<NodeConfigurationOptionsFilter>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlNodeConfigurationOptionsFilterList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<NodeConfigurationOptionsFilter>,
}

impl From<Vec<NodeConfigurationOptionsFilter>> for XmlNodeConfigurationOptionsFilterList {
    fn from(v: Vec<NodeConfigurationOptionsFilter>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<NodeConfigurationOptionsFilter> for XmlNodeConfigurationOptionsFilterList {
    fn from_iter<I: IntoIterator<Item = NodeConfigurationOptionsFilter>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "NodeConfigurationOptionsFilter")]
pub struct NodeConfigurationOptionsFilter {
    #[serde(rename = "Name")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Operator")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(rename = "Value")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<ValueStringList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyUsageLimitMessage")]
pub struct ModifyUsageLimitMessage {
    #[serde(rename = "Amount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(rename = "BreachAction")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breach_action: Option<String>,
    #[serde(rename = "UsageLimitId")]
    #[serde(default)]
    pub usage_limit_id: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeResizeResult")]
pub struct ResizeProgressMessage {
    #[serde(rename = "AvgResizeRateInMegaBytesPerSecond")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_resize_rate_in_mega_bytes_per_second: Option<f64>,
    #[serde(rename = "DataTransferProgressPercent")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_transfer_progress_percent: Option<f64>,
    #[serde(rename = "ElapsedTimeInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elapsed_time_in_seconds: Option<i64>,
    #[serde(rename = "EstimatedTimeToCompletionInSeconds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_time_to_completion_in_seconds: Option<i64>,
    #[serde(rename = "ImportTablesCompleted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_tables_completed: Option<ImportTablesCompleted>,
    #[serde(rename = "ImportTablesInProgress")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_tables_in_progress: Option<ImportTablesInProgress>,
    #[serde(rename = "ImportTablesNotStarted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_tables_not_started: Option<ImportTablesNotStarted>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "ProgressInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_in_mega_bytes: Option<i64>,
    #[serde(rename = "ResizeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resize_type: Option<String>,
    #[serde(rename = "Status")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "TargetClusterType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cluster_type: Option<String>,
    #[serde(rename = "TargetEncryptionType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_encryption_type: Option<String>,
    #[serde(rename = "TargetNodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_node_type: Option<String>,
    #[serde(rename = "TargetNumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_number_of_nodes: Option<i32>,
    #[serde(rename = "TotalResizeDataInMegaBytes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_resize_data_in_mega_bytes: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTablesCompleted {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ImportTablesCompleted {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ImportTablesCompleted {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTablesInProgress {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ImportTablesInProgress {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ImportTablesInProgress {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ImportTablesNotStarted {
    #[serde(rename = "member", default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<String>,
}
impl From<Vec<String>> for ImportTablesNotStarted {
    fn from(v: Vec<String>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<String> for ImportTablesNotStarted {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteHsmClientCertificateMessage")]
pub struct DeleteHsmClientCertificateMessage {
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    pub hsm_client_certificate_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeReservedNodeExchangeStatusResult")]
pub struct DescribeReservedNodeExchangeStatusOutputMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "ReservedNodeExchangeStatusDetails")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_exchange_status_details: Option<ReservedNodeExchangeStatusList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ReservedNodeExchangeStatusList {
    #[serde(
        rename = "ReservedNodeExchangeStatus",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ReservedNodeExchangeStatus>,
}
impl From<Vec<ReservedNodeExchangeStatus>> for ReservedNodeExchangeStatusList {
    fn from(v: Vec<ReservedNodeExchangeStatus>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ReservedNodeExchangeStatus> for ReservedNodeExchangeStatusList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeExchangeStatus>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ReservedNodeExchangeStatus>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlReservedNodeExchangeStatusList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ReservedNodeExchangeStatus>,
}

impl From<Vec<ReservedNodeExchangeStatus>> for XmlReservedNodeExchangeStatusList {
    fn from(v: Vec<ReservedNodeExchangeStatus>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ReservedNodeExchangeStatus> for XmlReservedNodeExchangeStatusList {
    fn from_iter<I: IntoIterator<Item = ReservedNodeExchangeStatus>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "GetIdentityCenterAuthTokenResult")]
pub struct GetIdentityCenterAuthTokenResponse {
    #[serde(rename = "ExpirationTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "Token")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterSnapshotResult")]
pub struct ModifyClusterSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterSecurityGroupMessage")]
pub struct CreateClusterSecurityGroupMessage {
    #[serde(rename = "ClusterSecurityGroupName")]
    #[serde(default)]
    pub cluster_security_group_name: String,
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateCustomDomainAssociationMessage")]
pub struct CreateCustomDomainAssociationMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "CustomDomainCertificateArn")]
    #[serde(default)]
    pub custom_domain_certificate_arn: String,
    #[serde(rename = "CustomDomainName")]
    #[serde(default)]
    pub custom_domain_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterSecurityGroupsResult")]
pub struct ClusterSecurityGroupMessage {
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<ClusterSecurityGroups>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterSecurityGroups {
    #[serde(
        rename = "ClusterSecurityGroup",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterSecurityGroup>,
}
impl From<Vec<ClusterSecurityGroup>> for ClusterSecurityGroups {
    fn from(v: Vec<ClusterSecurityGroup>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterSecurityGroup> for ClusterSecurityGroups {
    fn from_iter<I: IntoIterator<Item = ClusterSecurityGroup>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterSecurityGroup>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterSecurityGroupList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterSecurityGroup>,
}

impl From<Vec<ClusterSecurityGroup>> for XmlClusterSecurityGroupList {
    fn from(v: Vec<ClusterSecurityGroup>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterSecurityGroup> for XmlClusterSecurityGroupList {
    fn from_iter<I: IntoIterator<Item = ClusterSecurityGroup>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "EnableLoggingResult")]
pub struct LoggingStatus {
    #[serde(rename = "BucketName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bucket_name: Option<String>,
    #[serde(rename = "LastFailureMessage")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_message: Option<String>,
    #[serde(rename = "LastFailureTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failure_time: Option<String>,
    #[serde(rename = "LastSuccessfulDeliveryTime")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_successful_delivery_time: Option<String>,
    #[serde(rename = "LogDestinationType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_destination_type: Option<String>,
    #[serde(rename = "LogExports")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_exports: Option<LogTypeList>,
    #[serde(rename = "LoggingEnabled")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging_enabled: Option<bool>,
    #[serde(rename = "S3KeyPrefix")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_key_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateHsmConfigurationMessage")]
pub struct CreateHsmConfigurationMessage {
    #[serde(rename = "Description")]
    #[serde(default)]
    pub description: String,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    pub hsm_configuration_identifier: String,
    #[serde(rename = "HsmIpAddress")]
    #[serde(default)]
    pub hsm_ip_address: String,
    #[serde(rename = "HsmPartitionName")]
    #[serde(default)]
    pub hsm_partition_name: String,
    #[serde(rename = "HsmPartitionPassword")]
    #[serde(default)]
    pub hsm_partition_password: String,
    #[serde(rename = "HsmServerPublicCertificate")]
    #[serde(default)]
    pub hsm_server_public_certificate: String,
    #[serde(rename = "Tags")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<TagList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEndpointAccessMessage")]
pub struct DescribeEndpointAccessMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoint_name: Option<String>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "VpcId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyAuthenticationProfileMessage")]
pub struct ModifyAuthenticationProfileMessage {
    #[serde(rename = "AuthenticationProfileContent")]
    #[serde(default)]
    pub authentication_profile_content: String,
    #[serde(rename = "AuthenticationProfileName")]
    #[serde(default)]
    pub authentication_profile_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterParametersMessage")]
pub struct DescribeClusterParametersMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    pub parameter_group_name: String,
    #[serde(rename = "Source")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteIntegrationMessage")]
pub struct DeleteIntegrationMessage {
    #[serde(rename = "IntegrationArn")]
    #[serde(default)]
    pub integration_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "BatchModifyClusterSnapshotsMessage")]
pub struct BatchModifyClusterSnapshotsMessage {
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "SnapshotIdentifierList")]
    #[serde(default)]
    pub snapshot_identifier_list: SnapshotIdentifierList,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeAccountAttributesResult")]
pub struct AccountAttributeList {
    #[serde(rename = "AccountAttributes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_attributes: Option<AttributeList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AttributeList {
    #[serde(
        rename = "AccountAttribute",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AccountAttribute>,
}
impl From<Vec<AccountAttribute>> for AttributeList {
    fn from(v: Vec<AccountAttribute>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AccountAttribute> for AttributeList {
    fn from_iter<I: IntoIterator<Item = AccountAttribute>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AccountAttribute>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAccountAttributeList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AccountAttribute>,
}

impl From<Vec<AccountAttribute>> for XmlAccountAttributeList {
    fn from(v: Vec<AccountAttribute>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AccountAttribute> for XmlAccountAttributeList {
    fn from_iter<I: IntoIterator<Item = AccountAttribute>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AccountAttribute")]
pub struct AccountAttribute {
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
        rename = "AttributeValueTarget",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<AttributeValueTarget>,
}
impl From<Vec<AttributeValueTarget>> for AttributeValueList {
    fn from(v: Vec<AttributeValueTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<AttributeValueTarget> for AttributeValueList {
    fn from_iter<I: IntoIterator<Item = AttributeValueTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<AttributeValueTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlAttributeValueTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<AttributeValueTarget>,
}

impl From<Vec<AttributeValueTarget>> for XmlAttributeValueTargetList {
    fn from(v: Vec<AttributeValueTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<AttributeValueTarget> for XmlAttributeValueTargetList {
    fn from_iter<I: IntoIterator<Item = AttributeValueTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AttributeValueTarget")]
pub struct AttributeValueTarget {
    #[serde(rename = "AttributeValue")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute_value: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyLakehouseConfigurationMessage")]
pub struct ModifyLakehouseConfigurationMessage {
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DryRun")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(rename = "LakehouseIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_idc_application_arn: Option<String>,
    #[serde(rename = "LakehouseIdcRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_idc_registration: Option<String>,
    #[serde(rename = "LakehouseRegistration")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lakehouse_registration: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateEndpointAccessMessage")]
pub struct CreateEndpointAccessMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "ResourceOwner")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_owner: Option<String>,
    #[serde(rename = "SubnetGroupName")]
    #[serde(default)]
    pub subnet_group_name: String,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeClusterParameterGroupsMessage")]
pub struct DescribeClusterParameterGroupsMessage {
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
    #[serde(rename = "MaxRecords")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_records: Option<i32>,
    #[serde(rename = "ParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter_group_name: Option<String>,
    #[serde(rename = "TagKeys")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_keys: Option<TagKeyList>,
    #[serde(rename = "TagValues")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag_values: Option<TagValueList>,
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
    #[serde(rename = "EventId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(rename = "Message")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
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
#[serde(rename = "DescribeClusterDbRevisionsResult")]
pub struct ClusterDbRevisionsMessage {
    #[serde(rename = "ClusterDbRevisions")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_db_revisions: Option<ClusterDbRevisionsList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ClusterDbRevisionsList {
    #[serde(
        rename = "ClusterDbRevision",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<ClusterDbRevision>,
}
impl From<Vec<ClusterDbRevision>> for ClusterDbRevisionsList {
    fn from(v: Vec<ClusterDbRevision>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<ClusterDbRevision> for ClusterDbRevisionsList {
    fn from_iter<I: IntoIterator<Item = ClusterDbRevision>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<ClusterDbRevision>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlClusterDbRevisionList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<ClusterDbRevision>,
}

impl From<Vec<ClusterDbRevision>> for XmlClusterDbRevisionList {
    fn from(v: Vec<ClusterDbRevision>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<ClusterDbRevision> for XmlClusterDbRevisionList {
    fn from_iter<I: IntoIterator<Item = ClusterDbRevision>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ClusterDbRevision")]
pub struct ClusterDbRevision {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "CurrentDatabaseRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_database_revision: Option<String>,
    #[serde(rename = "DatabaseRevisionReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_revision_release_date: Option<String>,
    #[serde(rename = "RevisionTargets")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision_targets: Option<RevisionTargetsList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct RevisionTargetsList {
    #[serde(
        rename = "RevisionTarget",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<RevisionTarget>,
}
impl From<Vec<RevisionTarget>> for RevisionTargetsList {
    fn from(v: Vec<RevisionTarget>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<RevisionTarget> for RevisionTargetsList {
    fn from_iter<I: IntoIterator<Item = RevisionTarget>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<RevisionTarget>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlRevisionTargetList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<RevisionTarget>,
}

impl From<Vec<RevisionTarget>> for XmlRevisionTargetList {
    fn from(v: Vec<RevisionTarget>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<RevisionTarget> for XmlRevisionTargetList {
    fn from_iter<I: IntoIterator<Item = RevisionTarget>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevisionTarget")]
pub struct RevisionTarget {
    #[serde(rename = "DatabaseRevision")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_revision: Option<String>,
    #[serde(rename = "DatabaseRevisionReleaseDate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database_revision_release_date: Option<String>,
    #[serde(rename = "Description")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyAquaInputMessage")]
pub struct ModifyAquaInputMessage {
    #[serde(rename = "AquaConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration_status: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterDbRevisionMessage")]
pub struct ModifyClusterDbRevisionMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "RevisionTarget")]
    #[serde(default)]
    pub revision_target: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyRedshiftIdcApplicationResult")]
pub struct ModifyRedshiftIdcApplicationResult {
    #[serde(rename = "RedshiftIdcApplication")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application: Option<RedshiftIdcApplication>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RejectDataShareMessage")]
pub struct RejectDataShareMessage {
    #[serde(rename = "DataShareArn")]
    #[serde(default)]
    pub data_share_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "AuthorizeSnapshotAccessMessage")]
pub struct AuthorizeSnapshotAccessMessage {
    #[serde(rename = "AccountWithRestoreAccess")]
    #[serde(default)]
    pub account_with_restore_access: String,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RestoreFromClusterSnapshotMessage")]
pub struct RestoreFromClusterSnapshotMessage {
    #[serde(rename = "AdditionalInfo")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
    #[serde(rename = "AllowVersionUpgrade")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_version_upgrade: Option<bool>,
    #[serde(rename = "AquaConfigurationStatus")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aqua_configuration_status: Option<String>,
    #[serde(rename = "AutomatedSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated_snapshot_retention_period: Option<i32>,
    #[serde(rename = "AvailabilityZone")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone: Option<String>,
    #[serde(rename = "AvailabilityZoneRelocation")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_zone_relocation: Option<bool>,
    #[serde(rename = "CatalogName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub catalog_name: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "ClusterParameterGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_parameter_group_name: Option<String>,
    #[serde(rename = "ClusterSecurityGroups")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_security_groups: Option<ClusterSecurityGroupNameList>,
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_subnet_group_name: Option<String>,
    #[serde(rename = "DefaultIamRoleArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_iam_role_arn: Option<String>,
    #[serde(rename = "ElasticIp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elastic_ip: Option<String>,
    #[serde(rename = "Encrypted")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted: Option<bool>,
    #[serde(rename = "EnhancedVpcRouting")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_vpc_routing: Option<bool>,
    #[serde(rename = "HsmClientCertificateIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_client_certificate_identifier: Option<String>,
    #[serde(rename = "HsmConfigurationIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configuration_identifier: Option<String>,
    #[serde(rename = "IamRoles")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iam_roles: Option<IamRoleArnList>,
    #[serde(rename = "IpAddressType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_type: Option<String>,
    #[serde(rename = "KmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kms_key_id: Option<String>,
    #[serde(rename = "MaintenanceTrackName")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_track_name: Option<String>,
    #[serde(rename = "ManageMasterPassword")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manage_master_password: Option<bool>,
    #[serde(rename = "ManualSnapshotRetentionPeriod")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manual_snapshot_retention_period: Option<i32>,
    #[serde(rename = "MasterPasswordSecretKmsKeyId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub master_password_secret_kms_key_id: Option<String>,
    #[serde(rename = "MultiAZ")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_a_z: Option<bool>,
    #[serde(rename = "NodeType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_type: Option<String>,
    #[serde(rename = "NumberOfNodes")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_nodes: Option<i32>,
    #[serde(rename = "OwnerAccount")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_account: Option<String>,
    #[serde(rename = "Port")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
    #[serde(rename = "PreferredMaintenanceWindow")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_maintenance_window: Option<String>,
    #[serde(rename = "PubliclyAccessible")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publicly_accessible: Option<bool>,
    #[serde(rename = "RedshiftIdcApplicationArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redshift_idc_application_arn: Option<String>,
    #[serde(rename = "ReservedNodeId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_node_id: Option<String>,
    #[serde(rename = "SnapshotArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_arn: Option<String>,
    #[serde(rename = "SnapshotClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_cluster_identifier: Option<String>,
    #[serde(rename = "SnapshotIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_identifier: Option<String>,
    #[serde(rename = "SnapshotScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_schedule_identifier: Option<String>,
    #[serde(rename = "TargetReservedNodeOfferingId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_reserved_node_offering_id: Option<String>,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeEventCategoriesMessage")]
pub struct DescribeEventCategoriesMessage {
    #[serde(rename = "SourceType")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyClusterSnapshotScheduleMessage")]
pub struct ModifyClusterSnapshotScheduleMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
    #[serde(rename = "DisassociateSchedule")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disassociate_schedule: Option<bool>,
    #[serde(rename = "ScheduleIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_identifier: Option<String>,
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
#[serde(rename = "ModifyClusterMaintenanceResult")]
pub struct ModifyClusterMaintenanceResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "PutResourcePolicyMessage")]
pub struct PutResourcePolicyMessage {
    #[serde(rename = "Policy")]
    #[serde(default)]
    pub policy: String,
    #[serde(rename = "ResourceArn")]
    #[serde(default)]
    pub resource_arn: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "ModifyEndpointAccessMessage")]
pub struct ModifyEndpointAccessMessage {
    #[serde(rename = "EndpointName")]
    #[serde(default)]
    pub endpoint_name: String,
    #[serde(rename = "VpcSecurityGroupIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_security_group_ids: Option<VpcSecurityGroupIdList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RebootClusterMessage")]
pub struct RebootClusterMessage {
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    pub cluster_identifier: String,
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
    #[serde(rename = "Severity")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(rename = "SnsTopicArn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
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
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "CreateClusterSnapshotResult")]
pub struct CreateClusterSnapshotResult {
    #[serde(rename = "Snapshot")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<Snapshot>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterSubnetGroupMessage")]
pub struct DeleteClusterSubnetGroupMessage {
    #[serde(rename = "ClusterSubnetGroupName")]
    #[serde(default)]
    pub cluster_subnet_group_name: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "RevokeEndpointAccessMessage")]
pub struct RevokeEndpointAccessMessage {
    #[serde(rename = "Account")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    #[serde(rename = "ClusterIdentifier")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster_identifier: Option<String>,
    #[serde(rename = "Force")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force: Option<bool>,
    #[serde(rename = "VpcIds")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_ids: Option<VpcIdentifierList>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DeleteClusterResult")]
pub struct DeleteClusterResult {
    #[serde(rename = "Cluster")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cluster: Option<Cluster>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename = "DescribeHsmConfigurationsResult")]
pub struct HsmConfigurationMessage {
    #[serde(rename = "HsmConfigurations")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsm_configurations: Option<HsmConfigurationList>,
    #[serde(rename = "Marker")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HsmConfigurationList {
    #[serde(
        rename = "HsmConfiguration",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub items: Vec<HsmConfiguration>,
}
impl From<Vec<HsmConfiguration>> for HsmConfigurationList {
    fn from(v: Vec<HsmConfiguration>) -> Self {
        Self { items: v }
    }
}
impl FromIterator<HsmConfiguration> for HsmConfigurationList {
    fn from_iter<I: IntoIterator<Item = HsmConfiguration>>(iter: I) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

/// awsQuery member-wrapper for `Vec<HsmConfiguration>`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct XmlHsmConfigurationList {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member: Vec<HsmConfiguration>,
}

impl From<Vec<HsmConfiguration>> for XmlHsmConfigurationList {
    fn from(v: Vec<HsmConfiguration>) -> Self {
        Self { member: v }
    }
}

impl FromIterator<HsmConfiguration> for XmlHsmConfigurationList {
    fn from_iter<I: IntoIterator<Item = HsmConfiguration>>(iter: I) -> Self {
        Self {
            member: iter.into_iter().collect(),
        }
    }
}
