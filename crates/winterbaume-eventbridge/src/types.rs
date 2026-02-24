/// An EventBridge rule.
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub schedule_expression: Option<String>,
    pub state: RuleState,
    pub description: Option<String>,
    pub event_bus_name: String,
    pub targets: Vec<Target>,
    pub tags: Vec<Tag>,
}

/// Rule state enum.
#[derive(Debug, Clone, PartialEq)]
pub enum RuleState {
    Enabled,
    Disabled,
}

impl RuleState {
    pub fn as_str(&self) -> &str {
        match self {
            RuleState::Enabled => "ENABLED",
            RuleState::Disabled => "DISABLED",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "DISABLED" => RuleState::Disabled,
            _ => RuleState::Enabled,
        }
    }
}

/// A rule target.
#[derive(Debug, Clone)]
pub struct Target {
    pub id: String,
    pub arn: String,
    pub input: Option<String>,
    pub input_path: Option<String>,
}

/// A tag key-value pair.
#[derive(Debug, Clone)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

/// An EventBridge event bus.
#[derive(Debug, Clone)]
pub struct EventBus {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
    pub description: Option<String>,
    pub kms_key_identifier: Option<String>,
}

/// An EventBridge archive.
#[derive(Debug, Clone)]
pub struct Archive {
    pub name: String,
    pub arn: String,
    pub event_source_arn: String,
    pub description: Option<String>,
    pub event_pattern: Option<String>,
    pub retention_days: i64,
    pub state: String,
    pub creation_time: f64,
    pub size_bytes: i64,
}

/// An EventBridge connection.
#[derive(Debug, Clone)]
pub struct Connection {
    pub name: String,
    pub arn: String,
    pub state: String,
    pub description: Option<String>,
    pub authorization_type: String,
    pub auth_parameters: String,
    pub creation_time: f64,
}

/// An EventBridge API destination.
#[derive(Debug, Clone)]
pub struct ApiDestination {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub connection_arn: String,
    pub invocation_endpoint: String,
    pub http_method: String,
    pub invocation_rate_limit: i64,
    pub state: String,
    pub creation_time: f64,
}

/// An EventBridge replay.
#[derive(Debug, Clone)]
pub struct Replay {
    pub name: String,
    pub arn: String,
    pub event_source_arn: String,
    pub description: Option<String>,
    pub state: String,
    pub start_time: f64,
    pub end_time: f64,
    pub destination: String,
    pub replay_start_time: f64,
    pub replay_end_time: f64,
}

/// A partner event source.
#[derive(Debug, Clone)]
pub struct PartnerEventSource {
    pub name: String,
    pub arn: String,
    /// Whether this event source is currently active ("ACTIVE" or "PENDING").
    pub state: String,
}

/// An EventBridge endpoint (global endpoint for multi-region failover).
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub routing_config: Option<String>, // JSON-serialized RoutingConfig
    pub replication_config: Option<String>, // JSON-serialized ReplicationConfig
    pub event_buses: Vec<String>,       // list of event bus ARNs
    pub role_arn: Option<String>,
    pub state: String,
    pub endpoint_id: String,
    pub endpoint_url: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
}
