use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KafkaState {
    pub clusters: HashMap<String, Cluster>,
    /// Tags keyed by resource ARN -> (key -> value).
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum KafkaError {
    #[error("A cluster with the name '{cluster_name}' already exists.")]
    ClusterAlreadyExists { cluster_name: String },

    #[error("The cluster with ARN '{cluster_arn}' does not exist.")]
    ClusterNotFound { cluster_arn: String },
}

impl KafkaState {
    pub fn create_cluster(
        &mut self,
        cluster_name: &str,
        cluster_arn: &str,
        cluster_type: ClusterType,
        provisioned: Option<ProvisionedClusterInfo>,
        serverless: Option<ServerlessClusterInfo>,
        tags: HashMap<String, String>,
    ) -> Result<&Cluster, KafkaError> {
        // Check for duplicate cluster name
        if self
            .clusters
            .values()
            .any(|c| c.cluster_name == cluster_name)
        {
            return Err(KafkaError::ClusterAlreadyExists {
                cluster_name: cluster_name.to_string(),
            });
        }

        // Store tags separately by ARN
        if !tags.is_empty() {
            self.tags.insert(cluster_arn.to_string(), tags.clone());
        }

        let cluster = Cluster {
            cluster_name: cluster_name.to_string(),
            cluster_arn: cluster_arn.to_string(),
            state: ClusterState::Creating,
            cluster_type,
            creation_time: "2024-01-01T00:00:00Z".to_string(),
            provisioned,
            serverless,
            tags,
            extra_config: None,
        };

        self.clusters.insert(cluster_arn.to_string(), cluster);
        Ok(self.clusters.get(cluster_arn).unwrap())
    }

    pub fn describe_cluster(&self, cluster_arn: &str) -> Result<&Cluster, KafkaError> {
        self.clusters
            .get(cluster_arn)
            .ok_or_else(|| KafkaError::ClusterNotFound {
                cluster_arn: cluster_arn.to_string(),
            })
    }

    pub fn delete_cluster(&mut self, cluster_arn: &str) -> Result<Cluster, KafkaError> {
        self.tags.remove(cluster_arn);
        self.clusters
            .remove(cluster_arn)
            .ok_or_else(|| KafkaError::ClusterNotFound {
                cluster_arn: cluster_arn.to_string(),
            })
    }

    pub fn list_clusters(&self) -> Vec<&Cluster> {
        self.clusters.values().collect()
    }

    pub fn tag_resource(&mut self, arn: &str, tags: &HashMap<String, String>) {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k.clone(), v.clone());
        }
        // Also update the cluster tags if this is a cluster ARN
        if let Some(cluster) = self.clusters.get_mut(arn) {
            for (k, v) in tags {
                cluster.tags.insert(k.clone(), v.clone());
            }
        }
    }

    pub fn untag_resource(&mut self, arn: &str, tag_keys: &[String]) {
        if let Some(tag_map) = self.tags.get_mut(arn) {
            for key in tag_keys {
                tag_map.remove(key);
            }
        }
        // Also update the cluster tags if this is a cluster ARN
        if let Some(cluster) = self.clusters.get_mut(arn) {
            for key in tag_keys {
                cluster.tags.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, arn: &str) -> HashMap<String, String> {
        self.tags.get(arn).cloned().unwrap_or_default()
    }
}
