//! Serde-compatible view types for AppSync state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppSyncService;
use crate::state::AppSyncState;
use crate::types::{
    Api, ApiCacheEntry, ApiKeyEntry, ChannelNamespaceEntry, GraphqlApi, SchemaStatus, TypeEntry,
};

/// Serializable view of the entire AppSync state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppsyncStateView {
    /// GraphQL APIs (v1) keyed by api_id.
    #[serde(default)]
    pub apis: HashMap<String, GraphqlApiView>,
    /// Event APIs (v2) keyed by api_id.
    #[serde(default)]
    pub event_apis: HashMap<String, ApiView>,
    /// API caches keyed by api_id.
    #[serde(default)]
    pub api_caches: HashMap<String, ApiCacheView>,
    /// API keys keyed by api_id.
    #[serde(default)]
    pub api_keys: HashMap<String, Vec<ApiKeyView>>,
    /// Channel namespaces keyed by api_id.
    #[serde(default)]
    pub channel_namespaces: HashMap<String, Vec<ChannelNamespaceView>>,
    /// Types keyed by api_id.
    #[serde(default)]
    pub types: HashMap<String, Vec<TypeView>>,
    /// Schema statuses keyed by api_id.
    #[serde(default)]
    pub schema_statuses: HashMap<String, SchemaStatusView>,
    /// Resource tags keyed by resource ARN.
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
}

/// Serializable view of a GraphQL API (v1).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphqlApiView {
    pub api_id: String,
    pub name: String,
    pub authentication_type: String,
    pub arn: String,
    #[serde(default)]
    pub uris: HashMap<String, String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub xray_enabled: bool,
    #[serde(default)]
    pub additional_authentication_provider: Option<serde_json::Value>,
    #[serde(default)]
    pub lambda_authorizer_config: Option<serde_json::Value>,
    #[serde(default)]
    pub user_pool_config: Option<serde_json::Value>,
    #[serde(default)]
    pub enhanced_metrics_config: Option<serde_json::Value>,
}

/// Serializable view of an Event API (v2).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiView {
    pub api_id: String,
    pub name: String,
    pub api_arn: String,
    pub created: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub owner_contact: Option<String>,
}

/// Serializable view of an API cache.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiCacheView {
    pub api_id: String,
    pub api_caching_behavior: String,
    pub r#type: String,
    pub ttl: i64,
    pub at_rest_encryption_enabled: bool,
    pub transit_encryption_enabled: bool,
    pub status: String,
    pub health_metrics_config: Option<String>,
}

/// Serializable view of an API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyView {
    pub id: String,
    pub api_id: String,
    pub description: Option<String>,
    pub expires: i64,
    pub deletes: i64,
}

/// Serializable view of a channel namespace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelNamespaceView {
    pub api_id: String,
    pub name: String,
    pub channel_namespace_arn: String,
    pub created: f64,
    pub last_modified: f64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a GraphQL type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeView {
    pub api_id: String,
    pub name: String,
    pub definition: Option<String>,
    pub format: String,
    pub arn: String,
}

/// Serializable view of a schema creation status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaStatusView {
    pub status: String,
    pub details: Option<String>,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&GraphqlApi> for GraphqlApiView {
    fn from(api: &GraphqlApi) -> Self {
        GraphqlApiView {
            api_id: api.api_id.clone(),
            name: api.name.clone(),
            authentication_type: api.authentication_type.clone(),
            arn: api.arn.clone(),
            uris: api.uris.clone(),
            tags: api.tags.clone(),
            xray_enabled: api.xray_enabled,
            additional_authentication_provider: api.additional_authentication_provider.clone(),
            lambda_authorizer_config: api.lambda_authorizer_config.clone(),
            user_pool_config: api.user_pool_config.clone(),
            enhanced_metrics_config: api.enhanced_metrics_config.clone(),
        }
    }
}

impl From<GraphqlApiView> for GraphqlApi {
    fn from(v: GraphqlApiView) -> Self {
        GraphqlApi {
            api_id: v.api_id,
            name: v.name,
            authentication_type: v.authentication_type,
            arn: v.arn,
            uris: v.uris,
            tags: v.tags,
            xray_enabled: v.xray_enabled,
            additional_authentication_provider: v.additional_authentication_provider,
            lambda_authorizer_config: v.lambda_authorizer_config,
            user_pool_config: v.user_pool_config,
            enhanced_metrics_config: v.enhanced_metrics_config,
        }
    }
}

impl From<&Api> for ApiView {
    fn from(api: &Api) -> Self {
        ApiView {
            api_id: api.api_id.clone(),
            name: api.name.clone(),
            api_arn: api.api_arn.clone(),
            created: api.created,
            tags: api.tags.clone(),
            owner_contact: api.owner_contact.clone(),
        }
    }
}

impl From<ApiView> for Api {
    fn from(v: ApiView) -> Self {
        Api {
            api_id: v.api_id,
            name: v.name,
            api_arn: v.api_arn,
            created: v.created,
            tags: v.tags,
            owner_contact: v.owner_contact,
        }
    }
}

impl From<&ApiCacheEntry> for ApiCacheView {
    fn from(e: &ApiCacheEntry) -> Self {
        ApiCacheView {
            api_id: e.api_id.clone(),
            api_caching_behavior: e.api_caching_behavior.clone(),
            r#type: e.r#type.clone(),
            ttl: e.ttl,
            at_rest_encryption_enabled: e.at_rest_encryption_enabled,
            transit_encryption_enabled: e.transit_encryption_enabled,
            status: e.status.clone(),
            health_metrics_config: e.health_metrics_config.clone(),
        }
    }
}

impl From<ApiCacheView> for ApiCacheEntry {
    fn from(v: ApiCacheView) -> Self {
        ApiCacheEntry {
            api_id: v.api_id,
            api_caching_behavior: v.api_caching_behavior,
            r#type: v.r#type,
            ttl: v.ttl,
            at_rest_encryption_enabled: v.at_rest_encryption_enabled,
            transit_encryption_enabled: v.transit_encryption_enabled,
            status: v.status,
            health_metrics_config: v.health_metrics_config,
        }
    }
}

impl From<&ApiKeyEntry> for ApiKeyView {
    fn from(k: &ApiKeyEntry) -> Self {
        ApiKeyView {
            id: k.id.clone(),
            api_id: k.api_id.clone(),
            description: k.description.clone(),
            expires: k.expires,
            deletes: k.deletes,
        }
    }
}

impl From<ApiKeyView> for ApiKeyEntry {
    fn from(v: ApiKeyView) -> Self {
        ApiKeyEntry {
            id: v.id,
            api_id: v.api_id,
            description: v.description,
            expires: v.expires,
            deletes: v.deletes,
        }
    }
}

impl From<&ChannelNamespaceEntry> for ChannelNamespaceView {
    fn from(ns: &ChannelNamespaceEntry) -> Self {
        ChannelNamespaceView {
            api_id: ns.api_id.clone(),
            name: ns.name.clone(),
            channel_namespace_arn: ns.channel_namespace_arn.clone(),
            created: ns.created,
            last_modified: ns.last_modified,
            tags: ns.tags.clone(),
        }
    }
}

impl From<ChannelNamespaceView> for ChannelNamespaceEntry {
    fn from(v: ChannelNamespaceView) -> Self {
        ChannelNamespaceEntry {
            api_id: v.api_id,
            name: v.name,
            channel_namespace_arn: v.channel_namespace_arn,
            created: v.created,
            last_modified: v.last_modified,
            tags: v.tags,
        }
    }
}

impl From<&TypeEntry> for TypeView {
    fn from(t: &TypeEntry) -> Self {
        TypeView {
            api_id: t.api_id.clone(),
            name: t.name.clone(),
            definition: t.definition.clone(),
            format: t.format.clone(),
            arn: t.arn.clone(),
        }
    }
}

impl From<TypeView> for TypeEntry {
    fn from(v: TypeView) -> Self {
        TypeEntry {
            api_id: v.api_id,
            name: v.name,
            definition: v.definition,
            format: v.format,
            arn: v.arn,
        }
    }
}

impl From<&SchemaStatus> for SchemaStatusView {
    fn from(s: &SchemaStatus) -> Self {
        SchemaStatusView {
            status: s.status.clone(),
            details: s.details.clone(),
        }
    }
}

impl From<SchemaStatusView> for SchemaStatus {
    fn from(v: SchemaStatusView) -> Self {
        SchemaStatus {
            status: v.status,
            details: v.details,
        }
    }
}

impl From<&AppSyncState> for AppsyncStateView {
    fn from(state: &AppSyncState) -> Self {
        AppsyncStateView {
            apis: state
                .apis
                .iter()
                .map(|(k, v)| (k.clone(), GraphqlApiView::from(v)))
                .collect(),
            event_apis: state
                .event_apis
                .iter()
                .map(|(k, v)| (k.clone(), ApiView::from(v)))
                .collect(),
            api_caches: state
                .api_caches
                .iter()
                .map(|(k, v)| (k.clone(), ApiCacheView::from(v)))
                .collect(),
            api_keys: state
                .api_keys
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(ApiKeyView::from).collect()))
                .collect(),
            channel_namespaces: state
                .channel_namespaces
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(ChannelNamespaceView::from).collect(),
                    )
                })
                .collect(),
            types: state
                .types
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TypeView::from).collect()))
                .collect(),
            schema_statuses: state
                .schema_statuses
                .iter()
                .map(|(k, v)| (k.clone(), SchemaStatusView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
        }
    }
}

impl From<AppsyncStateView> for AppSyncState {
    fn from(view: AppsyncStateView) -> Self {
        AppSyncState {
            apis: view
                .apis
                .into_values()
                .map(|v| {
                    let api = GraphqlApi::from(v);
                    (api.api_id.clone(), api)
                })
                .collect(),
            event_apis: view
                .event_apis
                .into_values()
                .map(|v| {
                    let api = Api::from(v);
                    (api.api_id.clone(), api)
                })
                .collect(),
            api_caches: view
                .api_caches
                .into_values()
                .map(|v| {
                    let cache = ApiCacheEntry::from(v);
                    (cache.api_id.clone(), cache)
                })
                .collect(),
            api_keys: view
                .api_keys
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(ApiKeyEntry::from).collect()))
                .collect(),
            channel_namespaces: view
                .channel_namespaces
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(ChannelNamespaceEntry::from).collect()))
                .collect(),
            types: view
                .types
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(TypeEntry::from).collect()))
                .collect(),
            schema_statuses: view
                .schema_statuses
                .into_iter()
                .map(|(k, v)| (k, SchemaStatus::from(v)))
                .collect(),
            resource_tags: view.resource_tags,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for AppSyncService {
    type StateView = AppsyncStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppsyncStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = AppSyncState::from(view);
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
        let incoming = AppSyncState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.apis.extend(incoming.apis);
            guard.event_apis.extend(incoming.event_apis);
            guard.api_caches.extend(incoming.api_caches);
            for (k, v) in incoming.api_keys {
                guard.api_keys.entry(k).or_default().extend(v);
            }
            for (k, v) in incoming.channel_namespaces {
                guard.channel_namespaces.entry(k).or_default().extend(v);
            }
            for (k, v) in incoming.types {
                guard.types.entry(k).or_default().extend(v);
            }
            guard.schema_statuses.extend(incoming.schema_statuses);
            for (k, v) in incoming.resource_tags {
                guard.resource_tags.entry(k).or_default().extend(v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
