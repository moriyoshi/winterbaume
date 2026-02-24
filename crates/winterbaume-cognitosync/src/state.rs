use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CognitoSyncState {
    pub pools: HashMap<String, IdentityPoolRecord>,
}

#[derive(Debug, Clone, Default)]
pub struct IdentityPoolRecord {
    pub events: HashMap<String, String>,
    pub configuration: Value,
    pub bulk_publish: Value,
    pub identities: HashMap<String, IdentityRecord>,
    pub last_modified: f64,
}

#[derive(Debug, Clone, Default)]
pub struct IdentityRecord {
    pub datasets: HashMap<String, DatasetRecord>,
    pub devices: HashMap<String, DeviceRecord>,
}

#[derive(Debug, Clone, Default)]
pub struct DatasetRecord {
    pub records: HashMap<String, RecordEntry>,
    pub sync_count: i64,
    pub creation_date: f64,
    pub last_modified_date: f64,
    pub last_modified_by: String,
    pub data_storage: i64,
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct RecordEntry {
    pub key: String,
    pub value: Option<String>,
    pub sync_count: i64,
    pub last_modified_date: f64,
    pub last_modified_by: String,
    pub device_last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct DeviceRecord {
    pub device_id: String,
    pub platform: String,
    pub token: String,
}

#[derive(Debug, Error)]
pub enum CognitoSyncError {
    #[error("Resource {0} does not exist.")]
    NotFound(String),

    #[error("{0}")]
    Validation(String),
}

impl CognitoSyncState {
    pub fn pool_mut(&mut self, pool_id: &str) -> &mut IdentityPoolRecord {
        self.pools.entry(pool_id.to_string()).or_default()
    }

    pub fn identity_mut(&mut self, pool_id: &str, identity_id: &str) -> &mut IdentityRecord {
        self.pool_mut(pool_id)
            .identities
            .entry(identity_id.to_string())
            .or_default()
    }
}
