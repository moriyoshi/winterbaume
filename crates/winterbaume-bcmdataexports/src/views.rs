use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::BcmDataExportsService;
use crate::state::{BcmDataExportsState, TableCatalogueEntry, TableColumn};
use crate::types::{Execution, Export};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BcmDataExportsStateView {
    #[serde(default)]
    pub exports: HashMap<String, ExportView>,
    #[serde(default)]
    pub executions: HashMap<String, ExecutionView>,
    #[serde(default)]
    pub tables: HashMap<String, TableCatalogueEntryView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExportView {
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub data_query: Value,
    #[serde(default)]
    pub destination_configurations: Value,
    #[serde(default)]
    pub refresh_cadence: Value,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub last_refreshed_at: Option<String>,
    pub status_code: String,
    #[serde(default)]
    pub status_reason: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ExecutionView {
    pub id: String,
    pub export_arn: String,
    pub status_code: String,
    #[serde(default)]
    pub status_reason: Option<String>,
    pub created_at: String,
    pub last_updated_at: String,
    #[serde(default)]
    pub completed_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableCatalogueEntryView {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub schema: Vec<TableColumnView>,
    #[serde(default)]
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TableColumnView {
    pub name: String,
    pub r#type: String,
    #[serde(default)]
    pub description: Option<String>,
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
    ExportView,
    Export {
        arn,
        name,
        description,
        data_query,
        destination_configurations,
        refresh_cadence,
        created_at,
        last_updated_at,
        last_refreshed_at,
        status_code,
        status_reason,
        tags
    }
);

basic_from!(
    ExecutionView,
    Execution {
        id,
        export_arn,
        status_code,
        status_reason,
        created_at,
        last_updated_at,
        completed_at
    }
);

basic_from!(
    TableColumnView,
    TableColumn {
        name,
        r#type,
        description
    }
);

impl From<&TableCatalogueEntry> for TableCatalogueEntryView {
    fn from(t: &TableCatalogueEntry) -> Self {
        Self {
            name: t.name.clone(),
            description: t.description.clone(),
            schema: t.schema.iter().map(Into::into).collect(),
            properties: t.properties.clone(),
        }
    }
}

impl From<TableCatalogueEntryView> for TableCatalogueEntry {
    fn from(v: TableCatalogueEntryView) -> Self {
        Self {
            name: v.name,
            description: v.description,
            schema: v.schema.into_iter().map(Into::into).collect(),
            properties: v.properties,
        }
    }
}

impl From<&BcmDataExportsState> for BcmDataExportsStateView {
    fn from(state: &BcmDataExportsState) -> Self {
        Self {
            exports: state
                .exports
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            executions: state
                .executions
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            tables: state
                .tables
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<BcmDataExportsStateView> for BcmDataExportsState {
    fn from(view: BcmDataExportsStateView) -> Self {
        Self {
            exports: view
                .exports
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            executions: view
                .executions
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            tables: view
                .tables
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for BcmDataExportsService {
    type StateView = BcmDataExportsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        BcmDataExportsStateView::from(&*guard)
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
            *guard = BcmDataExportsState::from(view);
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
            for (k, v) in view.exports {
                guard.exports.insert(k, v.into());
            }
            for (k, v) in view.executions {
                guard.executions.insert(k, v.into());
            }
            for (k, v) in view.tables {
                guard.tables.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
