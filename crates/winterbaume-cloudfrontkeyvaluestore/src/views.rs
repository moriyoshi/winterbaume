use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudFrontKvsService;
use crate::state::{CloudFrontKvsState, KvsRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CloudFrontKvsStateView {
    #[serde(default)]
    pub stores: HashMap<String, KvsRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KvsRecordView {
    pub arn: String,
    pub created: f64,
    pub last_modified: f64,
    pub status: String,
    pub etag: String,
    #[serde(default)]
    pub items: HashMap<String, String>,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self { Self { $($field: s.$field.clone(),)* } }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self { Self { $($field: v.$field,)* } }
        }
    };
}

basic_from!(
    KvsRecordView,
    KvsRecord {
        arn,
        created,
        last_modified,
        status,
        etag,
        items,
    }
);

impl From<&CloudFrontKvsState> for CloudFrontKvsStateView {
    fn from(state: &CloudFrontKvsState) -> Self {
        Self {
            stores: state
                .stores
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<CloudFrontKvsStateView> for CloudFrontKvsState {
    fn from(view: CloudFrontKvsStateView) -> Self {
        Self {
            stores: view
                .stores
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for CloudFrontKvsService {
    type StateView = CloudFrontKvsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CloudFrontKvsStateView::from(&*guard)
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
            *guard = CloudFrontKvsState::from(view);
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
        let merged = CloudFrontKvsState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.stores {
                guard.stores.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
