use std::collections::HashMap;

use crate::types::{
    DbCluster, DbClusterEndpoint, DbClusterMember, DbClusterParameterGroup, DbClusterSnapshot,
    DbInstance, DbParameterGroup, DbSubnetGroup, EventSubscription, GlobalCluster, Parameter,
    ServerlessV2ScalingConfiguration, SnapshotAttribute, Tag,
};

/// In-memory state for the Neptune service.
#[derive(Debug, Default)]
pub struct NeptuneState {
    pub db_clusters: HashMap<String, DbCluster>,
    pub db_instances: HashMap<String, DbInstance>,
    pub db_subnet_groups: HashMap<String, DbSubnetGroup>,
    pub db_parameter_groups: HashMap<String, DbParameterGroup>,
    pub db_cluster_parameter_groups: HashMap<String, DbClusterParameterGroup>,
    pub db_cluster_snapshots: HashMap<String, DbClusterSnapshot>,
    pub global_clusters: HashMap<String, GlobalCluster>,
    pub db_cluster_endpoints: HashMap<String, DbClusterEndpoint>,
    pub event_subscriptions: HashMap<String, EventSubscription>,
    /// Snapshot sharing attributes keyed by snapshot identifier.
    pub snapshot_attributes: HashMap<String, Vec<SnapshotAttribute>>,
    /// Tags keyed by resource ARN.
    pub resource_tags: HashMap<String, Vec<Tag>>,
}

/// Error type for Neptune operations.
#[derive(Debug, thiserror::Error)]
pub enum NeptuneError {
    #[error("{resource_type} already exists: {name}")]
    AlreadyExists { resource_type: String, name: String },
    #[error("{resource_type} not found: {name}")]
    NotFound { resource_type: String, name: String },
    #[error("{message}")]
    InvalidParameter { message: String },
}

impl NeptuneState {
    // -------------------------------------------------------------------------
    // DB Clusters
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_db_cluster(
        &mut self,
        identifier: String,
        engine: String,
        engine_version: Option<String>,
        master_username: Option<String>,
        database_name: Option<String>,
        db_subnet_group_name: Option<String>,
        vpc_security_group_ids: Vec<String>,
        availability_zones: Vec<String>,
        storage_encrypted: bool,
        kms_key_id: Option<String>,
        db_cluster_parameter_group: Option<String>,
        engine_mode: Option<String>,
        backup_retention_period: i32,
        deletion_protection: bool,
        serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbCluster, NeptuneError> {
        if self.db_clusters.contains_key(&identifier) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBCluster".to_string(),
                name: identifier.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster:{identifier}");
        let endpoint = Some(format!(
            "{identifier}.cluster-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{identifier}.cluster-ro-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let cluster_create_time = Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let cluster = DbCluster {
            identifier: identifier.clone(),
            engine,
            engine_version,
            status: "available".to_string(),
            endpoint,
            reader_endpoint,
            port: Some(8182),
            master_username,
            database_name,
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zones,
            arn: arn.clone(),
            tags: tags.clone(),
            cluster_create_time,
            multi_az: false,
            storage_encrypted,
            kms_key_id,
            db_cluster_parameter_group,
            engine_mode,
            copy_tags_to_snapshot: false,
            deletion_protection,
            backup_retention_period,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration,
        };
        self.db_clusters.insert(identifier, cluster.clone());
        self.resource_tags.insert(arn, tags);
        Ok(cluster)
    }

    pub fn describe_db_clusters(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<DbCluster>, NeptuneError> {
        match identifier {
            Some(id) => self
                .db_clusters
                .get(id)
                .map(|c| vec![c.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBCluster".to_string(),
                    name: id.to_string(),
                }),
            None => {
                let mut clusters: Vec<DbCluster> = self.db_clusters.values().cloned().collect();
                clusters.sort_by(|a, b| a.identifier.cmp(&b.identifier));
                Ok(clusters)
            }
        }
    }

    pub fn delete_db_cluster(&mut self, identifier: &str) -> Result<DbCluster, NeptuneError> {
        self.db_clusters
            .remove(identifier)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: identifier.to_string(),
            })
    }

    pub fn modify_db_cluster(
        &mut self,
        identifier: &str,
        engine_version: Option<String>,
        db_cluster_parameter_group: Option<String>,
        backup_retention_period: Option<i32>,
        deletion_protection: Option<bool>,
        master_user_password: Option<String>,
        serverless_v2_scaling_configuration: Option<ServerlessV2ScalingConfiguration>,
    ) -> Result<DbCluster, NeptuneError> {
        let cluster =
            self.db_clusters
                .get_mut(identifier)
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBCluster".to_string(),
                    name: identifier.to_string(),
                })?;
        if let Some(v) = engine_version {
            cluster.engine_version = Some(v);
        }
        if let Some(v) = db_cluster_parameter_group {
            cluster.db_cluster_parameter_group = Some(v);
        }
        if let Some(v) = backup_retention_period {
            cluster.backup_retention_period = v;
        }
        if let Some(v) = deletion_protection {
            cluster.deletion_protection = v;
        }
        if let Some(v) = serverless_v2_scaling_configuration {
            cluster.serverless_v2_scaling_configuration = Some(v);
        }
        let _ = master_user_password; // accepted but not stored
        Ok(cluster.clone())
    }

    // -------------------------------------------------------------------------
    // DB Instances
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_db_instance(
        &mut self,
        identifier: String,
        db_instance_class: String,
        engine: String,
        engine_version: Option<String>,
        db_subnet_group_name: Option<String>,
        vpc_security_group_ids: Vec<String>,
        availability_zone: Option<String>,
        auto_minor_version_upgrade: bool,
        backup_retention_period: i32,
        db_cluster_identifier: Option<String>,
        storage_encrypted: bool,
        kms_key_id: Option<String>,
        publicly_accessible: bool,
        deletion_protection: bool,
        db_parameter_group_names: Vec<String>,
        multi_az: bool,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, NeptuneError> {
        if self.db_instances.contains_key(&identifier) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBInstance".to_string(),
                name: identifier.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:db:{identifier}");
        let endpoint_address = Some(format!(
            "{identifier}.{account_id}.{region}.neptune.amazonaws.com"
        ));
        let instance_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let inst = DbInstance {
            identifier: identifier.clone(),
            db_instance_class,
            engine,
            engine_version: engine_version.unwrap_or_else(|| "1.2.1.0".to_string()),
            status: "available".to_string(),
            endpoint_address,
            port: Some(8182),
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zone,
            auto_minor_version_upgrade,
            backup_retention_period,
            db_cluster_identifier,
            arn: arn.clone(),
            tags: tags.clone(),
            instance_create_time,
            storage_encrypted,
            kms_key_id,
            publicly_accessible,
            deletion_protection,
            db_parameter_group_names,
            multi_az,
        };
        // If instance belongs to a cluster, add as member
        if let Some(ref cluster_id) = inst.db_cluster_identifier {
            if let Some(cluster) = self.db_clusters.get_mut(cluster_id) {
                cluster.members.push(DbClusterMember {
                    db_instance_identifier: identifier.clone(),
                    is_cluster_writer: cluster.members.is_empty(),
                    db_cluster_parameter_group_status: "in-sync".to_string(),
                    promotion_tier: None,
                });
            }
        }
        self.db_instances.insert(identifier, inst.clone());
        self.resource_tags.insert(arn, tags);
        Ok(inst)
    }

    pub fn describe_db_instances(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<DbInstance>, NeptuneError> {
        match identifier {
            Some(id) => self
                .db_instances
                .get(id)
                .map(|i| vec![i.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBInstance".to_string(),
                    name: id.to_string(),
                }),
            None => {
                let mut instances: Vec<DbInstance> = self.db_instances.values().cloned().collect();
                instances.sort_by(|a, b| a.identifier.cmp(&b.identifier));
                Ok(instances)
            }
        }
    }

    pub fn delete_db_instance(&mut self, identifier: &str) -> Result<DbInstance, NeptuneError> {
        let inst = self
            .db_instances
            .remove(identifier)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBInstance".to_string(),
                name: identifier.to_string(),
            })?;
        // Remove from cluster members
        if let Some(ref cluster_id) = inst.db_cluster_identifier {
            if let Some(cluster) = self.db_clusters.get_mut(cluster_id) {
                cluster
                    .members
                    .retain(|m| m.db_instance_identifier != identifier);
            }
        }
        Ok(inst)
    }

    pub fn modify_db_instance(
        &mut self,
        identifier: &str,
        db_instance_class: Option<String>,
        engine_version: Option<String>,
        deletion_protection: Option<bool>,
        auto_minor_version_upgrade: Option<bool>,
    ) -> Result<DbInstance, NeptuneError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBInstance".to_string(),
                name: identifier.to_string(),
            })?;
        if let Some(v) = db_instance_class {
            inst.db_instance_class = v;
        }
        if let Some(v) = engine_version {
            inst.engine_version = v;
        }
        if let Some(v) = deletion_protection {
            inst.deletion_protection = v;
        }
        if let Some(v) = auto_minor_version_upgrade {
            inst.auto_minor_version_upgrade = v;
        }
        Ok(inst.clone())
    }

    // -------------------------------------------------------------------------
    // DB Subnet Groups
    // -------------------------------------------------------------------------

    pub fn create_db_subnet_group(
        &mut self,
        name: String,
        description: String,
        subnet_ids: Vec<String>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbSubnetGroup, NeptuneError> {
        if self.db_subnet_groups.contains_key(&name) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBSubnetGroup".to_string(),
                name: name.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:subgrp:{name}");
        let sg = DbSubnetGroup {
            name: name.clone(),
            description,
            vpc_id: None,
            subnet_ids,
            status: "Complete".to_string(),
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.db_subnet_groups.insert(name, sg.clone());
        self.resource_tags.insert(arn, tags);
        Ok(sg)
    }

    pub fn describe_db_subnet_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<DbSubnetGroup>, NeptuneError> {
        match name {
            Some(n) => self
                .db_subnet_groups
                .get(n)
                .map(|sg| vec![sg.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBSubnetGroup".to_string(),
                    name: n.to_string(),
                }),
            None => {
                let mut groups: Vec<DbSubnetGroup> =
                    self.db_subnet_groups.values().cloned().collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                Ok(groups)
            }
        }
    }

    pub fn delete_db_subnet_group(&mut self, name: &str) -> Result<(), NeptuneError> {
        self.db_subnet_groups
            .remove(name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBSubnetGroup".to_string(),
                name: name.to_string(),
            })?;
        Ok(())
    }

    pub fn modify_db_subnet_group(
        &mut self,
        name: &str,
        description: Option<String>,
        subnet_ids: Option<Vec<String>>,
    ) -> Result<DbSubnetGroup, NeptuneError> {
        let sg = self
            .db_subnet_groups
            .get_mut(name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBSubnetGroup".to_string(),
                name: name.to_string(),
            })?;
        if let Some(v) = description {
            sg.description = v;
        }
        if let Some(v) = subnet_ids {
            sg.subnet_ids = v;
        }
        Ok(sg.clone())
    }

    // -------------------------------------------------------------------------
    // DB Parameter Groups
    // -------------------------------------------------------------------------

    pub fn create_db_parameter_group(
        &mut self,
        name: String,
        family: String,
        description: String,
        parameters: Vec<Parameter>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbParameterGroup, NeptuneError> {
        if self.db_parameter_groups.contains_key(&name) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBParameterGroup".to_string(),
                name: name.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:pg:{name}");
        let pg = DbParameterGroup {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags: tags.clone(),
            parameters,
        };
        self.db_parameter_groups.insert(name, pg.clone());
        self.resource_tags.insert(arn, tags);
        Ok(pg)
    }

    pub fn describe_db_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<DbParameterGroup>, NeptuneError> {
        match name {
            Some(n) => self
                .db_parameter_groups
                .get(n)
                .map(|pg| vec![pg.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBParameterGroup".to_string(),
                    name: n.to_string(),
                }),
            None => {
                let mut groups: Vec<DbParameterGroup> =
                    self.db_parameter_groups.values().cloned().collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                Ok(groups)
            }
        }
    }

    pub fn delete_db_parameter_group(&mut self, name: &str) -> Result<(), NeptuneError> {
        self.db_parameter_groups
            .remove(name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBParameterGroup".to_string(),
                name: name.to_string(),
            })?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // DB Cluster Parameter Groups
    // -------------------------------------------------------------------------

    pub fn create_db_cluster_parameter_group(
        &mut self,
        name: String,
        family: String,
        description: String,
        parameters: Vec<Parameter>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterParameterGroup, NeptuneError> {
        if self.db_cluster_parameter_groups.contains_key(&name) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBClusterParameterGroup".to_string(),
                name: name.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster-pg:{name}");
        let pg = DbClusterParameterGroup {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags: tags.clone(),
            parameters,
        };
        self.db_cluster_parameter_groups.insert(name, pg.clone());
        self.resource_tags.insert(arn, tags);
        Ok(pg)
    }

    pub fn describe_db_cluster_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<DbClusterParameterGroup>, NeptuneError> {
        match name {
            Some(n) => self
                .db_cluster_parameter_groups
                .get(n)
                .map(|pg| vec![pg.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "DBClusterParameterGroup".to_string(),
                    name: n.to_string(),
                }),
            None => {
                let mut groups: Vec<DbClusterParameterGroup> =
                    self.db_cluster_parameter_groups.values().cloned().collect();
                groups.sort_by(|a, b| a.name.cmp(&b.name));
                Ok(groups)
            }
        }
    }

    pub fn delete_db_cluster_parameter_group(&mut self, name: &str) -> Result<(), NeptuneError> {
        self.db_cluster_parameter_groups
            .remove(name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBClusterParameterGroup".to_string(),
                name: name.to_string(),
            })?;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // DB Cluster Snapshots
    // -------------------------------------------------------------------------

    pub fn create_db_cluster_snapshot(
        &mut self,
        snapshot_id: String,
        cluster_id: &str,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterSnapshot, NeptuneError> {
        if self.db_cluster_snapshots.contains_key(&snapshot_id) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBClusterSnapshot".to_string(),
                name: snapshot_id.clone(),
            });
        }
        let cluster = self
            .db_clusters
            .get(cluster_id)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: cluster_id.to_string(),
            })?
            .clone();
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster-snapshot:{snapshot_id}");
        let snap = DbClusterSnapshot {
            identifier: snapshot_id.clone(),
            db_cluster_identifier: cluster_id.to_string(),
            engine: cluster.engine.clone(),
            engine_version: cluster.engine_version.clone(),
            allocated_storage: 1,
            status: "available".to_string(),
            port: cluster.port,
            vpc_id: None,
            cluster_create_time: cluster.cluster_create_time.clone(),
            master_username: cluster.master_username.clone(),
            snapshot_type: "manual".to_string(),
            percent_progress: 100,
            storage_encrypted: cluster.storage_encrypted,
            kms_key_id: cluster.kms_key_id.clone(),
            db_cluster_snapshot_arn: arn.clone(),
            availability_zones: cluster.availability_zones.clone(),
            snapshot_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            tags: tags.clone(),
        };
        self.db_cluster_snapshots.insert(snapshot_id, snap.clone());
        self.resource_tags.insert(arn, tags);
        Ok(snap)
    }

    pub fn describe_db_cluster_snapshots(
        &self,
        snapshot_id: Option<&str>,
        cluster_id: Option<&str>,
    ) -> Vec<DbClusterSnapshot> {
        let mut snaps: Vec<DbClusterSnapshot> = self
            .db_cluster_snapshots
            .values()
            .filter(|s| {
                snapshot_id.is_none_or(|id| s.identifier == id)
                    && cluster_id.is_none_or(|id| s.db_cluster_identifier == id)
            })
            .cloned()
            .collect();
        snaps.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        snaps
    }

    pub fn delete_db_cluster_snapshot(
        &mut self,
        snapshot_id: &str,
    ) -> Result<DbClusterSnapshot, NeptuneError> {
        self.db_cluster_snapshots
            .remove(snapshot_id)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBClusterSnapshot".to_string(),
                name: snapshot_id.to_string(),
            })
    }

    pub fn restore_db_cluster_from_snapshot(
        &mut self,
        snapshot_id: &str,
        new_cluster_id: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbCluster, NeptuneError> {
        if self.db_clusters.contains_key(&new_cluster_id) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBCluster".to_string(),
                name: new_cluster_id.clone(),
            });
        }
        let snap = self
            .db_cluster_snapshots
            .get(snapshot_id)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBClusterSnapshot".to_string(),
                name: snapshot_id.to_string(),
            })?
            .clone();
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster:{new_cluster_id}");
        let endpoint = Some(format!(
            "{new_cluster_id}.cluster-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{new_cluster_id}.cluster-ro-{account_id}.{region}.neptune.amazonaws.com"
        ));
        let cluster = DbCluster {
            identifier: new_cluster_id.clone(),
            engine: snap.engine.clone(),
            engine_version: snap.engine_version.clone(),
            status: "available".to_string(),
            endpoint,
            reader_endpoint,
            port: snap.port,
            master_username: snap.master_username.clone(),
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            availability_zones: snap.availability_zones.clone(),
            arn: arn.clone(),
            tags: tags.clone(),
            cluster_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            multi_az: false,
            storage_encrypted: snap.storage_encrypted,
            kms_key_id: snap.kms_key_id.clone(),
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            deletion_protection: false,
            backup_retention_period: 1,
            members: Vec::new(),
            associated_roles: Vec::new(),
            serverless_v2_scaling_configuration: None,
        };
        self.db_clusters.insert(new_cluster_id, cluster.clone());
        self.resource_tags.insert(arn, tags);
        Ok(cluster)
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn add_tags_to_resource(&mut self, resource_arn: &str, new_tags: Vec<Tag>) {
        let tags = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for new_tag in new_tags {
            if let Some(existing) = tags.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                tags.push(new_tag);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<Tag> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    pub fn remove_tags_from_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    // -------------------------------------------------------------------------
    // Global Clusters
    // -------------------------------------------------------------------------

    pub fn create_global_cluster(
        &mut self,
        identifier: String,
        engine: String,
        engine_version: Option<String>,
        database_name: Option<String>,
        deletion_protection: bool,
        storage_encrypted: bool,
        source_db_cluster_identifier: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<GlobalCluster, NeptuneError> {
        if self.global_clusters.contains_key(&identifier) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "GlobalCluster".to_string(),
                name: identifier.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:global-cluster:{identifier}");
        let gc = GlobalCluster {
            identifier: identifier.clone(),
            engine,
            engine_version,
            database_name,
            deletion_protection,
            storage_encrypted,
            status: "available".to_string(),
            arn,
            source_db_cluster_identifier,
        };
        self.global_clusters.insert(identifier, gc.clone());
        Ok(gc)
    }

    pub fn describe_global_clusters(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<GlobalCluster>, NeptuneError> {
        match identifier {
            Some(id) => self
                .global_clusters
                .get(id)
                .map(|gc| vec![gc.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "GlobalCluster".to_string(),
                    name: id.to_string(),
                }),
            None => {
                let mut clusters: Vec<GlobalCluster> =
                    self.global_clusters.values().cloned().collect();
                clusters.sort_by(|a, b| a.identifier.cmp(&b.identifier));
                Ok(clusters)
            }
        }
    }

    pub fn delete_global_cluster(
        &mut self,
        identifier: &str,
    ) -> Result<GlobalCluster, NeptuneError> {
        let mut gc =
            self.global_clusters
                .remove(identifier)
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "GlobalCluster".to_string(),
                    name: identifier.to_string(),
                })?;
        gc.status = "deleting".to_string();
        Ok(gc)
    }

    pub fn modify_global_cluster(
        &mut self,
        identifier: &str,
        engine_version: Option<String>,
        deletion_protection: Option<bool>,
        new_identifier: Option<String>,
    ) -> Result<GlobalCluster, NeptuneError> {
        let gc =
            self.global_clusters
                .get_mut(identifier)
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "GlobalCluster".to_string(),
                    name: identifier.to_string(),
                })?;
        if let Some(v) = engine_version {
            gc.engine_version = Some(v);
        }
        if let Some(v) = deletion_protection {
            gc.deletion_protection = v;
        }
        if let Some(new_id) = new_identifier {
            gc.identifier = new_id.clone();
            let gc = self.global_clusters.remove(identifier).unwrap();
            self.global_clusters.insert(new_id, gc.clone());
            return Ok(gc);
        }
        Ok(gc.clone())
    }

    pub fn failover_global_cluster(&self, identifier: &str) -> Result<GlobalCluster, NeptuneError> {
        self.global_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "GlobalCluster".to_string(),
                name: identifier.to_string(),
            })
    }

    pub fn remove_from_global_cluster(
        &self,
        identifier: &str,
    ) -> Result<GlobalCluster, NeptuneError> {
        self.global_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "GlobalCluster".to_string(),
                name: identifier.to_string(),
            })
    }

    // -------------------------------------------------------------------------
    // DB Cluster Endpoints
    // -------------------------------------------------------------------------

    pub fn create_db_cluster_endpoint(
        &mut self,
        identifier: String,
        db_cluster_identifier: String,
        endpoint_type: String,
        static_members: Vec<String>,
        excluded_members: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterEndpoint, NeptuneError> {
        if self.db_cluster_endpoints.contains_key(&identifier) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "DBClusterEndpoint".to_string(),
                name: identifier.clone(),
            });
        }
        // Verify cluster exists
        if !self.db_clusters.contains_key(&db_cluster_identifier) {
            return Err(NeptuneError::NotFound {
                resource_type: "DBCluster".to_string(),
                name: db_cluster_identifier.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:cluster-endpoint:{identifier}");
        let resource_id = format!("cluster-endpoint-{}", uuid::Uuid::new_v4().simple());
        let ep_address =
            format!("{identifier}.cluster-custom-{account_id}.{region}.neptune.amazonaws.com");
        let ep = DbClusterEndpoint {
            identifier: identifier.clone(),
            db_cluster_identifier,
            endpoint_type: "CUSTOM".to_string(),
            custom_endpoint_type: Some(endpoint_type),
            endpoint: Some(ep_address),
            arn,
            resource_identifier: resource_id,
            status: "available".to_string(),
            static_members,
            excluded_members,
        };
        self.db_cluster_endpoints.insert(identifier, ep.clone());
        Ok(ep)
    }

    pub fn describe_db_cluster_endpoints(
        &self,
        endpoint_identifier: Option<&str>,
        cluster_identifier: Option<&str>,
    ) -> Vec<DbClusterEndpoint> {
        let mut eps: Vec<DbClusterEndpoint> = self
            .db_cluster_endpoints
            .values()
            .filter(|e| {
                endpoint_identifier.is_none_or(|id| e.identifier == id)
                    && cluster_identifier.is_none_or(|id| e.db_cluster_identifier == id)
            })
            .cloned()
            .collect();
        eps.sort_by(|a, b| a.identifier.cmp(&b.identifier));
        eps
    }

    pub fn delete_db_cluster_endpoint(
        &mut self,
        identifier: &str,
    ) -> Result<DbClusterEndpoint, NeptuneError> {
        let mut ep = self
            .db_cluster_endpoints
            .remove(identifier)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBClusterEndpoint".to_string(),
                name: identifier.to_string(),
            })?;
        ep.status = "deleting".to_string();
        Ok(ep)
    }

    pub fn modify_db_cluster_endpoint(
        &mut self,
        identifier: &str,
        endpoint_type: Option<String>,
        static_members: Option<Vec<String>>,
        excluded_members: Option<Vec<String>>,
    ) -> Result<DbClusterEndpoint, NeptuneError> {
        let ep = self
            .db_cluster_endpoints
            .get_mut(identifier)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "DBClusterEndpoint".to_string(),
                name: identifier.to_string(),
            })?;
        if let Some(v) = endpoint_type {
            ep.custom_endpoint_type = Some(v);
        }
        if let Some(v) = static_members {
            ep.static_members = v;
        }
        if let Some(v) = excluded_members {
            ep.excluded_members = v;
        }
        Ok(ep.clone())
    }

    // -------------------------------------------------------------------------
    // Event Subscriptions
    // -------------------------------------------------------------------------

    pub fn create_event_subscription(
        &mut self,
        subscription_name: String,
        sns_topic_arn: String,
        source_type: Option<String>,
        enabled: bool,
        event_categories: Vec<String>,
        source_ids: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> Result<EventSubscription, NeptuneError> {
        if self.event_subscriptions.contains_key(&subscription_name) {
            return Err(NeptuneError::AlreadyExists {
                resource_type: "EventSubscription".to_string(),
                name: subscription_name.clone(),
            });
        }
        let arn = format!("arn:aws:neptune:{region}:{account_id}:es:{subscription_name}");
        let creation_time = Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let sub = EventSubscription {
            subscription_name: subscription_name.clone(),
            sns_topic_arn,
            source_type,
            enabled,
            event_categories,
            source_ids,
            status: "active".to_string(),
            arn,
            customer_aws_id: account_id.to_string(),
            subscription_creation_time: creation_time,
        };
        self.event_subscriptions
            .insert(subscription_name, sub.clone());
        Ok(sub)
    }

    pub fn describe_event_subscriptions(
        &self,
        subscription_name: Option<&str>,
    ) -> Result<Vec<EventSubscription>, NeptuneError> {
        match subscription_name {
            Some(name) => self
                .event_subscriptions
                .get(name)
                .map(|s| vec![s.clone()])
                .ok_or_else(|| NeptuneError::NotFound {
                    resource_type: "EventSubscription".to_string(),
                    name: name.to_string(),
                }),
            None => {
                let mut subs: Vec<EventSubscription> =
                    self.event_subscriptions.values().cloned().collect();
                subs.sort_by(|a, b| a.subscription_name.cmp(&b.subscription_name));
                Ok(subs)
            }
        }
    }

    pub fn delete_event_subscription(
        &mut self,
        subscription_name: &str,
    ) -> Result<EventSubscription, NeptuneError> {
        self.event_subscriptions
            .remove(subscription_name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "EventSubscription".to_string(),
                name: subscription_name.to_string(),
            })
    }

    pub fn modify_event_subscription(
        &mut self,
        subscription_name: &str,
        sns_topic_arn: Option<String>,
        source_type: Option<String>,
        enabled: Option<bool>,
        event_categories: Option<Vec<String>>,
    ) -> Result<EventSubscription, NeptuneError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "EventSubscription".to_string(),
                name: subscription_name.to_string(),
            })?;
        if let Some(v) = sns_topic_arn {
            sub.sns_topic_arn = v;
        }
        if let Some(v) = source_type {
            sub.source_type = Some(v);
        }
        if let Some(v) = enabled {
            sub.enabled = v;
        }
        if let Some(v) = event_categories {
            sub.event_categories = v;
        }
        Ok(sub.clone())
    }

    pub fn add_source_identifier_to_subscription(
        &mut self,
        subscription_name: &str,
        source_identifier: &str,
    ) -> Result<EventSubscription, NeptuneError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "EventSubscription".to_string(),
                name: subscription_name.to_string(),
            })?;
        let sid = source_identifier.to_string();
        if !sub.source_ids.contains(&sid) {
            sub.source_ids.push(sid);
        }
        Ok(sub.clone())
    }

    pub fn remove_source_identifier_from_subscription(
        &mut self,
        subscription_name: &str,
        source_identifier: &str,
    ) -> Result<EventSubscription, NeptuneError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| NeptuneError::NotFound {
                resource_type: "EventSubscription".to_string(),
                name: subscription_name.to_string(),
            })?;
        sub.source_ids.retain(|s| s != source_identifier);
        Ok(sub.clone())
    }

    // -------------------------------------------------------------------------
    // Snapshot Attributes
    // -------------------------------------------------------------------------

    pub fn describe_db_cluster_snapshot_attributes(
        &self,
        snapshot_id: &str,
    ) -> Result<Vec<SnapshotAttribute>, NeptuneError> {
        // Verify snapshot exists
        if !self.db_cluster_snapshots.contains_key(snapshot_id) {
            return Err(NeptuneError::NotFound {
                resource_type: "DBClusterSnapshot".to_string(),
                name: snapshot_id.to_string(),
            });
        }
        Ok(self
            .snapshot_attributes
            .get(snapshot_id)
            .cloned()
            .unwrap_or_default())
    }

    pub fn modify_db_cluster_snapshot_attribute(
        &mut self,
        snapshot_id: &str,
        attribute_name: &str,
        values_to_add: Vec<String>,
        values_to_remove: Vec<String>,
    ) -> Result<Vec<SnapshotAttribute>, NeptuneError> {
        // Verify snapshot exists
        if !self.db_cluster_snapshots.contains_key(snapshot_id) {
            return Err(NeptuneError::NotFound {
                resource_type: "DBClusterSnapshot".to_string(),
                name: snapshot_id.to_string(),
            });
        }
        let attrs = self
            .snapshot_attributes
            .entry(snapshot_id.to_string())
            .or_default();
        let attr = if let Some(a) = attrs
            .iter_mut()
            .find(|a| a.attribute_name == attribute_name)
        {
            a
        } else {
            attrs.push(SnapshotAttribute {
                attribute_name: attribute_name.to_string(),
                attribute_values: Vec::new(),
            });
            attrs.last_mut().unwrap()
        };
        for v in values_to_add {
            if !attr.attribute_values.contains(&v) {
                attr.attribute_values.push(v);
            }
        }
        attr.attribute_values
            .retain(|v| !values_to_remove.contains(v));
        Ok(attrs.clone())
    }
}
