//! Serde-compatible view types for WorkSpaces Web state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::WorkspacesWebService;
use crate::state::WorkspacesWebState;
use crate::types::{
    BrowserSettings, DataProtectionSettings, IdentityProvider, IpAccessSettings, IpRule,
    NetworkSettings, Portal, Session, SessionLogger, TrustStore, TrustStoreCertificate,
    UserAccessLoggingSettings, UserSettings,
};

/// Serializable view of the entire WorkSpaces Web state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WorkspacesWebStateView {
    #[serde(default)]
    pub portals: HashMap<String, PortalView>,
    #[serde(default)]
    pub browser_settings: HashMap<String, BrowserSettingsView>,
    #[serde(default)]
    pub network_settings: HashMap<String, NetworkSettingsView>,
    #[serde(default)]
    pub user_access_logging_settings: HashMap<String, UserAccessLoggingSettingsView>,
    #[serde(default)]
    pub user_settings: HashMap<String, UserSettingsView>,
    #[serde(default)]
    pub identity_providers: HashMap<String, IdentityProviderView>,
    #[serde(default)]
    pub ip_access_settings: HashMap<String, IpAccessSettingsView>,
    #[serde(default)]
    pub trust_stores: HashMap<String, TrustStoreView>,
    #[serde(default)]
    pub data_protection_settings: HashMap<String, DataProtectionSettingsView>,
    #[serde(default)]
    pub session_loggers: HashMap<String, SessionLoggerView>,
    #[serde(default)]
    pub sessions: HashMap<String, SessionView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalView {
    pub portal_arn: String,
    pub portal_endpoint: String,
    pub display_name: String,
    pub portal_status: String,
    pub browser_type: String,
    pub renderer_type: String,
    pub creation_date: String,
    pub browser_settings_arn: Option<String>,
    pub network_settings_arn: Option<String>,
    pub user_access_logging_settings_arn: Option<String>,
    pub user_settings_arn: Option<String>,
    #[serde(default)]
    pub trust_store_arn: Option<String>,
    #[serde(default)]
    pub ip_access_settings_arn: Option<String>,
    #[serde(default)]
    pub data_protection_settings_arn: Option<String>,
    #[serde(default)]
    pub session_logger_arn: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserSettingsView {
    pub browser_settings_arn: String,
    pub browser_policy: Option<String>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkSettingsView {
    pub network_settings_arn: String,
    pub vpc_id: String,
    #[serde(default)]
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub security_group_ids: Vec<String>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccessLoggingSettingsView {
    pub user_access_logging_settings_arn: String,
    pub kinesis_stream_arn: String,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettingsView {
    pub user_settings_arn: String,
    pub copy_allowed: String,
    pub paste_allowed: String,
    pub download_allowed: String,
    pub upload_allowed: String,
    pub print_allowed: String,
    pub disconnect_timeout_in_minutes: Option<i32>,
    pub idle_disconnect_timeout_in_minutes: Option<i32>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProviderView {
    pub identity_provider_arn: String,
    pub portal_arn: String,
    pub identity_provider_name: String,
    pub identity_provider_type: String,
    #[serde(default)]
    pub identity_provider_details: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpRuleView {
    pub ip_range: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAccessSettingsView {
    pub ip_access_settings_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub ip_rules: Vec<IpRuleView>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    pub creation_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreCertificateView {
    pub thumbprint: String,
    pub body: String,
    pub issuer: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustStoreView {
    pub trust_store_arn: String,
    #[serde(default)]
    pub certificate_list: Vec<TrustStoreCertificateView>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProtectionSettingsView {
    pub data_protection_settings_arn: String,
    pub display_name: Option<String>,
    pub description: Option<String>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    pub creation_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionLoggerView {
    pub session_logger_arn: String,
    pub display_name: Option<String>,
    #[serde(default)]
    pub associated_portal_arns: Vec<String>,
    pub creation_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionView {
    pub session_id: String,
    pub portal_id: String,
    pub username: Option<String>,
    pub status: String,
    pub start_time: String,
    pub end_time: Option<String>,
    #[serde(default)]
    pub client_ip_addresses: Vec<String>,
}

// --- From internal types to view types ---

impl From<&WorkspacesWebState> for WorkspacesWebStateView {
    fn from(state: &WorkspacesWebState) -> Self {
        WorkspacesWebStateView {
            portals: state
                .portals
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        PortalView {
                            portal_arn: v.portal_arn.clone(),
                            portal_endpoint: v.portal_endpoint.clone(),
                            display_name: v.display_name.clone(),
                            portal_status: v.portal_status.clone(),
                            browser_type: v.browser_type.clone(),
                            renderer_type: v.renderer_type.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            browser_settings_arn: v.browser_settings_arn.clone(),
                            network_settings_arn: v.network_settings_arn.clone(),
                            user_access_logging_settings_arn: v
                                .user_access_logging_settings_arn
                                .clone(),
                            user_settings_arn: v.user_settings_arn.clone(),
                            trust_store_arn: v.trust_store_arn.clone(),
                            ip_access_settings_arn: v.ip_access_settings_arn.clone(),
                            data_protection_settings_arn: v.data_protection_settings_arn.clone(),
                            session_logger_arn: v.session_logger_arn.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            browser_settings: state
                .browser_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        BrowserSettingsView {
                            browser_settings_arn: v.browser_settings_arn.clone(),
                            browser_policy: v.browser_policy.clone(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            network_settings: state
                .network_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        NetworkSettingsView {
                            network_settings_arn: v.network_settings_arn.clone(),
                            vpc_id: v.vpc_id.clone(),
                            subnet_ids: v.subnet_ids.clone(),
                            security_group_ids: v.security_group_ids.clone(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            user_access_logging_settings: state
                .user_access_logging_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        UserAccessLoggingSettingsView {
                            user_access_logging_settings_arn: v
                                .user_access_logging_settings_arn
                                .clone(),
                            kinesis_stream_arn: v.kinesis_stream_arn.clone(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            user_settings: state
                .user_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        UserSettingsView {
                            user_settings_arn: v.user_settings_arn.clone(),
                            copy_allowed: v.copy_allowed.clone(),
                            paste_allowed: v.paste_allowed.clone(),
                            download_allowed: v.download_allowed.clone(),
                            upload_allowed: v.upload_allowed.clone(),
                            print_allowed: v.print_allowed.clone(),
                            disconnect_timeout_in_minutes: v.disconnect_timeout_in_minutes,
                            idle_disconnect_timeout_in_minutes: v
                                .idle_disconnect_timeout_in_minutes,
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            identity_providers: state
                .identity_providers
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IdentityProviderView {
                            identity_provider_arn: v.identity_provider_arn.clone(),
                            portal_arn: v.portal_arn.clone(),
                            identity_provider_name: v.identity_provider_name.clone(),
                            identity_provider_type: v.identity_provider_type.clone(),
                            identity_provider_details: v.identity_provider_details.clone(),
                        },
                    )
                })
                .collect(),
            ip_access_settings: state
                .ip_access_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        IpAccessSettingsView {
                            ip_access_settings_arn: v.ip_access_settings_arn.clone(),
                            display_name: v.display_name.clone(),
                            description: v.description.clone(),
                            ip_rules: v
                                .ip_rules
                                .iter()
                                .map(|r| IpRuleView {
                                    ip_range: r.ip_range.clone(),
                                    description: r.description.clone(),
                                })
                                .collect(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            trust_stores: state
                .trust_stores
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        TrustStoreView {
                            trust_store_arn: v.trust_store_arn.clone(),
                            certificate_list: v
                                .certificate_list
                                .iter()
                                .map(|c| TrustStoreCertificateView {
                                    thumbprint: c.thumbprint.clone(),
                                    body: c.body.clone(),
                                    issuer: c.issuer.clone(),
                                    subject: c.subject.clone(),
                                })
                                .collect(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            data_protection_settings: state
                .data_protection_settings
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        DataProtectionSettingsView {
                            data_protection_settings_arn: v.data_protection_settings_arn.clone(),
                            display_name: v.display_name.clone(),
                            description: v.description.clone(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            session_loggers: state
                .session_loggers
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SessionLoggerView {
                            session_logger_arn: v.session_logger_arn.clone(),
                            display_name: v.display_name.clone(),
                            associated_portal_arns: v.associated_portal_arns.clone(),
                            creation_date: v.creation_date.to_rfc3339(),
                            tags: v.tags.clone(),
                        },
                    )
                })
                .collect(),
            sessions: state
                .sessions
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        SessionView {
                            session_id: v.session_id.clone(),
                            portal_id: v.portal_id.clone(),
                            username: v.username.clone(),
                            status: v.status.clone(),
                            start_time: v.start_time.to_rfc3339(),
                            end_time: v.end_time.map(|t| t.to_rfc3339()),
                            client_ip_addresses: v.client_ip_addresses.clone(),
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- From view types to internal types ---

impl From<WorkspacesWebStateView> for WorkspacesWebState {
    fn from(view: WorkspacesWebStateView) -> Self {
        use chrono::{DateTime, Utc};
        WorkspacesWebState {
            portals: view
                .portals
                .into_iter()
                .map(|(k, v)| {
                    let creation_date = DateTime::parse_from_rfc3339(&v.creation_date)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    (
                        k,
                        Portal {
                            portal_arn: v.portal_arn,
                            portal_endpoint: v.portal_endpoint,
                            display_name: v.display_name,
                            portal_status: v.portal_status,
                            browser_type: v.browser_type,
                            renderer_type: v.renderer_type,
                            creation_date,
                            browser_settings_arn: v.browser_settings_arn,
                            network_settings_arn: v.network_settings_arn,
                            user_access_logging_settings_arn: v.user_access_logging_settings_arn,
                            user_settings_arn: v.user_settings_arn,
                            trust_store_arn: v.trust_store_arn,
                            ip_access_settings_arn: v.ip_access_settings_arn,
                            data_protection_settings_arn: v.data_protection_settings_arn,
                            session_logger_arn: v.session_logger_arn,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            browser_settings: view
                .browser_settings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        BrowserSettings {
                            browser_settings_arn: v.browser_settings_arn,
                            browser_policy: v.browser_policy,
                            associated_portal_arns: v.associated_portal_arns,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            network_settings: view
                .network_settings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        NetworkSettings {
                            network_settings_arn: v.network_settings_arn,
                            vpc_id: v.vpc_id,
                            subnet_ids: v.subnet_ids,
                            security_group_ids: v.security_group_ids,
                            associated_portal_arns: v.associated_portal_arns,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            user_access_logging_settings: view
                .user_access_logging_settings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        UserAccessLoggingSettings {
                            user_access_logging_settings_arn: v.user_access_logging_settings_arn,
                            kinesis_stream_arn: v.kinesis_stream_arn,
                            associated_portal_arns: v.associated_portal_arns,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            user_settings: view
                .user_settings
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        UserSettings {
                            user_settings_arn: v.user_settings_arn,
                            copy_allowed: v.copy_allowed,
                            paste_allowed: v.paste_allowed,
                            download_allowed: v.download_allowed,
                            upload_allowed: v.upload_allowed,
                            print_allowed: v.print_allowed,
                            disconnect_timeout_in_minutes: v.disconnect_timeout_in_minutes,
                            idle_disconnect_timeout_in_minutes: v
                                .idle_disconnect_timeout_in_minutes,
                            associated_portal_arns: v.associated_portal_arns,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            identity_providers: view
                .identity_providers
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        IdentityProvider {
                            identity_provider_arn: v.identity_provider_arn,
                            portal_arn: v.portal_arn,
                            identity_provider_name: v.identity_provider_name,
                            identity_provider_type: v.identity_provider_type,
                            identity_provider_details: v.identity_provider_details,
                        },
                    )
                })
                .collect(),
            ip_access_settings: view
                .ip_access_settings
                .into_iter()
                .map(|(k, v)| {
                    let creation_date = DateTime::parse_from_rfc3339(&v.creation_date)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    (
                        k,
                        IpAccessSettings {
                            ip_access_settings_arn: v.ip_access_settings_arn,
                            display_name: v.display_name,
                            description: v.description,
                            ip_rules: v
                                .ip_rules
                                .into_iter()
                                .map(|r| IpRule {
                                    ip_range: r.ip_range,
                                    description: r.description,
                                })
                                .collect(),
                            associated_portal_arns: v.associated_portal_arns,
                            creation_date,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            trust_stores: view
                .trust_stores
                .into_iter()
                .map(|(k, v)| {
                    (
                        k,
                        TrustStore {
                            trust_store_arn: v.trust_store_arn,
                            certificate_list: v
                                .certificate_list
                                .into_iter()
                                .map(|c| TrustStoreCertificate {
                                    thumbprint: c.thumbprint,
                                    body: c.body,
                                    issuer: c.issuer,
                                    subject: c.subject,
                                })
                                .collect(),
                            associated_portal_arns: v.associated_portal_arns,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            data_protection_settings: view
                .data_protection_settings
                .into_iter()
                .map(|(k, v)| {
                    let creation_date = DateTime::parse_from_rfc3339(&v.creation_date)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    (
                        k,
                        DataProtectionSettings {
                            data_protection_settings_arn: v.data_protection_settings_arn,
                            display_name: v.display_name,
                            description: v.description,
                            associated_portal_arns: v.associated_portal_arns,
                            creation_date,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            session_loggers: view
                .session_loggers
                .into_iter()
                .map(|(k, v)| {
                    let creation_date = DateTime::parse_from_rfc3339(&v.creation_date)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    (
                        k,
                        SessionLogger {
                            session_logger_arn: v.session_logger_arn,
                            display_name: v.display_name,
                            associated_portal_arns: v.associated_portal_arns,
                            creation_date,
                            tags: v.tags,
                        },
                    )
                })
                .collect(),
            sessions: view
                .sessions
                .into_iter()
                .map(|(k, v)| {
                    let start_time = DateTime::parse_from_rfc3339(&v.start_time)
                        .map(|dt| dt.with_timezone(&Utc))
                        .unwrap_or_else(|_| Utc::now());
                    let end_time = v.end_time.as_deref().and_then(|t| {
                        DateTime::parse_from_rfc3339(t)
                            .ok()
                            .map(|dt| dt.with_timezone(&Utc))
                    });
                    (
                        k,
                        Session {
                            session_id: v.session_id,
                            portal_id: v.portal_id,
                            username: v.username,
                            status: v.status,
                            start_time,
                            end_time,
                            client_ip_addresses: v.client_ip_addresses,
                        },
                    )
                })
                .collect(),
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for WorkspacesWebService {
    type StateView = WorkspacesWebStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        WorkspacesWebStateView::from(&*guard)
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
            *guard = WorkspacesWebState::from(view);
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
            let new_state = WorkspacesWebState::from(view);
            guard.portals.extend(new_state.portals);
            guard.browser_settings.extend(new_state.browser_settings);
            guard.network_settings.extend(new_state.network_settings);
            guard
                .user_access_logging_settings
                .extend(new_state.user_access_logging_settings);
            guard.user_settings.extend(new_state.user_settings);
            guard
                .identity_providers
                .extend(new_state.identity_providers);
            guard
                .ip_access_settings
                .extend(new_state.ip_access_settings);
            guard.trust_stores.extend(new_state.trust_stores);
            guard
                .data_protection_settings
                .extend(new_state.data_protection_settings);
            guard.session_loggers.extend(new_state.session_loggers);
            guard.sessions.extend(new_state.sessions);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
