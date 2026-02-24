//! Serde-compatible view types for DMS state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::DatabaseMigrationService;
use crate::state::DmsState;
use crate::types::{
    Certificate, Connection, Endpoint, EventSubscription, ReplicationInstance,
    ReplicationSubnetGroup, ReplicationTask,
};

/// Serializable view of the entire DMS state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DmsStateView {
    #[serde(default)]
    pub replication_instances: HashMap<String, ReplicationInstanceView>,
    #[serde(default)]
    pub endpoints: HashMap<String, EndpointView>,
    #[serde(default)]
    pub replication_tasks: HashMap<String, ReplicationTaskView>,
    /// Connections keyed by `<replication_instance_arn>:<endpoint_arn>`.
    #[serde(default)]
    pub connections: HashMap<String, ConnectionView>,
    #[serde(default)]
    pub replication_subnet_groups: HashMap<String, ReplicationSubnetGroupView>,
    #[serde(default)]
    pub certificates: HashMap<String, CertificateView>,
    #[serde(default)]
    pub event_subscriptions: HashMap<String, EventSubscriptionView>,
    /// Resource ARN -> tags
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub account_quotas: Vec<crate::model::AccountQuota>,
    #[serde(default)]
    pub event_category_groups: Vec<crate::model::EventCategoryGroup>,
    #[serde(default)]
    pub orderable_replication_instances: Vec<crate::model::OrderableReplicationInstance>,
    #[serde(default)]
    pub engine_versions: Vec<crate::model::EngineVersion>,
    #[serde(default)]
    pub supported_endpoint_types: Vec<crate::model::SupportedEndpointType>,
    #[serde(default)]
    pub endpoint_settings: Vec<crate::model::EndpointSetting>,
    #[serde(default)]
    pub events: Vec<crate::model::Event>,
    #[serde(default)]
    pub pending_maintenance_actions: Vec<crate::model::ResourcePendingMaintenanceActions>,
    #[serde(default)]
    pub individual_assessment_names: Vec<String>,
    #[serde(default)]
    pub replication_task_assessment_results: Vec<crate::model::ReplicationTaskAssessmentResult>,
    #[serde(default)]
    pub replication_task_assessment_runs: Vec<crate::model::ReplicationTaskAssessmentRun>,
    #[serde(default)]
    pub replication_task_individual_assessments:
        Vec<crate::model::ReplicationTaskIndividualAssessment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationInstanceView {
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
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointView {
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
    #[serde(default)]
    pub tags: HashMap<String, String>,
    /// `s3_settings` nested block.
    #[serde(default)]
    pub s3_settings: Option<serde_json::Value>,
    /// `kafka_settings` nested block.
    #[serde(default)]
    pub kafka_settings: Option<serde_json::Value>,
    /// `kinesis_settings` nested block.
    #[serde(default)]
    pub kinesis_settings: Option<serde_json::Value>,
    /// `mongodb_settings` nested block.
    #[serde(default)]
    pub mongodb_settings: Option<serde_json::Value>,
    /// `elasticsearch_settings` nested block.
    #[serde(default)]
    pub elasticsearch_settings: Option<serde_json::Value>,
    /// `redis_settings` nested block.
    #[serde(default)]
    pub redis_settings: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationTaskView {
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
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionView {
    pub replication_instance_arn: String,
    pub endpoint_arn: String,
    pub replication_instance_identifier: String,
    pub endpoint_identifier: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationSubnetGroupView {
    pub replication_subnet_group_identifier: String,
    pub replication_subnet_group_description: String,
    pub vpc_id: Option<String>,
    pub subnet_ids: Vec<String>,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateView {
    pub certificate_identifier: String,
    pub certificate_arn: String,
    pub certificate_pem: Option<String>,
    pub certificate_wallet: Option<String>,
    pub kms_key_id: Option<String>,
    pub certificate_creation_date: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSubscriptionView {
    pub subscription_name: String,
    pub sns_topic_arn: String,
    pub source_type: Option<String>,
    #[serde(default)]
    pub event_categories: Vec<String>,
    #[serde(default)]
    pub source_ids: Vec<String>,
    pub enabled: bool,
    pub status: String,
    pub subscription_creation_time: String,
    pub customer_aws_id: String,
}

impl From<&DmsState> for DmsStateView {
    fn from(state: &DmsState) -> Self {
        Self {
            replication_instances: state
                .replication_instances
                .iter()
                .map(|(k, v)| (k.clone(), ReplicationInstanceView::from(v)))
                .collect(),
            endpoints: state
                .endpoints
                .iter()
                .map(|(k, v)| (k.clone(), EndpointView::from(v)))
                .collect(),
            replication_tasks: state
                .replication_tasks
                .iter()
                .map(|(k, v)| (k.clone(), ReplicationTaskView::from(v)))
                .collect(),
            connections: state
                .connections
                .iter()
                .map(|(k, v)| (k.clone(), ConnectionView::from(v)))
                .collect(),
            replication_subnet_groups: state
                .replication_subnet_groups
                .iter()
                .map(|(k, v)| (k.clone(), ReplicationSubnetGroupView::from(v)))
                .collect(),
            certificates: state
                .certificates
                .iter()
                .map(|(k, v)| (k.clone(), CertificateView::from(v)))
                .collect(),
            event_subscriptions: state
                .event_subscriptions
                .iter()
                .map(|(k, v)| (k.clone(), EventSubscriptionView::from(v)))
                .collect(),
            tags: state.tags.clone(),
            account_quotas: state.account_quotas.clone(),
            event_category_groups: state.event_category_groups.clone(),
            orderable_replication_instances: state.orderable_replication_instances.clone(),
            engine_versions: state.engine_versions.clone(),
            supported_endpoint_types: state.supported_endpoint_types.clone(),
            endpoint_settings: state.endpoint_settings.clone(),
            events: state.events.clone(),
            pending_maintenance_actions: state.pending_maintenance_actions.clone(),
            individual_assessment_names: state.individual_assessment_names.clone(),
            replication_task_assessment_results: state.replication_task_assessment_results.clone(),
            replication_task_assessment_runs: state.replication_task_assessment_runs.clone(),
            replication_task_individual_assessments: state
                .replication_task_individual_assessments
                .clone(),
        }
    }
}

impl From<&ReplicationInstance> for ReplicationInstanceView {
    fn from(i: &ReplicationInstance) -> Self {
        Self {
            replication_instance_identifier: i.replication_instance_identifier.clone(),
            replication_instance_class: i.replication_instance_class.clone(),
            allocated_storage: i.allocated_storage,
            status: i.status.clone(),
            replication_instance_arn: i.replication_instance_arn.clone(),
            availability_zone: i.availability_zone.clone(),
            publicly_accessible: i.publicly_accessible,
            multi_az: i.multi_az,
            engine_version: i.engine_version.clone(),
            instance_create_time: i.instance_create_time,
            tags: i.tags.clone(),
        }
    }
}

impl From<&Endpoint> for EndpointView {
    fn from(e: &Endpoint) -> Self {
        Self {
            endpoint_identifier: e.endpoint_identifier.clone(),
            endpoint_type: e.endpoint_type.clone(),
            engine_name: e.engine_name.clone(),
            username: e.username.clone(),
            server_name: e.server_name.clone(),
            port: e.port,
            database_name: e.database_name.clone(),
            status: e.status.clone(),
            endpoint_arn: e.endpoint_arn.clone(),
            extra_connection_attributes: e.extra_connection_attributes.clone(),
            tags: e.tags.clone(),
            s3_settings: None,
            kafka_settings: None,
            kinesis_settings: None,
            mongodb_settings: None,
            elasticsearch_settings: None,
            redis_settings: None,
        }
    }
}

impl From<&ReplicationTask> for ReplicationTaskView {
    fn from(t: &ReplicationTask) -> Self {
        Self {
            replication_task_identifier: t.replication_task_identifier.clone(),
            source_endpoint_arn: t.source_endpoint_arn.clone(),
            target_endpoint_arn: t.target_endpoint_arn.clone(),
            replication_instance_arn: t.replication_instance_arn.clone(),
            migration_type: t.migration_type.clone(),
            table_mappings: t.table_mappings.clone(),
            replication_task_settings: t.replication_task_settings.clone(),
            status: t.status.clone(),
            replication_task_arn: t.replication_task_arn.clone(),
            replication_task_creation_date: t.replication_task_creation_date,
            replication_task_start_date: t.replication_task_start_date,
            tags: t.tags.clone(),
        }
    }
}

impl From<&ReplicationSubnetGroup> for ReplicationSubnetGroupView {
    fn from(g: &ReplicationSubnetGroup) -> Self {
        Self {
            replication_subnet_group_identifier: g.replication_subnet_group_identifier.clone(),
            replication_subnet_group_description: g.replication_subnet_group_description.clone(),
            vpc_id: g.vpc_id.clone(),
            subnet_ids: g.subnet_ids.clone(),
            tags: g.tags.clone(),
        }
    }
}

impl From<DmsStateView> for DmsState {
    fn from(view: DmsStateView) -> Self {
        Self {
            replication_instances: view
                .replication_instances
                .into_iter()
                .map(|(k, v)| (k, ReplicationInstance::from(v)))
                .collect(),
            endpoints: view
                .endpoints
                .into_iter()
                .map(|(k, v)| (k, Endpoint::from(v)))
                .collect(),
            replication_tasks: view
                .replication_tasks
                .into_iter()
                .map(|(k, v)| (k, ReplicationTask::from(v)))
                .collect(),
            connections: view
                .connections
                .into_iter()
                .map(|(k, v)| (k, Connection::from(v)))
                .collect(),
            replication_subnet_groups: view
                .replication_subnet_groups
                .into_iter()
                .map(|(k, v)| (k, ReplicationSubnetGroup::from(v)))
                .collect(),
            certificates: view
                .certificates
                .into_iter()
                .map(|(k, v)| (k, Certificate::from(v)))
                .collect(),
            event_subscriptions: view
                .event_subscriptions
                .into_iter()
                .map(|(k, v)| (k, EventSubscription::from(v)))
                .collect(),
            tags: view.tags,
            account_quotas: view.account_quotas,
            event_category_groups: view.event_category_groups,
            orderable_replication_instances: view.orderable_replication_instances,
            engine_versions: view.engine_versions,
            supported_endpoint_types: view.supported_endpoint_types,
            endpoint_settings: view.endpoint_settings,
            events: view.events,
            pending_maintenance_actions: view.pending_maintenance_actions,
            individual_assessment_names: view.individual_assessment_names,
            replication_task_assessment_results: view.replication_task_assessment_results,
            replication_task_assessment_runs: view.replication_task_assessment_runs,
            replication_task_individual_assessments: view.replication_task_individual_assessments,
        }
    }
}

impl From<ReplicationInstanceView> for ReplicationInstance {
    fn from(v: ReplicationInstanceView) -> Self {
        Self {
            replication_instance_identifier: v.replication_instance_identifier,
            replication_instance_class: v.replication_instance_class,
            allocated_storage: v.allocated_storage,
            status: v.status,
            replication_instance_arn: v.replication_instance_arn,
            availability_zone: v.availability_zone,
            publicly_accessible: v.publicly_accessible,
            multi_az: v.multi_az,
            engine_version: v.engine_version,
            instance_create_time: v.instance_create_time,
            tags: v.tags,
        }
    }
}

impl From<EndpointView> for Endpoint {
    fn from(v: EndpointView) -> Self {
        Self {
            endpoint_identifier: v.endpoint_identifier,
            endpoint_type: v.endpoint_type,
            engine_name: v.engine_name,
            username: v.username,
            server_name: v.server_name,
            port: v.port,
            database_name: v.database_name,
            status: v.status,
            endpoint_arn: v.endpoint_arn,
            extra_connection_attributes: v.extra_connection_attributes,
            tags: v.tags,
        }
    }
}

impl From<ReplicationTaskView> for ReplicationTask {
    fn from(v: ReplicationTaskView) -> Self {
        Self {
            replication_task_identifier: v.replication_task_identifier,
            source_endpoint_arn: v.source_endpoint_arn,
            target_endpoint_arn: v.target_endpoint_arn,
            replication_instance_arn: v.replication_instance_arn,
            migration_type: v.migration_type,
            table_mappings: v.table_mappings,
            replication_task_settings: v.replication_task_settings,
            status: v.status,
            replication_task_arn: v.replication_task_arn,
            replication_task_creation_date: v.replication_task_creation_date,
            replication_task_start_date: v.replication_task_start_date,
            tags: v.tags,
        }
    }
}

impl From<ReplicationSubnetGroupView> for ReplicationSubnetGroup {
    fn from(v: ReplicationSubnetGroupView) -> Self {
        Self {
            replication_subnet_group_identifier: v.replication_subnet_group_identifier,
            replication_subnet_group_description: v.replication_subnet_group_description,
            vpc_id: v.vpc_id,
            subnet_ids: v.subnet_ids,
            tags: v.tags,
        }
    }
}

impl From<&Connection> for ConnectionView {
    fn from(c: &Connection) -> Self {
        Self {
            replication_instance_arn: c.replication_instance_arn.clone(),
            endpoint_arn: c.endpoint_arn.clone(),
            replication_instance_identifier: c.replication_instance_identifier.clone(),
            endpoint_identifier: c.endpoint_identifier.clone(),
            status: c.status.clone(),
        }
    }
}

impl From<ConnectionView> for Connection {
    fn from(v: ConnectionView) -> Self {
        Self {
            replication_instance_arn: v.replication_instance_arn,
            endpoint_arn: v.endpoint_arn,
            replication_instance_identifier: v.replication_instance_identifier,
            endpoint_identifier: v.endpoint_identifier,
            status: v.status,
        }
    }
}

impl From<&Certificate> for CertificateView {
    fn from(c: &Certificate) -> Self {
        Self {
            certificate_identifier: c.certificate_identifier.clone(),
            certificate_arn: c.certificate_arn.clone(),
            certificate_pem: c.certificate_pem.clone(),
            certificate_wallet: c.certificate_wallet.clone(),
            kms_key_id: c.kms_key_id.clone(),
            certificate_creation_date: c.certificate_creation_date,
        }
    }
}

impl From<CertificateView> for Certificate {
    fn from(v: CertificateView) -> Self {
        Self {
            certificate_identifier: v.certificate_identifier,
            certificate_arn: v.certificate_arn,
            certificate_pem: v.certificate_pem,
            certificate_wallet: v.certificate_wallet,
            kms_key_id: v.kms_key_id,
            certificate_creation_date: v.certificate_creation_date,
        }
    }
}

impl From<&EventSubscription> for EventSubscriptionView {
    fn from(s: &EventSubscription) -> Self {
        Self {
            subscription_name: s.subscription_name.clone(),
            sns_topic_arn: s.sns_topic_arn.clone(),
            source_type: s.source_type.clone(),
            event_categories: s.event_categories.clone(),
            source_ids: s.source_ids.clone(),
            enabled: s.enabled,
            status: s.status.clone(),
            subscription_creation_time: s.subscription_creation_time.clone(),
            customer_aws_id: s.customer_aws_id.clone(),
        }
    }
}

impl From<EventSubscriptionView> for EventSubscription {
    fn from(v: EventSubscriptionView) -> Self {
        Self {
            subscription_name: v.subscription_name,
            sns_topic_arn: v.sns_topic_arn,
            source_type: v.source_type,
            event_categories: v.event_categories,
            source_ids: v.source_ids,
            enabled: v.enabled,
            status: v.status,
            subscription_creation_time: v.subscription_creation_time,
            customer_aws_id: v.customer_aws_id,
        }
    }
}

impl StatefulService for DatabaseMigrationService {
    type StateView = DmsStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        DmsStateView::from(&*guard)
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
            *guard = DmsState::from(view);
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
            for (k, v) in view.replication_instances {
                guard
                    .replication_instances
                    .insert(k, ReplicationInstance::from(v));
            }
            for (k, v) in view.endpoints {
                guard.endpoints.insert(k, Endpoint::from(v));
            }
            for (k, v) in view.replication_tasks {
                guard.replication_tasks.insert(k, ReplicationTask::from(v));
            }
            for (k, v) in view.connections {
                guard.connections.insert(k, Connection::from(v));
            }
            for (k, v) in view.replication_subnet_groups {
                guard
                    .replication_subnet_groups
                    .insert(k, ReplicationSubnetGroup::from(v));
            }
            for (k, v) in view.certificates {
                guard.certificates.insert(k, Certificate::from(v));
            }
            for (k, v) in view.event_subscriptions {
                guard
                    .event_subscriptions
                    .insert(k, EventSubscription::from(v));
            }
            for (arn, tags) in view.tags {
                guard.tags.entry(arn).or_default().extend(tags);
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
