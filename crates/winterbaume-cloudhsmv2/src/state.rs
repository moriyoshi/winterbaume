use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CloudHsmV2State {
    pub clusters: HashMap<String, Cluster>,
    pub backups: HashMap<String, Backup>,
    pub resource_policies: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum CloudHsmV2Error {
    #[error("Cluster not found: {cluster_id}")]
    ClusterNotFound { cluster_id: String },

    #[error("Cluster {cluster_id} is already being deleted or has been deleted")]
    ClusterAlreadyDeleted { cluster_id: String },

    #[error("Cluster {cluster_id} is in state {state} and cannot be initialized")]
    ClusterNotInitializable { cluster_id: String, state: String },

    #[error("Cluster {cluster_id} is deleted and cannot accept HSMs")]
    ClusterDeletedCannotAddHsm { cluster_id: String },

    #[error("Cluster {cluster_id} is deleted and cannot be modified")]
    ClusterDeletedCannotModify { cluster_id: String },

    #[error("HSM not found: {identifier}")]
    HsmNotFound { identifier: String },

    #[error("Backup not found: {backup_id}")]
    BackupNotFound { backup_id: String },

    #[error("Backup {backup_id} is already deleted")]
    BackupAlreadyDeleted { backup_id: String },

    #[error("Backup {backup_id} is not in DELETED state and cannot be restored")]
    BackupNotRestorable { backup_id: String },

    #[error("Resource not found: {resource_id}")]
    ResourceNotFound { resource_id: String },
}

impl CloudHsmV2State {
    pub fn create_cluster(
        &mut self,
        hsm_type: &str,
        subnet_ids: Vec<String>,
        source_backup_id: Option<String>,
        backup_retention_policy: Option<BackupRetentionPolicy>,
        tag_list: Vec<Tag>,
        account_id: &str,
        region: &str,
    ) -> Result<&Cluster, CloudHsmV2Error> {
        let cluster_id = format!("cluster-{}", &uuid::Uuid::new_v4().to_string()[..17]);

        // Build subnet mapping: map each subnet to an availability zone
        let mut subnet_mapping = HashMap::new();
        for (i, subnet_id) in subnet_ids.iter().enumerate() {
            let az = format!("{}{}", region, (b'a' + i as u8) as char);
            subnet_mapping.insert(az, subnet_id.clone());
        }

        let vpc_id = format!("vpc-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let security_group = format!("sg-{}", &uuid::Uuid::new_v4().to_string()[..8]);

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let cluster = Cluster {
            cluster_id: cluster_id.clone(),
            hsm_type: hsm_type.to_string(),
            subnet_mapping,
            vpc_id,
            state: ClusterState::Uninitialized,
            security_group,
            source_backup_id,
            backup_policy: "DEFAULT".to_string(),
            backup_retention_policy,
            create_timestamp: now,
            tag_list,
            region: region.to_string(),
            account_id: account_id.to_string(),
            hsms: Vec::new(),
        };

        self.clusters.insert(cluster_id.clone(), cluster);
        Ok(self.clusters.get(&cluster_id).unwrap())
    }

    pub fn describe_clusters(&self, filters: &HashMap<String, Vec<String>>) -> Vec<&Cluster> {
        self.clusters
            .values()
            .filter(|c| {
                // Filter by clusterIds if specified
                if let Some(ids) = filters.get("clusterIds")
                    && !ids.is_empty()
                    && !ids.contains(&c.cluster_id)
                {
                    return false;
                }
                // Filter by states if specified
                if let Some(states) = filters.get("states")
                    && !states.is_empty()
                    && !states.contains(&c.state.as_str().to_string())
                {
                    return false;
                }
                // Filter by vpcIds if specified
                if let Some(vpc_ids) = filters.get("vpcIds")
                    && !vpc_ids.is_empty()
                    && !vpc_ids.contains(&c.vpc_id)
                {
                    return false;
                }
                true
            })
            .collect()
    }

    pub fn delete_cluster(&mut self, cluster_id: &str) -> Result<&Cluster, CloudHsmV2Error> {
        let cluster =
            self.clusters
                .get_mut(cluster_id)
                .ok_or_else(|| CloudHsmV2Error::ClusterNotFound {
                    cluster_id: cluster_id.to_string(),
                })?;

        if cluster.state == ClusterState::Deleted || cluster.state == ClusterState::DeleteInProgress
        {
            return Err(CloudHsmV2Error::ClusterAlreadyDeleted {
                cluster_id: cluster_id.to_string(),
            });
        }

        cluster.state = ClusterState::Deleted;
        Ok(self.clusters.get(cluster_id).unwrap())
    }

    pub fn initialize_cluster(
        &mut self,
        cluster_id: &str,
        _signed_cert: &str,
        _trust_anchor: &str,
    ) -> Result<&Cluster, CloudHsmV2Error> {
        let cluster =
            self.clusters
                .get_mut(cluster_id)
                .ok_or_else(|| CloudHsmV2Error::ClusterNotFound {
                    cluster_id: cluster_id.to_string(),
                })?;

        if cluster.state != ClusterState::Uninitialized {
            return Err(CloudHsmV2Error::ClusterNotInitializable {
                cluster_id: cluster_id.to_string(),
                state: cluster.state.as_str().to_string(),
            });
        }

        cluster.state = ClusterState::Initialized;
        Ok(self.clusters.get(cluster_id).unwrap())
    }

    pub fn create_hsm(
        &mut self,
        cluster_id: &str,
        availability_zone: &str,
        ip_address: Option<String>,
    ) -> Result<Hsm, CloudHsmV2Error> {
        let cluster =
            self.clusters
                .get_mut(cluster_id)
                .ok_or_else(|| CloudHsmV2Error::ClusterNotFound {
                    cluster_id: cluster_id.to_string(),
                })?;

        if cluster.state == ClusterState::Deleted || cluster.state == ClusterState::DeleteInProgress
        {
            return Err(CloudHsmV2Error::ClusterDeletedCannotAddHsm {
                cluster_id: cluster_id.to_string(),
            });
        }

        let hsm_id = format!("hsm-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let eni_id = format!("eni-{}", &uuid::Uuid::new_v4().to_string()[..8]);
        let eni_ip = ip_address.unwrap_or_else(|| "10.0.0.1".to_string());

        // Find subnet_id from subnet_mapping by availability zone
        let subnet_id = cluster.subnet_mapping.get(availability_zone).cloned();

        let hsm = Hsm {
            hsm_id: hsm_id.clone(),
            cluster_id: cluster_id.to_string(),
            availability_zone: availability_zone.to_string(),
            subnet_id,
            eni_id: Some(eni_id),
            eni_ip: Some(eni_ip),
            state: "ACTIVE".to_string(),
        };

        cluster.hsms.push(hsm.clone());
        Ok(hsm)
    }

    pub fn delete_hsm(
        &mut self,
        cluster_id: &str,
        hsm_id: Option<&str>,
        eni_id: Option<&str>,
        eni_ip: Option<&str>,
    ) -> Result<String, CloudHsmV2Error> {
        let cluster =
            self.clusters
                .get_mut(cluster_id)
                .ok_or_else(|| CloudHsmV2Error::ClusterNotFound {
                    cluster_id: cluster_id.to_string(),
                })?;

        let pos = cluster.hsms.iter().position(|h| {
            if let Some(id) = hsm_id {
                h.hsm_id == id
            } else if let Some(eid) = eni_id {
                h.eni_id.as_deref() == Some(eid)
            } else if let Some(ip) = eni_ip {
                h.eni_ip.as_deref() == Some(ip)
            } else {
                false
            }
        });

        match pos {
            Some(idx) => {
                let removed = cluster.hsms.remove(idx);
                Ok(removed.hsm_id)
            }
            None => {
                let identifier = hsm_id.or(eni_id).or(eni_ip).unwrap_or("unknown");
                Err(CloudHsmV2Error::HsmNotFound {
                    identifier: identifier.to_string(),
                })
            }
        }
    }

    pub fn describe_backups(&self, filters: &HashMap<String, Vec<String>>) -> Vec<&Backup> {
        self.backups
            .values()
            .filter(|b| {
                if let Some(ids) = filters.get("backupIds")
                    && !ids.is_empty()
                    && !ids.contains(&b.backup_id)
                {
                    return false;
                }
                if let Some(states) = filters.get("states")
                    && !states.is_empty()
                    && !states.contains(&b.backup_state)
                {
                    return false;
                }
                if let Some(cluster_ids) = filters.get("clusterIds")
                    && !cluster_ids.is_empty()
                    && !b
                        .cluster_id
                        .as_ref()
                        .map(|id| cluster_ids.contains(id))
                        .unwrap_or(false)
                {
                    return false;
                }
                true
            })
            .collect()
    }

    pub fn delete_backup(&mut self, backup_id: &str) -> Result<Backup, CloudHsmV2Error> {
        let backup =
            self.backups
                .get_mut(backup_id)
                .ok_or_else(|| CloudHsmV2Error::BackupNotFound {
                    backup_id: backup_id.to_string(),
                })?;

        if backup.backup_state == "DELETED" {
            return Err(CloudHsmV2Error::BackupAlreadyDeleted {
                backup_id: backup_id.to_string(),
            });
        }

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        backup.backup_state = "DELETED".to_string();
        backup.delete_timestamp = Some(now);
        Ok(backup.clone())
    }

    pub fn restore_backup(&mut self, backup_id: &str) -> Result<Backup, CloudHsmV2Error> {
        let backup =
            self.backups
                .get_mut(backup_id)
                .ok_or_else(|| CloudHsmV2Error::BackupNotFound {
                    backup_id: backup_id.to_string(),
                })?;

        if backup.backup_state != "DELETED" {
            return Err(CloudHsmV2Error::BackupNotRestorable {
                backup_id: backup_id.to_string(),
            });
        }

        backup.backup_state = "READY".to_string();
        backup.delete_timestamp = None;
        Ok(backup.clone())
    }

    pub fn copy_backup_to_region(
        &mut self,
        backup_id: &str,
        destination_region: &str,
        tag_list: Vec<Tag>,
        account_id: &str,
    ) -> Result<Backup, CloudHsmV2Error> {
        let source_backup = self
            .backups
            .get(backup_id)
            .ok_or_else(|| CloudHsmV2Error::BackupNotFound {
                backup_id: backup_id.to_string(),
            })?
            .clone();

        let new_backup_id = format!("backup-{}", &uuid::Uuid::new_v4().to_string()[..17]);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();

        let new_backup = Backup {
            backup_id: new_backup_id.clone(),
            backup_arn: format!(
                "arn:aws:cloudhsm:{destination_region}:{account_id}:backup/{new_backup_id}"
            ),
            backup_state: "CREATE_IN_PROGRESS".to_string(),
            cluster_id: source_backup.cluster_id.clone(),
            create_timestamp: now,
            copy_timestamp: Some(now),
            delete_timestamp: None,
            hsm_type: source_backup.hsm_type.clone(),
            never_expires: false,
            source_backup: Some(backup_id.to_string()),
            source_cluster: source_backup.cluster_id.clone(),
            source_region: None,
            tag_list,
        };

        self.backups.insert(new_backup_id, new_backup.clone());
        Ok(new_backup)
    }

    pub fn modify_backup_attributes(
        &mut self,
        backup_id: &str,
        never_expires: bool,
    ) -> Result<Backup, CloudHsmV2Error> {
        let backup =
            self.backups
                .get_mut(backup_id)
                .ok_or_else(|| CloudHsmV2Error::BackupNotFound {
                    backup_id: backup_id.to_string(),
                })?;

        backup.never_expires = never_expires;
        Ok(backup.clone())
    }

    pub fn modify_cluster(
        &mut self,
        cluster_id: &str,
        backup_retention_policy: Option<BackupRetentionPolicy>,
        hsm_type: Option<String>,
    ) -> Result<&Cluster, CloudHsmV2Error> {
        let cluster =
            self.clusters
                .get_mut(cluster_id)
                .ok_or_else(|| CloudHsmV2Error::ClusterNotFound {
                    cluster_id: cluster_id.to_string(),
                })?;

        if cluster.state == ClusterState::Deleted {
            return Err(CloudHsmV2Error::ClusterDeletedCannotModify {
                cluster_id: cluster_id.to_string(),
            });
        }

        if let Some(policy) = backup_retention_policy {
            cluster.backup_retention_policy = Some(policy);
        }
        if let Some(ht) = hsm_type {
            cluster.hsm_type = ht;
        }

        Ok(self.clusters.get(cluster_id).unwrap())
    }

    pub fn list_tags(&self, resource_id: &str) -> Result<Vec<Tag>, CloudHsmV2Error> {
        // resource_id can be a cluster ID or backup ID
        if let Some(cluster) = self.clusters.get(resource_id) {
            return Ok(cluster.tag_list.clone());
        }
        if let Some(backup) = self.backups.get(resource_id) {
            return Ok(backup.tag_list.clone());
        }
        Err(CloudHsmV2Error::ResourceNotFound {
            resource_id: resource_id.to_string(),
        })
    }

    pub fn tag_resource(
        &mut self,
        resource_id: &str,
        tags: Vec<Tag>,
    ) -> Result<(), CloudHsmV2Error> {
        if let Some(cluster) = self.clusters.get_mut(resource_id) {
            for new_tag in tags {
                if let Some(existing) = cluster.tag_list.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value;
                } else {
                    cluster.tag_list.push(new_tag);
                }
            }
            return Ok(());
        }
        if let Some(backup) = self.backups.get_mut(resource_id) {
            for new_tag in tags {
                if let Some(existing) = backup.tag_list.iter_mut().find(|t| t.key == new_tag.key) {
                    existing.value = new_tag.value;
                } else {
                    backup.tag_list.push(new_tag);
                }
            }
            return Ok(());
        }
        Err(CloudHsmV2Error::ResourceNotFound {
            resource_id: resource_id.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_id: &str,
        tag_key_list: &[String],
    ) -> Result<(), CloudHsmV2Error> {
        if let Some(cluster) = self.clusters.get_mut(resource_id) {
            cluster.tag_list.retain(|t| !tag_key_list.contains(&t.key));
            return Ok(());
        }
        if let Some(backup) = self.backups.get_mut(resource_id) {
            backup.tag_list.retain(|t| !tag_key_list.contains(&t.key));
            return Ok(());
        }
        Err(CloudHsmV2Error::ResourceNotFound {
            resource_id: resource_id.to_string(),
        })
    }

    pub fn get_resource_policy(&self, resource_arn: &str) -> Option<String> {
        self.resource_policies.get(resource_arn).cloned()
    }

    pub fn put_resource_policy(&mut self, resource_arn: &str, policy: Option<String>) -> String {
        let policy_str = policy.unwrap_or_default();
        self.resource_policies
            .insert(resource_arn.to_string(), policy_str.clone());
        policy_str
    }

    pub fn delete_resource_policy(&mut self, resource_arn: &str) -> Option<String> {
        self.resource_policies.remove(resource_arn)
    }
}
