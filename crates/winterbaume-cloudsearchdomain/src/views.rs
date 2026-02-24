use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudSearchDomainService;
use crate::state::{CloudSearchDomainState, Document};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudSearchDomainStateView {
    #[serde(default)]
    pub documents: HashMap<String, DocumentView>,
    #[serde(default)]
    pub total_adds: i64,
    #[serde(default)]
    pub total_deletes: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentView {
    pub id: String,
    pub op: String,
    #[serde(default)]
    pub fields: HashMap<String, String>,
}

impl From<&Document> for DocumentView {
    fn from(d: &Document) -> Self {
        Self {
            id: d.id.clone(),
            op: d.op.clone(),
            fields: d.fields.clone(),
        }
    }
}

impl From<DocumentView> for Document {
    fn from(v: DocumentView) -> Self {
        Self {
            id: v.id,
            op: v.op,
            fields: v.fields,
        }
    }
}

impl From<&CloudSearchDomainState> for CloudSearchDomainStateView {
    fn from(state: &CloudSearchDomainState) -> Self {
        Self {
            documents: state
                .documents
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            total_adds: state.total_adds,
            total_deletes: state.total_deletes,
        }
    }
}

impl From<CloudSearchDomainStateView> for CloudSearchDomainState {
    fn from(view: CloudSearchDomainStateView) -> Self {
        Self {
            documents: view
                .documents
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            total_adds: view.total_adds,
            total_deletes: view.total_deletes,
        }
    }
}

impl StatefulService for CloudSearchDomainService {
    type StateView = CloudSearchDomainStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudSearchDomainStateView::from(&*guard)
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
            *guard = CloudSearchDomainState::from(view);
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
        let merged = CloudSearchDomainState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.documents {
                guard.documents.insert(k, v);
            }
            guard.total_adds += merged.total_adds;
            guard.total_deletes += merged.total_deletes;
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
