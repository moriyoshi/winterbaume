//! Serde-compatible view types for Secrets Manager state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

/// Serialise `Option<Vec<u8>>` as a lowercase hex string, deserialise back.
mod opt_hex {
    use super::*;

    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(bytes) => {
                let hex: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
                serializer.serialize_some(&hex)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt: Option<String> = Option::deserialize(deserializer)?;
        match opt {
            None => Ok(None),
            Some(hex) => {
                if hex.len() % 2 != 0 {
                    return Err(serde::de::Error::custom("odd-length hex string"));
                }
                let bytes = (0..hex.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
                    .collect::<Result<Vec<u8>, _>>()
                    .map_err(|e| serde::de::Error::custom(format!("invalid hex: {e}")))?;
                Ok(Some(bytes))
            }
        }
    }
}

use crate::handlers::SecretsManagerService;
use crate::state::SecretsManagerState;
use crate::types::{ReplicationStatus, RotationRules, Secret, SecretVersion};

/// Serializable view of the entire Secrets Manager state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretsmanagerStateView {
    /// Secrets keyed by secret name.
    #[serde(default)]
    pub secrets: HashMap<String, SecretView>,
}

/// Serializable view of a single secret.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretView {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub created_date: String,
    pub last_changed_date: String,
    #[serde(default)]
    pub versions: HashMap<String, SecretVersionView>,
    pub current_version_id: Option<String>,
    pub deleted_date: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub resource_policy: Option<String>,
    pub rotation_enabled: Option<bool>,
    pub rotation_lambda_arn: Option<String>,
    pub rotation_rules: Option<RotationRulesView>,
    pub last_rotated_date: Option<String>,
    #[serde(default)]
    pub replication_status: Vec<ReplicationStatusView>,
    pub primary_region: Option<String>,
}

/// Serializable view of a single secret version.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVersionView {
    pub version_id: String,
    pub secret_string: Option<String>,
    #[serde(default, with = "opt_hex")]
    pub secret_binary: Option<Vec<u8>>,
    pub created_date: String,
    #[serde(default)]
    pub version_stages: Vec<String>,
}

/// Serializable view of rotation rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationRulesView {
    pub automatically_after_days: Option<i64>,
    pub duration: Option<String>,
    pub schedule_expression: Option<String>,
}

/// Serializable view of replication status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationStatusView {
    pub region: String,
    pub status: String,
    pub status_message: Option<String>,
    pub kms_key_id: Option<String>,
    pub last_accessed_date: Option<String>,
}

// --- From internal types to view types ---

impl From<&SecretsManagerState> for SecretsmanagerStateView {
    fn from(state: &SecretsManagerState) -> Self {
        SecretsmanagerStateView {
            secrets: state
                .secrets
                .iter()
                .map(|(k, v)| (k.clone(), SecretView::from(v)))
                .collect(),
        }
    }
}

impl From<&Secret> for SecretView {
    fn from(s: &Secret) -> Self {
        SecretView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            description: s.description.clone(),
            created_date: s.created_date.to_rfc3339(),
            last_changed_date: s.last_changed_date.to_rfc3339(),
            versions: s
                .versions
                .iter()
                .map(|(k, v)| (k.clone(), SecretVersionView::from(v)))
                .collect(),
            current_version_id: s.current_version_id.clone(),
            deleted_date: s.deleted_date.map(|d| d.to_rfc3339()),
            tags: s.tags.clone(),
            resource_policy: s.resource_policy.clone(),
            rotation_enabled: s.rotation_enabled,
            rotation_lambda_arn: s.rotation_lambda_arn.clone(),
            rotation_rules: s.rotation_rules.as_ref().map(RotationRulesView::from),
            last_rotated_date: s.last_rotated_date.map(|d| d.to_rfc3339()),
            replication_status: s
                .replication_status
                .iter()
                .map(ReplicationStatusView::from)
                .collect(),
            primary_region: s.primary_region.clone(),
        }
    }
}

impl From<&SecretVersion> for SecretVersionView {
    fn from(v: &SecretVersion) -> Self {
        SecretVersionView {
            version_id: v.version_id.clone(),
            secret_string: v.secret_string.clone(),
            secret_binary: v.secret_binary.clone(),
            created_date: v.created_date.to_rfc3339(),
            version_stages: v.version_stages.clone(),
        }
    }
}

impl From<&RotationRules> for RotationRulesView {
    fn from(r: &RotationRules) -> Self {
        RotationRulesView {
            automatically_after_days: r.automatically_after_days,
            duration: r.duration.clone(),
            schedule_expression: r.schedule_expression.clone(),
        }
    }
}

impl From<&ReplicationStatus> for ReplicationStatusView {
    fn from(r: &ReplicationStatus) -> Self {
        ReplicationStatusView {
            region: r.region.clone(),
            status: r.status.clone(),
            status_message: r.status_message.clone(),
            kms_key_id: r.kms_key_id.clone(),
            last_accessed_date: r.last_accessed_date.map(|d| d.to_rfc3339()),
        }
    }
}

// --- From view types to internal types ---

impl From<SecretsmanagerStateView> for SecretsManagerState {
    fn from(view: SecretsmanagerStateView) -> Self {
        SecretsManagerState {
            secrets: view
                .secrets
                .into_iter()
                .map(|(k, v)| (k, Secret::from(v)))
                .collect(),
        }
    }
}

impl From<SecretView> for Secret {
    fn from(v: SecretView) -> Self {
        let parse_dt = |s: &str| -> DateTime<Utc> {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now())
        };
        Secret {
            name: v.name,
            arn: v.arn,
            description: v.description,
            created_date: parse_dt(&v.created_date),
            last_changed_date: parse_dt(&v.last_changed_date),
            versions: v
                .versions
                .into_iter()
                .map(|(k, sv)| (k, SecretVersion::from(sv)))
                .collect(),
            current_version_id: v.current_version_id,
            deleted_date: v.deleted_date.as_deref().map(parse_dt),
            tags: v.tags,
            resource_policy: v.resource_policy,
            rotation_enabled: v.rotation_enabled,
            rotation_lambda_arn: v.rotation_lambda_arn,
            rotation_rules: v.rotation_rules.map(RotationRules::from),
            last_rotated_date: v.last_rotated_date.as_deref().map(parse_dt),
            replication_status: v
                .replication_status
                .into_iter()
                .map(ReplicationStatus::from)
                .collect(),
            primary_region: v.primary_region,
        }
    }
}

impl From<SecretVersionView> for SecretVersion {
    fn from(v: SecretVersionView) -> Self {
        let created_date = DateTime::parse_from_rfc3339(&v.created_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        SecretVersion {
            version_id: v.version_id,
            secret_string: v.secret_string,
            secret_binary: v.secret_binary,
            created_date,
            version_stages: v.version_stages,
        }
    }
}

impl From<RotationRulesView> for RotationRules {
    fn from(v: RotationRulesView) -> Self {
        RotationRules {
            automatically_after_days: v.automatically_after_days,
            duration: v.duration,
            schedule_expression: v.schedule_expression,
        }
    }
}

impl From<ReplicationStatusView> for ReplicationStatus {
    fn from(v: ReplicationStatusView) -> Self {
        let last_accessed_date = v.last_accessed_date.as_deref().and_then(|s| {
            DateTime::parse_from_rfc3339(s)
                .map(|dt| dt.with_timezone(&Utc))
                .ok()
        });
        ReplicationStatus {
            region: v.region,
            status: v.status,
            status_message: v.status_message,
            kms_key_id: v.kms_key_id,
            last_accessed_date,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for SecretsManagerService {
    type StateView = SecretsmanagerStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        SecretsmanagerStateView::from(&*guard)
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
            *guard = SecretsManagerState::from(view);
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
            for (name, secret_view) in view.secrets {
                guard.secrets.insert(name, Secret::from(secret_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
