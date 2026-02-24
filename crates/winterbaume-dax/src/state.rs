use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct DaxState {
    pub clusters: HashMap<String, DaxCluster>,
    pub parameter_groups: HashMap<String, DaxParameterGroup>,
    pub subnet_groups: HashMap<String, DaxSubnetGroup>,
}

#[derive(Debug, Error)]
pub enum DaxError {
    #[error(
        "Cluster ID specified is not a valid identifier. Identifiers must begin with a letter; must contain only ASCII letters, digits, and hyphens; and must not end with a hyphen or contain two consecutive hyphens."
    )]
    InvalidClusterName,

    #[error("ARNs must start with 'arn:': {arn}")]
    InvalidArnMissingPrefix { arn: String },

    #[error("Second colon partition not found: {arn}")]
    InvalidArnMissingPartition { arn: String },

    #[error("Third colon vendor not found: {arn}")]
    InvalidArnMissingVendor { arn: String },

    #[error("Fourth colon (region/namespace delimiter) not found: {arn}")]
    InvalidArnMissingRegionDelimiter { arn: String },

    #[error("Fifth colon (namespace/relative-id delimiter) not found: {arn}")]
    InvalidArnMissingNamespaceDelimiter { arn: String },

    #[error("Cluster {cluster_name} already exists.")]
    ClusterAlreadyExists { cluster_name: String },

    #[error("{message}")]
    ClusterNotFound { message: String },

    #[error("{message}")]
    InvalidParameterValue { message: String },
}

/// Validate DAX cluster name: must start with letter, contain only ASCII letters/digits/hyphens,
/// not end with hyphen, no consecutive hyphens.
fn validate_cluster_name(name: &str) -> Result<(), DaxError> {
    if name.is_empty() {
        return Err(DaxError::InvalidClusterName);
    }
    let first = name.chars().next().unwrap();
    if !first.is_ascii_alphabetic() {
        return Err(DaxError::InvalidClusterName);
    }
    if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
        return Err(DaxError::InvalidClusterName);
    }
    if name.ends_with('-') {
        return Err(DaxError::InvalidClusterName);
    }
    if name.contains("--") {
        return Err(DaxError::InvalidClusterName);
    }
    Ok(())
}

/// Validate IAM role ARN format.
fn validate_iam_arn(arn: &str) -> Result<(), DaxError> {
    if !arn.starts_with("arn:") {
        return Err(DaxError::InvalidArnMissingPrefix {
            arn: arn.to_string(),
        });
    }
    let parts: Vec<&str> = arn.splitn(6, ':').collect();
    if parts.len() < 2 || parts[1].is_empty() {
        return Err(DaxError::InvalidArnMissingPartition {
            arn: arn.to_string(),
        });
    }
    if parts.len() < 3 {
        return Err(DaxError::InvalidArnMissingVendor {
            arn: arn.to_string(),
        });
    }
    if parts.len() < 4 {
        return Err(DaxError::InvalidArnMissingRegionDelimiter {
            arn: arn.to_string(),
        });
    }
    if parts.len() < 5 {
        return Err(DaxError::InvalidArnMissingNamespaceDelimiter {
            arn: arn.to_string(),
        });
    }
    Ok(())
}

impl DaxState {
    pub fn create_cluster(
        &mut self,
        cluster_name: &str,
        node_type: &str,
        replication_factor: i32,
        iam_role_arn: &str,
        description: &str,
        sse_enabled: bool,
        cluster_endpoint_encryption_type: String,
        tags: Vec<DaxTag>,
        region: &str,
        account_id: &str,
    ) -> Result<&DaxCluster, DaxError> {
        validate_cluster_name(cluster_name)?;
        validate_iam_arn(iam_role_arn)?;

        if self.clusters.contains_key(cluster_name) {
            return Err(DaxError::ClusterAlreadyExists {
                cluster_name: cluster_name.to_string(),
            });
        }

        let cluster_arn = format!("arn:aws:dax:{region}:{account_id}:cache/{cluster_name}");

        let cluster = DaxCluster {
            cluster_name: cluster_name.to_string(),
            cluster_arn,
            node_type: node_type.to_string(),
            status: "available".to_string(),
            description: description.to_string(),
            iam_role_arn: iam_role_arn.to_string(),
            replication_factor,
            sse_enabled,
            cluster_endpoint_encryption_type,
            tags,
        };

        self.clusters.insert(cluster_name.to_string(), cluster);
        Ok(self.clusters.get(cluster_name).unwrap())
    }

    pub fn describe_clusters(
        &self,
        cluster_names: Option<&[String]>,
    ) -> Result<Vec<&DaxCluster>, DaxError> {
        match cluster_names {
            Some(names) if !names.is_empty() => {
                let mut result = Vec::new();
                for name in names {
                    validate_cluster_name(name)?;
                    match self.clusters.get(name.as_str()) {
                        Some(c) => result.push(c),
                        None => {
                            return Err(DaxError::ClusterNotFound {
                                message: format!("Cluster {name} not found."),
                            });
                        }
                    }
                }
                Ok(result)
            }
            _ => Ok(self.clusters.values().collect()),
        }
    }

    pub fn delete_cluster(&mut self, cluster_name: &str) -> Result<DaxCluster, DaxError> {
        self.clusters
            .remove(cluster_name)
            .ok_or_else(|| DaxError::ClusterNotFound {
                message: "Cluster not found.".to_string(),
            })
    }
}
