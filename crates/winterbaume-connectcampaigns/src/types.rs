use std::collections::HashMap;

/// An Amazon Connect campaign.
#[derive(Debug, Clone)]
pub struct Campaign {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub connect_instance_id: String,
    pub dialer_config: serde_json::Value,
    pub outbound_call_config: serde_json::Value,
    pub tags: HashMap<String, String>,
    pub state: CampaignState,
}

/// Campaign lifecycle states.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CampaignState {
    Initialized,
    Running,
    Paused,
    Stopped,
}

impl CampaignState {
    pub fn as_str(&self) -> &'static str {
        match self {
            CampaignState::Initialized => "Initialized",
            CampaignState::Running => "Running",
            CampaignState::Paused => "Paused",
            CampaignState::Stopped => "Stopped",
        }
    }
}

/// Instance onboarding job status.
#[derive(Debug, Clone)]
pub struct InstanceOnboardingJob {
    pub connect_instance_id: String,
    pub status: String,
}

/// Instance configuration stored after onboarding.
#[derive(Debug, Clone)]
pub struct InstanceConfig {
    pub connect_instance_id: String,
    pub encryption_enabled: bool,
    pub encryption_type: Option<String>,
    pub key_arn: Option<String>,
    pub service_linked_role_arn: String,
}
