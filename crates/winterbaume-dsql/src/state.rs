use std::collections::HashMap;

use chrono::Utc;

use crate::types::Cluster;

#[derive(Debug, Default)]
pub struct DsqlState {
    pub clusters: HashMap<String, Cluster>,
    /// Maps client_token -> cluster identifier for idempotency.
    pub client_tokens: HashMap<String, String>,
}

#[derive(Debug, thiserror::Error)]
pub enum DsqlError {
    #[error("Cluster '{identifier}' not found.")]
    ResourceNotFound { identifier: String },

    #[error("Cluster '{identifier}' has deletion protection enabled and cannot be deleted.")]
    DeletionProtectionEnabled { identifier: String },

    #[error("Invalid next_token value.")]
    InvalidNextToken,
}

impl DsqlState {
    pub fn create_cluster(
        &mut self,
        account_id: &str,
        region: &str,
        deletion_protection_enabled: Option<bool>,
        client_token: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<Cluster, DsqlError> {
        // Idempotency: if the same client token was used, return existing cluster
        if let Some(token) = client_token
            && let Some(cluster_id) = self.client_tokens.get(token)
            && let Some(cluster) = self.clusters.get(cluster_id)
        {
            return Ok(cluster.clone());
        }

        // DSQL cluster IDs are 26 lowercase alphanumeric characters
        let id_raw = uuid::Uuid::new_v4().to_string().replace('-', "");
        let identifier = id_raw[..26].to_lowercase();
        let arn = format!("arn:aws:dsql:{region}:{account_id}:cluster/{identifier}");

        let deletion_protection = deletion_protection_enabled.unwrap_or(true);

        let cluster = Cluster {
            identifier: identifier.clone(),
            arn,
            status: "ACTIVE".to_string(),
            creation_time: Utc::now(),
            deletion_protection_enabled: deletion_protection,
            tags,
        };

        if let Some(token) = client_token {
            self.client_tokens
                .insert(token.to_string(), identifier.clone());
        }
        self.clusters.insert(identifier.clone(), cluster.clone());
        Ok(cluster)
    }

    pub fn get_cluster(&self, identifier: &str) -> Result<&Cluster, DsqlError> {
        self.clusters
            .get(identifier)
            .ok_or_else(|| DsqlError::ResourceNotFound {
                identifier: identifier.to_string(),
            })
    }

    pub fn delete_cluster(&mut self, identifier: &str) -> Result<Cluster, DsqlError> {
        let cluster = self
            .clusters
            .get(identifier)
            .ok_or_else(|| DsqlError::ResourceNotFound {
                identifier: identifier.to_string(),
            })?;

        if cluster.deletion_protection_enabled {
            return Err(DsqlError::DeletionProtectionEnabled {
                identifier: identifier.to_string(),
            });
        }

        let mut cluster = self.clusters.remove(identifier).unwrap();
        cluster.status = "DELETING".to_string();

        // Remove any client tokens pointing to this cluster
        self.client_tokens.retain(|_, v| v != identifier);

        Ok(cluster)
    }

    pub fn list_clusters(
        &self,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> Result<(Vec<&Cluster>, Option<String>), DsqlError> {
        use base64::Engine as _;

        let max = max_results.unwrap_or(20);

        // Decode the continuation token (base64-encoded cluster identifier).
        let start_after: Option<String> = if let Some(token) = next_token {
            match base64::engine::general_purpose::STANDARD.decode(token) {
                Ok(bytes) => String::from_utf8(bytes).ok(),
                Err(_) => {
                    return Err(DsqlError::InvalidNextToken);
                }
            }
        } else {
            None
        };

        // Sort clusters by identifier for a stable, deterministic order.
        let mut sorted: Vec<&Cluster> = self.clusters.values().collect();
        sorted.sort_by(|a, b| a.identifier.cmp(&b.identifier));

        // Skip past the cluster whose identifier equals `start_after`.
        let iter = if let Some(ref after) = start_after {
            let pos = sorted
                .iter()
                .position(|c| &c.identifier == after)
                .map(|i| i + 1)
                .unwrap_or(sorted.len());
            sorted.into_iter().skip(pos).collect::<Vec<_>>()
        } else {
            sorted
        };

        // Take at most `max` results and determine if there are more.
        let page: Vec<&Cluster> = iter.into_iter().take(max + 1).collect();
        let has_more = page.len() > max;
        let page: Vec<&Cluster> = page.into_iter().take(max).collect();

        let new_token = if has_more {
            page.last()
                .map(|c| base64::engine::general_purpose::STANDARD.encode(c.identifier.as_bytes()))
        } else {
            None
        };

        Ok((page, new_token))
    }
}
