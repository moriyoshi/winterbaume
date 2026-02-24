//! Serde-compatible view types for CloudWatch Logs state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::CloudWatchLogsService;
use crate::state::LogsState;
use crate::types::{
    AccountPolicy, Anomaly, Delivery, DeliveryDestination, DeliveryDestinationConfiguration,
    DeliveryDestinationPolicy, DeliverySource, Destination, ExportTask, ImportTask, IndexPolicy,
    Integration, LogAnomalyDetector, LogGroup, LogStream, MetricFilter, MetricTransformation,
    QueryDefinition, ResourcePolicy, ScheduledQuery, SubscriptionFilter, Transformer,
};

// ---------------------------------------------------------------------------
// View types
// ---------------------------------------------------------------------------

/// Serializable view of the entire CloudWatch Logs state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogsStateView {
    #[serde(default)]
    pub log_groups: HashMap<String, LogGroupView>,
    #[serde(default)]
    pub metric_filters: HashMap<String, MetricFilterView>,
    #[serde(default)]
    pub subscription_filters: HashMap<String, SubscriptionFilterView>,
    #[serde(default)]
    pub resource_policies: HashMap<String, ResourcePolicyView>,
    #[serde(default)]
    pub destinations: HashMap<String, DestinationView>,
    #[serde(default)]
    pub export_tasks: HashMap<String, ExportTaskView>,
    #[serde(default)]
    pub delivery_sources: HashMap<String, DeliverySourceView>,
    #[serde(default)]
    pub delivery_destinations: HashMap<String, DeliveryDestinationView>,
    #[serde(default)]
    pub delivery_destination_policies: HashMap<String, DeliveryDestinationPolicyView>,
    #[serde(default)]
    pub deliveries: HashMap<String, DeliveryView>,
    #[serde(default)]
    pub account_policies: HashMap<String, AccountPolicyView>,
    #[serde(default)]
    pub query_definitions: HashMap<String, QueryDefinitionView>,
    #[serde(default)]
    pub anomaly_detectors: HashMap<String, LogAnomalyDetectorView>,
    #[serde(default)]
    pub anomalies: HashMap<String, AnomalyView>,
    #[serde(default)]
    pub index_policies: HashMap<String, IndexPolicyView>,
    #[serde(default)]
    pub transformers: HashMap<String, TransformerView>,
    #[serde(default)]
    pub integrations: HashMap<String, IntegrationView>,
    #[serde(default)]
    pub import_tasks: HashMap<String, ImportTaskView>,
    #[serde(default)]
    pub scheduled_queries: HashMap<String, ScheduledQueryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogGroupView {
    pub name: String,
    pub arn: String,
    pub creation_time: i64,
    pub retention_in_days: Option<i32>,
    /// Streams, keyed by stream name — log events are included for durability.
    #[serde(default)]
    pub streams: HashMap<String, LogStreamView>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub kms_key_id: Option<String>,
    pub data_protection_policy: Option<String>,
    pub deletion_protection_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamView {
    pub name: String,
    pub arn: String,
    pub creation_time: i64,
    pub first_event_timestamp: Option<i64>,
    pub last_event_timestamp: Option<i64>,
    pub upload_sequence_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricFilterView {
    pub filter_name: String,
    pub log_group_name: String,
    pub filter_pattern: String,
    pub metric_transformations: Vec<MetricTransformationView>,
    pub creation_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricTransformationView {
    pub metric_namespace: String,
    pub metric_name: String,
    pub metric_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionFilterView {
    pub filter_name: String,
    pub log_group_name: String,
    pub filter_pattern: String,
    pub destination_arn: String,
    pub role_arn: Option<String>,
    pub creation_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcePolicyView {
    pub policy_name: String,
    pub policy_document: String,
    pub last_updated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DestinationView {
    pub destination_name: String,
    pub target_arn: String,
    pub role_arn: String,
    pub access_policy: Option<String>,
    pub arn: String,
    pub creation_time: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportTaskView {
    pub task_id: String,
    pub task_name: Option<String>,
    pub log_group_name: String,
    pub destination: String,
    pub from_time: i64,
    pub to_time: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliverySourceView {
    pub name: String,
    pub log_type: Option<String>,
    pub service: Option<String>,
    pub resource_arns: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryDestinationView {
    pub name: String,
    pub arn: String,
    pub delivery_destination_type: Option<String>,
    pub output_format: Option<String>,
    pub delivery_destination_configuration: Option<DeliveryDestinationConfigurationView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryDestinationConfigurationView {
    pub destination_resource_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryDestinationPolicyView {
    pub delivery_destination_name: String,
    pub policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeliveryView {
    pub id: String,
    pub source: String,
    pub destination: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountPolicyView {
    pub policy_name: String,
    pub policy_document: String,
    pub policy_type: String,
    pub scope: Option<String>,
    pub selection_criteria: Option<String>,
    pub account_id: String,
    pub last_updated_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDefinitionView {
    pub query_definition_id: String,
    pub name: String,
    pub query_string: String,
    pub log_group_names: Vec<String>,
    pub last_modified: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogAnomalyDetectorView {
    pub anomaly_detector_arn: String,
    pub detector_name: String,
    pub log_group_arn_list: Vec<String>,
    pub evaluation_frequency: Option<String>,
    pub filter_pattern: Option<String>,
    pub anomaly_visibility_time: Option<i64>,
    pub status: String,
    pub creation_time: i64,
    pub last_modified_time: i64,
    pub kms_key_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyView {
    pub anomaly_id: String,
    pub anomaly_detector_arn: String,
    pub description: String,
    pub first_seen: i64,
    pub last_seen: i64,
    pub state: String,
    pub suppressed: bool,
    pub suppressed_date: Option<i64>,
    pub suppressed_until: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexPolicyView {
    pub policy_name: String,
    pub log_group_identifier: String,
    pub policy_document: String,
    pub last_update_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformerView {
    pub log_group_identifier: String,
    pub transformers: Vec<serde_json::Value>,
    pub creation_time: i64,
    pub last_modified_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationView {
    pub integration_name: String,
    pub integration_type: String,
    pub resource_config: serde_json::Value,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportTaskView {
    pub task_id: String,
    pub task_name: Option<String>,
    pub log_group_name: String,
    pub from_time: i64,
    pub to_time: i64,
    pub source: serde_json::Value,
    pub status: String,
    pub creation_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledQueryView {
    pub scheduled_query_id: String,
    pub name: String,
    pub query_string: String,
    pub log_group_name: String,
    pub schedule_expression: String,
    pub status: String,
    pub creation_time: i64,
}

// ---------------------------------------------------------------------------
// From internal -> view
// ---------------------------------------------------------------------------

impl From<&LogsState> for LogsStateView {
    fn from(s: &LogsState) -> Self {
        LogsStateView {
            log_groups: s
                .log_groups
                .iter()
                .map(|(k, v)| (k.clone(), LogGroupView::from(v)))
                .collect(),
            metric_filters: s
                .metric_filters
                .iter()
                .map(|(k, v)| (k.clone(), MetricFilterView::from(v)))
                .collect(),
            subscription_filters: s
                .subscription_filters
                .iter()
                .map(|(k, v)| (k.clone(), SubscriptionFilterView::from(v)))
                .collect(),
            resource_policies: s
                .resource_policies
                .iter()
                .map(|(k, v)| (k.clone(), ResourcePolicyView::from(v)))
                .collect(),
            destinations: s
                .destinations
                .iter()
                .map(|(k, v)| (k.clone(), DestinationView::from(v)))
                .collect(),
            export_tasks: s
                .export_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ExportTaskView::from(v)))
                .collect(),
            delivery_sources: s
                .delivery_sources
                .iter()
                .map(|(k, v)| (k.clone(), DeliverySourceView::from(v)))
                .collect(),
            delivery_destinations: s
                .delivery_destinations
                .iter()
                .map(|(k, v)| (k.clone(), DeliveryDestinationView::from(v)))
                .collect(),
            delivery_destination_policies: s
                .delivery_destination_policies
                .iter()
                .map(|(k, v)| (k.clone(), DeliveryDestinationPolicyView::from(v)))
                .collect(),
            deliveries: s
                .deliveries
                .iter()
                .map(|(k, v)| (k.clone(), DeliveryView::from(v)))
                .collect(),
            account_policies: s
                .account_policies
                .iter()
                .map(|(k, v)| (k.clone(), AccountPolicyView::from(v)))
                .collect(),
            query_definitions: s
                .query_definitions
                .iter()
                .map(|(k, v)| (k.clone(), QueryDefinitionView::from(v)))
                .collect(),
            anomaly_detectors: s
                .anomaly_detectors
                .iter()
                .map(|(k, v)| (k.clone(), LogAnomalyDetectorView::from(v)))
                .collect(),
            anomalies: s
                .anomalies
                .iter()
                .map(|(k, v)| (k.clone(), AnomalyView::from(v)))
                .collect(),
            index_policies: s
                .index_policies
                .iter()
                .map(|(k, v)| (k.clone(), IndexPolicyView::from(v)))
                .collect(),
            transformers: s
                .transformers
                .iter()
                .map(|(k, v)| (k.clone(), TransformerView::from(v)))
                .collect(),
            integrations: s
                .integrations
                .iter()
                .map(|(k, v)| (k.clone(), IntegrationView::from(v)))
                .collect(),
            import_tasks: s
                .import_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ImportTaskView::from(v)))
                .collect(),
            scheduled_queries: s
                .scheduled_queries
                .iter()
                .map(|(k, v)| (k.clone(), ScheduledQueryView::from(v)))
                .collect(),
        }
    }
}

impl From<&LogGroup> for LogGroupView {
    fn from(g: &LogGroup) -> Self {
        LogGroupView {
            name: g.name.clone(),
            arn: g.arn.clone(),
            creation_time: g.creation_time,
            retention_in_days: g.retention_in_days,
            streams: g
                .streams
                .iter()
                .map(|(k, v)| (k.clone(), LogStreamView::from(v)))
                .collect(),
            tags: g.tags.clone(),
            kms_key_id: g.kms_key_id.clone(),
            data_protection_policy: g.data_protection_policy.clone(),
            deletion_protection_enabled: g.deletion_protection_enabled,
        }
    }
}

impl From<&LogStream> for LogStreamView {
    fn from(s: &LogStream) -> Self {
        LogStreamView {
            name: s.name.clone(),
            arn: s.arn.clone(),
            creation_time: s.creation_time,
            first_event_timestamp: s.first_event_timestamp,
            last_event_timestamp: s.last_event_timestamp,
            upload_sequence_token: s.upload_sequence_token.clone(),
        }
    }
}

impl From<&MetricFilter> for MetricFilterView {
    fn from(f: &MetricFilter) -> Self {
        MetricFilterView {
            filter_name: f.filter_name.clone(),
            log_group_name: f.log_group_name.clone(),
            filter_pattern: f.filter_pattern.clone(),
            metric_transformations: f
                .metric_transformations
                .iter()
                .map(MetricTransformationView::from)
                .collect(),
            creation_time: f.creation_time,
        }
    }
}

impl From<&MetricTransformation> for MetricTransformationView {
    fn from(t: &MetricTransformation) -> Self {
        MetricTransformationView {
            metric_namespace: t.metric_namespace.clone(),
            metric_name: t.metric_name.clone(),
            metric_value: t.metric_value.clone(),
        }
    }
}

impl From<&SubscriptionFilter> for SubscriptionFilterView {
    fn from(f: &SubscriptionFilter) -> Self {
        SubscriptionFilterView {
            filter_name: f.filter_name.clone(),
            log_group_name: f.log_group_name.clone(),
            filter_pattern: f.filter_pattern.clone(),
            destination_arn: f.destination_arn.clone(),
            role_arn: f.role_arn.clone(),
            creation_time: f.creation_time,
        }
    }
}

impl From<&ResourcePolicy> for ResourcePolicyView {
    fn from(p: &ResourcePolicy) -> Self {
        ResourcePolicyView {
            policy_name: p.policy_name.clone(),
            policy_document: p.policy_document.clone(),
            last_updated_time: p.last_updated_time,
        }
    }
}

impl From<&Destination> for DestinationView {
    fn from(d: &Destination) -> Self {
        DestinationView {
            destination_name: d.destination_name.clone(),
            target_arn: d.target_arn.clone(),
            role_arn: d.role_arn.clone(),
            access_policy: d.access_policy.clone(),
            arn: d.arn.clone(),
            creation_time: d.creation_time,
            tags: d.tags.clone(),
        }
    }
}

impl From<&ExportTask> for ExportTaskView {
    fn from(t: &ExportTask) -> Self {
        ExportTaskView {
            task_id: t.task_id.clone(),
            task_name: t.task_name.clone(),
            log_group_name: t.log_group_name.clone(),
            destination: t.destination.clone(),
            from_time: t.from_time,
            to_time: t.to_time,
            status: t.status.clone(),
        }
    }
}

impl From<&DeliverySource> for DeliverySourceView {
    fn from(s: &DeliverySource) -> Self {
        DeliverySourceView {
            name: s.name.clone(),
            log_type: s.log_type.clone(),
            service: s.service.clone(),
            resource_arns: s.resource_arns.clone(),
        }
    }
}

impl From<&DeliveryDestination> for DeliveryDestinationView {
    fn from(d: &DeliveryDestination) -> Self {
        DeliveryDestinationView {
            name: d.name.clone(),
            arn: d.arn.clone(),
            delivery_destination_type: d.delivery_destination_type.clone(),
            output_format: d.output_format.clone(),
            delivery_destination_configuration: d
                .delivery_destination_configuration
                .as_ref()
                .map(DeliveryDestinationConfigurationView::from),
        }
    }
}

impl From<&DeliveryDestinationConfiguration> for DeliveryDestinationConfigurationView {
    fn from(c: &DeliveryDestinationConfiguration) -> Self {
        DeliveryDestinationConfigurationView {
            destination_resource_arn: c.destination_resource_arn.clone(),
        }
    }
}

impl From<&DeliveryDestinationPolicy> for DeliveryDestinationPolicyView {
    fn from(p: &DeliveryDestinationPolicy) -> Self {
        DeliveryDestinationPolicyView {
            delivery_destination_name: p.delivery_destination_name.clone(),
            policy: p.policy.clone(),
        }
    }
}

impl From<&Delivery> for DeliveryView {
    fn from(d: &Delivery) -> Self {
        DeliveryView {
            id: d.id.clone(),
            source: d.source.clone(),
            destination: d.destination.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<&AccountPolicy> for AccountPolicyView {
    fn from(p: &AccountPolicy) -> Self {
        AccountPolicyView {
            policy_name: p.policy_name.clone(),
            policy_document: p.policy_document.clone(),
            policy_type: p.policy_type.clone(),
            scope: p.scope.clone(),
            selection_criteria: p.selection_criteria.clone(),
            account_id: p.account_id.clone(),
            last_updated_time: p.last_updated_time,
        }
    }
}

impl From<&QueryDefinition> for QueryDefinitionView {
    fn from(q: &QueryDefinition) -> Self {
        QueryDefinitionView {
            query_definition_id: q.query_definition_id.clone(),
            name: q.name.clone(),
            query_string: q.query_string.clone(),
            log_group_names: q.log_group_names.clone(),
            last_modified: q.last_modified,
        }
    }
}

impl From<&LogAnomalyDetector> for LogAnomalyDetectorView {
    fn from(d: &LogAnomalyDetector) -> Self {
        LogAnomalyDetectorView {
            anomaly_detector_arn: d.anomaly_detector_arn.clone(),
            detector_name: d.detector_name.clone(),
            log_group_arn_list: d.log_group_arn_list.clone(),
            evaluation_frequency: d.evaluation_frequency.clone(),
            filter_pattern: d.filter_pattern.clone(),
            anomaly_visibility_time: d.anomaly_visibility_time,
            status: d.status.clone(),
            creation_time: d.creation_time,
            last_modified_time: d.last_modified_time,
            kms_key_id: d.kms_key_id.clone(),
        }
    }
}

impl From<&Anomaly> for AnomalyView {
    fn from(a: &Anomaly) -> Self {
        AnomalyView {
            anomaly_id: a.anomaly_id.clone(),
            anomaly_detector_arn: a.anomaly_detector_arn.clone(),
            description: a.description.clone(),
            first_seen: a.first_seen,
            last_seen: a.last_seen,
            state: a.state.clone(),
            suppressed: a.suppressed,
            suppressed_date: a.suppressed_date,
            suppressed_until: a.suppressed_until,
        }
    }
}

impl From<&IndexPolicy> for IndexPolicyView {
    fn from(p: &IndexPolicy) -> Self {
        IndexPolicyView {
            policy_name: p.policy_name.clone(),
            log_group_identifier: p.log_group_identifier.clone(),
            policy_document: p.policy_document.clone(),
            last_update_time: p.last_update_time,
        }
    }
}

impl From<&Transformer> for TransformerView {
    fn from(t: &Transformer) -> Self {
        TransformerView {
            log_group_identifier: t.log_group_identifier.clone(),
            transformers: t.transformers.clone(),
            creation_time: t.creation_time,
            last_modified_time: t.last_modified_time,
        }
    }
}

impl From<&Integration> for IntegrationView {
    fn from(i: &Integration) -> Self {
        IntegrationView {
            integration_name: i.integration_name.clone(),
            integration_type: i.integration_type.clone(),
            resource_config: i.resource_config.clone(),
            status: i.status.clone(),
        }
    }
}

impl From<&ImportTask> for ImportTaskView {
    fn from(t: &ImportTask) -> Self {
        ImportTaskView {
            task_id: t.task_id.clone(),
            task_name: t.task_name.clone(),
            log_group_name: t.log_group_name.clone(),
            from_time: t.from_time,
            to_time: t.to_time,
            source: t.source.clone(),
            status: t.status.clone(),
            creation_time: t.creation_time,
        }
    }
}

impl From<&ScheduledQuery> for ScheduledQueryView {
    fn from(q: &ScheduledQuery) -> Self {
        ScheduledQueryView {
            scheduled_query_id: q.scheduled_query_id.clone(),
            name: q.name.clone(),
            query_string: q.query_string.clone(),
            log_group_name: q.log_group_name.clone(),
            schedule_expression: q.schedule_expression.clone(),
            status: q.status.clone(),
            creation_time: q.creation_time,
        }
    }
}

// ---------------------------------------------------------------------------
// From view -> internal
// ---------------------------------------------------------------------------

impl From<LogsStateView> for LogsState {
    fn from(v: LogsStateView) -> Self {
        LogsState {
            log_groups: v
                .log_groups
                .into_iter()
                .map(|(k, g)| (k, LogGroup::from(g)))
                .collect(),
            metric_filters: v
                .metric_filters
                .into_iter()
                .map(|(k, f)| (k, MetricFilter::from(f)))
                .collect(),
            subscription_filters: v
                .subscription_filters
                .into_iter()
                .map(|(k, f)| (k, SubscriptionFilter::from(f)))
                .collect(),
            resource_policies: v
                .resource_policies
                .into_iter()
                .map(|(k, p)| (k, ResourcePolicy::from(p)))
                .collect(),
            destinations: v
                .destinations
                .into_iter()
                .map(|(k, d)| (k, Destination::from(d)))
                .collect(),
            export_tasks: v
                .export_tasks
                .into_iter()
                .map(|(k, t)| (k, ExportTask::from(t)))
                .collect(),
            queries: HashMap::new(),
            delivery_sources: v
                .delivery_sources
                .into_iter()
                .map(|(k, s)| (k, DeliverySource::from(s)))
                .collect(),
            delivery_destinations: v
                .delivery_destinations
                .into_iter()
                .map(|(k, d)| (k, DeliveryDestination::from(d)))
                .collect(),
            delivery_destination_policies: v
                .delivery_destination_policies
                .into_iter()
                .map(|(k, p)| (k, DeliveryDestinationPolicy::from(p)))
                .collect(),
            deliveries: v
                .deliveries
                .into_iter()
                .map(|(k, d)| (k, Delivery::from(d)))
                .collect(),
            account_policies: v
                .account_policies
                .into_iter()
                .map(|(k, p)| (k, AccountPolicy::from(p)))
                .collect(),
            query_definitions: v
                .query_definitions
                .into_iter()
                .map(|(k, q)| (k, QueryDefinition::from(q)))
                .collect(),
            anomaly_detectors: v
                .anomaly_detectors
                .into_iter()
                .map(|(k, d)| (k, LogAnomalyDetector::from(d)))
                .collect(),
            anomalies: v
                .anomalies
                .into_iter()
                .map(|(k, a)| (k, Anomaly::from(a)))
                .collect(),
            index_policies: v
                .index_policies
                .into_iter()
                .map(|(k, p)| (k, IndexPolicy::from(p)))
                .collect(),
            transformers: v
                .transformers
                .into_iter()
                .map(|(k, t)| (k, Transformer::from(t)))
                .collect(),
            integrations: v
                .integrations
                .into_iter()
                .map(|(k, i)| (k, Integration::from(i)))
                .collect(),
            import_tasks: v
                .import_tasks
                .into_iter()
                .map(|(k, t)| (k, ImportTask::from(t)))
                .collect(),
            scheduled_queries: v
                .scheduled_queries
                .into_iter()
                .map(|(k, q)| (k, ScheduledQuery::from(q)))
                .collect(),
        }
    }
}

impl From<LogGroupView> for LogGroup {
    fn from(v: LogGroupView) -> Self {
        LogGroup {
            name: v.name,
            arn: v.arn,
            creation_time: v.creation_time,
            retention_in_days: v.retention_in_days,
            streams: v
                .streams
                .into_iter()
                .map(|(k, s)| (k, LogStream::from(s)))
                .collect(),
            tags: v.tags,
            kms_key_id: v.kms_key_id,
            data_protection_policy: v.data_protection_policy,
            deletion_protection_enabled: v.deletion_protection_enabled,
        }
    }
}

impl From<LogStreamView> for LogStream {
    fn from(v: LogStreamView) -> Self {
        LogStream {
            name: v.name,
            arn: v.arn,
            creation_time: v.creation_time,
            events: Vec::new(),
            first_event_timestamp: v.first_event_timestamp,
            last_event_timestamp: v.last_event_timestamp,
            upload_sequence_token: v.upload_sequence_token,
        }
    }
}

impl From<MetricFilterView> for MetricFilter {
    fn from(v: MetricFilterView) -> Self {
        MetricFilter {
            filter_name: v.filter_name,
            log_group_name: v.log_group_name,
            filter_pattern: v.filter_pattern,
            metric_transformations: v
                .metric_transformations
                .into_iter()
                .map(MetricTransformation::from)
                .collect(),
            creation_time: v.creation_time,
        }
    }
}

impl From<MetricTransformationView> for MetricTransformation {
    fn from(v: MetricTransformationView) -> Self {
        MetricTransformation {
            metric_namespace: v.metric_namespace,
            metric_name: v.metric_name,
            metric_value: v.metric_value,
        }
    }
}

impl From<SubscriptionFilterView> for SubscriptionFilter {
    fn from(v: SubscriptionFilterView) -> Self {
        SubscriptionFilter {
            filter_name: v.filter_name,
            log_group_name: v.log_group_name,
            filter_pattern: v.filter_pattern,
            destination_arn: v.destination_arn,
            role_arn: v.role_arn,
            creation_time: v.creation_time,
        }
    }
}

impl From<ResourcePolicyView> for ResourcePolicy {
    fn from(v: ResourcePolicyView) -> Self {
        ResourcePolicy {
            policy_name: v.policy_name,
            policy_document: v.policy_document,
            last_updated_time: v.last_updated_time,
        }
    }
}

impl From<DestinationView> for Destination {
    fn from(v: DestinationView) -> Self {
        Destination {
            destination_name: v.destination_name,
            target_arn: v.target_arn,
            role_arn: v.role_arn,
            access_policy: v.access_policy,
            arn: v.arn,
            creation_time: v.creation_time,
            tags: v.tags,
        }
    }
}

impl From<ExportTaskView> for ExportTask {
    fn from(v: ExportTaskView) -> Self {
        ExportTask {
            task_id: v.task_id,
            task_name: v.task_name,
            log_group_name: v.log_group_name,
            destination: v.destination,
            from_time: v.from_time,
            to_time: v.to_time,
            status: v.status,
        }
    }
}

impl From<DeliverySourceView> for DeliverySource {
    fn from(v: DeliverySourceView) -> Self {
        DeliverySource {
            name: v.name,
            log_type: v.log_type,
            service: v.service,
            resource_arns: v.resource_arns,
        }
    }
}

impl From<DeliveryDestinationView> for DeliveryDestination {
    fn from(v: DeliveryDestinationView) -> Self {
        DeliveryDestination {
            name: v.name,
            arn: v.arn,
            delivery_destination_type: v.delivery_destination_type,
            output_format: v.output_format,
            delivery_destination_configuration: v
                .delivery_destination_configuration
                .map(DeliveryDestinationConfiguration::from),
        }
    }
}

impl From<DeliveryDestinationConfigurationView> for DeliveryDestinationConfiguration {
    fn from(v: DeliveryDestinationConfigurationView) -> Self {
        DeliveryDestinationConfiguration {
            destination_resource_arn: v.destination_resource_arn,
        }
    }
}

impl From<DeliveryDestinationPolicyView> for DeliveryDestinationPolicy {
    fn from(v: DeliveryDestinationPolicyView) -> Self {
        DeliveryDestinationPolicy {
            delivery_destination_name: v.delivery_destination_name,
            policy: v.policy,
        }
    }
}

impl From<DeliveryView> for Delivery {
    fn from(v: DeliveryView) -> Self {
        Delivery {
            id: v.id,
            source: v.source,
            destination: v.destination,
            tags: v.tags,
        }
    }
}

impl From<AccountPolicyView> for AccountPolicy {
    fn from(v: AccountPolicyView) -> Self {
        AccountPolicy {
            policy_name: v.policy_name,
            policy_document: v.policy_document,
            policy_type: v.policy_type,
            scope: v.scope,
            selection_criteria: v.selection_criteria,
            account_id: v.account_id,
            last_updated_time: v.last_updated_time,
        }
    }
}

impl From<QueryDefinitionView> for QueryDefinition {
    fn from(v: QueryDefinitionView) -> Self {
        QueryDefinition {
            query_definition_id: v.query_definition_id,
            name: v.name,
            query_string: v.query_string,
            log_group_names: v.log_group_names,
            last_modified: v.last_modified,
        }
    }
}

impl From<LogAnomalyDetectorView> for LogAnomalyDetector {
    fn from(v: LogAnomalyDetectorView) -> Self {
        LogAnomalyDetector {
            anomaly_detector_arn: v.anomaly_detector_arn,
            detector_name: v.detector_name,
            log_group_arn_list: v.log_group_arn_list,
            evaluation_frequency: v.evaluation_frequency,
            filter_pattern: v.filter_pattern,
            anomaly_visibility_time: v.anomaly_visibility_time,
            status: v.status,
            creation_time: v.creation_time,
            last_modified_time: v.last_modified_time,
            kms_key_id: v.kms_key_id,
        }
    }
}

impl From<AnomalyView> for Anomaly {
    fn from(v: AnomalyView) -> Self {
        Anomaly {
            anomaly_id: v.anomaly_id,
            anomaly_detector_arn: v.anomaly_detector_arn,
            description: v.description,
            first_seen: v.first_seen,
            last_seen: v.last_seen,
            state: v.state,
            suppressed: v.suppressed,
            suppressed_date: v.suppressed_date,
            suppressed_until: v.suppressed_until,
        }
    }
}

impl From<IndexPolicyView> for IndexPolicy {
    fn from(v: IndexPolicyView) -> Self {
        IndexPolicy {
            policy_name: v.policy_name,
            log_group_identifier: v.log_group_identifier,
            policy_document: v.policy_document,
            last_update_time: v.last_update_time,
        }
    }
}

impl From<TransformerView> for Transformer {
    fn from(v: TransformerView) -> Self {
        Transformer {
            log_group_identifier: v.log_group_identifier,
            transformers: v.transformers,
            creation_time: v.creation_time,
            last_modified_time: v.last_modified_time,
        }
    }
}

impl From<IntegrationView> for Integration {
    fn from(v: IntegrationView) -> Self {
        Integration {
            integration_name: v.integration_name,
            integration_type: v.integration_type,
            resource_config: v.resource_config,
            status: v.status,
        }
    }
}

impl From<ImportTaskView> for ImportTask {
    fn from(v: ImportTaskView) -> Self {
        ImportTask {
            task_id: v.task_id,
            task_name: v.task_name,
            log_group_name: v.log_group_name,
            from_time: v.from_time,
            to_time: v.to_time,
            source: v.source,
            status: v.status,
            creation_time: v.creation_time,
        }
    }
}

impl From<ScheduledQueryView> for ScheduledQuery {
    fn from(v: ScheduledQueryView) -> Self {
        ScheduledQuery {
            scheduled_query_id: v.scheduled_query_id,
            name: v.name,
            query_string: v.query_string,
            log_group_name: v.log_group_name,
            schedule_expression: v.schedule_expression,
            status: v.status,
            creation_time: v.creation_time,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for CloudWatchLogsService {
    type StateView = LogsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        LogsStateView::from(&*guard)
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
            *guard = LogsState::from(view);
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
            for (k, v) in view.log_groups {
                guard.log_groups.insert(k, LogGroup::from(v));
            }
            for (k, v) in view.metric_filters {
                guard.metric_filters.insert(k, MetricFilter::from(v));
            }
            for (k, v) in view.subscription_filters {
                guard
                    .subscription_filters
                    .insert(k, SubscriptionFilter::from(v));
            }
            for (k, v) in view.resource_policies {
                guard.resource_policies.insert(k, ResourcePolicy::from(v));
            }
            for (k, v) in view.destinations {
                guard.destinations.insert(k, Destination::from(v));
            }
            for (k, v) in view.export_tasks {
                guard.export_tasks.insert(k, ExportTask::from(v));
            }
            for (k, v) in view.delivery_sources {
                guard.delivery_sources.insert(k, DeliverySource::from(v));
            }
            for (k, v) in view.delivery_destinations {
                guard
                    .delivery_destinations
                    .insert(k, DeliveryDestination::from(v));
            }
            for (k, v) in view.delivery_destination_policies {
                guard
                    .delivery_destination_policies
                    .insert(k, DeliveryDestinationPolicy::from(v));
            }
            for (k, v) in view.deliveries {
                guard.deliveries.insert(k, Delivery::from(v));
            }
            for (k, v) in view.account_policies {
                guard.account_policies.insert(k, AccountPolicy::from(v));
            }
            for (k, v) in view.query_definitions {
                guard.query_definitions.insert(k, QueryDefinition::from(v));
            }
            for (k, v) in view.anomaly_detectors {
                guard
                    .anomaly_detectors
                    .insert(k, LogAnomalyDetector::from(v));
            }
            for (k, v) in view.anomalies {
                guard.anomalies.insert(k, Anomaly::from(v));
            }
            for (k, v) in view.index_policies {
                guard.index_policies.insert(k, IndexPolicy::from(v));
            }
            for (k, v) in view.transformers {
                guard.transformers.insert(k, Transformer::from(v));
            }
            for (k, v) in view.integrations {
                guard.integrations.insert(k, Integration::from(v));
            }
            for (k, v) in view.import_tasks {
                guard.import_tasks.insert(k, ImportTask::from(v));
            }
            for (k, v) in view.scheduled_queries {
                guard.scheduled_queries.insert(k, ScheduledQuery::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
