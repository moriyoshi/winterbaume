use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ApplicationInstance {
    pub application_instance_id: String,
    pub name: String,
    pub status: String,
    pub description: Option<String>,
    pub default_runtime_context_device: String,
    pub application_instance_id_to_replace: Option<String>,
    pub tags: HashMap<String, String>,
    pub arn: String,
    pub created_time: String,
}

#[derive(Debug, Clone)]
pub struct Device {
    pub device_id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
    pub created_time: String,
}

#[derive(Debug, Clone)]
pub struct NodeFromTemplateJob {
    pub job_id: String,
    pub node_name: String,
    pub status: String,
    pub template_type: String,
    pub created_time: String,
}
