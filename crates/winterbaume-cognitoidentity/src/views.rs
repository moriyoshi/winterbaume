//! Serde-compatible view types for Cognito Identity state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CognitoIdentityService;
use crate::state::CognitoIdentityState;
use crate::types::{
    CognitoIdentityProvider, Identity, IdentityPool, IdentityPoolRoles, PrincipalTagEntry,
};

/// Serializable view of the entire Cognito Identity state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CognitoIdentityStateView {
    /// Identity pools keyed by pool ID.
    #[serde(default)]
    pub identity_pools: HashMap<String, IdentityPoolView>,
    /// Identities keyed by pool ID (each value is a list of identities in that pool).
    #[serde(default)]
    pub identities: HashMap<String, Vec<IdentityView>>,
    /// IAM roles keyed by pool ID.
    #[serde(default)]
    pub pool_roles: HashMap<String, IdentityPoolRolesView>,
    /// Principal tag entries keyed by composite key "{pool_id}\x00{provider_name}".
    #[serde(default)]
    pub principal_tags: HashMap<String, PrincipalTagEntryView>,
    /// Resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Developer identity links keyed by composite key.
    #[serde(default)]
    pub developer_identities: HashMap<String, DeveloperIdentityLinkView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperIdentityLinkView {
    pub identity_id: String,
    pub identity_pool_id: String,
    pub developer_provider_name: String,
    pub developer_user_identifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityView {
    pub identity_id: String,
    pub identity_pool_id: String,
    #[serde(default)]
    pub logins: Vec<String>,
    /// Creation date in RFC 3339 format.
    pub creation_date: String,
    /// Last modified date in RFC 3339 format.
    pub last_modified_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityPoolRolesView {
    #[serde(default)]
    pub roles: HashMap<String, String>,
    #[serde(default)]
    pub role_mappings: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrincipalTagEntryView {
    pub identity_pool_id: String,
    pub identity_provider_name: String,
    pub use_defaults: bool,
    #[serde(default)]
    pub principal_tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityPoolView {
    pub identity_pool_id: String,
    pub identity_pool_name: String,
    pub allow_unauthenticated_identities: bool,
    #[serde(default)]
    pub supported_login_providers: HashMap<String, String>,
    pub developer_provider_name: Option<String>,
    #[serde(default)]
    pub open_id_connect_provider_arns: Vec<String>,
    #[serde(default)]
    pub cognito_identity_providers: Vec<CognitoIdentityProviderView>,
    #[serde(default)]
    pub saml_provider_arns: Vec<String>,
    /// Creation date in RFC 3339 format.
    pub created_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitoIdentityProviderView {
    pub provider_name: String,
    pub client_id: String,
    pub server_side_token_check: bool,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&CognitoIdentityProvider> for CognitoIdentityProviderView {
    fn from(p: &CognitoIdentityProvider) -> Self {
        CognitoIdentityProviderView {
            provider_name: p.provider_name.clone(),
            client_id: p.client_id.clone(),
            server_side_token_check: p.server_side_token_check,
        }
    }
}

impl From<CognitoIdentityProviderView> for CognitoIdentityProvider {
    fn from(v: CognitoIdentityProviderView) -> Self {
        CognitoIdentityProvider {
            provider_name: v.provider_name,
            client_id: v.client_id,
            server_side_token_check: v.server_side_token_check,
        }
    }
}

impl From<&Identity> for IdentityView {
    fn from(i: &Identity) -> Self {
        IdentityView {
            identity_id: i.identity_id.clone(),
            identity_pool_id: i.identity_pool_id.clone(),
            logins: i.logins.clone(),
            creation_date: i.creation_date.to_rfc3339(),
            last_modified_date: i.last_modified_date.to_rfc3339(),
        }
    }
}

impl From<IdentityView> for Identity {
    fn from(v: IdentityView) -> Self {
        use chrono::{DateTime, Utc};
        let creation_date = DateTime::parse_from_rfc3339(&v.creation_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        let last_modified_date = DateTime::parse_from_rfc3339(&v.last_modified_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        Identity {
            identity_id: v.identity_id,
            identity_pool_id: v.identity_pool_id,
            logins: v.logins,
            creation_date,
            last_modified_date,
        }
    }
}

impl From<&IdentityPoolRoles> for IdentityPoolRolesView {
    fn from(r: &IdentityPoolRoles) -> Self {
        IdentityPoolRolesView {
            roles: r.roles.clone(),
            role_mappings: r.role_mappings.clone(),
        }
    }
}

impl From<IdentityPoolRolesView> for IdentityPoolRoles {
    fn from(v: IdentityPoolRolesView) -> Self {
        IdentityPoolRoles {
            roles: v.roles,
            role_mappings: v.role_mappings,
        }
    }
}

impl From<&PrincipalTagEntry> for PrincipalTagEntryView {
    fn from(e: &PrincipalTagEntry) -> Self {
        PrincipalTagEntryView {
            identity_pool_id: e.identity_pool_id.clone(),
            identity_provider_name: e.identity_provider_name.clone(),
            use_defaults: e.use_defaults,
            principal_tags: e.principal_tags.clone(),
        }
    }
}

impl From<PrincipalTagEntryView> for PrincipalTagEntry {
    fn from(v: PrincipalTagEntryView) -> Self {
        PrincipalTagEntry {
            identity_pool_id: v.identity_pool_id,
            identity_provider_name: v.identity_provider_name,
            use_defaults: v.use_defaults,
            principal_tags: v.principal_tags,
        }
    }
}

impl From<&IdentityPool> for IdentityPoolView {
    fn from(p: &IdentityPool) -> Self {
        IdentityPoolView {
            identity_pool_id: p.identity_pool_id.clone(),
            identity_pool_name: p.identity_pool_name.clone(),
            allow_unauthenticated_identities: p.allow_unauthenticated_identities,
            supported_login_providers: p.supported_login_providers.clone(),
            developer_provider_name: p.developer_provider_name.clone(),
            open_id_connect_provider_arns: p.open_id_connect_provider_arns.clone(),
            cognito_identity_providers: p
                .cognito_identity_providers
                .iter()
                .map(CognitoIdentityProviderView::from)
                .collect(),
            saml_provider_arns: p.saml_provider_arns.clone(),
            created_date: Some(p.created_date.to_rfc3339()),
        }
    }
}

impl From<IdentityPoolView> for IdentityPool {
    fn from(v: IdentityPoolView) -> Self {
        use chrono::{DateTime, Utc};
        let created_date = v
            .created_date
            .as_deref()
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(Utc::now);
        IdentityPool {
            identity_pool_id: v.identity_pool_id,
            identity_pool_name: v.identity_pool_name,
            allow_unauthenticated_identities: v.allow_unauthenticated_identities,
            supported_login_providers: v.supported_login_providers,
            developer_provider_name: v.developer_provider_name,
            open_id_connect_provider_arns: v.open_id_connect_provider_arns,
            cognito_identity_providers: v
                .cognito_identity_providers
                .into_iter()
                .map(CognitoIdentityProvider::from)
                .collect(),
            saml_provider_arns: v.saml_provider_arns,
            created_date,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for CognitoIdentityService {
    type StateView = CognitoIdentityStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let identity_pools = guard
            .identity_pools
            .iter()
            .map(|(k, v)| (k.clone(), IdentityPoolView::from(v)))
            .collect();

        let identities = guard
            .identities
            .iter()
            .map(|(k, v)| (k.clone(), v.iter().map(IdentityView::from).collect()))
            .collect();

        let pool_roles = guard
            .pool_roles
            .iter()
            .map(|(k, v)| (k.clone(), IdentityPoolRolesView::from(v)))
            .collect();

        let principal_tags = guard
            .principal_tags
            .iter()
            .map(|(k, v)| (k.clone(), PrincipalTagEntryView::from(v)))
            .collect();

        let resource_tags = guard.resource_tags.clone();

        let developer_identities = guard
            .developer_identities
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    DeveloperIdentityLinkView {
                        identity_id: v.identity_id.clone(),
                        identity_pool_id: v.identity_pool_id.clone(),
                        developer_provider_name: v.developer_provider_name.clone(),
                        developer_user_identifier: v.developer_user_identifier.clone(),
                    },
                )
            })
            .collect();

        CognitoIdentityStateView {
            identity_pools,
            identities,
            pool_roles,
            principal_tags,
            resource_tags,
            developer_identities,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = CognitoIdentityState::default();
        for (id, pv) in view.identity_pools {
            new_state.identity_pools.insert(id, IdentityPool::from(pv));
        }
        for (pool_id, iv_list) in view.identities {
            let identities: Vec<Identity> = iv_list.into_iter().map(Identity::from).collect();
            // Rebuild reverse index from identities
            for identity in &identities {
                new_state
                    .identity_index
                    .insert(identity.identity_id.clone(), pool_id.clone());
            }
            new_state.identities.insert(pool_id, identities);
        }
        for (pool_id, rv) in view.pool_roles {
            new_state
                .pool_roles
                .insert(pool_id, IdentityPoolRoles::from(rv));
        }
        for (key, ev) in view.principal_tags {
            new_state
                .principal_tags
                .insert(key, PrincipalTagEntry::from(ev));
        }
        new_state.resource_tags = view.resource_tags;
        for (key, dv) in view.developer_identities {
            new_state.developer_identities.insert(
                key,
                crate::types::DeveloperIdentityLink {
                    identity_id: dv.identity_id,
                    identity_pool_id: dv.identity_pool_id,
                    developer_provider_name: dv.developer_provider_name,
                    developer_user_identifier: dv.developer_user_identifier,
                },
            );
        }
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
            for (id, pv) in view.identity_pools {
                guard.identity_pools.insert(id, IdentityPool::from(pv));
            }
            for (pool_id, iv_list) in view.identities {
                let identities: Vec<Identity> = iv_list.into_iter().map(Identity::from).collect();
                // Rebuild reverse index entries for merged identities
                for identity in &identities {
                    guard
                        .identity_index
                        .insert(identity.identity_id.clone(), pool_id.clone());
                }
                guard.identities.insert(pool_id, identities);
            }
            for (pool_id, rv) in view.pool_roles {
                guard
                    .pool_roles
                    .insert(pool_id, IdentityPoolRoles::from(rv));
            }
            for (key, ev) in view.principal_tags {
                guard
                    .principal_tags
                    .insert(key, PrincipalTagEntry::from(ev));
            }
            for (arn, tags) in view.resource_tags {
                guard.resource_tags.insert(arn, tags);
            }
            for (key, dv) in view.developer_identities {
                guard.developer_identities.insert(
                    key,
                    crate::types::DeveloperIdentityLink {
                        identity_id: dv.identity_id,
                        identity_pool_id: dv.identity_pool_id,
                        developer_provider_name: dv.developer_provider_name,
                        developer_user_identifier: dv.developer_user_identifier,
                    },
                );
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
