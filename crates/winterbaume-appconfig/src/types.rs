#[derive(Debug, Clone)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ConfigurationProfileData {
    pub id: String,
    pub application_id: String,
    pub name: String,
    pub description: String,
    pub location_uri: String,
    pub r#type: String,
    pub retrieval_role_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct HostedConfigurationVersionData {
    pub application_id: String,
    pub configuration_profile_id: String,
    pub version_number: i32,
    pub content_type: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct DeploymentStrategyData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub deployment_duration_in_minutes: i32,
    pub final_bake_time_in_minutes: i32,
    pub growth_factor: f32,
    pub growth_type: String,
    pub replicate_to: String,
}

#[derive(Debug, Clone)]
pub struct EnvironmentData {
    pub id: String,
    pub application_id: String,
    pub name: String,
    pub description: String,
    pub state: String,
    pub monitors: Vec<MonitorData>,
}

#[derive(Debug, Clone)]
pub struct MonitorData {
    pub alarm_arn: String,
    pub alarm_role_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeploymentData {
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

#[derive(Debug, Clone)]
pub struct ExtensionData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version_number: i32,
    pub arn: String,
    pub actions: std::collections::HashMap<String, Vec<ActionData>>,
    pub parameters: std::collections::HashMap<String, ParameterData>,
}

#[derive(Debug, Clone)]
pub struct ActionData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub uri: Option<String>,
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ParameterData {
    pub description: Option<String>,
    pub required: Option<bool>,
    pub dynamic: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ExtensionAssociationData {
    pub id: String,
    pub arn: String,
    pub extension_arn: String,
    pub resource_arn: String,
    pub extension_version_number: i32,
    pub parameters: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Default)]
pub struct AccountSettingsData {
    pub deletion_protection_enabled: Option<bool>,
    pub deletion_protection_period_in_minutes: Option<i32>,
}
