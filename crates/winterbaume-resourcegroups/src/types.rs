#[derive(Debug, Clone)]
pub struct ResourceGroup {
    pub name: String,
    pub arn: String,
    pub description: String,
    pub resource_query_type: String,
    pub resource_query_query: String,
    pub tags: std::collections::HashMap<String, String>,
    pub configuration: Option<Vec<GroupConfigurationEntry>>,
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct AccountSettings {
    pub group_lifecycle_events_desired_status: String,
}

#[derive(Debug, Clone)]
pub struct GroupConfigurationEntry {
    pub config_type: String,
    pub parameters: Vec<GroupConfigParam>,
}

#[derive(Debug, Clone)]
pub struct GroupConfigParam {
    pub name: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TagSyncTask {
    pub task_arn: String,
    pub group_name: String,
    pub group_arn: String,
    pub tag_key: String,
    pub tag_value: String,
    pub role_arn: String,
    pub status: String,
    pub created_at: f64,
}
