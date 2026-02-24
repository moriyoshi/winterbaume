use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Coarse-grained cluster status reported by the Redshift API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ClusterStatus {
    Available,
    Paused,
    Rebooting,
    Resizing,
    Modifying,
    Creating,
}

impl ClusterStatus {
    /// The fine-grained `ClusterStatus` value (lowercase, as returned by the
    /// real Redshift API).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Paused => "paused",
            Self::Rebooting => "rebooting",
            Self::Resizing => "resizing",
            Self::Modifying => "modifying",
            Self::Creating => "creating",
        }
    }

    /// The coarse `ClusterAvailabilityStatus` value (title-case) that the
    /// Terraform AWS provider waits on for create/update operations.
    pub fn availability_status(self) -> Option<&'static str> {
        match self {
            Self::Available => Some("Available"),
            Self::Modifying | Self::Rebooting | Self::Resizing => Some("Modifying"),
            Self::Paused => None,
            Self::Creating => Some("Unavailable"),
        }
    }
}

impl fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftCluster {
    pub cluster_identifier: String,
    pub node_type: String,
    pub cluster_status: ClusterStatus,
    pub master_username: String,
    pub db_name: String,
    pub cluster_subnet_group_name: Option<String>,
    pub vpc_id: Option<String>,
    pub availability_zone: Option<String>,
    pub number_of_nodes: i32,
    pub arn: String,
    pub endpoint_address: Option<String>,
    pub endpoint_port: Option<i32>,
    pub cluster_version: String,
    pub encrypted: bool,
    pub publicly_accessible: bool,
    pub tags: Vec<(String, String)>,
    /// Snapshot copy config: (destination_region, retention_period, snapshot_copy_grant_name)
    pub snapshot_copy: Option<(String, i32, Option<String>)>,
    pub logging_enabled: bool,
    pub logging_bucket_name: Option<String>,
    pub logging_s3_key_prefix: Option<String>,
    pub preferred_maintenance_window: Option<String>,
    pub automated_snapshot_retention_period: i32,
    pub availability_zone_relocation: bool,
}

impl RedshiftCluster {
    pub fn new(
        cluster_identifier: String,
        node_type: String,
        master_username: String,
        db_name: String,
        region: &str,
        account_id: &str,
        number_of_nodes: i32,
    ) -> Self {
        let arn = format!("arn:aws:redshift:{region}:{account_id}:cluster:{cluster_identifier}");
        let endpoint_address =
            format!("{cluster_identifier}.abc123.{region}.redshift.amazonaws.com");
        Self {
            cluster_identifier,
            node_type,
            cluster_status: ClusterStatus::Available,
            master_username,
            db_name,
            cluster_subnet_group_name: None,
            vpc_id: None,
            availability_zone: Some(format!("{region}a")),
            number_of_nodes,
            arn,
            endpoint_address: Some(endpoint_address),
            endpoint_port: Some(5439),
            cluster_version: "1.0".to_string(),
            encrypted: false,
            publicly_accessible: true,
            tags: Vec::new(),
            snapshot_copy: None,
            logging_enabled: false,
            logging_bucket_name: None,
            logging_s3_key_prefix: None,
            preferred_maintenance_window: Some("sun:06:30-sun:07:00".to_string()),
            automated_snapshot_retention_period: 1,
            availability_zone_relocation: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftSubnetGroup {
    pub name: String,
    pub description: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftParameterGroup {
    pub name: String,
    pub family: String,
    pub description: String,
    pub tags: Vec<(String, String)>,
    pub parameters: Vec<RedshiftParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftParameter {
    pub name: String,
    pub value: String,
    pub description: String,
    pub is_modifiable: bool,
    pub apply_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftSecurityGroup {
    pub name: String,
    pub description: String,
    pub tags: Vec<(String, String)>,
    pub ec2_security_groups: Vec<Ec2SecurityGroupIngress>,
    pub ip_ranges: Vec<IpRangeIngress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ec2SecurityGroupIngress {
    pub ec2_security_group_name: String,
    pub ec2_security_group_owner_id: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRangeIngress {
    pub cidrip: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftSnapshot {
    pub snapshot_identifier: String,
    pub cluster_identifier: String,
    pub status: String,
    pub arn: String,
    pub tags: Vec<(String, String)>,
    pub master_username: String,
    pub db_name: String,
    pub node_type: String,
    pub number_of_nodes: i32,
    pub cluster_version: String,
}

impl RedshiftSnapshot {
    pub fn new(
        snapshot_identifier: String,
        cluster: &RedshiftCluster,
        region: &str,
        account_id: &str,
    ) -> Self {
        let arn = format!(
            "arn:aws:redshift:{region}:{account_id}:snapshot:{}/{}",
            cluster.cluster_identifier, snapshot_identifier
        );
        Self {
            snapshot_identifier,
            cluster_identifier: cluster.cluster_identifier.clone(),
            status: "available".to_string(),
            arn,
            tags: cluster.tags.clone(),
            master_username: cluster.master_username.clone(),
            db_name: cluster.db_name.clone(),
            node_type: cluster.node_type.clone(),
            number_of_nodes: cluster.number_of_nodes,
            cluster_version: cluster.cluster_version.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftSnapshotCopyGrant {
    pub name: String,
    pub kms_key_id: Option<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TagsByArn(pub HashMap<String, Vec<(String, String)>>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftHsmClientCertificate {
    pub identifier: String,
    pub public_key: String,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftHsmConfiguration {
    pub identifier: String,
    pub description: String,
    pub hsm_ip_address: String,
    pub hsm_partition_name: String,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftAuthenticationProfile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftUsageLimit {
    pub usage_limit_id: String,
    pub cluster_identifier: String,
    pub feature_type: String,
    pub limit_type: String,
    pub amount: i64,
    pub period: Option<String>,
    pub breach_action: Option<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftSnapshotSchedule {
    pub schedule_identifier: String,
    pub schedule_definitions: Vec<String>,
    pub schedule_description: Option<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftScheduledAction {
    pub name: String,
    pub schedule: Option<String>,
    pub iam_role: Option<String>,
    pub description: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftResourcePolicy {
    pub resource_arn: String,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftPartnerIntegration {
    pub cluster_identifier: String,
    pub database_name: String,
    pub partner_name: String,
    pub status: String,
    pub status_message: Option<String>,
}

/// A reserved node record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftReservedNode {
    pub reserved_node_id: String,
    pub reserved_node_offering_id: String,
    pub node_type: String,
    pub start_time: String,
    pub duration: i32,
    pub fixed_price: f64,
    pub usage_price: f64,
    pub currency_code: String,
    pub node_count: i32,
    pub state: String,
    pub offering_type: String,
}

/// A table restore status record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftTableRestoreStatus {
    pub table_restore_request_id: String,
    pub status: String,
    pub cluster_identifier: String,
    pub snapshot_identifier: String,
    pub source_database_name: String,
    pub source_schema_name: String,
    pub source_table_name: String,
    pub target_database_name: String,
    pub target_schema_name: Option<String>,
    pub new_table_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedshiftEventSubscription {
    pub subscription_name: String,
    pub sns_topic_arn: String,
    pub source_type: Option<String>,
    pub source_ids: Vec<String>,
    pub event_categories: Vec<String>,
    pub severity: Option<String>,
    pub enabled: bool,
    pub tags: Vec<(String, String)>,
    pub status: String,
    pub creation_time: String,
    pub customer_aws_id: String,
}
