use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct KinesisVideoState {
    /// Streams keyed by stream name.
    pub streams: HashMap<String, Stream>,
    /// Signaling channels keyed by channel name.
    pub channels: HashMap<String, SignalingChannel>,
}

#[derive(Debug, thiserror::Error)]
pub enum KinesisVideoError {
    #[error("The stream {0} already exists.")]
    StreamAlreadyExists(String),
    #[error("The channel {0} already exists.")]
    ChannelAlreadyExists(String),
    #[error("The stream {0} was not found.")]
    StreamNotFoundByName(String),
    #[error("The stream with ARN {0} was not found.")]
    StreamNotFoundByArn(String),
    #[error("The channel {0} was not found.")]
    ChannelNotFoundByName(String),
    #[error("The channel with ARN {0} was not found.")]
    ChannelNotFoundByArn(String),
    #[error("Resource with ARN {0} was not found.")]
    ResourceNotFoundByArn(String),
    #[error("No edge configuration found for this stream.")]
    EdgeConfigNotFound,
    #[error("Either StreamName or StreamARN must be specified.")]
    StreamIdentifierRequired,
    #[error("Either ChannelName or ChannelARN must be specified.")]
    ChannelIdentifierRequired,
    #[error("Unknown operation: {0}")]
    UnknownOperation(String),
    #[error("Version mismatch: expected {expected} but stream version is {actual}.")]
    StreamVersionMismatch { expected: String, actual: String },
    #[error("Version mismatch: expected {expected} but channel version is {actual}.")]
    ChannelVersionMismatch { expected: String, actual: String },
}

impl KinesisVideoState {
    // =========================================================================
    // Stream operations
    // =========================================================================

    pub fn create_stream(
        &mut self,
        stream_name: &str,
        device_name: Option<&str>,
        media_type: Option<&str>,
        kms_key_id: Option<&str>,
        data_retention_in_hours: Option<i32>,
        account_id: &str,
        region: &str,
    ) -> Result<&Stream, KinesisVideoError> {
        if self.streams.contains_key(stream_name) {
            return Err(KinesisVideoError::StreamAlreadyExists(
                stream_name.to_string(),
            ));
        }

        let stream_arn = format!(
            "arn:aws:kinesisvideo:{region}:{account_id}:stream/{stream_name}/{}",
            Utc::now().timestamp_millis()
        );
        let version = uuid::Uuid::new_v4().to_string();

        let stream = Stream {
            stream_name: stream_name.to_string(),
            stream_arn,
            device_name: device_name.map(String::from),
            media_type: media_type.map(String::from),
            kms_key_id: kms_key_id.unwrap_or("aws/kinesisvideo").to_string(),
            version,
            status: "ACTIVE".to_string(),
            creation_time: Utc::now(),
            data_retention_in_hours: data_retention_in_hours.unwrap_or(0),
            tags: HashMap::new(),
            image_generation_config: None,
            notification_config: None,
            storage_config: None,
            edge_config: None,
        };

        self.streams.insert(stream_name.to_string(), stream);
        Ok(self.streams.get(stream_name).unwrap())
    }

    pub fn describe_stream(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<&Stream, KinesisVideoError> {
        if let Some(name) = stream_name {
            self.streams
                .get(name)
                .ok_or_else(|| KinesisVideoError::StreamNotFoundByName(name.to_string()))
        } else if let Some(arn) = stream_arn {
            self.streams
                .values()
                .find(|s| s.stream_arn == arn)
                .ok_or_else(|| KinesisVideoError::StreamNotFoundByArn(arn.to_string()))
        } else {
            Err(KinesisVideoError::StreamIdentifierRequired)
        }
    }

    fn get_stream_mut(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<&mut Stream, KinesisVideoError> {
        let key = if let Some(name) = stream_name {
            if self.streams.contains_key(name) {
                name.to_string()
            } else {
                return Err(KinesisVideoError::StreamNotFoundByName(name.to_string()));
            }
        } else if let Some(arn) = stream_arn {
            match self.streams.values().find(|s| s.stream_arn == arn) {
                Some(s) => s.stream_name.clone(),
                None => {
                    return Err(KinesisVideoError::StreamNotFoundByArn(arn.to_string()));
                }
            }
        } else {
            return Err(KinesisVideoError::StreamIdentifierRequired);
        };
        Ok(self.streams.get_mut(&key).unwrap())
    }

    pub fn delete_stream(
        &mut self,
        stream_arn: &str,
        current_version: Option<&str>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.streams.values().find(|s| s.stream_arn == stream_arn);

        match stream {
            Some(s) => {
                // If caller supplied CurrentVersion, verify it matches.
                if let Some(ver) = current_version {
                    if ver != s.version {
                        return Err(KinesisVideoError::StreamVersionMismatch {
                            expected: ver.to_string(),
                            actual: s.version.clone(),
                        });
                    }
                }
                let name = s.stream_name.clone();
                self.streams.remove(&name);
                Ok(())
            }
            None => Err(KinesisVideoError::StreamNotFoundByArn(
                stream_arn.to_string(),
            )),
        }
    }

    pub fn list_streams(&self) -> Vec<&Stream> {
        self.streams.values().collect()
    }

    pub fn update_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        current_version: &str,
        device_name: Option<&str>,
        media_type: Option<&str>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;

        if stream.version != current_version {
            return Err(KinesisVideoError::StreamVersionMismatch {
                expected: current_version.to_string(),
                actual: stream.version.clone(),
            });
        }

        if let Some(name) = device_name {
            stream.device_name = Some(name.to_string());
        }
        if let Some(mt) = media_type {
            stream.media_type = Some(mt.to_string());
        }
        stream.version = uuid::Uuid::new_v4().to_string();
        Ok(())
    }

    pub fn update_data_retention(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        current_version: &str,
        operation: &str,
        data_retention_change_in_hours: i32,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;

        if stream.version != current_version {
            return Err(KinesisVideoError::StreamVersionMismatch {
                expected: current_version.to_string(),
                actual: stream.version.clone(),
            });
        }

        match operation {
            "INCREASE_DATA_RETENTION" => {
                stream.data_retention_in_hours += data_retention_change_in_hours;
            }
            "DECREASE_DATA_RETENTION" => {
                stream.data_retention_in_hours =
                    (stream.data_retention_in_hours - data_retention_change_in_hours).max(0);
            }
            _ => {
                return Err(KinesisVideoError::UnknownOperation(operation.to_string()));
            }
        }

        stream.version = uuid::Uuid::new_v4().to_string();
        Ok(())
    }

    pub fn get_data_endpoint(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        api_name: &str,
    ) -> Result<String, KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        // Return a mock endpoint URL
        let region = stream
            .stream_arn
            .split(':')
            .nth(3)
            .unwrap_or("us-east-1")
            .to_string();
        let endpoint = format!(
            "https://{}.kinesisvideo.{}.amazonaws.com",
            api_name.to_lowercase().replace('_', "-"),
            region
        );
        Ok(endpoint)
    }

    // =========================================================================
    // Stream tag operations
    // =========================================================================

    pub fn tag_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        for (k, v) in tags {
            stream.tags.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_stream(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        tag_keys: &[String],
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        for k in tag_keys {
            stream.tags.remove(k);
        }
        Ok(())
    }

    pub fn list_tags_for_stream(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<HashMap<String, String>, KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        Ok(stream.tags.clone())
    }

    // =========================================================================
    // Resource (channel) tag operations
    // =========================================================================

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: &[(String, String)],
    ) -> Result<(), KinesisVideoError> {
        // Try channels first
        if let Some(ch) = self
            .channels
            .values_mut()
            .find(|c| c.channel_arn == resource_arn)
        {
            for (k, v) in tags {
                ch.tags.insert(k.clone(), v.clone());
            }
            return Ok(());
        }
        Err(KinesisVideoError::ResourceNotFoundByArn(
            resource_arn.to_string(),
        ))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), KinesisVideoError> {
        if let Some(ch) = self
            .channels
            .values_mut()
            .find(|c| c.channel_arn == resource_arn)
        {
            for k in tag_keys {
                ch.tags.remove(k);
            }
            return Ok(());
        }
        Err(KinesisVideoError::ResourceNotFoundByArn(
            resource_arn.to_string(),
        ))
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, KinesisVideoError> {
        if let Some(ch) = self
            .channels
            .values()
            .find(|c| c.channel_arn == resource_arn)
        {
            return Ok(ch.tags.clone());
        }
        Err(KinesisVideoError::ResourceNotFoundByArn(
            resource_arn.to_string(),
        ))
    }

    // =========================================================================
    // Stream configuration operations
    // =========================================================================

    pub fn describe_image_generation_configuration(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<Option<&ImageGenerationConfig>, KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        Ok(stream.image_generation_config.as_ref())
    }

    pub fn update_image_generation_configuration(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        config: Option<ImageGenerationConfig>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        stream.image_generation_config = config;
        Ok(())
    }

    pub fn describe_notification_configuration(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<Option<&NotificationConfig>, KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        Ok(stream.notification_config.as_ref())
    }

    pub fn update_notification_configuration(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        config: Option<NotificationConfig>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        stream.notification_config = config;
        Ok(())
    }

    pub fn describe_stream_storage_configuration(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<(&Stream, Option<&StreamStorageConfig>), KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        Ok((stream, stream.storage_config.as_ref()))
    }

    pub fn update_stream_storage_configuration(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        current_version: &str,
        config: StreamStorageConfig,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        if stream.version != current_version {
            return Err(KinesisVideoError::StreamVersionMismatch {
                expected: current_version.to_string(),
                actual: stream.version.clone(),
            });
        }
        stream.storage_config = Some(config);
        stream.version = uuid::Uuid::new_v4().to_string();
        Ok(())
    }

    pub fn describe_mapped_resource_configuration(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<Vec<(String, String)>, KinesisVideoError> {
        // Verify the stream exists
        let _ = self.describe_stream(stream_name, stream_arn)?;
        // In this mock we return an empty list — the real API lists Rekognition/AppConfig bindings
        Ok(vec![])
    }

    // =========================================================================
    // Edge configuration operations
    // =========================================================================

    pub fn start_edge_configuration_update(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
        edge_config: EdgeConfigState,
    ) -> Result<&EdgeConfigState, KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        stream.edge_config = Some(edge_config);
        Ok(stream.edge_config.as_ref().unwrap())
    }

    pub fn describe_edge_configuration(
        &self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<(&Stream, &EdgeConfigState), KinesisVideoError> {
        let stream = self.describe_stream(stream_name, stream_arn)?;
        match stream.edge_config.as_ref() {
            Some(ec) => Ok((stream, ec)),
            None => Err(KinesisVideoError::EdgeConfigNotFound),
        }
    }

    pub fn delete_edge_configuration(
        &mut self,
        stream_name: Option<&str>,
        stream_arn: Option<&str>,
    ) -> Result<(), KinesisVideoError> {
        let stream = self.get_stream_mut(stream_name, stream_arn)?;
        stream.edge_config = None;
        Ok(())
    }

    pub fn list_edge_agent_configurations(
        &self,
        _hub_device_arn: &str,
    ) -> Vec<(&Stream, &EdgeConfigState)> {
        self.streams
            .values()
            .filter_map(|s| s.edge_config.as_ref().map(|ec| (s, ec)))
            .collect()
    }

    // =========================================================================
    // Signaling channel operations
    // =========================================================================

    pub fn create_signaling_channel(
        &mut self,
        channel_name: &str,
        channel_type: Option<&str>,
        message_ttl_seconds: Option<i32>,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&SignalingChannel, KinesisVideoError> {
        if self.channels.contains_key(channel_name) {
            return Err(KinesisVideoError::ChannelAlreadyExists(
                channel_name.to_string(),
            ));
        }

        let channel_arn = format!(
            "arn:aws:kinesisvideo:{region}:{account_id}:channel/{channel_name}/{}",
            Utc::now().timestamp_millis()
        );
        let version = uuid::Uuid::new_v4().to_string();

        let channel = SignalingChannel {
            channel_name: channel_name.to_string(),
            channel_arn,
            channel_type: channel_type.unwrap_or("SINGLE_MASTER").to_string(),
            channel_status: "ACTIVE".to_string(),
            creation_time: Utc::now(),
            version,
            message_ttl_seconds,
            tags: tags.into_iter().collect(),
            media_storage_config: None,
        };

        self.channels.insert(channel_name.to_string(), channel);
        Ok(self.channels.get(channel_name).unwrap())
    }

    pub fn describe_signaling_channel(
        &self,
        channel_name: Option<&str>,
        channel_arn: Option<&str>,
    ) -> Result<&SignalingChannel, KinesisVideoError> {
        if let Some(name) = channel_name {
            self.channels
                .get(name)
                .ok_or_else(|| KinesisVideoError::ChannelNotFoundByName(name.to_string()))
        } else if let Some(arn) = channel_arn {
            self.channels
                .values()
                .find(|c| c.channel_arn == arn)
                .ok_or_else(|| KinesisVideoError::ChannelNotFoundByArn(arn.to_string()))
        } else {
            Err(KinesisVideoError::ChannelIdentifierRequired)
        }
    }

    pub fn delete_signaling_channel(
        &mut self,
        channel_arn: &str,
        current_version: Option<&str>,
    ) -> Result<(), KinesisVideoError> {
        let channel = self
            .channels
            .values()
            .find(|c| c.channel_arn == channel_arn);

        match channel {
            Some(c) => {
                if let Some(ver) = current_version {
                    if ver != c.version {
                        return Err(KinesisVideoError::ChannelVersionMismatch {
                            expected: ver.to_string(),
                            actual: c.version.clone(),
                        });
                    }
                }
                let name = c.channel_name.clone();
                self.channels.remove(&name);
                Ok(())
            }
            None => Err(KinesisVideoError::ChannelNotFoundByArn(
                channel_arn.to_string(),
            )),
        }
    }

    pub fn list_signaling_channels(&self) -> Vec<&SignalingChannel> {
        self.channels.values().collect()
    }

    pub fn update_signaling_channel(
        &mut self,
        channel_arn: &str,
        current_version: &str,
        message_ttl_seconds: Option<i32>,
    ) -> Result<(), KinesisVideoError> {
        let channel = self
            .channels
            .values_mut()
            .find(|c| c.channel_arn == channel_arn)
            .ok_or_else(|| KinesisVideoError::ChannelNotFoundByArn(channel_arn.to_string()))?;

        if channel.version != current_version {
            return Err(KinesisVideoError::ChannelVersionMismatch {
                expected: current_version.to_string(),
                actual: channel.version.clone(),
            });
        }

        if let Some(ttl) = message_ttl_seconds {
            channel.message_ttl_seconds = Some(ttl);
        }
        channel.version = uuid::Uuid::new_v4().to_string();
        Ok(())
    }

    pub fn get_signaling_channel_endpoint(
        &self,
        channel_arn: &str,
        protocols: Option<&[String]>,
        _role: Option<&str>,
    ) -> Result<Vec<(String, String)>, KinesisVideoError> {
        let channel = self
            .channels
            .values()
            .find(|c| c.channel_arn == channel_arn)
            .ok_or_else(|| KinesisVideoError::ChannelNotFoundByArn(channel_arn.to_string()))?;

        let region = channel_arn.split(':').nth(3).unwrap_or("us-east-1");
        let default_protocols = vec!["WSS".to_string(), "HTTPS".to_string()];
        let protos = protocols.unwrap_or(&default_protocols);

        let endpoints = protos
            .iter()
            .map(|p| {
                let endpoint = format!(
                    "https://{}.kinesisvideo.{}.amazonaws.com",
                    channel.channel_name.to_lowercase(),
                    region
                );
                (p.clone(), endpoint)
            })
            .collect();

        Ok(endpoints)
    }

    // =========================================================================
    // Media storage configuration
    // =========================================================================

    pub fn describe_media_storage_configuration(
        &self,
        channel_name: Option<&str>,
        channel_arn: Option<&str>,
    ) -> Result<Option<&MediaStorageConfig>, KinesisVideoError> {
        let channel = self.describe_signaling_channel(channel_name, channel_arn)?;
        Ok(channel.media_storage_config.as_ref())
    }

    pub fn update_media_storage_configuration(
        &mut self,
        channel_arn: &str,
        status: &str,
        stream_arn: Option<&str>,
    ) -> Result<(), KinesisVideoError> {
        let channel = self
            .channels
            .values_mut()
            .find(|c| c.channel_arn == channel_arn)
            .ok_or_else(|| KinesisVideoError::ChannelNotFoundByArn(channel_arn.to_string()))?;

        channel.media_storage_config = Some(MediaStorageConfig {
            status: status.to_string(),
            stream_arn: stream_arn.map(String::from),
        });
        Ok(())
    }
}
