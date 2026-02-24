use std::collections::HashMap;

/// A Timestream for InfluxDB DB instance.
#[derive(Debug, Clone)]
pub struct DbInstance {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub endpoint: Option<String>,
    pub port: Option<i32>,
    pub db_instance_type: String,
    pub db_storage_type: Option<String>,
    pub allocated_storage: i32,
    pub deployment_type: Option<String>,
    pub vpc_subnet_ids: Vec<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub publicly_accessible: Option<bool>,
    pub db_parameter_group_identifier: Option<String>,
    pub availability_zone: Option<String>,
    pub tags: HashMap<String, String>,
}

/// Summary of a DB instance for listing.
#[derive(Debug, Clone)]
pub struct DbInstanceSummary {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: Option<String>,
    pub endpoint: Option<String>,
    pub port: Option<i32>,
    pub db_instance_type: Option<String>,
    pub db_storage_type: Option<String>,
    pub allocated_storage: Option<i32>,
    pub deployment_type: Option<String>,
}

/// A Timestream for InfluxDB DB cluster.
#[derive(Debug, Clone)]
pub struct DbCluster {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: String,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub port: Option<i32>,
    pub deployment_type: Option<String>,
    pub db_instance_type: Option<String>,
    pub network_type: Option<String>,
    pub db_storage_type: Option<String>,
    pub allocated_storage: Option<i32>,
    pub publicly_accessible: Option<bool>,
    pub db_parameter_group_identifier: Option<String>,
    pub vpc_subnet_ids: Vec<String>,
    pub vpc_security_group_ids: Vec<String>,
    pub failover_mode: Option<String>,
    pub tags: HashMap<String, String>,
}

/// Summary of a DB cluster for listing.
#[derive(Debug, Clone)]
pub struct DbClusterSummary {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub status: Option<String>,
    pub endpoint: Option<String>,
    pub reader_endpoint: Option<String>,
    pub port: Option<i32>,
    pub deployment_type: Option<String>,
    pub db_instance_type: Option<String>,
    pub network_type: Option<String>,
    pub db_storage_type: Option<String>,
    pub allocated_storage: Option<i32>,
}

/// A Timestream for InfluxDB DB parameter group.
#[derive(Debug, Clone)]
pub struct DbParameterGroup {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub tags: HashMap<String, String>,
}

/// Summary of a DB parameter group for listing.
#[derive(Debug, Clone)]
pub struct DbParameterGroupSummary {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
}
