use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Topic {
    pub arn: String,
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub tags: HashMap<String, String>,
    /// Policy permissions: label -> statement info
    pub permissions: HashMap<String, Permission>,
    /// Data protection policy JSON string.
    pub data_protection_policy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Permission {
    pub label: String,
    pub aws_account_ids: Vec<String>,
    pub action_names: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Subscription {
    pub arn: String,
    pub topic_arn: String,
    pub protocol: String,
    pub endpoint: String,
    pub confirmed: bool,
    pub owner: String,
    /// Subscription-level attributes (FilterPolicy, RawMessageDelivery, etc.)
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct PublishedMessage {
    pub message_id: String,
    pub message: String,
    pub subject: Option<String>,
}

/// A platform application for push notifications (APNs, GCM, etc.)
#[derive(Debug, Clone)]
pub struct PlatformApplicationState {
    pub arn: String,
    pub name: String,
    pub platform: String,
    pub attributes: HashMap<String, String>,
}

/// A platform endpoint (device registration).
#[derive(Debug, Clone)]
pub struct PlatformEndpointState {
    pub arn: String,
    pub platform_application_arn: String,
    pub token: String,
    pub enabled: bool,
    pub custom_user_data: Option<String>,
    pub attributes: HashMap<String, String>,
}

/// An SMS sandbox phone number entry.
#[derive(Debug, Clone)]
pub struct SmsSandboxPhoneNumber {
    pub phone_number: String,
    pub status: String, // "Pending" or "Verified"
}
