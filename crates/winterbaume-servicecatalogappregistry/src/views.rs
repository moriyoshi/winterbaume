//! Serde-compatible view types for ServiceCatalogAppRegistry state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ServiceCatalogAppRegistryService;
use crate::state::ServiceCatalogAppRegistryState;
use crate::types::{AppRegistryConfig, Application, AttributeGroup, ResourceAssociation};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceCatalogAppRegistryStateView {
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    #[serde(default)]
    pub application_names: HashMap<String, String>,
    #[serde(default)]
    pub attribute_groups: HashMap<String, AttributeGroupView>,
    #[serde(default)]
    pub attribute_group_names: HashMap<String, String>,
    #[serde(default)]
    pub app_to_attr_groups: HashMap<String, Vec<String>>,
    #[serde(default)]
    pub app_to_resources: HashMap<String, Vec<ResourceAssociationView>>,
    #[serde(default)]
    pub configuration: AppRegistryConfigView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttributeGroupView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub attributes: String,
    pub creation_time: DateTime<Utc>,
    pub last_update_time: DateTime<Utc>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAssociationView {
    pub application_id: String,
    pub resource_type: String,
    pub resource: String,
    pub resource_arn: Option<String>,
    #[serde(default)]
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppRegistryConfigView {
    pub tag_key: Option<String>,
}

// ---- From impls ----

impl From<&ServiceCatalogAppRegistryState> for ServiceCatalogAppRegistryStateView {
    fn from(state: &ServiceCatalogAppRegistryState) -> Self {
        ServiceCatalogAppRegistryStateView {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), ApplicationView::from(v)))
                .collect(),
            application_names: state.application_names.clone(),
            attribute_groups: state
                .attribute_groups
                .iter()
                .map(|(k, v)| (k.clone(), AttributeGroupView::from(v)))
                .collect(),
            attribute_group_names: state.attribute_group_names.clone(),
            app_to_attr_groups: state.app_to_attr_groups.clone(),
            app_to_resources: state
                .app_to_resources
                .iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        v.iter().map(ResourceAssociationView::from).collect(),
                    )
                })
                .collect(),
            configuration: AppRegistryConfigView {
                tag_key: state.configuration.tag_key.clone(),
            },
        }
    }
}

impl From<&Application> for ApplicationView {
    fn from(a: &Application) -> Self {
        ApplicationView {
            id: a.id.clone(),
            arn: a.arn.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            creation_time: a.creation_time,
            last_update_time: a.last_update_time,
            tags: a.tags.clone(),
        }
    }
}

impl From<&AttributeGroup> for AttributeGroupView {
    fn from(a: &AttributeGroup) -> Self {
        AttributeGroupView {
            id: a.id.clone(),
            arn: a.arn.clone(),
            name: a.name.clone(),
            description: a.description.clone(),
            attributes: a.attributes.clone(),
            creation_time: a.creation_time,
            last_update_time: a.last_update_time,
            tags: a.tags.clone(),
        }
    }
}

impl From<&ResourceAssociation> for ResourceAssociationView {
    fn from(r: &ResourceAssociation) -> Self {
        ResourceAssociationView {
            application_id: r.application_id.clone(),
            resource_type: r.resource_type.clone(),
            resource: r.resource.clone(),
            resource_arn: r.resource_arn.clone(),
            options: r.options.clone(),
        }
    }
}

impl From<ServiceCatalogAppRegistryStateView> for ServiceCatalogAppRegistryState {
    fn from(view: ServiceCatalogAppRegistryStateView) -> Self {
        ServiceCatalogAppRegistryState {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| (k, Application::from(v)))
                .collect(),
            application_names: view.application_names,
            attribute_groups: view
                .attribute_groups
                .into_iter()
                .map(|(k, v)| (k, AttributeGroup::from(v)))
                .collect(),
            attribute_group_names: view.attribute_group_names,
            app_to_attr_groups: view.app_to_attr_groups,
            app_to_resources: view
                .app_to_resources
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(ResourceAssociation::from).collect()))
                .collect(),
            configuration: AppRegistryConfig {
                tag_key: view.configuration.tag_key,
            },
        }
    }
}

impl From<ApplicationView> for Application {
    fn from(v: ApplicationView) -> Self {
        Application {
            id: v.id,
            arn: v.arn,
            name: v.name,
            description: v.description,
            creation_time: v.creation_time,
            last_update_time: v.last_update_time,
            tags: v.tags,
        }
    }
}

impl From<AttributeGroupView> for AttributeGroup {
    fn from(v: AttributeGroupView) -> Self {
        AttributeGroup {
            id: v.id,
            arn: v.arn,
            name: v.name,
            description: v.description,
            attributes: v.attributes,
            creation_time: v.creation_time,
            last_update_time: v.last_update_time,
            tags: v.tags,
        }
    }
}

impl From<ResourceAssociationView> for ResourceAssociation {
    fn from(v: ResourceAssociationView) -> Self {
        ResourceAssociation {
            application_id: v.application_id,
            resource_type: v.resource_type,
            resource: v.resource,
            resource_arn: v.resource_arn,
            options: v.options,
        }
    }
}

impl StatefulService for ServiceCatalogAppRegistryService {
    type StateView = ServiceCatalogAppRegistryStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        ServiceCatalogAppRegistryStateView::from(&*guard)
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
            *guard = ServiceCatalogAppRegistryState::from(view);
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
                let name = v.name.clone();
                let id = v.id.clone();
                guard.applications.insert(k, Application::from(v));
                guard.application_names.insert(name, id);
            }
            for (k, v) in view.attribute_groups {
                let name = v.name.clone();
                let id = v.id.clone();
                guard.attribute_groups.insert(k, AttributeGroup::from(v));
                guard.attribute_group_names.insert(name, id);
            }
            for (k, v) in view.app_to_attr_groups {
                guard.app_to_attr_groups.insert(k, v);
            }
            for (k, v) in view.app_to_resources {
                guard
                    .app_to_resources
                    .insert(k, v.into_iter().map(ResourceAssociation::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
