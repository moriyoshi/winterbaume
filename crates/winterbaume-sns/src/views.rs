//! Serde-compatible view types for SNS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StatefulService};

use crate::handlers::SnsService;
use crate::state::SnsState;
use crate::types::{
    Permission, PlatformApplicationState, PlatformEndpointState, SmsSandboxPhoneNumber,
    Subscription, Topic,
};

/// Serializable view of the entire SNS state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnsStateView {
    /// Topics keyed by ARN.
    #[serde(default)]
    pub topics: HashMap<String, TopicView>,
    /// Subscriptions keyed by subscription ARN.
    #[serde(default)]
    pub subscriptions: HashMap<String, SubscriptionView>,
    /// Global SMS attributes (e.g., DefaultSenderID, DefaultSMSType, etc.).
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub sms_attributes: HashMap<String, String>,
    /// Platform applications keyed by ARN.
    #[serde(default)]
    pub platform_applications: HashMap<String, PlatformApplicationView>,
    /// Platform endpoints keyed by ARN.
    #[serde(default)]
    pub platform_endpoints: HashMap<String, PlatformEndpointView>,
    /// SMS sandbox phone numbers.
    #[serde(default)]
    pub sms_sandbox_phone_numbers: Vec<SmsSandboxPhoneNumberView>,
    /// Whether the account is in the SMS sandbox.
    #[serde(default = "default_true")]
    pub sms_sandbox_enabled: bool,
    /// Opted-out phone numbers.
    #[serde(default)]
    pub opted_out_phone_numbers: Vec<String>,
}

fn default_true() -> bool {
    true
}

impl Default for SnsStateView {
    fn default() -> Self {
        Self {
            topics: Default::default(),
            subscriptions: Default::default(),
            sms_attributes: Default::default(),
            platform_applications: Default::default(),
            platform_endpoints: Default::default(),
            sms_sandbox_phone_numbers: Default::default(),
            sms_sandbox_enabled: true,
            opted_out_phone_numbers: Default::default(),
        }
    }
}

/// Serializable view of an SNS topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicView {
    /// Topic ARN.
    pub arn: String,
    /// Topic name.
    pub name: String,
    /// Topic attributes (DisplayName, Policy, etc.).
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    /// Resource tags.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// IAM permissions keyed by label.
    #[serde(default)]
    pub permissions: HashMap<String, PermissionView>,
    /// Data protection policy.
    #[serde(default)]
    pub data_protection_policy: Option<String>,
}

/// Serializable view of an IAM permission statement on a topic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionView {
    /// Statement label.
    pub label: String,
    /// AWS account IDs granted access.
    pub aws_account_ids: Vec<String>,
    /// SNS action names granted.
    pub action_names: Vec<String>,
}

/// Serializable view of an SNS subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionView {
    /// Subscription ARN.
    pub arn: String,
    /// ARN of the topic this subscription belongs to.
    pub topic_arn: String,
    /// Delivery protocol (e.g., "email", "sqs", "lambda", "https").
    pub protocol: String,
    /// Delivery endpoint.
    pub endpoint: String,
    /// Whether the subscription has been confirmed.
    pub confirmed: bool,
    /// AWS account ID that owns this subscription.
    pub owner: String,
    /// Subscription attributes.
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Serializable view of a platform application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformApplicationView {
    pub arn: String,
    pub name: String,
    pub platform: String,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Serializable view of a platform endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlatformEndpointView {
    pub arn: String,
    pub platform_application_arn: String,
    pub token: String,
    pub enabled: bool,
    pub custom_user_data: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
}

/// Serializable view of an SMS sandbox phone number.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsSandboxPhoneNumberView {
    pub phone_number: String,
    pub status: String,
}

// --- From internal types to view types ---

impl From<&SnsState> for SnsStateView {
    fn from(state: &SnsState) -> Self {
        SnsStateView {
            topics: state
                .topics
                .iter()
                .map(|(k, v)| (k.clone(), TopicView::from(v)))
                .collect(),
            subscriptions: state
                .subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), SubscriptionView::from(v)))
                .collect(),
            sms_attributes: state.sms_attributes.clone(),
            platform_applications: state
                .platform_applications
                .iter()
                .map(|(k, v)| (k.clone(), PlatformApplicationView::from(v)))
                .collect(),
            platform_endpoints: state
                .platform_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), PlatformEndpointView::from(v)))
                .collect(),
            sms_sandbox_phone_numbers: state
                .sms_sandbox_phone_numbers
                .iter()
                .map(SmsSandboxPhoneNumberView::from)
                .collect(),
            sms_sandbox_enabled: state.sms_sandbox_enabled,
            opted_out_phone_numbers: state.opted_out_phone_numbers.clone(),
        }
    }
}

impl From<&Topic> for TopicView {
    fn from(t: &Topic) -> Self {
        TopicView {
            arn: t.arn.clone(),
            name: t.name.clone(),
            attributes: t.attributes.clone(),
            tags: t.tags.clone(),
            permissions: t
                .permissions
                .iter()
                .map(|(k, v)| (k.clone(), PermissionView::from(v)))
                .collect(),
            data_protection_policy: t.data_protection_policy.clone(),
        }
    }
}

impl From<&Permission> for PermissionView {
    fn from(p: &Permission) -> Self {
        PermissionView {
            label: p.label.clone(),
            aws_account_ids: p.aws_account_ids.clone(),
            action_names: p.action_names.clone(),
        }
    }
}

impl From<&Subscription> for SubscriptionView {
    fn from(s: &Subscription) -> Self {
        SubscriptionView {
            arn: s.arn.clone(),
            topic_arn: s.topic_arn.clone(),
            protocol: s.protocol.clone(),
            endpoint: s.endpoint.clone(),
            confirmed: s.confirmed,
            owner: s.owner.clone(),
            attributes: s.attributes.clone(),
        }
    }
}

impl From<&PlatformApplicationState> for PlatformApplicationView {
    fn from(a: &PlatformApplicationState) -> Self {
        PlatformApplicationView {
            arn: a.arn.clone(),
            name: a.name.clone(),
            platform: a.platform.clone(),
            attributes: a.attributes.clone(),
        }
    }
}

impl From<&PlatformEndpointState> for PlatformEndpointView {
    fn from(e: &PlatformEndpointState) -> Self {
        PlatformEndpointView {
            arn: e.arn.clone(),
            platform_application_arn: e.platform_application_arn.clone(),
            token: e.token.clone(),
            enabled: e.enabled,
            custom_user_data: e.custom_user_data.clone(),
            attributes: e.attributes.clone(),
        }
    }
}

impl From<&SmsSandboxPhoneNumber> for SmsSandboxPhoneNumberView {
    fn from(p: &SmsSandboxPhoneNumber) -> Self {
        SmsSandboxPhoneNumberView {
            phone_number: p.phone_number.clone(),
            status: p.status.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<SnsStateView> for SnsState {
    fn from(view: SnsStateView) -> Self {
        SnsState {
            topics: view
                .topics
                .into_iter()
                .map(|(k, v)| (k, Topic::from(v)))
                .collect(),
            subscriptions: view
                .subscriptions
                .into_iter()
                .map(|(k, v)| (k, Subscription::from(v)))
                .collect(),
            sms_attributes: view.sms_attributes,
            platform_applications: view
                .platform_applications
                .into_iter()
                .map(|(k, v)| (k, PlatformApplicationState::from(v)))
                .collect(),
            platform_endpoints: view
                .platform_endpoints
                .into_iter()
                .map(|(k, v)| (k, PlatformEndpointState::from(v)))
                .collect(),
            sms_sandbox_phone_numbers: view
                .sms_sandbox_phone_numbers
                .into_iter()
                .map(SmsSandboxPhoneNumber::from)
                .collect(),
            sms_sandbox_enabled: view.sms_sandbox_enabled,
            opted_out_phone_numbers: view.opted_out_phone_numbers,
        }
    }
}

impl From<TopicView> for Topic {
    fn from(v: TopicView) -> Self {
        Topic {
            arn: v.arn,
            name: v.name,
            attributes: v.attributes,
            tags: v.tags,
            permissions: v
                .permissions
                .into_iter()
                .map(|(k, pv)| (k, Permission::from(pv)))
                .collect(),
            data_protection_policy: v.data_protection_policy,
        }
    }
}

impl From<PermissionView> for Permission {
    fn from(v: PermissionView) -> Self {
        Permission {
            label: v.label,
            aws_account_ids: v.aws_account_ids,
            action_names: v.action_names,
        }
    }
}

impl From<SubscriptionView> for Subscription {
    fn from(v: SubscriptionView) -> Self {
        Subscription {
            arn: v.arn,
            topic_arn: v.topic_arn,
            protocol: v.protocol,
            endpoint: v.endpoint,
            confirmed: v.confirmed,
            owner: v.owner,
            attributes: v.attributes,
        }
    }
}

impl From<PlatformApplicationView> for PlatformApplicationState {
    fn from(v: PlatformApplicationView) -> Self {
        PlatformApplicationState {
            arn: v.arn,
            name: v.name,
            platform: v.platform,
            attributes: v.attributes,
        }
    }
}

impl From<PlatformEndpointView> for PlatformEndpointState {
    fn from(v: PlatformEndpointView) -> Self {
        PlatformEndpointState {
            arn: v.arn,
            platform_application_arn: v.platform_application_arn,
            token: v.token,
            enabled: v.enabled,
            custom_user_data: v.custom_user_data,
            attributes: v.attributes,
        }
    }
}

impl From<SmsSandboxPhoneNumberView> for SmsSandboxPhoneNumber {
    fn from(v: SmsSandboxPhoneNumberView) -> Self {
        SmsSandboxPhoneNumber {
            phone_number: v.phone_number,
            status: v.status,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SnsService {
    type StateView = SnsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        self.backend
            .snapshot(account_id.to_owned(), region.to_owned())
            .await
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .restore(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), winterbaume_core::StateViewError> {
        self.backend
            .merge(account_id.to_owned(), region.to_owned(), view)
            .await?;
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
