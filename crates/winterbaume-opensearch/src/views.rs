//! Serde-compatible view types for OpenSearch state snapshots.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::OpenSearchService;
use crate::state::OpenSearchState;
use crate::types::{
    AuthorizedPrincipal, DataSource, DataSourceTypeKind, DirectQueryDataSource,
    DirectQueryDataSourceTypeKind, Domain, InboundConnection, OutboundConnection, Package,
    ReservedInstance, VpcEndpoint,
};

/// Serializable view of the entire OpenSearch state for one account/region.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpenSearchStateView {
    /// Domains keyed by domain name.
    #[serde(default)]
    pub domains: HashMap<String, DomainView>,
    /// Tags keyed by ARN -> (key -> value).
    #[serde(default)]
    pub tags: HashMap<String, HashMap<String, String>>,
    /// VPC endpoints keyed by endpoint ID.
    #[serde(default)]
    pub vpc_endpoints: HashMap<String, VpcEndpointView>,
    /// Authorized principals keyed by domain name -> list.
    #[serde(default)]
    pub vpc_authorized_principals: HashMap<String, Vec<AuthorizedPrincipalView>>,
    /// Data sources keyed by domain name -> (source name -> DataSource).
    #[serde(default)]
    pub data_sources: HashMap<String, HashMap<String, DataSourceView>>,
    /// Direct query data sources keyed by data source name.
    #[serde(default)]
    pub direct_query_data_sources: HashMap<String, DirectQueryDataSourceView>,
    /// Packages keyed by package ID.
    #[serde(default)]
    pub packages: HashMap<String, PackageView>,
    /// Outbound connections keyed by connection ID.
    #[serde(default)]
    pub outbound_connections: HashMap<String, OutboundConnectionView>,
    /// Inbound connections keyed by connection ID.
    #[serde(default)]
    pub inbound_connections: HashMap<String, InboundConnectionView>,
    /// Reserved instances keyed by reserved instance ID.
    #[serde(default)]
    pub reserved_instances: HashMap<String, ReservedInstanceView>,
}

/// Serializable view of a single OpenSearch domain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainView {
    pub domain_name: String,
    pub arn: String,
    pub domain_id: String,
    pub endpoint: Option<String>,
    pub engine_version: String,
    pub created: bool,
    pub deleted: bool,
    pub processing: bool,
    pub instance_type: String,
    pub instance_count: i32,
    pub dedicated_master_enabled: bool,
    pub zone_awareness_enabled: bool,
    pub ebs_enabled: bool,
    pub ebs_volume_size: i32,
    pub ebs_volume_type: String,
    pub access_policies: String,
    #[serde(default)]
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcEndpointView {
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_owner: String,
    pub domain_arn: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub status: String,
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizedPrincipalView {
    pub principal: String,
    pub principal_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSourceView {
    pub name: String,
    pub description: Option<String>,
    pub data_source_type: DataSourceTypeKindView,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSourceTypeKindView {
    S3GlueDataCatalog { role_arn: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectQueryDataSourceView {
    pub data_source_name: String,
    pub data_source_arn: String,
    pub description: Option<String>,
    pub data_source_type: DirectQueryDataSourceTypeKindView,
    pub open_search_arns: Vec<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DirectQueryDataSourceTypeKindView {
    CloudWatchLog { role_arn: String },
    SecurityLake { role_arn: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageView {
    pub package_id: String,
    pub package_name: String,
    pub package_type: String,
    pub package_description: Option<String>,
    pub engine_version: Option<String>,
    pub status: String,
    pub created_at: f64,
    pub last_updated_at: f64,
    pub associated_domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundConnectionView {
    pub connection_id: String,
    pub connection_alias: String,
    pub connection_mode: Option<String>,
    pub status_code: String,
    pub local_domain_name: String,
    pub local_owner_id: String,
    pub local_region: Option<String>,
    pub remote_domain_name: String,
    pub remote_owner_id: Option<String>,
    pub remote_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InboundConnectionView {
    pub connection_id: String,
    pub connection_mode: Option<String>,
    pub status_code: String,
    pub local_domain_name: String,
    pub local_owner_id: String,
    pub local_region: Option<String>,
    pub remote_domain_name: String,
    pub remote_owner_id: Option<String>,
    pub remote_region: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservedInstanceView {
    pub reserved_instance_id: String,
    pub reservation_name: String,
    pub reserved_instance_offering_id: String,
    pub instance_type: String,
    pub instance_count: i32,
    pub duration: i32,
    pub fixed_price: f64,
    pub usage_price: f64,
    pub currency_code: String,
    pub payment_option: String,
    pub state: String,
}

// ---------------------------------------------------------------------------
// From conversions: Domain
// ---------------------------------------------------------------------------

impl From<&Domain> for DomainView {
    fn from(d: &Domain) -> Self {
        DomainView {
            domain_name: d.domain_name.clone(),
            arn: d.arn.clone(),
            domain_id: d.domain_id.clone(),
            endpoint: d.endpoint.clone(),
            engine_version: d.engine_version.clone(),
            created: d.created,
            deleted: d.deleted,
            processing: d.processing,
            instance_type: d.instance_type.clone(),
            instance_count: d.instance_count,
            dedicated_master_enabled: d.dedicated_master_enabled,
            zone_awareness_enabled: d.zone_awareness_enabled,
            ebs_enabled: d.ebs_enabled,
            ebs_volume_size: d.ebs_volume_size,
            ebs_volume_type: d.ebs_volume_type.clone(),
            access_policies: d.access_policies.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<DomainView> for Domain {
    fn from(v: DomainView) -> Self {
        Domain {
            domain_name: v.domain_name,
            arn: v.arn,
            domain_id: v.domain_id,
            endpoint: v.endpoint,
            engine_version: v.engine_version,
            created: v.created,
            deleted: v.deleted,
            processing: v.processing,
            instance_type: v.instance_type,
            instance_count: v.instance_count,
            dedicated_master_enabled: v.dedicated_master_enabled,
            zone_awareness_enabled: v.zone_awareness_enabled,
            ebs_enabled: v.ebs_enabled,
            ebs_volume_size: v.ebs_volume_size,
            ebs_volume_type: v.ebs_volume_type,
            access_policies: v.access_policies,
            tags: v.tags,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: VpcEndpoint
// ---------------------------------------------------------------------------

impl From<&VpcEndpoint> for VpcEndpointView {
    fn from(e: &VpcEndpoint) -> Self {
        VpcEndpointView {
            vpc_endpoint_id: e.vpc_endpoint_id.clone(),
            vpc_endpoint_owner: e.vpc_endpoint_owner.clone(),
            domain_arn: e.domain_arn.clone(),
            subnet_ids: e.subnet_ids.clone(),
            security_group_ids: e.security_group_ids.clone(),
            status: e.status.clone(),
            endpoint: e.endpoint.clone(),
        }
    }
}

impl From<VpcEndpointView> for VpcEndpoint {
    fn from(v: VpcEndpointView) -> Self {
        VpcEndpoint {
            vpc_endpoint_id: v.vpc_endpoint_id,
            vpc_endpoint_owner: v.vpc_endpoint_owner,
            domain_arn: v.domain_arn,
            subnet_ids: v.subnet_ids,
            security_group_ids: v.security_group_ids,
            status: v.status,
            endpoint: v.endpoint,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: AuthorizedPrincipal
// ---------------------------------------------------------------------------

impl From<&AuthorizedPrincipal> for AuthorizedPrincipalView {
    fn from(a: &AuthorizedPrincipal) -> Self {
        AuthorizedPrincipalView {
            principal: a.principal.clone(),
            principal_type: a.principal_type.clone(),
        }
    }
}

impl From<AuthorizedPrincipalView> for AuthorizedPrincipal {
    fn from(v: AuthorizedPrincipalView) -> Self {
        AuthorizedPrincipal {
            principal: v.principal,
            principal_type: v.principal_type,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: DataSource
// ---------------------------------------------------------------------------

impl From<&DataSource> for DataSourceView {
    fn from(d: &DataSource) -> Self {
        DataSourceView {
            name: d.name.clone(),
            description: d.description.clone(),
            data_source_type: DataSourceTypeKindView::from(&d.data_source_type),
            status: d.status.clone(),
        }
    }
}

impl From<&DataSourceTypeKind> for DataSourceTypeKindView {
    fn from(t: &DataSourceTypeKind) -> Self {
        match t {
            DataSourceTypeKind::S3GlueDataCatalog { role_arn } => {
                DataSourceTypeKindView::S3GlueDataCatalog {
                    role_arn: role_arn.clone(),
                }
            }
        }
    }
}

impl From<DataSourceView> for DataSource {
    fn from(v: DataSourceView) -> Self {
        DataSource {
            name: v.name,
            description: v.description,
            data_source_type: DataSourceTypeKind::from(v.data_source_type),
            status: v.status,
        }
    }
}

impl From<DataSourceTypeKindView> for DataSourceTypeKind {
    fn from(v: DataSourceTypeKindView) -> Self {
        match v {
            DataSourceTypeKindView::S3GlueDataCatalog { role_arn } => {
                DataSourceTypeKind::S3GlueDataCatalog { role_arn }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: DirectQueryDataSource
// ---------------------------------------------------------------------------

impl From<&DirectQueryDataSource> for DirectQueryDataSourceView {
    fn from(d: &DirectQueryDataSource) -> Self {
        DirectQueryDataSourceView {
            data_source_name: d.data_source_name.clone(),
            data_source_arn: d.data_source_arn.clone(),
            description: d.description.clone(),
            data_source_type: DirectQueryDataSourceTypeKindView::from(&d.data_source_type),
            open_search_arns: d.open_search_arns.clone(),
            tags: d.tags.clone(),
        }
    }
}

impl From<&DirectQueryDataSourceTypeKind> for DirectQueryDataSourceTypeKindView {
    fn from(t: &DirectQueryDataSourceTypeKind) -> Self {
        match t {
            DirectQueryDataSourceTypeKind::CloudWatchLog { role_arn } => {
                DirectQueryDataSourceTypeKindView::CloudWatchLog {
                    role_arn: role_arn.clone(),
                }
            }
            DirectQueryDataSourceTypeKind::SecurityLake { role_arn } => {
                DirectQueryDataSourceTypeKindView::SecurityLake {
                    role_arn: role_arn.clone(),
                }
            }
        }
    }
}

impl From<DirectQueryDataSourceView> for DirectQueryDataSource {
    fn from(v: DirectQueryDataSourceView) -> Self {
        DirectQueryDataSource {
            data_source_name: v.data_source_name,
            data_source_arn: v.data_source_arn,
            description: v.description,
            data_source_type: DirectQueryDataSourceTypeKind::from(v.data_source_type),
            open_search_arns: v.open_search_arns,
            tags: v.tags,
        }
    }
}

impl From<DirectQueryDataSourceTypeKindView> for DirectQueryDataSourceTypeKind {
    fn from(v: DirectQueryDataSourceTypeKindView) -> Self {
        match v {
            DirectQueryDataSourceTypeKindView::CloudWatchLog { role_arn } => {
                DirectQueryDataSourceTypeKind::CloudWatchLog { role_arn }
            }
            DirectQueryDataSourceTypeKindView::SecurityLake { role_arn } => {
                DirectQueryDataSourceTypeKind::SecurityLake { role_arn }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: Package
// ---------------------------------------------------------------------------

impl From<&Package> for PackageView {
    fn from(p: &Package) -> Self {
        PackageView {
            package_id: p.package_id.clone(),
            package_name: p.package_name.clone(),
            package_type: p.package_type.clone(),
            package_description: p.package_description.clone(),
            engine_version: p.engine_version.clone(),
            status: p.status.clone(),
            created_at: p.created_at,
            last_updated_at: p.last_updated_at,
            associated_domains: p.associated_domains.clone(),
        }
    }
}

impl From<PackageView> for Package {
    fn from(v: PackageView) -> Self {
        Package {
            package_id: v.package_id,
            package_name: v.package_name,
            package_type: v.package_type,
            package_description: v.package_description,
            engine_version: v.engine_version,
            status: v.status,
            created_at: v.created_at,
            last_updated_at: v.last_updated_at,
            associated_domains: v.associated_domains,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: OutboundConnection
// ---------------------------------------------------------------------------

impl From<&OutboundConnection> for OutboundConnectionView {
    fn from(c: &OutboundConnection) -> Self {
        OutboundConnectionView {
            connection_id: c.connection_id.clone(),
            connection_alias: c.connection_alias.clone(),
            connection_mode: c.connection_mode.clone(),
            status_code: c.status_code.clone(),
            local_domain_name: c.local_domain_name.clone(),
            local_owner_id: c.local_owner_id.clone(),
            local_region: c.local_region.clone(),
            remote_domain_name: c.remote_domain_name.clone(),
            remote_owner_id: c.remote_owner_id.clone(),
            remote_region: c.remote_region.clone(),
        }
    }
}

impl From<OutboundConnectionView> for OutboundConnection {
    fn from(v: OutboundConnectionView) -> Self {
        OutboundConnection {
            connection_id: v.connection_id,
            connection_alias: v.connection_alias,
            connection_mode: v.connection_mode,
            status_code: v.status_code,
            local_domain_name: v.local_domain_name,
            local_owner_id: v.local_owner_id,
            local_region: v.local_region,
            remote_domain_name: v.remote_domain_name,
            remote_owner_id: v.remote_owner_id,
            remote_region: v.remote_region,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: InboundConnection
// ---------------------------------------------------------------------------

impl From<&InboundConnection> for InboundConnectionView {
    fn from(c: &InboundConnection) -> Self {
        InboundConnectionView {
            connection_id: c.connection_id.clone(),
            connection_mode: c.connection_mode.clone(),
            status_code: c.status_code.clone(),
            local_domain_name: c.local_domain_name.clone(),
            local_owner_id: c.local_owner_id.clone(),
            local_region: c.local_region.clone(),
            remote_domain_name: c.remote_domain_name.clone(),
            remote_owner_id: c.remote_owner_id.clone(),
            remote_region: c.remote_region.clone(),
        }
    }
}

impl From<InboundConnectionView> for InboundConnection {
    fn from(v: InboundConnectionView) -> Self {
        InboundConnection {
            connection_id: v.connection_id,
            connection_mode: v.connection_mode,
            status_code: v.status_code,
            local_domain_name: v.local_domain_name,
            local_owner_id: v.local_owner_id,
            local_region: v.local_region,
            remote_domain_name: v.remote_domain_name,
            remote_owner_id: v.remote_owner_id,
            remote_region: v.remote_region,
        }
    }
}

// ---------------------------------------------------------------------------
// From conversions: ReservedInstance
// ---------------------------------------------------------------------------

impl From<&ReservedInstance> for ReservedInstanceView {
    fn from(r: &ReservedInstance) -> Self {
        ReservedInstanceView {
            reserved_instance_id: r.reserved_instance_id.clone(),
            reservation_name: r.reservation_name.clone(),
            reserved_instance_offering_id: r.reserved_instance_offering_id.clone(),
            instance_type: r.instance_type.clone(),
            instance_count: r.instance_count,
            duration: r.duration,
            fixed_price: r.fixed_price,
            usage_price: r.usage_price,
            currency_code: r.currency_code.clone(),
            payment_option: r.payment_option.clone(),
            state: r.state.clone(),
        }
    }
}

impl From<ReservedInstanceView> for ReservedInstance {
    fn from(v: ReservedInstanceView) -> Self {
        ReservedInstance {
            reserved_instance_id: v.reserved_instance_id,
            reservation_name: v.reservation_name,
            reserved_instance_offering_id: v.reserved_instance_offering_id,
            instance_type: v.instance_type,
            instance_count: v.instance_count,
            duration: v.duration,
            fixed_price: v.fixed_price,
            usage_price: v.usage_price,
            currency_code: v.currency_code,
            payment_option: v.payment_option,
            state: v.state,
        }
    }
}

// ---------------------------------------------------------------------------
// StatefulService implementation
// ---------------------------------------------------------------------------

impl StatefulService for OpenSearchService {
    type StateView = OpenSearchStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;

        let domains = guard
            .domains
            .iter()
            .map(|(k, v)| (k.clone(), DomainView::from(v)))
            .collect();

        let tags = guard.tags.clone();

        let vpc_endpoints = guard
            .vpc_endpoints
            .iter()
            .map(|(k, v)| (k.clone(), VpcEndpointView::from(v)))
            .collect();

        let vpc_authorized_principals = guard
            .vpc_authorized_principals
            .iter()
            .map(|(k, v)| {
                (
                    k.clone(),
                    v.iter().map(AuthorizedPrincipalView::from).collect(),
                )
            })
            .collect();

        let data_sources = guard
            .data_sources
            .iter()
            .map(|(domain, ds_map)| {
                (
                    domain.clone(),
                    ds_map
                        .iter()
                        .map(|(name, ds)| (name.clone(), DataSourceView::from(ds)))
                        .collect(),
                )
            })
            .collect();

        let direct_query_data_sources = guard
            .direct_query_data_sources
            .iter()
            .map(|(k, v)| (k.clone(), DirectQueryDataSourceView::from(v)))
            .collect();

        let packages = guard
            .packages
            .iter()
            .map(|(k, v)| (k.clone(), PackageView::from(v)))
            .collect();

        let outbound_connections = guard
            .outbound_connections
            .iter()
            .map(|(k, v)| (k.clone(), OutboundConnectionView::from(v)))
            .collect();

        let inbound_connections = guard
            .inbound_connections
            .iter()
            .map(|(k, v)| (k.clone(), InboundConnectionView::from(v)))
            .collect();

        let reserved_instances = guard
            .reserved_instances
            .iter()
            .map(|(k, v)| (k.clone(), ReservedInstanceView::from(v)))
            .collect();

        OpenSearchStateView {
            domains,
            tags,
            vpc_endpoints,
            vpc_authorized_principals,
            data_sources,
            direct_query_data_sources,
            packages,
            outbound_connections,
            inbound_connections,
            reserved_instances,
        }
    }

    async fn restore(
        &self,
        account_id: &str,
        region: &str,
        view: Self::StateView,
    ) -> Result<(), StateViewError> {
        let mut new_state = OpenSearchState::default();
        for (name, dv) in view.domains {
            new_state.domains.insert(name, Domain::from(dv));
        }
        new_state.tags = view.tags;
        for (id, ev) in view.vpc_endpoints {
            new_state.vpc_endpoints.insert(id, VpcEndpoint::from(ev));
        }
        for (domain, principals) in view.vpc_authorized_principals {
            new_state.vpc_authorized_principals.insert(
                domain,
                principals
                    .into_iter()
                    .map(AuthorizedPrincipal::from)
                    .collect(),
            );
        }
        for (domain, ds_map) in view.data_sources {
            new_state.data_sources.insert(
                domain,
                ds_map
                    .into_iter()
                    .map(|(name, ds)| (name, DataSource::from(ds)))
                    .collect(),
            );
        }
        for (name, ds) in view.direct_query_data_sources {
            new_state
                .direct_query_data_sources
                .insert(name, DirectQueryDataSource::from(ds));
        }
        for (id, pkg) in view.packages {
            new_state.packages.insert(id, Package::from(pkg));
        }
        for (id, conn) in view.outbound_connections {
            new_state
                .outbound_connections
                .insert(id, OutboundConnection::from(conn));
        }
        for (id, conn) in view.inbound_connections {
            new_state
                .inbound_connections
                .insert(id, InboundConnection::from(conn));
        }
        for (id, ri) in view.reserved_instances {
            new_state
                .reserved_instances
                .insert(id, ReservedInstance::from(ri));
        }
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
        let state = self.state.get(account_id, region);
        {
            let mut guard = state.write().await;
            for (name, dv) in view.domains {
                guard.domains.insert(name, Domain::from(dv));
            }
            for (arn, tag_map) in view.tags {
                guard.tags.insert(arn, tag_map);
            }
            for (id, ev) in view.vpc_endpoints {
                guard.vpc_endpoints.insert(id, VpcEndpoint::from(ev));
            }
            for (domain, principals) in view.vpc_authorized_principals {
                guard.vpc_authorized_principals.insert(
                    domain,
                    principals
                        .into_iter()
                        .map(AuthorizedPrincipal::from)
                        .collect(),
                );
            }
            for (domain, ds_map) in view.data_sources {
                let entry = guard.data_sources.entry(domain).or_default();
                for (name, ds) in ds_map {
                    entry.insert(name, DataSource::from(ds));
                }
            }
            for (name, ds) in view.direct_query_data_sources {
                guard
                    .direct_query_data_sources
                    .insert(name, DirectQueryDataSource::from(ds));
            }
            for (id, pkg) in view.packages {
                guard.packages.insert(id, Package::from(pkg));
            }
            for (id, conn) in view.outbound_connections {
                guard
                    .outbound_connections
                    .insert(id, OutboundConnection::from(conn));
            }
            for (id, conn) in view.inbound_connections {
                guard
                    .inbound_connections
                    .insert(id, InboundConnection::from(conn));
            }
            for (id, ri) in view.reserved_instances {
                guard
                    .reserved_instances
                    .insert(id, ReservedInstance::from(ri));
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
