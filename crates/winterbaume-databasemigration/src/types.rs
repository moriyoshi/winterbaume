use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ReplicationInstance {
    pub replication_instance_identifier: String,
    pub replication_instance_class: String,
    pub allocated_storage: i32,
    pub status: String,
    pub replication_instance_arn: String,
    pub availability_zone: Option<String>,
    pub publicly_accessible: bool,
    pub multi_az: bool,
    pub engine_version: Option<String>,
    pub instance_create_time: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub endpoint_identifier: String,
    pub endpoint_type: String,
    pub engine_name: String,
    pub username: Option<String>,
    pub server_name: Option<String>,
    pub port: Option<i32>,
    pub database_name: Option<String>,
    pub status: String,
    pub endpoint_arn: String,
    pub extra_connection_attributes: Option<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct ReplicationTask {
    pub replication_task_identifier: String,
    pub source_endpoint_arn: String,
    pub target_endpoint_arn: String,
    pub replication_instance_arn: String,
    pub migration_type: String,
    pub table_mappings: String,
    pub replication_task_settings: Option<String>,
    pub status: String,
    pub replication_task_arn: String,
    pub replication_task_creation_date: f64,
    pub replication_task_start_date: Option<f64>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Connection {
    pub replication_instance_arn: String,
    pub endpoint_arn: String,
    pub replication_instance_identifier: String,
    pub endpoint_identifier: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ReplicationSubnetGroup {
    pub replication_subnet_group_identifier: String,
    pub replication_subnet_group_description: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct Certificate {
    pub certificate_identifier: String,
    pub certificate_arn: String,
    pub certificate_pem: Option<String>,
    pub certificate_wallet: Option<String>,
    pub kms_key_id: Option<String>,
    pub certificate_creation_date: f64,
}

#[derive(Debug, Clone)]
pub struct EventSubscription {
    pub subscription_name: String,
    pub sns_topic_arn: String,
    pub source_type: Option<String>,
    pub event_categories: Vec<String>,
    pub source_ids: Vec<String>,
    pub enabled: bool,
    pub status: String,
    pub subscription_creation_time: String,
    pub customer_aws_id: String,
}
