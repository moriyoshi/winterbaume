/// An EventBridge rule.
///
/// A rule belongs to one event bus, and rules are keyed by ( bus, name ):
/// two buses can each hold a rule with the same name. The rule's tags are not
/// stored here but in `EventsState::tags` under the rule ARN, which is where
/// `TagResource` / `UntagResource` / `ListTagsForResource` look.
#[derive(Debug, Clone)]
pub struct Rule {
    pub name: String,
    /// `arn:aws:events:<region>:<account>:rule/<name>` on the default bus,
    /// `arn:aws:events:<region>:<account>:rule/<bus>/<name>` on any other bus.
    pub arn: String,
    pub event_pattern: Option<String>,
    pub schedule_expression: Option<String>,
    pub state: RuleState,
    pub description: Option<String>,
    /// The bus name ( never an ARN ).
    pub event_bus_name: String,
    /// `RoleArn` as given to the latest `PutRule`.
    pub role_arn: Option<String>,
    pub targets: Vec<Target>,
}

/// Identifies a rule: the name of its event bus and the rule name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RuleKey {
    pub event_bus_name: String,
    pub name: String,
}

impl RuleKey {
    pub fn new(event_bus_name: &str, name: &str) -> Self {
        Self {
            event_bus_name: event_bus_name.to_string(),
            name: name.to_string(),
        }
    }
}

/// Rule state enum.
#[derive(Debug, Clone, PartialEq)]
pub enum RuleState {
    Enabled,
    Disabled,
    EnabledWithAllCloudtrailManagementEvents,
}

impl RuleState {
    pub fn as_str(&self) -> &str {
        match self {
            RuleState::Enabled => "ENABLED",
            RuleState::Disabled => "DISABLED",
            RuleState::EnabledWithAllCloudtrailManagementEvents => {
                "ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS"
            }
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "DISABLED" => RuleState::Disabled,
            "ENABLED_WITH_ALL_CLOUDTRAIL_MANAGEMENT_EVENTS" => {
                RuleState::EnabledWithAllCloudtrailManagementEvents
            }
            _ => RuleState::Enabled,
        }
    }
}

/// A rule target.
///
/// The target is kept exactly as `PutTargets` received it, as the
/// Smithy-derived wire structure, so that every modelled member
/// ( `RetryPolicy`, `DeadLetterConfig`, `RoleArn`, `InputTransformer`, the
/// service-specific `*Parameters` blocks, and whatever the model gains later )
/// is stored and echoed by `ListTargetsByRule` without per-member plumbing.
#[derive(Debug, Clone, Default)]
pub struct Target {
    /// The target as sent to `PutTargets`.
    pub spec: crate::model::Target,
    /// Snapshot-only data from `TargetView::extra` that is not an API member
    /// ( e.g. Terraform-shaped blocks written by a state injector ). It is
    /// never surfaced through the API; it only round-trips through
    /// snapshot/restore. A `PutTargets` for the same Id replaces the whole
    /// target and therefore clears it.
    pub passthrough: Option<serde_json::Value>,
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
