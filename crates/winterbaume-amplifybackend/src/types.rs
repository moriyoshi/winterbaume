#[derive(Debug, Clone)]
pub struct Backend {
    pub app_id: String,
    pub app_name: String,
    pub backend_environment_name: String,
    pub resource_name: Option<String>,
    pub amplify_meta_config: Option<String>,
    pub amplify_feature_flags: Option<String>,
}
