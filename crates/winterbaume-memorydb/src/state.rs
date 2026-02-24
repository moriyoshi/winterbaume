use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

/// In-memory state for the MemoryDB service.
#[derive(Debug, Default)]
pub struct MemoryDbState {
    /// Clusters keyed by cluster name.
    pub clusters: HashMap<String, Cluster>,
    /// Snapshots keyed by snapshot name.
    pub snapshots: HashMap<String, SnapshotData>,
    /// Subnet groups keyed by subnet group name.
    pub subnet_groups: HashMap<String, SubnetGroupData>,
    /// ACLs keyed by ACL name.
    pub acls: HashMap<String, AclData>,
    /// Tags keyed by resource ARN.
    pub tags: TagsMap,
}

/// Error type for MemoryDB operations.
#[derive(Debug, Error)]
pub enum MemoryDbError {
    #[error("Cluster {0} already exists.")]
    ClusterAlreadyExists(String),
    #[error("Cluster {0} not found.")]
    ClusterNotFound(String),
    #[error("Snapshot {0} already exists.")]
    SnapshotAlreadyExists(String),
    #[error("Snapshot {0} not found.")]
    SnapshotNotFound(String),
    #[error("Subnet group {0} already exists.")]
    SubnetGroupAlreadyExists(String),
    #[error("Subnet group {0} not found.")]
    SubnetGroupNotFound(String),
    #[error("Subnet group {0} is in use and cannot be deleted.")]
    SubnetGroupInUse(String),
    #[error("The default subnet group cannot be deleted.")]
    DefaultSubnetGroupCannotBeDeleted,
    #[error("Tag with key {0} not found.")]
    TagNotFound(String),
    #[error("The resource with ARN {0} was not found.")]
    InvalidArn(String),
}

impl MemoryDbState {
    pub fn create_cluster(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        node_type: &str,
        acl_name: &str,
        description: Option<&str>,
        num_shards: Option<i32>,
        num_replicas_per_shard: Option<i32>,
        subnet_group_name: Option<&str>,
        security_group_ids: Vec<String>,
        engine_version: Option<&str>,
        maintenance_window: Option<&str>,
        snapshot_retention_limit: Option<i32>,
        snapshot_window: Option<&str>,
        parameter_group_name: Option<&str>,
        tls_enabled: Option<bool>,
        auto_minor_version_upgrade: Option<bool>,
        tags: Vec<TagData>,
    ) -> Result<&Cluster, MemoryDbError> {
        if self.clusters.contains_key(name) {
            return Err(MemoryDbError::ClusterAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:memorydb:{region}:{account_id}:cluster/{name}");
        let cluster = Cluster {
            name: name.to_string(),
            arn: arn.clone(),
            status: "available".to_string(),
            node_type: node_type.to_string(),
            num_shards: num_shards.unwrap_or(1),
            num_replicas_per_shard: num_replicas_per_shard.unwrap_or(1),
            description: description.unwrap_or("").to_string(),
            engine: "redis".to_string(),
            engine_version: engine_version.unwrap_or("7.1").to_string(),
            subnet_group_name: subnet_group_name.unwrap_or("default").to_string(),
            security_group_ids,
            maintenance_window: maintenance_window
                .unwrap_or("wed:03:00-wed:04:00")
                .to_string(),
            snapshot_retention_limit: snapshot_retention_limit.unwrap_or(0),
            snapshot_window: snapshot_window.unwrap_or("05:00-06:00").to_string(),
            acl_name: acl_name.to_string(),
            parameter_group_name: parameter_group_name
                .unwrap_or("default.memorydb-redis7")
                .to_string(),
            tls_enabled: tls_enabled.unwrap_or(true),
            auto_minor_version_upgrade: auto_minor_version_upgrade.unwrap_or(true),
            creation_time: Utc::now(),
        };

        self.clusters.insert(name.to_string(), cluster);
        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }
        Ok(self.clusters.get(name).unwrap())
    }

    pub fn describe_clusters(
        &self,
        cluster_name: Option<&str>,
    ) -> Result<Vec<&Cluster>, MemoryDbError> {
        match cluster_name {
            Some(name) => {
                if let Some(cluster) = self.clusters.get(name) {
                    Ok(vec![cluster])
                } else {
                    Err(MemoryDbError::ClusterNotFound(name.to_string()))
                }
            }
            None => Ok(self.clusters.values().collect()),
        }
    }

    pub fn delete_cluster(
        &mut self,
        name: &str,
        final_snapshot_name: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<Cluster, MemoryDbError> {
        if !self.clusters.contains_key(name) {
            return Err(MemoryDbError::ClusterNotFound(name.to_string()));
        }

        // Create final snapshot if requested (before removing cluster)
        if let Some(snap_name) = final_snapshot_name {
            self.create_snapshot(account_id, region, snap_name, name, None, vec![])?;
        }

        let mut cluster = self.clusters.remove(name).unwrap();
        cluster.status = "deleting".to_string();
        Ok(cluster)
    }

    pub fn update_cluster(
        &mut self,
        name: &str,
        description: Option<&str>,
        security_group_ids: Option<Vec<String>>,
        maintenance_window: Option<&str>,
        snapshot_retention_limit: Option<i32>,
        snapshot_window: Option<&str>,
        parameter_group_name: Option<&str>,
        engine_version: Option<&str>,
        num_shards: Option<i32>,
        num_replicas_per_shard: Option<i32>,
        node_type: Option<&str>,
        acl_name: Option<&str>,
    ) -> Result<&Cluster, MemoryDbError> {
        let cluster = self
            .clusters
            .get_mut(name)
            .ok_or_else(|| MemoryDbError::ClusterNotFound(name.to_string()))?;

        if let Some(desc) = description {
            cluster.description = desc.to_string();
        }
        if let Some(sgs) = security_group_ids {
            cluster.security_group_ids = sgs;
        }
        if let Some(mw) = maintenance_window {
            cluster.maintenance_window = mw.to_string();
        }
        if let Some(srl) = snapshot_retention_limit {
            cluster.snapshot_retention_limit = srl;
        }
        if let Some(sw) = snapshot_window {
            cluster.snapshot_window = sw.to_string();
        }
        if let Some(pgn) = parameter_group_name {
            cluster.parameter_group_name = pgn.to_string();
        }
        if let Some(ev) = engine_version {
            cluster.engine_version = ev.to_string();
        }
        if let Some(ns) = num_shards {
            cluster.num_shards = ns;
        }
        if let Some(nr) = num_replicas_per_shard {
            cluster.num_replicas_per_shard = nr;
        }
        if let Some(nt) = node_type {
            cluster.node_type = nt.to_string();
        }
        if let Some(acl) = acl_name {
            cluster.acl_name = acl.to_string();
        }

        Ok(self.clusters.get(name).unwrap())
    }

    // --- Snapshot operations ---

    pub fn create_snapshot(
        &mut self,
        account_id: &str,
        region: &str,
        snapshot_name: &str,
        cluster_name: &str,
        kms_key_id: Option<&str>,
        tags: Vec<TagData>,
    ) -> Result<&SnapshotData, MemoryDbError> {
        if self.snapshots.contains_key(snapshot_name) {
            return Err(MemoryDbError::SnapshotAlreadyExists(
                snapshot_name.to_string(),
            ));
        }

        let cluster = self
            .clusters
            .get(cluster_name)
            .ok_or_else(|| MemoryDbError::ClusterNotFound(cluster_name.to_string()))?;

        let arn = format!("arn:aws:memorydb:{region}:{account_id}:snapshot/{snapshot_name}");
        let snapshot = SnapshotData {
            name: snapshot_name.to_string(),
            arn: arn.clone(),
            status: "available".to_string(),
            source: "manual".to_string(),
            cluster_name: cluster.name.clone(),
            cluster_description: cluster.description.clone(),
            cluster_engine: cluster.engine.clone(),
            cluster_engine_version: cluster.engine_version.clone(),
            cluster_node_type: cluster.node_type.clone(),
            cluster_num_shards: cluster.num_shards,
            cluster_subnet_group_name: cluster.subnet_group_name.clone(),
            cluster_snapshot_retention_limit: cluster.snapshot_retention_limit,
            cluster_snapshot_window: cluster.snapshot_window.clone(),
            cluster_maintenance_window: cluster.maintenance_window.clone(),
            cluster_parameter_group_name: cluster.parameter_group_name.clone(),
            kms_key_id: kms_key_id.map(|s| s.to_string()),
        };

        self.snapshots.insert(snapshot_name.to_string(), snapshot);
        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }
        Ok(self.snapshots.get(snapshot_name).unwrap())
    }

    pub fn delete_snapshot(&mut self, snapshot_name: &str) -> Result<SnapshotData, MemoryDbError> {
        match self.snapshots.remove(snapshot_name) {
            Some(mut snapshot) => {
                // Clean up tags
                self.tags.remove(&snapshot.arn);
                snapshot.status = "deleting".to_string();
                Ok(snapshot)
            }
            None => Err(MemoryDbError::SnapshotNotFound(snapshot_name.to_string())),
        }
    }

    pub fn describe_snapshots(
        &self,
        cluster_name: Option<&str>,
        snapshot_name: Option<&str>,
        source: Option<&str>,
    ) -> Result<Vec<&SnapshotData>, MemoryDbError> {
        // When snapshot_name is specified, filter by it (and also by cluster if given)
        if let Some(name) = snapshot_name {
            if let Some(snapshot) = self.snapshots.get(name) {
                // If cluster_name is also given, verify it matches
                if let Some(cn) = cluster_name
                    && snapshot.cluster_name != cn
                {
                    return Err(MemoryDbError::SnapshotNotFound(name.to_string()));
                }
                return Ok(vec![snapshot]);
            } else {
                return Err(MemoryDbError::SnapshotNotFound(name.to_string()));
            }
        }

        let mut results: Vec<&SnapshotData> = self.snapshots.values().collect();

        if let Some(cn) = cluster_name {
            results.retain(|s| s.cluster_name == cn);
        }
        if let Some(src) = source {
            results.retain(|s| s.source == src);
        }

        Ok(results)
    }

    // --- Subnet group operations ---

    pub fn create_subnet_group(
        &mut self,
        account_id: &str,
        region: &str,
        name: &str,
        description: Option<&str>,
        subnet_ids: Vec<String>,
        tags: Vec<TagData>,
    ) -> Result<&SubnetGroupData, MemoryDbError> {
        if self.subnet_groups.contains_key(name) {
            return Err(MemoryDbError::SubnetGroupAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:memorydb:{region}:{account_id}:subnetgroup/{name}");
        let subnet_group = SubnetGroupData {
            name: name.to_string(),
            arn: arn.clone(),
            description: description.unwrap_or("").to_string(),
            vpc_id: format!(
                "vpc-{:012x}",
                name.as_bytes()
                    .iter()
                    .fold(0u64, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u64))
            ),
            subnet_ids,
        };

        self.subnet_groups.insert(name.to_string(), subnet_group);
        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }
        Ok(self.subnet_groups.get(name).unwrap())
    }

    pub fn delete_subnet_group(&mut self, name: &str) -> Result<SubnetGroupData, MemoryDbError> {
        if name == "default" {
            return Err(MemoryDbError::DefaultSubnetGroupCannotBeDeleted);
        }

        if !self.subnet_groups.contains_key(name) {
            return Err(MemoryDbError::SubnetGroupNotFound(name.to_string()));
        }

        // Check if the subnet group is in use by any cluster
        let in_use = self.clusters.values().any(|c| c.subnet_group_name == name);
        if in_use {
            return Err(MemoryDbError::SubnetGroupInUse(name.to_string()));
        }

        let sg = self.subnet_groups.remove(name).unwrap();
        self.tags.remove(&sg.arn);
        Ok(sg)
    }

    pub fn describe_subnet_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&SubnetGroupData>, MemoryDbError> {
        match name {
            Some(n) => {
                if let Some(sg) = self.subnet_groups.get(n) {
                    Ok(vec![sg])
                } else {
                    Err(MemoryDbError::SubnetGroupNotFound(n.to_string()))
                }
            }
            None => Ok(self.subnet_groups.values().collect()),
        }
    }

    // --- Tag operations ---

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<TagData>,
    ) -> Result<Vec<TagData>, MemoryDbError> {
        // Validate that the resource exists
        self.validate_resource_arn(resource_arn)?;

        let existing = self.tags.entry(resource_arn.to_string()).or_default();
        for tag in tags {
            // Update existing tag or add new one
            if let Some(pos) = existing.iter().position(|t| t.key == tag.key) {
                existing[pos] = tag;
            } else {
                existing.push(tag);
            }
        }
        Ok(existing.clone())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<Vec<TagData>, MemoryDbError> {
        self.validate_resource_arn(resource_arn)?;

        let existing = self.tags.entry(resource_arn.to_string()).or_default();

        // Validate that all tag keys exist
        for key in tag_keys {
            if !existing.iter().any(|t| &t.key == key) {
                return Err(MemoryDbError::TagNotFound(key.clone()));
            }
        }

        existing.retain(|t| !tag_keys.contains(&t.key));
        Ok(existing.clone())
    }

    pub fn list_tags(&self, resource_arn: &str) -> Result<Vec<TagData>, MemoryDbError> {
        self.validate_resource_arn(resource_arn)?;

        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }

    fn validate_resource_arn(&self, arn: &str) -> Result<(), MemoryDbError> {
        // Parse the resource type from the ARN to return a type-specific error
        // ARN format: arn:aws:memorydb:region:account:resource-type/name
        if let Some(resource_part) = arn.rsplit(':').next() {
            if resource_part.starts_with("cluster/") {
                let name = resource_part.strip_prefix("cluster/").unwrap_or("");
                let is_cluster = self.clusters.values().any(|c| c.arn == arn);
                if is_cluster {
                    return Ok(());
                }
                return Err(MemoryDbError::ClusterNotFound(name.to_string()));
            } else if resource_part.starts_with("snapshot/") {
                let name = resource_part.strip_prefix("snapshot/").unwrap_or("");
                let is_snapshot = self.snapshots.values().any(|s| s.arn == arn);
                if is_snapshot {
                    return Ok(());
                }
                return Err(MemoryDbError::SnapshotNotFound(name.to_string()));
            } else if resource_part.starts_with("subnetgroup/") {
                let name = resource_part.strip_prefix("subnetgroup/").unwrap_or("");
                let is_subnet_group = self.subnet_groups.values().any(|sg| sg.arn == arn);
                if is_subnet_group {
                    return Ok(());
                }
                return Err(MemoryDbError::SubnetGroupNotFound(name.to_string()));
            }
        }

        // Check all resources as fallback
        let is_cluster = self.clusters.values().any(|c| c.arn == arn);
        let is_snapshot = self.snapshots.values().any(|s| s.arn == arn);
        let is_subnet_group = self.subnet_groups.values().any(|sg| sg.arn == arn);

        if is_cluster || is_snapshot || is_subnet_group {
            Ok(())
        } else {
            Err(MemoryDbError::InvalidArn(arn.to_string()))
        }
    }
}
