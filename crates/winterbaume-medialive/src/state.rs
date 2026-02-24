use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaLiveState {
    pub channels: Vec<MediaLiveChannel>,
    pub inputs: Vec<MediaLiveInput>,
}

#[derive(Debug, Error)]
pub enum MediaLiveError {
    #[error("Channel {channel_id} not found")]
    ChannelNotFound { channel_id: String },

    #[error("Input {input_id} not found")]
    InputNotFound { input_id: String },
}

impl MediaLiveState {
    pub fn create_channel(
        &mut self,
        name: &str,
        channel_class: Option<&str>,
        role_arn: &str,
        input_attachments: Value,
        destinations: Value,
        encoder_settings: Value,
        input_specification: Value,
        log_level: &str,
        tags: HashMap<String, String>,
    ) -> &MediaLiveChannel {
        let id = format!("{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:medialive:channel:{id}");
        let channel_class_str = channel_class.unwrap_or("STANDARD");
        let pipelines_running_count = if channel_class_str == "SINGLE_PIPELINE" {
            1
        } else {
            2
        };

        let channel = MediaLiveChannel {
            id: id.clone(),
            arn,
            name: name.to_string(),
            state: "CREATING".to_string(),
            channel_class: channel_class_str.to_string(),
            pipelines_running_count,
            role_arn: role_arn.to_string(),
            input_attachments,
            destinations,
            encoder_settings,
            input_specification,
            log_level: log_level.to_string(),
            tags,
        };

        self.channels.push(channel);
        self.channels.last().unwrap()
    }

    pub fn describe_channel(&self, channel_id: &str) -> Result<&MediaLiveChannel, MediaLiveError> {
        self.channels
            .iter()
            .find(|c| c.id == channel_id && c.state != "DELETING")
            .ok_or_else(|| MediaLiveError::ChannelNotFound {
                channel_id: channel_id.to_string(),
            })
    }

    pub fn delete_channel(
        &mut self,
        channel_id: &str,
    ) -> Result<&MediaLiveChannel, MediaLiveError> {
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or_else(|| MediaLiveError::ChannelNotFound {
                channel_id: channel_id.to_string(),
            })?;

        channel.state = "DELETING".to_string();

        Ok(self.channels.iter().find(|c| c.id == channel_id).unwrap())
    }

    pub fn list_channels(
        &self,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&MediaLiveChannel>, Option<String>) {
        let non_deleted: Vec<&MediaLiveChannel> = self
            .channels
            .iter()
            .filter(|c| c.state != "DELETING" && c.state != "DELETED")
            .collect();

        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let limit = max_results.unwrap_or(non_deleted.len());
        let end = std::cmp::min(start + limit, non_deleted.len());
        let items: Vec<&MediaLiveChannel> = non_deleted[start..end].to_vec();

        let next = if end < non_deleted.len() {
            Some(end.to_string())
        } else {
            None
        };

        (items, next)
    }

    pub fn start_channel(&mut self, channel_id: &str) -> Result<&MediaLiveChannel, MediaLiveError> {
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or_else(|| MediaLiveError::ChannelNotFound {
                channel_id: channel_id.to_string(),
            })?;

        channel.state = "RUNNING".to_string();

        Ok(self.channels.iter().find(|c| c.id == channel_id).unwrap())
    }

    pub fn stop_channel(&mut self, channel_id: &str) -> Result<&MediaLiveChannel, MediaLiveError> {
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or_else(|| MediaLiveError::ChannelNotFound {
                channel_id: channel_id.to_string(),
            })?;

        channel.state = "IDLE".to_string();

        Ok(self.channels.iter().find(|c| c.id == channel_id).unwrap())
    }

    pub fn update_channel(
        &mut self,
        channel_id: &str,
        name: Option<&str>,
        destinations: Option<Value>,
        encoder_settings: Option<Value>,
        input_attachments: Option<Value>,
        input_specification: Option<Value>,
        log_level: Option<&str>,
        role_arn: Option<&str>,
    ) -> Result<&MediaLiveChannel, MediaLiveError> {
        let channel = self
            .channels
            .iter_mut()
            .find(|c| c.id == channel_id)
            .ok_or_else(|| MediaLiveError::ChannelNotFound {
                channel_id: channel_id.to_string(),
            })?;

        if let Some(n) = name {
            channel.name = n.to_string();
        }
        if let Some(d) = destinations {
            channel.destinations = d;
        }
        if let Some(e) = encoder_settings {
            channel.encoder_settings = e;
        }
        if let Some(i) = input_attachments {
            channel.input_attachments = i;
        }
        if let Some(i) = input_specification {
            channel.input_specification = i;
        }
        if let Some(l) = log_level {
            channel.log_level = l.to_string();
        }
        if let Some(r) = role_arn {
            channel.role_arn = r.to_string();
        }

        // State transitions through UPDATING then back to IDLE on describe
        channel.state = "IDLE".to_string();

        Ok(self.channels.iter().find(|c| c.id == channel_id).unwrap())
    }

    pub fn create_input(
        &mut self,
        name: &str,
        input_type: &str,
        role_arn: &str,
        destinations: Value,
        input_devices: Value,
        media_connect_flows: Value,
        sources: Value,
        tags: HashMap<String, String>,
    ) -> &MediaLiveInput {
        let id = format!("{}", uuid::Uuid::new_v4().simple());
        let arn = format!("arn:aws:medialive:input:{id}");

        let input = MediaLiveInput {
            id: id.clone(),
            arn,
            name: name.to_string(),
            state: "CREATING".to_string(),
            input_class: "STANDARD".to_string(),
            input_source_type: "STATIC".to_string(),
            input_type: input_type.to_string(),
            role_arn: role_arn.to_string(),
            attached_channels: Vec::new(),
            destinations,
            input_devices,
            media_connect_flows,
            security_groups: Vec::new(),
            sources,
            tags,
        };

        self.inputs.push(input);
        self.inputs.last().unwrap()
    }

    pub fn describe_input(&self, input_id: &str) -> Result<&MediaLiveInput, MediaLiveError> {
        self.inputs
            .iter()
            .find(|i| i.id == input_id)
            .ok_or_else(|| MediaLiveError::InputNotFound {
                input_id: input_id.to_string(),
            })
    }

    pub fn delete_input(&mut self, input_id: &str) -> Result<(), MediaLiveError> {
        let input = self
            .inputs
            .iter_mut()
            .find(|i| i.id == input_id)
            .ok_or_else(|| MediaLiveError::InputNotFound {
                input_id: input_id.to_string(),
            })?;

        input.state = "DELETED".to_string();
        Ok(())
    }

    pub fn list_inputs(
        &self,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&MediaLiveInput>, Option<String>) {
        let non_deleted: Vec<&MediaLiveInput> = self
            .inputs
            .iter()
            .filter(|i| i.state != "DELETED")
            .collect();

        let start = next_token
            .and_then(|t| t.parse::<usize>().ok())
            .unwrap_or(0);

        let limit = max_results.unwrap_or(non_deleted.len());
        let end = std::cmp::min(start + limit, non_deleted.len());
        let items: Vec<&MediaLiveInput> = non_deleted[start..end].to_vec();

        let next = if end < non_deleted.len() {
            Some(end.to_string())
        } else {
            None
        };

        (items, next)
    }

    /// Look up tags for a resource referenced by either its ARN or bare id.
    ///
    /// MediaLive's REST tag URL is `/prod/tags/{resource-arn}`; terraform sends
    /// the full ARN (e.g. `arn:aws:medialive:input:abcd1234`). Both channels
    /// and inputs share this URL space so we search both collections, falling
    /// back to a trailing-segment match (everything after the last `:` or `/`)
    /// to be tolerant of slightly different ARN shapes.
    pub fn list_tags_for_resource(&self, resource: &str) -> Option<HashMap<String, String>> {
        let id = resource.rsplit([':', '/']).next().unwrap_or(resource);
        if let Some(c) = self
            .channels
            .iter()
            .find(|c| c.arn == resource || c.id == resource || c.id == id)
        {
            return Some(c.tags.clone());
        }
        if let Some(i) = self
            .inputs
            .iter()
            .find(|i| i.arn == resource || i.id == resource || i.id == id)
        {
            return Some(i.tags.clone());
        }
        None
    }

    /// Merge new tag entries into a resource (channel or input).
    pub fn create_tags(&mut self, resource: &str, tags: HashMap<String, String>) -> bool {
        let id = resource.rsplit([':', '/']).next().unwrap_or(resource);
        if let Some(c) = self
            .channels
            .iter_mut()
            .find(|c| c.arn == resource || c.id == resource || c.id == id)
        {
            for (k, v) in tags {
                c.tags.insert(k, v);
            }
            return true;
        }
        if let Some(i) = self
            .inputs
            .iter_mut()
            .find(|i| i.arn == resource || i.id == resource || i.id == id)
        {
            for (k, v) in tags {
                i.tags.insert(k, v);
            }
            return true;
        }
        false
    }

    /// Remove the named tag keys from a resource.
    pub fn delete_tags(&mut self, resource: &str, keys: &[String]) -> bool {
        let id = resource.rsplit([':', '/']).next().unwrap_or(resource);
        if let Some(c) = self
            .channels
            .iter_mut()
            .find(|c| c.arn == resource || c.id == resource || c.id == id)
        {
            for k in keys {
                c.tags.remove(k);
            }
            return true;
        }
        if let Some(i) = self
            .inputs
            .iter_mut()
            .find(|i| i.arn == resource || i.id == resource || i.id == id)
        {
            for k in keys {
                i.tags.remove(k);
            }
            return true;
        }
        false
    }

    pub fn update_input(
        &mut self,
        input_id: &str,
        name: Option<&str>,
        destinations: Option<Value>,
        input_devices: Option<Value>,
        media_connect_flows: Option<Value>,
        role_arn: Option<&str>,
        sources: Option<Value>,
    ) -> Result<&MediaLiveInput, MediaLiveError> {
        let input = self
            .inputs
            .iter_mut()
            .find(|i| i.id == input_id)
            .ok_or_else(|| MediaLiveError::InputNotFound {
                input_id: input_id.to_string(),
            })?;

        if let Some(n) = name {
            input.name = n.to_string();
        }
        if let Some(d) = destinations {
            input.destinations = d;
        }
        if let Some(id) = input_devices {
            input.input_devices = id;
        }
        if let Some(m) = media_connect_flows {
            input.media_connect_flows = m;
        }
        if let Some(r) = role_arn {
            input.role_arn = r.to_string();
        }
        if let Some(s) = sources {
            input.sources = s;
        }

        Ok(self.inputs.iter().find(|i| i.id == input_id).unwrap())
    }
}
