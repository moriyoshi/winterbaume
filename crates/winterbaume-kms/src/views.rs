//! Serde-compatible view types for KMS state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::KmsService;
use crate::state::KmsState;
use crate::types::{Alias, CustomKeyStore, Grant, GrantConstraints, Key, KeyRotation, KeyState};

/// Serde helper module for serialising `Vec<u8>` as lowercase hex strings.
mod hex_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_str: String = bytes.iter().map(|b| format!("{b:02x}")).collect();
        serializer.serialize_str(&hex_str)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s.len() % 2 != 0 {
            return Err(serde::de::Error::custom("hex string has odd length"));
        }
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(serde::de::Error::custom))
            .collect()
    }
}

/// Serde helper module for serialising `Option<Vec<u8>>` as an optional
/// lowercase hex string.
mod optional_hex_serde {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match bytes {
            Some(b) => {
                let hex_str: String = b.iter().map(|byte| format!("{byte:02x}")).collect();
                serializer.serialize_some(&hex_str)
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
            Some(s) => {
                if s.len() % 2 != 0 {
                    return Err(serde::de::Error::custom("hex string has odd length"));
                }
                let bytes = (0..s.len())
                    .step_by(2)
                    .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(serde::de::Error::custom))
                    .collect::<Result<Vec<u8>, _>>()?;
                Ok(Some(bytes))
            }
        }
    }
}

/// Serializable view of the entire KMS state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KmsStateView {
    /// Keys keyed by key ID.
    #[serde(default)]
    pub keys: HashMap<String, KeyView>,
    /// Aliases keyed by alias name.
    #[serde(default)]
    pub aliases: HashMap<String, AliasView>,
    /// Grants keyed by grant ID.
    #[serde(default)]
    pub grants: HashMap<String, GrantView>,
    /// Key policies keyed by key ID.
    #[serde(default)]
    pub key_policies: HashMap<String, String>,
    /// Key rotation history keyed by key ID.
    #[serde(default)]
    pub key_rotations: HashMap<String, Vec<KeyRotationView>>,
    /// Custom key stores keyed by custom key store ID.
    #[serde(default)]
    pub custom_key_stores: HashMap<String, CustomKeyStoreView>,
    /// Imported key material flags keyed by key ID.
    #[serde(default)]
    pub imported_key_material: HashMap<String, bool>,
}

/// Serializable view of a KMS key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyView {
    pub key_id: String,
    pub arn: String,
    pub account_id: String,
    pub region: String,
    pub description: String,
    pub key_spec: String,
    pub key_usage: String,
    pub key_state: String,
    pub creation_date: String,
    pub enabled: bool,
    pub origin: String,
    pub key_manager: String,
    pub deletion_date: Option<String>,
    pub key_rotation_enabled: bool,
    #[serde(with = "hex_serde")]
    pub key_material: Vec<u8>,
    /// Hex-encoded DER SubjectPublicKeyInfo for asymmetric keys.
    #[serde(default, with = "optional_hex_serde")]
    pub public_key_der: Option<Vec<u8>>,
    pub tags: HashMap<String, String>,
    pub multi_region: bool,
}

/// Serializable view of a KMS alias.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AliasView {
    pub alias_name: String,
    pub alias_arn: String,
    pub target_key_id: String,
}

/// Serializable view of a KMS grant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantView {
    pub grant_id: String,
    pub grant_token: String,
    pub key_id: String,
    pub grantee_principal: String,
    pub retiring_principal: Option<String>,
    pub operations: Vec<String>,
    pub constraints: Option<GrantConstraintsView>,
    pub issuing_account: String,
    pub name: Option<String>,
    pub creation_date: String,
}

/// Serializable view of KMS grant constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrantConstraintsView {
    pub encryption_context_subset: Option<HashMap<String, String>>,
    pub encryption_context_equals: Option<HashMap<String, String>>,
}

/// Serializable view of a key rotation record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotationView {
    pub key_id: String,
    pub rotation_date: String,
    pub rotation_type: String,
}

/// Serializable view of a KMS custom key store.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomKeyStoreView {
    pub custom_key_store_id: String,
    pub custom_key_store_name: String,
    pub cloud_hsm_cluster_id: Option<String>,
    pub trust_anchor_certificate: Option<String>,
    pub connection_state: String,
    pub creation_date: String,
    pub custom_key_store_type: String,
}

// --- From internal types to view types ---

impl From<&KmsState> for KmsStateView {
    fn from(state: &KmsState) -> Self {
        KmsStateView {
            keys: state
                .keys
                .iter()
                .map(|(k, v)| (k.clone(), KeyView::from(v)))
                .collect(),
            aliases: state
                .aliases
                .iter()
                .map(|(k, v)| (k.clone(), AliasView::from(v)))
                .collect(),
            grants: state
                .grants
                .iter()
                .map(|(k, v)| (k.clone(), GrantView::from(v)))
                .collect(),
            key_policies: state.key_policies.clone(),
            key_rotations: state
                .key_rotations
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(KeyRotationView::from).collect()))
                .collect(),
            custom_key_stores: state
                .custom_key_stores
                .iter()
                .map(|(k, v)| (k.clone(), CustomKeyStoreView::from(v)))
                .collect(),
            imported_key_material: state.imported_key_material.clone(),
        }
    }
}

impl From<&Key> for KeyView {
    fn from(key: &Key) -> Self {
        KeyView {
            key_id: key.key_id.clone(),
            arn: key.arn.clone(),
            account_id: key.account_id.clone(),
            region: key.region.clone(),
            description: key.description.clone(),
            key_spec: key.key_spec.clone(),
            key_usage: key.key_usage.clone(),
            key_state: key.key_state.as_str().to_string(),
            creation_date: key.creation_date.to_rfc3339(),
            enabled: key.enabled,
            origin: key.origin.clone(),
            key_manager: key.key_manager.clone(),
            deletion_date: key.deletion_date.map(|d| d.to_rfc3339()),
            key_rotation_enabled: key.key_rotation_enabled,
            key_material: key.key_material.clone(),
            public_key_der: key.public_key_der.clone(),
            tags: key.tags.clone(),
            multi_region: key.multi_region,
        }
    }
}

impl From<&Alias> for AliasView {
    fn from(alias: &Alias) -> Self {
        AliasView {
            alias_name: alias.alias_name.clone(),
            alias_arn: alias.alias_arn.clone(),
            target_key_id: alias.target_key_id.clone(),
        }
    }
}

impl From<&Grant> for GrantView {
    fn from(grant: &Grant) -> Self {
        GrantView {
            grant_id: grant.grant_id.clone(),
            grant_token: grant.grant_token.clone(),
            key_id: grant.key_id.clone(),
            grantee_principal: grant.grantee_principal.clone(),
            retiring_principal: grant.retiring_principal.clone(),
            operations: grant.operations.clone(),
            constraints: grant.constraints.as_ref().map(GrantConstraintsView::from),
            issuing_account: grant.issuing_account.clone(),
            name: grant.name.clone(),
            creation_date: grant.creation_date.to_rfc3339(),
        }
    }
}

impl From<&GrantConstraints> for GrantConstraintsView {
    fn from(gc: &GrantConstraints) -> Self {
        GrantConstraintsView {
            encryption_context_subset: gc.encryption_context_subset.clone(),
            encryption_context_equals: gc.encryption_context_equals.clone(),
        }
    }
}

impl From<&KeyRotation> for KeyRotationView {
    fn from(kr: &KeyRotation) -> Self {
        KeyRotationView {
            key_id: kr.key_id.clone(),
            rotation_date: kr.rotation_date.to_rfc3339(),
            rotation_type: kr.rotation_type.clone(),
        }
    }
}

impl From<&CustomKeyStore> for CustomKeyStoreView {
    fn from(cks: &CustomKeyStore) -> Self {
        CustomKeyStoreView {
            custom_key_store_id: cks.custom_key_store_id.clone(),
            custom_key_store_name: cks.custom_key_store_name.clone(),
            cloud_hsm_cluster_id: cks.cloud_hsm_cluster_id.clone(),
            trust_anchor_certificate: cks.trust_anchor_certificate.clone(),
            connection_state: cks.connection_state.clone(),
            creation_date: cks.creation_date.to_rfc3339(),
            custom_key_store_type: cks.custom_key_store_type.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<KmsStateView> for KmsState {
    fn from(view: KmsStateView) -> Self {
        KmsState {
            keys: view
                .keys
                .into_iter()
                .map(|(k, v)| (k, Key::from(v)))
                .collect(),
            aliases: view
                .aliases
                .into_iter()
                .map(|(k, v)| (k, Alias::from(v)))
                .collect(),
            grants: view
                .grants
                .into_iter()
                .map(|(k, v)| (k, Grant::from(v)))
                .collect(),
            key_policies: view.key_policies,
            key_rotations: view
                .key_rotations
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(KeyRotation::from).collect()))
                .collect(),
            custom_key_stores: view
                .custom_key_stores
                .into_iter()
                .map(|(k, v)| (k, CustomKeyStore::from(v)))
                .collect(),
            imported_key_material: view.imported_key_material,
        }
    }
}

impl From<KeyView> for Key {
    fn from(view: KeyView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let deletion_date = view
            .deletion_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc));
        let key_state = match view.key_state.as_str() {
            "Disabled" => KeyState::Disabled,
            "PendingDeletion" => KeyState::PendingDeletion,
            _ => KeyState::Enabled,
        };
        Key {
            key_id: view.key_id,
            arn: view.arn,
            account_id: view.account_id,
            region: view.region,
            description: view.description,
            key_spec: view.key_spec,
            key_usage: view.key_usage,
            key_state,
            creation_date,
            enabled: view.enabled,
            origin: view.origin,
            key_manager: view.key_manager,
            deletion_date,
            key_rotation_enabled: view.key_rotation_enabled,
            key_material: view.key_material,
            public_key_der: view.public_key_der,
            tags: view.tags,
            multi_region: view.multi_region,
        }
    }
}

impl From<AliasView> for Alias {
    fn from(view: AliasView) -> Self {
        Alias {
            alias_name: view.alias_name,
            alias_arn: view.alias_arn,
            target_key_id: view.target_key_id,
        }
    }
}

impl From<GrantView> for Grant {
    fn from(view: GrantView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        Grant {
            grant_id: view.grant_id,
            grant_token: view.grant_token,
            key_id: view.key_id,
            grantee_principal: view.grantee_principal,
            retiring_principal: view.retiring_principal,
            operations: view.operations,
            constraints: view.constraints.map(GrantConstraints::from),
            issuing_account: view.issuing_account,
            name: view.name,
            creation_date,
        }
    }
}

impl From<GrantConstraintsView> for GrantConstraints {
    fn from(view: GrantConstraintsView) -> Self {
        GrantConstraints {
            encryption_context_subset: view.encryption_context_subset,
            encryption_context_equals: view.encryption_context_equals,
        }
    }
}

impl From<KeyRotationView> for KeyRotation {
    fn from(view: KeyRotationView) -> Self {
        let rotation_date = DateTime::parse_from_rfc3339(&view.rotation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        KeyRotation {
            key_id: view.key_id,
            rotation_date,
            rotation_type: view.rotation_type,
        }
    }
}

impl From<CustomKeyStoreView> for CustomKeyStore {
    fn from(view: CustomKeyStoreView) -> Self {
        let creation_date = DateTime::parse_from_rfc3339(&view.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        CustomKeyStore {
            custom_key_store_id: view.custom_key_store_id,
            custom_key_store_name: view.custom_key_store_name,
            cloud_hsm_cluster_id: view.cloud_hsm_cluster_id,
            trust_anchor_certificate: view.trust_anchor_certificate,
            connection_state: view.connection_state,
            creation_date,
            custom_key_store_type: view.custom_key_store_type,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for KmsService {
    type StateView = KmsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        KmsStateView::from(&*guard)
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
            *guard = KmsState::from(view);
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
            for (key_id, key_view) in view.keys {
                guard.keys.insert(key_id, Key::from(key_view));
            }
            for (alias_name, alias_view) in view.aliases {
                guard.aliases.insert(alias_name, Alias::from(alias_view));
            }
            for (grant_id, grant_view) in view.grants {
                guard.grants.insert(grant_id, Grant::from(grant_view));
            }
            for (key_id, policy) in view.key_policies {
                guard.key_policies.insert(key_id, policy);
            }
            for (key_id, rotations) in view.key_rotations {
                guard.key_rotations.insert(
                    key_id,
                    rotations.into_iter().map(KeyRotation::from).collect(),
                );
            }
            for (id, cks_view) in view.custom_key_stores {
                guard
                    .custom_key_stores
                    .insert(id, CustomKeyStore::from(cks_view));
            }
            for (key_id, imported) in view.imported_key_material {
                guard.imported_key_material.insert(key_id, imported);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
