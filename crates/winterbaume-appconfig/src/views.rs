//! Serde-compatible view types for AppConfig state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::AppConfigService;
use crate::state::AppConfigState;
use crate::types::{
    AccountSettingsData, ActionData, Application, ConfigurationProfileData, DeploymentData,
    DeploymentStrategyData, EnvironmentData, ExtensionAssociationData, ExtensionData,
    HostedConfigurationVersionData, MonitorData, ParameterData,
};

/// Serializable view of the entire AppConfig state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppconfigStateView {
    /// Applications keyed by application ID.
    #[serde(default)]
    pub applications: HashMap<String, ApplicationView>,
    /// Configuration profiles keyed by (application_id, profile_id).
    #[serde(default)]
    pub configuration_profiles: Vec<ConfigurationProfileView>,
    /// Hosted configuration versions.
    #[serde(default)]
    pub hosted_configuration_versions: Vec<HostedConfigurationVersionView>,
    /// Next version number per (application_id, profile_id), stored as serializable pairs.
    #[serde(default)]
    pub next_version: Vec<((String, String), i32)>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Deployment strategies keyed by ID.
    #[serde(default)]
    pub deployment_strategies: HashMap<String, DeploymentStrategyView>,
    /// Environments keyed by "appId/envId".
    #[serde(default)]
    pub environments: HashMap<String, EnvironmentView>,
    /// Deployments keyed by "appId/envId/depNum".
    #[serde(default)]
    pub deployments: HashMap<String, DeploymentView>,
    /// Next deployment number per (application_id, environment_id).
    #[serde(default)]
    pub next_deployment_number: Vec<((String, String), i32)>,
    /// Extensions keyed by ID.
    #[serde(default)]
    pub extensions: HashMap<String, ExtensionView>,
    /// Extension associations keyed by ID.
    #[serde(default)]
    pub extension_associations: HashMap<String, ExtensionAssociationView>,
    /// Account settings.
    #[serde(default)]
    pub account_settings: AccountSettingsView,
}

/// Serializable view of an AppConfig application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationView {
    pub id: String,
    pub name: String,
    pub description: String,
}

/// Serializable view of a configuration profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigurationProfileView {
    pub id: String,
    pub application_id: String,
    pub name: String,
    pub description: String,
    pub location_uri: String,
    pub r#type: String,
    pub retrieval_role_arn: Option<String>,
}

/// Serializable view of a hosted configuration version.
/// Configuration content is excluded from snapshots; restored as empty bytes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedConfigurationVersionView {
    pub application_id: String,
    pub configuration_profile_id: String,
    pub version_number: i32,
    pub content_type: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentStrategyView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub deployment_duration_in_minutes: i32,
    pub final_bake_time_in_minutes: i32,
    pub growth_factor: f32,
    pub growth_type: String,
    pub replicate_to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorView {
    pub alarm_arn: String,
    pub alarm_role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentView {
    pub id: String,
    pub application_id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub monitors: Vec<MonitorView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentView {
    pub deployment_number: i32,
    pub application_id: String,
    pub environment_id: String,
    pub deployment_strategy_id: String,
    pub configuration_profile_id: String,
    pub configuration_version: String,
    pub state: String,
    pub started_at: String,
    pub completed_at: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionView {
    pub name: Option<String>,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterView {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub dynamic: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionView {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version_number: i32,
    pub arn: String,
    pub actions: HashMap<String, Vec<ActionView>>,
    pub parameters: HashMap<String, ParameterView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionAssociationView {
    pub id: String,
    pub arn: String,
    pub extension_arn: String,
    pub resource_arn: String,
    pub extension_version_number: i32,
    pub parameters: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AccountSettingsView {
    pub deletion_protection_enabled: Option<bool>,
    pub deletion_protection_period_in_minutes: Option<i32>,
}

// --- From internal types to view types ---

impl From<&AppConfigState> for AppconfigStateView {
    fn from(state: &AppConfigState) -> Self {
        AppconfigStateView {
            applications: state
                .applications
                .iter()
                .map(|(k, v)| (k.clone(), ApplicationView::from(v)))
                .collect(),
            configuration_profiles: state
                .configuration_profiles
                .values()
                .map(ConfigurationProfileView::from)
                .collect(),
            hosted_configuration_versions: state
                .hosted_configuration_versions
                .values()
                .map(HostedConfigurationVersionView::from)
                .collect(),
            next_version: state
                .next_version
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect(),
            tags: state.tags.clone(),
            deployment_strategies: state
                .deployment_strategies
                .iter()
                .map(|(k, v)| (k.clone(), DeploymentStrategyView::from(v)))
                .collect(),
            environments: state
                .environments
                .iter()
                .map(|(k, v)| (k.clone(), EnvironmentView::from(v)))
                .collect(),
            deployments: state
                .deployments
                .iter()
                .map(|(k, v)| (k.clone(), DeploymentView::from(v)))
                .collect(),
            next_deployment_number: state
                .next_deployment_number
                .iter()
                .map(|(k, v)| (k.clone(), *v))
                .collect(),
            extensions: state
                .extensions
                .iter()
                .map(|(k, v)| (k.clone(), ExtensionView::from(v)))
                .collect(),
            extension_associations: state
                .extension_associations
                .iter()
                .map(|(k, v)| (k.clone(), ExtensionAssociationView::from(v)))
                .collect(),
            account_settings: AccountSettingsView::from(&state.account_settings),
        }
    }
}

impl From<&Application> for ApplicationView {
    fn from(app: &Application) -> Self {
        ApplicationView {
            id: app.id.clone(),
            name: app.name.clone(),
            description: app.description.clone(),
        }
    }
}

impl From<&ConfigurationProfileData> for ConfigurationProfileView {
    fn from(p: &ConfigurationProfileData) -> Self {
        ConfigurationProfileView {
            id: p.id.clone(),
            application_id: p.application_id.clone(),
            name: p.name.clone(),
            description: p.description.clone(),
            location_uri: p.location_uri.clone(),
            r#type: p.r#type.clone(),
            retrieval_role_arn: p.retrieval_role_arn.clone(),
        }
    }
}

impl From<&HostedConfigurationVersionData> for HostedConfigurationVersionView {
    fn from(v: &HostedConfigurationVersionData) -> Self {
        HostedConfigurationVersionView {
            application_id: v.application_id.clone(),
            configuration_profile_id: v.configuration_profile_id.clone(),
            version_number: v.version_number,
            content_type: v.content_type.clone(),
            description: v.description.clone(),
        }
    }
}

impl From<&DeploymentStrategyData> for DeploymentStrategyView {
    fn from(d: &DeploymentStrategyData) -> Self {
        DeploymentStrategyView {
            id: d.id.clone(),
            name: d.name.clone(),
            description: d.description.clone(),
            deployment_duration_in_minutes: d.deployment_duration_in_minutes,
            final_bake_time_in_minutes: d.final_bake_time_in_minutes,
            growth_factor: d.growth_factor,
            growth_type: d.growth_type.clone(),
            replicate_to: d.replicate_to.clone(),
        }
    }
}

impl From<&MonitorData> for MonitorView {
    fn from(m: &MonitorData) -> Self {
        MonitorView {
            alarm_arn: m.alarm_arn.clone(),
            alarm_role_arn: m.alarm_role_arn.clone(),
        }
    }
}

impl From<&EnvironmentData> for EnvironmentView {
    fn from(e: &EnvironmentData) -> Self {
        EnvironmentView {
            id: e.id.clone(),
            application_id: e.application_id.clone(),
            name: e.name.clone(),
            description: e.description.clone(),
            state: e.state.clone(),
            monitors: e.monitors.iter().map(MonitorView::from).collect(),
        }
    }
}

impl From<&DeploymentData> for DeploymentView {
    fn from(d: &DeploymentData) -> Self {
        DeploymentView {
            deployment_number: d.deployment_number,
            application_id: d.application_id.clone(),
            environment_id: d.environment_id.clone(),
            deployment_strategy_id: d.deployment_strategy_id.clone(),
            configuration_profile_id: d.configuration_profile_id.clone(),
            configuration_version: d.configuration_version.clone(),
            state: d.state.clone(),
            started_at: d.started_at.clone(),
            completed_at: d.completed_at.clone(),
            description: d.description.clone(),
        }
    }
}

impl From<&ActionData> for ActionView {
    fn from(a: &ActionData) -> Self {
        ActionView {
            name: a.name.clone(),
            description: a.description.clone(),
            uri: a.uri.clone(),
            role_arn: a.role_arn.clone(),
        }
    }
}

impl From<&ParameterData> for ParameterView {
    fn from(p: &ParameterData) -> Self {
        ParameterView {
            description: p.description.clone(),
            required: p.required,
            dynamic: p.dynamic,
        }
    }
}

impl From<&ExtensionData> for ExtensionView {
    fn from(e: &ExtensionData) -> Self {
        ExtensionView {
            id: e.id.clone(),
            name: e.name.clone(),
            description: e.description.clone(),
            version_number: e.version_number,
            arn: e.arn.clone(),
            actions: e
                .actions
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(ActionView::from).collect()))
                .collect(),
            parameters: e
                .parameters
                .iter()
                .map(|(k, v)| (k.clone(), ParameterView::from(v)))
                .collect(),
        }
    }
}

impl From<&ExtensionAssociationData> for ExtensionAssociationView {
    fn from(a: &ExtensionAssociationData) -> Self {
        ExtensionAssociationView {
            id: a.id.clone(),
            arn: a.arn.clone(),
            extension_arn: a.extension_arn.clone(),
            resource_arn: a.resource_arn.clone(),
            extension_version_number: a.extension_version_number,
            parameters: a.parameters.clone(),
        }
    }
}

impl From<&AccountSettingsData> for AccountSettingsView {
    fn from(a: &AccountSettingsData) -> Self {
        AccountSettingsView {
            deletion_protection_enabled: a.deletion_protection_enabled,
            deletion_protection_period_in_minutes: a.deletion_protection_period_in_minutes,
        }
    }
}

// --- From view types to internal types ---

impl From<AppconfigStateView> for AppConfigState {
    fn from(view: AppconfigStateView) -> Self {
        AppConfigState {
            applications: view
                .applications
                .into_iter()
                .map(|(k, v)| (k, Application::from(v)))
                .collect(),
            configuration_profiles: view
                .configuration_profiles
                .into_iter()
                .map(|v| {
                    let key = (v.application_id.clone(), v.id.clone());
                    (key, ConfigurationProfileData::from(v))
                })
                .collect(),
            hosted_configuration_versions: view
                .hosted_configuration_versions
                .into_iter()
                .map(|v| {
                    let key = (
                        v.application_id.clone(),
                        v.configuration_profile_id.clone(),
                        v.version_number,
                    );
                    (key, HostedConfigurationVersionData::from(v))
                })
                .collect(),
            next_version: view.next_version.into_iter().collect(),
            tags: view.tags,
            deployment_strategies: view
                .deployment_strategies
                .into_iter()
                .map(|(k, v)| (k, DeploymentStrategyData::from(v)))
                .collect(),
            environments: view
                .environments
                .into_iter()
                .map(|(k, v)| (k, EnvironmentData::from(v)))
                .collect(),
            deployments: view
                .deployments
                .into_iter()
                .map(|(k, v)| (k, DeploymentData::from(v)))
                .collect(),
            next_deployment_number: view.next_deployment_number.into_iter().collect(),
            extensions: view
                .extensions
                .into_iter()
                .map(|(k, v)| (k, ExtensionData::from(v)))
                .collect(),
            extension_associations: view
                .extension_associations
                .into_iter()
                .map(|(k, v)| (k, ExtensionAssociationData::from(v)))
                .collect(),
            account_settings: AccountSettingsData::from(view.account_settings),
        }
    }
}

impl From<ApplicationView> for Application {
    fn from(v: ApplicationView) -> Self {
        Application {
            id: v.id,
            name: v.name,
            description: v.description,
        }
    }
}

impl From<ConfigurationProfileView> for ConfigurationProfileData {
    fn from(v: ConfigurationProfileView) -> Self {
        ConfigurationProfileData {
            id: v.id,
            application_id: v.application_id,
            name: v.name,
            description: v.description,
            location_uri: v.location_uri,
            r#type: v.r#type,
            retrieval_role_arn: v.retrieval_role_arn,
        }
    }
}

impl From<HostedConfigurationVersionView> for HostedConfigurationVersionData {
    fn from(v: HostedConfigurationVersionView) -> Self {
        HostedConfigurationVersionData {
            application_id: v.application_id,
            configuration_profile_id: v.configuration_profile_id,
            version_number: v.version_number,
            content_type: v.content_type,
            description: v.description,
            // Content is excluded from snapshots ( see `HostedConfigurationVersionView`
            // doc comment ); restored versions re-materialise with empty bytes.
            content: Vec::new(),
        }
    }
}

impl From<DeploymentStrategyView> for DeploymentStrategyData {
    fn from(v: DeploymentStrategyView) -> Self {
        DeploymentStrategyData {
            id: v.id,
            name: v.name,
            description: v.description,
            deployment_duration_in_minutes: v.deployment_duration_in_minutes,
            final_bake_time_in_minutes: v.final_bake_time_in_minutes,
            growth_factor: v.growth_factor,
            growth_type: v.growth_type,
            replicate_to: v.replicate_to,
        }
    }
}

impl From<MonitorView> for MonitorData {
    fn from(v: MonitorView) -> Self {
        MonitorData {
            alarm_arn: v.alarm_arn,
            alarm_role_arn: v.alarm_role_arn,
        }
    }
}

impl From<EnvironmentView> for EnvironmentData {
    fn from(v: EnvironmentView) -> Self {
        EnvironmentData {
            id: v.id,
            application_id: v.application_id,
            name: v.name,
            description: v.description,
            state: v.state,
            monitors: v.monitors.into_iter().map(MonitorData::from).collect(),
        }
    }
}

impl From<DeploymentView> for DeploymentData {
    fn from(v: DeploymentView) -> Self {
        DeploymentData {
            deployment_number: v.deployment_number,
            application_id: v.application_id,
            environment_id: v.environment_id,
            deployment_strategy_id: v.deployment_strategy_id,
            configuration_profile_id: v.configuration_profile_id,
            configuration_version: v.configuration_version,
            state: v.state,
            started_at: v.started_at,
            completed_at: v.completed_at,
            description: v.description,
        }
    }
}

impl From<ActionView> for ActionData {
    fn from(v: ActionView) -> Self {
        ActionData {
            name: v.name,
            description: v.description,
            uri: v.uri,
            role_arn: v.role_arn,
        }
    }
}

impl From<ParameterView> for ParameterData {
    fn from(v: ParameterView) -> Self {
        ParameterData {
            description: v.description,
            required: v.required,
            dynamic: v.dynamic,
        }
    }
}

impl From<ExtensionView> for ExtensionData {
    fn from(v: ExtensionView) -> Self {
        ExtensionData {
            id: v.id,
            name: v.name,
            description: v.description,
            version_number: v.version_number,
            arn: v.arn,
            actions: v
                .actions
                .into_iter()
                .map(|(k, acts)| (k, acts.into_iter().map(ActionData::from).collect()))
                .collect(),
            parameters: v
                .parameters
                .into_iter()
                .map(|(k, p)| (k, ParameterData::from(p)))
                .collect(),
        }
    }
}

impl From<ExtensionAssociationView> for ExtensionAssociationData {
    fn from(v: ExtensionAssociationView) -> Self {
        ExtensionAssociationData {
            id: v.id,
            arn: v.arn,
            extension_arn: v.extension_arn,
            resource_arn: v.resource_arn,
            extension_version_number: v.extension_version_number,
            parameters: v.parameters,
        }
    }
}

impl From<AccountSettingsView> for AccountSettingsData {
    fn from(v: AccountSettingsView) -> Self {
        AccountSettingsData {
            deletion_protection_enabled: v.deletion_protection_enabled,
            deletion_protection_period_in_minutes: v.deletion_protection_period_in_minutes,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for AppConfigService {
    type StateView = AppconfigStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        AppconfigStateView::from(&*guard)
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
            *guard = AppConfigState::from(view);
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
            for (id, app_view) in view.applications {
                guard.applications.insert(id, Application::from(app_view));
            }
            for profile_view in view.configuration_profiles {
                let key = (profile_view.application_id.clone(), profile_view.id.clone());
                guard
                    .configuration_profiles
                    .insert(key, ConfigurationProfileData::from(profile_view));
            }
            for version_view in view.hosted_configuration_versions {
                let key = (
                    version_view.application_id.clone(),
                    version_view.configuration_profile_id.clone(),
                    version_view.version_number,
                );
                guard
                    .hosted_configuration_versions
                    .insert(key, HostedConfigurationVersionData::from(version_view));
            }
            for (k, v) in view.next_version {
                guard.next_version.insert(k, v);
            }
            for (arn, tags) in view.tags {
                guard.tags.entry(arn).or_default().extend(tags);
            }
            for (id, ds_view) in view.deployment_strategies {
                guard
                    .deployment_strategies
                    .insert(id, DeploymentStrategyData::from(ds_view));
            }
            for (k, env_view) in view.environments {
                guard
                    .environments
                    .insert(k, EnvironmentData::from(env_view));
            }
            for (k, dep_view) in view.deployments {
                guard.deployments.insert(k, DeploymentData::from(dep_view));
            }
            for (k, v) in view.next_deployment_number {
                guard.next_deployment_number.insert(k, v);
            }
            for (id, ext_view) in view.extensions {
                guard.extensions.insert(id, ExtensionData::from(ext_view));
            }
            for (id, assoc_view) in view.extension_associations {
                guard
                    .extension_associations
                    .insert(id, ExtensionAssociationData::from(assoc_view));
            }
            if let Some(v) = view.account_settings.deletion_protection_enabled {
                guard.account_settings.deletion_protection_enabled = Some(v);
            }
            if let Some(v) = view.account_settings.deletion_protection_period_in_minutes {
                guard.account_settings.deletion_protection_period_in_minutes = Some(v);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
