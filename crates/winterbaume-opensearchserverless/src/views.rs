use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::OpenSearchServerlessService;
use crate::state::OpenSearchServerlessState;
use crate::types::{Collection, SecurityPolicy, Tag, VpcEndpoint};

/// Serializable view of the entire OpenSearch Serverless state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenSearchServerlessStateView {
    #[serde(default)]
    pub collections: HashMap<String, CollectionView>,
    #[serde(default)]
    pub security_policies: HashMap<String, SecurityPolicyView>,
    #[serde(default)]
    pub vpc_endpoints: HashMap<String, VpcEndpointView>,
    /// ARN -> tags
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub type_: String,
    pub status: String,
    pub description: Option<String>,
    pub kms_key_arn: Option<String>,
    pub created_date: i64,
    pub last_modified_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicyView {
    pub name: String,
    pub type_: String,
    pub policy: serde_json::Value,
    pub policy_version: String,
    pub description: Option<String>,
    pub created_date: i64,
    pub last_modified_date: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointView {
    pub id: String,
    pub name: String,
    pub vpc_id: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Collection> for CollectionView {
    fn from(c: &Collection) -> Self {
        CollectionView {
            id: c.id.clone(),
            name: c.name.clone(),
            arn: c.arn.clone(),
            type_: c.type_.clone(),
            status: c.status.clone(),
            description: c.description.clone(),
            kms_key_arn: c.kms_key_arn.clone(),
            created_date: c.created_date,
            last_modified_date: c.last_modified_date,
        }
    }
}

impl From<CollectionView> for Collection {
    fn from(v: CollectionView) -> Self {
        Collection {
            id: v.id,
            name: v.name,
            arn: v.arn,
            type_: v.type_,
            status: v.status,
            description: v.description,
            kms_key_arn: v.kms_key_arn,
            tags: vec![],
            created_date: v.created_date,
            last_modified_date: v.last_modified_date,
        }
    }
}

impl From<&SecurityPolicy> for SecurityPolicyView {
    fn from(sp: &SecurityPolicy) -> Self {
        SecurityPolicyView {
            name: sp.name.clone(),
            type_: sp.type_.clone(),
            policy: sp.policy.clone(),
            policy_version: sp.policy_version.clone(),
            description: sp.description.clone(),
            created_date: sp.created_date,
            last_modified_date: sp.last_modified_date,
        }
    }
}

impl From<SecurityPolicyView> for SecurityPolicy {
    fn from(v: SecurityPolicyView) -> Self {
        SecurityPolicy {
            name: v.name,
            type_: v.type_,
            policy: v.policy,
            policy_version: v.policy_version,
            description: v.description,
            created_date: v.created_date,
            last_modified_date: v.last_modified_date,
        }
    }
}

impl From<&VpcEndpoint> for VpcEndpointView {
    fn from(ep: &VpcEndpoint) -> Self {
        VpcEndpointView {
            id: ep.id.clone(),
            name: ep.name.clone(),
            vpc_id: ep.vpc_id.clone(),
            subnet_ids: ep.subnet_ids.clone(),
            security_group_ids: ep.security_group_ids.clone(),
            status: ep.status.clone(),
        }
    }
}

impl From<VpcEndpointView> for VpcEndpoint {
    fn from(v: VpcEndpointView) -> Self {
        VpcEndpoint {
            id: v.id,
            name: v.name,
            vpc_id: v.vpc_id,
            subnet_ids: v.subnet_ids,
            security_group_ids: v.security_group_ids,
            status: v.status,
        }
    }
}

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Tag {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<&OpenSearchServerlessState> for OpenSearchServerlessStateView {
    fn from(state: &OpenSearchServerlessState) -> Self {
        OpenSearchServerlessStateView {
            collections: state
                .collections
                .iter()
                .map(|(k, v)| (k.clone(), CollectionView::from(v)))
                .collect(),
            security_policies: state
                .security_policies
                .iter()
                .map(|((type_, name), v)| (format!("{type_}/{name}"), SecurityPolicyView::from(v)))
                .collect(),
            vpc_endpoints: state
                .vpc_endpoints
                .iter()
                .map(|(k, v)| (k.clone(), VpcEndpointView::from(v)))
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TagView::from).collect()))
                .collect(),
        }
    }
}

impl From<OpenSearchServerlessStateView> for OpenSearchServerlessState {
    fn from(view: OpenSearchServerlessStateView) -> Self {
        OpenSearchServerlessState {
            collections: view
                .collections
                .into_iter()
                .map(|(k, v)| (k, Collection::from(v)))
                .collect(),
            security_policies: view
                .security_policies
                .into_iter()
                .map(|(key, v)| {
                    let parts: Vec<&str> = key.splitn(2, '/').collect();
                    let type_ = parts.first().copied().unwrap_or("").to_string();
                    let name = parts.get(1).copied().unwrap_or("").to_string();
                    ((type_, name), SecurityPolicy::from(v))
                })
                .collect(),
            vpc_endpoints: view
                .vpc_endpoints
                .into_iter()
                .map(|(k, v)| (k, VpcEndpoint::from(v)))
                .collect(),
            tags: view
                .tags
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(Tag::from).collect()))
                .collect(),
        }
    }
}

impl StatefulService for OpenSearchServerlessService {
    type StateView = OpenSearchServerlessStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        OpenSearchServerlessStateView::from(&*guard)
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
            *guard = OpenSearchServerlessState::from(view);
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
            for (k, v) in view.collections {
                guard.collections.insert(k, Collection::from(v));
            }
            for (key, v) in view.security_policies {
                let parts: Vec<&str> = key.splitn(2, '/').collect();
                let type_ = parts.first().copied().unwrap_or("").to_string();
                let name = parts.get(1).copied().unwrap_or("").to_string();
                guard
                    .security_policies
                    .insert((type_, name), SecurityPolicy::from(v));
            }
            for (k, v) in view.vpc_endpoints {
                guard.vpc_endpoints.insert(k, VpcEndpoint::from(v));
            }
            for (k, v) in view.tags {
                guard.tags.insert(k, v.into_iter().map(Tag::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
