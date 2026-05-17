use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::types::*;

static APPCONFIG_ID_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Generate a short, unique ID valid for AppConfig resource IDs.
/// AppConfig requires IDs to be 4-7 lowercase alphanumeric characters.
fn short_appconfig_id() -> String {
    let n = APPCONFIG_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    // Base-36 encode to 6 lowercase alphanumeric chars.
    const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789";
    let mut n = n;
    let mut chars = [b'a'; 6];
    for i in (0..6).rev() {
        chars[i] = ALPHABET[(n % 36) as usize];
        n /= 36;
    }
    String::from_utf8(chars.to_vec()).unwrap()
}

#[derive(Debug, Default)]
pub struct AppConfigState {
    pub applications: HashMap<String, Application>,
    /// Key: (application_id, profile_id)
    pub configuration_profiles: HashMap<(String, String), ConfigurationProfileData>,
    /// Key: (application_id, profile_id, version_number)
    pub hosted_configuration_versions:
        HashMap<(String, String, i32), HostedConfigurationVersionData>,
    /// Next version number per (application_id, profile_id)
    pub next_version: HashMap<(String, String), i32>,
    /// Tags keyed by resource ARN
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Deployment strategies keyed by strategy ID
    pub deployment_strategies: HashMap<String, DeploymentStrategyData>,
    /// Environments keyed by "appId/envId"
    pub environments: HashMap<String, EnvironmentData>,
    /// Deployments keyed by "appId/envId/depNum"
    pub deployments: HashMap<String, DeploymentData>,
    /// Next deployment number per (application_id, environment_id)
    pub next_deployment_number: HashMap<(String, String), i32>,
    /// Extensions keyed by extension ID
    pub extensions: HashMap<String, ExtensionData>,
    /// Extension associations keyed by association ID
    pub extension_associations: HashMap<String, ExtensionAssociationData>,
    /// Account settings
    pub account_settings: AccountSettingsData,
}

#[derive(Debug, thiserror::Error)]
pub enum AppConfigError {
    #[error("Application {0} not found.")]
    ApplicationNotFound(String),
    #[error("Configuration profile {0} not found.")]
    ConfigurationProfileNotFound(String),
    #[error("Hosted configuration version {0} not found.")]
    HostedConfigurationVersionNotFound(i32),
    #[error("Deployment strategy {0} not found.")]
    DeploymentStrategyNotFound(String),
    #[error("Environment {0} not found.")]
    EnvironmentNotFound(String),
    #[error("Deployment {0} not found.")]
    DeploymentNotFound(i32),
    #[error("Extension {0} not found.")]
    ExtensionNotFound(String),
    #[error("Extension association {0} not found.")]
    ExtensionAssociationNotFound(String),
}

impl AppConfigState {
    // ---- Application CRUD ----

    pub fn create_application(
        &mut self,
        name: &str,
        description: &str,
    ) -> Result<&Application, AppConfigError> {
        // AppConfig application IDs must be 4-7 lowercase alphanumeric characters.
        let id = short_appconfig_id();

        let app = Application {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
        };

        self.applications.insert(id.clone(), app);
        Ok(self.applications.get(&id).unwrap())
    }

    pub fn get_application(&self, id: &str) -> Result<&Application, AppConfigError> {
        self.applications
            .get(id)
            .ok_or_else(|| AppConfigError::ApplicationNotFound(id.to_string()))
    }

    pub fn delete_application(&mut self, id: &str) -> Result<(), AppConfigError> {
        if self.applications.remove(id).is_none() {
            return Err(AppConfigError::ApplicationNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn list_applications(&self) -> Vec<&Application> {
        self.applications.values().collect()
    }

    pub fn update_application(
        &mut self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<&Application, AppConfigError> {
        let app = self
            .applications
            .get_mut(id)
            .ok_or_else(|| AppConfigError::ApplicationNotFound(id.to_string()))?;
        if let Some(n) = name {
            app.name = n.to_string();
        }
        if let Some(d) = description {
            app.description = d.to_string();
        }
        Ok(app)
    }

    // ---- ConfigurationProfile CRUD ----

    pub fn create_configuration_profile(
        &mut self,
        application_id: &str,
        name: &str,
        location_uri: &str,
        description: &str,
        profile_type: &str,
        retrieval_role_arn: Option<&str>,
    ) -> Result<&ConfigurationProfileData, AppConfigError> {
        // Verify the application exists
        if !self.applications.contains_key(application_id) {
            return Err(AppConfigError::ApplicationNotFound(
                application_id.to_string(),
            ));
        }
        let id = short_appconfig_id();
        let key = (application_id.to_string(), id.clone());
        let profile = ConfigurationProfileData {
            id: id.clone(),
            application_id: application_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            location_uri: location_uri.to_string(),
            r#type: profile_type.to_string(),
            retrieval_role_arn: retrieval_role_arn.map(String::from),
        };
        self.configuration_profiles.insert(key.clone(), profile);
        Ok(self.configuration_profiles.get(&key).unwrap())
    }

    pub fn get_configuration_profile(
        &self,
        application_id: &str,
        profile_id: &str,
    ) -> Result<&ConfigurationProfileData, AppConfigError> {
        let key = (application_id.to_string(), profile_id.to_string());
        self.configuration_profiles
            .get(&key)
            .ok_or_else(|| AppConfigError::ConfigurationProfileNotFound(profile_id.to_string()))
    }

    pub fn delete_configuration_profile(
        &mut self,
        application_id: &str,
        profile_id: &str,
    ) -> Result<(), AppConfigError> {
        let key = (application_id.to_string(), profile_id.to_string());
        if self.configuration_profiles.remove(&key).is_none() {
            return Err(AppConfigError::ConfigurationProfileNotFound(
                profile_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_configuration_profiles(
        &self,
        application_id: &str,
    ) -> Vec<&ConfigurationProfileData> {
        self.configuration_profiles
            .values()
            .filter(|p| p.application_id == application_id)
            .collect()
    }

    pub fn update_configuration_profile(
        &mut self,
        application_id: &str,
        profile_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        retrieval_role_arn: Option<&str>,
    ) -> Result<&ConfigurationProfileData, AppConfigError> {
        let key = (application_id.to_string(), profile_id.to_string());
        let profile = self
            .configuration_profiles
            .get_mut(&key)
            .ok_or_else(|| AppConfigError::ConfigurationProfileNotFound(profile_id.to_string()))?;
        if let Some(n) = name {
            profile.name = n.to_string();
        }
        if let Some(d) = description {
            profile.description = d.to_string();
        }
        if let Some(r) = retrieval_role_arn {
            profile.retrieval_role_arn = Some(r.to_string());
        }
        Ok(profile)
    }

    // ---- HostedConfigurationVersion CRUD ----

    pub fn create_hosted_configuration_version(
        &mut self,
        application_id: &str,
        configuration_profile_id: &str,
        content_type: &str,
        description: &str,
        content: Vec<u8>,
    ) -> Result<&HostedConfigurationVersionData, AppConfigError> {
        // Verify the configuration profile exists
        let profile_key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
        );
        if !self.configuration_profiles.contains_key(&profile_key) {
            return Err(AppConfigError::ConfigurationProfileNotFound(
                configuration_profile_id.to_string(),
            ));
        }

        let version_key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
        );
        let version_number = self.next_version.entry(version_key).or_insert(1);
        let vn = *version_number;
        *version_number += 1;

        let key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
            vn,
        );
        let version = HostedConfigurationVersionData {
            application_id: application_id.to_string(),
            configuration_profile_id: configuration_profile_id.to_string(),
            version_number: vn,
            content_type: content_type.to_string(),
            description: description.to_string(),
            content,
        };
        self.hosted_configuration_versions
            .insert(key.clone(), version);
        Ok(self.hosted_configuration_versions.get(&key).unwrap())
    }

    /// Look up the configuration content currently deployed to
    /// `(application_id, environment_id, configuration_profile_id)`.
    ///
    /// Returns the `(content_type, content)` of the hosted configuration
    /// version referenced by the highest-numbered `COMPLETE` deployment
    /// for that target, or `None` if no completed deployment exists or
    /// the referenced hosted version is missing / un-parseable.
    ///
    /// Used by `winterbaume-appconfigdata::GetLatestConfiguration` to
    /// resolve the actual deployed payload rather than returning an empty
    /// body.
    pub fn get_deployed_configuration(
        &self,
        application_id: &str,
        environment_id: &str,
        configuration_profile_id: &str,
    ) -> Option<(&str, &[u8])> {
        let deployment = self
            .deployments
            .values()
            .filter(|d| {
                d.application_id == application_id
                    && d.environment_id == environment_id
                    && d.configuration_profile_id == configuration_profile_id
                    && d.state == "COMPLETE"
            })
            .max_by_key(|d| d.deployment_number)?;
        let version_number: i32 = deployment.configuration_version.parse().ok()?;
        let key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
            version_number,
        );
        let version = self.hosted_configuration_versions.get(&key)?;
        Some((version.content_type.as_str(), version.content.as_slice()))
    }

    pub fn get_hosted_configuration_version(
        &self,
        application_id: &str,
        configuration_profile_id: &str,
        version_number: i32,
    ) -> Result<&HostedConfigurationVersionData, AppConfigError> {
        let key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
            version_number,
        );
        self.hosted_configuration_versions.get(&key).ok_or(
            AppConfigError::HostedConfigurationVersionNotFound(version_number),
        )
    }

    pub fn delete_hosted_configuration_version(
        &mut self,
        application_id: &str,
        configuration_profile_id: &str,
        version_number: i32,
    ) -> Result<(), AppConfigError> {
        let key = (
            application_id.to_string(),
            configuration_profile_id.to_string(),
            version_number,
        );
        if self.hosted_configuration_versions.remove(&key).is_none() {
            return Err(AppConfigError::HostedConfigurationVersionNotFound(
                version_number,
            ));
        }
        Ok(())
    }

    // ---- Tag operations ----

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.tags.get(resource_arn).cloned().unwrap_or_default()
    }

    // ---- DeploymentStrategy CRUD ----

    pub fn create_deployment_strategy(
        &mut self,
        name: &str,
        description: &str,
        deployment_duration_in_minutes: i32,
        final_bake_time_in_minutes: i32,
        growth_factor: f32,
        growth_type: &str,
        replicate_to: &str,
    ) -> Result<&DeploymentStrategyData, AppConfigError> {
        let id = short_appconfig_id();
        let ds = DeploymentStrategyData {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            deployment_duration_in_minutes,
            final_bake_time_in_minutes,
            growth_factor,
            growth_type: growth_type.to_string(),
            replicate_to: replicate_to.to_string(),
        };
        self.deployment_strategies.insert(id.clone(), ds);
        Ok(self.deployment_strategies.get(&id).unwrap())
    }

    pub fn get_deployment_strategy(
        &self,
        id: &str,
    ) -> Result<&DeploymentStrategyData, AppConfigError> {
        self.deployment_strategies
            .get(id)
            .ok_or_else(|| AppConfigError::DeploymentStrategyNotFound(id.to_string()))
    }

    pub fn delete_deployment_strategy(&mut self, id: &str) -> Result<(), AppConfigError> {
        if self.deployment_strategies.remove(id).is_none() {
            return Err(AppConfigError::DeploymentStrategyNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn list_deployment_strategies(&self) -> Vec<&DeploymentStrategyData> {
        self.deployment_strategies.values().collect()
    }

    pub fn update_deployment_strategy(
        &mut self,
        id: &str,
        name: Option<&str>,
        description: Option<&str>,
        deployment_duration_in_minutes: Option<i32>,
        final_bake_time_in_minutes: Option<i32>,
        growth_factor: Option<f32>,
        growth_type: Option<&str>,
    ) -> Result<&DeploymentStrategyData, AppConfigError> {
        let ds = self
            .deployment_strategies
            .get_mut(id)
            .ok_or_else(|| AppConfigError::DeploymentStrategyNotFound(id.to_string()))?;
        if let Some(n) = name {
            ds.name = n.to_string();
        }
        if let Some(d) = description {
            ds.description = d.to_string();
        }
        if let Some(v) = deployment_duration_in_minutes {
            ds.deployment_duration_in_minutes = v;
        }
        if let Some(v) = final_bake_time_in_minutes {
            ds.final_bake_time_in_minutes = v;
        }
        if let Some(v) = growth_factor {
            ds.growth_factor = v;
        }
        if let Some(v) = growth_type {
            ds.growth_type = v.to_string();
        }
        Ok(ds)
    }

    // ---- Environment CRUD ----

    pub fn create_environment(
        &mut self,
        application_id: &str,
        name: &str,
        description: &str,
        monitors: Vec<MonitorData>,
    ) -> Result<&EnvironmentData, AppConfigError> {
        if !self.applications.contains_key(application_id) {
            return Err(AppConfigError::ApplicationNotFound(
                application_id.to_string(),
            ));
        }
        let id = short_appconfig_id();
        let key = format!("{application_id}/{id}");
        let env = EnvironmentData {
            id: id.clone(),
            application_id: application_id.to_string(),
            name: name.to_string(),
            description: description.to_string(),
            state: "READY_FOR_DEPLOYMENT".to_string(),
            monitors,
        };
        self.environments.insert(key.clone(), env);
        Ok(self.environments.get(&key).unwrap())
    }

    pub fn get_environment(
        &self,
        application_id: &str,
        environment_id: &str,
    ) -> Result<&EnvironmentData, AppConfigError> {
        let key = format!("{application_id}/{environment_id}");
        self.environments
            .get(&key)
            .ok_or_else(|| AppConfigError::EnvironmentNotFound(environment_id.to_string()))
    }

    pub fn delete_environment(
        &mut self,
        application_id: &str,
        environment_id: &str,
    ) -> Result<(), AppConfigError> {
        let key = format!("{application_id}/{environment_id}");
        if self.environments.remove(&key).is_none() {
            return Err(AppConfigError::EnvironmentNotFound(
                environment_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_environments(&self, application_id: &str) -> Vec<&EnvironmentData> {
        self.environments
            .values()
            .filter(|e| e.application_id == application_id)
            .collect()
    }

    pub fn update_environment(
        &mut self,
        application_id: &str,
        environment_id: &str,
        name: Option<&str>,
        description: Option<&str>,
        monitors: Option<Vec<MonitorData>>,
    ) -> Result<&EnvironmentData, AppConfigError> {
        let key = format!("{application_id}/{environment_id}");
        let env = self
            .environments
            .get_mut(&key)
            .ok_or_else(|| AppConfigError::EnvironmentNotFound(environment_id.to_string()))?;
        if let Some(n) = name {
            env.name = n.to_string();
        }
        if let Some(d) = description {
            env.description = d.to_string();
        }
        if let Some(m) = monitors {
            env.monitors = m;
        }
        Ok(env)
    }

    // ---- Deployment CRUD ----

    pub fn start_deployment(
        &mut self,
        application_id: &str,
        environment_id: &str,
        deployment_strategy_id: &str,
        configuration_profile_id: &str,
        configuration_version: &str,
        description: &str,
    ) -> Result<&DeploymentData, AppConfigError> {
        // Verify environment exists
        let env_key = format!("{application_id}/{environment_id}");
        if !self.environments.contains_key(&env_key) {
            return Err(AppConfigError::EnvironmentNotFound(
                environment_id.to_string(),
            ));
        }
        let dep_key = (application_id.to_string(), environment_id.to_string());
        let dep_num = self.next_deployment_number.entry(dep_key).or_insert(1);
        let num = *dep_num;
        *dep_num += 1;

        let now = "2024-01-01T00:00:00Z".to_string();
        let key = format!("{application_id}/{environment_id}/{num}");
        let dep = DeploymentData {
            deployment_number: num,
            application_id: application_id.to_string(),
            environment_id: environment_id.to_string(),
            deployment_strategy_id: deployment_strategy_id.to_string(),
            configuration_profile_id: configuration_profile_id.to_string(),
            configuration_version: configuration_version.to_string(),
            state: "COMPLETE".to_string(),
            started_at: now.clone(),
            completed_at: now,
            description: description.to_string(),
        };
        self.deployments.insert(key.clone(), dep);
        Ok(self.deployments.get(&key).unwrap())
    }

    pub fn get_deployment(
        &self,
        application_id: &str,
        environment_id: &str,
        deployment_number: i32,
    ) -> Result<&DeploymentData, AppConfigError> {
        let key = format!("{application_id}/{environment_id}/{deployment_number}");
        self.deployments
            .get(&key)
            .ok_or(AppConfigError::DeploymentNotFound(deployment_number))
    }

    pub fn stop_deployment(
        &mut self,
        application_id: &str,
        environment_id: &str,
        deployment_number: i32,
    ) -> Result<&DeploymentData, AppConfigError> {
        let key = format!("{application_id}/{environment_id}/{deployment_number}");
        let dep = self
            .deployments
            .get_mut(&key)
            .ok_or(AppConfigError::DeploymentNotFound(deployment_number))?;
        dep.state = "REVERTED".to_string();
        Ok(dep)
    }

    pub fn list_deployments(
        &self,
        application_id: &str,
        environment_id: &str,
    ) -> Vec<&DeploymentData> {
        self.deployments
            .values()
            .filter(|d| d.application_id == application_id && d.environment_id == environment_id)
            .collect()
    }

    // ---- Extension CRUD ----

    pub fn create_extension(
        &mut self,
        name: &str,
        description: &str,
        actions: HashMap<String, Vec<ActionData>>,
        parameters: HashMap<String, ParameterData>,
        region: &str,
        account_id: &str,
    ) -> Result<&ExtensionData, AppConfigError> {
        let id = short_appconfig_id();
        let arn = format!("arn:aws:appconfig:{region}:{account_id}:extension/{id}/1");
        let ext = ExtensionData {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            version_number: 1,
            arn,
            actions,
            parameters,
        };
        self.extensions.insert(id.clone(), ext);
        Ok(self.extensions.get(&id).unwrap())
    }

    pub fn get_extension(&self, identifier: &str) -> Result<&ExtensionData, AppConfigError> {
        // Look up by ID first, then by ARN, then by name.
        if let Some(ext) = self.extensions.get(identifier) {
            return Ok(ext);
        }
        self.extensions
            .values()
            .find(|e| e.arn == identifier || e.name == identifier)
            .ok_or_else(|| AppConfigError::ExtensionNotFound(identifier.to_string()))
    }

    pub fn delete_extension(&mut self, identifier: &str) -> Result<(), AppConfigError> {
        // Try by ID first
        if self.extensions.remove(identifier).is_some() {
            return Ok(());
        }
        // Try by ARN or name
        let id = self
            .extensions
            .values()
            .find(|e| e.arn == identifier || e.name == identifier)
            .map(|e| e.id.clone());
        if let Some(id) = id {
            self.extensions.remove(&id);
            Ok(())
        } else {
            Err(AppConfigError::ExtensionNotFound(identifier.to_string()))
        }
    }

    pub fn list_extensions(&self) -> Vec<&ExtensionData> {
        self.extensions.values().collect()
    }

    pub fn update_extension(
        &mut self,
        identifier: &str,
        description: Option<&str>,
        actions: Option<HashMap<String, Vec<ActionData>>>,
        parameters: Option<HashMap<String, ParameterData>>,
    ) -> Result<&ExtensionData, AppConfigError> {
        // Find by ID, ARN, or name
        let id = if self.extensions.contains_key(identifier) {
            identifier.to_string()
        } else {
            self.extensions
                .values()
                .find(|e| e.arn == identifier || e.name == identifier)
                .map(|e| e.id.clone())
                .ok_or_else(|| AppConfigError::ExtensionNotFound(identifier.to_string()))?
        };
        let ext = self.extensions.get_mut(&id).unwrap();
        if let Some(d) = description {
            ext.description = d.to_string();
        }
        if let Some(a) = actions {
            ext.actions = a;
        }
        if let Some(p) = parameters {
            ext.parameters = p;
        }
        Ok(ext)
    }

    // ---- ExtensionAssociation CRUD ----

    pub fn create_extension_association(
        &mut self,
        extension_arn: &str,
        resource_arn: &str,
        extension_version_number: i32,
        parameters: HashMap<String, String>,
        region: &str,
        account_id: &str,
    ) -> Result<&ExtensionAssociationData, AppConfigError> {
        let id = short_appconfig_id();
        let arn = format!("arn:aws:appconfig:{region}:{account_id}:extensionassociation/{id}");
        let assoc = ExtensionAssociationData {
            id: id.clone(),
            arn,
            extension_arn: extension_arn.to_string(),
            resource_arn: resource_arn.to_string(),
            extension_version_number,
            parameters,
        };
        self.extension_associations.insert(id.clone(), assoc);
        Ok(self.extension_associations.get(&id).unwrap())
    }

    pub fn get_extension_association(
        &self,
        id: &str,
    ) -> Result<&ExtensionAssociationData, AppConfigError> {
        self.extension_associations
            .get(id)
            .ok_or_else(|| AppConfigError::ExtensionAssociationNotFound(id.to_string()))
    }

    pub fn delete_extension_association(&mut self, id: &str) -> Result<(), AppConfigError> {
        if self.extension_associations.remove(id).is_none() {
            return Err(AppConfigError::ExtensionAssociationNotFound(id.to_string()));
        }
        Ok(())
    }

    pub fn list_extension_associations(&self) -> Vec<&ExtensionAssociationData> {
        self.extension_associations.values().collect()
    }

    pub fn update_extension_association(
        &mut self,
        id: &str,
        parameters: Option<HashMap<String, String>>,
    ) -> Result<&ExtensionAssociationData, AppConfigError> {
        let assoc = self
            .extension_associations
            .get_mut(id)
            .ok_or_else(|| AppConfigError::ExtensionAssociationNotFound(id.to_string()))?;
        if let Some(p) = parameters {
            assoc.parameters = p;
        }
        Ok(assoc)
    }

    // ---- Account settings ----

    pub fn get_account_settings(&self) -> &AccountSettingsData {
        &self.account_settings
    }

    pub fn update_account_settings(
        &mut self,
        deletion_protection_enabled: Option<bool>,
        deletion_protection_period_in_minutes: Option<i32>,
    ) -> &AccountSettingsData {
        if let Some(v) = deletion_protection_enabled {
            self.account_settings.deletion_protection_enabled = Some(v);
        }
        if let Some(v) = deletion_protection_period_in_minutes {
            self.account_settings.deletion_protection_period_in_minutes = Some(v);
        }
        &self.account_settings
    }

    // ---- ListHostedConfigurationVersions ----

    pub fn list_hosted_configuration_versions(
        &self,
        application_id: &str,
        configuration_profile_id: &str,
    ) -> Vec<&HostedConfigurationVersionData> {
        self.hosted_configuration_versions
            .values()
            .filter(|v| {
                v.application_id == application_id
                    && v.configuration_profile_id == configuration_profile_id
            })
            .collect()
    }
}
