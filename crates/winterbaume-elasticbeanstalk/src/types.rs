use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Application {
    pub application_name: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub date_created: String,
    pub date_updated: String,
    pub arn: String,
}

#[derive(Debug, Clone)]
pub struct Environment {
    pub environment_name: String,
    pub application_name: String,
    pub environment_id: String,
    pub description: Option<String>,
    pub status: String,
    pub tier_name: String,
    pub tier_type: String,
    pub health: String,
    pub solution_stack_name: Option<String>,
    pub tags: HashMap<String, String>,
    pub date_created: String,
    pub date_updated: String,
    pub arn: String,
    pub cname: Option<String>,
    pub endpoint_url: Option<String>,
    pub platform_arn: Option<String>,
    pub version_label: Option<String>,
    pub template_name: Option<String>,
}
