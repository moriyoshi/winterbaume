use chrono::{DateTime, Utc};

/// A Shield Advanced subscription.
#[derive(Debug, Clone)]
pub struct Subscription {
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub time_commitment_in_seconds: i64,
    pub auto_renew: AutoRenew,
    pub subscription_arn: String,
}

/// Auto-renew setting for a subscription.
#[derive(Debug, Clone, PartialEq)]
pub enum AutoRenew {
    Enabled,
    Disabled,
}

impl AutoRenew {
    pub fn as_str(&self) -> &str {
        match self {
            AutoRenew::Enabled => "ENABLED",
            AutoRenew::Disabled => "DISABLED",
        }
    }
}

/// A resource protection.
#[derive(Debug, Clone)]
pub struct Protection {
    pub id: String,
    pub name: String,
    pub resource_arn: String,
    pub protection_arn: String,
    pub health_check_ids: Vec<String>,
    pub tags: Vec<Tag>,
}

/// A key-value tag.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}
