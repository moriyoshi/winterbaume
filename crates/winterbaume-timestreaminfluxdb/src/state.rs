use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

/// In-memory state for the Timestream for InfluxDB service.
#[derive(Debug, Default)]
pub struct TimestreamInfluxDbState {
    /// DB instances keyed by ID.
    pub db_instances: HashMap<String, DbInstance>,
    /// DB clusters keyed by ID.
    pub db_clusters: HashMap<String, DbCluster>,
    /// DB parameter groups keyed by ID.
    pub db_parameter_groups: HashMap<String, DbParameterGroup>,
}

/// Error type for Timestream InfluxDB operations.
#[derive(Debug, Error)]
pub enum TsInfluxError {
    #[error("A DB instance with the name {name} already exists")]
    DbInstanceNameConflict { name: String },

    #[error("A DB cluster with the name {name} already exists")]
    DbClusterNameConflict { name: String },

    #[error("A DB parameter group with the name {name} already exists")]
    DbParameterGroupNameConflict { name: String },

    #[error("The DB instance with identifier '{identifier}' does not exist")]
    DbInstanceNotFound { identifier: String },

    #[error("The DB cluster with identifier '{identifier}' does not exist")]
    DbClusterNotFound { identifier: String },

    #[error("The DB parameter group with identifier '{identifier}' does not exist")]
    DbParameterGroupNotFound { identifier: String },

    #[error("Resource with ARN '{arn}' not found")]
    ResourceArnNotFound { arn: String },
}

impl TimestreamInfluxDbState {
    pub fn create_db_instance(
        &mut self,
        name: &str,
        db_instance_type: &str,
        allocated_storage: i32,
        vpc_subnet_ids: Vec<String>,
        vpc_security_group_ids: Vec<String>,
        publicly_accessible: Option<bool>,
        db_storage_type: Option<&str>,
        db_parameter_group_identifier: Option<&str>,
        deployment_type: Option<&str>,
        port: Option<i32>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&DbInstance, TsInfluxError> {
        // Check for duplicate name
        if self.db_instances.values().any(|i| i.name == name) {
            return Err(TsInfluxError::DbInstanceNameConflict {
                name: name.to_string(),
            });
        }

        let id = generate_id();
        let arn = format!("arn:aws:timestream-influxdb:{region}:{account_id}:db-instance/{id}");
        let endpoint = format!("{name}-{id}.timestream-influxdb.{region}.amazonaws.com");
        let az = format!("{region}a");

        let instance = DbInstance {
            id: id.clone(),
            name: name.to_string(),
            arn,
            status: "CREATING".to_string(),
            endpoint: Some(endpoint),
            port,
            db_instance_type: db_instance_type.to_string(),
            db_storage_type: db_storage_type.map(|s| s.to_string()),
            allocated_storage,
            deployment_type: deployment_type.map(|s| s.to_string()),
            vpc_subnet_ids,
            vpc_security_group_ids,
            publicly_accessible,
            db_parameter_group_identifier: db_parameter_group_identifier.map(|s| s.to_string()),
            availability_zone: Some(az),
            tags,
        };

        self.db_instances.insert(id.clone(), instance);
        Ok(self.db_instances.get(&id).unwrap())
    }

    pub fn get_db_instance(&self, identifier: &str) -> Result<&DbInstance, TsInfluxError> {
        self.db_instances
            .get(identifier)
            .ok_or_else(|| TsInfluxError::DbInstanceNotFound {
                identifier: identifier.to_string(),
            })
    }

    pub fn delete_db_instance(&mut self, identifier: &str) -> Result<DbInstance, TsInfluxError> {
        match self.db_instances.remove(identifier) {
            Some(mut instance) => {
                instance.status = "DELETING".to_string();
                Ok(instance)
            }
            None => Err(TsInfluxError::DbInstanceNotFound {
                identifier: identifier.to_string(),
            }),
        }
    }

    pub fn list_db_instances(&self) -> Vec<DbInstanceSummary> {
        self.db_instances
            .values()
            .map(|i| DbInstanceSummary {
                id: i.id.clone(),
                name: i.name.clone(),
                arn: i.arn.clone(),
                status: Some(i.status.clone()),
                endpoint: i.endpoint.clone(),
                port: i.port,
                db_instance_type: Some(i.db_instance_type.clone()),
                db_storage_type: i.db_storage_type.clone(),
                allocated_storage: Some(i.allocated_storage),
                deployment_type: i.deployment_type.clone(),
            })
            .collect()
    }

    // --- DbCluster methods ---

    pub fn create_db_cluster(
        &mut self,
        name: &str,
        db_instance_type: Option<&str>,
        allocated_storage: Option<i32>,
        vpc_subnet_ids: Vec<String>,
        vpc_security_group_ids: Vec<String>,
        publicly_accessible: Option<bool>,
        db_storage_type: Option<&str>,
        db_parameter_group_identifier: Option<&str>,
        deployment_type: Option<&str>,
        network_type: Option<&str>,
        failover_mode: Option<&str>,
        port: Option<i32>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&DbCluster, TsInfluxError> {
        if self.db_clusters.values().any(|c| c.name == name) {
            return Err(TsInfluxError::DbClusterNameConflict {
                name: name.to_string(),
            });
        }

        let id = generate_id();
        let arn = format!("arn:aws:timestream-influxdb:{region}:{account_id}:db-cluster/{id}");
        let endpoint = format!("{name}-{id}.timestream-influxdb.{region}.amazonaws.com");
        let reader_ep = format!("{name}-{id}-ro.timestream-influxdb.{region}.amazonaws.com");

        let cluster = DbCluster {
            id: id.clone(),
            name: name.to_string(),
            arn,
            status: "CREATING".to_string(),
            endpoint: Some(endpoint),
            reader_endpoint: Some(reader_ep),
            port,
            deployment_type: deployment_type.map(|s| s.to_string()),
            db_instance_type: db_instance_type.map(|s| s.to_string()),
            network_type: network_type.map(|s| s.to_string()),
            db_storage_type: db_storage_type.map(|s| s.to_string()),
            allocated_storage,
            publicly_accessible,
            db_parameter_group_identifier: db_parameter_group_identifier.map(|s| s.to_string()),
            vpc_subnet_ids,
            vpc_security_group_ids,
            failover_mode: failover_mode.map(|s| s.to_string()),
            tags,
        };

        self.db_clusters.insert(id.clone(), cluster);
        Ok(self.db_clusters.get(&id).unwrap())
    }

    pub fn get_db_cluster(&self, identifier: &str) -> Result<&DbCluster, TsInfluxError> {
        self.db_clusters
            .get(identifier)
            .ok_or_else(|| TsInfluxError::DbClusterNotFound {
                identifier: identifier.to_string(),
            })
    }

    pub fn delete_db_cluster(&mut self, identifier: &str) -> Result<DbCluster, TsInfluxError> {
        match self.db_clusters.remove(identifier) {
            Some(mut cluster) => {
                cluster.status = "DELETING".to_string();
                Ok(cluster)
            }
            None => Err(TsInfluxError::DbClusterNotFound {
                identifier: identifier.to_string(),
            }),
        }
    }

    pub fn list_db_instances_for_cluster(
        &self,
        cluster_id: &str,
    ) -> Result<Vec<DbInstanceSummary>, TsInfluxError> {
        // Verify the cluster exists first
        if !self.db_clusters.contains_key(cluster_id) {
            return Err(TsInfluxError::DbClusterNotFound {
                identifier: cluster_id.to_string(),
            });
        }
        // In this mock, instances don't track which cluster they belong to,
        // so we return all instances (a simple approximation).
        let summaries = self.list_db_instances();
        Ok(summaries)
    }

    pub fn reboot_db_cluster(&mut self, identifier: &str) -> Result<&DbCluster, TsInfluxError> {
        if let Some(cluster) = self.db_clusters.get_mut(identifier) {
            cluster.status = "REBOOTING".to_string();
            Ok(self.db_clusters.get(identifier).unwrap())
        } else {
            Err(TsInfluxError::DbClusterNotFound {
                identifier: identifier.to_string(),
            })
        }
    }

    pub fn reboot_db_instance(&mut self, identifier: &str) -> Result<&DbInstance, TsInfluxError> {
        if let Some(instance) = self.db_instances.get_mut(identifier) {
            instance.status = "REBOOTING".to_string();
            Ok(self.db_instances.get(identifier).unwrap())
        } else {
            Err(TsInfluxError::DbInstanceNotFound {
                identifier: identifier.to_string(),
            })
        }
    }

    pub fn update_db_cluster(
        &mut self,
        identifier: &str,
        db_parameter_group_identifier: Option<&str>,
        failover_mode: Option<&str>,
        port: Option<i32>,
    ) -> Result<&DbCluster, TsInfluxError> {
        if let Some(cluster) = self.db_clusters.get_mut(identifier) {
            if let Some(pg) = db_parameter_group_identifier {
                cluster.db_parameter_group_identifier = Some(pg.to_string());
            }
            if let Some(fm) = failover_mode {
                cluster.failover_mode = Some(fm.to_string());
            }
            if let Some(p) = port {
                cluster.port = Some(p);
            }
            Ok(self.db_clusters.get(identifier).unwrap())
        } else {
            Err(TsInfluxError::DbClusterNotFound {
                identifier: identifier.to_string(),
            })
        }
    }

    pub fn update_db_instance(
        &mut self,
        identifier: &str,
        db_parameter_group_identifier: Option<&str>,
        db_storage_type: Option<&str>,
        allocated_storage: Option<i32>,
        deployment_type: Option<&str>,
        port: Option<i32>,
    ) -> Result<&DbInstance, TsInfluxError> {
        if let Some(instance) = self.db_instances.get_mut(identifier) {
            if let Some(pg) = db_parameter_group_identifier {
                instance.db_parameter_group_identifier = Some(pg.to_string());
            }
            if let Some(st) = db_storage_type {
                instance.db_storage_type = Some(st.to_string());
            }
            if let Some(s) = allocated_storage {
                instance.allocated_storage = s;
            }
            if let Some(dt) = deployment_type {
                instance.deployment_type = Some(dt.to_string());
            }
            if let Some(p) = port {
                instance.port = Some(p);
            }
            Ok(self.db_instances.get(identifier).unwrap())
        } else {
            Err(TsInfluxError::DbInstanceNotFound {
                identifier: identifier.to_string(),
            })
        }
    }

    pub fn list_db_clusters(&self) -> Vec<DbClusterSummary> {
        self.db_clusters
            .values()
            .map(|c| DbClusterSummary {
                id: c.id.clone(),
                name: c.name.clone(),
                arn: c.arn.clone(),
                status: Some(c.status.clone()),
                endpoint: c.endpoint.clone(),
                reader_endpoint: c.reader_endpoint.clone(),
                port: c.port,
                deployment_type: c.deployment_type.clone(),
                db_instance_type: c.db_instance_type.clone(),
                network_type: c.network_type.clone(),
                db_storage_type: c.db_storage_type.clone(),
                allocated_storage: c.allocated_storage,
            })
            .collect()
    }

    // --- DbParameterGroup methods ---

    pub fn create_db_parameter_group(
        &mut self,
        name: &str,
        description: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&DbParameterGroup, TsInfluxError> {
        if self.db_parameter_groups.values().any(|pg| pg.name == name) {
            return Err(TsInfluxError::DbParameterGroupNameConflict {
                name: name.to_string(),
            });
        }

        let id = generate_id();
        let arn =
            format!("arn:aws:timestream-influxdb:{region}:{account_id}:db-parameter-group/{id}");

        let pg = DbParameterGroup {
            id: id.clone(),
            name: name.to_string(),
            arn,
            description: description.map(|s| s.to_string()),
            tags,
        };

        self.db_parameter_groups.insert(id.clone(), pg);
        Ok(self.db_parameter_groups.get(&id).unwrap())
    }

    pub fn get_db_parameter_group(
        &self,
        identifier: &str,
    ) -> Result<&DbParameterGroup, TsInfluxError> {
        self.db_parameter_groups.get(identifier).ok_or_else(|| {
            TsInfluxError::DbParameterGroupNotFound {
                identifier: identifier.to_string(),
            }
        })
    }

    pub fn list_db_parameter_groups(&self) -> Vec<DbParameterGroupSummary> {
        self.db_parameter_groups
            .values()
            .map(|pg| DbParameterGroupSummary {
                id: pg.id.clone(),
                name: pg.name.clone(),
                arn: pg.arn.clone(),
                description: pg.description.clone(),
            })
            .collect()
    }

    // --- Tag operations ---

    pub fn find_resource_tags_mut(
        &mut self,
        resource_arn: &str,
    ) -> Result<&mut HashMap<String, String>, TsInfluxError> {
        // Check instances
        if let Some(inst) = self
            .db_instances
            .values_mut()
            .find(|i| i.arn == resource_arn)
        {
            return Ok(&mut inst.tags);
        }
        // Check clusters
        if let Some(cluster) = self
            .db_clusters
            .values_mut()
            .find(|c| c.arn == resource_arn)
        {
            return Ok(&mut cluster.tags);
        }
        // Check parameter groups
        if let Some(pg) = self
            .db_parameter_groups
            .values_mut()
            .find(|pg| pg.arn == resource_arn)
        {
            return Ok(&mut pg.tags);
        }
        Err(TsInfluxError::ResourceArnNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn find_resource_tags(
        &self,
        resource_arn: &str,
    ) -> Result<&HashMap<String, String>, TsInfluxError> {
        if let Some(inst) = self.db_instances.values().find(|i| i.arn == resource_arn) {
            return Ok(&inst.tags);
        }
        if let Some(cluster) = self.db_clusters.values().find(|c| c.arn == resource_arn) {
            return Ok(&cluster.tags);
        }
        if let Some(pg) = self
            .db_parameter_groups
            .values()
            .find(|pg| pg.arn == resource_arn)
        {
            return Ok(&pg.tags);
        }
        Err(TsInfluxError::ResourceArnNotFound {
            arn: resource_arn.to_string(),
        })
    }
}

fn generate_id() -> String {
    uuid::Uuid::new_v4()
        .to_string()
        .replace('-', "")
        .chars()
        .take(10)
        .collect()
}
