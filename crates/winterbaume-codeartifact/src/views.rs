use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CodeArtifactService;
use crate::state::CodeArtifactState;
use crate::types::{Domain, Repository};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CodeArtifactStateView {
    #[serde(default)]
    pub domains: HashMap<String, DomainView>,
    #[serde(default)]
    pub repositories: HashMap<String, RepositoryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainView {
    pub name: String,
    pub owner: String,
    pub arn: String,
    pub encryption_key: Option<String>,
    pub status: String,
    pub created_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryView {
    pub name: String,
    pub domain_name: String,
    pub domain_owner: String,
    pub arn: String,
    pub description: Option<String>,
    pub created_time: f64,
    pub tags: HashMap<String, String>,
    #[serde(default)]
    pub external_connections: Option<serde_json::Value>,
    #[serde(default)]
    pub upstream: Option<serde_json::Value>,
}

impl From<&Domain> for DomainView {
    fn from(d: &Domain) -> Self {
        Self {
            name: d.name.clone(),
            owner: d.owner.clone(),
            arn: d.arn.clone(),
            encryption_key: d.encryption_key.clone(),
            status: d.status.clone(),
            created_time: d.created_time,
            tags: d.tags.clone(),
        }
    }
}

impl From<DomainView> for Domain {
    fn from(v: DomainView) -> Self {
        Self {
            name: v.name,
            owner: v.owner,
            arn: v.arn,
            encryption_key: v.encryption_key,
            status: v.status,
            created_time: v.created_time,
            tags: v.tags,
        }
    }
}

impl From<&Repository> for RepositoryView {
    fn from(r: &Repository) -> Self {
        Self {
            name: r.name.clone(),
            domain_name: r.domain_name.clone(),
            domain_owner: r.domain_owner.clone(),
            arn: r.arn.clone(),
            description: r.description.clone(),
            created_time: r.created_time,
            tags: r.tags.clone(),
            external_connections: None,
            upstream: None,
        }
    }
}

impl From<RepositoryView> for Repository {
    fn from(v: RepositoryView) -> Self {
        Self {
            name: v.name,
            domain_name: v.domain_name,
            domain_owner: v.domain_owner,
            arn: v.arn,
            description: v.description,
            created_time: v.created_time,
            tags: v.tags,
        }
    }
}

impl From<&CodeArtifactState> for CodeArtifactStateView {
    fn from(state: &CodeArtifactState) -> Self {
        Self {
            domains: state
                .domains
                .iter()
                .map(|(k, v)| (k.clone(), DomainView::from(v)))
                .collect(),
            repositories: state
                .repositories
                .iter()
                .map(|(k, v)| (k.clone(), RepositoryView::from(v)))
                .collect(),
        }
    }
}

impl From<CodeArtifactStateView> for CodeArtifactState {
    fn from(view: CodeArtifactStateView) -> Self {
        Self {
            domains: view
                .domains
                .into_iter()
                .map(|(k, v)| (k, Domain::from(v)))
                .collect(),
            repositories: view
                .repositories
                .into_iter()
                .map(|(k, v)| (k, Repository::from(v)))
                .collect(),
        }
    }
}

impl StatefulService for CodeArtifactService {
    type StateView = CodeArtifactStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CodeArtifactStateView::from(&*guard)
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
            *guard = CodeArtifactState::from(view);
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
            for (k, v) in view.domains {
                guard.domains.insert(k, Domain::from(v));
            }
            for (k, v) in view.repositories {
                guard.repositories.insert(k, Repository::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
