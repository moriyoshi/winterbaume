use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3OutpostsService;
use crate::state::{EndpointRecord, OutpostRecord, S3OutpostsState};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct S3OutpostsStateView {
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointRecordView>,
    #[serde(default)]
    pub outposts: Vec<OutpostRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EndpointRecordView {
    pub arn: String,
    pub outpost_id: String,
    pub subnet_id: String,
    pub security_group_id: String,
    pub access_type: String,
    #[serde(default)]
    pub customer_owned_ipv4_pool: Option<String>,
    pub vpc_id: String,
    pub cidr_block: String,
    pub creation_time: f64,
    pub status: String,
    #[serde(default)]
    pub network_interface_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct OutpostRecordView {
    pub outpost_id: String,
    pub outpost_arn: String,
    pub s3_outpost_arn: String,
    pub owner_id: String,
    pub capacity_in_bytes: i64,
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
    EndpointRecordView,
    EndpointRecord {
        arn,
        outpost_id,
        subnet_id,
        security_group_id,
        access_type,
        customer_owned_ipv4_pool,
        vpc_id,
        cidr_block,
        creation_time,
        status,
        network_interface_ids,
    }
);

basic_from!(
    OutpostRecordView,
    OutpostRecord {
        outpost_id,
        outpost_arn,
        s3_outpost_arn,
        owner_id,
        capacity_in_bytes,
    }
);

impl From<&S3OutpostsState> for S3OutpostsStateView {
    fn from(state: &S3OutpostsState) -> Self {
        Self {
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            outposts: state.outposts.iter().map(|o| o.into()).collect(),
        }
    }
}

impl From<S3OutpostsStateView> for S3OutpostsState {
    fn from(view: S3OutpostsStateView) -> Self {
        Self {
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            outposts: view.outposts.into_iter().map(|o| o.into()).collect(),
        }
    }
}

impl StatefulService for S3OutpostsService {
    type StateView = S3OutpostsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        S3OutpostsStateView::from(&*guard)
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
            *guard = S3OutpostsState::from(view);
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
        let merged = S3OutpostsState::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.endpoints {
                guard.endpoints.insert(k, v);
            }
            for o in merged.outposts {
                if !guard.outposts.iter().any(|x| x.outpost_id == o.outpost_id) {
                    guard.outposts.push(o);
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
