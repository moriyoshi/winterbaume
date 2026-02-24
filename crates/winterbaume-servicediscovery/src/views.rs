//! Serde-compatible view types for ServiceDiscovery state snapshots.

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::ServiceDiscoveryService;
use crate::state::ServiceDiscoveryState;
use crate::types::{
    DnsConfigEntry, DnsRecordEntry, HealthCheckConfigEntry, HealthCheckCustomConfigEntry,
    InstanceEntry, Namespace, Operation, ServiceEntry,
};

/// Serializable view of the entire ServiceDiscovery state.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServiceDiscoveryStateView {
    /// Namespaces keyed by namespace ID.
    #[serde(default)]
    pub namespaces: HashMap<String, NamespaceView>,
    /// Services keyed by service ID.
    #[serde(default)]
    pub services: HashMap<String, ServiceEntryView>,
    /// Instances keyed by service ID, then instance ID.
    #[serde(default)]
    pub instances: HashMap<String, HashMap<String, InstanceEntryView>>,
    /// Operations keyed by operation ID.
    #[serde(default)]
    pub operations: HashMap<String, OperationView>,
    /// Global instance revision counter.
    #[serde(default)]
    pub instances_revision: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamespaceView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace_type: String,
    pub description: Option<String>,
    pub creator_request_id: Option<String>,
    pub vpc: Option<String>,
    pub hosted_zone_id: Option<String>,
    pub soa_ttl: Option<i64>,
    pub service_count: i32,
    pub create_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperationView {
    pub id: String,
    pub operation_type: String,
    pub status: String,
    #[serde(default)]
    pub targets: HashMap<String, String>,
    pub create_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsRecordEntryView {
    pub record_type: String,
    pub ttl: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsConfigEntryView {
    pub namespace_id: Option<String>,
    pub routing_policy: Option<String>,
    pub dns_records: Vec<DnsRecordEntryView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfigEntryView {
    pub check_type: String,
    pub resource_path: Option<String>,
    pub failure_threshold: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckCustomConfigEntryView {
    pub failure_threshold: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEntryView {
    pub id: String,
    pub arn: String,
    pub name: String,
    pub namespace_id: String,
    pub description: Option<String>,
    pub creator_request_id: Option<String>,
    pub dns_config: Option<DnsConfigEntryView>,
    pub health_check_config: Option<HealthCheckConfigEntryView>,
    pub health_check_custom_config: Option<HealthCheckCustomConfigEntryView>,
    pub instance_count: i32,
    pub create_date: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
    pub service_type: Option<String>,
    pub include_namespace_id_in_response: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceEntryView {
    pub id: String,
    pub service_id: String,
    pub creator_request_id: Option<String>,
    #[serde(default)]
    pub attributes: HashMap<String, String>,
    // Health status is transient — we snapshot it but restore as HEALTHY.
    pub health_status: String,
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

impl From<&Namespace> for NamespaceView {
    fn from(ns: &Namespace) -> Self {
        Self {
            id: ns.id.clone(),
            arn: ns.arn.clone(),
            name: ns.name.clone(),
            namespace_type: ns.namespace_type.clone(),
            description: ns.description.clone(),
            creator_request_id: ns.creator_request_id.clone(),
            vpc: ns.vpc.clone(),
            hosted_zone_id: ns.hosted_zone_id.clone(),
            soa_ttl: ns.soa_ttl,
            service_count: ns.service_count,
            create_date: ns.create_date.to_rfc3339(),
            tags: ns.tags.clone(),
        }
    }
}

impl From<&Operation> for OperationView {
    fn from(op: &Operation) -> Self {
        Self {
            id: op.id.clone(),
            operation_type: op.operation_type.clone(),
            status: op.status.clone(),
            targets: op.targets.clone(),
            create_date: op.create_date.to_rfc3339(),
        }
    }
}

impl From<&DnsRecordEntry> for DnsRecordEntryView {
    fn from(r: &DnsRecordEntry) -> Self {
        Self {
            record_type: r.record_type.clone(),
            ttl: r.ttl,
        }
    }
}

impl From<&DnsConfigEntry> for DnsConfigEntryView {
    fn from(d: &DnsConfigEntry) -> Self {
        Self {
            namespace_id: d.namespace_id.clone(),
            routing_policy: d.routing_policy.clone(),
            dns_records: d.dns_records.iter().map(DnsRecordEntryView::from).collect(),
        }
    }
}

impl From<&HealthCheckConfigEntry> for HealthCheckConfigEntryView {
    fn from(h: &HealthCheckConfigEntry) -> Self {
        Self {
            check_type: h.check_type.clone(),
            resource_path: h.resource_path.clone(),
            failure_threshold: h.failure_threshold,
        }
    }
}

impl From<&HealthCheckCustomConfigEntry> for HealthCheckCustomConfigEntryView {
    fn from(h: &HealthCheckCustomConfigEntry) -> Self {
        Self {
            failure_threshold: h.failure_threshold,
        }
    }
}

impl From<&ServiceEntry> for ServiceEntryView {
    fn from(s: &ServiceEntry) -> Self {
        Self {
            id: s.id.clone(),
            arn: s.arn.clone(),
            name: s.name.clone(),
            namespace_id: s.namespace_id.clone(),
            description: s.description.clone(),
            creator_request_id: s.creator_request_id.clone(),
            dns_config: s.dns_config.as_ref().map(DnsConfigEntryView::from),
            health_check_config: s
                .health_check_config
                .as_ref()
                .map(HealthCheckConfigEntryView::from),
            health_check_custom_config: s
                .health_check_custom_config
                .as_ref()
                .map(HealthCheckCustomConfigEntryView::from),
            instance_count: s.instance_count,
            create_date: s.create_date.to_rfc3339(),
            tags: s.tags.clone(),
            service_type: s.service_type.clone(),
            include_namespace_id_in_response: s.include_namespace_id_in_response,
        }
    }
}

impl From<&InstanceEntry> for InstanceEntryView {
    fn from(i: &InstanceEntry) -> Self {
        Self {
            id: i.id.clone(),
            service_id: i.service_id.clone(),
            creator_request_id: i.creator_request_id.clone(),
            attributes: i.attributes.clone(),
            health_status: i.health_status.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for ServiceDiscoveryService {
    type StateView = ServiceDiscoveryStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        ServiceDiscoveryStateView {
            namespaces: guard
                .namespaces
                .iter()
                .map(|(k, v)| (k.clone(), NamespaceView::from(v)))
                .collect(),
            services: guard
                .services
                .iter()
                .map(|(k, v)| (k.clone(), ServiceEntryView::from(v)))
                .collect(),
            instances: guard
                .instances
                .iter()
                .map(|(svc_id, inst_map)| {
                    let views: HashMap<String, InstanceEntryView> = inst_map
                        .iter()
                        .map(|(k, v)| (k.clone(), InstanceEntryView::from(v)))
                        .collect();
                    (svc_id.clone(), views)
                })
                .collect(),
            operations: guard
                .operations
                .iter()
                .map(|(k, v)| (k.clone(), OperationView::from(v)))
                .collect(),
            instances_revision: guard.instances_revision,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let new_state = state_from_view(view);
        {
            let state = self.state.get(account_id, region);
            *state.write().await = new_state;
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
        let incoming = state_from_view(view);
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            guard.namespaces.extend(incoming.namespaces);
            guard.services.extend(incoming.services);
            guard.instances.extend(incoming.instances);
            guard.operations.extend(incoming.operations);
            guard.instances_revision = guard.instances_revision.max(incoming.instances_revision);
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}

fn parse_dt(s: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now())
}

fn state_from_view(view: ServiceDiscoveryStateView) -> ServiceDiscoveryState {
    let mut state = ServiceDiscoveryState::default();

    for (id, nv) in view.namespaces {
        state.namespaces.insert(
            id,
            Namespace {
                id: nv.id,
                arn: nv.arn,
                name: nv.name,
                namespace_type: nv.namespace_type,
                description: nv.description,
                creator_request_id: nv.creator_request_id,
                vpc: nv.vpc,
                hosted_zone_id: nv.hosted_zone_id,
                soa_ttl: nv.soa_ttl,
                service_count: nv.service_count,
                create_date: parse_dt(&nv.create_date),
                tags: nv.tags,
            },
        );
    }

    for (id, sv) in view.services {
        let dns_config = sv.dns_config.map(|d| DnsConfigEntry {
            namespace_id: d.namespace_id,
            routing_policy: d.routing_policy,
            dns_records: d
                .dns_records
                .into_iter()
                .map(|r| DnsRecordEntry {
                    record_type: r.record_type,
                    ttl: r.ttl,
                })
                .collect(),
        });
        let health_check_config = sv.health_check_config.map(|h| HealthCheckConfigEntry {
            check_type: h.check_type,
            resource_path: h.resource_path,
            failure_threshold: h.failure_threshold,
        });
        let health_check_custom_config =
            sv.health_check_custom_config
                .map(|h| HealthCheckCustomConfigEntry {
                    failure_threshold: h.failure_threshold,
                });
        state.services.insert(
            id,
            ServiceEntry {
                id: sv.id,
                arn: sv.arn,
                name: sv.name,
                namespace_id: sv.namespace_id,
                description: sv.description,
                creator_request_id: sv.creator_request_id,
                dns_config,
                health_check_config,
                health_check_custom_config,
                instance_count: sv.instance_count,
                create_date: parse_dt(&sv.create_date),
                tags: sv.tags,
                service_type: sv.service_type,
                include_namespace_id_in_response: sv.include_namespace_id_in_response,
            },
        );
    }

    for (svc_id, inst_map) in view.instances {
        let mut instances = HashMap::new();
        for (inst_id, iv) in inst_map {
            instances.insert(
                inst_id,
                InstanceEntry {
                    id: iv.id,
                    service_id: iv.service_id,
                    creator_request_id: iv.creator_request_id,
                    attributes: iv.attributes,
                    health_status: iv.health_status,
                },
            );
        }
        state.instances.insert(svc_id, instances);
    }

    for (id, ov) in view.operations {
        state.operations.insert(
            id,
            Operation {
                id: ov.id,
                operation_type: ov.operation_type,
                status: ov.status,
                targets: ov.targets,
                create_date: parse_dt(&ov.create_date),
            },
        );
    }

    state.instances_revision = view.instances_revision;

    state
}
