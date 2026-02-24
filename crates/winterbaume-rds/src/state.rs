use std::collections::HashMap;

use crate::types::{
    BlueGreenDeployment, DbCluster, DbClusterEndpoint, DbClusterParameterGroup, DbClusterSnapshot,
    DbInstance, DbParameterGroup, DbProxy, DbProxyEndpoint, DbProxyTargetGroup, DbSecurityGroup,
    DbShardGroup, DbSnapshot, DbSubnetGroup, EventSubscription, ExportTask, GlobalCluster,
    OptionGroup, Tag,
};

/// In-memory state for the RDS service.
#[derive(Debug, Default)]
pub struct RdsState {
    pub db_instances: HashMap<String, DbInstance>,
    pub db_clusters: HashMap<String, DbCluster>,
    pub db_subnet_groups: HashMap<String, DbSubnetGroup>,
    pub db_parameter_groups: HashMap<String, DbParameterGroup>,
    pub db_cluster_parameter_groups: HashMap<String, DbClusterParameterGroup>,
    pub db_snapshots: HashMap<String, DbSnapshot>,
    pub db_cluster_snapshots: HashMap<String, DbClusterSnapshot>,
    pub db_security_groups: HashMap<String, DbSecurityGroup>,
    pub event_subscriptions: HashMap<String, EventSubscription>,
    pub option_groups: HashMap<String, OptionGroup>,
    pub export_tasks: HashMap<String, ExportTask>,
    pub global_clusters: HashMap<String, GlobalCluster>,
    pub db_proxies: HashMap<String, DbProxy>,
    /// Proxy target groups keyed by "{proxy_name}/{target_group_name}".
    pub db_proxy_target_groups: HashMap<String, DbProxyTargetGroup>,
    pub blue_green_deployments: HashMap<String, BlueGreenDeployment>,
    pub db_shard_groups: HashMap<String, DbShardGroup>,
    /// DB cluster endpoints keyed by endpoint identifier.
    pub db_cluster_endpoints: HashMap<String, DbClusterEndpoint>,
    /// DB proxy endpoints keyed by endpoint name.
    pub db_proxy_endpoints: HashMap<String, DbProxyEndpoint>,
    /// Tags keyed by resource ARN.
    pub resource_tags: HashMap<String, Vec<Tag>>,
}

/// Error type for RDS operations.
#[derive(Debug, thiserror::Error)]
pub enum RdsError {
    #[error("{resource_type} already exists: {name}")]
    AlreadyExists { resource_type: String, name: String },
    #[error("{resource_type} not found: {name}")]
    NotFound { resource_type: String, name: String },
    #[error("{0}")]
    InvalidParameter(String),
    #[error("{0}")]
    InvalidState(String),
}

impl RdsError {
    pub fn already_exists(resource_type: &str, name: &str) -> Self {
        Self::AlreadyExists {
            resource_type: resource_type.to_string(),
            name: name.to_string(),
        }
    }

    pub fn not_found(resource_type: &str, name: &str) -> Self {
        Self::NotFound {
            resource_type: resource_type.to_string(),
            name: name.to_string(),
        }
    }

    pub fn invalid_parameter(message: &str) -> Self {
        Self::InvalidParameter(message.to_string())
    }

    pub fn invalid_state(message: &str) -> Self {
        Self::InvalidState(message.to_string())
    }
}

impl RdsState {
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
        master_username: Option<String>,
        db_name: Option<String>,
        port: Option<i32>,
        multi_az: bool,
        storage_type: Option<String>,
        allocated_storage: i32,
        db_subnet_group_name: Option<String>,
        vpc_security_group_ids: Vec<String>,
        db_parameter_group_names: Vec<String>,
        availability_zone: Option<String>,
        publicly_accessible: bool,
        auto_minor_version_upgrade: bool,
        backup_retention_period: i32,
        db_cluster_identifier: Option<String>,
        license_model: Option<String>,
        iops: Option<i32>,
        deletion_protection: bool,
        copy_tags_to_snapshot: bool,
        monitoring_interval: Option<i32>,
        performance_insights_enabled: bool,
        storage_encrypted: bool,
        kms_key_id: Option<String>,
        ca_certificate_identifier: Option<String>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, RdsError> {
        if self.db_instances.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBInstance", &identifier));
        }
        let engine_str = engine.as_str();
        let default_port = match engine_str {
            "mysql" | "mariadb" => 3306,
            "postgres" => 5432,
            "oracle-ee" | "oracle-se2" | "oracle-se1" | "oracle-se" => 1521,
            "sqlserver-ee" | "sqlserver-se" | "sqlserver-ex" | "sqlserver-web" => 1433,
            _ => 3306,
        };
        let port = port.or(Some(default_port));
        let endpoint_address = Some(format!(
            "{identifier}.{account_id}.{region}.rds.amazonaws.com"
        ));
        let arn = format!("arn:aws:rds:{region}:{account_id}:db:{identifier}");
        let instance_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let inst = DbInstance {
            identifier: identifier.clone(),
            db_instance_class,
            engine,
            engine_version: engine_version.unwrap_or_else(|| "8.0".to_string()),
            status: "available".to_string(),
            master_username,
            db_name,
            endpoint_address,
            port,
            multi_az,
            storage_type: Some(storage_type.unwrap_or_else(|| "gp2".to_string())),
            allocated_storage,
            db_subnet_group_name,
            vpc_security_group_ids,
            db_parameter_group_names,
            availability_zone,
            publicly_accessible,
            auto_minor_version_upgrade,
            backup_retention_period,
            db_cluster_identifier,
            arn: arn.clone(),
            tags: tags.clone(),
            instance_create_time,
            license_model,
            iops,
            deletion_protection,
            copy_tags_to_snapshot,
            monitoring_interval,
            performance_insights_enabled,
            storage_encrypted,
            kms_key_id,
            ca_certificate_identifier,
            secondary_availability_zone: None,
            associated_roles: Vec::new(),
        };
        // Store tags in resource_tags map too
        self.resource_tags.insert(arn, tags);
        self.db_instances.insert(identifier, inst.clone());
        Ok(inst)
    }

    pub fn describe_db_instances(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<&DbInstance>, RdsError> {
        if let Some(id) = identifier {
            match self.db_instances.get(id) {
                Some(inst) => Ok(vec![inst]),
                None => Err(RdsError::not_found("DBInstance", id)),
            }
        } else {
            Ok(self.db_instances.values().collect())
        }
    }

    pub fn delete_db_instance(&mut self, identifier: &str) -> Result<DbInstance, RdsError> {
        self.db_instances
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_db_instance(
        &mut self,
        identifier: &str,
        db_instance_class: Option<String>,
        engine_version: Option<String>,
        multi_az: Option<bool>,
        storage_type: Option<String>,
        allocated_storage: Option<i32>,
        backup_retention_period: Option<i32>,
        iops: Option<i32>,
        deletion_protection: Option<bool>,
        auto_minor_version_upgrade: Option<bool>,
        new_db_instance_identifier: Option<String>,
        master_user_password: Option<String>,
    ) -> Result<DbInstance, RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        if let Some(v) = db_instance_class {
            inst.db_instance_class = v;
        }
        if let Some(v) = engine_version {
            inst.engine_version = v;
        }
        if let Some(v) = multi_az {
            inst.multi_az = v;
        }
        if let Some(v) = storage_type {
            inst.storage_type = Some(v);
        }
        if let Some(v) = allocated_storage {
            inst.allocated_storage = v;
        }
        if let Some(v) = backup_retention_period {
            inst.backup_retention_period = v;
        }
        if let Some(v) = iops {
            inst.iops = Some(v);
        }
        if let Some(v) = deletion_protection {
            inst.deletion_protection = v;
        }
        if let Some(v) = auto_minor_version_upgrade {
            inst.auto_minor_version_upgrade = v;
        }
        let _ = master_user_password; // accepted but not stored
        let result = inst.clone();
        if let Some(new_id) = new_db_instance_identifier {
            if new_id != identifier {
                if self.db_instances.contains_key(&new_id) {
                    return Err(RdsError::already_exists("DBInstance", &new_id));
                }
                let inst = self.db_instances.remove(identifier).unwrap();
                self.db_instances.insert(new_id, inst);
            }
        }
        Ok(result)
    }

    pub fn reboot_db_instance(&self, identifier: &str) -> Result<DbInstance, RdsError> {
        self.db_instances
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))
    }

    pub fn start_db_instance(&mut self, identifier: &str) -> Result<DbInstance, RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        inst.status = "available".to_string();
        Ok(inst.clone())
    }

    pub fn stop_db_instance(&mut self, identifier: &str) -> Result<DbInstance, RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        inst.status = "stopped".to_string();
        Ok(inst.clone())
    }

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
        port: Option<i32>,
        db_subnet_group_name: Option<String>,
        vpc_security_group_ids: Vec<String>,
        availability_zones: Vec<String>,
        backup_retention_period: i32,
        deletion_protection: bool,
        storage_encrypted: bool,
        kms_key_id: Option<String>,
        db_cluster_parameter_group: Option<String>,
        engine_mode: Option<String>,
        copy_tags_to_snapshot: bool,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbCluster, RdsError> {
        if self.db_clusters.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBCluster", &identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster:{identifier}");
        let engine_str = engine.as_str();
        let default_port = match engine_str {
            "aurora-mysql" | "mysql" => 3306,
            "aurora-postgresql" | "postgres" => 5432,
            _ => 3306,
        };
        let port = port.or(Some(default_port));
        let endpoint = Some(format!(
            "{identifier}.cluster-{account_id}.{region}.rds.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{identifier}.cluster-ro-{account_id}.{region}.rds.amazonaws.com"
        ));
        let cluster_create_time = Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        let cluster = DbCluster {
            identifier: identifier.clone(),
            engine,
            engine_version,
            status: "available".to_string(),
            endpoint,
            reader_endpoint,
            port,
            master_username,
            database_name,
            db_subnet_group_name,
            vpc_security_group_ids,
            availability_zones,
            arn: arn.clone(),
            tags: tags.clone(),
            cluster_create_time,
            multi_az: false,
            storage_type: None,
            allocated_storage: None,
            backup_retention_period,
            deletion_protection,
            storage_encrypted,
            kms_key_id,
            db_cluster_parameter_group,
            engine_mode,
            copy_tags_to_snapshot,
            members: Vec::new(),
            associated_roles: Vec::new(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_clusters.insert(identifier, cluster.clone());
        Ok(cluster)
    }

    pub fn describe_db_clusters(
        &self,
        identifier: Option<&str>,
    ) -> Result<Vec<&DbCluster>, RdsError> {
        if let Some(id) = identifier {
            match self.db_clusters.get(id) {
                Some(c) => Ok(vec![c]),
                None => Err(RdsError::not_found("DBCluster", id)),
            }
        } else {
            Ok(self.db_clusters.values().collect())
        }
    }

    pub fn delete_db_cluster(&mut self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn modify_db_cluster(
        &mut self,
        identifier: &str,
        engine_version: Option<String>,
        master_user_password: Option<String>,
        backup_retention_period: Option<i32>,
        deletion_protection: Option<bool>,
        new_db_cluster_identifier: Option<String>,
        vpc_security_group_ids: Option<Vec<String>>,
        db_cluster_parameter_group: Option<String>,
    ) -> Result<DbCluster, RdsError> {
        let cluster = self
            .db_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))?;
        if let Some(v) = engine_version {
            cluster.engine_version = Some(v);
        }
        if let Some(v) = backup_retention_period {
            cluster.backup_retention_period = v;
        }
        if let Some(v) = deletion_protection {
            cluster.deletion_protection = v;
        }
        if let Some(v) = vpc_security_group_ids {
            cluster.vpc_security_group_ids = v;
        }
        if let Some(v) = db_cluster_parameter_group {
            cluster.db_cluster_parameter_group = Some(v);
        }
        let _ = master_user_password;
        let result = cluster.clone();
        if let Some(new_id) = new_db_cluster_identifier {
            if new_id != identifier {
                if self.db_clusters.contains_key(&new_id) {
                    return Err(RdsError::already_exists("DBCluster", &new_id));
                }
                let c = self.db_clusters.remove(identifier).unwrap();
                self.db_clusters.insert(new_id, c);
            }
        }
        Ok(result)
    }

    pub fn failover_db_cluster(&self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    pub fn start_db_cluster(&mut self, identifier: &str) -> Result<DbCluster, RdsError> {
        let c = self
            .db_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))?;
        c.status = "available".to_string();
        Ok(c.clone())
    }

    pub fn stop_db_cluster(&mut self, identifier: &str) -> Result<DbCluster, RdsError> {
        let c = self
            .db_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))?;
        c.status = "stopped".to_string();
        Ok(c.clone())
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
    ) -> Result<DbSubnetGroup, RdsError> {
        if self.db_subnet_groups.contains_key(&name) {
            return Err(RdsError::already_exists("DBSubnetGroup", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:subgrp:{name}");
        let sg = DbSubnetGroup {
            name: name.clone(),
            description,
            vpc_id: None,
            subnet_ids,
            status: "Complete".to_string(),
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_subnet_groups.insert(name, sg.clone());
        Ok(sg)
    }

    pub fn describe_db_subnet_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&DbSubnetGroup>, RdsError> {
        if let Some(n) = name {
            match self.db_subnet_groups.get(n) {
                Some(sg) => Ok(vec![sg]),
                None => Err(RdsError::not_found("DBSubnetGroup", n)),
            }
        } else {
            Ok(self.db_subnet_groups.values().collect())
        }
    }

    pub fn delete_db_subnet_group(&mut self, name: &str) -> Result<(), RdsError> {
        self.db_subnet_groups
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| RdsError::not_found("DBSubnetGroup", name))
    }

    pub fn modify_db_subnet_group(
        &mut self,
        name: &str,
        description: Option<String>,
        subnet_ids: Option<Vec<String>>,
    ) -> Result<DbSubnetGroup, RdsError> {
        let sg = self
            .db_subnet_groups
            .get_mut(name)
            .ok_or_else(|| RdsError::not_found("DBSubnetGroup", name))?;
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
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbParameterGroup, RdsError> {
        if self.db_parameter_groups.contains_key(&name) {
            return Err(RdsError::already_exists("DBParameterGroup", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:pg:{name}");
        let pg = DbParameterGroup {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_parameter_groups.insert(name, pg.clone());
        Ok(pg)
    }

    pub fn describe_db_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&DbParameterGroup>, RdsError> {
        if let Some(n) = name {
            match self.db_parameter_groups.get(n) {
                Some(pg) => Ok(vec![pg]),
                None => Err(RdsError::not_found("DBParameterGroup", n)),
            }
        } else {
            Ok(self.db_parameter_groups.values().collect())
        }
    }

    pub fn delete_db_parameter_group(&mut self, name: &str) -> Result<(), RdsError> {
        self.db_parameter_groups
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| RdsError::not_found("DBParameterGroup", name))
    }

    // -------------------------------------------------------------------------
    // DB Cluster Parameter Groups
    // -------------------------------------------------------------------------

    pub fn create_db_cluster_parameter_group(
        &mut self,
        name: String,
        family: String,
        description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterParameterGroup, RdsError> {
        if self.db_cluster_parameter_groups.contains_key(&name) {
            return Err(RdsError::already_exists("DBClusterParameterGroup", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster-pg:{name}");
        let pg = DbClusterParameterGroup {
            name: name.clone(),
            family,
            description,
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_cluster_parameter_groups.insert(name, pg.clone());
        Ok(pg)
    }

    pub fn describe_db_cluster_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&DbClusterParameterGroup>, RdsError> {
        if let Some(n) = name {
            match self.db_cluster_parameter_groups.get(n) {
                Some(pg) => Ok(vec![pg]),
                None => Err(RdsError::not_found("DBClusterParameterGroup", n)),
            }
        } else {
            Ok(self.db_cluster_parameter_groups.values().collect())
        }
    }

    pub fn delete_db_cluster_parameter_group(&mut self, name: &str) -> Result<(), RdsError> {
        self.db_cluster_parameter_groups
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| RdsError::not_found("DBClusterParameterGroup", name))
    }

    // -------------------------------------------------------------------------
    // DB Snapshots
    // -------------------------------------------------------------------------

    pub fn create_db_snapshot(
        &mut self,
        snapshot_id: String,
        db_instance_id: &str,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbSnapshot, RdsError> {
        if self.db_snapshots.contains_key(&snapshot_id) {
            return Err(RdsError::already_exists("DBSnapshot", &snapshot_id));
        }
        let inst = self
            .db_instances
            .get(db_instance_id)
            .ok_or_else(|| RdsError::not_found("DBInstance", db_instance_id))?;
        let arn = format!("arn:aws:rds:{region}:{account_id}:snapshot:{snapshot_id}");
        let snap = DbSnapshot {
            identifier: snapshot_id.clone(),
            db_instance_identifier: db_instance_id.to_string(),
            engine: inst.engine.clone(),
            engine_version: Some(inst.engine_version.clone()),
            allocated_storage: inst.allocated_storage,
            status: "available".to_string(),
            port: inst.port,
            availability_zone: inst.availability_zone.clone(),
            vpc_id: None,
            instance_create_time: inst.instance_create_time.clone(),
            master_username: inst.master_username.clone(),
            snapshot_type: "manual".to_string(),
            iops: inst.iops,
            option_group_name: None,
            percent_progress: 100,
            source_region: None,
            source_db_snapshot_identifier: None,
            storage_type: inst.storage_type.clone(),
            tde_credential_arn: None,
            encrypted: inst.storage_encrypted,
            kms_key_id: inst.kms_key_id.clone(),
            db_snapshot_arn: arn.clone(),
            timezone: None,
            db_instance_automated_backups_arn: None,
            snapshot_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_snapshots.insert(snapshot_id, snap.clone());
        Ok(snap)
    }

    pub fn describe_db_snapshots(
        &self,
        snapshot_id: Option<&str>,
        db_instance_id: Option<&str>,
    ) -> Vec<&DbSnapshot> {
        if let Some(id) = snapshot_id {
            return self.db_snapshots.get(id).into_iter().collect();
        }
        self.db_snapshots
            .values()
            .filter(|s| {
                db_instance_id.is_none()
                    || Some(s.db_instance_identifier.as_str()) == db_instance_id
            })
            .collect()
    }

    pub fn delete_db_snapshot(&mut self, snapshot_id: &str) -> Result<DbSnapshot, RdsError> {
        self.db_snapshots
            .remove(snapshot_id)
            .ok_or_else(|| RdsError::not_found("DBSnapshot", snapshot_id))
    }

    pub fn copy_db_snapshot(
        &mut self,
        source_id: &str,
        target_id: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbSnapshot, RdsError> {
        let source = self
            .db_snapshots
            .get(source_id)
            .ok_or_else(|| RdsError::not_found("DBSnapshot", source_id))?
            .clone();
        if self.db_snapshots.contains_key(&target_id) {
            return Err(RdsError::already_exists("DBSnapshot", &target_id));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:snapshot:{target_id}");
        let mut copy = source;
        copy.identifier = target_id.clone();
        copy.db_snapshot_arn = arn.clone();
        copy.snapshot_type = "manual".to_string();
        copy.source_db_snapshot_identifier = Some(source_id.to_string());
        copy.tags = tags.clone();
        copy.snapshot_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        self.resource_tags.insert(arn, tags);
        self.db_snapshots.insert(target_id, copy.clone());
        Ok(copy)
    }

    pub fn restore_db_instance_from_db_snapshot(
        &mut self,
        snapshot_id: &str,
        new_identifier: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, RdsError> {
        let snap = self
            .db_snapshots
            .get(snapshot_id)
            .ok_or_else(|| RdsError::not_found("DBSnapshot", snapshot_id))?
            .clone();
        if self.db_instances.contains_key(&new_identifier) {
            return Err(RdsError::already_exists("DBInstance", &new_identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:db:{new_identifier}");
        let endpoint = Some(format!(
            "{new_identifier}.{account_id}.{region}.rds.amazonaws.com"
        ));
        let inst = DbInstance {
            identifier: new_identifier.clone(),
            db_instance_class: "db.t3.medium".to_string(),
            engine: snap.engine.clone(),
            engine_version: snap.engine_version.unwrap_or_default(),
            status: "available".to_string(),
            master_username: snap.master_username.clone(),
            db_name: None,
            endpoint_address: endpoint,
            port: snap.port,
            multi_az: false,
            storage_type: snap.storage_type.clone(),
            allocated_storage: snap.allocated_storage,
            db_subnet_group_name: None,
            vpc_security_group_ids: Vec::new(),
            db_parameter_group_names: Vec::new(),
            availability_zone: snap.availability_zone.clone(),
            publicly_accessible: false,
            auto_minor_version_upgrade: true,
            backup_retention_period: 1,
            db_cluster_identifier: None,
            arn: arn.clone(),
            tags: tags.clone(),
            instance_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            license_model: None,
            iops: snap.iops,
            deletion_protection: false,
            copy_tags_to_snapshot: false,
            monitoring_interval: None,
            performance_insights_enabled: false,
            storage_encrypted: snap.encrypted,
            kms_key_id: snap.kms_key_id.clone(),
            ca_certificate_identifier: None,
            secondary_availability_zone: None,
            associated_roles: Vec::new(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_instances.insert(new_identifier, inst.clone());
        Ok(inst)
    }

    pub fn restore_db_instance_to_point_in_time(
        &mut self,
        source_id: &str,
        target_id: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, RdsError> {
        let source = self
            .db_instances
            .get(source_id)
            .ok_or_else(|| RdsError::not_found("DBInstance", source_id))?
            .clone();
        if self.db_instances.contains_key(&target_id) {
            return Err(RdsError::already_exists("DBInstance", &target_id));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:db:{target_id}");
        let endpoint = Some(format!(
            "{target_id}.{account_id}.{region}.rds.amazonaws.com"
        ));
        let mut inst = source;
        inst.identifier = target_id.clone();
        inst.arn = arn.clone();
        inst.endpoint_address = endpoint;
        inst.tags = tags.clone();
        inst.status = "available".to_string();
        self.resource_tags.insert(arn, tags);
        self.db_instances.insert(target_id, inst.clone());
        Ok(inst)
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
    ) -> Result<DbClusterSnapshot, RdsError> {
        if self.db_cluster_snapshots.contains_key(&snapshot_id) {
            return Err(RdsError::already_exists("DBClusterSnapshot", &snapshot_id));
        }
        let cluster = self
            .db_clusters
            .get(cluster_id)
            .ok_or_else(|| RdsError::not_found("DBCluster", cluster_id))?;
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster-snapshot:{snapshot_id}");
        let snap = DbClusterSnapshot {
            identifier: snapshot_id.clone(),
            db_cluster_identifier: cluster_id.to_string(),
            engine: cluster.engine.clone(),
            engine_version: cluster.engine_version.clone(),
            allocated_storage: cluster.allocated_storage.unwrap_or(0),
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
            source_db_cluster_snapshot_arn: None,
            availability_zones: cluster.availability_zones.clone(),
            snapshot_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            tags: tags.clone(),
            storage_type: cluster.storage_type.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_cluster_snapshots.insert(snapshot_id, snap.clone());
        Ok(snap)
    }

    pub fn describe_db_cluster_snapshots(
        &self,
        snapshot_id: Option<&str>,
        cluster_id: Option<&str>,
    ) -> Vec<&DbClusterSnapshot> {
        if let Some(id) = snapshot_id {
            return self.db_cluster_snapshots.get(id).into_iter().collect();
        }
        self.db_cluster_snapshots
            .values()
            .filter(|s| {
                cluster_id.is_none() || Some(s.db_cluster_identifier.as_str()) == cluster_id
            })
            .collect()
    }

    pub fn delete_db_cluster_snapshot(
        &mut self,
        snapshot_id: &str,
    ) -> Result<DbClusterSnapshot, RdsError> {
        self.db_cluster_snapshots
            .remove(snapshot_id)
            .ok_or_else(|| RdsError::not_found("DBClusterSnapshot", snapshot_id))
    }

    pub fn copy_db_cluster_snapshot(
        &mut self,
        source_id: &str,
        target_id: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterSnapshot, RdsError> {
        let source = self
            .db_cluster_snapshots
            .get(source_id)
            .ok_or_else(|| RdsError::not_found("DBClusterSnapshot", source_id))?
            .clone();
        if self.db_cluster_snapshots.contains_key(&target_id) {
            return Err(RdsError::already_exists("DBClusterSnapshot", &target_id));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster-snapshot:{target_id}");
        let mut copy = source;
        copy.identifier = target_id.clone();
        copy.db_cluster_snapshot_arn = arn.clone();
        copy.source_db_cluster_snapshot_arn = Some(source_id.to_string());
        copy.tags = tags.clone();
        copy.snapshot_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        self.resource_tags.insert(arn, tags);
        self.db_cluster_snapshots.insert(target_id, copy.clone());
        Ok(copy)
    }

    pub fn restore_db_cluster_from_snapshot(
        &mut self,
        snapshot_id: &str,
        new_cluster_id: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbCluster, RdsError> {
        let snap = self
            .db_cluster_snapshots
            .get(snapshot_id)
            .ok_or_else(|| RdsError::not_found("DBClusterSnapshot", snapshot_id))?
            .clone();
        if self.db_clusters.contains_key(&new_cluster_id) {
            return Err(RdsError::already_exists("DBCluster", &new_cluster_id));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster:{new_cluster_id}");
        let endpoint = Some(format!(
            "{new_cluster_id}.cluster-{account_id}.{region}.rds.amazonaws.com"
        ));
        let reader_endpoint = Some(format!(
            "{new_cluster_id}.cluster-ro-{account_id}.{region}.rds.amazonaws.com"
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
            storage_type: snap.storage_type.clone(),
            allocated_storage: Some(snap.allocated_storage),
            backup_retention_period: 1,
            deletion_protection: false,
            storage_encrypted: snap.storage_encrypted,
            kms_key_id: snap.kms_key_id.clone(),
            db_cluster_parameter_group: None,
            engine_mode: None,
            copy_tags_to_snapshot: false,
            members: Vec::new(),
            associated_roles: Vec::new(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_clusters.insert(new_cluster_id, cluster.clone());
        Ok(cluster)
    }

    // -------------------------------------------------------------------------
    // DB Security Groups
    // -------------------------------------------------------------------------

    pub fn create_db_security_group(
        &mut self,
        name: String,
        description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbSecurityGroup, RdsError> {
        if self.db_security_groups.contains_key(&name) {
            return Err(RdsError::already_exists("DBSecurityGroup", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:secgrp:{name}");
        let sg = DbSecurityGroup {
            name: name.clone(),
            description,
            vpc_id: None,
            ec2_security_groups: Vec::new(),
            ip_ranges: Vec::new(),
            arn: arn.clone(),
            tags: tags.clone(),
            owner_id: account_id.to_string(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_security_groups.insert(name, sg.clone());
        Ok(sg)
    }

    pub fn describe_db_security_groups(&self, name: Option<&str>) -> Vec<&DbSecurityGroup> {
        if let Some(n) = name {
            return self.db_security_groups.get(n).into_iter().collect();
        }
        self.db_security_groups.values().collect()
    }

    pub fn delete_db_security_group(&mut self, name: &str) -> Result<(), RdsError> {
        self.db_security_groups
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| RdsError::not_found("DBSecurityGroup", name))
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn add_tags_to_resource(&mut self, resource_arn: &str, tags: Vec<Tag>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for new_tag in tags {
            if let Some(existing) = entry.iter_mut().find(|t| t.key == new_tag.key) {
                existing.value = new_tag.value;
            } else {
                entry.push(new_tag);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<&Tag> {
        self.resource_tags
            .get(resource_arn)
            .map(|v| v.iter().collect())
            .unwrap_or_default()
    }

    pub fn remove_tags_from_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            tags.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    // -------------------------------------------------------------------------
    // Event Subscriptions
    // -------------------------------------------------------------------------

    pub fn create_event_subscription(
        &mut self,
        name: String,
        sns_topic_arn: String,
        source_type: Option<String>,
        source_ids: Vec<String>,
        event_categories: Vec<String>,
        enabled: bool,
        account_id: &str,
        region: &str,
    ) -> Result<EventSubscription, RdsError> {
        if self.event_subscriptions.contains_key(&name) {
            return Err(RdsError::already_exists("EventSubscription", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:es:{name}");
        let sub = EventSubscription {
            subscription_name: name.clone(),
            sns_topic_arn,
            source_type,
            source_ids,
            event_categories,
            enabled,
            status: "active".to_string(),
            arn,
            customer_aws_id: account_id.to_string(),
            subscription_creation_time: chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        };
        self.event_subscriptions.insert(name, sub.clone());
        Ok(sub)
    }

    pub fn describe_event_subscriptions(&self, name: Option<&str>) -> Vec<&EventSubscription> {
        if let Some(n) = name {
            return self.event_subscriptions.get(n).into_iter().collect();
        }
        self.event_subscriptions.values().collect()
    }

    pub fn delete_event_subscription(&mut self, name: &str) -> Result<EventSubscription, RdsError> {
        self.event_subscriptions
            .remove(name)
            .ok_or_else(|| RdsError::not_found("SubscriptionNotFound", name))
    }

    // -------------------------------------------------------------------------
    // Option Groups
    // -------------------------------------------------------------------------

    pub fn create_option_group(
        &mut self,
        name: String,
        engine_name: String,
        major_engine_version: String,
        description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<OptionGroup, RdsError> {
        if self.option_groups.contains_key(&name) {
            return Err(RdsError::already_exists("OptionGroup", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:og:{name}");
        let og = OptionGroup {
            name: name.clone(),
            engine_name,
            major_engine_version,
            description,
            allows_vpc_and_non_vpc_instance_memberships: true,
            vpc_id: None,
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.option_groups.insert(name, og.clone());
        Ok(og)
    }

    pub fn describe_option_groups(&self, name: Option<&str>) -> Vec<&OptionGroup> {
        if let Some(n) = name {
            return self.option_groups.get(n).into_iter().collect();
        }
        self.option_groups.values().collect()
    }

    pub fn delete_option_group(&mut self, name: &str) -> Result<(), RdsError> {
        self.option_groups
            .remove(name)
            .map(|_| ())
            .ok_or_else(|| RdsError::not_found("OptionGroupNotFoundFault", name))
    }

    // -------------------------------------------------------------------------
    // Global Clusters
    // -------------------------------------------------------------------------

    pub fn create_global_cluster(
        &mut self,
        identifier: String,
        engine: Option<String>,
        engine_version: Option<String>,
        database_name: Option<String>,
        deletion_protection: bool,
        storage_encrypted: bool,
        account_id: &str,
        region: &str,
    ) -> Result<GlobalCluster, RdsError> {
        if self.global_clusters.contains_key(&identifier) {
            return Err(RdsError::already_exists("GlobalCluster", &identifier));
        }
        let resource_id = format!("cluster-{}", uuid::Uuid::new_v4());
        let arn = format!("arn:aws:rds::{account_id}:global-cluster:{identifier}");
        let _ = region;
        let gc = GlobalCluster {
            global_cluster_identifier: identifier.clone(),
            global_cluster_resource_id: resource_id,
            global_cluster_arn: arn,
            status: "available".to_string(),
            engine,
            engine_version,
            database_name,
            storage_encrypted,
            deletion_protection,
        };
        self.global_clusters.insert(identifier, gc.clone());
        Ok(gc)
    }

    pub fn describe_global_clusters(&self, identifier: Option<&str>) -> Vec<&GlobalCluster> {
        if let Some(id) = identifier {
            return self.global_clusters.get(id).into_iter().collect();
        }
        self.global_clusters.values().collect()
    }

    pub fn delete_global_cluster(&mut self, identifier: &str) -> Result<GlobalCluster, RdsError> {
        self.global_clusters
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("GlobalCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // DB Instance Read Replicas
    // -------------------------------------------------------------------------

    pub fn create_db_instance_read_replica(
        &mut self,
        replica_id: String,
        source_id: &str,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, RdsError> {
        let source = self
            .db_instances
            .get(source_id)
            .ok_or_else(|| RdsError::not_found("DBInstance", source_id))?
            .clone();
        if self.db_instances.contains_key(&replica_id) {
            return Err(RdsError::already_exists("DBInstance", &replica_id));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:db:{replica_id}");
        let endpoint = Some(format!(
            "{replica_id}.{account_id}.{region}.rds.amazonaws.com"
        ));
        let mut inst = source;
        inst.identifier = replica_id.clone();
        inst.arn = arn.clone();
        inst.endpoint_address = endpoint;
        inst.tags = tags.clone();
        inst.status = "available".to_string();
        inst.instance_create_time =
            Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        self.resource_tags.insert(arn, tags);
        self.db_instances.insert(replica_id, inst.clone());
        Ok(inst)
    }

    pub fn promote_read_replica(
        &mut self,
        identifier: &str,
        backup_retention_period: Option<i32>,
    ) -> Result<DbInstance, RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        if let Some(v) = backup_retention_period {
            inst.backup_retention_period = v;
        }
        inst.db_cluster_identifier = None;
        Ok(inst.clone())
    }

    pub fn promote_read_replica_db_cluster(&self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // Export Tasks
    // -------------------------------------------------------------------------

    pub fn start_export_task(
        &mut self,
        export_task_identifier: String,
        source_arn: String,
        s3_bucket: String,
        s3_prefix: Option<String>,
        iam_role_arn: String,
        kms_key_id: String,
        export_only: Vec<String>,
    ) -> Result<ExportTask, RdsError> {
        if self.export_tasks.contains_key(&export_task_identifier) {
            return Err(RdsError::already_exists(
                "ExportTask",
                &export_task_identifier,
            ));
        }
        let task = ExportTask {
            export_task_identifier: export_task_identifier.clone(),
            source_arn,
            export_only,
            source_type: None,
            snapshot_time: None,
            task_start_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            task_end_time: None,
            s3_bucket,
            s3_prefix,
            iam_role_arn,
            kms_key_id,
            status: "starting".to_string(),
            percent_progress: 0,
            total_extracted_data_in_gb: None,
            failure_cause: None,
            warning_message: None,
        };
        self.export_tasks
            .insert(export_task_identifier, task.clone());
        Ok(task)
    }

    pub fn cancel_export_task(&mut self, identifier: &str) -> Result<ExportTask, RdsError> {
        let task = self
            .export_tasks
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("ExportTask", identifier))?;
        task.status = "canceling".to_string();
        Ok(task.clone())
    }

    pub fn describe_export_tasks(&self, identifier: Option<&str>) -> Vec<&ExportTask> {
        if let Some(id) = identifier {
            return self.export_tasks.get(id).into_iter().collect();
        }
        self.export_tasks.values().collect()
    }

    // -------------------------------------------------------------------------
    // DB Proxies
    // -------------------------------------------------------------------------

    pub fn create_db_proxy(
        &mut self,
        name: String,
        engine_family: String,
        vpc_subnet_ids: Vec<String>,
        vpc_security_group_ids: Vec<String>,
        role_arn: String,
        require_tls: bool,
        idle_client_timeout: i32,
        debug_logging: bool,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbProxy, RdsError> {
        if self.db_proxies.contains_key(&name) {
            return Err(RdsError::already_exists("DBProxy", &name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:db-proxy:{name}");
        let endpoint = format!("{name}.proxy-{account_id}.{region}.rds.amazonaws.com");
        let proxy = DbProxy {
            db_proxy_name: name.clone(),
            db_proxy_arn: arn,
            status: "available".to_string(),
            engine_family,
            vpc_id: None,
            vpc_security_group_ids,
            vpc_subnet_ids,
            endpoint,
            require_tls,
            idle_client_timeout,
            debug_logging,
            role_arn,
            created_date: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            updated_date: None,
            tags,
            targets: Vec::new(),
        };
        self.db_proxies.insert(name, proxy.clone());
        Ok(proxy)
    }

    pub fn describe_db_proxies(&self, name: Option<&str>) -> Vec<&DbProxy> {
        if let Some(n) = name {
            return self.db_proxies.get(n).into_iter().collect();
        }
        self.db_proxies.values().collect()
    }

    pub fn delete_db_proxy(&mut self, name: &str) -> Result<DbProxy, RdsError> {
        self.db_proxies
            .remove(name)
            .ok_or_else(|| RdsError::not_found("DBProxy", name))
    }

    // -------------------------------------------------------------------------
    // Blue/Green Deployments
    // -------------------------------------------------------------------------

    pub fn create_blue_green_deployment(
        &mut self,
        name: String,
        source: Option<String>,
        tags: Vec<Tag>,
    ) -> Result<BlueGreenDeployment, RdsError> {
        let id = format!("bgd-{}", uuid::Uuid::new_v4().simple());
        let deploy = BlueGreenDeployment {
            blue_green_deployment_identifier: id.clone(),
            blue_green_deployment_name: name,
            source: source.clone(),
            target: None,
            status: "PROVISIONING".to_string(),
            status_details: None,
            create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            delete_time: None,
            tags,
        };
        self.blue_green_deployments.insert(id, deploy.clone());
        Ok(deploy)
    }

    pub fn describe_blue_green_deployments(
        &self,
        identifier: Option<&str>,
    ) -> Vec<&BlueGreenDeployment> {
        if let Some(id) = identifier {
            return self.blue_green_deployments.get(id).into_iter().collect();
        }
        self.blue_green_deployments.values().collect()
    }

    pub fn delete_blue_green_deployment(
        &mut self,
        identifier: &str,
    ) -> Result<BlueGreenDeployment, RdsError> {
        self.blue_green_deployments
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("BlueGreenDeployment", identifier))
    }

    pub fn switchover_blue_green_deployment(
        &mut self,
        identifier: &str,
    ) -> Result<BlueGreenDeployment, RdsError> {
        let d = self
            .blue_green_deployments
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("BlueGreenDeployment", identifier))?;
        d.status = "SWITCHOVER_COMPLETED".to_string();
        Ok(d.clone())
    }

    // -------------------------------------------------------------------------
    // DB Shard Groups
    // -------------------------------------------------------------------------

    pub fn create_db_shard_group(
        &mut self,
        identifier: String,
        db_cluster_identifier: String,
        max_acu: f64,
        min_acu: Option<f64>,
        publicly_accessible: bool,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbShardGroup, RdsError> {
        if self.db_shard_groups.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBShardGroup", &identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:shard-group:{identifier}");
        let resource_id = format!("shardgroup-{}", uuid::Uuid::new_v4().simple());
        let sg = DbShardGroup {
            db_shard_group_identifier: identifier.clone(),
            db_shard_group_resource_id: Some(resource_id),
            db_cluster_identifier,
            max_acu,
            min_acu,
            publicly_accessible,
            status: "available".to_string(),
            endpoint: None,
            db_shard_group_arn: Some(arn),
            tag_list: tags,
        };
        self.db_shard_groups.insert(identifier, sg.clone());
        Ok(sg)
    }

    pub fn describe_db_shard_groups(&self, identifier: Option<&str>) -> Vec<&DbShardGroup> {
        if let Some(id) = identifier {
            return self.db_shard_groups.get(id).into_iter().collect();
        }
        self.db_shard_groups.values().collect()
    }

    // -------------------------------------------------------------------------
    // DB Instance Automated Backups
    // -------------------------------------------------------------------------

    pub fn describe_db_instance_automated_backups(
        &self,
        identifier: Option<&str>,
    ) -> Vec<&DbInstance> {
        // Return instances that match - simplified implementation
        if let Some(id) = identifier {
            return self.db_instances.get(id).into_iter().collect();
        }
        self.db_instances.values().collect()
    }

    // -------------------------------------------------------------------------
    // Role Associations
    // -------------------------------------------------------------------------

    pub fn add_role_to_db_instance(
        &mut self,
        identifier: &str,
        role_arn: String,
    ) -> Result<(), RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        if !inst.associated_roles.contains(&role_arn) {
            inst.associated_roles.push(role_arn);
        }
        Ok(())
    }

    pub fn remove_role_from_db_instance(
        &mut self,
        identifier: &str,
        role_arn: &str,
    ) -> Result<(), RdsError> {
        let inst = self
            .db_instances
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))?;
        inst.associated_roles.retain(|r| r != role_arn);
        Ok(())
    }

    pub fn add_role_to_db_cluster(
        &mut self,
        identifier: &str,
        role_arn: String,
    ) -> Result<(), RdsError> {
        let cluster = self
            .db_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))?;
        if !cluster.associated_roles.contains(&role_arn) {
            cluster.associated_roles.push(role_arn);
        }
        Ok(())
    }

    pub fn remove_role_from_db_cluster(
        &mut self,
        identifier: &str,
        role_arn: &str,
    ) -> Result<(), RdsError> {
        let cluster = self
            .db_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))?;
        cluster.associated_roles.retain(|r| r != role_arn);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // CopyDBParameterGroup
    // -------------------------------------------------------------------------

    pub fn copy_db_parameter_group(
        &mut self,
        source_name: &str,
        target_name: String,
        target_description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbParameterGroup, RdsError> {
        let source = self
            .db_parameter_groups
            .get(source_name)
            .ok_or_else(|| RdsError::not_found("DBParameterGroup", source_name))?
            .clone();
        if self.db_parameter_groups.contains_key(&target_name) {
            return Err(RdsError::already_exists("DBParameterGroup", &target_name));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:pg:{target_name}");
        let pg = DbParameterGroup {
            name: target_name.clone(),
            family: source.family,
            description: target_description,
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_parameter_groups.insert(target_name, pg.clone());
        Ok(pg)
    }

    // -------------------------------------------------------------------------
    // DB Proxy Target Groups
    // -------------------------------------------------------------------------

    pub fn register_db_proxy_targets(
        &mut self,
        db_proxy_name: &str,
        target_group_name: &str,
        db_instance_identifiers: Vec<String>,
        db_cluster_identifiers: Vec<String>,
        account_id: &str,
        region: &str,
    ) -> Result<Vec<crate::types::DbProxyTarget>, RdsError> {
        // Ensure proxy exists
        if !self.db_proxies.contains_key(db_proxy_name) {
            return Err(RdsError::not_found("DBProxy", db_proxy_name));
        }
        // Ensure or create target group
        let key = format!("{db_proxy_name}/{target_group_name}");
        self.db_proxy_target_groups.entry(key.clone()).or_insert_with(|| {
            let arn = format!("arn:aws:rds:{region}:{account_id}:target-group:{db_proxy_name}/{target_group_name}");
            DbProxyTargetGroup {
                target_group_name: target_group_name.to_string(),
                db_proxy_name: db_proxy_name.to_string(),
                target_group_arn: arn,
                is_default: target_group_name == "default",
                status: "available".to_string(),
                connection_pool_config: None,
                created_date: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                updated_date: None,
            }
        });
        // Build registered targets
        let proxy = self.db_proxies.get_mut(db_proxy_name).unwrap();
        let mut new_targets = Vec::new();
        for id in &db_instance_identifiers {
            let target = crate::types::DbProxyTarget {
                target_arn: format!("arn:aws:rds:{region}:{account_id}:db:{id}"),
                endpoint: None,
                tracked_cluster_id: None,
                rds_resource_id: Some(id.clone()),
                port: Some(3306),
                type_: Some("RDS_INSTANCE".to_string()),
                role: Some("READ_WRITE".to_string()),
                target_health_status: Some("AVAILABLE".to_string()),
            };
            new_targets.push(target.clone());
            proxy.targets.push(target);
        }
        for id in &db_cluster_identifiers {
            let target = crate::types::DbProxyTarget {
                target_arn: format!("arn:aws:rds:{region}:{account_id}:cluster:{id}"),
                endpoint: None,
                tracked_cluster_id: Some(id.clone()),
                rds_resource_id: Some(id.clone()),
                port: Some(3306),
                type_: Some("TRACKED_CLUSTER".to_string()),
                role: Some("READ_WRITE".to_string()),
                target_health_status: Some("AVAILABLE".to_string()),
            };
            new_targets.push(target.clone());
            proxy.targets.push(target);
        }
        Ok(new_targets)
    }

    pub fn deregister_db_proxy_targets(
        &mut self,
        db_proxy_name: &str,
        db_instance_identifiers: Vec<String>,
        db_cluster_identifiers: Vec<String>,
    ) -> Result<(), RdsError> {
        let proxy = self
            .db_proxies
            .get_mut(db_proxy_name)
            .ok_or_else(|| RdsError::not_found("DBProxy", db_proxy_name))?;
        proxy.targets.retain(|t| {
            let rid = t.rds_resource_id.as_deref().unwrap_or("");
            !db_instance_identifiers.iter().any(|id| id == rid)
                && !db_cluster_identifiers.iter().any(|id| id == rid)
        });
        Ok(())
    }

    pub fn describe_db_proxy_targets(
        &self,
        db_proxy_name: &str,
    ) -> Result<Vec<&crate::types::DbProxyTarget>, RdsError> {
        let proxy = self
            .db_proxies
            .get(db_proxy_name)
            .ok_or_else(|| RdsError::not_found("DBProxy", db_proxy_name))?;
        Ok(proxy.targets.iter().collect())
    }

    pub fn describe_db_proxy_target_groups(
        &self,
        db_proxy_name: &str,
        target_group_name: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<Vec<DbProxyTargetGroup>, RdsError> {
        if !self.db_proxies.contains_key(db_proxy_name) {
            return Err(RdsError::not_found("DBProxy", db_proxy_name));
        }
        // Collect matching groups, or create a default one if none exist
        let matching: Vec<DbProxyTargetGroup> = self
            .db_proxy_target_groups
            .values()
            .filter(|g| {
                g.db_proxy_name == db_proxy_name
                    && target_group_name.is_none_or(|n| g.target_group_name == n)
            })
            .cloned()
            .collect();
        if matching.is_empty() {
            // Return a synthetic default group
            let name = target_group_name.unwrap_or("default");
            let arn =
                format!("arn:aws:rds:{region}:{account_id}:target-group:{db_proxy_name}/{name}");
            Ok(vec![DbProxyTargetGroup {
                target_group_name: name.to_string(),
                db_proxy_name: db_proxy_name.to_string(),
                target_group_arn: arn,
                is_default: name == "default",
                status: "available".to_string(),
                connection_pool_config: None,
                created_date: None,
                updated_date: None,
            }])
        } else {
            Ok(matching)
        }
    }

    pub fn modify_db_proxy_target_group(
        &mut self,
        db_proxy_name: &str,
        target_group_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<DbProxyTargetGroup, RdsError> {
        if !self.db_proxies.contains_key(db_proxy_name) {
            return Err(RdsError::not_found("DBProxy", db_proxy_name));
        }
        let key = format!("{db_proxy_name}/{target_group_name}");
        let group = self.db_proxy_target_groups.entry(key).or_insert_with(|| {
            let arn = format!(
                "arn:aws:rds:{region}:{account_id}:target-group:{db_proxy_name}/{target_group_name}"
            );
            DbProxyTargetGroup {
                target_group_name: target_group_name.to_string(),
                db_proxy_name: db_proxy_name.to_string(),
                target_group_arn: arn,
                is_default: target_group_name == "default",
                status: "available".to_string(),
                connection_pool_config: None,
                created_date: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
                updated_date: None,
            }
        });
        group.updated_date = Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string());
        Ok(group.clone())
    }

    // -------------------------------------------------------------------------
    // Modify / Reboot DB Shard Group
    // -------------------------------------------------------------------------

    pub fn modify_db_shard_group(
        &mut self,
        identifier: &str,
        max_acu: Option<f64>,
        min_acu: Option<f64>,
    ) -> Result<DbShardGroup, RdsError> {
        let sg = self
            .db_shard_groups
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBShardGroup", identifier))?;
        if let Some(v) = max_acu {
            sg.max_acu = v;
        }
        if let Some(v) = min_acu {
            sg.min_acu = Some(v);
        }
        Ok(sg.clone())
    }

    pub fn reboot_db_shard_group(&self, identifier: &str) -> Result<DbShardGroup, RdsError> {
        self.db_shard_groups
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBShardGroup", identifier))
    }

    // -------------------------------------------------------------------------
    // Backtrack DB Cluster
    // -------------------------------------------------------------------------

    pub fn backtrack_db_cluster(&self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // HTTP Endpoint (enable/disable on Aurora Serverless)
    // -------------------------------------------------------------------------

    pub fn enable_http_endpoint(&mut self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    pub fn disable_http_endpoint(&mut self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // Modify Current DB Cluster Capacity
    // -------------------------------------------------------------------------

    pub fn modify_current_db_cluster_capacity(
        &self,
        identifier: &str,
    ) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // Restore DB Cluster/Instance from S3
    // -------------------------------------------------------------------------

    pub fn restore_db_cluster_from_s3(
        &mut self,
        identifier: String,
        engine: String,
        master_username: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<DbCluster, RdsError> {
        if self.db_clusters.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBCluster", &identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster:{identifier}");
        let cluster = DbCluster {
            identifier: identifier.clone(),
            engine,
            engine_version: None,
            status: "available".to_string(),
            endpoint: Some(format!(
                "{identifier}.cluster-{account_id}.{region}.rds.amazonaws.com"
            )),
            reader_endpoint: None,
            port: Some(3306),
            master_username,
            database_name: None,
            db_subnet_group_name: None,
            vpc_security_group_ids: vec![],
            availability_zones: vec![],
            arn,
            tags: vec![],
            cluster_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            multi_az: false,
            storage_type: Some("aurora".to_string()),
            allocated_storage: None,
            backup_retention_period: 1,
            deletion_protection: false,
            storage_encrypted: false,
            kms_key_id: None,
            db_cluster_parameter_group: None,
            engine_mode: Some("provisioned".to_string()),
            copy_tags_to_snapshot: false,
            members: vec![],
            associated_roles: vec![],
        };
        self.db_clusters.insert(identifier, cluster.clone());
        Ok(cluster)
    }

    pub fn restore_db_instance_from_s3(
        &mut self,
        identifier: String,
        db_instance_class: String,
        engine: String,
        master_username: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<DbInstance, RdsError> {
        if self.db_instances.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBInstance", &identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:db:{identifier}");
        let inst = DbInstance {
            identifier: identifier.clone(),
            db_instance_class,
            engine,
            engine_version: "8.0".to_string(),
            status: "available".to_string(),
            master_username,
            db_name: None,
            endpoint_address: Some(format!(
                "{identifier}.{account_id}.{region}.rds.amazonaws.com"
            )),
            port: Some(3306),
            multi_az: false,
            storage_type: Some("gp2".to_string()),
            allocated_storage: 20,
            db_subnet_group_name: None,
            vpc_security_group_ids: vec![],
            db_parameter_group_names: vec![],
            availability_zone: None,
            publicly_accessible: false,
            auto_minor_version_upgrade: true,
            backup_retention_period: 1,
            db_cluster_identifier: None,
            arn,
            tags: vec![],
            instance_create_time: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
            license_model: None,
            iops: None,
            deletion_protection: false,
            copy_tags_to_snapshot: false,
            monitoring_interval: None,
            performance_insights_enabled: false,
            storage_encrypted: false,
            kms_key_id: None,
            ca_certificate_identifier: None,
            secondary_availability_zone: None,
            associated_roles: vec![],
        };
        self.db_instances.insert(identifier, inst.clone());
        Ok(inst)
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
    ) -> Result<DbClusterEndpoint, RdsError> {
        if self.db_cluster_endpoints.contains_key(&identifier) {
            return Err(RdsError::already_exists("DBClusterEndpoint", &identifier));
        }
        if !self.db_clusters.contains_key(&db_cluster_identifier) {
            return Err(RdsError::not_found("DBCluster", &db_cluster_identifier));
        }
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster-endpoint:{identifier}");
        let ep = DbClusterEndpoint {
            db_cluster_endpoint_identifier: identifier.clone(),
            db_cluster_identifier,
            db_cluster_endpoint_arn: arn,
            endpoint: format!("{identifier}.cluster-custom-endpoint.{region}.rds.amazonaws.com"),
            status: "available".to_string(),
            endpoint_type: "CUSTOM".to_string(),
            custom_endpoint_type: Some(endpoint_type),
            static_members,
            excluded_members,
        };
        self.db_cluster_endpoints.insert(identifier, ep.clone());
        Ok(ep)
    }

    pub fn describe_db_cluster_endpoints(
        &self,
        db_cluster_identifier: Option<&str>,
        endpoint_identifier: Option<&str>,
    ) -> Vec<&DbClusterEndpoint> {
        self.db_cluster_endpoints
            .values()
            .filter(|ep| {
                db_cluster_identifier.is_none_or(|id| ep.db_cluster_identifier == id)
                    && endpoint_identifier.is_none_or(|id| ep.db_cluster_endpoint_identifier == id)
            })
            .collect()
    }

    pub fn modify_db_cluster_endpoint(
        &mut self,
        identifier: &str,
        static_members: Option<Vec<String>>,
        excluded_members: Option<Vec<String>>,
    ) -> Result<DbClusterEndpoint, RdsError> {
        let ep = self
            .db_cluster_endpoints
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("DBClusterEndpoint", identifier))?;
        if let Some(members) = static_members {
            ep.static_members = members;
        }
        if let Some(members) = excluded_members {
            ep.excluded_members = members;
        }
        Ok(ep.clone())
    }

    pub fn delete_db_cluster_endpoint(
        &mut self,
        identifier: &str,
    ) -> Result<DbClusterEndpoint, RdsError> {
        self.db_cluster_endpoints
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("DBClusterEndpoint", identifier))
    }

    // -------------------------------------------------------------------------
    // DB Proxy Endpoints
    // -------------------------------------------------------------------------

    pub fn create_db_proxy_endpoint(
        &mut self,
        name: String,
        db_proxy_name: String,
        vpc_subnet_ids: Vec<String>,
        vpc_security_group_ids: Vec<String>,
        target_role: Option<String>,
        account_id: &str,
        region: &str,
    ) -> Result<DbProxyEndpoint, RdsError> {
        if self.db_proxy_endpoints.contains_key(&name) {
            return Err(RdsError::already_exists("DBProxyEndpoint", &name));
        }
        let proxy = self
            .db_proxies
            .get(&db_proxy_name)
            .ok_or_else(|| RdsError::not_found("DBProxy", &db_proxy_name))?;
        let vpc_id = proxy.vpc_id.clone();
        let arn = format!("arn:aws:rds:{region}:{account_id}:db-proxy-endpoint:{name}");
        let role = target_role.unwrap_or_else(|| "READ_WRITE".to_string());
        let ep = DbProxyEndpoint {
            db_proxy_endpoint_name: name.clone(),
            db_proxy_endpoint_arn: arn,
            db_proxy_name,
            status: "available".to_string(),
            vpc_id,
            vpc_security_group_ids,
            vpc_subnet_ids,
            endpoint: format!("{name}.endpoint.proxy-{region}.rds.amazonaws.com"),
            is_default: false,
            target_role: role,
            created_date: Some(chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()),
        };
        self.db_proxy_endpoints.insert(name, ep.clone());
        Ok(ep)
    }

    pub fn describe_db_proxy_endpoints(
        &self,
        db_proxy_name: Option<&str>,
        endpoint_name: Option<&str>,
    ) -> Vec<&DbProxyEndpoint> {
        self.db_proxy_endpoints
            .values()
            .filter(|ep| {
                db_proxy_name.is_none_or(|n| ep.db_proxy_name == n)
                    && endpoint_name.is_none_or(|n| ep.db_proxy_endpoint_name == n)
            })
            .collect()
    }

    pub fn modify_db_proxy_endpoint(
        &mut self,
        name: &str,
        vpc_security_group_ids: Option<Vec<String>>,
    ) -> Result<DbProxyEndpoint, RdsError> {
        let ep = self
            .db_proxy_endpoints
            .get_mut(name)
            .ok_or_else(|| RdsError::not_found("DBProxyEndpoint", name))?;
        if let Some(sgs) = vpc_security_group_ids {
            ep.vpc_security_group_ids = sgs;
        }
        Ok(ep.clone())
    }

    pub fn delete_db_proxy_endpoint(&mut self, name: &str) -> Result<DbProxyEndpoint, RdsError> {
        self.db_proxy_endpoints
            .remove(name)
            .ok_or_else(|| RdsError::not_found("DBProxyEndpoint", name))
    }

    // -------------------------------------------------------------------------
    // Copy operations
    // -------------------------------------------------------------------------

    pub fn copy_db_cluster_parameter_group(
        &mut self,
        source_name: &str,
        target_name: String,
        target_description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<DbClusterParameterGroup, RdsError> {
        if self.db_cluster_parameter_groups.contains_key(&target_name) {
            return Err(RdsError::already_exists(
                "DBClusterParameterGroup",
                &target_name,
            ));
        }
        let source = self
            .db_cluster_parameter_groups
            .get(source_name)
            .ok_or_else(|| RdsError::not_found("DBClusterParameterGroup", source_name))?
            .clone();
        let arn = format!("arn:aws:rds:{region}:{account_id}:cluster-pg:{target_name}");
        let pg = DbClusterParameterGroup {
            name: target_name.clone(),
            family: source.family.clone(),
            description: target_description,
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.db_cluster_parameter_groups
            .insert(target_name, pg.clone());
        Ok(pg)
    }

    pub fn copy_option_group(
        &mut self,
        source_name: &str,
        target_name: String,
        target_description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<OptionGroup, RdsError> {
        if self.option_groups.contains_key(&target_name) {
            return Err(RdsError::already_exists("OptionGroup", &target_name));
        }
        let source = self
            .option_groups
            .get(source_name)
            .ok_or_else(|| RdsError::not_found("OptionGroup", source_name))?
            .clone();
        let arn = format!("arn:aws:rds:{region}:{account_id}:og:{target_name}");
        let og = OptionGroup {
            name: target_name.clone(),
            engine_name: source.engine_name.clone(),
            major_engine_version: source.major_engine_version.clone(),
            description: target_description,
            allows_vpc_and_non_vpc_instance_memberships: source
                .allows_vpc_and_non_vpc_instance_memberships,
            vpc_id: source.vpc_id.clone(),
            arn: arn.clone(),
            tags: tags.clone(),
        };
        self.resource_tags.insert(arn, tags);
        self.option_groups.insert(target_name, og.clone());
        Ok(og)
    }

    // -------------------------------------------------------------------------
    // Event subscription: add/remove source identifiers
    // -------------------------------------------------------------------------

    pub fn add_source_identifier_to_subscription(
        &mut self,
        subscription_name: &str,
        source_identifier: String,
    ) -> Result<EventSubscription, RdsError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| RdsError::not_found("SubscriptionNotFound", subscription_name))?;
        if !sub.source_ids.contains(&source_identifier) {
            sub.source_ids.push(source_identifier);
        }
        Ok(sub.clone())
    }

    pub fn remove_source_identifier_from_subscription(
        &mut self,
        subscription_name: &str,
        source_identifier: &str,
    ) -> Result<EventSubscription, RdsError> {
        let sub = self
            .event_subscriptions
            .get_mut(subscription_name)
            .ok_or_else(|| RdsError::not_found("SubscriptionNotFound", subscription_name))?;
        sub.source_ids.retain(|id| id != source_identifier);
        Ok(sub.clone())
    }

    // -------------------------------------------------------------------------
    // DB Security Groups: authorize/revoke ingress
    // -------------------------------------------------------------------------

    pub fn authorize_db_security_group_ingress(
        &mut self,
        name: &str,
        cidrip: Option<String>,
        ec2_sg_name: Option<String>,
        ec2_sg_id: Option<String>,
        ec2_sg_owner_id: Option<String>,
    ) -> Result<crate::types::DbSecurityGroup, RdsError> {
        let sg = self
            .db_security_groups
            .get_mut(name)
            .ok_or_else(|| RdsError::not_found("DBSecurityGroup", name))?;
        if let Some(cidr) = cidrip {
            sg.ip_ranges.push(crate::types::IPRange {
                status: "authorized".to_string(),
                cidrip: cidr,
            });
        }
        if ec2_sg_name.is_some() || ec2_sg_id.is_some() {
            sg.ec2_security_groups.push(crate::types::EC2SecurityGroup {
                status: "authorized".to_string(),
                ec2_security_group_name: ec2_sg_name,
                ec2_security_group_id: ec2_sg_id,
                ec2_security_group_owner_id: ec2_sg_owner_id,
            });
        }
        Ok(sg.clone())
    }

    pub fn revoke_db_security_group_ingress(
        &mut self,
        name: &str,
        cidrip: Option<&str>,
        ec2_sg_name: Option<&str>,
        ec2_sg_id: Option<&str>,
    ) -> Result<crate::types::DbSecurityGroup, RdsError> {
        let sg = self
            .db_security_groups
            .get_mut(name)
            .ok_or_else(|| RdsError::not_found("DBSecurityGroup", name))?;
        if let Some(cidr) = cidrip {
            sg.ip_ranges.retain(|r| r.cidrip != cidr);
        }
        if let Some(sg_name) = ec2_sg_name {
            sg.ec2_security_groups
                .retain(|g| g.ec2_security_group_name.as_deref() != Some(sg_name));
        }
        if let Some(sg_id) = ec2_sg_id {
            sg.ec2_security_groups
                .retain(|g| g.ec2_security_group_id.as_deref() != Some(sg_id));
        }
        Ok(sg.clone())
    }

    // -------------------------------------------------------------------------
    // Global Cluster operations
    // -------------------------------------------------------------------------

    pub fn modify_global_cluster(
        &mut self,
        identifier: &str,
        new_identifier: Option<String>,
        deletion_protection: Option<bool>,
        engine_version: Option<String>,
    ) -> Result<GlobalCluster, RdsError> {
        let gc = self
            .global_clusters
            .get_mut(identifier)
            .ok_or_else(|| RdsError::not_found("GlobalCluster", identifier))?;
        if let Some(dp) = deletion_protection {
            gc.deletion_protection = dp;
        }
        if let Some(ev) = engine_version {
            gc.engine_version = Some(ev);
        }
        let updated = gc.clone();
        if let Some(new_id) = new_identifier {
            if new_id != identifier {
                let mut gc2 = updated.clone();
                gc2.global_cluster_identifier = new_id.clone();
                self.global_clusters.remove(identifier);
                self.global_clusters.insert(new_id, gc2.clone());
                return Ok(gc2);
            }
        }
        Ok(updated)
    }

    pub fn failover_global_cluster(&self, identifier: &str) -> Result<GlobalCluster, RdsError> {
        self.global_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("GlobalCluster", identifier))
    }

    pub fn switchover_global_cluster(&self, identifier: &str) -> Result<GlobalCluster, RdsError> {
        self.global_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("GlobalCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // DB Cluster reboot
    // -------------------------------------------------------------------------

    pub fn reboot_db_cluster(&self, identifier: &str) -> Result<DbCluster, RdsError> {
        self.db_clusters
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBCluster", identifier))
    }

    // -------------------------------------------------------------------------
    // Delete DB Shard Group
    // -------------------------------------------------------------------------

    pub fn delete_db_shard_group(&mut self, identifier: &str) -> Result<DbShardGroup, RdsError> {
        self.db_shard_groups
            .remove(identifier)
            .ok_or_else(|| RdsError::not_found("DBShardGroup", identifier))
    }

    // -------------------------------------------------------------------------
    // Modify DB Proxy
    // -------------------------------------------------------------------------

    pub fn modify_db_proxy(
        &mut self,
        name: &str,
        new_name: Option<String>,
        require_tls: Option<bool>,
        idle_client_timeout: Option<i32>,
        debug_logging: Option<bool>,
        role_arn: Option<String>,
        security_groups: Option<Vec<String>>,
    ) -> Result<DbProxy, RdsError> {
        let proxy = self
            .db_proxies
            .get_mut(name)
            .ok_or_else(|| RdsError::not_found("DBProxy", name))?;
        if let Some(v) = require_tls {
            proxy.require_tls = v;
        }
        if let Some(v) = idle_client_timeout {
            proxy.idle_client_timeout = v;
        }
        if let Some(v) = debug_logging {
            proxy.debug_logging = v;
        }
        if let Some(v) = role_arn {
            proxy.role_arn = v;
        }
        if let Some(v) = security_groups {
            proxy.vpc_security_group_ids = v;
        }
        let updated = proxy.clone();
        if let Some(new_n) = new_name {
            if new_n != name {
                self.db_proxies.remove(name);
                self.db_proxies.insert(new_n, updated.clone());
                return Ok(updated);
            }
        }
        Ok(updated)
    }

    // -------------------------------------------------------------------------
    // Modify DB Snapshot
    // -------------------------------------------------------------------------

    pub fn modify_db_snapshot(
        &mut self,
        snapshot_id: &str,
        engine_version: Option<String>,
        option_group_name: Option<String>,
    ) -> Result<DbSnapshot, RdsError> {
        let snap = self
            .db_snapshots
            .get_mut(snapshot_id)
            .ok_or_else(|| RdsError::not_found("DBSnapshot", snapshot_id))?;
        if let Some(ev) = engine_version {
            snap.engine_version = Some(ev);
        }
        if let Some(og) = option_group_name {
            snap.option_group_name = Some(og);
        }
        Ok(snap.clone())
    }

    // -------------------------------------------------------------------------
    // Switchover Read Replica
    // -------------------------------------------------------------------------

    pub fn switchover_read_replica(&self, identifier: &str) -> Result<DbInstance, RdsError> {
        self.db_instances
            .get(identifier)
            .cloned()
            .ok_or_else(|| RdsError::not_found("DBInstance", identifier))
    }

    // -------------------------------------------------------------------------
    // Apply Pending Maintenance Action
    // -------------------------------------------------------------------------

    pub fn apply_pending_maintenance_action(
        &self,
        resource_identifier: &str,
    ) -> Result<String, RdsError> {
        // Find the resource by ARN or identifier
        // For simplicity, just verify it's a known resource
        let found = self
            .db_instances
            .values()
            .any(|i| i.arn == resource_identifier || i.identifier == resource_identifier)
            || self
                .db_clusters
                .values()
                .any(|c| c.arn == resource_identifier || c.identifier == resource_identifier);
        if found {
            Ok(resource_identifier.to_string())
        } else {
            Err(RdsError::not_found("Resource", resource_identifier))
        }
    }
}
