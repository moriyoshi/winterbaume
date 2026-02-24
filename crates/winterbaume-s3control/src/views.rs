use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use winterbaume_core::{StateChangeNotifier, StateViewError, StatefulService};

use crate::handlers::S3ControlService;
use crate::state::S3ControlState;
use crate::types::{
    AccessGrant, AccessGrantsInstance, AccessGrantsLocation, AccessGrantsResourcePolicy,
    AccessPoint, AccessPointScope, BucketLifecycleConfig, BucketReplicationConfig,
    BucketVersioningConfig, DirectoryBucketAccessPoint, Job, MrapRegion, MrapRoute,
    MultiRegionAccessPoint, OlAccessPoint, OutpostsBucket, PublicAccessBlock, StorageLensConfig,
    StorageLensGroup,
};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct S3ControlStateView {
    #[serde(default)]
    pub access_points: HashMap<String, AccessPointView>,
    #[serde(default)]
    pub public_access_block: Option<PublicAccessBlockView>,
    #[serde(default)]
    pub access_grants_instance: Option<AccessGrantsInstanceView>,
    #[serde(default)]
    pub access_grants_locations: HashMap<String, AccessGrantsLocationView>,
    #[serde(default)]
    pub access_grants: HashMap<String, AccessGrantView>,
    #[serde(default)]
    pub multi_region_access_points: HashMap<String, MultiRegionAccessPointView>,
    #[serde(default)]
    pub jobs: HashMap<String, JobView>,
    #[serde(default)]
    pub storage_lens_groups: HashMap<String, StorageLensGroupView>,
    #[serde(default)]
    pub object_lambda_access_points: HashMap<String, OlAccessPointView>,
    #[serde(default)]
    pub outposts_buckets: HashMap<String, OutpostsBucketView>,
    #[serde(default)]
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    #[serde(default)]
    pub access_point_scopes: HashMap<String, AccessPointScopeView>,
    #[serde(default)]
    pub directory_bucket_access_points: HashMap<String, DirectoryBucketAccessPointView>,
    #[serde(default)]
    pub storage_lens_configs: HashMap<String, StorageLensConfigView>,
    #[serde(default)]
    pub bucket_lifecycle_configs: HashMap<String, BucketLifecycleConfigView>,
    #[serde(default)]
    pub bucket_replication_configs: HashMap<String, BucketReplicationConfigView>,
    #[serde(default)]
    pub bucket_versioning_configs: HashMap<String, BucketVersioningConfigView>,
    #[serde(default)]
    pub mrap_routes: HashMap<String, Vec<MrapRouteView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPointView {
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
    #[serde(default)]
    pub policy: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicAccessBlockView {
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrantsInstanceView {
    pub instance_id: String,
    pub instance_arn: String,
    pub created_at: String,
    pub identity_center_arn: Option<String>,
    pub identity_center_instance_arn: Option<String>,
    pub resource_policy: Option<AccessGrantsResourcePolicyView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrantsResourcePolicyView {
    pub policy: String,
    pub created_at: String,
    pub organization: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrantsLocationView {
    pub location_id: String,
    pub location_arn: String,
    pub location_scope: String,
    pub iam_role_arn: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessGrantView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrapRegionView {
    pub bucket: String,
    pub bucket_account_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiRegionAccessPointView {
    pub name: String,
    pub alias: String,
    pub arn: String,
    pub created_at: String,
    pub status: String,
    pub regions: Vec<MrapRegionView>,
    pub block_public_acls: bool,
    pub ignore_public_acls: bool,
    pub block_public_policy: bool,
    pub restrict_public_buckets: bool,
    pub policy: Option<String>,
    pub request_token_arn: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobView {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLensGroupView {
    pub name: String,
    pub arn: String,
    pub region: String,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OlAccessPointView {
    pub name: String,
    pub arn: String,
    pub alias: String,
    pub alias_status: String,
    pub access_point_arn: String,
    pub creation_date: String,
    pub policy: Option<String>,
    pub configuration_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutpostsBucketView {
    pub name: String,
    pub arn: String,
    pub outpost_id: String,
    pub creation_date: String,
    pub public_access_block_enabled: bool,
    pub policy: Option<String>,
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessPointScopeView {
    #[serde(default)]
    pub prefixes: Vec<String>,
    #[serde(default)]
    pub permissions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectoryBucketAccessPointView {
    pub name: String,
    pub bucket: String,
    pub arn: String,
    pub alias: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLensConfigView {
    pub config_id: String,
    pub arn: String,
    pub home_region: String,
    pub is_enabled: bool,
    #[serde(default)]
    pub tags: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketLifecycleConfigView {
    #[serde(default)]
    pub rules_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketReplicationConfigView {
    #[serde(default)]
    pub role: String,
    #[serde(default)]
    pub rules_json: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BucketVersioningConfigView {
    #[serde(default)]
    pub status: String,
    #[serde(default)]
    pub mfa_delete: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrapRouteView {
    pub bucket: String,
    pub region: String,
    pub traffic_dial_percentage: i32,
}

// -------------------------------------------------------------------------
// From impls: internal type → view
// -------------------------------------------------------------------------

impl From<&AccessPoint> for AccessPointView {
    fn from(ap: &AccessPoint) -> Self {
        Self {
            name: ap.name.clone(),
            bucket: ap.bucket.clone(),
            account_id: ap.account_id.clone(),
            region: ap.region.clone(),
            alias: ap.alias.clone(),
            arn: ap.arn.clone(),
            network_origin: ap.network_origin.clone(),
            vpc_id: ap.vpc_id.clone(),
            block_public_acls: ap.block_public_acls,
            ignore_public_acls: ap.ignore_public_acls,
            block_public_policy: ap.block_public_policy,
            restrict_public_buckets: ap.restrict_public_buckets,
            creation_date: ap.creation_date.clone(),
            policy: ap.policy.clone(),
        }
    }
}

impl From<AccessPointView> for AccessPoint {
    fn from(v: AccessPointView) -> Self {
        Self {
            name: v.name,
            bucket: v.bucket,
            account_id: v.account_id,
            region: v.region,
            alias: v.alias,
            arn: v.arn,
            network_origin: v.network_origin,
            vpc_id: v.vpc_id,
            block_public_acls: v.block_public_acls,
            ignore_public_acls: v.ignore_public_acls,
            block_public_policy: v.block_public_policy,
            restrict_public_buckets: v.restrict_public_buckets,
            creation_date: v.creation_date,
            policy: v.policy,
        }
    }
}

impl From<&PublicAccessBlock> for PublicAccessBlockView {
    fn from(p: &PublicAccessBlock) -> Self {
        Self {
            block_public_acls: p.block_public_acls,
            ignore_public_acls: p.ignore_public_acls,
            block_public_policy: p.block_public_policy,
            restrict_public_buckets: p.restrict_public_buckets,
        }
    }
}

impl From<PublicAccessBlockView> for PublicAccessBlock {
    fn from(v: PublicAccessBlockView) -> Self {
        Self {
            block_public_acls: v.block_public_acls,
            ignore_public_acls: v.ignore_public_acls,
            block_public_policy: v.block_public_policy,
            restrict_public_buckets: v.restrict_public_buckets,
        }
    }
}

impl From<&AccessGrantsResourcePolicy> for AccessGrantsResourcePolicyView {
    fn from(p: &AccessGrantsResourcePolicy) -> Self {
        Self {
            policy: p.policy.clone(),
            created_at: p.created_at.clone(),
            organization: p.organization.clone(),
        }
    }
}

impl From<AccessGrantsResourcePolicyView> for AccessGrantsResourcePolicy {
    fn from(v: AccessGrantsResourcePolicyView) -> Self {
        Self {
            policy: v.policy,
            created_at: v.created_at,
            organization: v.organization,
        }
    }
}

impl From<&AccessGrantsInstance> for AccessGrantsInstanceView {
    fn from(i: &AccessGrantsInstance) -> Self {
        Self {
            instance_id: i.instance_id.clone(),
            instance_arn: i.instance_arn.clone(),
            created_at: i.created_at.clone(),
            identity_center_arn: i.identity_center_arn.clone(),
            identity_center_instance_arn: i.identity_center_instance_arn.clone(),
            resource_policy: i
                .resource_policy
                .as_ref()
                .map(AccessGrantsResourcePolicyView::from),
        }
    }
}

impl From<AccessGrantsInstanceView> for AccessGrantsInstance {
    fn from(v: AccessGrantsInstanceView) -> Self {
        Self {
            instance_id: v.instance_id,
            instance_arn: v.instance_arn,
            created_at: v.created_at,
            identity_center_arn: v.identity_center_arn,
            identity_center_instance_arn: v.identity_center_instance_arn,
            resource_policy: v.resource_policy.map(AccessGrantsResourcePolicy::from),
        }
    }
}

impl From<&AccessGrantsLocation> for AccessGrantsLocationView {
    fn from(l: &AccessGrantsLocation) -> Self {
        Self {
            location_id: l.location_id.clone(),
            location_arn: l.location_arn.clone(),
            location_scope: l.location_scope.clone(),
            iam_role_arn: l.iam_role_arn.clone(),
            created_at: l.created_at.clone(),
        }
    }
}

impl From<AccessGrantsLocationView> for AccessGrantsLocation {
    fn from(v: AccessGrantsLocationView) -> Self {
        Self {
            location_id: v.location_id,
            location_arn: v.location_arn,
            location_scope: v.location_scope,
            iam_role_arn: v.iam_role_arn,
            created_at: v.created_at,
        }
    }
}

impl From<&AccessGrant> for AccessGrantView {
    fn from(g: &AccessGrant) -> Self {
        Self {
            grant_id: g.grant_id.clone(),
            grant_arn: g.grant_arn.clone(),
            grantee_type: g.grantee_type.clone(),
            grantee_identifier: g.grantee_identifier.clone(),
            permission: g.permission.clone(),
            grant_scope: g.grant_scope.clone(),
            location_id: g.location_id.clone(),
            location_configuration_prefix: g.location_configuration_prefix.clone(),
            application_arn: g.application_arn.clone(),
            created_at: g.created_at.clone(),
        }
    }
}

impl From<AccessGrantView> for AccessGrant {
    fn from(v: AccessGrantView) -> Self {
        Self {
            grant_id: v.grant_id,
            grant_arn: v.grant_arn,
            grantee_type: v.grantee_type,
            grantee_identifier: v.grantee_identifier,
            permission: v.permission,
            grant_scope: v.grant_scope,
            location_id: v.location_id,
            location_configuration_prefix: v.location_configuration_prefix,
            application_arn: v.application_arn,
            created_at: v.created_at,
        }
    }
}

impl From<&MrapRegion> for MrapRegionView {
    fn from(r: &MrapRegion) -> Self {
        Self {
            bucket: r.bucket.clone(),
            bucket_account_id: r.bucket_account_id.clone(),
        }
    }
}

impl From<MrapRegionView> for MrapRegion {
    fn from(v: MrapRegionView) -> Self {
        Self {
            bucket: v.bucket,
            bucket_account_id: v.bucket_account_id,
        }
    }
}

impl From<&MultiRegionAccessPoint> for MultiRegionAccessPointView {
    fn from(m: &MultiRegionAccessPoint) -> Self {
        Self {
            name: m.name.clone(),
            alias: m.alias.clone(),
            arn: m.arn.clone(),
            created_at: m.created_at.clone(),
            status: m.status.clone(),
            regions: m.regions.iter().map(MrapRegionView::from).collect(),
            block_public_acls: m.block_public_acls,
            ignore_public_acls: m.ignore_public_acls,
            block_public_policy: m.block_public_policy,
            restrict_public_buckets: m.restrict_public_buckets,
            policy: m.policy.clone(),
            request_token_arn: m.request_token_arn.clone(),
        }
    }
}

impl From<MultiRegionAccessPointView> for MultiRegionAccessPoint {
    fn from(v: MultiRegionAccessPointView) -> Self {
        Self {
            name: v.name,
            alias: v.alias,
            arn: v.arn,
            created_at: v.created_at,
            status: v.status,
            regions: v.regions.into_iter().map(MrapRegion::from).collect(),
            block_public_acls: v.block_public_acls,
            ignore_public_acls: v.ignore_public_acls,
            block_public_policy: v.block_public_policy,
            restrict_public_buckets: v.restrict_public_buckets,
            policy: v.policy,
            request_token_arn: v.request_token_arn,
        }
    }
}

impl From<&Job> for JobView {
    fn from(j: &Job) -> Self {
        Self {
            job_id: j.job_id.clone(),
            job_arn: j.job_arn.clone(),
            status: j.status.clone(),
            priority: j.priority,
            description: j.description.clone(),
            role_arn: j.role_arn.clone(),
            creation_time: j.creation_time.clone(),
            confirmation_required: j.confirmation_required,
            tags: j.tags.clone(),
            operation_type: j.operation_type.clone(),
        }
    }
}

impl From<JobView> for Job {
    fn from(v: JobView) -> Self {
        Self {
            job_id: v.job_id,
            job_arn: v.job_arn,
            status: v.status,
            priority: v.priority,
            description: v.description,
            role_arn: v.role_arn,
            creation_time: v.creation_time,
            confirmation_required: v.confirmation_required,
            tags: v.tags,
            operation_type: v.operation_type,
        }
    }
}

impl From<&StorageLensGroup> for StorageLensGroupView {
    fn from(g: &StorageLensGroup) -> Self {
        Self {
            name: g.name.clone(),
            arn: g.arn.clone(),
            region: g.region.clone(),
            tags: g.tags.clone(),
        }
    }
}

impl From<StorageLensGroupView> for StorageLensGroup {
    fn from(v: StorageLensGroupView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            region: v.region,
            tags: v.tags,
        }
    }
}

impl From<&OlAccessPoint> for OlAccessPointView {
    fn from(ap: &OlAccessPoint) -> Self {
        Self {
            name: ap.name.clone(),
            arn: ap.arn.clone(),
            alias: ap.alias.clone(),
            alias_status: ap.alias_status.clone(),
            access_point_arn: ap.access_point_arn.clone(),
            creation_date: ap.creation_date.clone(),
            policy: ap.policy.clone(),
            configuration_json: ap.configuration_json.clone(),
        }
    }
}

impl From<OlAccessPointView> for OlAccessPoint {
    fn from(v: OlAccessPointView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            alias: v.alias,
            alias_status: v.alias_status,
            access_point_arn: v.access_point_arn,
            creation_date: v.creation_date,
            policy: v.policy,
            configuration_json: v.configuration_json,
        }
    }
}

impl From<&OutpostsBucket> for OutpostsBucketView {
    fn from(b: &OutpostsBucket) -> Self {
        Self {
            name: b.name.clone(),
            arn: b.arn.clone(),
            outpost_id: b.outpost_id.clone(),
            creation_date: b.creation_date.clone(),
            public_access_block_enabled: b.public_access_block_enabled,
            policy: b.policy.clone(),
            tags: b.tags.clone(),
        }
    }
}

impl From<OutpostsBucketView> for OutpostsBucket {
    fn from(v: OutpostsBucketView) -> Self {
        Self {
            name: v.name,
            arn: v.arn,
            outpost_id: v.outpost_id,
            creation_date: v.creation_date,
            public_access_block_enabled: v.public_access_block_enabled,
            policy: v.policy,
            tags: v.tags,
        }
    }
}

impl From<&AccessPointScope> for AccessPointScopeView {
    fn from(s: &AccessPointScope) -> Self {
        Self {
            prefixes: s.prefixes.clone(),
            permissions: s.permissions.clone(),
        }
    }
}

impl From<AccessPointScopeView> for AccessPointScope {
    fn from(v: AccessPointScopeView) -> Self {
        Self {
            prefixes: v.prefixes,
            permissions: v.permissions,
        }
    }
}

impl From<&DirectoryBucketAccessPoint> for DirectoryBucketAccessPointView {
    fn from(d: &DirectoryBucketAccessPoint) -> Self {
        Self {
            name: d.name.clone(),
            bucket: d.bucket.clone(),
            arn: d.arn.clone(),
            alias: d.alias.clone(),
        }
    }
}

impl From<DirectoryBucketAccessPointView> for DirectoryBucketAccessPoint {
    fn from(v: DirectoryBucketAccessPointView) -> Self {
        Self {
            name: v.name,
            bucket: v.bucket,
            arn: v.arn,
            alias: v.alias,
        }
    }
}

impl From<&StorageLensConfig> for StorageLensConfigView {
    fn from(c: &StorageLensConfig) -> Self {
        Self {
            config_id: c.config_id.clone(),
            arn: c.arn.clone(),
            home_region: c.home_region.clone(),
            is_enabled: c.is_enabled,
            tags: c.tags.clone(),
        }
    }
}

impl From<StorageLensConfigView> for StorageLensConfig {
    fn from(v: StorageLensConfigView) -> Self {
        Self {
            config_id: v.config_id,
            arn: v.arn,
            home_region: v.home_region,
            is_enabled: v.is_enabled,
            tags: v.tags,
        }
    }
}

impl From<&BucketLifecycleConfig> for BucketLifecycleConfigView {
    fn from(c: &BucketLifecycleConfig) -> Self {
        Self {
            rules_json: c.rules_json.clone(),
        }
    }
}

impl From<BucketLifecycleConfigView> for BucketLifecycleConfig {
    fn from(v: BucketLifecycleConfigView) -> Self {
        Self {
            rules_json: v.rules_json,
        }
    }
}

impl From<&BucketReplicationConfig> for BucketReplicationConfigView {
    fn from(c: &BucketReplicationConfig) -> Self {
        Self {
            role: c.role.clone(),
            rules_json: c.rules_json.clone(),
        }
    }
}

impl From<BucketReplicationConfigView> for BucketReplicationConfig {
    fn from(v: BucketReplicationConfigView) -> Self {
        Self {
            role: v.role,
            rules_json: v.rules_json,
        }
    }
}

impl From<&BucketVersioningConfig> for BucketVersioningConfigView {
    fn from(c: &BucketVersioningConfig) -> Self {
        Self {
            status: c.status.clone(),
            mfa_delete: c.mfa_delete.clone(),
        }
    }
}

impl From<BucketVersioningConfigView> for BucketVersioningConfig {
    fn from(v: BucketVersioningConfigView) -> Self {
        Self {
            status: v.status,
            mfa_delete: v.mfa_delete,
        }
    }
}

impl From<&MrapRoute> for MrapRouteView {
    fn from(r: &MrapRoute) -> Self {
        Self {
            bucket: r.bucket.clone(),
            region: r.region.clone(),
            traffic_dial_percentage: r.traffic_dial_percentage,
        }
    }
}

impl From<MrapRouteView> for MrapRoute {
    fn from(v: MrapRouteView) -> Self {
        Self {
            bucket: v.bucket,
            region: v.region,
            traffic_dial_percentage: v.traffic_dial_percentage,
        }
    }
}

// -------------------------------------------------------------------------
// S3ControlState ↔ S3ControlStateView
// -------------------------------------------------------------------------

impl From<&S3ControlState> for S3ControlStateView {
    fn from(state: &S3ControlState) -> Self {
        Self {
            access_points: state
                .access_points
                .iter()
                .map(|(k, v)| (k.clone(), AccessPointView::from(v)))
                .collect(),
            public_access_block: state
                .public_access_block
                .as_ref()
                .map(PublicAccessBlockView::from),
            access_grants_instance: state
                .access_grants_instance
                .as_ref()
                .map(AccessGrantsInstanceView::from),
            access_grants_locations: state
                .access_grants_locations
                .iter()
                .map(|(k, v)| (k.clone(), AccessGrantsLocationView::from(v)))
                .collect(),
            access_grants: state
                .access_grants
                .iter()
                .map(|(k, v)| (k.clone(), AccessGrantView::from(v)))
                .collect(),
            multi_region_access_points: state
                .multi_region_access_points
                .iter()
                .map(|(k, v)| (k.clone(), MultiRegionAccessPointView::from(v)))
                .collect(),
            jobs: state
                .jobs
                .iter()
                .map(|(k, v)| (k.clone(), JobView::from(v)))
                .collect(),
            storage_lens_groups: state
                .storage_lens_groups
                .iter()
                .map(|(k, v)| (k.clone(), StorageLensGroupView::from(v)))
                .collect(),
            object_lambda_access_points: state
                .object_lambda_access_points
                .iter()
                .map(|(k, v)| (k.clone(), OlAccessPointView::from(v)))
                .collect(),
            outposts_buckets: state
                .outposts_buckets
                .iter()
                .map(|(k, v)| (k.clone(), OutpostsBucketView::from(v)))
                .collect(),
            resource_tags: state.resource_tags.clone(),
            access_point_scopes: state
                .access_point_scopes
                .iter()
                .map(|(k, v)| (k.clone(), AccessPointScopeView::from(v)))
                .collect(),
            directory_bucket_access_points: state
                .directory_bucket_access_points
                .iter()
                .map(|(k, v)| (k.clone(), DirectoryBucketAccessPointView::from(v)))
                .collect(),
            storage_lens_configs: state
                .storage_lens_configs
                .iter()
                .map(|(k, v)| (k.clone(), StorageLensConfigView::from(v)))
                .collect(),
            bucket_lifecycle_configs: state
                .bucket_lifecycle_configs
                .iter()
                .map(|(k, v)| (k.clone(), BucketLifecycleConfigView::from(v)))
                .collect(),
            bucket_replication_configs: state
                .bucket_replication_configs
                .iter()
                .map(|(k, v)| (k.clone(), BucketReplicationConfigView::from(v)))
                .collect(),
            bucket_versioning_configs: state
                .bucket_versioning_configs
                .iter()
                .map(|(k, v)| (k.clone(), BucketVersioningConfigView::from(v)))
                .collect(),
            mrap_routes: state
                .mrap_routes
                .iter()
                .map(|(k, v)| (k.clone(), v.iter().map(MrapRouteView::from).collect()))
                .collect(),
        }
    }
}

impl From<S3ControlStateView> for S3ControlState {
    fn from(view: S3ControlStateView) -> Self {
        Self {
            access_points: view
                .access_points
                .into_iter()
                .map(|(k, v)| (k, AccessPoint::from(v)))
                .collect(),
            public_access_block: view.public_access_block.map(PublicAccessBlock::from),
            access_grants_instance: view.access_grants_instance.map(AccessGrantsInstance::from),
            access_grants_locations: view
                .access_grants_locations
                .into_iter()
                .map(|(k, v)| (k, AccessGrantsLocation::from(v)))
                .collect(),
            access_grants: view
                .access_grants
                .into_iter()
                .map(|(k, v)| (k, AccessGrant::from(v)))
                .collect(),
            multi_region_access_points: view
                .multi_region_access_points
                .into_iter()
                .map(|(k, v)| (k, MultiRegionAccessPoint::from(v)))
                .collect(),
            jobs: view
                .jobs
                .into_iter()
                .map(|(k, v)| (k, Job::from(v)))
                .collect(),
            storage_lens_groups: view
                .storage_lens_groups
                .into_iter()
                .map(|(k, v)| (k, StorageLensGroup::from(v)))
                .collect(),
            object_lambda_access_points: view
                .object_lambda_access_points
                .into_iter()
                .map(|(k, v)| (k, OlAccessPoint::from(v)))
                .collect(),
            outposts_buckets: view
                .outposts_buckets
                .into_iter()
                .map(|(k, v)| (k, OutpostsBucket::from(v)))
                .collect(),
            resource_tags: view.resource_tags,
            access_point_scopes: view
                .access_point_scopes
                .into_iter()
                .map(|(k, v)| (k, AccessPointScope::from(v)))
                .collect(),
            directory_bucket_access_points: view
                .directory_bucket_access_points
                .into_iter()
                .map(|(k, v)| (k, DirectoryBucketAccessPoint::from(v)))
                .collect(),
            storage_lens_configs: view
                .storage_lens_configs
                .into_iter()
                .map(|(k, v)| (k, StorageLensConfig::from(v)))
                .collect(),
            bucket_lifecycle_configs: view
                .bucket_lifecycle_configs
                .into_iter()
                .map(|(k, v)| (k, BucketLifecycleConfig::from(v)))
                .collect(),
            bucket_replication_configs: view
                .bucket_replication_configs
                .into_iter()
                .map(|(k, v)| (k, BucketReplicationConfig::from(v)))
                .collect(),
            bucket_versioning_configs: view
                .bucket_versioning_configs
                .into_iter()
                .map(|(k, v)| (k, BucketVersioningConfig::from(v)))
                .collect(),
            mrap_routes: view
                .mrap_routes
                .into_iter()
                .map(|(k, v)| (k, v.into_iter().map(MrapRoute::from).collect()))
                .collect(),
        }
    }
}

// -------------------------------------------------------------------------
// StatefulService impl
// -------------------------------------------------------------------------

impl StatefulService for S3ControlService {
    type StateView = S3ControlStateView;

    async fn snapshot(&self, account_id: &str, region: &str) -> Self::StateView {
        let state = self.state.get(account_id, region);
        let guard = state.read().await;
        S3ControlStateView::from(&*guard)
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
            *guard = S3ControlState::from(view);
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
            for (name, ap_view) in view.access_points {
                guard.access_points.insert(name, AccessPoint::from(ap_view));
            }
            if let Some(pab) = view.public_access_block {
                guard.public_access_block = Some(PublicAccessBlock::from(pab));
            }
            if let Some(instance) = view.access_grants_instance {
                guard.access_grants_instance = Some(AccessGrantsInstance::from(instance));
            }
            for (id, loc_view) in view.access_grants_locations {
                guard
                    .access_grants_locations
                    .insert(id, AccessGrantsLocation::from(loc_view));
            }
            for (id, grant_view) in view.access_grants {
                guard
                    .access_grants
                    .insert(id, AccessGrant::from(grant_view));
            }
            for (name, mrap_view) in view.multi_region_access_points {
                guard
                    .multi_region_access_points
                    .insert(name, MultiRegionAccessPoint::from(mrap_view));
            }
            for (id, job_view) in view.jobs {
                guard.jobs.insert(id, Job::from(job_view));
            }
            for (name, group_view) in view.storage_lens_groups {
                guard
                    .storage_lens_groups
                    .insert(name, StorageLensGroup::from(group_view));
            }
            for (name, ap_view) in view.object_lambda_access_points {
                guard
                    .object_lambda_access_points
                    .insert(name, OlAccessPoint::from(ap_view));
            }
            for (name, bucket_view) in view.outposts_buckets {
                guard
                    .outposts_buckets
                    .insert(name, OutpostsBucket::from(bucket_view));
            }
            for (arn, tags) in view.resource_tags {
                let entry = guard.resource_tags.entry(arn).or_default();
                entry.extend(tags);
            }
            for (name, scope_view) in view.access_point_scopes {
                guard
                    .access_point_scopes
                    .insert(name, AccessPointScope::from(scope_view));
            }
            for (name, ap_view) in view.directory_bucket_access_points {
                guard
                    .directory_bucket_access_points
                    .insert(name, DirectoryBucketAccessPoint::from(ap_view));
            }
            for (id, config_view) in view.storage_lens_configs {
                guard
                    .storage_lens_configs
                    .insert(id, StorageLensConfig::from(config_view));
            }
            for (bucket, config_view) in view.bucket_lifecycle_configs {
                guard
                    .bucket_lifecycle_configs
                    .insert(bucket, BucketLifecycleConfig::from(config_view));
            }
            for (bucket, config_view) in view.bucket_replication_configs {
                guard
                    .bucket_replication_configs
                    .insert(bucket, BucketReplicationConfig::from(config_view));
            }
            for (bucket, config_view) in view.bucket_versioning_configs {
                guard
                    .bucket_versioning_configs
                    .insert(bucket, BucketVersioningConfig::from(config_view));
            }
            for (name, routes_view) in view.mrap_routes {
                guard
                    .mrap_routes
                    .insert(name, routes_view.into_iter().map(MrapRoute::from).collect());
            }
        }
        self.notify_state_changed(account_id, region).await;
        Ok(())
    }

    fn notifier(&self) -> &StateChangeNotifier<Self::StateView> {
        &self.notifier
    }
}
