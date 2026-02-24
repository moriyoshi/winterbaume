use std::collections::HashMap;

/// An OpenSearch domain.
#[derive(Debug, Clone)]
pub struct Domain {
    /// The name of the domain.
    pub domain_name: String,
    /// The ARN of the domain.
    pub arn: String,
    /// The unique internal ID of the domain.
    pub domain_id: String,
    /// The endpoint for the domain.
    pub endpoint: Option<String>,
    /// The engine version (e.g., "OpenSearch_2.11").
    pub engine_version: String,
    /// Whether the domain is being created.
    pub created: bool,
    /// Whether the domain is being deleted.
    pub deleted: bool,
    /// Whether a domain configuration change is currently processing.
    pub processing: bool,
    /// The instance type of data nodes.
    pub instance_type: String,
    /// The number of data node instances.
    pub instance_count: i32,
    /// Whether a dedicated master is enabled.
    pub dedicated_master_enabled: bool,
    /// Whether zone awareness is enabled.
    pub zone_awareness_enabled: bool,
    /// The EBS volume type.
    pub ebs_enabled: bool,
    /// The EBS volume size in GiB.
    pub ebs_volume_size: i32,
    /// The EBS volume type.
    pub ebs_volume_type: String,
    /// Access policies JSON.
    pub access_policies: String,
    /// Resource tags keyed by tag key.
    pub tags: HashMap<String, String>,
}

/// A VPC endpoint for an OpenSearch domain.
#[derive(Debug, Clone)]
pub struct VpcEndpoint {
    pub vpc_endpoint_id: String,
    pub vpc_endpoint_owner: String,
    pub domain_arn: String,
    pub subnet_ids: Vec<String>,
    pub security_group_ids: Vec<String>,
    pub status: String,
    pub endpoint: Option<String>,
}

/// A data source associated with an OpenSearch domain.
#[derive(Debug, Clone)]
pub struct DataSource {
    pub name: String,
    pub description: Option<String>,
    pub data_source_type: DataSourceTypeKind,
    pub status: String,
}

/// The type of a data source.
#[derive(Debug, Clone)]
pub enum DataSourceTypeKind {
    S3GlueDataCatalog { role_arn: Option<String> },
}

/// A direct query data source.
#[derive(Debug, Clone)]
pub struct DirectQueryDataSource {
    pub data_source_name: String,
    pub data_source_arn: String,
    pub description: Option<String>,
    pub data_source_type: DirectQueryDataSourceTypeKind,
    pub open_search_arns: Vec<String>,
    pub tags: Vec<(String, String)>,
}

/// The type of a direct query data source.
#[derive(Debug, Clone)]
pub enum DirectQueryDataSourceTypeKind {
    CloudWatchLog { role_arn: String },
    SecurityLake { role_arn: String },
}

/// A custom package.
#[derive(Debug, Clone)]
pub struct Package {
    pub package_id: String,
    pub package_name: String,
    pub package_type: String,
    pub package_description: Option<String>,
    pub engine_version: Option<String>,
    pub status: String,
    pub created_at: f64,
    pub last_updated_at: f64,
    /// Domains this package is associated with.
    pub associated_domains: Vec<String>,
}

/// An outbound cross-cluster connection.
#[derive(Debug, Clone)]
pub struct OutboundConnection {
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

/// An inbound cross-cluster connection.
#[derive(Debug, Clone)]
pub struct InboundConnection {
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

/// A reserved instance.
#[derive(Debug, Clone)]
pub struct ReservedInstance {
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

/// An authorized principal for VPC endpoint access.
#[derive(Debug, Clone)]
pub struct AuthorizedPrincipal {
    pub principal: String,
    pub principal_type: String,
}
