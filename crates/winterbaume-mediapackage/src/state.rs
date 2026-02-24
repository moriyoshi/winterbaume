use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaPackageState {
    pub channels: HashMap<String, Channel>,
    pub origin_endpoints: HashMap<String, OriginEndpoint>,
}

#[derive(Debug, Error)]
pub enum MediaPackageError {
    #[error("A channel with id '{id}' already exists.")]
    ChannelAlreadyExists { id: String },

    #[error("Channel with id '{id}' not found.")]
    ChannelNotFound { id: String },

    #[error("An origin endpoint with id '{id}' already exists.")]
    OriginEndpointAlreadyExists { id: String },

    #[error("Origin endpoint with id '{id}' not found.")]
    OriginEndpointNotFound { id: String },
}

impl MediaPackageState {
    pub fn create_channel(
        &mut self,
        id: &str,
        description: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Channel, MediaPackageError> {
        if self.channels.contains_key(id) {
            return Err(MediaPackageError::ChannelAlreadyExists { id: id.to_string() });
        }

        let arn = format!("arn:aws:mediapackage:{region}:{account_id}:channels/{id}");

        let channel = Channel {
            arn,
            id: id.to_string(),
            description: description.to_string(),
            tags,
            created_at: Utc::now(),
        };

        self.channels.insert(id.to_string(), channel);
        Ok(self.channels.get(id).unwrap())
    }

    pub fn describe_channel(&self, id: &str) -> Result<&Channel, MediaPackageError> {
        self.channels
            .get(id)
            .ok_or_else(|| MediaPackageError::ChannelNotFound { id: id.to_string() })
    }

    pub fn delete_channel(&mut self, id: &str) -> Result<(), MediaPackageError> {
        if self.channels.remove(id).is_none() {
            return Err(MediaPackageError::ChannelNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn list_channels(&self) -> Vec<&Channel> {
        self.channels.values().collect()
    }

    pub fn create_origin_endpoint(
        &mut self,
        id: &str,
        channel_id: &str,
        description: &str,
        manifest_name: &str,
        startover_window_seconds: i32,
        time_delay_seconds: i32,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&OriginEndpoint, MediaPackageError> {
        if !self.channels.contains_key(channel_id) {
            return Err(MediaPackageError::ChannelNotFound {
                id: channel_id.to_string(),
            });
        }
        if self.origin_endpoints.contains_key(id) {
            return Err(MediaPackageError::OriginEndpointAlreadyExists { id: id.to_string() });
        }

        let arn = format!("arn:aws:mediapackage:{region}:{account_id}:origin_endpoints/{id}");
        let url = format!("https://{id}.mediapackage.{region}.amazonaws.com/out/v1/{id}",);

        let ep = OriginEndpoint {
            arn,
            id: id.to_string(),
            channel_id: channel_id.to_string(),
            description: description.to_string(),
            manifest_name: manifest_name.to_string(),
            startover_window_seconds,
            time_delay_seconds,
            url,
            tags,
        };

        self.origin_endpoints.insert(id.to_string(), ep);
        Ok(self.origin_endpoints.get(id).unwrap())
    }

    pub fn describe_origin_endpoint(&self, id: &str) -> Result<&OriginEndpoint, MediaPackageError> {
        self.origin_endpoints
            .get(id)
            .ok_or_else(|| MediaPackageError::OriginEndpointNotFound { id: id.to_string() })
    }

    pub fn delete_origin_endpoint(&mut self, id: &str) -> Result<(), MediaPackageError> {
        if self.origin_endpoints.remove(id).is_none() {
            return Err(MediaPackageError::OriginEndpointNotFound { id: id.to_string() });
        }
        Ok(())
    }

    pub fn list_origin_endpoints(&self, channel_id: Option<&str>) -> Vec<&OriginEndpoint> {
        match channel_id {
            Some(cid) => self
                .origin_endpoints
                .values()
                .filter(|ep| ep.channel_id == cid)
                .collect(),
            None => self.origin_endpoints.values().collect(),
        }
    }

    pub fn update_origin_endpoint(
        &mut self,
        id: &str,
        description: Option<&str>,
        manifest_name: Option<&str>,
        startover_window_seconds: Option<i32>,
        time_delay_seconds: Option<i32>,
    ) -> Result<&OriginEndpoint, MediaPackageError> {
        let ep = self
            .origin_endpoints
            .get_mut(id)
            .ok_or_else(|| MediaPackageError::OriginEndpointNotFound { id: id.to_string() })?;
        if let Some(d) = description {
            ep.description = d.to_string();
        }
        if let Some(m) = manifest_name {
            ep.manifest_name = m.to_string();
        }
        if let Some(s) = startover_window_seconds {
            ep.startover_window_seconds = s;
        }
        if let Some(t) = time_delay_seconds {
            ep.time_delay_seconds = t;
        }
        Ok(ep)
    }
}
