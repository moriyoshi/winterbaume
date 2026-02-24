use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KinesisState {
    pub streams: HashMap<String, Stream>,
    next_sequence: u64,
    /// Tags keyed by resource ARN (for TagResource / UntagResource / ListTagsForResource)
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Account-level minimum throughput billing commitment status (stub)
    pub account_settings_commitment_status: String,
}

#[derive(Debug, thiserror::Error)]
pub enum KinesisError {
    // ResourceInUseException
    #[error("Stream {name} under account {account_id} already exists.")]
    StreamAlreadyExists { name: String, account_id: String },
    #[error("Consumer {consumer_name} already exists.")]
    ConsumerAlreadyExists { consumer_name: String },

    // ResourceNotFoundException
    #[error("Stream {name} under account {account_id} not found.")]
    StreamNotFound { name: String, account_id: String },
    #[error("Stream ARN {arn} under account {account_id} not found.")]
    StreamArnNotFound { arn: String, account_id: String },
    #[error("Stream ARN {stream_arn} does not exist.")]
    StreamArnDoesNotExist { stream_arn: String },
    #[error("Consumer {consumer}, account {account_id} not found.")]
    ConsumerNotFound {
        consumer: String,
        account_id: String,
    },
    #[error("Shard not found.")]
    ShardNotFound,
    #[error("Shard {shard_id} in stream {stream_name} under account {account_id} does not exist")]
    ShardDoesNotExist {
        shard_id: String,
        stream_name: String,
        account_id: String,
    },
    #[error("Could not find shard {shard_id} in stream {stream_name} under account {account_id}.")]
    ShardInStreamNotFound {
        shard_id: String,
        stream_name: String,
        account_id: String,
    },
    #[error("Stream {resource_arn} under account {account_id} not found.")]
    ResourceStreamNotFound {
        resource_arn: String,
        account_id: String,
    },
    #[error("Resource {resource_arn} not found.")]
    ResourceNotFound { resource_arn: String },
    #[error("No resource policy found for resource ARN {resource_arn}.")]
    NoResourcePolicy { resource_arn: String },
    #[error("Stream ARN {stream_arn} not found.")]
    StreamArnModeNotFound { stream_arn: String },

    // ValidationException
    #[error("Missing StreamName or StreamARN")]
    MissingStreamIdentifier,
    #[error(
        "Request is invalid. Stream {stream_name} under account {account_id} is in On-Demand mode."
    )]
    StreamOnDemand {
        stream_name: String,
        account_id: String,
    },
    #[error(
        "1 validation error detected: Value '{shard_id}' at 'shardToSplit' failed to satisfy constraint: Member must satisfy regular expression pattern: [a-zA-Z0-9_.-]+"
    )]
    InvalidShardIdPattern { shard_id: String },
    #[error(
        "1 validation error detected: Value '{hash_key}' at 'newStartingHashKey' failed to satisfy constraint: Member must satisfy regular expression pattern: 0|([1-9]\\d{{0,38}})"
    )]
    InvalidHashKeyPattern { hash_key: String },

    // InvalidArgumentException
    #[error(
        "Minimum allowed retention period is 24 hours. Requested retention period ({hours} hours) is too short."
    )]
    RetentionPeriodTooShort { hours: u32 },
    #[error(
        "Maximum allowed retention period is 8760 hours. Requested retention period ({hours} hours) is too long."
    )]
    RetentionPeriodTooLong { hours: u32 },
    #[error(
        "Requested retention period ({hours} hours) for stream {stream_name} can not be shorter than existing retention period ({current} hours). Use DecreaseRetentionPeriod API."
    )]
    RetentionCannotShorten {
        hours: u32,
        stream_name: String,
        current: u32,
    },
    #[error(
        "Requested retention period ({hours} hours) for stream {stream_name} can not be longer than existing retention period ({current} hours). Use IncreaseRetentionPeriod API."
    )]
    RetentionCannotLengthen {
        hours: u32,
        stream_name: String,
        current: u32,
    },
    #[error("{adjacent_shard}")]
    ShardsNotAdjacent { adjacent_shard: String },
    #[error(
        "Shard {shard_id} in stream {stream_name} under account {account_id} has already been merged or split, and thus is not eligible for merging or splitting."
    )]
    ShardAlreadyClosed {
        shard_id: String,
        stream_name: String,
        account_id: String,
    },
    #[error(
        "NewStartingHashKey {new_starting_hash_key} used in SplitShard() on shard {shard_id} in stream {stream_name} under account {account_id} is not both greater than one plus the shard's StartingHashKey {starting_hash_key} and less than the shard's EndingHashKey {ending_hash_key}."
    )]
    InvalidHashKeyForSplit {
        new_starting_hash_key: String,
        shard_id: String,
        stream_name: String,
        account_id: String,
        starting_hash_key: String,
        ending_hash_key: String,
    },
}

/// Maximum hash key value for Kinesis (2^128 - 1)
const MAX_HASH_KEY: &str = "340282366920938463463374607431768211455";

impl KinesisState {
    pub fn create_stream(
        &mut self,
        name: &str,
        shard_count: Option<i32>,
        stream_mode: Option<&str>,
        account_id: &str,
        region: &str,
    ) -> Result<&Stream, KinesisError> {
        if self.streams.contains_key(name) {
            return Err(KinesisError::StreamAlreadyExists {
                name: name.to_string(),
                account_id: account_id.to_string(),
            });
        }

        let arn = format!("arn:aws:kinesis:{region}:{account_id}:stream/{name}");

        let mode = stream_mode.unwrap_or("PROVISIONED");
        let effective_shard_count = if mode == "ON_DEMAND" {
            4 // AWS default for on-demand
        } else {
            shard_count.unwrap_or(1)
        };

        let shards = create_initial_shards(effective_shard_count);

        let stream = Stream {
            name: name.to_string(),
            arn,
            status: "ACTIVE".to_string(),
            shards,
            records: Vec::new(),
            created_timestamp: Utc::now(),
            tags: HashMap::new(),
            consumers: Vec::new(),
            retention_period_hours: 24,
            encryption_type: "NONE".to_string(),
            key_id: None,
            enhanced_monitoring: Vec::new(),
            resource_policy: None,
            stream_mode: mode.to_string(),
            account_id: account_id.to_string(),
            max_record_size_in_ki_b: None,
        };

        self.streams.insert(name.to_string(), stream);
        Ok(self.streams.get(name).unwrap())
    }

    pub fn delete_stream(&mut self, name: &str, account_id: &str) -> Result<(), KinesisError> {
        if self.streams.remove(name).is_none() {
            return Err(KinesisError::StreamNotFound {
                name: name.to_string(),
                account_id: account_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn delete_stream_by_arn(
        &mut self,
        arn: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let name = self
            .streams
            .iter()
            .find(|(_, s)| s.arn == arn)
            .map(|(k, _)| k.clone())
            .ok_or_else(|| KinesisError::StreamArnNotFound {
                arn: arn.to_string(),
                account_id: account_id.to_string(),
            })?;
        self.streams.remove(&name);
        Ok(())
    }

    pub fn describe_stream(&self, name: &str, account_id: &str) -> Result<&Stream, KinesisError> {
        self.streams
            .get(name)
            .ok_or_else(|| KinesisError::StreamNotFound {
                name: name.to_string(),
                account_id: account_id.to_string(),
            })
    }

    pub fn describe_stream_by_arn(
        &self,
        arn: &str,
        account_id: &str,
    ) -> Result<&Stream, KinesisError> {
        self.streams.values().find(|s| s.arn == arn).ok_or_else(|| {
            KinesisError::StreamArnNotFound {
                arn: arn.to_string(),
                account_id: account_id.to_string(),
            }
        })
    }

    pub fn list_streams(&self) -> Vec<&Stream> {
        self.streams.values().collect()
    }

    pub fn put_record(
        &mut self,
        stream_name: &str,
        partition_key: &str,
        account_id: &str,
    ) -> Result<(String, String), KinesisError> {
        let stream =
            self.streams
                .get_mut(stream_name)
                .ok_or_else(|| KinesisError::StreamNotFound {
                    name: stream_name.to_string(),
                    account_id: account_id.to_string(),
                })?;

        self.next_sequence += 1;
        let sequence_number = format!("{}", self.next_sequence);
        let active_shards: Vec<&ShardData> = stream.shards.iter().filter(|s| !s.closed).collect();
        let shard_id = compute_shard_id_from_shards(partition_key, &active_shards);

        let record = Record {
            sequence_number: sequence_number.clone(),
            partition_key: partition_key.to_string(),
            shard_id: shard_id.clone(),
            timestamp: Utc::now(),
        };

        stream.records.push(record);
        Ok((sequence_number, shard_id))
    }

    pub fn put_record_by_arn(
        &mut self,
        stream_arn: &str,
        partition_key: &str,
        account_id: &str,
    ) -> Result<(String, String), KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == stream_arn)
            .ok_or_else(|| KinesisError::StreamArnNotFound {
                arn: stream_arn.to_string(),
                account_id: account_id.to_string(),
            })?;

        self.next_sequence += 1;
        let sequence_number = format!("{}", self.next_sequence);
        let active_shards: Vec<&ShardData> = stream.shards.iter().filter(|s| !s.closed).collect();
        let shard_id = compute_shard_id_from_shards(partition_key, &active_shards);

        let record = Record {
            sequence_number: sequence_number.clone(),
            partition_key: partition_key.to_string(),
            shard_id: shard_id.clone(),
            timestamp: Utc::now(),
        };

        stream.records.push(record);
        Ok((sequence_number, shard_id))
    }

    pub fn put_records(
        &mut self,
        stream_name: &str,
        records: Vec<String>, // partition_key per record
        account_id: &str,
    ) -> Result<Vec<(String, String)>, KinesisError> {
        let stream =
            self.streams
                .get_mut(stream_name)
                .ok_or_else(|| KinesisError::StreamNotFound {
                    name: stream_name.to_string(),
                    account_id: account_id.to_string(),
                })?;

        let active_shards: Vec<ShardData> = stream
            .shards
            .iter()
            .filter(|s| !s.closed)
            .cloned()
            .collect();
        let active_refs: Vec<&ShardData> = active_shards.iter().collect();
        let mut results = Vec::new();
        for partition_key in records {
            self.next_sequence += 1;
            let sequence_number = format!("{}", self.next_sequence);
            let shard_id = compute_shard_id_from_shards(&partition_key, &active_refs);

            let record = Record {
                sequence_number: sequence_number.clone(),
                partition_key,
                shard_id: shard_id.clone(),
                timestamp: Utc::now(),
            };

            stream.records.push(record);
            results.push((sequence_number, shard_id));
        }
        Ok(results)
    }

    pub fn get_records(
        &self,
        stream_name: &str,
        shard_id: Option<&str>,
        limit: Option<usize>,
        account_id: &str,
    ) -> Result<Vec<&Record>, KinesisError> {
        let stream = self
            .streams
            .get(stream_name)
            .ok_or_else(|| KinesisError::StreamNotFound {
                name: stream_name.to_string(),
                account_id: account_id.to_string(),
            })?;

        let limit = limit.unwrap_or(100);
        let iter = stream.records.iter().filter(|r| match shard_id {
            Some(sid) => r.shard_id == sid,
            None => true,
        });
        Ok(iter.take(limit).collect())
    }

    pub fn get_shard_iterator(
        &self,
        stream_name: &str,
        shard_id: &str,
        account_id: &str,
    ) -> Result<String, KinesisError> {
        let stream = self
            .streams
            .get(stream_name)
            .ok_or_else(|| KinesisError::StreamNotFound {
                name: stream_name.to_string(),
                account_id: account_id.to_string(),
            })?;
        // Validate shard exists
        if !stream.shards.iter().any(|s| s.shard_id == shard_id) {
            return Err(KinesisError::ShardDoesNotExist {
                shard_id: shard_id.to_string(),
                stream_name: stream_name.to_string(),
                account_id: account_id.to_string(),
            });
        }
        Ok(format!("iter:{stream_name}:{shard_id}"))
    }

    fn get_stream_mut(
        &mut self,
        name: &str,
        account_id: &str,
    ) -> Result<&mut Stream, KinesisError> {
        self.streams
            .get_mut(name)
            .ok_or_else(|| KinesisError::StreamNotFound {
                name: name.to_string(),
                account_id: account_id.to_string(),
            })
    }

    fn get_stream(&self, name: &str, account_id: &str) -> Result<&Stream, KinesisError> {
        self.streams
            .get(name)
            .ok_or_else(|| KinesisError::StreamNotFound {
                name: name.to_string(),
                account_id: account_id.to_string(),
            })
    }

    fn get_stream_by_name_or_arn_mut(
        &mut self,
        name: Option<&str>,
        arn: Option<&str>,
        account_id: &str,
    ) -> Result<&mut Stream, KinesisError> {
        if let Some(name) = name {
            return self.get_stream_mut(name, account_id);
        }
        if let Some(arn) = arn {
            return self
                .streams
                .values_mut()
                .find(|s| s.arn == arn)
                .ok_or_else(|| KinesisError::StreamArnNotFound {
                    arn: arn.to_string(),
                    account_id: account_id.to_string(),
                });
        }
        Err(KinesisError::MissingStreamIdentifier)
    }

    fn get_stream_by_name_or_arn(
        &self,
        name: Option<&str>,
        arn: Option<&str>,
        account_id: &str,
    ) -> Result<&Stream, KinesisError> {
        if let Some(name) = name {
            return self.get_stream(name, account_id);
        }
        if let Some(arn) = arn {
            return self.streams.values().find(|s| s.arn == arn).ok_or_else(|| {
                KinesisError::StreamArnNotFound {
                    arn: arn.to_string(),
                    account_id: account_id.to_string(),
                }
            });
        }
        Err(KinesisError::MissingStreamIdentifier)
    }

    pub fn add_tags_to_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        tags: HashMap<String, String>,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        stream.tags.extend(tags);
        Ok(())
    }

    pub fn remove_tags_from_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        tag_keys: &[String],
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        for key in tag_keys {
            stream.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags_for_stream(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        account_id: &str,
    ) -> Result<&HashMap<String, String>, KinesisError> {
        let stream = self.get_stream_by_name_or_arn(stream_name, stream_arn, account_id)?;
        Ok(&stream.tags)
    }

    pub fn register_stream_consumer(
        &mut self,
        stream_arn: &str,
        consumer_name: &str,
    ) -> Result<StreamConsumer, KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == stream_arn)
            .ok_or_else(|| KinesisError::StreamArnDoesNotExist {
                stream_arn: stream_arn.to_string(),
            })?;

        if stream
            .consumers
            .iter()
            .any(|c| c.consumer_name == consumer_name)
        {
            return Err(KinesisError::ConsumerAlreadyExists {
                consumer_name: consumer_name.to_string(),
            });
        }

        // ARN format without version suffix to match moto/AWS
        let consumer_arn = format!("{stream_arn}/consumer/{consumer_name}");
        let consumer = StreamConsumer {
            consumer_name: consumer_name.to_string(),
            consumer_arn: consumer_arn.clone(),
            consumer_status: "ACTIVE".to_string(),
            creation_timestamp: Utc::now(),
        };

        stream.consumers.push(consumer.clone());
        Ok(consumer)
    }

    pub fn deregister_stream_consumer(
        &mut self,
        stream_arn: &str,
        consumer_name: Option<&str>,
        consumer_arn: Option<&str>,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == stream_arn || consumer_arn.is_some_and(|ca| ca.starts_with(&s.arn)))
            .ok_or_else(|| KinesisError::ConsumerNotFound {
                consumer: consumer_arn.unwrap_or("unknown").to_string(),
                account_id: account_id.to_string(),
            })?;

        let idx = stream.consumers.iter().position(|c| {
            consumer_arn.is_some_and(|ca| c.consumer_arn == ca)
                || consumer_name.is_some_and(|cn| c.consumer_name == cn)
        });

        match idx {
            Some(i) => {
                stream.consumers.remove(i);
                Ok(())
            }
            None => Err(KinesisError::ConsumerNotFound {
                consumer: consumer_arn.unwrap_or("unknown").to_string(),
                account_id: account_id.to_string(),
            }),
        }
    }

    pub fn describe_stream_consumer(
        &self,
        stream_arn: &str,
        consumer_name: Option<&str>,
        consumer_arn: Option<&str>,
        account_id: &str,
    ) -> Result<(&StreamConsumer, &str), KinesisError> {
        let stream = self
            .streams
            .values()
            .find(|s| s.arn == stream_arn || consumer_arn.is_some_and(|ca| ca.starts_with(&s.arn)))
            .ok_or_else(|| KinesisError::ConsumerNotFound {
                consumer: consumer_arn.unwrap_or(stream_arn).to_string(),
                account_id: account_id.to_string(),
            })?;

        let consumer = stream
            .consumers
            .iter()
            .find(|c| {
                consumer_arn.is_some_and(|ca| c.consumer_arn == ca)
                    || consumer_name.is_some_and(|cn| c.consumer_name == cn)
            })
            .ok_or_else(|| KinesisError::ConsumerNotFound {
                consumer: consumer_arn.unwrap_or("unknown").to_string(),
                account_id: account_id.to_string(),
            })?;

        Ok((consumer, &stream.arn))
    }

    pub fn list_stream_consumers(
        &self,
        stream_arn: &str,
    ) -> Result<Vec<&StreamConsumer>, KinesisError> {
        let stream = self
            .streams
            .values()
            .find(|s| s.arn == stream_arn)
            .ok_or_else(|| KinesisError::StreamArnDoesNotExist {
                stream_arn: stream_arn.to_string(),
            })?;

        Ok(stream.consumers.iter().collect())
    }

    pub fn increase_retention_period(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        hours: u32,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        // Validate bounds
        if hours < 24 {
            return Err(KinesisError::RetentionPeriodTooShort { hours });
        }
        if hours > 8760 {
            return Err(KinesisError::RetentionPeriodTooLong { hours });
        }

        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        let current = stream.retention_period_hours;
        let sname = stream.name.clone();
        if hours < current {
            return Err(KinesisError::RetentionCannotShorten {
                hours,
                stream_name: sname,
                current,
            });
        }
        stream.retention_period_hours = hours;
        Ok(())
    }

    pub fn decrease_retention_period(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        hours: u32,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        // Validate bounds
        if hours < 24 {
            return Err(KinesisError::RetentionPeriodTooShort { hours });
        }
        if hours > 8760 {
            return Err(KinesisError::RetentionPeriodTooLong { hours });
        }

        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        let current = stream.retention_period_hours;
        let sname = stream.name.clone();
        if hours > current {
            return Err(KinesisError::RetentionCannotLengthen {
                hours,
                stream_name: sname,
                current,
            });
        }
        stream.retention_period_hours = hours;
        Ok(())
    }

    pub fn start_stream_encryption(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        encryption_type: &str,
        key_id: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        stream.encryption_type = encryption_type.to_string();
        stream.key_id = Some(key_id.to_string());
        Ok(())
    }

    pub fn stop_stream_encryption(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        stream.encryption_type = "NONE".to_string();
        stream.key_id = None;
        Ok(())
    }

    /// Returns (current_before, desired_after, stream_name, stream_arn)
    pub fn enable_enhanced_monitoring(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        shard_level_metrics: Vec<String>,
        account_id: &str,
    ) -> Result<(Vec<String>, Vec<String>, String, String), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        let current_before = stream.enhanced_monitoring.clone();
        for metric in &shard_level_metrics {
            if !stream.enhanced_monitoring.contains(metric) {
                stream.enhanced_monitoring.push(metric.clone());
            }
        }
        let desired_after = stream.enhanced_monitoring.clone();
        let sname = stream.name.clone();
        let sarn = stream.arn.clone();
        Ok((current_before, desired_after, sname, sarn))
    }

    /// Returns (current_before, desired_after, stream_name, stream_arn)
    pub fn disable_enhanced_monitoring(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        shard_level_metrics: Vec<String>,
        account_id: &str,
    ) -> Result<(Vec<String>, Vec<String>, String, String), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        let current_before = stream.enhanced_monitoring.clone();
        if shard_level_metrics.contains(&"ALL".to_string()) {
            stream.enhanced_monitoring.clear();
        } else {
            stream
                .enhanced_monitoring
                .retain(|m| !shard_level_metrics.contains(m));
        }
        let desired_after = stream.enhanced_monitoring.clone();
        let sname = stream.name.clone();
        let sarn = stream.arn.clone();
        Ok((current_before, desired_after, sname, sarn))
    }

    pub fn put_resource_policy(
        &mut self,
        resource_arn: &str,
        policy: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == resource_arn)
            .ok_or_else(|| KinesisError::ResourceStreamNotFound {
                resource_arn: resource_arn.to_string(),
                account_id: account_id.to_string(),
            })?;
        stream.resource_policy = Some(policy.to_string());
        Ok(())
    }

    pub fn get_resource_policy(
        &self,
        resource_arn: &str,
        account_id: &str,
    ) -> Result<Option<&str>, KinesisError> {
        let stream = self
            .streams
            .values()
            .find(|s| s.arn == resource_arn)
            .ok_or_else(|| KinesisError::ResourceStreamNotFound {
                resource_arn: resource_arn.to_string(),
                account_id: account_id.to_string(),
            })?;
        Ok(stream.resource_policy.as_deref())
    }

    pub fn delete_resource_policy(
        &mut self,
        resource_arn: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == resource_arn)
            .ok_or_else(|| KinesisError::ResourceStreamNotFound {
                resource_arn: resource_arn.to_string(),
                account_id: account_id.to_string(),
            })?;
        if stream.resource_policy.is_none() {
            return Err(KinesisError::NoResourcePolicy {
                resource_arn: resource_arn.to_string(),
            });
        }
        stream.resource_policy = None;
        Ok(())
    }

    pub fn update_stream_mode(
        &mut self,
        stream_arn: &str,
        stream_mode: &str,
    ) -> Result<(), KinesisError> {
        let stream = self
            .streams
            .values_mut()
            .find(|s| s.arn == stream_arn)
            .ok_or_else(|| KinesisError::StreamArnModeNotFound {
                stream_arn: stream_arn.to_string(),
            })?;
        stream.stream_mode = stream_mode.to_string();
        Ok(())
    }

    pub fn update_shard_count(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        target_shard_count: i32,
        account_id: &str,
    ) -> Result<(i32, i32, String), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;

        // Check if stream is on-demand
        if stream.stream_mode == "ON_DEMAND" {
            let sname = stream.name.clone();
            return Err(KinesisError::StreamOnDemand {
                stream_name: sname,
                account_id: account_id.to_string(),
            });
        }

        let current_active: i32 = stream.shards.iter().filter(|s| !s.closed).count() as i32;
        let sname = stream.name.clone();

        if target_shard_count > current_active {
            // Scale up: split shards
            self.do_scale_up(
                stream_name,
                stream_arn,
                current_active,
                target_shard_count,
                account_id,
            )?;
        } else if target_shard_count < current_active {
            // Scale down: merge shards
            self.do_scale_down(
                stream_name,
                stream_arn,
                current_active,
                target_shard_count,
                account_id,
            )?;
        }

        Ok((current_active, target_shard_count, sname))
    }

    fn do_scale_up(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        current: i32,
        target: i32,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        // Iteratively double shards until we reach or exceed target
        let mut active_count = current;
        while active_count < target {
            let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
            // Find active shards to split
            let active_shard_ids: Vec<String> = stream
                .shards
                .iter()
                .filter(|s| !s.closed)
                .map(|s| s.shard_id.clone())
                .collect();

            let need = (target - active_count).min(active_count) as usize;
            let to_split: Vec<String> = active_shard_ids.into_iter().take(need).collect();

            for shard_id in to_split {
                let stream =
                    self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
                let shard_idx = stream
                    .shards
                    .iter()
                    .position(|s| s.shard_id == shard_id)
                    .unwrap();
                let shard = &stream.shards[shard_idx];
                let start = parse_big_int(&shard.starting_hash_key);
                let end = parse_big_int(&shard.ending_hash_key);
                let mid = (&start + &end) / 2u64;
                let mid_str = mid.to_string();
                let mid_plus_one = (&mid + 1u64).to_string();

                let next_id = stream.shards.len() as i32;
                let parent_id = shard.shard_id.clone();
                let parent_start = shard.starting_hash_key.clone();
                let parent_end = shard.ending_hash_key.clone();

                // Close parent
                stream.shards[shard_idx].closed = true;

                // Create two children
                stream.shards.push(ShardData {
                    shard_id: format!("shardId-{:012}", next_id),
                    starting_hash_key: parent_start,
                    ending_hash_key: mid_str,
                    parent_shard_id: Some(parent_id.clone()),
                    adjacent_parent_shard_id: None,
                    closed: false,
                });
                stream.shards.push(ShardData {
                    shard_id: format!("shardId-{:012}", next_id + 1),
                    starting_hash_key: mid_plus_one,
                    ending_hash_key: parent_end,
                    parent_shard_id: Some(parent_id),
                    adjacent_parent_shard_id: None,
                    closed: false,
                });
            }

            let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
            active_count = stream.shards.iter().filter(|s| !s.closed).count() as i32;
        }
        Ok(())
    }

    fn do_scale_down(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        _current: i32,
        target: i32,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        loop {
            let stream = self.get_stream_by_name_or_arn(stream_name, stream_arn, account_id)?;
            let active_count = stream.shards.iter().filter(|s| !s.closed).count() as i32;
            if active_count <= target {
                break;
            }

            // Get current active shards, sorted by starting hash key for adjacency
            let stream = self.get_stream_by_name_or_arn(stream_name, stream_arn, account_id)?;
            let mut active: Vec<(String, String)> = stream
                .shards
                .iter()
                .filter(|s| !s.closed)
                .map(|s| (s.shard_id.clone(), s.starting_hash_key.clone()))
                .collect();
            active.sort_by(|a, b| {
                let a_val = parse_big_int(&a.1);
                let b_val = parse_big_int(&b.1);
                a_val.cmp(&b_val)
            });

            if active.len() < 2 {
                break;
            }

            // Merge adjacent pairs
            let merges = ((active_count - target) as usize).min(active.len() / 2);
            for i in 0..merges {
                let shard1 = active[i * 2].0.clone();
                let shard2 = active[i * 2 + 1].0.clone();
                self.do_merge_shards(stream_name, stream_arn, &shard1, &shard2, account_id)?;
            }
        }
        Ok(())
    }

    pub fn list_shards(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        account_id: &str,
    ) -> Result<&Vec<ShardData>, KinesisError> {
        let stream = self.get_stream_by_name_or_arn(stream_name, stream_arn, account_id)?;
        Ok(&stream.shards)
    }

    pub fn merge_shards(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        shard_to_merge: &str,
        adjacent_shard: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        // Validate adjacency
        {
            let stream = self.get_stream_by_name_or_arn(stream_name, stream_arn, account_id)?;

            let shard1 = stream
                .shards
                .iter()
                .find(|s| s.shard_id == shard_to_merge && !s.closed);
            let shard2 = stream
                .shards
                .iter()
                .find(|s| s.shard_id == adjacent_shard && !s.closed);

            if shard1.is_none() || shard2.is_none() {
                return Err(KinesisError::ShardNotFound);
            }

            let s1 = shard1.unwrap();
            let s2 = shard2.unwrap();

            // Check adjacency: s1.ending_hash_key + 1 == s2.starting_hash_key (or vice versa)
            let s1_end = parse_big_int(&s1.ending_hash_key);
            let s2_start = parse_big_int(&s2.starting_hash_key);
            let s2_end = parse_big_int(&s2.ending_hash_key);
            let s1_start = parse_big_int(&s1.starting_hash_key);

            let adjacent = (&s1_end + 1u64) == s2_start || (&s2_end + 1u64) == s1_start;
            if !adjacent {
                return Err(KinesisError::ShardsNotAdjacent {
                    adjacent_shard: adjacent_shard.to_string(),
                });
            }
        }

        self.do_merge_shards(
            stream_name,
            stream_arn,
            shard_to_merge,
            adjacent_shard,
            account_id,
        )
    }

    fn do_merge_shards(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        shard_to_merge: &str,
        adjacent_shard: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;

        let s1_idx = stream
            .shards
            .iter()
            .position(|s| s.shard_id == shard_to_merge)
            .unwrap();
        let s2_idx = stream
            .shards
            .iter()
            .position(|s| s.shard_id == adjacent_shard)
            .unwrap();

        let s1_start = stream.shards[s1_idx].starting_hash_key.clone();
        let s1_end = stream.shards[s1_idx].ending_hash_key.clone();
        let s2_start = stream.shards[s2_idx].starting_hash_key.clone();
        let s2_end = stream.shards[s2_idx].ending_hash_key.clone();

        let new_start = if parse_big_int(&s1_start) < parse_big_int(&s2_start) {
            s1_start
        } else {
            s2_start
        };
        let new_end = if parse_big_int(&s1_end) > parse_big_int(&s2_end) {
            s1_end
        } else {
            s2_end
        };

        let next_id = stream.shards.len() as i32;
        let parent_id = stream.shards[s1_idx].shard_id.clone();
        let adj_parent_id = stream.shards[s2_idx].shard_id.clone();

        stream.shards[s1_idx].closed = true;
        stream.shards[s2_idx].closed = true;

        stream.shards.push(ShardData {
            shard_id: format!("shardId-{:012}", next_id),
            starting_hash_key: new_start,
            ending_hash_key: new_end,
            parent_shard_id: Some(parent_id),
            adjacent_parent_shard_id: Some(adj_parent_id),
            closed: false,
        });

        Ok(())
    }

    pub fn split_shard(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        shard_to_split: &str,
        new_starting_hash_key: &str,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        // Validate shard name pattern
        let shard_pattern = regex::Regex::new(r"^[a-zA-Z0-9_.\-]+$").unwrap();
        if !shard_pattern.is_match(shard_to_split) {
            return Err(KinesisError::InvalidShardIdPattern {
                shard_id: shard_to_split.to_string(),
            });
        }

        // Validate hash key pattern
        let hash_pattern = regex::Regex::new(r"^0|([1-9]\d{0,38})$").unwrap();
        if !hash_pattern.is_match(new_starting_hash_key) {
            return Err(KinesisError::InvalidHashKeyPattern {
                hash_key: new_starting_hash_key.to_string(),
            });
        }

        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        let sname = stream.name.clone();
        let saccount = stream.account_id.clone();

        let shard_idx = stream
            .shards
            .iter()
            .position(|s| s.shard_id == shard_to_split)
            .ok_or_else(|| KinesisError::ShardInStreamNotFound {
                shard_id: shard_to_split.to_string(),
                stream_name: sname.clone(),
                account_id: saccount.clone(),
            })?;

        if stream.shards[shard_idx].closed {
            return Err(KinesisError::ShardAlreadyClosed {
                shard_id: shard_to_split.to_string(),
                stream_name: sname.clone(),
                account_id: saccount.clone(),
            });
        }

        // Validate hash key is within shard range
        let shard_start = parse_big_int(&stream.shards[shard_idx].starting_hash_key);
        let shard_end = parse_big_int(&stream.shards[shard_idx].ending_hash_key);
        let new_key = parse_big_int(new_starting_hash_key);

        if new_key <= &shard_start + 1u64 || new_key >= shard_end {
            return Err(KinesisError::InvalidHashKeyForSplit {
                new_starting_hash_key: new_starting_hash_key.to_string(),
                shard_id: shard_to_split.to_string(),
                stream_name: sname.clone(),
                account_id: saccount.clone(),
                starting_hash_key: stream.shards[shard_idx].starting_hash_key.clone(),
                ending_hash_key: stream.shards[shard_idx].ending_hash_key.clone(),
            });
        }

        let parent_start = stream.shards[shard_idx].starting_hash_key.clone();
        let parent_end = stream.shards[shard_idx].ending_hash_key.clone();
        let parent_id = stream.shards[shard_idx].shard_id.clone();
        let next_id = stream.shards.len() as i32;

        let new_key_minus_one = (&new_key - 1u64).to_string();

        // Close parent shard
        stream.shards[shard_idx].closed = true;

        // Create two new child shards
        stream.shards.push(ShardData {
            shard_id: format!("shardId-{:012}", next_id),
            starting_hash_key: parent_start,
            ending_hash_key: new_key_minus_one,
            parent_shard_id: Some(parent_id.clone()),
            adjacent_parent_shard_id: None,
            closed: false,
        });
        stream.shards.push(ShardData {
            shard_id: format!("shardId-{:012}", next_id + 1),
            starting_hash_key: new_starting_hash_key.to_string(),
            ending_hash_key: parent_end,
            parent_shard_id: Some(parent_id),
            adjacent_parent_shard_id: None,
            closed: false,
        });

        Ok(())
    }

    /// Get the number of active (open) shards for a stream
    pub fn open_shard_count(&self, stream: &Stream) -> i32 {
        stream.shards.iter().filter(|s| !s.closed).count() as i32
    }

    // --------------- Resource-level tag operations (TagResource / UntagResource / ListTagsForResource) ---------------

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), KinesisError> {
        // Verify the resource exists (stream identified by ARN)
        let exists = self.streams.values().any(|s| s.arn == resource_arn);
        if !exists {
            return Err(KinesisError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        self.resource_tags
            .entry(resource_arn.to_string())
            .or_default()
            .extend(tags);
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), KinesisError> {
        let exists = self.streams.values().any(|s| s.arn == resource_arn);
        if !exists {
            return Err(KinesisError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        if let Some(map) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                map.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<Vec<(String, String)>, KinesisError> {
        let exists = self.streams.values().any(|s| s.arn == resource_arn);
        if !exists {
            return Err(KinesisError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        let tags = self
            .resource_tags
            .get(resource_arn)
            .map(|m| {
                let mut v: Vec<(String, String)> =
                    m.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                v.sort_by(|a, b| a.0.cmp(&b.0));
                v
            })
            .unwrap_or_default();
        Ok(tags)
    }

    // --------------- Account settings ---------------

    pub fn describe_account_settings(&self) -> &str {
        &self.account_settings_commitment_status
    }

    pub fn update_account_settings(&mut self, commitment_status: Option<&str>) {
        if let Some(s) = commitment_status {
            self.account_settings_commitment_status = s.to_string();
        }
    }

    // --------------- UpdateMaxRecordSize (stream-level, stored on stream) ---------------

    pub fn update_max_record_size(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        max_record_size_in_ki_b: i32,
        account_id: &str,
    ) -> Result<(), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        stream.max_record_size_in_ki_b = Some(max_record_size_in_ki_b);
        Ok(())
    }

    // --------------- UpdateStreamWarmThroughput (stream-level, stored on stream) ---------------

    pub fn update_stream_warm_throughput(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        account_id: &str,
    ) -> Result<(String, String), KinesisError> {
        let stream = self.get_stream_by_name_or_arn_mut(stream_name, stream_arn, account_id)?;
        Ok((stream.name.clone(), stream.arn.clone()))
    }
}

/// Create initial shards for a stream with evenly divided hash key ranges
fn create_initial_shards(shard_count: i32) -> Vec<ShardData> {
    let max_hash = parse_big_int(MAX_HASH_KEY);
    let mut shards = Vec::new();

    for i in 0..shard_count {
        let start = &max_hash * i as u64 / shard_count as u64;
        let end = if i == shard_count - 1 {
            max_hash.clone()
        } else {
            &max_hash * (i + 1) as u64 / shard_count as u64 - 1u64
        };

        shards.push(ShardData {
            shard_id: format!("shardId-{:012}", i),
            starting_hash_key: start.to_string(),
            ending_hash_key: end.to_string(),
            parent_shard_id: None,
            adjacent_parent_shard_id: None,
            closed: false,
        });
    }

    shards
}

/// Simple big integer type using num-bigint would be ideal, but we use string-based arithmetic
/// For now, use a Vec<u8> representation
fn parse_big_int(s: &str) -> num_bigint::BigUint {
    s.parse::<num_bigint::BigUint>().unwrap_or_default()
}

/// Compute shard ID from partition key using hash, matching to active shard ranges
fn compute_shard_id_from_shards(partition_key: &str, active_shards: &[&ShardData]) -> String {
    if active_shards.is_empty() {
        return "shardId-000000000000".to_string();
    }

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    partition_key.hash(&mut hasher);
    let hash = hasher.finish();

    // Map to hash key space
    let max_hash = parse_big_int(MAX_HASH_KEY);
    let hash_key = &max_hash * hash / u64::MAX;

    for shard in active_shards {
        let start = parse_big_int(&shard.starting_hash_key);
        let end = parse_big_int(&shard.ending_hash_key);
        if hash_key >= start && hash_key <= end {
            return shard.shard_id.clone();
        }
    }

    // Fallback to first active shard
    active_shards[0].shard_id.clone()
}
