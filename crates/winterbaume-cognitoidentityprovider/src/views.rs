//! Serde-compatible view types for Cognito IDP state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CognitoIdentityProviderService;
use crate::state::CognitoIdpState;
use crate::types::{
    CognitoUser, CustomDomainConfig, Group, IdentityProvider, MfaConfig, ResourceServer,
    ResourceServerScope, SmsMfaConfig, SoftwareTokenMfaConfig, UserPool, UserPoolClient,
    UserPoolDomain,
};

/// Serializable view of the entire Cognito IDP state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitoidpStateView {
    /// User pools keyed by pool ID.
    #[serde(default)]
    pub user_pools: HashMap<String, UserPoolView>,
}

/// Serializable view of a single user pool and all its sub-resources.
///
/// Intentionally excluded fields (transient or out-of-scope for restore):
/// - `devices`: not tracked across restores; rebuilt from live operations.
/// - `ui_customization`: opaque customization blob; excluded for brevity.
/// - `managed_login_brandings`: new-style branding; excluded for brevity.
/// - `risk_configuration`: adaptive auth config stored as opaque JSON; excluded.
/// - `log_delivery_config`: log delivery settings stored as opaque JSON; excluded.
/// - `user_import_jobs`: import job lifecycle is transient; excluded.
/// - `terms`: lightweight opaque JSON; excluded for brevity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPoolView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub created_date: String,
    #[serde(default)]
    pub clients: HashMap<String, UserPoolClientView>,
    #[serde(default)]
    pub users: HashMap<String, CognitoUserView>,
    #[serde(default)]
    pub groups: HashMap<String, GroupView>,
    #[serde(default)]
    pub identity_providers: HashMap<String, IdentityProviderView>,
    #[serde(default)]
    pub resource_servers: HashMap<String, ResourceServerView>,
    pub domain: Option<UserPoolDomainView>,
    pub mfa_config: Option<MfaConfigView>,
    #[serde(default)]
    pub custom_attributes: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a user pool client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPoolClientView {
    pub id: String,
    pub name: String,
    pub user_pool_id: String,
    pub client_secret: Option<String>,
    #[serde(default)]
    pub explicit_auth_flows: Vec<String>,
    #[serde(default)]
    pub allowed_oauth_flows: Vec<String>,
    #[serde(default)]
    pub allowed_oauth_scopes: Vec<String>,
    #[serde(default)]
    pub callback_urls: Vec<String>,
    #[serde(default)]
    pub logout_urls: Vec<String>,
    #[serde(default)]
    pub allowed_oauth_flows_user_pool_client: bool,
    #[serde(default)]
    pub refresh_token_validity: Option<i32>,
    #[serde(default)]
    pub supported_identity_providers: Vec<String>,
}

/// Serializable view of a Cognito user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitoUserView {
    pub username: String,
    pub status: String,
    pub created_date: String,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    pub enabled: bool,
    pub password: Option<String>,
    pub confirmed: bool,
    #[serde(default)]
    pub groups: Vec<String>,
}

/// Serializable view of a group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupView {
    pub group_name: String,
    pub user_pool_id: String,
    pub description: Option<String>,
    pub role_arn: Option<String>,
    pub precedence: Option<i32>,
    pub created_date: String,
    pub last_modified_date: String,
}

/// Serializable view of an identity provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityProviderView {
    pub provider_name: String,
    pub provider_type: String,
    pub user_pool_id: String,
    #[serde(default)]
    pub provider_details: HashMap<String, String>,
    #[serde(default)]
    pub attribute_mapping: HashMap<String, String>,
    #[serde(default)]
    pub idp_identifiers: Vec<String>,
    pub created_date: String,
    pub last_modified_date: String,
}

/// Serializable view of a resource server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceServerView {
    pub identifier: String,
    pub name: String,
    pub user_pool_id: String,
    #[serde(default)]
    pub scopes: Vec<ResourceServerScopeView>,
}

/// Serializable view of a resource server scope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceServerScopeView {
    pub scope_name: String,
    pub scope_description: String,
}

/// Serializable view of a user pool domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPoolDomainView {
    pub domain: String,
    pub user_pool_id: String,
    pub status: String,
    pub cloud_front_distribution: Option<String>,
    pub custom_domain_config: Option<CustomDomainConfigView>,
}

/// Serializable view of a custom domain config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomDomainConfigView {
    pub certificate_arn: String,
}

/// Serializable view of MFA configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaConfigView {
    pub mfa_configuration: String,
    pub sms_mfa_configuration: Option<SmsMfaConfigView>,
    pub software_token_mfa_configuration: Option<SoftwareTokenMfaConfigView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmsMfaConfigView {
    pub sms_authentication_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareTokenMfaConfigView {
    pub enabled: bool,
}

// --- From internal types to view types ---

impl From<&CognitoIdpState> for CognitoidpStateView {
    fn from(state: &CognitoIdpState) -> Self {
        CognitoidpStateView {
            user_pools: state
                .user_pools
                .iter()
                .map(|(k, v)| (k.clone(), UserPoolView::from(v)))
                .collect(),
        }
    }
}

impl From<&UserPool> for UserPoolView {
    fn from(pool: &UserPool) -> Self {
        UserPoolView {
            id: pool.id.clone(),
            name: pool.name.clone(),
            arn: pool.arn.clone(),
            status: pool.status.clone(),
            created_date: pool.created_date.to_rfc3339(),
            clients: pool
                .clients
                .iter()
                .map(|(k, v)| (k.clone(), UserPoolClientView::from(v)))
                .collect(),
            users: pool
                .users
                .iter()
                .map(|(k, v)| (k.clone(), CognitoUserView::from(v)))
                .collect(),
            groups: pool
                .groups
                .iter()
                .map(|(k, v)| (k.clone(), GroupView::from(v)))
                .collect(),
            identity_providers: pool
                .identity_providers
                .iter()
                .map(|(k, v)| (k.clone(), IdentityProviderView::from(v)))
                .collect(),
            resource_servers: pool
                .resource_servers
                .iter()
                .map(|(k, v)| (k.clone(), ResourceServerView::from(v)))
                .collect(),
            domain: pool.domain.as_ref().map(UserPoolDomainView::from),
            mfa_config: pool.mfa_config.as_ref().map(MfaConfigView::from),
            custom_attributes: pool.custom_attributes.clone(),
            tags: pool.tags.clone(),
        }
    }
}

impl From<&UserPoolClient> for UserPoolClientView {
    fn from(client: &UserPoolClient) -> Self {
        UserPoolClientView {
            id: client.id.clone(),
            name: client.name.clone(),
            user_pool_id: client.user_pool_id.clone(),
            client_secret: client.client_secret.clone(),
            explicit_auth_flows: client.explicit_auth_flows.clone(),
            allowed_oauth_flows: client.allowed_oauth_flows.clone(),
            allowed_oauth_scopes: client.allowed_oauth_scopes.clone(),
            callback_urls: client.callback_urls.clone(),
            logout_urls: client.logout_urls.clone(),
            allowed_oauth_flows_user_pool_client: client.allowed_oauth_flows_user_pool_client,
            refresh_token_validity: client.refresh_token_validity,
            supported_identity_providers: client.supported_identity_providers.clone(),
        }
    }
}

impl From<&CognitoUser> for CognitoUserView {
    fn from(user: &CognitoUser) -> Self {
        CognitoUserView {
            username: user.username.clone(),
            status: user.status.clone(),
            created_date: user.created_date.to_rfc3339(),
            attributes: user.attributes.clone(),
            enabled: user.enabled,
            password: user.password.clone(),
            confirmed: user.confirmed,
            groups: user.groups.clone(),
        }
    }
}

impl From<&Group> for GroupView {
    fn from(group: &Group) -> Self {
        GroupView {
            group_name: group.group_name.clone(),
            user_pool_id: group.user_pool_id.clone(),
            description: group.description.clone(),
            role_arn: group.role_arn.clone(),
            precedence: group.precedence,
            created_date: group.created_date.to_rfc3339(),
            last_modified_date: group.last_modified_date.to_rfc3339(),
        }
    }
}

impl From<&IdentityProvider> for IdentityProviderView {
    fn from(idp: &IdentityProvider) -> Self {
        IdentityProviderView {
            provider_name: idp.provider_name.clone(),
            provider_type: idp.provider_type.clone(),
            user_pool_id: idp.user_pool_id.clone(),
            provider_details: idp.provider_details.clone(),
            attribute_mapping: idp.attribute_mapping.clone(),
            idp_identifiers: idp.idp_identifiers.clone(),
            created_date: idp.created_date.to_rfc3339(),
            last_modified_date: idp.last_modified_date.to_rfc3339(),
        }
    }
}

impl From<&ResourceServer> for ResourceServerView {
    fn from(rs: &ResourceServer) -> Self {
        ResourceServerView {
            identifier: rs.identifier.clone(),
            name: rs.name.clone(),
            user_pool_id: rs.user_pool_id.clone(),
            scopes: rs
                .scopes
                .iter()
                .map(ResourceServerScopeView::from)
                .collect(),
        }
    }
}

impl From<&ResourceServerScope> for ResourceServerScopeView {
    fn from(scope: &ResourceServerScope) -> Self {
        ResourceServerScopeView {
            scope_name: scope.scope_name.clone(),
            scope_description: scope.scope_description.clone(),
        }
    }
}

impl From<&UserPoolDomain> for UserPoolDomainView {
    fn from(domain: &UserPoolDomain) -> Self {
        UserPoolDomainView {
            domain: domain.domain.clone(),
            user_pool_id: domain.user_pool_id.clone(),
            status: domain.status.clone(),
            cloud_front_distribution: domain.cloud_front_distribution.clone(),
            custom_domain_config: domain
                .custom_domain_config
                .as_ref()
                .map(CustomDomainConfigView::from),
        }
    }
}

impl From<&CustomDomainConfig> for CustomDomainConfigView {
    fn from(cfg: &CustomDomainConfig) -> Self {
        CustomDomainConfigView {
            certificate_arn: cfg.certificate_arn.clone(),
        }
    }
}

impl From<&MfaConfig> for MfaConfigView {
    fn from(cfg: &MfaConfig) -> Self {
        MfaConfigView {
            mfa_configuration: cfg.mfa_configuration.clone(),
            sms_mfa_configuration: cfg
                .sms_mfa_configuration
                .as_ref()
                .map(SmsMfaConfigView::from),
            software_token_mfa_configuration: cfg
                .software_token_mfa_configuration
                .as_ref()
                .map(SoftwareTokenMfaConfigView::from),
        }
    }
}

impl From<&SmsMfaConfig> for SmsMfaConfigView {
    fn from(cfg: &SmsMfaConfig) -> Self {
        SmsMfaConfigView {
            sms_authentication_message: cfg.sms_authentication_message.clone(),
        }
    }
}

impl From<&SoftwareTokenMfaConfig> for SoftwareTokenMfaConfigView {
    fn from(cfg: &SoftwareTokenMfaConfig) -> Self {
        SoftwareTokenMfaConfigView {
            enabled: cfg.enabled,
        }
    }
}

// --- From view types to internal types ---

impl From<CognitoidpStateView> for CognitoIdpState {
    fn from(view: CognitoidpStateView) -> Self {
        CognitoIdpState {
            user_pools: view
                .user_pools
                .into_iter()
                .map(|(k, v)| (k, UserPool::from(v)))
                .collect(),
        }
    }
}

impl From<UserPoolView> for UserPool {
    fn from(view: UserPoolView) -> Self {
        let created_date = view
            .created_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        UserPool {
            id: view.id,
            name: view.name,
            arn: view.arn,
            status: view.status,
            created_date,
            clients: view
                .clients
                .into_iter()
                .map(|(k, v)| (k, UserPoolClient::from(v)))
                .collect(),
            users: view
                .users
                .into_iter()
                .map(|(k, v)| (k, CognitoUser::from(v)))
                .collect(),
            groups: view
                .groups
                .into_iter()
                .map(|(k, v)| (k, Group::from(v)))
                .collect(),
            identity_providers: view
                .identity_providers
                .into_iter()
                .map(|(k, v)| (k, IdentityProvider::from(v)))
                .collect(),
            resource_servers: view
                .resource_servers
                .into_iter()
                .map(|(k, v)| (k, ResourceServer::from(v)))
                .collect(),
            domain: view.domain.map(UserPoolDomain::from),
            mfa_config: view.mfa_config.map(MfaConfig::from),
            custom_attributes: view.custom_attributes,
            tags: view.tags,
            devices: HashMap::new(),
            ui_customization: None,
            managed_login_brandings: HashMap::new(),
            risk_configuration: None,
            log_delivery_config: None,
            user_import_jobs: HashMap::new(),
            terms: HashMap::new(),
        }
    }
}

impl From<UserPoolClientView> for UserPoolClient {
    fn from(view: UserPoolClientView) -> Self {
        UserPoolClient {
            id: view.id,
            name: view.name,
            user_pool_id: view.user_pool_id,
            client_secret: view.client_secret,
            explicit_auth_flows: view.explicit_auth_flows,
            allowed_oauth_flows: view.allowed_oauth_flows,
            allowed_oauth_scopes: view.allowed_oauth_scopes,
            callback_urls: view.callback_urls,
            logout_urls: view.logout_urls,
            allowed_oauth_flows_user_pool_client: view.allowed_oauth_flows_user_pool_client,
            refresh_token_validity: view.refresh_token_validity,
            supported_identity_providers: view.supported_identity_providers,
        }
    }
}

impl From<CognitoUserView> for CognitoUser {
    fn from(view: CognitoUserView) -> Self {
        let created_date = view
            .created_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        CognitoUser {
            username: view.username,
            status: view.status,
            created_date,
            attributes: view.attributes,
            enabled: view.enabled,
            password: view.password,
            confirmed: view.confirmed,
            groups: view.groups,
            linked_providers: Vec::new(),
        }
    }
}

impl From<GroupView> for Group {
    fn from(view: GroupView) -> Self {
        let created_date = view
            .created_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        let last_modified_date = view
            .last_modified_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        Group {
            group_name: view.group_name,
            user_pool_id: view.user_pool_id,
            description: view.description,
            role_arn: view.role_arn,
            precedence: view.precedence,
            created_date,
            last_modified_date,
        }
    }
}

impl From<IdentityProviderView> for IdentityProvider {
    fn from(view: IdentityProviderView) -> Self {
        let created_date = view
            .created_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        let last_modified_date = view
            .last_modified_date
            .parse::<DateTime<Utc>>()
            .unwrap_or_else(|_| Utc::now());
        IdentityProvider {
            provider_name: view.provider_name,
            provider_type: view.provider_type,
            user_pool_id: view.user_pool_id,
            provider_details: view.provider_details,
            attribute_mapping: view.attribute_mapping,
            idp_identifiers: view.idp_identifiers,
            created_date,
            last_modified_date,
        }
    }
}

impl From<ResourceServerView> for ResourceServer {
    fn from(view: ResourceServerView) -> Self {
        ResourceServer {
            identifier: view.identifier,
            name: view.name,
            user_pool_id: view.user_pool_id,
            scopes: view
                .scopes
                .into_iter()
                .map(ResourceServerScope::from)
                .collect(),
        }
    }
}

impl From<ResourceServerScopeView> for ResourceServerScope {
    fn from(view: ResourceServerScopeView) -> Self {
        ResourceServerScope {
            scope_name: view.scope_name,
            scope_description: view.scope_description,
        }
    }
}

impl From<UserPoolDomainView> for UserPoolDomain {
    fn from(view: UserPoolDomainView) -> Self {
        UserPoolDomain {
            domain: view.domain,
            user_pool_id: view.user_pool_id,
            status: view.status,
            cloud_front_distribution: view.cloud_front_distribution,
            custom_domain_config: view.custom_domain_config.map(CustomDomainConfig::from),
        }
    }
}

impl From<CustomDomainConfigView> for CustomDomainConfig {
    fn from(view: CustomDomainConfigView) -> Self {
        CustomDomainConfig {
            certificate_arn: view.certificate_arn,
        }
    }
}

impl From<MfaConfigView> for MfaConfig {
    fn from(view: MfaConfigView) -> Self {
        MfaConfig {
            mfa_configuration: view.mfa_configuration,
            sms_mfa_configuration: view.sms_mfa_configuration.map(SmsMfaConfig::from),
            software_token_mfa_configuration: view
                .software_token_mfa_configuration
                .map(SoftwareTokenMfaConfig::from),
        }
    }
}

impl From<SmsMfaConfigView> for SmsMfaConfig {
    fn from(view: SmsMfaConfigView) -> Self {
        SmsMfaConfig {
            sms_authentication_message: view.sms_authentication_message,
        }
    }
}

impl From<SoftwareTokenMfaConfigView> for SoftwareTokenMfaConfig {
    fn from(view: SoftwareTokenMfaConfigView) -> Self {
        SoftwareTokenMfaConfig {
            enabled: view.enabled,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for CognitoIdentityProviderService {
    type StateView = CognitoidpStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CognitoidpStateView::from(&*guard)
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
            *guard = CognitoIdpState::from(view);
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
            for (id, pool_view) in view.user_pools {
                guard.user_pools.insert(id, UserPool::from(pool_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
