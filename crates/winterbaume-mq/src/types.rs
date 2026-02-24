use std::collections::HashMap;

use chrono::{DateTime, Utc};

/// Represents an Amazon MQ broker.
#[derive(Debug, Clone)]
pub struct Broker {
    pub broker_id: String,
    pub broker_name: String,
    pub broker_arn: String,
    pub broker_state: String,
    pub engine_type: String,
    pub engine_version: String,
    pub host_instance_type: String,
    pub deployment_mode: String,
    pub created: DateTime<Utc>,
    pub publicly_accessible: bool,
    pub auto_minor_version_upgrade: bool,
    pub tags: HashMap<String, String>,
    pub users: HashMap<String, MqUser>,
}

/// Summary of a broker for list operations.
#[derive(Debug, Clone)]
pub struct BrokerSummary {
    pub broker_id: String,
    pub broker_name: String,
    pub broker_arn: String,
    pub broker_state: String,
    pub engine_type: String,
    pub deployment_mode: String,
    pub created: DateTime<Utc>,
    pub host_instance_type: String,
}

/// Represents an Amazon MQ configuration.
#[derive(Debug, Clone)]
pub struct MqConfiguration {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub description: String,
    pub engine_type: String,
    pub engine_version: String,
    pub authentication_strategy: String,
    pub created: DateTime<Utc>,
    pub tags: HashMap<String, String>,
    pub revisions: Vec<MqConfigurationRevision>,
}

/// Represents a configuration revision.
#[derive(Debug, Clone)]
pub struct MqConfigurationRevision {
    pub revision: i32,
    pub created: DateTime<Utc>,
    pub description: String,
    pub data: String,
}

/// Represents a user within a broker.
#[derive(Debug, Clone)]
pub struct MqUser {
    pub username: String,
    pub console_access: bool,
    pub groups: Vec<String>,
    pub replication_user: bool,
}
