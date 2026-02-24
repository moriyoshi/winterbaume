use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CognitoSyncService;
use crate::state::{
    CognitoSyncState, DatasetRecord, DeviceRecord, IdentityPoolRecord, IdentityRecord, RecordEntry,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CognitoSyncStateView {
    #[serde(default)]
    pub pools: HashMap<String, IdentityPoolRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityPoolRecordView {
    #[serde(default)]
    pub events: HashMap<String, String>,
    #[serde(default)]
    pub configuration: Value,
    #[serde(default)]
    pub bulk_publish: Value,
    #[serde(default)]
    pub identities: HashMap<String, IdentityRecordView>,
    #[serde(default)]
    pub last_modified: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IdentityRecordView {
    #[serde(default)]
    pub datasets: HashMap<String, DatasetRecordView>,
    #[serde(default)]
    pub devices: HashMap<String, DeviceRecordView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DatasetRecordView {
    #[serde(default)]
    pub records: HashMap<String, RecordEntryView>,
    #[serde(default)]
    pub sync_count: i64,
    #[serde(default)]
    pub creation_date: f64,
    #[serde(default)]
    pub last_modified_date: f64,
    #[serde(default)]
    pub last_modified_by: String,
    #[serde(default)]
    pub data_storage: i64,
    #[serde(default)]
    pub subscriptions: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecordEntryView {
    pub key: String,
    pub value: Option<String>,
    #[serde(default)]
    pub sync_count: i64,
    #[serde(default)]
    pub last_modified_date: f64,
    #[serde(default)]
    pub last_modified_by: String,
    #[serde(default)]
    pub device_last_modified_date: Option<f64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceRecordView {
    pub device_id: String,
    pub platform: String,
    pub token: String,
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
    RecordEntryView,
    RecordEntry {
        key,
        value,
        sync_count,
        last_modified_date,
        last_modified_by,
        device_last_modified_date,
    }
);

basic_from!(
    DeviceRecordView,
    DeviceRecord {
        device_id,
        platform,
        token
    }
);

impl From<&DatasetRecord> for DatasetRecordView {
    fn from(s: &DatasetRecord) -> Self {
        Self {
            records: s
                .records
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            sync_count: s.sync_count,
            creation_date: s.creation_date,
            last_modified_date: s.last_modified_date,
            last_modified_by: s.last_modified_by.clone(),
            data_storage: s.data_storage,
            subscriptions: s.subscriptions.clone(),
        }
    }
}

impl From<DatasetRecordView> for DatasetRecord {
    fn from(v: DatasetRecordView) -> Self {
        Self {
            records: v.records.into_iter().map(|(k, r)| (k, r.into())).collect(),
            sync_count: v.sync_count,
            creation_date: v.creation_date,
            last_modified_date: v.last_modified_date,
            last_modified_by: v.last_modified_by,
            data_storage: v.data_storage,
            subscriptions: v.subscriptions,
        }
    }
}

impl From<&IdentityRecord> for IdentityRecordView {
    fn from(s: &IdentityRecord) -> Self {
        Self {
            datasets: s
                .datasets
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            devices: s
                .devices
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<IdentityRecordView> for IdentityRecord {
    fn from(v: IdentityRecordView) -> Self {
        Self {
            datasets: v.datasets.into_iter().map(|(k, d)| (k, d.into())).collect(),
            devices: v.devices.into_iter().map(|(k, d)| (k, d.into())).collect(),
        }
    }
}

impl From<&IdentityPoolRecord> for IdentityPoolRecordView {
    fn from(s: &IdentityPoolRecord) -> Self {
        Self {
            events: s.events.clone(),
            configuration: s.configuration.clone(),
            bulk_publish: s.bulk_publish.clone(),
            identities: s
                .identities
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            last_modified: s.last_modified,
        }
    }
}

impl From<IdentityPoolRecordView> for IdentityPoolRecord {
    fn from(v: IdentityPoolRecordView) -> Self {
        Self {
            events: v.events,
            configuration: v.configuration,
            bulk_publish: v.bulk_publish,
            identities: v
                .identities
                .into_iter()
                .map(|(k, i)| (k, i.into()))
                .collect(),
            last_modified: v.last_modified,
        }
    }
}

impl From<&CognitoSyncState> for CognitoSyncStateView {
    fn from(s: &CognitoSyncState) -> Self {
        Self {
            pools: s.pools.iter().map(|(k, v)| (k.clone(), v.into())).collect(),
        }
    }
}

impl From<CognitoSyncStateView> for CognitoSyncState {
    fn from(v: CognitoSyncStateView) -> Self {
        Self {
            pools: v.pools.into_iter().map(|(k, p)| (k, p.into())).collect(),
        }
    }
}

impl StatefulService for CognitoSyncService {
    type StateView = CognitoSyncStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        CognitoSyncStateView::from(&*guard)
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
            *guard = CognitoSyncState::from(view);
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
            for (k, v) in view.pools {
                guard.pools.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
