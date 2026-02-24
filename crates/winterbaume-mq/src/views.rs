//! Serde-compatible view types for MQ state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::MqService;
use crate::state::MqState;
use crate::types::{Broker, MqConfiguration, MqConfigurationRevision, MqUser};

/// Serializable view of the entire MQ state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MqStateView {
    /// Brokers keyed by broker ID.
    #[serde(default)]
    pub brokers: HashMap<String, BrokerView>,
    /// Broker configurations keyed by configuration ID.
    #[serde(default)]
    pub configurations: HashMap<String, MqConfigurationView>,
}

/// Serializable view of a single MQ broker.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerView {
    pub broker_id: String,
    pub broker_name: String,
    pub broker_arn: String,
    pub broker_state: String,
    pub engine_type: String,
    pub engine_version: String,
    pub host_instance_type: String,
    pub deployment_mode: String,
    /// Creation timestamp in RFC 3339 format.
    pub created: Option<String>,
    pub publicly_accessible: bool,
    pub auto_minor_version_upgrade: bool,
    /// Tags as key-value pairs.
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// Users keyed by username.
    #[serde(default)]
    pub users: HashMap<String, MqUserView>,
    #[serde(default)]
    pub configuration: Option<serde_json::Value>,
    #[serde(default)]
    pub encryption_options: Option<serde_json::Value>,
    #[serde(default)]
    pub ldap_server_metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub maintenance_window_start_time: Option<serde_json::Value>,
}

/// Serializable view of a single MQ user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqUserView {
    pub username: String,
    pub console_access: bool,
    #[serde(default)]
    pub groups: Vec<String>,
    pub replication_user: bool,
}

/// Serializable view of a single MQ configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqConfigurationView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: String,
    pub engine_type: String,
    pub engine_version: String,
    pub authentication_strategy: String,
    /// Creation timestamp in RFC 3339 format.
    pub created: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub revisions: Vec<MqConfigurationRevisionView>,
}

/// Serializable view of a single MQ configuration revision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MqConfigurationRevisionView {
    pub revision: i32,
    /// Creation timestamp in RFC 3339 format.
    pub created: Option<String>,
    pub description: String,
    pub data: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&MqState> for MqStateView {
    fn from(state: &MqState) -> Self {
        let brokers = state
            .brokers
            .iter()
            .map(|(id, broker)| {
                let users = broker
                    .users
                    .iter()
                    .map(|(uname, user)| {
                        (
                            uname.clone(),
                            MqUserView {
                                username: user.username.clone(),
                                console_access: user.console_access,
                                groups: user.groups.clone(),
                                replication_user: user.replication_user,
                            },
                        )
                    })
                    .collect();
                (
                    id.clone(),
                    BrokerView {
                        broker_id: broker.broker_id.clone(),
                        broker_name: broker.broker_name.clone(),
                        broker_arn: broker.broker_arn.clone(),
                        broker_state: broker.broker_state.clone(),
                        engine_type: broker.engine_type.clone(),
                        engine_version: broker.engine_version.clone(),
                        host_instance_type: broker.host_instance_type.clone(),
                        deployment_mode: broker.deployment_mode.clone(),
                        created: Some(broker.created.to_rfc3339()),
                        publicly_accessible: broker.publicly_accessible,
                        auto_minor_version_upgrade: broker.auto_minor_version_upgrade,
                        tags: broker.tags.clone(),
                        users,
                        configuration: None,
                        encryption_options: None,
                        ldap_server_metadata: None,
                        maintenance_window_start_time: None,
                    },
                )
            })
            .collect();

        let configurations = state
            .configurations
            .iter()
            .map(|(id, config)| {
                let revisions = config
                    .revisions
                    .iter()
                    .map(|r| MqConfigurationRevisionView {
                        revision: r.revision,
                        created: Some(r.created.to_rfc3339()),
                        description: r.description.clone(),
                        data: r.data.clone(),
                    })
                    .collect();
                (
                    id.clone(),
                    MqConfigurationView {
                        id: config.id.clone(),
                        arn: config.arn.clone(),
                        name: config.name.clone(),
                        description: config.description.clone(),
                        engine_type: config.engine_type.clone(),
                        engine_version: config.engine_version.clone(),
                        authentication_strategy: config.authentication_strategy.clone(),
                        created: Some(config.created.to_rfc3339()),
                        tags: config.tags.clone(),
                        revisions,
                    },
                )
            })
            .collect();

        MqStateView {
            brokers,
            configurations,
        }
    }
}

impl From<MqStateView> for MqState {
    fn from(view: MqStateView) -> Self {
        let brokers = view
            .brokers
            .into_iter()
            .map(|(id, bv)| {
                let created = bv
                    .created
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                let users = bv
                    .users
                    .into_iter()
                    .map(|(uname, uv)| {
                        (
                            uname,
                            MqUser {
                                username: uv.username,
                                console_access: uv.console_access,
                                groups: uv.groups,
                                replication_user: uv.replication_user,
                            },
                        )
                    })
                    .collect();
                (
                    id,
                    Broker {
                        broker_id: bv.broker_id,
                        broker_name: bv.broker_name,
                        broker_arn: bv.broker_arn,
                        broker_state: bv.broker_state,
                        engine_type: bv.engine_type,
                        engine_version: bv.engine_version,
                        host_instance_type: bv.host_instance_type,
                        deployment_mode: bv.deployment_mode,
                        created,
                        publicly_accessible: bv.publicly_accessible,
                        auto_minor_version_upgrade: bv.auto_minor_version_upgrade,
                        tags: bv.tags,
                        users,
                    },
                )
            })
            .collect();

        let configurations = view
            .configurations
            .into_iter()
            .map(|(id, cv)| {
                let created = cv
                    .created
                    .as_deref()
                    .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                    .map(|dt| dt.with_timezone(&Utc))
                    .unwrap_or_else(Utc::now);
                let revisions = cv
                    .revisions
                    .into_iter()
                    .map(|rv| MqConfigurationRevision {
                        revision: rv.revision,
                        created: rv
                            .created
                            .as_deref()
                            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now),
                        description: rv.description,
                        data: rv.data,
                    })
                    .collect();
                (
                    id,
                    MqConfiguration {
                        id: cv.id,
                        arn: cv.arn,
                        name: cv.name,
                        description: cv.description,
                        engine_type: cv.engine_type,
                        engine_version: cv.engine_version,
                        authentication_strategy: cv.authentication_strategy,
                        created,
                        tags: cv.tags,
                        revisions,
                    },
                )
            })
            .collect();

        MqState {
            brokers,
            configurations,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for MqService {
    type StateView = MqStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        MqStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = MqState::from(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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
            let incoming = MqState::from(view);
            for (id, broker) in incoming.brokers {
                guard.brokers.insert(id, broker);
            }
            for (id, config) in incoming.configurations {
                guard.configurations.insert(id, config);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
