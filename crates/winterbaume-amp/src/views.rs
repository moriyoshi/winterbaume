//! Serde-compatible view types for AMP (Amazon Managed Service for Prometheus) state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AmpService;
use crate::state::AmpState;
use crate::types::{
    LoggingConfiguration, LoggingConfigurationStatus, RuleGroupsNamespace,
    RuleGroupsNamespaceStatus, Workspace, WorkspaceStatus,
};

/// Serializable view of the entire AMP state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AmpStateView {
    /// Workspaces keyed by workspace ID.
    #[serde(default)]
    pub workspaces: HashMap<String, WorkspaceView>,
    /// Logging configurations keyed by workspace ID.
    #[serde(default)]
    pub logging_configs: HashMap<String, LoggingConfigurationView>,
    /// Rule groups namespaces keyed by workspace ID, then by namespace name.
    #[serde(default)]
    pub rule_groups_namespaces: HashMap<String, HashMap<String, RuleGroupsNamespaceView>>,
}

/// Serializable view of an AMP workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceView {
    pub workspace_id: String,
    pub arn: String,
    pub alias: Option<String>,
    pub status_code: String,
    pub prometheus_endpoint: String,
    pub created_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of an AMP logging configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfigurationView {
    pub workspace_id: String,
    pub log_group_arn: String,
    pub status_code: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}

/// Serializable view of an AMP rule groups namespace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleGroupsNamespaceView {
    pub name: String,
    pub arn: String,
    pub workspace_id: String,
    pub data: String,
    pub status_code: String,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Workspace> for WorkspaceView {
    fn from(ws: &Workspace) -> Self {
        WorkspaceView {
            workspace_id: ws.workspace_id.clone(),
            arn: ws.arn.clone(),
            alias: ws.alias.clone(),
            status_code: ws.status.status_code.clone(),
            prometheus_endpoint: ws.prometheus_endpoint.clone(),
            created_at: Some(ws.created_at.to_rfc3339()),
            tags: ws.tags.clone(),
        }
    }
}

impl From<WorkspaceView> for Workspace {
    fn from(v: WorkspaceView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        Workspace {
            workspace_id: v.workspace_id,
            arn: v.arn,
            alias: v.alias,
            status: WorkspaceStatus {
                status_code: v.status_code,
            },
            prometheus_endpoint: v.prometheus_endpoint,
            created_at,
            tags: v.tags,
        }
    }
}

impl From<&LoggingConfiguration> for LoggingConfigurationView {
    fn from(lc: &LoggingConfiguration) -> Self {
        LoggingConfigurationView {
            workspace_id: lc.workspace_id.clone(),
            log_group_arn: lc.log_group_arn.clone(),
            status_code: lc.status.status_code.clone(),
            created_at: Some(lc.created_at.to_rfc3339()),
            modified_at: Some(lc.modified_at.to_rfc3339()),
        }
    }
}

impl From<LoggingConfigurationView> for LoggingConfiguration {
    fn from(v: LoggingConfigurationView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let modified_at = v
            .modified_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        LoggingConfiguration {
            workspace_id: v.workspace_id,
            log_group_arn: v.log_group_arn,
            status: LoggingConfigurationStatus {
                status_code: v.status_code,
            },
            created_at,
            modified_at,
        }
    }
}

impl From<&RuleGroupsNamespace> for RuleGroupsNamespaceView {
    fn from(ns: &RuleGroupsNamespace) -> Self {
        RuleGroupsNamespaceView {
            name: ns.name.clone(),
            arn: ns.arn.clone(),
            workspace_id: ns.workspace_id.clone(),
            data: ns.data.clone(),
            status_code: ns.status.status_code.clone(),
            created_at: Some(ns.created_at.to_rfc3339()),
            modified_at: Some(ns.modified_at.to_rfc3339()),
            tags: ns.tags.clone(),
        }
    }
}

impl From<RuleGroupsNamespaceView> for RuleGroupsNamespace {
    fn from(v: RuleGroupsNamespaceView) -> Self {
        let created_at = v
            .created_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        let modified_at = v
            .modified_at
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        RuleGroupsNamespace {
            name: v.name,
            arn: v.arn,
            workspace_id: v.workspace_id,
            data: v.data,
            status: RuleGroupsNamespaceStatus {
                status_code: v.status_code,
            },
            created_at,
            modified_at,
            tags: v.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for AmpService {
    type StateView = AmpStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let workspaces = guard
            .workspaces
            .iter()
            .map(|(k, v)| (k.clone(), WorkspaceView::from(v)))
            .collect();

        let logging_configs = guard
            .logging_configs
            .iter()
            .map(|(k, v)| (k.clone(), LoggingConfigurationView::from(v)))
            .collect();

        // Nest rule groups namespaces: workspace_id -> name -> view
        let mut rule_groups_namespaces: HashMap<String, HashMap<String, RuleGroupsNamespaceView>> =
            HashMap::new();
        for ((workspace_id, name), ns) in &guard.rule_groups_namespaces {
            rule_groups_namespaces
                .entry(workspace_id.clone())
                .or_default()
                .insert(name.clone(), RuleGroupsNamespaceView::from(ns));
        }

        AmpStateView {
            workspaces,
            logging_configs,
            rule_groups_namespaces,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = AmpState {
            workspaces: view
                .workspaces
                .into_values()
                .map(|v| {
                    let ws = Workspace::from(v);
                    (ws.workspace_id.clone(), ws)
                })
                .collect(),
            logging_configs: view
                .logging_configs
                .into_values()
                .map(|v| {
                    let lc = LoggingConfiguration::from(v);
                    (lc.workspace_id.clone(), lc)
                })
                .collect(),
            rule_groups_namespaces: view
                .rule_groups_namespaces
                .into_iter()
                .flat_map(|(workspace_id, namespaces)| {
                    namespaces.into_values().map(move |v| {
                        let ns = RuleGroupsNamespace::from(v);
                        ((workspace_id.clone(), ns.name.clone()), ns)
                    })
                })
                .collect(),
        };

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
            for v in view.workspaces.into_values() {
                let ws = Workspace::from(v);
                guard.workspaces.insert(ws.workspace_id.clone(), ws);
            }
            for v in view.logging_configs.into_values() {
                let lc = LoggingConfiguration::from(v);
                guard.logging_configs.insert(lc.workspace_id.clone(), lc);
            }
            for (workspace_id, namespaces) in view.rule_groups_namespaces {
                for v in namespaces.into_values() {
                    let ns = RuleGroupsNamespace::from(v);
                    guard
                        .rule_groups_namespaces
                        .insert((workspace_id.clone(), ns.name.clone()), ns);
                }
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
