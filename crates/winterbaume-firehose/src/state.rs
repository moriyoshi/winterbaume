use std::collections::HashMap;

use chrono::Utc;
use serde_json::Value;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct FirehoseState {
    pub streams: HashMap<String, DeliveryStream>,
}

#[derive(Debug, Error)]
pub enum FirehoseError {
    #[error("Delivery stream {name} already exists.")]
    ResourceInUse { name: String },

    #[error("Delivery stream {name} under account does not exist.")]
    ResourceNotFound { name: String },

    #[error("The stream version does not match.")]
    ConcurrentModification,
}

impl FirehoseState {
    pub fn create_delivery_stream(
        &mut self,
        name: &str,
        destination_type: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&DeliveryStream, FirehoseError> {
        if self.streams.contains_key(name) {
            return Err(FirehoseError::ResourceInUse {
                name: name.to_string(),
            });
        }

        let arn = format!("arn:aws:firehose:{region}:{account_id}:deliverystream/{name}");

        let destination_id = format!("destinationId-{}", uuid::Uuid::new_v4());

        let stream = DeliveryStream {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            destination_type: destination_type.unwrap_or("ExtendedS3").to_string(),
            destination_id,
            created_timestamp: Utc::now(),
            records: Vec::new(),
            tags: HashMap::new(),
            encryption_status: "DISABLED".to_string(),
            version_id: "1".to_string(),
        };

        self.streams.insert(name.to_string(), stream);
        Ok(self.streams.get(name).unwrap())
    }

    pub fn delete_delivery_stream(&mut self, name: &str) -> Result<(), FirehoseError> {
        if self.streams.remove(name).is_none() {
            return Err(FirehoseError::ResourceNotFound {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_delivery_stream(&self, name: &str) -> Result<&DeliveryStream, FirehoseError> {
        self.streams
            .get(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })
    }

    pub fn list_delivery_streams(&self) -> Vec<&str> {
        self.streams.keys().map(|k| k.as_str()).collect()
    }

    pub fn put_record(&mut self, name: &str) -> Result<String, FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;

        let record_id = uuid::Uuid::new_v4().to_string();
        let record = FirehoseRecord {
            record_id: record_id.clone(),
        };

        stream.records.push(record);
        Ok(record_id)
    }

    pub fn put_record_batch(
        &mut self,
        name: &str,
        count: usize,
    ) -> Result<Vec<String>, FirehoseError> {
        let mut record_ids = Vec::with_capacity(count);
        for _ in 0..count {
            let record_id = self.put_record(name)?;
            record_ids.push(record_id);
        }
        Ok(record_ids)
    }

    pub fn list_tags_for_delivery_stream(
        &self,
        name: &str,
    ) -> Result<&HashMap<String, String>, FirehoseError> {
        let stream = self
            .streams
            .get(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        Ok(&stream.tags)
    }

    pub fn tag_delivery_stream(&mut self, name: &str, tags: &[Value]) -> Result<(), FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        for tag in tags {
            if let (Some(key), Some(value)) = (
                tag.get("Key").and_then(|v| v.as_str()),
                tag.get("Value").and_then(|v| v.as_str()),
            ) {
                stream.tags.insert(key.to_string(), value.to_string());
            }
        }
        Ok(())
    }

    pub fn untag_delivery_stream(
        &mut self,
        name: &str,
        tag_keys: &[&str],
    ) -> Result<(), FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        for key in tag_keys {
            stream.tags.remove(*key);
        }
        Ok(())
    }

    pub fn start_delivery_stream_encryption(&mut self, name: &str) -> Result<(), FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        stream.encryption_status = "ENABLED".to_string();
        Ok(())
    }

    pub fn stop_delivery_stream_encryption(&mut self, name: &str) -> Result<(), FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        stream.encryption_status = "DISABLED".to_string();
        Ok(())
    }

    pub fn update_destination(
        &mut self,
        name: &str,
        current_version_id: &str,
    ) -> Result<(), FirehoseError> {
        let stream = self
            .streams
            .get_mut(name)
            .ok_or_else(|| FirehoseError::ResourceNotFound {
                name: name.to_string(),
            })?;
        if stream.version_id != current_version_id {
            return Err(FirehoseError::ConcurrentModification);
        }
        let next_version: u64 = stream.version_id.parse().unwrap_or(1) + 1;
        stream.version_id = next_version.to_string();
        Ok(())
    }
}
