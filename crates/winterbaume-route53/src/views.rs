//! Serde-compatible view types for Route 53 state snapshots.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::Route53Service;
use crate::state::Route53State;
use crate::types::{
    CidrCollectionEntry, HostedZone, KeySigningKeyEntry, ResourceRecordSet, Route53DelegationSet,
    Route53HealthCheck, Route53HealthCheckConfig, Route53QueryLoggingConfig, TrafficPolicyEntry,
    TrafficPolicyInstanceEntry, Vpc, VpcAssociationAuthorization,
};

/// Serializable view of the entire Route 53 state for one account/region.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Route53StateView {
    /// Hosted zones keyed by full zone ID (`/hostedzone/{id}`).
    #[serde(default)]
    pub hosted_zones: HashMap<String, HostedZoneView>,
    /// Health checks keyed by health check ID.
    #[serde(default)]
    pub health_checks: HashMap<String, HealthCheckView>,
    /// Query logging configs keyed by config ID.
    #[serde(default)]
    pub query_logging_configs: HashMap<String, QueryLoggingConfigView>,
    /// Reusable delegation sets keyed by delegation set ID.
    #[serde(default)]
    pub reusable_delegation_sets: HashMap<String, DelegationSetView>,
    /// Deleted health check caller references.
    #[serde(default)]
    pub deleted_health_check_caller_references: Vec<String>,
    /// Key signing keys keyed by `{hosted_zone_id}:{name}`.
    #[serde(default)]
    pub key_signing_keys: HashMap<String, KeySigningKeyView>,
    /// Whether DNSSEC is enabled per hosted zone.
    #[serde(default)]
    pub dnssec_enabled: HashMap<String, bool>,
    /// CIDR collections keyed by collection ID.
    #[serde(default)]
    pub cidr_collections: HashMap<String, CidrCollectionView>,
    /// Traffic policies keyed by `{id}:{version}`.
    #[serde(default)]
    pub traffic_policies: HashMap<String, TrafficPolicyView>,
    /// Traffic policy instances keyed by instance ID.
    #[serde(default)]
    pub traffic_policy_instances: HashMap<String, TrafficPolicyInstanceView>,
    /// VPC association authorizations keyed by `{hosted_zone_id}:{vpc_id}`.
    #[serde(default)]
    pub vpc_association_authorizations: HashMap<String, VpcAssociationAuthorizationView>,
}

/// Serializable view of a hosted zone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostedZoneView {
    pub id: String,
    pub name: String,
    pub caller_reference: String,
    pub resource_record_count: u64,
    pub records: Vec<ResourceRecordSetView>,
    pub vpcs: Vec<VpcView>,
    pub comment: Option<String>,
    pub delegation_set: DelegationSetView,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a VPC association.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcView {
    pub vpc_id: String,
    pub vpc_region: String,
}

/// Serializable view of a resource record set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceRecordSetView {
    pub name: String,
    pub record_type: String,
    pub ttl: Option<u64>,
    #[serde(default)]
    pub resource_records: Vec<String>,
    /// Alias target block (`alias { name, zone_id, evaluate_target_health }`).
    #[serde(default)]
    pub alias: Option<JsonValue>,
    /// Weighted routing policy block.
    #[serde(default)]
    pub weighted_routing_policy: Option<JsonValue>,
    /// Failover routing policy block.
    #[serde(default)]
    pub failover_routing_policy: Option<JsonValue>,
    /// Geolocation routing policy block.
    #[serde(default)]
    pub geolocation_routing_policy: Option<JsonValue>,
    /// Latency routing policy block.
    #[serde(default)]
    pub latency_routing_policy: Option<JsonValue>,
    /// Multivalue answer routing policy block.
    #[serde(default)]
    pub multivalue_answer_routing_policy: Option<JsonValue>,
    /// CIDR routing policy block.
    #[serde(default)]
    pub cidr_routing_policy: Option<JsonValue>,
    /// Set identifier for routing policies.
    #[serde(default)]
    pub set_identifier: Option<String>,
    /// Health check ID.
    #[serde(default)]
    pub health_check_id: Option<String>,
}

/// Serializable view of a delegation set.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DelegationSetView {
    pub id: String,
    pub caller_reference: Option<String>,
    #[serde(default)]
    pub name_servers: Vec<String>,
}

/// Serializable view of a health check.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckView {
    pub id: String,
    pub caller_reference: String,
    pub config: HealthCheckConfigView,
    pub version: i64,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

/// Serializable view of a health check configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfigView {
    pub type_: String,
    pub ip_address: Option<String>,
    pub port: Option<i32>,
    pub resource_path: Option<String>,
    pub fully_qualified_domain_name: Option<String>,
    pub failure_threshold: Option<i32>,
    pub request_interval: Option<i32>,
    pub inverted: Option<bool>,
    pub disabled: Option<bool>,
    pub measure_latency: Option<bool>,
    pub search_string: Option<String>,
    pub enable_sni: Option<bool>,
    pub health_threshold: Option<i32>,
    pub insufficient_data_health_status: Option<String>,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(default)]
    pub child_health_checks: Vec<String>,
}

/// Serializable view of a query logging config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryLoggingConfigView {
    pub id: String,
    pub hosted_zone_id: String,
    pub cloud_watch_logs_log_group_arn: String,
}

/// Serializable view of a key signing key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeySigningKeyView {
    pub name: String,
    pub hosted_zone_id: String,
    pub kms_arn: String,
    pub status: String,
    pub created_date: String,
    pub last_modified_date: String,
}

/// Serializable view of a CIDR collection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CidrCollectionView {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub version: i64,
    #[serde(default)]
    pub locations: HashMap<String, Vec<String>>,
}

/// Serializable view of a traffic policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPolicyView {
    pub id: String,
    pub name: String,
    pub version: i32,
    pub document: String,
    pub comment: Option<String>,
    pub type_: String,
}

/// Serializable view of a traffic policy instance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrafficPolicyInstanceView {
    pub id: String,
    pub hosted_zone_id: String,
    pub name: String,
    pub ttl: i64,
    pub state: String,
    pub message: String,
    pub traffic_policy_id: String,
    pub traffic_policy_version: i32,
    pub traffic_policy_type: String,
}

/// Serializable view of a VPC association authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcAssociationAuthorizationView {
    pub hosted_zone_id: String,
    pub vpc_id: String,
    pub vpc_region: String,
}

// --- From internal types to view types ---

impl From<&Route53State> for Route53StateView {
    fn from(state: &Route53State) -> Self {
        Route53StateView {
            hosted_zones: state
                .hosted_zones
                .iter()
                .map(|(k, v)| (k.clone(), HostedZoneView::from(v)))
                .collect(),
            health_checks: state
                .health_checks
                .iter()
                .map(|(k, v)| (k.clone(), HealthCheckView::from(v)))
                .collect(),
            query_logging_configs: state
                .query_logging_configs
                .iter()
                .map(|(k, v)| (k.clone(), QueryLoggingConfigView::from(v)))
                .collect(),
            reusable_delegation_sets: state
                .reusable_delegation_sets
                .iter()
                .map(|(k, v)| (k.clone(), DelegationSetView::from(v)))
                .collect(),
            deleted_health_check_caller_references: state
                .deleted_health_check_caller_references
                .iter()
                .cloned()
                .collect(),
            key_signing_keys: state
                .key_signing_keys
                .iter()
                .map(|(k, v)| (k.clone(), KeySigningKeyView::from(v)))
                .collect(),
            dnssec_enabled: state.dnssec_enabled.clone(),
            cidr_collections: state
                .cidr_collections
                .iter()
                .map(|(k, v)| (k.clone(), CidrCollectionView::from(v)))
                .collect(),
            traffic_policies: state
                .traffic_policies
                .iter()
                .map(|(k, v)| (k.clone(), TrafficPolicyView::from(v)))
                .collect(),
            traffic_policy_instances: state
                .traffic_policy_instances
                .iter()
                .map(|(k, v)| (k.clone(), TrafficPolicyInstanceView::from(v)))
                .collect(),
            vpc_association_authorizations: state
                .vpc_association_authorizations
                .iter()
                .map(|(k, v)| (k.clone(), VpcAssociationAuthorizationView::from(v)))
                .collect(),
        }
    }
}

impl From<&HostedZone> for HostedZoneView {
    fn from(zone: &HostedZone) -> Self {
        HostedZoneView {
            id: zone.id.clone(),
            name: zone.name.clone(),
            caller_reference: zone.caller_reference.clone(),
            resource_record_count: zone.resource_record_count,
            records: zone
                .records
                .iter()
                .map(ResourceRecordSetView::from)
                .collect(),
            vpcs: zone.vpcs.iter().map(VpcView::from).collect(),
            comment: zone.comment.clone(),
            delegation_set: DelegationSetView::from(&zone.delegation_set),
            tags: zone.tags.clone(),
        }
    }
}

impl From<&Vpc> for VpcView {
    fn from(vpc: &Vpc) -> Self {
        VpcView {
            vpc_id: vpc.vpc_id.clone(),
            vpc_region: vpc.vpc_region.clone(),
        }
    }
}

impl From<&ResourceRecordSet> for ResourceRecordSetView {
    fn from(rrs: &ResourceRecordSet) -> Self {
        ResourceRecordSetView {
            name: rrs.name.clone(),
            record_type: rrs.record_type.clone(),
            ttl: rrs.ttl,
            resource_records: rrs.resource_records.clone(),
            alias: None,
            weighted_routing_policy: None,
            failover_routing_policy: None,
            geolocation_routing_policy: None,
            latency_routing_policy: None,
            multivalue_answer_routing_policy: None,
            cidr_routing_policy: None,
            set_identifier: None,
            health_check_id: None,
        }
    }
}

impl From<&Route53DelegationSet> for DelegationSetView {
    fn from(ds: &Route53DelegationSet) -> Self {
        DelegationSetView {
            id: ds.id.clone(),
            caller_reference: ds.caller_reference.clone(),
            name_servers: ds.name_servers.clone(),
        }
    }
}

impl From<&Route53HealthCheck> for HealthCheckView {
    fn from(hc: &Route53HealthCheck) -> Self {
        HealthCheckView {
            id: hc.id.clone(),
            caller_reference: hc.caller_reference.clone(),
            config: HealthCheckConfigView::from(&hc.config),
            version: hc.version,
            tags: hc.tags.clone(),
        }
    }
}

impl From<&Route53HealthCheckConfig> for HealthCheckConfigView {
    fn from(cfg: &Route53HealthCheckConfig) -> Self {
        HealthCheckConfigView {
            type_: cfg.type_.clone(),
            ip_address: cfg.ip_address.clone(),
            port: cfg.port,
            resource_path: cfg.resource_path.clone(),
            fully_qualified_domain_name: cfg.fully_qualified_domain_name.clone(),
            failure_threshold: cfg.failure_threshold,
            request_interval: cfg.request_interval,
            inverted: cfg.inverted,
            disabled: cfg.disabled,
            measure_latency: cfg.measure_latency,
            search_string: cfg.search_string.clone(),
            enable_sni: cfg.enable_sni,
            health_threshold: cfg.health_threshold,
            insufficient_data_health_status: cfg.insufficient_data_health_status.clone(),
            regions: cfg.regions.clone(),
            child_health_checks: cfg.child_health_checks.clone(),
        }
    }
}

impl From<&Route53QueryLoggingConfig> for QueryLoggingConfigView {
    fn from(cfg: &Route53QueryLoggingConfig) -> Self {
        QueryLoggingConfigView {
            id: cfg.id.clone(),
            hosted_zone_id: cfg.hosted_zone_id.clone(),
            cloud_watch_logs_log_group_arn: cfg.cloud_watch_logs_log_group_arn.clone(),
        }
    }
}

impl From<&KeySigningKeyEntry> for KeySigningKeyView {
    fn from(entry: &KeySigningKeyEntry) -> Self {
        KeySigningKeyView {
            name: entry.name.clone(),
            hosted_zone_id: entry.hosted_zone_id.clone(),
            kms_arn: entry.kms_arn.clone(),
            status: entry.status.clone(),
            created_date: entry.created_date.clone(),
            last_modified_date: entry.last_modified_date.clone(),
        }
    }
}

impl From<&CidrCollectionEntry> for CidrCollectionView {
    fn from(entry: &CidrCollectionEntry) -> Self {
        CidrCollectionView {
            id: entry.id.clone(),
            name: entry.name.clone(),
            arn: entry.arn.clone(),
            version: entry.version,
            locations: entry.locations.clone(),
        }
    }
}

impl From<&TrafficPolicyEntry> for TrafficPolicyView {
    fn from(entry: &TrafficPolicyEntry) -> Self {
        TrafficPolicyView {
            id: entry.id.clone(),
            name: entry.name.clone(),
            version: entry.version,
            document: entry.document.clone(),
            comment: entry.comment.clone(),
            type_: entry.type_.clone(),
        }
    }
}

impl From<&TrafficPolicyInstanceEntry> for TrafficPolicyInstanceView {
    fn from(entry: &TrafficPolicyInstanceEntry) -> Self {
        TrafficPolicyInstanceView {
            id: entry.id.clone(),
            hosted_zone_id: entry.hosted_zone_id.clone(),
            name: entry.name.clone(),
            ttl: entry.ttl,
            state: entry.state.clone(),
            message: entry.message.clone(),
            traffic_policy_id: entry.traffic_policy_id.clone(),
            traffic_policy_version: entry.traffic_policy_version,
            traffic_policy_type: entry.traffic_policy_type.clone(),
        }
    }
}

impl From<&VpcAssociationAuthorization> for VpcAssociationAuthorizationView {
    fn from(auth: &VpcAssociationAuthorization) -> Self {
        VpcAssociationAuthorizationView {
            hosted_zone_id: auth.hosted_zone_id.clone(),
            vpc_id: auth.vpc_id.clone(),
            vpc_region: auth.vpc_region.clone(),
        }
    }
}

// --- From view types to internal types ---

impl From<Route53StateView> for Route53State {
    fn from(view: Route53StateView) -> Self {
        Route53State {
            hosted_zones: view
                .hosted_zones
                .into_iter()
                .map(|(k, v)| (k, HostedZone::from(v)))
                .collect(),
            health_checks: view
                .health_checks
                .into_iter()
                .map(|(k, v)| (k, Route53HealthCheck::from(v)))
                .collect(),
            query_logging_configs: view
                .query_logging_configs
                .into_iter()
                .map(|(k, v)| (k, Route53QueryLoggingConfig::from(v)))
                .collect(),
            reusable_delegation_sets: view
                .reusable_delegation_sets
                .into_iter()
                .map(|(k, v)| (k, Route53DelegationSet::from(v)))
                .collect(),
            deleted_health_check_caller_references: view
                .deleted_health_check_caller_references
                .into_iter()
                .collect::<HashSet<_>>(),
            key_signing_keys: view
                .key_signing_keys
                .into_iter()
                .map(|(k, v)| (k, KeySigningKeyEntry::from(v)))
                .collect(),
            dnssec_enabled: view.dnssec_enabled,
            cidr_collections: view
                .cidr_collections
                .into_iter()
                .map(|(k, v)| (k, CidrCollectionEntry::from(v)))
                .collect(),
            traffic_policies: view
                .traffic_policies
                .into_iter()
                .map(|(k, v)| (k, TrafficPolicyEntry::from(v)))
                .collect(),
            traffic_policy_instances: view
                .traffic_policy_instances
                .into_iter()
                .map(|(k, v)| (k, TrafficPolicyInstanceEntry::from(v)))
                .collect(),
            vpc_association_authorizations: view
                .vpc_association_authorizations
                .into_iter()
                .map(|(k, v)| (k, VpcAssociationAuthorization::from(v)))
                .collect(),
        }
    }
}

impl From<HostedZoneView> for HostedZone {
    fn from(view: HostedZoneView) -> Self {
        HostedZone {
            id: view.id,
            name: view.name,
            caller_reference: view.caller_reference,
            resource_record_count: view.resource_record_count,
            records: view
                .records
                .into_iter()
                .map(ResourceRecordSet::from)
                .collect(),
            vpcs: view.vpcs.into_iter().map(Vpc::from).collect(),
            comment: view.comment,
            delegation_set: Route53DelegationSet::from(view.delegation_set),
            tags: view.tags,
        }
    }
}

impl From<VpcView> for Vpc {
    fn from(view: VpcView) -> Self {
        Vpc {
            vpc_id: view.vpc_id,
            vpc_region: view.vpc_region,
        }
    }
}

impl From<ResourceRecordSetView> for ResourceRecordSet {
    fn from(view: ResourceRecordSetView) -> Self {
        ResourceRecordSet {
            name: view.name,
            record_type: view.record_type,
            ttl: view.ttl,
            resource_records: view.resource_records,
        }
    }
}

impl From<DelegationSetView> for Route53DelegationSet {
    fn from(view: DelegationSetView) -> Self {
        Route53DelegationSet {
            id: view.id,
            caller_reference: view.caller_reference,
            name_servers: view.name_servers,
        }
    }
}

impl From<HealthCheckView> for Route53HealthCheck {
    fn from(view: HealthCheckView) -> Self {
        Route53HealthCheck {
            id: view.id,
            caller_reference: view.caller_reference,
            config: Route53HealthCheckConfig::from(view.config),
            version: view.version,
            tags: view.tags,
        }
    }
}

impl From<HealthCheckConfigView> for Route53HealthCheckConfig {
    fn from(view: HealthCheckConfigView) -> Self {
        Route53HealthCheckConfig {
            type_: view.type_,
            ip_address: view.ip_address,
            port: view.port,
            resource_path: view.resource_path,
            fully_qualified_domain_name: view.fully_qualified_domain_name,
            failure_threshold: view.failure_threshold,
            request_interval: view.request_interval,
            inverted: view.inverted,
            disabled: view.disabled,
            measure_latency: view.measure_latency,
            search_string: view.search_string,
            enable_sni: view.enable_sni,
            health_threshold: view.health_threshold,
            insufficient_data_health_status: view.insufficient_data_health_status,
            regions: view.regions,
            child_health_checks: view.child_health_checks,
        }
    }
}

impl From<QueryLoggingConfigView> for Route53QueryLoggingConfig {
    fn from(view: QueryLoggingConfigView) -> Self {
        Route53QueryLoggingConfig {
            id: view.id,
            hosted_zone_id: view.hosted_zone_id,
            cloud_watch_logs_log_group_arn: view.cloud_watch_logs_log_group_arn,
        }
    }
}

impl From<KeySigningKeyView> for KeySigningKeyEntry {
    fn from(view: KeySigningKeyView) -> Self {
        KeySigningKeyEntry {
            name: view.name,
            hosted_zone_id: view.hosted_zone_id,
            kms_arn: view.kms_arn,
            status: view.status,
            created_date: view.created_date,
            last_modified_date: view.last_modified_date,
        }
    }
}

impl From<CidrCollectionView> for CidrCollectionEntry {
    fn from(view: CidrCollectionView) -> Self {
        CidrCollectionEntry {
            id: view.id,
            name: view.name,
            arn: view.arn,
            version: view.version,
            locations: view.locations,
        }
    }
}

impl From<TrafficPolicyView> for TrafficPolicyEntry {
    fn from(view: TrafficPolicyView) -> Self {
        TrafficPolicyEntry {
            id: view.id,
            name: view.name,
            version: view.version,
            document: view.document,
            comment: view.comment,
            type_: view.type_,
        }
    }
}

impl From<TrafficPolicyInstanceView> for TrafficPolicyInstanceEntry {
    fn from(view: TrafficPolicyInstanceView) -> Self {
        TrafficPolicyInstanceEntry {
            id: view.id,
            hosted_zone_id: view.hosted_zone_id,
            name: view.name,
            ttl: view.ttl,
            state: view.state,
            message: view.message,
            traffic_policy_id: view.traffic_policy_id,
            traffic_policy_version: view.traffic_policy_version,
            traffic_policy_type: view.traffic_policy_type,
        }
    }
}

impl From<VpcAssociationAuthorizationView> for VpcAssociationAuthorization {
    fn from(view: VpcAssociationAuthorizationView) -> Self {
        VpcAssociationAuthorization {
            hosted_zone_id: view.hosted_zone_id,
            vpc_id: view.vpc_id,
            vpc_region: view.vpc_region,
        }
    }
}

// --- StatefulService implementation ---

impl StatefulService for Route53Service {
    type StateView = Route53StateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        Route53StateView::from(&*guard)
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
            *guard = Route53State::from(view);
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
            for (id, zone_view) in view.hosted_zones {
                guard.hosted_zones.insert(id, HostedZone::from(zone_view));
            }
            for (id, hc_view) in view.health_checks {
                guard
                    .health_checks
                    .insert(id, Route53HealthCheck::from(hc_view));
            }
            for (id, qlc_view) in view.query_logging_configs {
                guard
                    .query_logging_configs
                    .insert(id, Route53QueryLoggingConfig::from(qlc_view));
            }
            for (id, ds_view) in view.reusable_delegation_sets {
                guard
                    .reusable_delegation_sets
                    .insert(id, Route53DelegationSet::from(ds_view));
            }
            for caller_ref in view.deleted_health_check_caller_references {
                guard
                    .deleted_health_check_caller_references
                    .insert(caller_ref);
            }
            for (k, v) in view.key_signing_keys {
                guard
                    .key_signing_keys
                    .insert(k, KeySigningKeyEntry::from(v));
            }
            for (k, v) in view.dnssec_enabled {
                guard.dnssec_enabled.insert(k, v);
            }
            for (k, v) in view.cidr_collections {
                guard
                    .cidr_collections
                    .insert(k, CidrCollectionEntry::from(v));
            }
            for (k, v) in view.traffic_policies {
                guard
                    .traffic_policies
                    .insert(k, TrafficPolicyEntry::from(v));
            }
            for (k, v) in view.traffic_policy_instances {
                guard
                    .traffic_policy_instances
                    .insert(k, TrafficPolicyInstanceEntry::from(v));
            }
            for (k, v) in view.vpc_association_authorizations {
                guard
                    .vpc_association_authorizations
                    .insert(k, VpcAssociationAuthorization::from(v));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
