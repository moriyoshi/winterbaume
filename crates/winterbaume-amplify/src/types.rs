use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct AmplifyApp {
    pub app_id: String,
    pub app_arn: String,
    pub name: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub platform: Option<String>,
    pub create_time: f64,
    pub update_time: f64,
    pub iam_service_role_arn: Option<String>,
    pub environment_variables: HashMap<String, String>,
    pub default_domain: String,
    pub enable_branch_auto_build: bool,
    pub enable_branch_auto_deletion: bool,
    pub enable_basic_auth: bool,
    pub build_spec: Option<String>,
    pub custom_headers: Option<String>,
    pub tags: HashMap<String, String>,
    pub auto_branch_creation_config: Option<serde_json::Value>,
    pub cache_config: Option<serde_json::Value>,
    pub custom_rules: Vec<serde_json::Value>,
}

#[derive(Debug, Clone)]
pub struct AmplifyBranch {
    pub branch_arn: String,
    pub branch_name: String,
    pub description: Option<String>,
    pub stage: Option<String>,
    pub display_name: Option<String>,
    pub enable_auto_build: bool,
    pub enable_basic_auth: bool,
    pub enable_notification: bool,
    pub enable_performance_mode: bool,
    pub enable_pull_request_preview: bool,
    pub environment_variables: HashMap<String, String>,
    pub framework: Option<String>,
    pub ttl: Option<String>,
    pub create_time: f64,
    pub update_time: f64,
    pub total_number_of_jobs: String,
    pub active_job_id: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct AmplifyDomainAssociation {
    pub domain_association_arn: String,
    pub domain_name: String,
    pub enable_auto_sub_domain: bool,
    pub domain_status: String,
    pub status_reason: String,
    pub sub_domains: Vec<SubDomain>,
}

#[derive(Debug, Clone)]
pub struct SubDomain {
    pub prefix: String,
    pub branch_name: String,
    pub dns_record: Option<String>,
    pub verified: bool,
}

#[derive(Debug, Clone)]
pub struct AmplifyJob {
    pub job_id: String,
    pub job_arn: String,
    pub job_type: String,
    pub status: String,
    pub start_time: f64,
    pub end_time: Option<f64>,
    pub commit_id: Option<String>,
    pub commit_message: Option<String>,
    pub commit_time: Option<f64>,
}
