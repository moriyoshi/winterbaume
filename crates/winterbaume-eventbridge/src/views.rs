//! Serde-compatible view types for EventBridge state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::EventBridgeService;
use crate::state::EventsState;
use crate::types::{
    ApiDestination, Archive, Connection, Endpoint, EventBus, PartnerEventSource, Replay, Rule,
    RuleState, Tag, Target,
};

/// Serializable view of the entire EventBridge state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventsStateView {
    /// Rules keyed by rule name.
    #[serde(default)]
    pub rules: HashMap<String, RuleView>,
    /// Event buses keyed by name.
    #[serde(default)]
    pub event_buses: HashMap<String, EventBusView>,
    /// Archives keyed by name.
    #[serde(default)]
    pub archives: HashMap<String, ArchiveView>,
    /// Connections keyed by name.
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
    /// API destinations keyed by name.
    #[serde(default)]
    pub api_destinations: HashMap<String, ApiDestinationView>,
    /// Replays keyed by name.
    #[serde(default)]
    pub replays: HashMap<String, ReplayView>,
    /// Partner event sources keyed by name.
    #[serde(default)]
    pub partner_event_sources: HashMap<String, PartnerEventSourceView>,
    /// Tags keyed by resource ARN.
    #[serde(default)]
    pub tags: HashMap<String, Vec<TagView>>,
    /// Endpoints keyed by name.
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointView>,
}

/// Serializable view of a rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleView {
    pub name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub schedule_expression: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub event_bus_name: String,
    pub targets: Vec<TargetView>,
    pub tags: Vec<TagView>,
}

/// Serializable view of a rule target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetView {
    pub id: String,
    pub arn: String,
    pub input: Option<String>,
    pub input_path: Option<String>,
    #[serde(default)]
    pub extra: Option<serde_json::Value>,
}

/// Serializable view of a tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagView {
    pub key: String,
    pub value: String,
}

/// Serializable view of an event bus.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventBusView {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub kms_key_identifier: Option<String>,
    #[serde(default)]
    pub dead_letter_config: Option<serde_json::Value>,
}

/// Serializable view of an endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointView {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub routing_config: Option<String>,
    pub replication_config: Option<String>,
    #[serde(default)]
    pub event_buses: Vec<String>,
    pub role_arn: Option<String>,
    pub state: String,
    pub endpoint_id: String,
    pub endpoint_url: String,
    pub creation_time: f64,
    pub last_modified_time: f64,
}

/// Serializable view of an archive.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveView {
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

/// Serializable view of a connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub name: String,
    pub arn: String,
    pub state: String,
    pub description: Option<String>,
    pub authorization_type: String,
    pub auth_parameters: String,
    pub creation_time: f64,
}

/// Serializable view of an API destination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiDestinationView {
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

/// Serializable view of a replay.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplayView {
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

/// Serializable view of a partner event source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartnerEventSourceView {
    pub name: String,
    pub arn: String,
    #[serde(default = "default_event_source_state")]
    pub state: String,
}

fn default_event_source_state() -> String {
    "ACTIVE".to_string()
}

// --- From internal types to view types ---

impl From<&EventsState> for EventsStateView {
    fn from(state: &EventsState) -> Self {
        EventsStateView {
            rules: state
                .rules
                .iter()
                .map(|(k, v)| (k.clone(), RuleView::from(v)))
                .collect(),
            event_buses: state
                .event_buses
                .iter()
                .map(|(k, v)| (k.clone(), EventBusView::from(v)))
                .collect(),
            archives: state
                .archives
                .iter()
                .map(|(k, v)| (k.clone(), ArchiveView::from(v)))
                .collect(),
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
            api_destinations: state
                .api_destinations
                .iter()
                .map(|(k, v)| (k.clone(), ApiDestinationView::from(v)))
                .collect(),
            replays: state
                .replays
                .iter()
                .map(|(k, v)| (k.clone(), ReplayView::from(v)))
                .collect(),
            partner_event_sources: state
                .partner_event_sources
                .iter()
                .map(|(k, v)| (k.clone(), PartnerEventSourceView::from(v)))
                .collect(),
            tags: state
                .tags
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(TagView::from).collect()))
                .collect(),
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, v)| (k.clone(), EndpointView::from(v)))
                .collect(),
        }
    }
}

impl From<&Rule> for RuleView {
    fn from(r: &Rule) -> Self {
        RuleView {
            name: r.name.clone(),
            arn: r.arn.clone(),
            event_pattern: r.event_pattern.clone(),
            schedule_expression: r.schedule_expression.clone(),
            state: r.state.as_str().to_string(),
            description: r.description.clone(),
            event_bus_name: r.event_bus_name.clone(),
            targets: r.targets.iter().map(TargetView::from).collect(),
            tags: r.tags.iter().map(TagView::from).collect(),
        }
    }
}

impl From<&Target> for TargetView {
    fn from(t: &Target) -> Self {
        TargetView {
            id: t.id.clone(),
            arn: t.arn.clone(),
            input: t.input.clone(),
            input_path: t.input_path.clone(),
            extra: None,
        }
    }
}

impl From<&Tag> for TagView {
    fn from(t: &Tag) -> Self {
        TagView {
            key: t.key.clone(),
            value: t.value.clone(),
        }
    }
}

impl From<&EventBus> for EventBusView {
    fn from(b: &EventBus) -> Self {
        EventBusView {
            name: b.name.clone(),
            arn: b.arn.clone(),
            policy: b.policy.clone(),
            description: b.description.clone(),
            kms_key_identifier: b.kms_key_identifier.clone(),
            dead_letter_config: None,
        }
    }
}

impl From<&Endpoint> for EndpointView {
    fn from(e: &Endpoint) -> Self {
        EndpointView {
            name: e.name.clone(),
            arn: e.arn.clone(),
            description: e.description.clone(),
            routing_config: e.routing_config.clone(),
            replication_config: e.replication_config.clone(),
            event_buses: e.event_buses.clone(),
            role_arn: e.role_arn.clone(),
            state: e.state.clone(),
            endpoint_id: e.endpoint_id.clone(),
            endpoint_url: e.endpoint_url.clone(),
            creation_time: e.creation_time,
            last_modified_time: e.last_modified_time,
        }
    }
}

impl From<&Archive> for ArchiveView {
    fn from(a: &Archive) -> Self {
        ArchiveView {
            name: a.name.clone(),
            arn: a.arn.clone(),
            event_source_arn: a.event_source_arn.clone(),
            description: a.description.clone(),
            event_pattern: a.event_pattern.clone(),
            retention_days: a.retention_days,
            state: a.state.clone(),
            creation_time: a.creation_time,
            size_bytes: a.size_bytes,
        }
    }
}

impl From<&Connection> for ConnectionView {
    fn from(c: &Connection) -> Self {
        ConnectionView {
            name: c.name.clone(),
            arn: c.arn.clone(),
            state: c.state.clone(),
            description: c.description.clone(),
            authorization_type: c.authorization_type.clone(),
            auth_parameters: c.auth_parameters.clone(),
            creation_time: c.creation_time,
        }
    }
}

impl From<&ApiDestination> for ApiDestinationView {
    fn from(d: &ApiDestination) -> Self {
        ApiDestinationView {
            name: d.name.clone(),
            arn: d.arn.clone(),
            description: d.description.clone(),
            connection_arn: d.connection_arn.clone(),
            invocation_endpoint: d.invocation_endpoint.clone(),
            http_method: d.http_method.clone(),
            invocation_rate_limit: d.invocation_rate_limit,
            state: d.state.clone(),
            creation_time: d.creation_time,
        }
    }
}

impl From<&Replay> for ReplayView {
    fn from(r: &Replay) -> Self {
        ReplayView {
            name: r.name.clone(),
            arn: r.arn.clone(),
            event_source_arn: r.event_source_arn.clone(),
            description: r.description.clone(),
            state: r.state.clone(),
            start_time: r.start_time,
            end_time: r.end_time,
            destination: r.destination.clone(),
            replay_start_time: r.replay_start_time,
            replay_end_time: r.replay_end_time,
        }
    }
}

impl From<&PartnerEventSource> for PartnerEventSourceView {
    fn from(p: &PartnerEventSource) -> Self {
        PartnerEventSourceView {
            name: p.name.clone(),
            arn: p.arn.clone(),
            state: p.state.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<EventsStateView> for EventsState {
    fn from(view: EventsStateView) -> Self {
        EventsState {
            rules: view
                .rules
                .into_iter()
                .map(|(k, v)| (k, Rule::from(v)))
                .collect(),
            event_buses: view
                .event_buses
                .into_iter()
                .map(|(k, v)| (k, EventBus::from(v)))
                .collect(),
            archives: view
                .archives
                .into_iter()
                .map(|(k, v)| (k, Archive::from(v)))
                .collect(),
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, Connection::from(v)))
                .collect(),
            api_destinations: view
                .api_destinations
                .into_iter()
                .map(|(k, v)| (k, ApiDestination::from(v)))
                .collect(),
            replays: view
                .replays
                .into_iter()
                .map(|(k, v)| (k, Replay::from(v)))
                .collect(),
            partner_event_sources: view
                .partner_event_sources
                .into_iter()
                .map(|(k, v)| (k, PartnerEventSource::from(v)))
                .collect(),
            tags: view
                .tags
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(Tag::from).collect()))
                .collect(),
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, v)| (k, Endpoint::from(v)))
                .collect(),
        }
    }
}

impl From<RuleView> for Rule {
    fn from(v: RuleView) -> Self {
        Rule {
            name: v.name,
            arn: v.arn,
            event_pattern: v.event_pattern,
            schedule_expression: v.schedule_expression,
            state: RuleState::parse(&v.state),
            description: v.description,
            event_bus_name: v.event_bus_name,
            targets: v.targets.into_iter().map(Target::from).collect(),
            tags: v.tags.into_iter().map(Tag::from).collect(),
        }
    }
}

impl From<TargetView> for Target {
    fn from(v: TargetView) -> Self {
        Target {
            id: v.id,
            arn: v.arn,
            input: v.input,
            input_path: v.input_path,
        }
    }
}

impl From<TagView> for Tag {
    fn from(v: TagView) -> Self {
        Tag {
            key: v.key,
            value: v.value,
        }
    }
}

impl From<EventBusView> for EventBus {
    fn from(v: EventBusView) -> Self {
        EventBus {
            name: v.name,
            arn: v.arn,
            policy: v.policy,
            description: v.description,
            kms_key_identifier: v.kms_key_identifier,
        }
    }
}

impl From<EndpointView> for Endpoint {
    fn from(v: EndpointView) -> Self {
        Endpoint {
            name: v.name,
            arn: v.arn,
            description: v.description,
            routing_config: v.routing_config,
            replication_config: v.replication_config,
            event_buses: v.event_buses,
            role_arn: v.role_arn,
            state: v.state,
            endpoint_id: v.endpoint_id,
            endpoint_url: v.endpoint_url,
            creation_time: v.creation_time,
            last_modified_time: v.last_modified_time,
        }
    }
}

impl From<ArchiveView> for Archive {
    fn from(v: ArchiveView) -> Self {
        Archive {
            name: v.name,
            arn: v.arn,
            event_source_arn: v.event_source_arn,
            description: v.description,
            event_pattern: v.event_pattern,
            retention_days: v.retention_days,
            state: v.state,
            creation_time: v.creation_time,
            size_bytes: v.size_bytes,
        }
    }
}

impl From<ConnectionView> for Connection {
    fn from(v: ConnectionView) -> Self {
        Connection {
            name: v.name,
            arn: v.arn,
            state: v.state,
            description: v.description,
            authorization_type: v.authorization_type,
            auth_parameters: v.auth_parameters,
            creation_time: v.creation_time,
        }
    }
}

impl From<ApiDestinationView> for ApiDestination {
    fn from(v: ApiDestinationView) -> Self {
        ApiDestination {
            name: v.name,
            arn: v.arn,
            description: v.description,
            connection_arn: v.connection_arn,
            invocation_endpoint: v.invocation_endpoint,
            http_method: v.http_method,
            invocation_rate_limit: v.invocation_rate_limit,
            state: v.state,
            creation_time: v.creation_time,
        }
    }
}

impl From<ReplayView> for Replay {
    fn from(v: ReplayView) -> Self {
        Replay {
            name: v.name,
            arn: v.arn,
            event_source_arn: v.event_source_arn,
            description: v.description,
            state: v.state,
            start_time: v.start_time,
            end_time: v.end_time,
            destination: v.destination,
            replay_start_time: v.replay_start_time,
            replay_end_time: v.replay_end_time,
        }
    }
}

impl From<PartnerEventSourceView> for PartnerEventSource {
    fn from(v: PartnerEventSourceView) -> Self {
        PartnerEventSource {
            name: v.name,
            arn: v.arn,
            state: v.state,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for EventBridgeService {
    type StateView = EventsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        EventsStateView::from(&*guard)
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            *guard = EventsState::from(view);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    async fn merge(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, rule_view) in view.rules {
                guard.rules.insert(name, Rule::from(rule_view));
            }
            for (name, bus_view) in view.event_buses {
                guard.event_buses.insert(name, EventBus::from(bus_view));
            }
            for (name, archive_view) in view.archives {
                guard.archives.insert(name, Archive::from(archive_view));
            }
            for (name, conn_view) in view.connections {
                guard.connections.insert(name, Connection::from(conn_view));
            }
            for (name, dest_view) in view.api_destinations {
                guard
                    .api_destinations
                    .insert(name, ApiDestination::from(dest_view));
            }
            for (name, replay_view) in view.replays {
                guard.replays.insert(name, Replay::from(replay_view));
            }
            for (name, pes_view) in view.partner_event_sources {
                guard
                    .partner_event_sources
                    .insert(name, PartnerEventSource::from(pes_view));
            }
            for (arn, tag_views) in view.tags {
                guard
                    .tags
                    .insert(arn, tag_views.into_iter().map(Tag::from).collect());
            }
            for (name, ep_view) in view.endpoints {
                guard.endpoints.insert(name, Endpoint::from(ep_view));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
