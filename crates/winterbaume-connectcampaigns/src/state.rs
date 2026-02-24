use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ConnectCampaignsState {
    pub campaigns: HashMap<String, Campaign>,
    pub instance_configs: HashMap<String, InstanceConfig>,
    pub instance_onboarding_jobs: HashMap<String, InstanceOnboardingJob>,
    /// Tags keyed by ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum ConnectCampaignsError {
    #[error("Campaign name must not be empty")]
    CampaignNameEmpty,
    #[error("Campaign {0} not found")]
    CampaignNotFound(String),
    #[error("Instance config for {0} not found")]
    InstanceConfigNotFound(String),
    #[error("Campaign is already running")]
    CampaignAlreadyRunning,
    #[error("Campaign is already stopped")]
    CampaignAlreadyStopped,
    #[error("Campaign must be running to pause")]
    CampaignMustBeRunningToPause,
    #[error("Campaign must be paused to resume")]
    CampaignMustBePausedToResume,
}

impl ConnectCampaignsState {
    pub fn create_campaign(
        &mut self,
        name: &str,
        connect_instance_id: &str,
        dialer_config: serde_json::Value,
        outbound_call_config: serde_json::Value,
        tags: HashMap<String, String>,
        account_id: &str,
        region: &str,
    ) -> Result<Campaign, ConnectCampaignsError> {
        if name.is_empty() {
            return Err(ConnectCampaignsError::CampaignNameEmpty);
        }

        let id = uuid::Uuid::new_v4().to_string();
        let arn = format!("arn:aws:connect-campaigns:{region}:{account_id}:campaign/{id}");

        let campaign = Campaign {
            id: id.clone(),
            arn: arn.clone(),
            name: name.to_string(),
            connect_instance_id: connect_instance_id.to_string(),
            dialer_config,
            outbound_call_config,
            tags: tags.clone(),
            state: CampaignState::Initialized,
        };

        if !tags.is_empty() {
            self.tags.insert(arn, tags);
        }

        self.campaigns.insert(id.clone(), campaign.clone());
        Ok(campaign)
    }

    pub fn get_campaign(&self, id: &str) -> Result<&Campaign, ConnectCampaignsError> {
        self.campaigns
            .get(id)
            .ok_or_else(|| ConnectCampaignsError::CampaignNotFound(id.to_string()))
    }

    pub fn delete_campaign(&mut self, id: &str) -> Result<(), ConnectCampaignsError> {
        if let Some(campaign) = self.campaigns.remove(id) {
            self.tags.remove(&campaign.arn);
            Ok(())
        } else {
            Err(ConnectCampaignsError::CampaignNotFound(id.to_string()))
        }
    }

    pub fn list_campaigns(&self) -> Vec<&Campaign> {
        self.campaigns.values().collect()
    }

    pub fn get_campaign_state(&self, id: &str) -> Result<CampaignState, ConnectCampaignsError> {
        let campaign = self.get_campaign(id)?;
        Ok(campaign.state)
    }

    pub fn start_campaign(&mut self, id: &str) -> Result<(), ConnectCampaignsError> {
        let campaign = self
            .campaigns
            .get_mut(id)
            .ok_or_else(|| ConnectCampaignsError::CampaignNotFound(id.to_string()))?;
        match campaign.state {
            CampaignState::Running => {
                return Err(ConnectCampaignsError::CampaignAlreadyRunning);
            }
            _ => {
                campaign.state = CampaignState::Running;
            }
        }
        Ok(())
    }

    pub fn stop_campaign(&mut self, id: &str) -> Result<(), ConnectCampaignsError> {
        let campaign = self
            .campaigns
            .get_mut(id)
            .ok_or_else(|| ConnectCampaignsError::CampaignNotFound(id.to_string()))?;
        match campaign.state {
            CampaignState::Stopped => {
                return Err(ConnectCampaignsError::CampaignAlreadyStopped);
            }
            _ => {
                campaign.state = CampaignState::Stopped;
            }
        }
        Ok(())
    }

    pub fn pause_campaign(&mut self, id: &str) -> Result<(), ConnectCampaignsError> {
        let campaign = self
            .campaigns
            .get_mut(id)
            .ok_or_else(|| ConnectCampaignsError::CampaignNotFound(id.to_string()))?;
        if campaign.state != CampaignState::Running {
            return Err(ConnectCampaignsError::CampaignMustBeRunningToPause);
        }
        campaign.state = CampaignState::Paused;
        Ok(())
    }

    pub fn resume_campaign(&mut self, id: &str) -> Result<(), ConnectCampaignsError> {
        let campaign = self
            .campaigns
            .get_mut(id)
            .ok_or_else(|| ConnectCampaignsError::CampaignNotFound(id.to_string()))?;
        if campaign.state != CampaignState::Paused {
            return Err(ConnectCampaignsError::CampaignMustBePausedToResume);
        }
        campaign.state = CampaignState::Running;
        Ok(())
    }

    pub fn get_connect_instance_config(
        &self,
        connect_instance_id: &str,
    ) -> Result<&InstanceConfig, ConnectCampaignsError> {
        self.instance_configs
            .get(connect_instance_id)
            .ok_or_else(|| {
                ConnectCampaignsError::InstanceConfigNotFound(connect_instance_id.to_string())
            })
    }

    pub fn start_instance_onboarding_job(
        &mut self,
        connect_instance_id: &str,
        encryption_config: &serde_json::Value,
        account_id: &str,
        region: &str,
    ) -> Result<InstanceOnboardingJob, ConnectCampaignsError> {
        let enabled = encryption_config
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let encryption_type = encryption_config
            .get("encryptionType")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let key_arn = encryption_config
            .get("keyArn")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let service_linked_role_arn = format!(
            "arn:aws:iam::{account_id}:role/aws-service-role/connect-campaigns.amazonaws.com/AWSServiceRoleForConnectCampaigns"
        );

        let config = InstanceConfig {
            connect_instance_id: connect_instance_id.to_string(),
            encryption_enabled: enabled,
            encryption_type,
            key_arn,
            service_linked_role_arn,
        };
        self.instance_configs
            .insert(connect_instance_id.to_string(), config);

        let job = InstanceOnboardingJob {
            connect_instance_id: connect_instance_id.to_string(),
            status: "SUCCEEDED".to_string(),
        };
        self.instance_onboarding_jobs
            .insert(connect_instance_id.to_string(), job.clone());
        Ok(job)
    }

    pub fn list_tags_for_resource(
        &self,
        arn: &str,
    ) -> Result<HashMap<String, String>, ConnectCampaignsError> {
        Ok(self.tags.get(arn).cloned().unwrap_or_default())
    }

    pub fn tag_resource(
        &mut self,
        arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), ConnectCampaignsError> {
        let entry = self.tags.entry(arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k.clone(), v.clone());
        }
        // Also update campaign tags if this ARN belongs to a campaign
        for campaign in self.campaigns.values_mut() {
            if campaign.arn == arn {
                for (k, v) in entry.iter() {
                    campaign.tags.insert(k.clone(), v.clone());
                }
                break;
            }
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        arn: &str,
        tag_keys: &[String],
    ) -> Result<(), ConnectCampaignsError> {
        if let Some(entry) = self.tags.get_mut(arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
        // Also update campaign tags if this ARN belongs to a campaign
        for campaign in self.campaigns.values_mut() {
            if campaign.arn == arn {
                for key in tag_keys {
                    campaign.tags.remove(key);
                }
                break;
            }
        }
        Ok(())
    }
}
