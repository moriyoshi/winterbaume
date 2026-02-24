use std::collections::HashMap;

use thiserror::Error;

use crate::types::{
    Channel, PlaybackKeyPair, PlaybackRestrictionPolicy, RecordingConfiguration,
    RecordingDestination, StreamKey,
};

#[derive(Debug, Default)]
pub struct IvsState {
    pub channels: HashMap<String, Channel>,
    pub stream_keys: HashMap<String, StreamKey>,
    pub recording_configurations: HashMap<String, RecordingConfiguration>,
    pub playback_key_pairs: HashMap<String, PlaybackKeyPair>,
    pub playback_restriction_policies: HashMap<String, PlaybackRestrictionPolicy>,
    /// Tags keyed by resource ARN (for resources not tracked above)
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum IvsError {
    #[error("Channel with name '{name}' already exists.")]
    ChannelConflict { name: String },

    #[error("Channel '{arn}' not found.")]
    ChannelNotFound { arn: String },

    #[error("StreamKey '{arn}' not found.")]
    StreamKeyNotFound { arn: String },

    #[error("RecordingConfiguration '{arn}' not found.")]
    RecordingConfigurationNotFound { arn: String },

    #[error("PlaybackKeyPair '{arn}' not found.")]
    PlaybackKeyPairNotFound { arn: String },

    #[error("PlaybackRestrictionPolicy '{arn}' not found.")]
    PlaybackRestrictionPolicyNotFound { arn: String },

    #[error("Resource '{arn}' not found.")]
    ResourceNotFound { arn: String },
}

impl IvsState {
    // -------------------------------------------------------------------------
    // Channels
    // -------------------------------------------------------------------------

    pub fn create_channel(
        &mut self,
        name: &str,
        latency_mode: &str,
        channel_type: &str,
        region: &str,
        account_id: &str,
    ) -> Result<&Channel, IvsError> {
        // Check for duplicate name
        for ch in self.channels.values() {
            if ch.name == name {
                return Err(IvsError::ChannelConflict {
                    name: name.to_string(),
                });
            }
        }

        let channel_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:ivs:{region}:{account_id}:channel/{channel_id}");

        let channel = Channel {
            arn: arn.clone(),
            name: name.to_string(),
            latency_mode: latency_mode.to_string(),
            channel_type: channel_type.to_string(),
            authorized: false,
            tags: HashMap::new(),
        };

        self.channels.insert(arn.clone(), channel);
        Ok(self.channels.get(&arn).unwrap())
    }

    pub fn get_channel(&self, arn: &str) -> Result<&Channel, IvsError> {
        self.channels
            .get(arn)
            .ok_or_else(|| IvsError::ChannelNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_channel(&mut self, arn: &str) -> Result<(), IvsError> {
        if self.channels.remove(arn).is_none() {
            return Err(IvsError::ChannelNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_channels(&self) -> Vec<&Channel> {
        self.channels.values().collect()
    }

    pub fn update_channel(
        &mut self,
        arn: &str,
        name: Option<&str>,
        latency_mode: Option<&str>,
        channel_type: Option<&str>,
        authorized: Option<bool>,
    ) -> Result<&Channel, IvsError> {
        let channel = self
            .channels
            .get_mut(arn)
            .ok_or_else(|| IvsError::ChannelNotFound {
                arn: arn.to_string(),
            })?;

        if let Some(n) = name {
            channel.name = n.to_string();
        }
        if let Some(lm) = latency_mode {
            channel.latency_mode = lm.to_string();
        }
        if let Some(ct) = channel_type {
            channel.channel_type = ct.to_string();
        }
        if let Some(a) = authorized {
            channel.authorized = a;
        }

        Ok(self.channels.get(arn).unwrap())
    }

    // -------------------------------------------------------------------------
    // Stream keys
    // -------------------------------------------------------------------------

    pub fn create_stream_key(
        &mut self,
        channel_arn: &str,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&StreamKey, IvsError> {
        // Channel must exist
        if !self.channels.contains_key(channel_arn) {
            return Err(IvsError::ChannelNotFound {
                arn: channel_arn.to_string(),
            });
        }

        let key_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:ivs:{region}:{account_id}:stream-key/{key_id}");
        let value = format!("sk-{}", uuid::Uuid::new_v4().to_string().replace('-', ""));

        let sk = StreamKey {
            arn: arn.clone(),
            channel_arn: channel_arn.to_string(),
            value,
            tags,
        };
        self.stream_keys.insert(arn.clone(), sk);
        Ok(self.stream_keys.get(&arn).unwrap())
    }

    pub fn get_stream_key(&self, arn: &str) -> Result<&StreamKey, IvsError> {
        self.stream_keys
            .get(arn)
            .ok_or_else(|| IvsError::StreamKeyNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_stream_key(&mut self, arn: &str) -> Result<(), IvsError> {
        if self.stream_keys.remove(arn).is_none() {
            return Err(IvsError::StreamKeyNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_stream_keys(&self, channel_arn: &str) -> Vec<&StreamKey> {
        self.stream_keys
            .values()
            .filter(|sk| sk.channel_arn == channel_arn)
            .collect()
    }

    pub fn batch_get_stream_keys(
        &self,
        arns: &[String],
    ) -> (Vec<&StreamKey>, Vec<(String, String)>) {
        let mut found = Vec::new();
        let mut errors = Vec::new();
        for arn in arns {
            match self.stream_keys.get(arn.as_str()) {
                Some(sk) => found.push(sk),
                None => errors.push((arn.clone(), "StreamKey not found".to_string())),
            }
        }
        (found, errors)
    }

    // -------------------------------------------------------------------------
    // Recording configurations
    // -------------------------------------------------------------------------

    pub fn create_recording_configuration(
        &mut self,
        name: Option<&str>,
        s3_bucket_name: Option<&str>,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&RecordingConfiguration, IvsError> {
        let rc_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:ivs:{region}:{account_id}:recording-configuration/{rc_id}");

        let rc = RecordingConfiguration {
            arn: arn.clone(),
            name: name.unwrap_or("").to_string(),
            destination_configuration: RecordingDestination {
                s3_bucket_name: s3_bucket_name.map(|s| s.to_string()),
            },
            state: "ACTIVE".to_string(),
            tags,
        };
        self.recording_configurations.insert(arn.clone(), rc);
        Ok(self.recording_configurations.get(&arn).unwrap())
    }

    pub fn get_recording_configuration(
        &self,
        arn: &str,
    ) -> Result<&RecordingConfiguration, IvsError> {
        self.recording_configurations.get(arn).ok_or_else(|| {
            IvsError::RecordingConfigurationNotFound {
                arn: arn.to_string(),
            }
        })
    }

    pub fn delete_recording_configuration(&mut self, arn: &str) -> Result<(), IvsError> {
        if self.recording_configurations.remove(arn).is_none() {
            return Err(IvsError::RecordingConfigurationNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_recording_configurations(&self) -> Vec<&RecordingConfiguration> {
        self.recording_configurations.values().collect()
    }

    // -------------------------------------------------------------------------
    // Playback key pairs
    // -------------------------------------------------------------------------

    pub fn import_playback_key_pair(
        &mut self,
        name: Option<&str>,
        _public_key_material: &str,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&PlaybackKeyPair, IvsError> {
        let key_id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:ivs:{region}:{account_id}:playback-key/{key_id}");

        let kp = PlaybackKeyPair {
            arn: arn.clone(),
            name: name.unwrap_or("").to_string(),
            fingerprint: format!("{}:{}", &key_id[..8], &key_id[9..17]),
            tags,
        };
        self.playback_key_pairs.insert(arn.clone(), kp);
        Ok(self.playback_key_pairs.get(&arn).unwrap())
    }

    pub fn get_playback_key_pair(&self, arn: &str) -> Result<&PlaybackKeyPair, IvsError> {
        self.playback_key_pairs
            .get(arn)
            .ok_or_else(|| IvsError::PlaybackKeyPairNotFound {
                arn: arn.to_string(),
            })
    }

    pub fn delete_playback_key_pair(&mut self, arn: &str) -> Result<(), IvsError> {
        if self.playback_key_pairs.remove(arn).is_none() {
            return Err(IvsError::PlaybackKeyPairNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_playback_key_pairs(&self) -> Vec<&PlaybackKeyPair> {
        self.playback_key_pairs.values().collect()
    }

    // -------------------------------------------------------------------------
    // Playback restriction policies
    // -------------------------------------------------------------------------

    pub fn create_playback_restriction_policy(
        &mut self,
        name: Option<&str>,
        allowed_countries: Vec<String>,
        allowed_origins: Vec<String>,
        enable_strict_origin_enforcement: bool,
        tags: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&PlaybackRestrictionPolicy, IvsError> {
        let policy_id = uuid::Uuid::new_v4().to_string();
        let arn =
            format!("arn:aws:ivs:{region}:{account_id}:playback-restriction-policy/{policy_id}");

        let policy = PlaybackRestrictionPolicy {
            arn: arn.clone(),
            name: name.unwrap_or("").to_string(),
            allowed_countries,
            allowed_origins,
            enable_strict_origin_enforcement,
            tags,
        };
        self.playback_restriction_policies
            .insert(arn.clone(), policy);
        Ok(self.playback_restriction_policies.get(&arn).unwrap())
    }

    pub fn get_playback_restriction_policy(
        &self,
        arn: &str,
    ) -> Result<&PlaybackRestrictionPolicy, IvsError> {
        self.playback_restriction_policies.get(arn).ok_or_else(|| {
            IvsError::PlaybackRestrictionPolicyNotFound {
                arn: arn.to_string(),
            }
        })
    }

    pub fn delete_playback_restriction_policy(&mut self, arn: &str) -> Result<(), IvsError> {
        if self.playback_restriction_policies.remove(arn).is_none() {
            return Err(IvsError::PlaybackRestrictionPolicyNotFound {
                arn: arn.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_playback_restriction_policies(&self) -> Vec<&PlaybackRestrictionPolicy> {
        self.playback_restriction_policies.values().collect()
    }

    pub fn update_playback_restriction_policy(
        &mut self,
        arn: &str,
        name: Option<&str>,
        allowed_countries: Option<Vec<String>>,
        allowed_origins: Option<Vec<String>>,
        enable_strict_origin_enforcement: Option<bool>,
    ) -> Result<&PlaybackRestrictionPolicy, IvsError> {
        let policy = self
            .playback_restriction_policies
            .get_mut(arn)
            .ok_or_else(|| IvsError::PlaybackRestrictionPolicyNotFound {
                arn: arn.to_string(),
            })?;

        if let Some(n) = name {
            policy.name = n.to_string();
        }
        if let Some(countries) = allowed_countries {
            policy.allowed_countries = countries;
        }
        if let Some(origins) = allowed_origins {
            policy.allowed_origins = origins;
        }
        if let Some(strict) = enable_strict_origin_enforcement {
            policy.enable_strict_origin_enforcement = strict;
        }

        Ok(self.playback_restriction_policies.get(arn).unwrap())
    }

    // -------------------------------------------------------------------------
    // Tags
    // -------------------------------------------------------------------------

    pub fn get_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        // Check channels
        if let Some(ch) = self.channels.get(resource_arn) {
            return ch.tags.clone();
        }
        // Check stream keys
        if let Some(sk) = self.stream_keys.get(resource_arn) {
            return sk.tags.clone();
        }
        // Check recording configs
        if let Some(rc) = self.recording_configurations.get(resource_arn) {
            return rc.tags.clone();
        }
        // Check playback key pairs
        if let Some(kp) = self.playback_key_pairs.get(resource_arn) {
            return kp.tags.clone();
        }
        // Check playback restriction policies
        if let Some(policy) = self.playback_restriction_policies.get(resource_arn) {
            return policy.tags.clone();
        }
        // Fall back to generic tags store
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), IvsError> {
        if let Some(ch) = self.channels.get_mut(resource_arn) {
            ch.tags.extend(tags);
            return Ok(());
        }
        if let Some(sk) = self.stream_keys.get_mut(resource_arn) {
            sk.tags.extend(tags);
            return Ok(());
        }
        if let Some(rc) = self.recording_configurations.get_mut(resource_arn) {
            rc.tags.extend(tags);
            return Ok(());
        }
        if let Some(kp) = self.playback_key_pairs.get_mut(resource_arn) {
            kp.tags.extend(tags);
            return Ok(());
        }
        if let Some(policy) = self.playback_restriction_policies.get_mut(resource_arn) {
            policy.tags.extend(tags);
            return Ok(());
        }
        Err(IvsError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), IvsError> {
        if let Some(ch) = self.channels.get_mut(resource_arn) {
            for k in tag_keys {
                ch.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(sk) = self.stream_keys.get_mut(resource_arn) {
            for k in tag_keys {
                sk.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(rc) = self.recording_configurations.get_mut(resource_arn) {
            for k in tag_keys {
                rc.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(kp) = self.playback_key_pairs.get_mut(resource_arn) {
            for k in tag_keys {
                kp.tags.remove(k);
            }
            return Ok(());
        }
        if let Some(policy) = self.playback_restriction_policies.get_mut(resource_arn) {
            for k in tag_keys {
                policy.tags.remove(k);
            }
            return Ok(());
        }
        Err(IvsError::ResourceNotFound {
            arn: resource_arn.to_string(),
        })
    }
}
