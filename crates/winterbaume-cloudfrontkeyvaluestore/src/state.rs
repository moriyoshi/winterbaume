use std::collections::HashMap;

use thiserror::Error;

#[derive(Debug, Default)]
pub struct CloudFrontKvsState {
    /// Key-value stores keyed by KvsARN.
    pub stores: HashMap<String, KvsRecord>,
}

#[derive(Debug, Clone)]
pub struct KvsRecord {
    pub arn: String,
    pub created: f64,
    pub last_modified: f64,
    pub status: String,
    pub etag: String,
    pub items: HashMap<String, String>,
}

impl Default for KvsRecord {
    fn default() -> Self {
        let now = chrono::Utc::now().timestamp() as f64;
        Self {
            arn: String::new(),
            created: now,
            last_modified: now,
            status: "READY".to_string(),
            etag: format!("etag-{}", uuid::Uuid::new_v4().simple()),
            items: HashMap::new(),
        }
    }
}

#[derive(Debug, Error)]
pub enum CloudFrontKvsError {
    #[error("Key {key} not found in store {arn}.")]
    KeyNotFound { arn: String, key: String },

    #[error("Conflict updating store {arn}: ETag mismatch.")]
    EtagMismatch { arn: String },

    #[error("{message}")]
    Validation { message: String },
}

impl CloudFrontKvsState {
    /// Lazily ensure a store record exists for the given ARN.
    pub fn ensure_store(&mut self, arn: &str) -> &mut KvsRecord {
        let entry = self
            .stores
            .entry(arn.to_string())
            .or_insert_with(|| KvsRecord {
                arn: arn.to_string(),
                ..Default::default()
            });
        entry
    }

    pub fn touch(&mut self, arn: &str) {
        let r = self.ensure_store(arn);
        r.last_modified = chrono::Utc::now().timestamp() as f64;
        r.etag = format!("etag-{}", uuid::Uuid::new_v4().simple());
    }

    pub fn get_store(&self, arn: &str) -> Option<&KvsRecord> {
        self.stores.get(arn)
    }

    pub fn put_key(
        &mut self,
        arn: &str,
        if_match: &str,
        key: String,
        value: String,
    ) -> Result<(), CloudFrontKvsError> {
        let r = self.ensure_store(arn);
        if !if_match.is_empty() && if_match != r.etag {
            return Err(CloudFrontKvsError::EtagMismatch {
                arn: arn.to_string(),
            });
        }
        r.items.insert(key, value);
        r.last_modified = chrono::Utc::now().timestamp() as f64;
        r.etag = format!("etag-{}", uuid::Uuid::new_v4().simple());
        Ok(())
    }

    pub fn get_key(&self, arn: &str, key: &str) -> Result<&String, CloudFrontKvsError> {
        let r = self
            .stores
            .get(arn)
            .ok_or(CloudFrontKvsError::KeyNotFound {
                arn: arn.to_string(),
                key: key.to_string(),
            })?;
        r.items.get(key).ok_or(CloudFrontKvsError::KeyNotFound {
            arn: arn.to_string(),
            key: key.to_string(),
        })
    }

    pub fn delete_key(
        &mut self,
        arn: &str,
        if_match: &str,
        key: &str,
    ) -> Result<(), CloudFrontKvsError> {
        let r = self
            .stores
            .get_mut(arn)
            .ok_or(CloudFrontKvsError::KeyNotFound {
                arn: arn.to_string(),
                key: key.to_string(),
            })?;
        if !if_match.is_empty() && if_match != r.etag {
            return Err(CloudFrontKvsError::EtagMismatch {
                arn: arn.to_string(),
            });
        }
        r.items.remove(key).ok_or(CloudFrontKvsError::KeyNotFound {
            arn: arn.to_string(),
            key: key.to_string(),
        })?;
        r.last_modified = chrono::Utc::now().timestamp() as f64;
        r.etag = format!("etag-{}", uuid::Uuid::new_v4().simple());
        Ok(())
    }

    pub fn batch_update(
        &mut self,
        arn: &str,
        if_match: &str,
        puts: Vec<(String, String)>,
        deletes: Vec<String>,
    ) -> Result<(), CloudFrontKvsError> {
        let r = self.ensure_store(arn);
        if !if_match.is_empty() && if_match != r.etag {
            return Err(CloudFrontKvsError::EtagMismatch {
                arn: arn.to_string(),
            });
        }
        for (k, v) in puts {
            r.items.insert(k, v);
        }
        for k in deletes {
            r.items.remove(&k);
        }
        r.last_modified = chrono::Utc::now().timestamp() as f64;
        r.etag = format!("etag-{}", uuid::Uuid::new_v4().simple());
        Ok(())
    }

    pub fn list_items(&self, arn: &str) -> Vec<(&String, &String)> {
        let Some(r) = self.stores.get(arn) else {
            return vec![];
        };
        let mut v: Vec<(&String, &String)> = r.items.iter().collect();
        v.sort_by(|a, b| a.0.cmp(b.0));
        v
    }

    pub fn item_count(&self, arn: &str) -> i32 {
        self.stores
            .get(arn)
            .map(|r| r.items.len() as i32)
            .unwrap_or(0)
    }

    pub fn total_size(&self, arn: &str) -> i64 {
        self.stores
            .get(arn)
            .map(|r| {
                r.items
                    .iter()
                    .map(|(k, v)| (k.len() + v.len()) as i64)
                    .sum()
            })
            .unwrap_or(0)
    }
}
