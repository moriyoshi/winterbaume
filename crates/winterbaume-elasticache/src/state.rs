use std::collections::HashMap;

use crate::types::{
    CacheCluster, CacheParameterGroup, CacheSecurityGroup, CacheSubnetGroup, ReplicationGroup,
    Snapshot, Tag, User,
};

/// In-memory state for the ElastiCache service.
#[derive(Debug, Default)]
pub struct ElastiCacheState {
    pub cache_clusters: HashMap<String, CacheCluster>,
    pub replication_groups: HashMap<String, ReplicationGroup>,
    pub cache_subnet_groups: HashMap<String, CacheSubnetGroup>,
    pub cache_parameter_groups: HashMap<String, CacheParameterGroup>,
    pub cache_security_groups: HashMap<String, CacheSecurityGroup>,
    pub snapshots: HashMap<String, Snapshot>,
    pub users: HashMap<String, User>,
}

/// Represents an error from the ElastiCache service.
#[derive(Debug, thiserror::Error)]
pub enum ElastiCacheError {
    #[error("Cache cluster {0} already exists")]
    CacheClusterAlreadyExists(String),
    #[error("Cache cluster {0} not found")]
    CacheClusterNotFound(String),
    #[error("Replication group {0} already exists")]
    ReplicationGroupAlreadyExists(String),
    #[error("Replication group {0} not found")]
    ReplicationGroupNotFound(String),
    #[error("Cache subnet group {0} already exists")]
    CacheSubnetGroupAlreadyExists(String),
    #[error("Cache subnet group {0} not found")]
    CacheSubnetGroupNotFound(String),
    #[error("Cache parameter group {0} already exists")]
    CacheParameterGroupAlreadyExists(String),
    #[error("Cache parameter group {0} not found")]
    CacheParameterGroupNotFound(String),
    #[error("Cache security group {0} already exists")]
    CacheSecurityGroupAlreadyExists(String),
    #[error("Cache security group {0} not found")]
    CacheSecurityGroupNotFound(String),
    #[error("Snapshot {0} already exists")]
    SnapshotAlreadyExistsFault(String),
    #[error("Snapshot {0} not found")]
    SnapshotNotFound(String),
    #[error("User {0} already exists")]
    UserAlreadyExists(String),
    #[error("User {0} not found")]
    UserNotFound(String),
    #[error("Resource {0} not found")]
    InvalidARN(String),
}

impl ElastiCacheState {
    // ---- CacheCluster -------------------------------------------------------

    pub fn create_cache_cluster(
        &mut self,
        id: String,
        engine: String,
        engine_version: String,
        cache_node_type: String,
        num_cache_nodes: i32,
        cache_subnet_group_name: Option<String>,
        replication_group_id: Option<String>,
        preferred_availability_zone: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&CacheCluster, ElastiCacheError> {
        if self.cache_clusters.contains_key(&id) {
            return Err(ElastiCacheError::CacheClusterAlreadyExists(id));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:cluster:{id}");
        let cluster = CacheCluster {
            cache_cluster_id: id.clone(),
            status: "available".to_string(),
            engine: engine.clone(),
            engine_version,
            cache_node_type,
            num_cache_nodes,
            preferred_availability_zone,
            cache_subnet_group_name,
            replication_group_id,
            arn,
            tags,
        };
        self.cache_clusters.insert(id.clone(), cluster);
        Ok(self.cache_clusters.get(&id).unwrap())
    }

    pub fn describe_cache_clusters(
        &self,
        cluster_id: Option<&str>,
    ) -> Result<Vec<&CacheCluster>, ElastiCacheError> {
        if let Some(id) = cluster_id {
            match self.cache_clusters.get(id) {
                Some(c) => Ok(vec![c]),
                None => Err(ElastiCacheError::CacheClusterNotFound(id.to_string())),
            }
        } else {
            let mut clusters: Vec<&CacheCluster> = self.cache_clusters.values().collect();
            clusters.sort_by(|a, b| a.cache_cluster_id.cmp(&b.cache_cluster_id));
            Ok(clusters)
        }
    }

    pub fn delete_cache_cluster(&mut self, id: &str) -> Result<CacheCluster, ElastiCacheError> {
        self.cache_clusters
            .remove(id)
            .ok_or_else(|| ElastiCacheError::CacheClusterNotFound(id.to_string()))
    }

    // ---- ReplicationGroup ---------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_replication_group(
        &mut self,
        id: String,
        description: String,
        engine: String,
        cache_node_type: String,
        num_cache_clusters: i32,
        automatic_failover: String,
        multi_az: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&ReplicationGroup, ElastiCacheError> {
        if self.replication_groups.contains_key(&id) {
            return Err(ElastiCacheError::ReplicationGroupAlreadyExists(id));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:replicationgroup:{id}");
        let rg = ReplicationGroup {
            replication_group_id: id.clone(),
            description,
            status: "available".to_string(),
            member_clusters: vec![],
            cache_node_type,
            engine,
            automatic_failover,
            multi_az,
            arn,
            tags,
            num_cache_clusters,
        };
        self.replication_groups.insert(id.clone(), rg);
        Ok(self.replication_groups.get(&id).unwrap())
    }

    pub fn describe_replication_groups(
        &self,
        rg_id: Option<&str>,
    ) -> Result<Vec<&ReplicationGroup>, ElastiCacheError> {
        if let Some(id) = rg_id {
            match self.replication_groups.get(id) {
                Some(r) => Ok(vec![r]),
                None => Err(ElastiCacheError::ReplicationGroupNotFound(id.to_string())),
            }
        } else {
            let mut groups: Vec<&ReplicationGroup> = self.replication_groups.values().collect();
            groups.sort_by(|a, b| a.replication_group_id.cmp(&b.replication_group_id));
            Ok(groups)
        }
    }

    pub fn delete_replication_group(
        &mut self,
        id: &str,
    ) -> Result<ReplicationGroup, ElastiCacheError> {
        self.replication_groups
            .remove(id)
            .ok_or_else(|| ElastiCacheError::ReplicationGroupNotFound(id.to_string()))
    }

    // ---- CacheSubnetGroup ---------------------------------------------------

    pub fn create_cache_subnet_group(
        &mut self,
        name: String,
        description: String,
        subnet_ids: Vec<String>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&CacheSubnetGroup, ElastiCacheError> {
        if self.cache_subnet_groups.contains_key(&name) {
            return Err(ElastiCacheError::CacheSubnetGroupAlreadyExists(name));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:subnetgroup:{name}");
        let sg = CacheSubnetGroup {
            name: name.clone(),
            description,
            subnet_ids,
            vpc_id: "vpc-mock".to_string(),
            arn,
            tags,
        };
        self.cache_subnet_groups.insert(name.clone(), sg);
        Ok(self.cache_subnet_groups.get(&name).unwrap())
    }

    pub fn describe_cache_subnet_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&CacheSubnetGroup>, ElastiCacheError> {
        if let Some(n) = name {
            match self.cache_subnet_groups.get(n) {
                Some(sg) => Ok(vec![sg]),
                None => Err(ElastiCacheError::CacheSubnetGroupNotFound(n.to_string())),
            }
        } else {
            let mut groups: Vec<&CacheSubnetGroup> = self.cache_subnet_groups.values().collect();
            groups.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(groups)
        }
    }

    pub fn delete_cache_subnet_group(&mut self, name: &str) -> Result<(), ElastiCacheError> {
        self.cache_subnet_groups
            .remove(name)
            .ok_or_else(|| ElastiCacheError::CacheSubnetGroupNotFound(name.to_string()))?;
        Ok(())
    }

    // ---- CacheParameterGroup ------------------------------------------------

    pub fn create_cache_parameter_group(
        &mut self,
        name: String,
        family: String,
        description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&CacheParameterGroup, ElastiCacheError> {
        if self.cache_parameter_groups.contains_key(&name) {
            return Err(ElastiCacheError::CacheParameterGroupAlreadyExists(name));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:parametergroup:{name}");
        let pg = CacheParameterGroup {
            name: name.clone(),
            family,
            description,
            arn,
            tags,
        };
        self.cache_parameter_groups.insert(name.clone(), pg);
        Ok(self.cache_parameter_groups.get(&name).unwrap())
    }

    pub fn describe_cache_parameter_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&CacheParameterGroup>, ElastiCacheError> {
        if let Some(n) = name {
            match self.cache_parameter_groups.get(n) {
                Some(pg) => Ok(vec![pg]),
                None => Err(ElastiCacheError::CacheParameterGroupNotFound(n.to_string())),
            }
        } else {
            let mut groups: Vec<&CacheParameterGroup> =
                self.cache_parameter_groups.values().collect();
            groups.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(groups)
        }
    }

    pub fn delete_cache_parameter_group(&mut self, name: &str) -> Result<(), ElastiCacheError> {
        self.cache_parameter_groups
            .remove(name)
            .ok_or_else(|| ElastiCacheError::CacheParameterGroupNotFound(name.to_string()))?;
        Ok(())
    }

    // ---- CacheSecurityGroup -------------------------------------------------

    pub fn create_cache_security_group(
        &mut self,
        name: String,
        description: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&CacheSecurityGroup, ElastiCacheError> {
        if self.cache_security_groups.contains_key(&name) {
            return Err(ElastiCacheError::CacheSecurityGroupAlreadyExists(name));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:securitygroup:{name}");
        let sg = CacheSecurityGroup {
            name: name.clone(),
            description,
            owner_id: account_id.to_string(),
            arn,
            tags,
        };
        self.cache_security_groups.insert(name.clone(), sg);
        Ok(self.cache_security_groups.get(&name).unwrap())
    }

    pub fn describe_cache_security_groups(
        &self,
        name: Option<&str>,
    ) -> Result<Vec<&CacheSecurityGroup>, ElastiCacheError> {
        if let Some(n) = name {
            match self.cache_security_groups.get(n) {
                Some(sg) => Ok(vec![sg]),
                None => Err(ElastiCacheError::CacheSecurityGroupNotFound(n.to_string())),
            }
        } else {
            let mut groups: Vec<&CacheSecurityGroup> =
                self.cache_security_groups.values().collect();
            groups.sort_by(|a, b| a.name.cmp(&b.name));
            Ok(groups)
        }
    }

    pub fn delete_cache_security_group(&mut self, name: &str) -> Result<(), ElastiCacheError> {
        self.cache_security_groups
            .remove(name)
            .ok_or_else(|| ElastiCacheError::CacheSecurityGroupNotFound(name.to_string()))?;
        Ok(())
    }

    // ---- Snapshot -----------------------------------------------------------

    pub fn create_snapshot(
        &mut self,
        snapshot_name: String,
        cache_cluster_id: Option<String>,
        replication_group_id: Option<String>,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&Snapshot, ElastiCacheError> {
        if self.snapshots.contains_key(&snapshot_name) {
            return Err(ElastiCacheError::SnapshotAlreadyExistsFault(snapshot_name));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:snapshot:{snapshot_name}");

        // Determine engine/version/node_type from the source cluster or rg
        let (engine, engine_version, cache_node_type, cache_subnet_group_name) =
            if let Some(ref cid) = cache_cluster_id {
                if let Some(cluster) = self.cache_clusters.get(cid) {
                    (
                        cluster.engine.clone(),
                        cluster.engine_version.clone(),
                        cluster.cache_node_type.clone(),
                        cluster.cache_subnet_group_name.clone(),
                    )
                } else {
                    (
                        "redis".to_string(),
                        "7.0.12".to_string(),
                        "cache.t3.micro".to_string(),
                        None,
                    )
                }
            } else if let Some(ref rgid) = replication_group_id {
                if let Some(rg) = self.replication_groups.get(rgid) {
                    (
                        rg.engine.clone(),
                        "7.0.12".to_string(),
                        rg.cache_node_type.clone(),
                        None,
                    )
                } else {
                    (
                        "redis".to_string(),
                        "7.0.12".to_string(),
                        "cache.t3.micro".to_string(),
                        None,
                    )
                }
            } else {
                (
                    "redis".to_string(),
                    "7.0.12".to_string(),
                    "cache.t3.micro".to_string(),
                    None,
                )
            };

        let snapshot = Snapshot {
            snapshot_name: snapshot_name.clone(),
            cache_cluster_id,
            replication_group_id,
            status: "available".to_string(),
            engine,
            engine_version,
            cache_node_type,
            cache_subnet_group_name,
            arn,
            tags,
        };
        self.snapshots.insert(snapshot_name.clone(), snapshot);
        Ok(self.snapshots.get(&snapshot_name).unwrap())
    }

    pub fn describe_snapshots(
        &self,
        snapshot_name: Option<&str>,
    ) -> Result<Vec<&Snapshot>, ElastiCacheError> {
        if let Some(name) = snapshot_name {
            match self.snapshots.get(name) {
                Some(s) => Ok(vec![s]),
                None => Err(ElastiCacheError::SnapshotNotFound(name.to_string())),
            }
        } else {
            let mut snaps: Vec<&Snapshot> = self.snapshots.values().collect();
            snaps.sort_by(|a, b| a.snapshot_name.cmp(&b.snapshot_name));
            Ok(snaps)
        }
    }

    pub fn delete_snapshot(&mut self, name: &str) -> Result<Snapshot, ElastiCacheError> {
        self.snapshots
            .remove(name)
            .ok_or_else(|| ElastiCacheError::SnapshotNotFound(name.to_string()))
    }

    // ---- User ---------------------------------------------------------------

    pub fn create_user(
        &mut self,
        user_id: String,
        user_name: String,
        engine: String,
        access_string: String,
        tags: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&User, ElastiCacheError> {
        if self.users.contains_key(&user_id) {
            return Err(ElastiCacheError::UserAlreadyExists(user_id));
        }
        let arn = format!("arn:aws:elasticache:{region}:{account_id}:user:{user_id}");
        let user = User {
            user_id: user_id.clone(),
            user_name,
            engine,
            status: "active".to_string(),
            access_string,
            arn,
            tags,
        };
        self.users.insert(user_id.clone(), user);
        Ok(self.users.get(&user_id).unwrap())
    }

    pub fn describe_users(&self, user_id: Option<&str>) -> Result<Vec<&User>, ElastiCacheError> {
        if let Some(id) = user_id {
            match self.users.get(id) {
                Some(u) => Ok(vec![u]),
                None => Err(ElastiCacheError::UserNotFound(id.to_string())),
            }
        } else {
            let mut users: Vec<&User> = self.users.values().collect();
            users.sort_by(|a, b| a.user_id.cmp(&b.user_id));
            Ok(users)
        }
    }

    pub fn delete_user(&mut self, user_id: &str) -> Result<User, ElastiCacheError> {
        self.users
            .remove(user_id)
            .ok_or_else(|| ElastiCacheError::UserNotFound(user_id.to_string()))
    }

    // ---- Tags ---------------------------------------------------------------

    pub fn add_tags(
        &mut self,
        resource_arn: &str,
        tags: Vec<Tag>,
    ) -> Result<Vec<Tag>, ElastiCacheError> {
        // Find the resource and update its tags
        let resource_tags = self.get_tags_mut(resource_arn)?;
        for tag in tags {
            // Replace existing key or add new
            if let Some(existing) = resource_tags.iter_mut().find(|t| t.key == tag.key) {
                existing.value = tag.value;
            } else {
                resource_tags.push(tag);
            }
        }
        Ok(resource_tags.clone())
    }

    pub fn list_tags(&self, resource_arn: &str) -> Result<Vec<Tag>, ElastiCacheError> {
        // Search in all resource types for the ARN
        for cluster in self.cache_clusters.values() {
            if cluster.arn == resource_arn {
                return Ok(cluster.tags.clone());
            }
        }
        for rg in self.replication_groups.values() {
            if rg.arn == resource_arn {
                return Ok(rg.tags.clone());
            }
        }
        for sg in self.cache_subnet_groups.values() {
            if sg.arn == resource_arn {
                return Ok(sg.tags.clone());
            }
        }
        for pg in self.cache_parameter_groups.values() {
            if pg.arn == resource_arn {
                return Ok(pg.tags.clone());
            }
        }
        for csg in self.cache_security_groups.values() {
            if csg.arn == resource_arn {
                return Ok(csg.tags.clone());
            }
        }
        for snap in self.snapshots.values() {
            if snap.arn == resource_arn {
                return Ok(snap.tags.clone());
            }
        }
        for user in self.users.values() {
            if user.arn == resource_arn {
                return Ok(user.tags.clone());
            }
        }
        Err(ElastiCacheError::InvalidARN(resource_arn.to_string()))
    }

    pub fn remove_tags(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<Vec<Tag>, ElastiCacheError> {
        let resource_tags = self.get_tags_mut(resource_arn)?;
        resource_tags.retain(|t| !tag_keys.contains(&t.key));
        Ok(resource_tags.clone())
    }

    fn get_tags_mut(&mut self, resource_arn: &str) -> Result<&mut Vec<Tag>, ElastiCacheError> {
        for cluster in self.cache_clusters.values_mut() {
            if cluster.arn == resource_arn {
                return Ok(&mut cluster.tags);
            }
        }
        for rg in self.replication_groups.values_mut() {
            if rg.arn == resource_arn {
                return Ok(&mut rg.tags);
            }
        }
        for sg in self.cache_subnet_groups.values_mut() {
            if sg.arn == resource_arn {
                return Ok(&mut sg.tags);
            }
        }
        for pg in self.cache_parameter_groups.values_mut() {
            if pg.arn == resource_arn {
                return Ok(&mut pg.tags);
            }
        }
        for csg in self.cache_security_groups.values_mut() {
            if csg.arn == resource_arn {
                return Ok(&mut csg.tags);
            }
        }
        for snap in self.snapshots.values_mut() {
            if snap.arn == resource_arn {
                return Ok(&mut snap.tags);
            }
        }
        for user in self.users.values_mut() {
            if user.arn == resource_arn {
                return Ok(&mut user.tags);
            }
        }
        Err(ElastiCacheError::InvalidARN(resource_arn.to_string()))
    }
}
