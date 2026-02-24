use std::collections::HashMap;

/// An S3 Access Point.
#[derive(Debug, Clone)]
pub struct AccessPoint {
    pub name: String,
    pub bucket: String,
    pub account_id: String,
    pub region: String,
    pub alias: String,
    pub arn: String,
    pub network_origin: String,
    pub vpc_id: Option<String>,
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
    pub creation_date: String,
    pub policy: Option<String>,
}

/// Account-level public access block configuration.
#[derive(Debug, Clone, Default)]
pub struct PublicAccessBlock {
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
}

/// An Access Grants Instance.
#[derive(Debug, Clone)]
pub struct AccessGrantsInstance {
    pub instance_id: String,
    pub instance_arn: String,
    pub created_at: String,
    pub identity_center_arn: Option<String>,
    pub identity_center_instance_arn: Option<String>,
    pub resource_policy: Option<AccessGrantsResourcePolicy>,
}

/// An Access Grants Resource Policy.
#[derive(Debug, Clone)]
pub struct AccessGrantsResourcePolicy {
    pub policy: String,
    pub created_at: String,
    pub organization: Option<String>,
}

/// An Access Grants Location.
#[derive(Debug, Clone)]
pub struct AccessGrantsLocation {
    pub location_id: String,
    pub location_arn: String,
    pub location_scope: String,
    pub iam_role_arn: String,
    pub created_at: String,
}

/// An Access Grant.
#[derive(Debug, Clone)]
pub struct AccessGrant {
    pub grant_id: String,
    pub grant_arn: String,
    pub grantee_type: String,
    pub grantee_identifier: String,
    pub permission: String,
    pub grant_scope: Option<String>,
    pub location_id: String,
    pub location_configuration_prefix: Option<String>,
    pub application_arn: Option<String>,
    pub created_at: String,
}

/// A Multi-Region Access Point (MRAP).
#[derive(Debug, Clone)]
pub struct MultiRegionAccessPoint {
    pub name: String,
    pub alias: String,
    pub arn: String,
    pub created_at: String,
    pub status: String,
    pub regions: Vec<MrapRegion>,
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
    pub policy: Option<String>,
    pub request_token_arn: String,
}

/// A region within a Multi-Region Access Point.
#[derive(Debug, Clone)]
pub struct MrapRegion {
    pub bucket: String,
    pub bucket_account_id: Option<String>,
}

/// An S3 Batch Operations Job.
#[derive(Debug, Clone)]
pub struct Job {
    pub job_id: String,
    pub job_arn: String,
    pub status: String,
    pub priority: i32,
    pub description: Option<String>,
    pub role_arn: String,
    pub creation_time: String,
    pub confirmation_required: bool,
    pub tags: Vec<(String, String)>,
    pub operation_type: String,
}

/// A Storage Lens Group.
#[derive(Debug, Clone)]
pub struct StorageLensGroup {
    pub name: String,
    pub arn: String,
    pub region: String,
    pub tags: HashMap<String, String>,
}

/// An Object Lambda Access Point (domain type, distinct from model::ObjectLambdaAccessPoint).
#[derive(Debug, Clone)]
pub struct OlAccessPoint {
    pub name: String,
    pub arn: String,
    pub alias: String,
    pub alias_status: String,
    pub access_point_arn: String,
    pub creation_date: String,
    pub policy: Option<String>,
    pub configuration_json: Option<String>,
}

/// An Outposts bucket.
#[derive(Debug, Clone)]
pub struct OutpostsBucket {
    pub name: String,
    pub arn: String,
    pub outpost_id: String,
    pub creation_date: String,
    pub public_access_block_enabled: bool,
    pub policy: Option<String>,
    pub tags: Vec<(String, String)>,
}

/// A resource-level tag store.
#[derive(Debug, Clone, Default)]
pub struct ResourceTags {
    pub tags: HashMap<String, String>,
}

/// Access point scope configuration.
#[derive(Debug, Clone, Default)]
pub struct AccessPointScope {
    pub prefixes: Vec<String>,
    pub permissions: Vec<String>,
}

/// Storage Lens configuration.
#[derive(Debug, Clone)]
pub struct StorageLensConfig {
    pub config_id: String,
    pub arn: String,
    pub home_region: String,
    pub is_enabled: bool,
    pub tags: Vec<(String, String)>,
}

/// Outposts bucket lifecycle configuration.
#[derive(Debug, Clone, Default)]
pub struct BucketLifecycleConfig {
    /// Serialised XML of lifecycle rules (opaque).
    pub rules_json: String,
}

/// Outposts bucket replication configuration.
#[derive(Debug, Clone, Default)]
pub struct BucketReplicationConfig {
    /// Serialised XML of replication rules (opaque).
    pub role: String,
    pub rules_json: String,
}

/// Outposts bucket versioning configuration.
#[derive(Debug, Clone, Default)]
pub struct BucketVersioningConfig {
    /// "Enabled" | "Suspended" | ""
    pub status: String,
    pub mfa_delete: String,
}

/// MRAP route configuration.
#[derive(Debug, Clone)]
pub struct MrapRoute {
    pub bucket: String,
    pub region: String,
    pub traffic_dial_percentage: i32,
}

/// Directory bucket access point.
#[derive(Debug, Clone)]
pub struct DirectoryBucketAccessPoint {
    pub name: String,
    pub bucket: String,
    pub arn: String,
    pub alias: String,
}
