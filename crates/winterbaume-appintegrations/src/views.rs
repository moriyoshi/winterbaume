//! Serde-compatible view types for AppIntegrations state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppIntegrationsService;
use crate::state::AppIntegrationsState;
use crate::types::{
    Application, ApplicationAssociation, DataIntegration, DataIntegrationAssociation,
    EventIntegration, EventIntegrationAssociationRecord,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppIntegrationsStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    #[serde(default)]
    pub data_integrations: HashMap<String, DataIntegrationView>,
    #[serde(default)]
    pub data_integration_associations: HashMap<String, DataIntegrationAssociationView>,
    #[serde(default)]
    pub event_integrations: HashMap<String, EventIntegrationView>,
    #[serde(default)]
    pub event_integration_associations: HashMap<String, EventIntegrationAssociationView>,
    #[serde(default)]
    pub application_associations: HashMap<String, ApplicationAssociationView>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub application_type: Option<String>,
    #[serde(default)]
    pub client_token: Option<String>,
    #[serde(default)]
    pub initialization_timeout: Option<i32>,
    #[serde(default)]
    pub is_service: Option<bool>,
    #[serde(default)]
    pub permissions: Vec<String>,
    #[serde(default)]
    pub publications: Vec<Value>,
    #[serde(default)]
    pub subscriptions: Vec<Value>,
    #[serde(default)]
    pub application_config: Option<Value>,
    #[serde(default)]
    pub application_source_config: Option<Value>,
    #[serde(default)]
    pub iframe_config: Option<Value>,
    pub created_time: i64,
    pub last_modified_time: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataIntegrationView {
    pub id: String,
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub kms_key: String,
    #[serde(default)]
    pub source_uri: Option<String>,
    #[serde(default)]
    pub file_configuration: Option<Value>,
    #[serde(default)]
    pub object_configuration: Option<Value>,
    #[serde(default)]
    pub schedule_config: Option<Value>,
    #[serde(default)]
    pub client_token: Option<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DataIntegrationAssociationView {
    pub id: String,
    pub arn: String,
    pub data_integration_id: String,
    pub data_integration_arn: String,
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub destination_uri: Option<String>,
    #[serde(default)]
    pub last_execution_status: Option<String>,
    #[serde(default)]
    pub execution_configuration: Option<Value>,
    #[serde(default)]
    pub client_association_metadata: HashMap<String, String>,
    pub created_time: i64,
    pub last_modified_time: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventIntegrationView {
    pub arn: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub event_filter_source: String,
    pub event_bridge_bus: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventIntegrationAssociationView {
    pub event_integration_association_arn: String,
    pub event_integration_association_id: String,
    pub event_integration_name: String,
    pub client_id: String,
    pub event_bridge_rule_name: String,
    #[serde(default)]
    pub client_association_metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApplicationAssociationView {
    pub application_association_arn: String,
    pub application_arn: String,
    pub application_id: String,
    pub client_id: String,
}

macro_rules! basic_from {
    ($view:ident, $domain:ident { $($field:ident),* $(,)? }) => {
        impl From<&$domain> for $view {
            fn from(s: &$domain) -> Self {
                Self {
                    $($field: s.$field.clone(),)*
                }
            }
        }
        impl From<$view> for $domain {
            fn from(v: $view) -> Self {
                Self {
                    $($field: v.$field,)*
                }
            }
        }
    };
}

basic_from!(
    ApplicationView,
    Application {
        id,
        arn,
        name,
        namespace,
        description,
        application_type,
        client_token,
        initialization_timeout,
        is_service,
        permissions,
        publications,
        subscriptions,
        application_config,
        application_source_config,
        iframe_config,
        created_time,
        last_modified_time,
        tags
    }
);

basic_from!(
    DataIntegrationView,
    DataIntegration {
        id,
        arn,
        name,
        description,
        kms_key,
        source_uri,
        file_configuration,
        object_configuration,
        schedule_config,
        client_token,
        tags
    }
);

basic_from!(
    DataIntegrationAssociationView,
    DataIntegrationAssociation {
        id,
        arn,
        data_integration_id,
        data_integration_arn,
        client_id,
        destination_uri,
        last_execution_status,
        execution_configuration,
        client_association_metadata,
        created_time,
        last_modified_time
    }
);

basic_from!(
    EventIntegrationView,
    EventIntegration {
        arn,
        name,
        description,
        event_filter_source,
        event_bridge_bus,
        tags
    }
);

basic_from!(
    EventIntegrationAssociationView,
    EventIntegrationAssociationRecord {
        event_integration_association_arn,
        event_integration_association_id,
        event_integration_name,
        client_id,
        event_bridge_rule_name,
        client_association_metadata
    }
);

basic_from!(
    ApplicationAssociationView,
    ApplicationAssociation {
        application_association_arn,
        application_arn,
        application_id,
        client_id
    }
);

impl From<&AppIntegrationsState> for AppIntegrationsStateView {
    fn from(state: &AppIntegrationsState) -> Self {
        Self {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            data_integrations: state
                .data_integrations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            data_integration_associations: state
                .data_integration_associations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            event_integrations: state
                .event_integrations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            event_integration_associations: state
                .event_integration_associations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
            application_associations: state
                .application_associations
                .iter()
                .map(|(k, v)| (k.clone(), v.into()))
                .collect(),
        }
    }
}

impl From<AppIntegrationsStateView> for AppIntegrationsState {
    fn from(view: AppIntegrationsStateView) -> Self {
        Self {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            data_integrations: view
                .data_integrations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            data_integration_associations: view
                .data_integration_associations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            event_integrations: view
                .event_integrations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            event_integration_associations: view
                .event_integration_associations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
            application_associations: view
                .application_associations
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl StatefulService for AppIntegrationsService {
    type StateView = AppIntegrationsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppIntegrationsStateView::from(&*guard)
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
            *guard = AppIntegrationsState::from(view);
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
            for (k, v) in view.applications {
                guard.applications.insert(k, v.into());
            }
            for (k, v) in view.data_integrations {
                guard.data_integrations.insert(k, v.into());
            }
            for (k, v) in view.data_integration_associations {
                guard.data_integration_associations.insert(k, v.into());
            }
            for (k, v) in view.event_integrations {
                guard.event_integrations.insert(k, v.into());
            }
            for (k, v) in view.event_integration_associations {
                guard.event_integration_associations.insert(k, v.into());
            }
            for (k, v) in view.application_associations {
                guard.application_associations.insert(k, v.into());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
