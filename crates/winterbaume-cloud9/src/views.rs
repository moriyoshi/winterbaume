use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Cloud9Service;
use crate::state::{Cloud9State, EnvironmentRecord, MembershipRecord};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Cloud9StateView {
    #[serde(default)]
    pub environments: HashMap<String, EnvironmentRecordView>,
    /// Memberships keyed by "{environment_id}/{user_arn}".
    #[serde(default)]
    pub memberships: HashMap<String, MembershipRecordView>,
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EnvironmentRecordView {
    pub id: String,
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub env_type: String,
    #[serde(default)]
    pub connection_type: Option<String>,
    pub owner_arn: String,
    #[serde(default)]
    pub instance_type: Option<String>,
    #[serde(default)]
    pub image_id: Option<String>,
    pub managed_credentials_status: String,
    pub status: String,
    #[serde(default)]
    pub status_reason: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MembershipRecordView {
    pub environment_id: String,
    pub user_arn: String,
    pub user_id: String,
    pub permissions: String,
    #[serde(default)]
    pub last_access: Option<f64>,
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
    EnvironmentRecordView,
    EnvironmentRecord {
        id,
        arn,
        name,
        description,
        env_type,
        connection_type,
        owner_arn,
        instance_type,
        image_id,
        managed_credentials_status,
        status,
        status_reason,
    }
);

basic_from!(
    MembershipRecordView,
    MembershipRecord {
        environment_id,
        user_arn,
        user_id,
        permissions,
        last_access,
    }
);

impl From<&Cloud9State> for Cloud9StateView {
    fn from(state: &Cloud9State) -> Self {
        Self {
            environments: state
                .environments
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            memberships: state
                .memberships
                .iter()
                .map(|((e, u), v)| (format!("{e}/{u}"), v.into()))
                .collect(),
            tags: state.tags.clone(),
        }
    }
}

impl From<Cloud9StateView> for Cloud9State {
    fn from(view: Cloud9StateView) -> Self {
        let memberships = view
            .memberships
            .into_values()
            .map(|v| {
                let m: MembershipRecord = v.into();
                ((m.environment_id.clone(), m.user_arn.clone()), m)
            })
            .collect();
        Self {
            environments: view
                .environments
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            memberships,
            tags: view.tags,
        }
    }
}

impl StatefulService for Cloud9Service {
    type StateView = Cloud9StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Cloud9StateView::from(&*guard)
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
            *guard = Cloud9State::from(view);
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
        let merged = Cloud9State::from(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (k, v) in merged.environments {
                guard.environments.insert(k, v);
            }
            for (k, v) in merged.memberships {
                guard.memberships.insert(k, v);
            }
            for (k, v) in merged.tags {
                guard.tags.insert(k, v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
