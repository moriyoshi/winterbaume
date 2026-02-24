use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct MediaPackageV2State {
    pub channel_groups: HashMap<String, ChannelGroup>,
}

#[derive(Debug, Error)]
pub enum MediaPackageV2Error {
    #[error("ChannelGroup with name '{0}' already exists")]
    ChannelGroupAlreadyExists(String),

    #[error("Channel with name '{0}' already exists")]
    ChannelAlreadyExists(String),

    #[error("ChannelGroup with name '{0}' not found")]
    ChannelGroupNotFound(String),

    #[error("Channel with name '{0}' not found")]
    ChannelNotFound(String),
}

impl MediaPackageV2State {
    pub fn create_channel_group(
        &mut self,
        channel_group_name: &str,
        description: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&ChannelGroup, MediaPackageV2Error> {
        if self.channel_groups.contains_key(channel_group_name) {
            return Err(MediaPackageV2Error::ChannelGroupAlreadyExists(
                channel_group_name.to_string(),
            ));
        }

        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let arn = format!(
            "arn:aws:mediapackagev2:{}:{}:channelGroup/{}",
            region, account_id, channel_group_name
        );
        let egress_domain = format!("abcde.egress.vwxyz.mediapackagev2.{}.amazonaws.com", region);
        let e_tag = uuid::Uuid::new_v4().to_string();

        let channel_group = ChannelGroup {
            channel_group_name: channel_group_name.to_string(),
            arn,
            egress_domain,
            description: description.to_string(),
            tags,
            created_at: now.clone(),
            modified_at: now,
            e_tag,
            channels: HashMap::new(),
        };

        self.channel_groups
            .insert(channel_group_name.to_string(), channel_group);
        Ok(self.channel_groups.get(channel_group_name).unwrap())
    }

    pub fn get_channel_group(
        &self,
        channel_group_name: &str,
    ) -> Result<&ChannelGroup, MediaPackageV2Error> {
        self.channel_groups.get(channel_group_name).ok_or_else(|| {
            MediaPackageV2Error::ChannelGroupNotFound(channel_group_name.to_string())
        })
    }

    pub fn delete_channel_group(
        &mut self,
        channel_group_name: &str,
    ) -> Result<(), MediaPackageV2Error> {
        if self.channel_groups.remove(channel_group_name).is_none() {
            return Err(MediaPackageV2Error::ChannelGroupNotFound(
                channel_group_name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_channel_groups(&self) -> Vec<&ChannelGroup> {
        self.channel_groups.values().collect()
    }

    pub fn create_channel(
        &mut self,
        channel_group_name: &str,
        channel_name: &str,
        description: &str,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<&Channel, MediaPackageV2Error> {
        let cg = self
            .channel_groups
            .get_mut(channel_group_name)
            .ok_or_else(|| {
                MediaPackageV2Error::ChannelGroupNotFound(channel_group_name.to_string())
            })?;

        if cg.channels.contains_key(channel_name) {
            return Err(MediaPackageV2Error::ChannelAlreadyExists(
                channel_name.to_string(),
            ));
        }

        let now = Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
        let arn = format!(
            "arn:aws:mediapackagev2:{}:{}:channelGroup/{}/channel/{}",
            region, account_id, channel_group_name, channel_name
        );
        let e_tag = uuid::Uuid::new_v4().to_string();

        let channel = Channel {
            channel_name: channel_name.to_string(),
            channel_group_name: channel_group_name.to_string(),
            arn,
            description: description.to_string(),
            tags,
            created_at: now.clone(),
            modified_at: now,
            e_tag,
        };

        cg.channels.insert(channel_name.to_string(), channel);
        Ok(cg.channels.get(channel_name).unwrap())
    }

    pub fn get_channel(
        &self,
        channel_group_name: &str,
        channel_name: &str,
    ) -> Result<&Channel, MediaPackageV2Error> {
        let cg = self.channel_groups.get(channel_group_name).ok_or_else(|| {
            MediaPackageV2Error::ChannelGroupNotFound(channel_group_name.to_string())
        })?;

        cg.channels
            .get(channel_name)
            .ok_or_else(|| MediaPackageV2Error::ChannelNotFound(channel_name.to_string()))
    }

    pub fn delete_channel(
        &mut self,
        channel_group_name: &str,
        channel_name: &str,
    ) -> Result<(), MediaPackageV2Error> {
        let cg = self
            .channel_groups
            .get_mut(channel_group_name)
            .ok_or_else(|| {
                MediaPackageV2Error::ChannelGroupNotFound(channel_group_name.to_string())
            })?;

        if cg.channels.remove(channel_name).is_none() {
            return Err(MediaPackageV2Error::ChannelNotFound(
                channel_name.to_string(),
            ));
        }
        Ok(())
    }
}
