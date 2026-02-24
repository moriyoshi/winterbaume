use std::collections::HashMap;

use crate::types::{
    ClusterStatus, Ec2SecurityGroupIngress, IpRangeIngress, RedshiftAuthenticationProfile,
    RedshiftCluster, RedshiftEventSubscription, RedshiftHsmClientCertificate,
    RedshiftHsmConfiguration, RedshiftParameter, RedshiftParameterGroup,
    RedshiftPartnerIntegration, RedshiftReservedNode, RedshiftResourcePolicy,
    RedshiftScheduledAction, RedshiftSecurityGroup, RedshiftSnapshot, RedshiftSnapshotCopyGrant,
    RedshiftSnapshotSchedule, RedshiftSubnetGroup, RedshiftTableRestoreStatus, RedshiftUsageLimit,
};

#[derive(Debug, Default)]
pub struct RedshiftState {
    pub clusters: HashMap<String, RedshiftCluster>,
    pub subnet_groups: HashMap<String, RedshiftSubnetGroup>,
    pub parameter_groups: HashMap<String, RedshiftParameterGroup>,
    pub security_groups: HashMap<String, RedshiftSecurityGroup>,
    pub snapshots: HashMap<String, RedshiftSnapshot>,
    pub snapshot_copy_grants: HashMap<String, RedshiftSnapshotCopyGrant>,
    /// Tags indexed by resource ARN
    pub tags_by_arn: HashMap<String, Vec<(String, String)>>,
    pub event_subscriptions: HashMap<String, RedshiftEventSubscription>,
    pub hsm_client_certificates: HashMap<String, RedshiftHsmClientCertificate>,
    pub hsm_configurations: HashMap<String, RedshiftHsmConfiguration>,
    pub authentication_profiles: HashMap<String, RedshiftAuthenticationProfile>,
    pub usage_limits: HashMap<String, RedshiftUsageLimit>,
    pub snapshot_schedules: HashMap<String, RedshiftSnapshotSchedule>,
    pub scheduled_actions: HashMap<String, RedshiftScheduledAction>,
    pub resource_policies: HashMap<String, RedshiftResourcePolicy>,
    pub partner_integrations: Vec<RedshiftPartnerIntegration>,
    /// AQUA configurations keyed by cluster identifier
    pub aqua_configurations: HashMap<String, (String, String)>, // (aqua_configuration_status, aqua_status)
    /// Reserved nodes keyed by reserved node id
    pub reserved_nodes: HashMap<String, RedshiftReservedNode>,
    /// Table restore status records keyed by request id
    pub table_restore_statuses: HashMap<String, RedshiftTableRestoreStatus>,
}

#[derive(Debug, thiserror::Error)]
pub enum RedshiftError {
    #[error("{0}")]
    ClusterNotFound(String),
    #[error("{0}")]
    ClusterAlreadyExists(String),
    #[error("{0}")]
    InvalidParameterValue(String),
    #[error("{0}")]
    ClusterSubnetGroupAlreadyExists(String),
    #[error("{0}")]
    ClusterSubnetGroupNotFoundFault(String),
    #[error("{0}")]
    ClusterParameterGroupAlreadyExists(String),
    #[error("{0}")]
    ClusterParameterGroupNotFound(String),
    #[error("{0}")]
    ClusterSecurityGroupAlreadyExists(String),
    #[error("{0}")]
    ClusterSecurityGroupNotFound(String),
    #[error("{0}")]
    ClusterSnapshotAlreadyExists(String),
    #[error("{0}")]
    ClusterSnapshotNotFound(String),
    #[error("{0}")]
    SnapshotCopyGrantAlreadyExistsFault(String),
    #[error("{0}")]
    SnapshotCopyGrantNotFoundFault(String),
    #[error("{0}")]
    SnapshotCopyDisabledFault(String),
    #[error("{0}")]
    SubscriptionAlreadyExistFault(String),
    #[error("{0}")]
    SubscriptionNotFoundFault(String),
    #[error("{0}")]
    HsmClientCertificateAlreadyExistsFault(String),
    #[error("{0}")]
    HsmClientCertificateNotFoundFault(String),
    #[error("{0}")]
    HsmConfigurationAlreadyExistsFault(String),
    #[error("{0}")]
    HsmConfigurationNotFoundFault(String),
    #[error("{0}")]
    AuthenticationProfileAlreadyExistsFault(String),
    #[error("{0}")]
    AuthenticationProfileNotFoundFault(String),
    #[error("{0}")]
    UsageLimitNotFoundFault(String),
    #[error("{0}")]
    SnapshotScheduleAlreadyExistsFault(String),
    #[error("{0}")]
    SnapshotScheduleNotFoundFault(String),
    #[error("{0}")]
    ScheduledActionAlreadyExistsFault(String),
    #[error("{0}")]
    ScheduledActionNotFoundFault(String),
    #[error("{0}")]
    PartnerNotFoundFault(String),
}

impl RedshiftState {
    // ---- Clusters ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_cluster(
        &mut self,
        cluster_identifier: String,
        node_type: String,
        master_username: String,
        db_name: String,
        region: &str,
        account_id: &str,
        number_of_nodes: i32,
        subnet_group_name: Option<String>,
        tags: Vec<(String, String)>,
        publicly_accessible: bool,
        encrypted: bool,
        availability_zone: Option<String>,
        availability_zone_relocation: bool,
        preferred_maintenance_window: Option<String>,
        automated_snapshot_retention_period: i32,
    ) -> Result<RedshiftCluster, RedshiftError> {
        if self.clusters.contains_key(&cluster_identifier) {
            return Err(RedshiftError::ClusterAlreadyExists(format!(
                "Cluster '{cluster_identifier}' already exists"
            )));
        }
        let mut cluster = RedshiftCluster::new(
            cluster_identifier.clone(),
            node_type,
            master_username,
            db_name,
            region,
            account_id,
            number_of_nodes,
        );
        cluster.cluster_subnet_group_name = subnet_group_name;
        cluster.tags = tags.clone();
        cluster.publicly_accessible = publicly_accessible;
        cluster.encrypted = encrypted;
        cluster.availability_zone_relocation = availability_zone_relocation;
        if let Some(az) = availability_zone {
            cluster.availability_zone = Some(az);
        }
        if let Some(mw) = preferred_maintenance_window {
            cluster.preferred_maintenance_window = Some(mw);
        }
        cluster.automated_snapshot_retention_period = automated_snapshot_retention_period;
        // Register tags on ARN
        if !tags.is_empty() {
            self.tags_by_arn.insert(cluster.arn.clone(), tags);
        }
        self.clusters.insert(cluster_identifier, cluster.clone());
        Ok(cluster)
    }

    pub fn describe_clusters(
        &self,
        cluster_identifier: Option<&str>,
    ) -> Result<Vec<RedshiftCluster>, RedshiftError> {
        if let Some(id) = cluster_identifier {
            match self.clusters.get(id) {
                Some(c) => Ok(vec![c.clone()]),
                None => Err(RedshiftError::ClusterNotFound(format!(
                    "Cluster '{id}' not found"
                ))),
            }
        } else {
            Ok(self.clusters.values().cloned().collect())
        }
    }

    pub fn delete_cluster(
        &mut self,
        cluster_identifier: &str,
        final_snapshot_id: Option<&str>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.remove(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        // Remove tags
        self.tags_by_arn.remove(&cluster.arn);
        // Create final snapshot if requested
        if let Some(snap_id) = final_snapshot_id {
            let snap = RedshiftSnapshot::new(snap_id.to_string(), &cluster, region, account_id);
            self.snapshots.insert(snap_id.to_string(), snap);
        }
        Ok(cluster)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_cluster(
        &mut self,
        cluster_identifier: &str,
        new_node_type: Option<String>,
        number_of_nodes: Option<i32>,
        new_cluster_identifier: Option<String>,
        master_user_password: Option<String>,
        publicly_accessible: Option<bool>,
        encrypted: Option<bool>,
        availability_zone_relocation: Option<bool>,
        preferred_maintenance_window: Option<String>,
        automated_snapshot_retention_period: Option<i32>,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        if let Some(nt) = new_node_type {
            cluster.node_type = nt;
        }
        if let Some(n) = number_of_nodes {
            cluster.number_of_nodes = n;
        }
        if let Some(pa) = publicly_accessible {
            cluster.publicly_accessible = pa;
        }
        if let Some(enc) = encrypted {
            cluster.encrypted = enc;
        }
        if let Some(azr) = availability_zone_relocation {
            cluster.availability_zone_relocation = azr;
        }
        if let Some(mw) = preferred_maintenance_window {
            cluster.preferred_maintenance_window = Some(mw);
        }
        if let Some(rp) = automated_snapshot_retention_period {
            cluster.automated_snapshot_retention_period = rp;
        }
        let _ = master_user_password; // not stored in mock
        let result = cluster.clone();
        if let Some(new_id) = new_cluster_identifier {
            if new_id != cluster_identifier {
                let cluster = self.clusters.remove(cluster_identifier).unwrap();
                self.clusters.insert(new_id, cluster);
            }
        }
        Ok(result)
    }

    pub fn pause_cluster(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.cluster_status = ClusterStatus::Paused;
        Ok(cluster.clone())
    }

    pub fn resume_cluster(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.cluster_status = ClusterStatus::Available;
        Ok(cluster.clone())
    }

    // ---- Subnet Groups ----

    pub fn create_cluster_subnet_group(
        &mut self,
        name: String,
        description: String,
        vpc_id: String,
        subnet_ids: Vec<String>,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftSubnetGroup, RedshiftError> {
        if self.subnet_groups.contains_key(&name) {
            return Err(RedshiftError::ClusterSubnetGroupAlreadyExists(format!(
                "Subnet group '{name}' already exists"
            )));
        }
        let sg = RedshiftSubnetGroup {
            name: name.clone(),
            description,
            vpc_id,
            subnet_ids,
            tags: tags.clone(),
        };
        // Register tags
        let arn = format!("arn:aws:redshift:{region}:{account_id}:subnetgroup:{name}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.subnet_groups.insert(name, sg.clone());
        Ok(sg)
    }

    pub fn describe_cluster_subnet_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<RedshiftSubnetGroup>, RedshiftError> {
        if let Some(n) = name {
            match self.subnet_groups.get(n) {
                Some(sg) => Ok(vec![sg.clone()]),
                None => Err(RedshiftError::ClusterSubnetGroupNotFoundFault(format!(
                    "Subnet group '{n}' not found"
                ))),
            }
        } else {
            Ok(self.subnet_groups.values().cloned().collect())
        }
    }

    pub fn delete_cluster_subnet_group(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.subnet_groups.remove(name).ok_or_else(|| {
            RedshiftError::ClusterSubnetGroupNotFoundFault(format!(
                "Subnet group '{name}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Parameter Groups ----

    pub fn create_cluster_parameter_group(
        &mut self,
        name: String,
        family: String,
        description: String,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftParameterGroup, RedshiftError> {
        if self.parameter_groups.contains_key(&name) {
            return Err(RedshiftError::ClusterParameterGroupAlreadyExists(format!(
                "Parameter group '{name}' already exists"
            )));
        }
        let pg = RedshiftParameterGroup {
            name: name.clone(),
            family: family.clone(),
            description,
            tags: tags.clone(),
            parameters: default_parameters(&family),
        };
        let arn = format!("arn:aws:redshift:{region}:{account_id}:parametergroup:{name}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.parameter_groups.insert(name, pg.clone());
        Ok(pg)
    }

    pub fn describe_cluster_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<RedshiftParameterGroup>, RedshiftError> {
        if let Some(n) = name {
            match self.parameter_groups.get(n) {
                Some(pg) => Ok(vec![pg.clone()]),
                None => Err(RedshiftError::ClusterParameterGroupNotFound(format!(
                    "Parameter group '{n}' not found"
                ))),
            }
        } else {
            Ok(self.parameter_groups.values().cloned().collect())
        }
    }

    pub fn describe_cluster_parameters(
        &self,
        name: &str,
    ) -> Result<Vec<RedshiftParameter>, RedshiftError> {
        match self.parameter_groups.get(name) {
            Some(pg) => Ok(pg.parameters.clone()),
            None => Err(RedshiftError::ClusterParameterGroupNotFound(format!(
                "Parameter group '{name}' not found"
            ))),
        }
    }

    pub fn delete_cluster_parameter_group(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.parameter_groups.remove(name).ok_or_else(|| {
            RedshiftError::ClusterParameterGroupNotFound(format!(
                "Parameter group '{name}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Security Groups ----

    pub fn create_cluster_security_group(
        &mut self,
        name: String,
        description: String,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftSecurityGroup, RedshiftError> {
        if self.security_groups.contains_key(&name) {
            return Err(RedshiftError::ClusterSecurityGroupAlreadyExists(format!(
                "Security group '{name}' already exists"
            )));
        }
        let sg = RedshiftSecurityGroup {
            name: name.clone(),
            description,
            tags: tags.clone(),
            ec2_security_groups: Vec::new(),
            ip_ranges: Vec::new(),
        };
        let arn = format!("arn:aws:redshift:{region}:{account_id}:securitygroup:{name}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.security_groups.insert(name, sg.clone());
        Ok(sg)
    }

    pub fn describe_cluster_security_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<RedshiftSecurityGroup>, RedshiftError> {
        if let Some(n) = name {
            match self.security_groups.get(n) {
                Some(sg) => Ok(vec![sg.clone()]),
                None => Err(RedshiftError::ClusterSecurityGroupNotFound(format!(
                    "Security group '{n}' not found"
                ))),
            }
        } else {
            Ok(self.security_groups.values().cloned().collect())
        }
    }

    pub fn delete_cluster_security_group(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.security_groups.remove(name).ok_or_else(|| {
            RedshiftError::ClusterSecurityGroupNotFound(format!(
                "Security group '{name}' not found"
            ))
        })?;
        Ok(())
    }

    pub fn authorize_cluster_security_group_ingress(
        &mut self,
        name: &str,
        ec2_security_group_name: Option<String>,
        ec2_security_group_owner_id: Option<String>,
        cidrip: Option<String>,
    ) -> Result<RedshiftSecurityGroup, RedshiftError> {
        let sg = self.security_groups.get_mut(name).ok_or_else(|| {
            RedshiftError::ClusterSecurityGroupNotFound(format!(
                "Security group '{name}' not found"
            ))
        })?;
        if let Some(ec2_name) = ec2_security_group_name {
            sg.ec2_security_groups.push(Ec2SecurityGroupIngress {
                ec2_security_group_name: ec2_name,
                ec2_security_group_owner_id,
                status: "authorized".to_string(),
            });
        }
        if let Some(cidr) = cidrip {
            sg.ip_ranges.push(IpRangeIngress {
                cidrip: cidr,
                status: "authorized".to_string(),
            });
        }
        Ok(sg.clone())
    }

    // ---- Snapshots ----

    pub fn create_cluster_snapshot(
        &mut self,
        snapshot_identifier: String,
        cluster_identifier: &str,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftSnapshot, RedshiftError> {
        if self.snapshots.contains_key(&snapshot_identifier) {
            return Err(RedshiftError::ClusterSnapshotAlreadyExists(format!(
                "Snapshot '{snapshot_identifier}' already exists"
            )));
        }
        let cluster = self.clusters.get(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        let mut snap =
            RedshiftSnapshot::new(snapshot_identifier.clone(), cluster, region, account_id);
        if !tags.is_empty() {
            snap.tags = tags.clone();
            self.tags_by_arn.insert(snap.arn.clone(), tags);
        }
        self.snapshots.insert(snapshot_identifier, snap.clone());
        Ok(snap)
    }

    pub fn describe_cluster_snapshots(
        &self,
        snapshot_identifier: Option<&str>,
        cluster_identifier: Option<&str>,
    ) -> Result<Vec<RedshiftSnapshot>, RedshiftError> {
        let snaps: Vec<RedshiftSnapshot> = self
            .snapshots
            .values()
            .filter(|s| {
                if let Some(id) = snapshot_identifier {
                    if s.snapshot_identifier != id {
                        return false;
                    }
                }
                if let Some(cid) = cluster_identifier {
                    if s.cluster_identifier != cid {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect();

        if let Some(id) = snapshot_identifier {
            if snaps.is_empty() {
                return Err(RedshiftError::ClusterSnapshotNotFound(format!(
                    "Snapshot '{id}' not found"
                )));
            }
        }
        Ok(snaps)
    }

    pub fn delete_cluster_snapshot(
        &mut self,
        snapshot_identifier: &str,
    ) -> Result<RedshiftSnapshot, RedshiftError> {
        self.snapshots.remove(snapshot_identifier).ok_or_else(|| {
            RedshiftError::ClusterSnapshotNotFound(format!(
                "Snapshot '{snapshot_identifier}' not found"
            ))
        })
    }

    pub fn restore_from_cluster_snapshot(
        &mut self,
        cluster_identifier: String,
        snapshot_identifier: &str,
        region: &str,
        account_id: &str,
        subnet_group_name: Option<String>,
        publicly_accessible: bool,
        availability_zone_relocation: bool,
    ) -> Result<RedshiftCluster, RedshiftError> {
        if self.clusters.contains_key(&cluster_identifier) {
            return Err(RedshiftError::ClusterAlreadyExists(format!(
                "Cluster '{cluster_identifier}' already exists"
            )));
        }
        let snap = self.snapshots.get(snapshot_identifier).ok_or_else(|| {
            RedshiftError::ClusterSnapshotNotFound(format!(
                "Snapshot '{snapshot_identifier}' not found"
            ))
        })?;

        let mut cluster = RedshiftCluster::new(
            cluster_identifier.clone(),
            snap.node_type.clone(),
            snap.master_username.clone(),
            snap.db_name.clone(),
            region,
            account_id,
            snap.number_of_nodes,
        );
        cluster.cluster_subnet_group_name = subnet_group_name;
        cluster.publicly_accessible = publicly_accessible;
        cluster.availability_zone_relocation = availability_zone_relocation;
        cluster.cluster_status = ClusterStatus::Available;
        self.clusters.insert(cluster_identifier, cluster.clone());
        Ok(cluster)
    }

    // ---- Snapshot Copy Grants ----

    pub fn create_snapshot_copy_grant(
        &mut self,
        name: String,
        kms_key_id: Option<String>,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftSnapshotCopyGrant, RedshiftError> {
        if self.snapshot_copy_grants.contains_key(&name) {
            return Err(RedshiftError::SnapshotCopyGrantAlreadyExistsFault(format!(
                "Snapshot copy grant '{name}' already exists"
            )));
        }
        let grant = RedshiftSnapshotCopyGrant {
            name: name.clone(),
            kms_key_id,
            tags: tags.clone(),
        };
        let arn = format!("arn:aws:redshift:{region}:{account_id}:snapshotcopygrant:{name}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.snapshot_copy_grants.insert(name, grant.clone());
        Ok(grant)
    }

    pub fn delete_snapshot_copy_grant(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.snapshot_copy_grants.remove(name).ok_or_else(|| {
            RedshiftError::SnapshotCopyGrantNotFoundFault(format!(
                "Snapshot copy grant '{name}' not found"
            ))
        })?;
        Ok(())
    }

    pub fn describe_snapshot_copy_grants(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<RedshiftSnapshotCopyGrant>, RedshiftError> {
        if let Some(n) = name {
            match self.snapshot_copy_grants.get(n) {
                Some(g) => Ok(vec![g.clone()]),
                None => Err(RedshiftError::SnapshotCopyGrantNotFoundFault(format!(
                    "Snapshot copy grant '{n}' not found"
                ))),
            }
        } else {
            Ok(self.snapshot_copy_grants.values().cloned().collect())
        }
    }

    // ---- Snapshot Copy Enable/Disable ----

    pub fn enable_snapshot_copy(
        &mut self,
        cluster_identifier: &str,
        destination_region: String,
        retention_period: i32,
        snapshot_copy_grant_name: Option<String>,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.snapshot_copy = Some((
            destination_region,
            retention_period,
            snapshot_copy_grant_name,
        ));
        Ok(cluster.clone())
    }

    pub fn disable_snapshot_copy(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.snapshot_copy = None;
        Ok(cluster.clone())
    }

    pub fn modify_snapshot_copy_retention_period(
        &mut self,
        cluster_identifier: &str,
        retention_period: i32,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        if let Some((dest, _, grant)) = cluster.snapshot_copy.take() {
            cluster.snapshot_copy = Some((dest, retention_period, grant));
        } else {
            return Err(RedshiftError::SnapshotCopyDisabledFault(
                "Snapshot copy not enabled for this cluster".to_string(),
            ));
        }
        Ok(cluster.clone())
    }

    // ---- Logging ----

    pub fn enable_logging(
        &mut self,
        cluster_identifier: &str,
        bucket_name: String,
        s3_key_prefix: Option<String>,
    ) -> Result<(bool, Option<String>, Option<String>), RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.logging_enabled = true;
        cluster.logging_bucket_name = Some(bucket_name);
        cluster.logging_s3_key_prefix = s3_key_prefix;
        Ok((
            cluster.logging_enabled,
            cluster.logging_bucket_name.clone(),
            cluster.logging_s3_key_prefix.clone(),
        ))
    }

    pub fn disable_logging(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<(bool, Option<String>, Option<String>), RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.logging_enabled = false;
        cluster.logging_bucket_name = None;
        cluster.logging_s3_key_prefix = None;
        Ok((false, None, None))
    }

    pub fn describe_logging_status(
        &self,
        cluster_identifier: &str,
    ) -> Result<(bool, Option<String>, Option<String>), RedshiftError> {
        let cluster = self.clusters.get(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        Ok((
            cluster.logging_enabled,
            cluster.logging_bucket_name.clone(),
            cluster.logging_s3_key_prefix.clone(),
        ))
    }

    // ---- Tags ----

    pub fn create_tags(&mut self, resource_name: &str, tags: Vec<(String, String)>) {
        let existing = self
            .tags_by_arn
            .entry(resource_name.to_string())
            .or_default();
        for (k, v) in tags {
            if let Some(pos) = existing.iter().position(|(ek, _)| *ek == k) {
                existing[pos].1 = v;
            } else {
                existing.push((k, v));
            }
        }
        // Sync tags back to the relevant resource
        let tags_clone = self
            .tags_by_arn
            .get(resource_name)
            .cloned()
            .unwrap_or_default();
        // Update cluster tags
        for cluster in self.clusters.values_mut() {
            if cluster.arn == resource_name {
                cluster.tags = tags_clone.clone();
                break;
            }
        }
        // Update snapshot tags
        for snap in self.snapshots.values_mut() {
            if snap.arn == resource_name {
                snap.tags = tags_clone.clone();
                break;
            }
        }
    }

    pub fn delete_tags(&mut self, resource_name: &str, tag_keys: Vec<String>) {
        if let Some(tags) = self.tags_by_arn.get_mut(resource_name) {
            tags.retain(|(k, _)| !tag_keys.contains(k));
        }
        // Sync back
        let tags_clone = self
            .tags_by_arn
            .get(resource_name)
            .cloned()
            .unwrap_or_default();
        for cluster in self.clusters.values_mut() {
            if cluster.arn == resource_name {
                cluster.tags = tags_clone.clone();
                break;
            }
        }
        for snap in self.snapshots.values_mut() {
            if snap.arn == resource_name {
                snap.tags = tags_clone.clone();
                break;
            }
        }
    }

    pub fn describe_tags(
        &self,
        resource_name: Option<&str>,
        resource_type: Option<&str>,
    ) -> Vec<(String, String, String)> {
        // Returns Vec of (resource_name, tag_key, tag_value)
        let mut results = Vec::new();
        for (arn, tags) in &self.tags_by_arn {
            if let Some(rn) = resource_name {
                if arn != rn {
                    continue;
                }
            }
            if let Some(rt) = resource_type {
                // e.g. "cluster", "snapshot"
                let arn_type = extract_resource_type_from_arn(arn);
                if !arn_type.eq_ignore_ascii_case(rt) {
                    continue;
                }
            }
            for (k, v) in tags {
                results.push((arn.clone(), k.clone(), v.clone()));
            }
        }
        results
    }

    // ---- Credentials ----

    pub fn get_cluster_credentials(
        &self,
        cluster_identifier: &str,
        db_user: &str,
        db_name: Option<&str>,
    ) -> Result<(String, String, String), RedshiftError> {
        let _cluster = self.clusters.get(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        let db = db_name.unwrap_or("dev");
        let password = format!("Pwd-{db_user}-{}", uuid::Uuid::new_v4().simple());
        let expiration = chrono::Utc::now() + chrono::Duration::hours(1);
        Ok((
            format!("IAM:{db_user}"),
            password,
            expiration.format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        ))
    }

    // ---- Default Cluster Parameters ----

    pub fn describe_default_cluster_parameters(
        &self,
        parameter_group_family: &str,
    ) -> Vec<RedshiftParameter> {
        default_parameters(parameter_group_family)
    }

    // ---- Reboot / Resize ----

    pub fn reboot_cluster(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.cluster_status = ClusterStatus::Rebooting;
        Ok(cluster.clone())
    }

    pub fn resize_cluster(
        &mut self,
        cluster_identifier: &str,
        node_type: Option<String>,
        number_of_nodes: Option<i32>,
        cluster_type: Option<String>,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        if let Some(nt) = node_type {
            cluster.node_type = nt;
        }
        if let Some(n) = number_of_nodes {
            cluster.number_of_nodes = n;
        }
        let _ = cluster_type; // not stored
        cluster.cluster_status = ClusterStatus::Resizing;
        Ok(cluster.clone())
    }

    // ---- Modify/Reset Parameter Group ----

    pub fn modify_cluster_parameter_group(
        &mut self,
        name: &str,
        parameters: Vec<(String, String)>,
    ) -> Result<(), RedshiftError> {
        let pg = self.parameter_groups.get_mut(name).ok_or_else(|| {
            RedshiftError::ClusterParameterGroupNotFound(format!(
                "Parameter group '{name}' not found"
            ))
        })?;
        for (param_name, param_value) in parameters {
            if let Some(p) = pg.parameters.iter_mut().find(|p| p.name == param_name) {
                p.value = param_value;
            } else {
                pg.parameters.push(RedshiftParameter {
                    name: param_name,
                    value: param_value,
                    description: String::new(),
                    is_modifiable: true,
                    apply_type: "dynamic".to_string(),
                });
            }
        }
        Ok(())
    }

    pub fn reset_cluster_parameter_group(
        &mut self,
        name: &str,
        reset_all_parameters: bool,
        parameters: Vec<String>,
    ) -> Result<(), RedshiftError> {
        let family = {
            let pg = self.parameter_groups.get(name).ok_or_else(|| {
                RedshiftError::ClusterParameterGroupNotFound(format!(
                    "Parameter group '{name}' not found"
                ))
            })?;
            pg.family.clone()
        };
        let defaults = default_parameters(&family);
        let pg = self.parameter_groups.get_mut(name).unwrap();
        if reset_all_parameters {
            pg.parameters = defaults;
        } else {
            for param_name in parameters {
                if let Some(def) = defaults.iter().find(|p| p.name == param_name) {
                    if let Some(p) = pg.parameters.iter_mut().find(|p| p.name == param_name) {
                        p.value = def.value.clone();
                    }
                }
            }
        }
        Ok(())
    }

    // ---- Copy Cluster Snapshot ----

    pub fn copy_cluster_snapshot(
        &mut self,
        source_snapshot_identifier: &str,
        target_snapshot_identifier: String,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftSnapshot, RedshiftError> {
        if self.snapshots.contains_key(&target_snapshot_identifier) {
            return Err(RedshiftError::ClusterSnapshotAlreadyExists(format!(
                "Snapshot '{target_snapshot_identifier}' already exists"
            )));
        }
        let source = self
            .snapshots
            .get(source_snapshot_identifier)
            .ok_or_else(|| {
                RedshiftError::ClusterSnapshotNotFound(format!(
                    "Snapshot '{source_snapshot_identifier}' not found"
                ))
            })?
            .clone();

        let arn = format!(
            "arn:aws:redshift:{region}:{account_id}:snapshot:{}/{}",
            source.cluster_identifier, target_snapshot_identifier
        );
        let mut new_snap = RedshiftSnapshot {
            snapshot_identifier: target_snapshot_identifier.clone(),
            cluster_identifier: source.cluster_identifier.clone(),
            status: "available".to_string(),
            arn,
            tags: if tags.is_empty() {
                source.tags.clone()
            } else {
                tags.clone()
            },
            master_username: source.master_username.clone(),
            db_name: source.db_name.clone(),
            node_type: source.node_type.clone(),
            number_of_nodes: source.number_of_nodes,
            cluster_version: source.cluster_version.clone(),
        };
        if !new_snap.tags.is_empty() {
            self.tags_by_arn
                .insert(new_snap.arn.clone(), new_snap.tags.clone());
        }
        self.snapshots
            .insert(target_snapshot_identifier, new_snap.clone());
        Ok(new_snap)
    }

    // ---- Revoke Security Group Ingress ----

    pub fn revoke_cluster_security_group_ingress(
        &mut self,
        name: &str,
        ec2_security_group_name: Option<String>,
        ec2_security_group_owner_id: Option<String>,
        cidrip: Option<String>,
    ) -> Result<RedshiftSecurityGroup, RedshiftError> {
        let sg = self.security_groups.get_mut(name).ok_or_else(|| {
            RedshiftError::ClusterSecurityGroupNotFound(format!(
                "Security group '{name}' not found"
            ))
        })?;
        if let Some(ec2_name) = &ec2_security_group_name {
            sg.ec2_security_groups
                .retain(|e| &e.ec2_security_group_name != ec2_name);
        }
        if let Some(cidr) = &cidrip {
            sg.ip_ranges.retain(|ip| &ip.cidrip != cidr);
        }
        let _ = ec2_security_group_owner_id;
        Ok(sg.clone())
    }

    // ---- Modify Cluster Subnet Group ----

    pub fn modify_cluster_subnet_group(
        &mut self,
        name: &str,
        description: Option<String>,
        subnet_ids: Vec<String>,
    ) -> Result<RedshiftSubnetGroup, RedshiftError> {
        let sg = self.subnet_groups.get_mut(name).ok_or_else(|| {
            RedshiftError::ClusterSubnetGroupNotFoundFault(format!(
                "Subnet group '{name}' not found"
            ))
        })?;
        if let Some(desc) = description {
            sg.description = desc;
        }
        if !subnet_ids.is_empty() {
            sg.subnet_ids = subnet_ids;
        }
        Ok(sg.clone())
    }

    // ---- Batch Delete Cluster Snapshots ----

    pub fn batch_delete_cluster_snapshots(
        &mut self,
        snapshot_identifiers: Vec<String>,
    ) -> (Vec<String>, Vec<(String, String)>) {
        let mut deleted = Vec::new();
        let mut errors = Vec::new();
        for id in snapshot_identifiers {
            if self.snapshots.remove(&id).is_some() {
                deleted.push(id);
            } else {
                errors.push((id, "ClusterSnapshotNotFound".to_string()));
            }
        }
        (deleted, errors)
    }

    // ---- Event Subscriptions ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_event_subscription(
        &mut self,
        subscription_name: String,
        sns_topic_arn: String,
        source_type: Option<String>,
        source_ids: Vec<String>,
        event_categories: Vec<String>,
        severity: Option<String>,
        enabled: bool,
        tags: Vec<(String, String)>,
        account_id: &str,
    ) -> Result<RedshiftEventSubscription, RedshiftError> {
        if self.event_subscriptions.contains_key(&subscription_name) {
            return Err(RedshiftError::SubscriptionAlreadyExistFault(format!(
                "Subscription '{subscription_name}' already exists"
            )));
        }
        let sub = RedshiftEventSubscription {
            subscription_name: subscription_name.clone(),
            sns_topic_arn,
            source_type,
            source_ids,
            event_categories,
            severity,
            enabled,
            tags,
            status: "active".to_string(),
            creation_time: chrono::Utc::now().to_rfc3339(),
            customer_aws_id: account_id.to_string(),
        };
        self.event_subscriptions
            .insert(subscription_name, sub.clone());
        Ok(sub)
    }

    pub fn describe_event_subscriptions(
        &self,
        subscription_name: Option<&str>,
    ) -> Result<Vec<RedshiftEventSubscription>, RedshiftError> {
        if let Some(name) = subscription_name {
            match self.event_subscriptions.get(name) {
                Some(sub) => Ok(vec![sub.clone()]),
                None => Err(RedshiftError::SubscriptionNotFoundFault(format!(
                    "Subscription '{name}' not found"
                ))),
            }
        } else {
            Ok(self.event_subscriptions.values().cloned().collect())
        }
    }

    pub fn delete_event_subscription(
        &mut self,
        subscription_name: &str,
    ) -> Result<(), RedshiftError> {
        self.event_subscriptions
            .remove(subscription_name)
            .ok_or_else(|| {
                RedshiftError::SubscriptionNotFoundFault(format!(
                    "Subscription '{subscription_name}' not found"
                ))
            })?;
        Ok(())
    }

    // ---- HSM Client Certificates ----

    pub fn create_hsm_client_certificate(
        &mut self,
        identifier: String,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftHsmClientCertificate, RedshiftError> {
        if self.hsm_client_certificates.contains_key(&identifier) {
            return Err(RedshiftError::HsmClientCertificateAlreadyExistsFault(
                format!("HSM client certificate '{identifier}' already exists"),
            ));
        }
        let cert = RedshiftHsmClientCertificate {
            identifier: identifier.clone(),
            public_key: format!(
                "-----BEGIN CERTIFICATE-----\nMIIMock{identifier}\n-----END CERTIFICATE-----"
            ),
            tags: tags.clone(),
        };
        let arn =
            format!("arn:aws:redshift:{region}:{account_id}:hsmclientcertificate:{identifier}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.hsm_client_certificates
            .insert(identifier, cert.clone());
        Ok(cert)
    }

    pub fn describe_hsm_client_certificates(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<RedshiftHsmClientCertificate>, RedshiftError> {
        if let Some(id) = identifier {
            match self.hsm_client_certificates.get(id) {
                Some(c) => Ok(vec![c.clone()]),
                None => Err(RedshiftError::HsmClientCertificateNotFoundFault(format!(
                    "HSM client certificate '{id}' not found"
                ))),
            }
        } else {
            Ok(self.hsm_client_certificates.values().cloned().collect())
        }
    }

    pub fn delete_hsm_client_certificate(&mut self, identifier: &str) -> Result<(), RedshiftError> {
        self.hsm_client_certificates
            .remove(identifier)
            .ok_or_else(|| {
                RedshiftError::HsmClientCertificateNotFoundFault(format!(
                    "HSM client certificate '{identifier}' not found"
                ))
            })?;
        Ok(())
    }

    // ---- HSM Configurations ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_hsm_configuration(
        &mut self,
        identifier: String,
        description: String,
        hsm_ip_address: String,
        hsm_partition_name: String,
        tags: Vec<(String, String)>,
        region: &str,
        account_id: &str,
    ) -> Result<RedshiftHsmConfiguration, RedshiftError> {
        if self.hsm_configurations.contains_key(&identifier) {
            return Err(RedshiftError::HsmConfigurationAlreadyExistsFault(format!(
                "HSM configuration '{identifier}' already exists"
            )));
        }
        let config = RedshiftHsmConfiguration {
            identifier: identifier.clone(),
            description,
            hsm_ip_address,
            hsm_partition_name,
            tags: tags.clone(),
        };
        let arn = format!("arn:aws:redshift:{region}:{account_id}:hsmconfiguration:{identifier}");
        if !tags.is_empty() {
            self.tags_by_arn.insert(arn, tags);
        }
        self.hsm_configurations.insert(identifier, config.clone());
        Ok(config)
    }

    pub fn describe_hsm_configurations(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<RedshiftHsmConfiguration>, RedshiftError> {
        if let Some(id) = identifier {
            match self.hsm_configurations.get(id) {
                Some(c) => Ok(vec![c.clone()]),
                None => Err(RedshiftError::HsmConfigurationNotFoundFault(format!(
                    "HSM configuration '{id}' not found"
                ))),
            }
        } else {
            Ok(self.hsm_configurations.values().cloned().collect())
        }
    }

    pub fn delete_hsm_configuration(&mut self, identifier: &str) -> Result<(), RedshiftError> {
        self.hsm_configurations.remove(identifier).ok_or_else(|| {
            RedshiftError::HsmConfigurationNotFoundFault(format!(
                "HSM configuration '{identifier}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Authentication Profiles ----

    pub fn create_authentication_profile(
        &mut self,
        name: String,
        content: String,
    ) -> Result<RedshiftAuthenticationProfile, RedshiftError> {
        if self.authentication_profiles.contains_key(&name) {
            return Err(RedshiftError::AuthenticationProfileAlreadyExistsFault(
                format!("Authentication profile '{name}' already exists"),
            ));
        }
        let profile = RedshiftAuthenticationProfile {
            name: name.clone(),
            content,
        };
        self.authentication_profiles.insert(name, profile.clone());
        Ok(profile)
    }

    pub fn describe_authentication_profiles(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<RedshiftAuthenticationProfile>, RedshiftError> {
        if let Some(n) = name {
            match self.authentication_profiles.get(n) {
                Some(p) => Ok(vec![p.clone()]),
                None => Err(RedshiftError::AuthenticationProfileNotFoundFault(format!(
                    "Authentication profile '{n}' not found"
                ))),
            }
        } else {
            Ok(self.authentication_profiles.values().cloned().collect())
        }
    }

    pub fn modify_authentication_profile(
        &mut self,
        name: &str,
        content: String,
    ) -> Result<RedshiftAuthenticationProfile, RedshiftError> {
        let profile = self.authentication_profiles.get_mut(name).ok_or_else(|| {
            RedshiftError::AuthenticationProfileNotFoundFault(format!(
                "Authentication profile '{name}' not found"
            ))
        })?;
        profile.content = content;
        Ok(profile.clone())
    }

    pub fn delete_authentication_profile(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.authentication_profiles.remove(name).ok_or_else(|| {
            RedshiftError::AuthenticationProfileNotFoundFault(format!(
                "Authentication profile '{name}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Usage Limits ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_usage_limit(
        &mut self,
        cluster_identifier: String,
        feature_type: String,
        limit_type: String,
        amount: i64,
        period: Option<String>,
        breach_action: Option<String>,
        tags: Vec<(String, String)>,
    ) -> Result<RedshiftUsageLimit, RedshiftError> {
        if !self.clusters.contains_key(&cluster_identifier) {
            return Err(RedshiftError::ClusterNotFound(format!(
                "Cluster '{cluster_identifier}' not found"
            )));
        }
        let id = uuid::Uuid::new_v4().to_string();
        let limit = RedshiftUsageLimit {
            usage_limit_id: id.clone(),
            cluster_identifier,
            feature_type,
            limit_type,
            amount,
            period,
            breach_action,
            tags,
        };
        self.usage_limits.insert(id, limit.clone());
        Ok(limit)
    }

    pub fn describe_usage_limits(
        &self,
        usage_limit_id: Option<&str>,
        cluster_identifier: Option<&str>,
    ) -> Vec<RedshiftUsageLimit> {
        self.usage_limits
            .values()
            .filter(|l| {
                if let Some(id) = usage_limit_id {
                    if l.usage_limit_id != id {
                        return false;
                    }
                }
                if let Some(cid) = cluster_identifier {
                    if l.cluster_identifier != cid {
                        return false;
                    }
                }
                true
            })
            .cloned()
            .collect()
    }

    pub fn modify_usage_limit(
        &mut self,
        usage_limit_id: &str,
        amount: Option<i64>,
        breach_action: Option<String>,
    ) -> Result<RedshiftUsageLimit, RedshiftError> {
        let limit = self.usage_limits.get_mut(usage_limit_id).ok_or_else(|| {
            RedshiftError::UsageLimitNotFoundFault(format!(
                "Usage limit '{usage_limit_id}' not found"
            ))
        })?;
        if let Some(a) = amount {
            limit.amount = a;
        }
        if let Some(ba) = breach_action {
            limit.breach_action = Some(ba);
        }
        Ok(limit.clone())
    }

    pub fn delete_usage_limit(&mut self, usage_limit_id: &str) -> Result<(), RedshiftError> {
        self.usage_limits.remove(usage_limit_id).ok_or_else(|| {
            RedshiftError::UsageLimitNotFoundFault(format!(
                "Usage limit '{usage_limit_id}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Snapshot Schedules ----

    pub fn create_snapshot_schedule(
        &mut self,
        schedule_identifier: Option<String>,
        schedule_definitions: Vec<String>,
        schedule_description: Option<String>,
        tags: Vec<(String, String)>,
    ) -> Result<RedshiftSnapshotSchedule, RedshiftError> {
        let id = schedule_identifier.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        if self.snapshot_schedules.contains_key(&id) {
            return Err(RedshiftError::SnapshotScheduleAlreadyExistsFault(format!(
                "Snapshot schedule '{id}' already exists"
            )));
        }
        let schedule = RedshiftSnapshotSchedule {
            schedule_identifier: id.clone(),
            schedule_definitions,
            schedule_description,
            tags,
        };
        self.snapshot_schedules.insert(id, schedule.clone());
        Ok(schedule)
    }

    pub fn describe_snapshot_schedules(
        &self,
        schedule_identifier: Option<&str>,
    ) -> Vec<RedshiftSnapshotSchedule> {
        if let Some(id) = schedule_identifier {
            self.snapshot_schedules
                .get(id)
                .cloned()
                .into_iter()
                .collect()
        } else {
            self.snapshot_schedules.values().cloned().collect()
        }
    }

    pub fn modify_snapshot_schedule(
        &mut self,
        schedule_identifier: &str,
        schedule_definitions: Vec<String>,
    ) -> Result<RedshiftSnapshotSchedule, RedshiftError> {
        let schedule = self
            .snapshot_schedules
            .get_mut(schedule_identifier)
            .ok_or_else(|| {
                RedshiftError::SnapshotScheduleNotFoundFault(format!(
                    "Snapshot schedule '{schedule_identifier}' not found"
                ))
            })?;
        schedule.schedule_definitions = schedule_definitions;
        Ok(schedule.clone())
    }

    pub fn delete_snapshot_schedule(
        &mut self,
        schedule_identifier: &str,
    ) -> Result<(), RedshiftError> {
        self.snapshot_schedules
            .remove(schedule_identifier)
            .ok_or_else(|| {
                RedshiftError::SnapshotScheduleNotFoundFault(format!(
                    "Snapshot schedule '{schedule_identifier}' not found"
                ))
            })?;
        Ok(())
    }

    // ---- Scheduled Actions ----

    #[allow(clippy::too_many_arguments)]
    pub fn create_scheduled_action(
        &mut self,
        name: String,
        schedule: Option<String>,
        iam_role: Option<String>,
        description: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<RedshiftScheduledAction, RedshiftError> {
        if self.scheduled_actions.contains_key(&name) {
            return Err(RedshiftError::ScheduledActionAlreadyExistsFault(format!(
                "Scheduled action '{name}' already exists"
            )));
        }
        let action = RedshiftScheduledAction {
            name: name.clone(),
            schedule,
            iam_role,
            description,
            start_time,
            end_time,
            state: "ACTIVE".to_string(),
        };
        self.scheduled_actions.insert(name, action.clone());
        Ok(action)
    }

    pub fn describe_scheduled_actions(&self, name: Option<&str>) -> Vec<RedshiftScheduledAction> {
        if let Some(n) = name {
            self.scheduled_actions.get(n).cloned().into_iter().collect()
        } else {
            self.scheduled_actions.values().cloned().collect()
        }
    }

    pub fn modify_scheduled_action(
        &mut self,
        name: &str,
        schedule: Option<String>,
        iam_role: Option<String>,
        description: Option<String>,
        start_time: Option<String>,
        end_time: Option<String>,
    ) -> Result<RedshiftScheduledAction, RedshiftError> {
        let action = self.scheduled_actions.get_mut(name).ok_or_else(|| {
            RedshiftError::ScheduledActionNotFoundFault(format!(
                "Scheduled action '{name}' not found"
            ))
        })?;
        if let Some(s) = schedule {
            action.schedule = Some(s);
        }
        if let Some(r) = iam_role {
            action.iam_role = Some(r);
        }
        if let Some(d) = description {
            action.description = Some(d);
        }
        if let Some(st) = start_time {
            action.start_time = Some(st);
        }
        if let Some(et) = end_time {
            action.end_time = Some(et);
        }
        Ok(action.clone())
    }

    pub fn delete_scheduled_action(&mut self, name: &str) -> Result<(), RedshiftError> {
        self.scheduled_actions.remove(name).ok_or_else(|| {
            RedshiftError::ScheduledActionNotFoundFault(format!(
                "Scheduled action '{name}' not found"
            ))
        })?;
        Ok(())
    }

    // ---- Snapshot access ----

    pub fn authorize_snapshot_access(
        &mut self,
        snapshot_identifier: &str,
        account_with_restore_access: String,
    ) -> Result<RedshiftSnapshot, RedshiftError> {
        self.snapshots
            .get(snapshot_identifier)
            .cloned()
            .ok_or_else(|| {
                RedshiftError::ClusterSnapshotNotFound(format!(
                    "Snapshot '{snapshot_identifier}' not found"
                ))
            })
            .map(|mut snap| {
                // Record the account in tags as a simple mechanism
                let key = format!("RestoreAccess:{account_with_restore_access}");
                snap.tags.retain(|(k, _)| k != &key);
                snap.tags.push((key, "authorized".to_string()));
                // update stored snapshot
                self.snapshots
                    .insert(snapshot_identifier.to_string(), snap.clone());
                snap
            })
    }

    pub fn revoke_snapshot_access(
        &mut self,
        snapshot_identifier: &str,
    ) -> Result<RedshiftSnapshot, RedshiftError> {
        self.snapshots
            .get(snapshot_identifier)
            .cloned()
            .ok_or_else(|| {
                RedshiftError::ClusterSnapshotNotFound(format!(
                    "Snapshot '{snapshot_identifier}' not found"
                ))
            })
    }

    // ---- Cluster IAM roles ----

    pub fn modify_cluster_iam_roles(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        self.clusters
            .get(cluster_identifier)
            .cloned()
            .ok_or_else(|| {
                RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
            })
    }

    // ---- Cluster maintenance ----

    pub fn modify_cluster_maintenance(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        self.clusters
            .get(cluster_identifier)
            .cloned()
            .ok_or_else(|| {
                RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
            })
    }

    // ---- Rotate encryption key ----

    pub fn rotate_encryption_key(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        let cluster = self.clusters.get_mut(cluster_identifier).ok_or_else(|| {
            RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
        })?;
        cluster.encrypted = true;
        Ok(cluster.clone())
    }

    // ---- Failover primary compute ----

    pub fn failover_primary_compute(
        &mut self,
        cluster_identifier: &str,
    ) -> Result<RedshiftCluster, RedshiftError> {
        self.clusters
            .get(cluster_identifier)
            .cloned()
            .ok_or_else(|| {
                RedshiftError::ClusterNotFound(format!("Cluster '{cluster_identifier}' not found"))
            })
    }

    // ---- Modify Event Subscription ----

    #[allow(clippy::too_many_arguments)]
    pub fn modify_event_subscription(
        &mut self,
        subscription_name: &str,
        sns_topic_arn: Option<String>,
        source_type: Option<String>,
        source_ids: Option<Vec<String>>,
        event_categories: Option<Vec<String>>,
        severity: Option<String>,
        enabled: Option<bool>,
    ) -> Result<RedshiftEventSubscription, RedshiftError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| {
                RedshiftError::SubscriptionNotFoundFault(format!(
                    "Subscription '{subscription_name}' not found"
                ))
            })?;
        if let Some(arn) = sns_topic_arn {
            sub.sns_topic_arn = arn;
        }
        if let Some(st) = source_type {
            sub.source_type = Some(st);
        }
        if let Some(ids) = source_ids {
            sub.source_ids = ids;
        }
        if let Some(cats) = event_categories {
            sub.event_categories = cats;
        }
        if let Some(sev) = severity {
            sub.severity = Some(sev);
        }
        if let Some(en) = enabled {
            sub.enabled = en;
        }
        Ok(sub.clone())
    }

    // ---- Resource Policies ----

    pub fn put_resource_policy(
        &mut self,
        resource_arn: String,
        policy: String,
    ) -> RedshiftResourcePolicy {
        let rp = RedshiftResourcePolicy {
            resource_arn: resource_arn.clone(),
            policy,
        };
        self.resource_policies.insert(resource_arn, rp.clone());
        rp
    }

    pub fn get_resource_policy(&self, resource_arn: &str) -> Option<RedshiftResourcePolicy> {
        self.resource_policies.get(resource_arn).cloned()
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) {
        self.resource_policies.remove(resource_arn);
    }

    // ---- Partner Integrations ----

    pub fn add_partner(
        &mut self,
        cluster_identifier: String,
        database_name: String,
        partner_name: String,
    ) -> Result<RedshiftPartnerIntegration, RedshiftError> {
        if !self.clusters.contains_key(&cluster_identifier) {
            return Err(RedshiftError::ClusterNotFound(format!(
                "Cluster '{cluster_identifier}' not found"
            )));
        }
        let pi = RedshiftPartnerIntegration {
            cluster_identifier: cluster_identifier.clone(),
            database_name: database_name.clone(),
            partner_name: partner_name.clone(),
            status: "Active".to_string(),
            status_message: None,
        };
        self.partner_integrations.push(pi.clone());
        Ok(pi)
    }

    pub fn describe_partners(
        &self,
        cluster_identifier: &str,
        database_name: Option<&str>,
        partner_name: Option<&str>,
    ) -> Vec<RedshiftPartnerIntegration> {
        self.partner_integrations
            .iter()
            .filter(|p| {
                p.cluster_identifier == cluster_identifier
                    && database_name.is_none_or(|d| p.database_name == d)
                    && partner_name.is_none_or(|n| p.partner_name == n)
            })
            .cloned()
            .collect()
    }

    pub fn delete_partner(
        &mut self,
        cluster_identifier: &str,
        database_name: &str,
        partner_name: &str,
    ) -> Result<RedshiftPartnerIntegration, RedshiftError> {
        let pos = self.partner_integrations.iter().position(|p| {
            p.cluster_identifier == cluster_identifier
                && p.database_name == database_name
                && p.partner_name == partner_name
        });
        match pos {
            Some(i) => Ok(self.partner_integrations.remove(i)),
            None => Err(RedshiftError::PartnerNotFoundFault(format!(
                "Partner '{partner_name}' not found"
            ))),
        }
    }

    pub fn update_partner_status(
        &mut self,
        cluster_identifier: &str,
        database_name: &str,
        partner_name: &str,
        status: String,
        status_message: Option<String>,
    ) -> Result<RedshiftPartnerIntegration, RedshiftError> {
        let pi = self.partner_integrations.iter_mut().find(|p| {
            p.cluster_identifier == cluster_identifier
                && p.database_name == database_name
                && p.partner_name == partner_name
        });
        match pi {
            Some(p) => {
                p.status = status;
                p.status_message = status_message;
                Ok(p.clone())
            }
            None => Err(RedshiftError::PartnerNotFoundFault(format!(
                "Partner '{partner_name}' not found"
            ))),
        }
    }
    // ---- AQUA Configuration ----

    pub fn get_aqua_configuration(&self, cluster_identifier: &str) -> (String, String) {
        self.aqua_configurations
            .get(cluster_identifier)
            .cloned()
            .unwrap_or_else(|| ("auto".to_string(), "disabled".to_string()))
    }

    pub fn set_aqua_configuration(
        &mut self,
        cluster_identifier: &str,
        aqua_configuration_status: &str,
    ) -> (String, String) {
        let aqua_status = if aqua_configuration_status == "enabled" {
            "applying"
        } else {
            "disabled"
        };
        let entry = (
            aqua_configuration_status.to_string(),
            aqua_status.to_string(),
        );
        self.aqua_configurations
            .insert(cluster_identifier.to_string(), entry.clone());
        entry
    }

    // ---- Reserved Nodes ----

    pub fn list_reserved_nodes(&self) -> Vec<&RedshiftReservedNode> {
        self.reserved_nodes.values().collect()
    }

    // ---- Table Restore Status ----

    pub fn list_table_restore_statuses(
        &self,
        cluster_identifier: Option<&str>,
    ) -> Vec<&RedshiftTableRestoreStatus> {
        self.table_restore_statuses
            .values()
            .filter(|s| {
                if let Some(cid) = cluster_identifier {
                    s.cluster_identifier == cid
                } else {
                    true
                }
            })
            .collect()
    }
}

fn extract_resource_type_from_arn(arn: &str) -> &str {
    // arn:aws:redshift:{region}:{account}:{type}:{name}
    let parts: Vec<&str> = arn.splitn(7, ':').collect();
    if parts.len() >= 6 { parts[5] } else { "" }
}

fn default_parameters(family: &str) -> Vec<RedshiftParameter> {
    let _ = family;
    vec![
        RedshiftParameter {
            name: "enable_user_activity_logging".to_string(),
            value: "false".to_string(),
            description: "Enables user activity logging".to_string(),
            is_modifiable: true,
            apply_type: "static".to_string(),
        },
        RedshiftParameter {
            name: "max_cursor_result_set_size".to_string(),
            value: "0".to_string(),
            description: "Maximum result set size for cursors".to_string(),
            is_modifiable: true,
            apply_type: "static".to_string(),
        },
        RedshiftParameter {
            name: "query_group".to_string(),
            value: "default".to_string(),
            description: "Query group".to_string(),
            is_modifiable: true,
            apply_type: "dynamic".to_string(),
        },
    ]
}
