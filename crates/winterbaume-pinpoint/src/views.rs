//! Serde-compatible view types for Pinpoint state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::PinpointService;
use crate::state::PinpointState;

/// Serializable view of a campaign hook.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CampaignHookView {
    pub lambda_function_name: Option<String>,
    pub mode: Option<String>,
    pub web_url: Option<String>,
}

/// Serializable view of campaign limits.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LimitsView {
    pub daily: Option<i64>,
    pub maximum_duration: Option<i64>,
    pub messages_per_second: Option<i64>,
    pub total: Option<i64>,
    pub session: Option<i64>,
}

/// Serializable view of application settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSettingsView {
    pub campaign_hook: Option<CampaignHookView>,
    pub limits: Option<LimitsView>,
    pub last_modified_date: String,
}

/// Serializable view of an event stream.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EventStreamView {
    pub application_id: String,
    pub destination_stream_arn: String,
    pub role_arn: String,
    pub last_modified_date: String,
}

/// Serializable view of a Pinpoint application.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PinpointAppView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub creation_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub settings: Option<ApplicationSettingsView>,
    pub event_stream: Option<EventStreamView>,
    /// Quiet time as `{"start": "HH:MM", "end": "HH:MM"}` — stored for Terraform round-trip.
    #[serde(default)]
    pub quiet_time: Option<serde_json::Value>,
}

/// Serializable view of a Pinpoint email channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailChannelView {
    pub application_id: String,
    pub enabled: bool,
    pub from_address: String,
    pub identity: String,
    pub role_arn: Option<String>,
    pub configuration_set: Option<String>,
    pub messages_per_second: Option<i32>,
}

/// Serializable view of the entire Pinpoint state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PinpointStateView {
    #[serde(default)]
    pub apps: HashMap<String, PinpointAppView>,
    /// Email channels keyed by application ID.
    #[serde(default)]
    pub email_channels: HashMap<String, EmailChannelView>,
}

// --- From internal types to view types ---

impl From<&crate::types::CampaignHook> for CampaignHookView {
    fn from(h: &crate::types::CampaignHook) -> Self {
        CampaignHookView {
            lambda_function_name: h.lambda_function_name.clone(),
            mode: h.mode.clone(),
            web_url: h.web_url.clone(),
        }
    }
}

impl From<&crate::types::Limits> for LimitsView {
    fn from(l: &crate::types::Limits) -> Self {
        LimitsView {
            daily: l.daily,
            maximum_duration: l.maximum_duration,
            messages_per_second: l.messages_per_second,
            total: l.total,
            session: l.session,
        }
    }
}

impl From<&crate::types::ApplicationSettings> for ApplicationSettingsView {
    fn from(s: &crate::types::ApplicationSettings) -> Self {
        ApplicationSettingsView {
            campaign_hook: s.campaign_hook.as_ref().map(CampaignHookView::from),
            limits: s.limits.as_ref().map(LimitsView::from),
            last_modified_date: s.last_modified_date.to_rfc3339(),
        }
    }
}

impl From<&crate::types::EventStream> for EventStreamView {
    fn from(es: &crate::types::EventStream) -> Self {
        EventStreamView {
            application_id: es.application_id.clone(),
            destination_stream_arn: es.destination_stream_arn.clone(),
            role_arn: es.role_arn.clone(),
            last_modified_date: es.last_modified_date.to_rfc3339(),
        }
    }
}

impl From<&crate::types::PinpointApp> for PinpointAppView {
    fn from(app: &crate::types::PinpointApp) -> Self {
        PinpointAppView {
            id: app.id.clone(),
            arn: app.arn.clone(),
            name: app.name.clone(),
            creation_date: app.creation_date.to_rfc3339(),
            tags: app.tags.clone(),
            settings: app.settings.as_ref().map(ApplicationSettingsView::from),
            event_stream: app.event_stream.as_ref().map(EventStreamView::from),
            quiet_time: app.quiet_time.clone(),
        }
    }
}

impl From<&crate::types::EmailChannel> for EmailChannelView {
    fn from(ec: &crate::types::EmailChannel) -> Self {
        EmailChannelView {
            application_id: ec.application_id.clone(),
            enabled: ec.enabled,
            from_address: ec.from_address.clone(),
            identity: ec.identity.clone(),
            role_arn: ec.role_arn.clone(),
            configuration_set: ec.configuration_set.clone(),
            messages_per_second: ec.messages_per_second,
        }
    }
}

impl From<EmailChannelView> for crate::types::EmailChannel {
    fn from(v: EmailChannelView) -> Self {
        crate::types::EmailChannel {
            application_id: v.application_id,
            enabled: v.enabled,
            from_address: v.from_address,
            identity: v.identity,
            role_arn: v.role_arn,
            configuration_set: v.configuration_set,
            messages_per_second: v.messages_per_second,
        }
    }
}

impl From<&PinpointState> for PinpointStateView {
    fn from(state: &PinpointState) -> Self {
        PinpointStateView {
            apps: state
                .apps
                .iter()
                .map(|(k, v)| (k.clone(), PinpointAppView::from(v)))
                .collect(),
            email_channels: state
                .email_channels
                .iter()
                .map(|(k, v)| (k.clone(), EmailChannelView::from(v)))
                .collect(),
        }
    }
}

// --- From view types to internal types ---

fn parse_dt(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<CampaignHookView> for crate::types::CampaignHook {
    fn from(v: CampaignHookView) -> Self {
        crate::types::CampaignHook {
            lambda_function_name: v.lambda_function_name,
            mode: v.mode,
            web_url: v.web_url,
        }
    }
}

impl From<LimitsView> for crate::types::Limits {
    fn from(v: LimitsView) -> Self {
        crate::types::Limits {
            daily: v.daily,
            maximum_duration: v.maximum_duration,
            messages_per_second: v.messages_per_second,
            total: v.total,
            session: v.session,
        }
    }
}

impl From<ApplicationSettingsView> for crate::types::ApplicationSettings {
    fn from(v: ApplicationSettingsView) -> Self {
        crate::types::ApplicationSettings {
            campaign_hook: v.campaign_hook.map(crate::types::CampaignHook::from),
            limits: v.limits.map(crate::types::Limits::from),
            last_modified_date: parse_dt(&v.last_modified_date),
        }
    }
}

impl From<EventStreamView> for crate::types::EventStream {
    fn from(v: EventStreamView) -> Self {
        crate::types::EventStream {
            application_id: v.application_id,
            destination_stream_arn: v.destination_stream_arn,
            role_arn: v.role_arn,
            last_modified_date: parse_dt(&v.last_modified_date),
        }
    }
}

impl From<PinpointAppView> for crate::types::PinpointApp {
    fn from(v: PinpointAppView) -> Self {
        crate::types::PinpointApp {
            id: v.id,
            arn: v.arn,
            name: v.name,
            creation_date: parse_dt(&v.creation_date),
            tags: v.tags,
            settings: v.settings.map(crate::types::ApplicationSettings::from),
            event_stream: v.event_stream.map(crate::types::EventStream::from),
            quiet_time: v.quiet_time,
        }
    }
}

impl From<PinpointStateView> for PinpointState {
    fn from(view: PinpointStateView) -> Self {
        PinpointState {
            apps: view
                .apps
                .into_iter()
                .map(|(k, v)| (k, crate::types::PinpointApp::from(v)))
                .collect(),
            email_channels: view
                .email_channels
                .into_iter()
                .map(|(k, v)| (k, crate::types::EmailChannel::from(v)))
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for PinpointService {
    type StateView = PinpointStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        PinpointStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = PinpointState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (id, app_view) in view.apps {
                guard
                    .apps
                    .insert(id, crate::types::PinpointApp::from(app_view));
            }
            for (id, ec_view) in view.email_channels {
                guard
                    .email_channels
                    .insert(id, crate::types::EmailChannel::from(ec_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
