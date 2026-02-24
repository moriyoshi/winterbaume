use std::collections::HashMap;

use crate::types::{
    AccessGrant, AccessGrantsInstance, AccessGrantsLocation, AccessGrantsResourcePolicy,
    AccessPoint, AccessPointScope, BucketLifecycleConfig, BucketReplicationConfig,
    BucketVersioningConfig, DirectoryBucketAccessPoint, Job, MrapRegion, MrapRoute,
    MultiRegionAccessPoint, OlAccessPoint, OutpostsBucket, PublicAccessBlock, StorageLensConfig,
    StorageLensGroup,
};

#[derive(Debug, thiserror::Error)]
pub enum S3ControlError {
    #[error("Access point '{0}' already exists")]
    AccessPointAlreadyOwnedByYou(String),
    #[error("Access point '{0}' does not exist")]
    NoSuchAccessPoint(String),
    #[error("The public access block configuration was not found")]
    NoSuchPublicAccessBlockConfiguration,
    #[error("An Access Grants instance already exists for this account")]
    AccessGrantsInstanceAlreadyExists,
    #[error("No Access Grants instance found for this account")]
    NoSuchAccessGrantsInstance,
    #[error("Access Grants location '{0}' does not exist")]
    NoSuchAccessGrantsLocation(String),
    #[error("Access grant '{0}' does not exist")]
    NoSuchAccessGrant(String),
    #[error("Multi-Region Access Point '{0}' already exists")]
    MultiRegionAccessPointAlreadyExists(String),
    #[error("Multi-Region Access Point '{0}' does not exist")]
    NoSuchMultiRegionAccessPoint(String),
    #[error("Job '{0}' does not exist")]
    NoSuchJob(String),
    #[error("Storage Lens group '{0}' already exists")]
    StorageLensGroupAlreadyExists(String),
    #[error("Storage Lens group '{0}' does not exist")]
    NoSuchStorageLensGroup(String),
    #[error("Object Lambda access point '{0}' already exists")]
    ObjectLambdaAccessPointAlreadyExists(String),
    #[error("Object Lambda access point '{0}' does not exist")]
    NoSuchObjectLambdaAccessPoint(String),
    #[error("Bucket '{0}' already exists")]
    BucketAlreadyExists(String),
    #[error("Bucket '{0}' does not exist")]
    NoSuchBucket(String),
    #[error("Storage Lens configuration '{0}' does not exist")]
    NoSuchStorageLensConfiguration(String),
    #[error("Storage Lens configuration '{0}' already exists")]
    StorageLensConfigurationAlreadyExists(String),
    #[error("Directory bucket access point '{0}' already exists")]
    DirectoryBucketAccessPointAlreadyExists(String),
}

/// Per-account, per-region state for S3 Control.
#[derive(Debug, Default)]
pub struct S3ControlState {
    /// Access points keyed by name.
    pub access_points: HashMap<String, AccessPoint>,
    /// Account-level public access block configuration (present if set).
    pub public_access_block: Option<PublicAccessBlock>,
    /// Access Grants Instance (one per account).
    pub access_grants_instance: Option<AccessGrantsInstance>,
    /// Access Grants Locations keyed by location ID.
    pub access_grants_locations: HashMap<String, AccessGrantsLocation>,
    /// Access Grants keyed by grant ID.
    pub access_grants: HashMap<String, AccessGrant>,
    /// Multi-Region Access Points keyed by name.
    pub multi_region_access_points: HashMap<String, MultiRegionAccessPoint>,
    /// S3 Batch Jobs keyed by job ID.
    pub jobs: HashMap<String, Job>,
    /// Storage Lens Groups keyed by name.
    pub storage_lens_groups: HashMap<String, StorageLensGroup>,
    /// Object Lambda Access Points keyed by name.
    pub object_lambda_access_points: HashMap<String, OlAccessPoint>,
    /// Outposts Buckets keyed by bucket name.
    pub outposts_buckets: HashMap<String, OutpostsBucket>,
    /// Resource-level tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Access point scopes keyed by access point name.
    pub access_point_scopes: HashMap<String, AccessPointScope>,
    /// Directory bucket access points keyed by name.
    pub directory_bucket_access_points: HashMap<String, DirectoryBucketAccessPoint>,
    /// Storage Lens configurations keyed by config ID.
    pub storage_lens_configs: HashMap<String, StorageLensConfig>,
    /// Outposts bucket lifecycle configurations keyed by bucket name.
    pub bucket_lifecycle_configs: HashMap<String, BucketLifecycleConfig>,
    /// Outposts bucket replication configurations keyed by bucket name.
    pub bucket_replication_configs: HashMap<String, BucketReplicationConfig>,
    /// Outposts bucket versioning configurations keyed by bucket name.
    pub bucket_versioning_configs: HashMap<String, BucketVersioningConfig>,
    /// MRAP route configurations keyed by MRAP name.
    pub mrap_routes: HashMap<String, Vec<MrapRoute>>,
}

impl S3ControlState {
    // -------------------------------------------------------------------------
    // Access Points
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_access_point(
        &mut self,
        name: String,
        bucket: String,
        account_id: String,
        region: String,
        vpc_id: Option<String>,
        block_public_acls: bool,
        ignore_public_acls: bool,
        block_public_policy: bool,
        restrict_public_buckets: bool,
        creation_date: String,
    ) -> Result<AccessPoint, S3ControlError> {
        if self.access_points.contains_key(&name) {
            return Err(S3ControlError::AccessPointAlreadyOwnedByYou(name));
        }

        let alias = format!(
            "{}-{}--x-s3",
            &name[..name.len().min(20)],
            &uuid::Uuid::new_v4().to_string()[..8]
        );
        let arn = format!("arn:aws:s3:{region}:{account_id}:accesspoint/{name}");
        let network_origin = if vpc_id.is_some() {
            "VPC".to_string()
        } else {
            "Internet".to_string()
        };

        let ap = AccessPoint {
            name: name.clone(),
            bucket,
            account_id,
            region,
            alias,
            arn,
            network_origin,
            vpc_id,
            block_public_acls,
            ignore_public_acls,
            block_public_policy,
            restrict_public_buckets,
            creation_date,
            policy: None,
        };

        self.access_points.insert(name, ap.clone());
        Ok(ap)
    }

    pub fn get_access_point(&self, name: &str) -> Result<&AccessPoint, S3ControlError> {
        self.access_points
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchAccessPoint(name.to_string()))
    }

    pub fn delete_access_point(&mut self, name: &str) -> Result<(), S3ControlError> {
        if self.access_points.remove(name).is_none() {
            return Err(S3ControlError::NoSuchAccessPoint(name.to_string()));
        }
        Ok(())
    }

    pub fn list_access_points(&self, bucket: Option<&str>) -> Vec<&AccessPoint> {
        self.access_points
            .values()
            .filter(|ap| bucket.is_none_or(|b| ap.bucket == b))
            .collect()
    }

    pub fn get_access_point_policy(&self, name: &str) -> Result<Option<&str>, S3ControlError> {
        self.access_points
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchAccessPoint(name.to_string()))
            .map(|ap| ap.policy.as_deref())
    }

    pub fn put_access_point_policy(
        &mut self,
        name: &str,
        policy: String,
    ) -> Result<(), S3ControlError> {
        let ap = self
            .access_points
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchAccessPoint(name.to_string()))?;
        ap.policy = Some(policy);
        Ok(())
    }

    pub fn delete_access_point_policy(&mut self, name: &str) -> Result<(), S3ControlError> {
        let ap = self
            .access_points
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchAccessPoint(name.to_string()))?;
        ap.policy = None;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Public Access Block
    // -------------------------------------------------------------------------

    pub fn get_public_access_block(&self) -> Result<&PublicAccessBlock, S3ControlError> {
        self.public_access_block
            .as_ref()
            .ok_or(S3ControlError::NoSuchPublicAccessBlockConfiguration)
    }

    pub fn put_public_access_block(&mut self, config: PublicAccessBlock) {
        self.public_access_block = Some(config);
    }

    pub fn delete_public_access_block(&mut self) -> Result<(), S3ControlError> {
        if self.public_access_block.take().is_none() {
            return Err(S3ControlError::NoSuchPublicAccessBlockConfiguration);
        }
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Access Grants Instance
    // -------------------------------------------------------------------------

    pub fn create_access_grants_instance(
        &mut self,
        account_id: &str,
        region: &str,
        identity_center_arn: Option<String>,
        created_at: String,
    ) -> Result<&AccessGrantsInstance, S3ControlError> {
        if self.access_grants_instance.is_some() {
            return Err(S3ControlError::AccessGrantsInstanceAlreadyExists);
        }
        let instance_id = uuid::Uuid::new_v4().to_string();
        let instance_arn = format!("arn:aws:s3:{region}:{account_id}:access-grants/default");
        let identity_center_instance_arn = identity_center_arn.clone();
        self.access_grants_instance = Some(AccessGrantsInstance {
            instance_id,
            instance_arn,
            created_at,
            identity_center_arn,
            identity_center_instance_arn,
            resource_policy: None,
        });
        Ok(self.access_grants_instance.as_ref().unwrap())
    }

    pub fn get_access_grants_instance(&self) -> Result<&AccessGrantsInstance, S3ControlError> {
        self.access_grants_instance
            .as_ref()
            .ok_or(S3ControlError::NoSuchAccessGrantsInstance)
    }

    pub fn delete_access_grants_instance(&mut self) -> Result<(), S3ControlError> {
        if self.access_grants_instance.take().is_none() {
            return Err(S3ControlError::NoSuchAccessGrantsInstance);
        }
        Ok(())
    }

    pub fn put_access_grants_resource_policy(
        &mut self,
        policy: String,
        organization: Option<String>,
        created_at: String,
    ) -> Result<&AccessGrantsResourcePolicy, S3ControlError> {
        let instance = self
            .access_grants_instance
            .as_mut()
            .ok_or(S3ControlError::NoSuchAccessGrantsInstance)?;
        instance.resource_policy = Some(AccessGrantsResourcePolicy {
            policy,
            created_at,
            organization,
        });
        Ok(instance.resource_policy.as_ref().unwrap())
    }

    pub fn get_access_grants_resource_policy(
        &self,
    ) -> Result<Option<&AccessGrantsResourcePolicy>, S3ControlError> {
        let instance = self
            .access_grants_instance
            .as_ref()
            .ok_or(S3ControlError::NoSuchAccessGrantsInstance)?;
        Ok(instance.resource_policy.as_ref())
    }

    pub fn delete_access_grants_resource_policy(&mut self) -> Result<(), S3ControlError> {
        let instance = self
            .access_grants_instance
            .as_mut()
            .ok_or(S3ControlError::NoSuchAccessGrantsInstance)?;
        instance.resource_policy = None;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Access Grants Locations
    // -------------------------------------------------------------------------

    pub fn create_access_grants_location(
        &mut self,
        account_id: &str,
        region: &str,
        location_scope: String,
        iam_role_arn: String,
        created_at: String,
    ) -> Result<&AccessGrantsLocation, S3ControlError> {
        let location_id = uuid::Uuid::new_v4().to_string();
        let location_arn = format!(
            "arn:aws:s3:{region}:{account_id}:access-grants/default/location/{location_id}"
        );
        let loc = AccessGrantsLocation {
            location_id: location_id.clone(),
            location_arn,
            location_scope,
            iam_role_arn,
            created_at,
        };
        self.access_grants_locations.insert(location_id, loc);
        Ok(self.access_grants_locations.values().last().unwrap())
    }

    pub fn get_access_grants_location(
        &self,
        location_id: &str,
    ) -> Result<&AccessGrantsLocation, S3ControlError> {
        self.access_grants_locations
            .get(location_id)
            .ok_or_else(|| S3ControlError::NoSuchAccessGrantsLocation(location_id.to_string()))
    }

    pub fn delete_access_grants_location(
        &mut self,
        location_id: &str,
    ) -> Result<(), S3ControlError> {
        if self.access_grants_locations.remove(location_id).is_none() {
            return Err(S3ControlError::NoSuchAccessGrantsLocation(
                location_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_access_grants_location(
        &mut self,
        location_id: &str,
        iam_role_arn: String,
    ) -> Result<&AccessGrantsLocation, S3ControlError> {
        let loc = self
            .access_grants_locations
            .get_mut(location_id)
            .ok_or_else(|| S3ControlError::NoSuchAccessGrantsLocation(location_id.to_string()))?;
        loc.iam_role_arn = iam_role_arn;
        Ok(loc)
    }

    pub fn list_access_grants_locations(&self) -> Vec<&AccessGrantsLocation> {
        self.access_grants_locations.values().collect()
    }

    // -------------------------------------------------------------------------
    // Access Grants
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_access_grant(
        &mut self,
        account_id: &str,
        region: &str,
        location_id: String,
        location_configuration_prefix: Option<String>,
        grantee_type: String,
        grantee_identifier: String,
        permission: String,
        application_arn: Option<String>,
        created_at: String,
    ) -> Result<&AccessGrant, S3ControlError> {
        let grant_id = uuid::Uuid::new_v4().to_string();
        let grant_arn =
            format!("arn:aws:s3:{region}:{account_id}:access-grants/default/grant/{grant_id}");
        // Compute grant scope from location scope + optional prefix
        let grant_scope = self.access_grants_locations.get(&location_id).map(|loc| {
            if let Some(ref prefix) = location_configuration_prefix {
                format!("{}{}", loc.location_scope.trim_end_matches('/'), prefix)
            } else {
                loc.location_scope.clone()
            }
        });

        let grant = AccessGrant {
            grant_id: grant_id.clone(),
            grant_arn,
            grantee_type,
            grantee_identifier,
            permission,
            grant_scope,
            location_id,
            location_configuration_prefix,
            application_arn,
            created_at,
        };
        self.access_grants.insert(grant_id, grant);
        Ok(self.access_grants.values().last().unwrap())
    }

    pub fn get_access_grant(&self, grant_id: &str) -> Result<&AccessGrant, S3ControlError> {
        self.access_grants
            .get(grant_id)
            .ok_or_else(|| S3ControlError::NoSuchAccessGrant(grant_id.to_string()))
    }

    pub fn delete_access_grant(&mut self, grant_id: &str) -> Result<(), S3ControlError> {
        if self.access_grants.remove(grant_id).is_none() {
            return Err(S3ControlError::NoSuchAccessGrant(grant_id.to_string()));
        }
        Ok(())
    }

    pub fn list_access_grants(&self) -> Vec<&AccessGrant> {
        self.access_grants.values().collect()
    }

    // -------------------------------------------------------------------------
    // Multi-Region Access Points
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_multi_region_access_point(
        &mut self,
        account_id: &str,
        name: String,
        regions: Vec<MrapRegion>,
        block_public_acls: bool,
        ignore_public_acls: bool,
        block_public_policy: bool,
        restrict_public_buckets: bool,
        created_at: String,
    ) -> Result<String, S3ControlError> {
        if self.multi_region_access_points.contains_key(&name) {
            return Err(S3ControlError::MultiRegionAccessPointAlreadyExists(name));
        }
        let request_token = uuid::Uuid::new_v4().to_string();
        let alias = format!("{}.mrap.amazonaws.com", uuid::Uuid::new_v4());
        let arn = format!("arn:aws:s3::{account_id}:accesspoint/{name}");
        let mrap = MultiRegionAccessPoint {
            name: name.clone(),
            alias,
            arn,
            created_at,
            status: "READY".to_string(),
            regions,
            block_public_acls,
            ignore_public_acls,
            block_public_policy,
            restrict_public_buckets,
            policy: None,
            request_token_arn: format!(
                "arn:aws:s3:us-east-1:{account_id}:async-request/mrap/create/{request_token}"
            ),
        };
        self.multi_region_access_points.insert(name, mrap);
        Ok(request_token)
    }

    pub fn get_multi_region_access_point(
        &self,
        name: &str,
    ) -> Result<&MultiRegionAccessPoint, S3ControlError> {
        self.multi_region_access_points
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchMultiRegionAccessPoint(name.to_string()))
    }

    pub fn delete_multi_region_access_point(
        &mut self,
        account_id: &str,
        name: &str,
    ) -> Result<String, S3ControlError> {
        if self.multi_region_access_points.remove(name).is_none() {
            return Err(S3ControlError::NoSuchMultiRegionAccessPoint(
                name.to_string(),
            ));
        }
        let request_token = uuid::Uuid::new_v4().to_string();
        Ok(format!(
            "arn:aws:s3:us-east-1:{account_id}:async-request/mrap/delete/{request_token}"
        ))
    }

    pub fn list_multi_region_access_points(&self) -> Vec<&MultiRegionAccessPoint> {
        self.multi_region_access_points.values().collect()
    }

    pub fn put_multi_region_access_point_policy(
        &mut self,
        name: &str,
        policy: String,
    ) -> Result<String, S3ControlError> {
        let mrap = self
            .multi_region_access_points
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchMultiRegionAccessPoint(name.to_string()))?;
        mrap.policy = Some(policy);
        // Return a fake async request token ARN
        Ok(mrap.request_token_arn.replace("/create/", "/put-policy/"))
    }

    // -------------------------------------------------------------------------
    // Jobs
    // -------------------------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_job(
        &mut self,
        account_id: &str,
        region: &str,
        priority: i32,
        role_arn: String,
        description: Option<String>,
        confirmation_required: bool,
        operation_type: String,
        created_at: String,
    ) -> Result<String, S3ControlError> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let job_arn = format!("arn:aws:s3:{region}:{account_id}:job/{job_id}");
        let status = if confirmation_required {
            "Suspended"
        } else {
            "New"
        }
        .to_string();
        let job = Job {
            job_id: job_id.clone(),
            job_arn,
            status,
            priority,
            description,
            role_arn,
            creation_time: created_at,
            confirmation_required,
            tags: Vec::new(),
            operation_type,
        };
        self.jobs.insert(job_id.clone(), job);
        Ok(job_id)
    }

    pub fn get_job(&self, job_id: &str) -> Result<&Job, S3ControlError> {
        self.jobs
            .get(job_id)
            .ok_or_else(|| S3ControlError::NoSuchJob(job_id.to_string()))
    }

    pub fn get_job_mut(&mut self, job_id: &str) -> Result<&mut Job, S3ControlError> {
        self.jobs
            .get_mut(job_id)
            .ok_or_else(|| S3ControlError::NoSuchJob(job_id.to_string()))
    }

    pub fn list_jobs(&self) -> Vec<&Job> {
        self.jobs.values().collect()
    }

    pub fn update_job_priority(
        &mut self,
        job_id: &str,
        priority: i32,
    ) -> Result<i32, S3ControlError> {
        let job = self.get_job_mut(job_id)?;
        job.priority = priority;
        Ok(priority)
    }

    pub fn update_job_status(
        &mut self,
        job_id: &str,
        requested_job_status: &str,
    ) -> Result<String, S3ControlError> {
        let job = self.get_job_mut(job_id)?;
        job.status = requested_job_status.to_string();
        Ok(requested_job_status.to_string())
    }

    pub fn get_job_tagging(&self, job_id: &str) -> Result<Vec<(String, String)>, S3ControlError> {
        let job = self.get_job(job_id)?;
        Ok(job.tags.clone())
    }

    pub fn put_job_tagging(
        &mut self,
        job_id: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), S3ControlError> {
        let job = self.get_job_mut(job_id)?;
        job.tags = tags;
        Ok(())
    }

    pub fn delete_job_tagging(&mut self, job_id: &str) -> Result<(), S3ControlError> {
        let job = self.get_job_mut(job_id)?;
        job.tags.clear();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Storage Lens Groups
    // -------------------------------------------------------------------------

    pub fn create_storage_lens_group(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        tags: HashMap<String, String>,
    ) -> Result<(), S3ControlError> {
        if self.storage_lens_groups.contains_key(&name) {
            return Err(S3ControlError::StorageLensGroupAlreadyExists(name));
        }
        let arn = format!("arn:aws:s3:{region}:{account_id}:storage-lens-group/{name}");
        self.storage_lens_groups.insert(
            name.clone(),
            StorageLensGroup {
                name,
                arn,
                region: region.to_string(),
                tags,
            },
        );
        Ok(())
    }

    pub fn get_storage_lens_group(&self, name: &str) -> Result<&StorageLensGroup, S3ControlError> {
        self.storage_lens_groups
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensGroup(name.to_string()))
    }

    pub fn delete_storage_lens_group(&mut self, name: &str) -> Result<(), S3ControlError> {
        if self.storage_lens_groups.remove(name).is_none() {
            return Err(S3ControlError::NoSuchStorageLensGroup(name.to_string()));
        }
        Ok(())
    }

    pub fn update_storage_lens_group(
        &mut self,
        name: &str,
        new_tags: HashMap<String, String>,
    ) -> Result<(), S3ControlError> {
        let group = self
            .storage_lens_groups
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensGroup(name.to_string()))?;
        group.tags = new_tags;
        Ok(())
    }

    pub fn list_storage_lens_groups(&self) -> Vec<&StorageLensGroup> {
        self.storage_lens_groups.values().collect()
    }

    // -------------------------------------------------------------------------
    // Object Lambda Access Points
    // -------------------------------------------------------------------------

    pub fn create_object_lambda_access_point(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        access_point_arn: String,
        creation_date: String,
        configuration_json: Option<String>,
    ) -> Result<&OlAccessPoint, S3ControlError> {
        if self.object_lambda_access_points.contains_key(&name) {
            return Err(S3ControlError::ObjectLambdaAccessPointAlreadyExists(name));
        }
        let arn = format!("arn:aws:s3-object-lambda:{region}:{account_id}:accesspoint/{name}");
        let alias = format!(
            "{}-{}--ol-s3",
            &name[..name.len().min(20)],
            &uuid::Uuid::new_v4().to_string()[..8]
        );
        let ap = OlAccessPoint {
            name: name.clone(),
            arn,
            alias,
            alias_status: "READY".to_string(),
            access_point_arn,
            creation_date,
            policy: None,
            configuration_json,
        };
        self.object_lambda_access_points.insert(name, ap);
        Ok(self.object_lambda_access_points.values().last().unwrap())
    }

    pub fn get_object_lambda_access_point(
        &self,
        name: &str,
    ) -> Result<&OlAccessPoint, S3ControlError> {
        self.object_lambda_access_points
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchObjectLambdaAccessPoint(name.to_string()))
    }

    pub fn delete_object_lambda_access_point(&mut self, name: &str) -> Result<(), S3ControlError> {
        if self.object_lambda_access_points.remove(name).is_none() {
            return Err(S3ControlError::NoSuchObjectLambdaAccessPoint(
                name.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_object_lambda_access_points(&self) -> Vec<&OlAccessPoint> {
        self.object_lambda_access_points.values().collect()
    }

    pub fn get_object_lambda_access_point_policy(
        &self,
        name: &str,
    ) -> Result<Option<&str>, S3ControlError> {
        self.object_lambda_access_points
            .get(name)
            .ok_or_else(|| S3ControlError::NoSuchObjectLambdaAccessPoint(name.to_string()))
            .map(|ap| ap.policy.as_deref())
    }

    pub fn put_object_lambda_access_point_policy(
        &mut self,
        name: &str,
        policy: String,
    ) -> Result<(), S3ControlError> {
        let ap = self
            .object_lambda_access_points
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchObjectLambdaAccessPoint(name.to_string()))?;
        ap.policy = Some(policy);
        Ok(())
    }

    pub fn delete_object_lambda_access_point_policy(
        &mut self,
        name: &str,
    ) -> Result<(), S3ControlError> {
        let ap = self
            .object_lambda_access_points
            .get_mut(name)
            .ok_or_else(|| S3ControlError::NoSuchObjectLambdaAccessPoint(name.to_string()))?;
        ap.policy = None;
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Outposts Buckets
    // -------------------------------------------------------------------------

    pub fn create_outposts_bucket(
        &mut self,
        account_id: &str,
        name: String,
        outpost_id: String,
        creation_date: String,
    ) -> Result<String, S3ControlError> {
        if self.outposts_buckets.contains_key(&name) {
            return Err(S3ControlError::BucketAlreadyExists(name));
        }
        let arn = format!("arn:aws:s3-outposts::{account_id}:outpost/{outpost_id}/bucket/{name}");
        let bucket_arn = arn.clone();
        self.outposts_buckets.insert(
            name,
            OutpostsBucket {
                name: bucket_arn.clone(),
                arn: arn.clone(),
                outpost_id,
                creation_date,
                public_access_block_enabled: true,
                policy: None,
                tags: Vec::new(),
            },
        );
        Ok(arn)
    }

    pub fn get_outposts_bucket(&self, bucket: &str) -> Result<&OutpostsBucket, S3ControlError> {
        self.outposts_buckets
            .get(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))
    }

    pub fn delete_outposts_bucket(&mut self, bucket: &str) -> Result<(), S3ControlError> {
        if self.outposts_buckets.remove(bucket).is_none() {
            return Err(S3ControlError::NoSuchBucket(bucket.to_string()));
        }
        Ok(())
    }

    pub fn list_outposts_buckets(&self) -> Vec<&OutpostsBucket> {
        self.outposts_buckets.values().collect()
    }

    pub fn get_outposts_bucket_policy(&self, bucket: &str) -> Result<Option<&str>, S3ControlError> {
        self.outposts_buckets
            .get(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))
            .map(|b| b.policy.as_deref())
    }

    pub fn put_outposts_bucket_policy(
        &mut self,
        bucket: &str,
        policy: String,
    ) -> Result<(), S3ControlError> {
        let b = self
            .outposts_buckets
            .get_mut(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))?;
        b.policy = Some(policy);
        Ok(())
    }

    pub fn delete_outposts_bucket_policy(&mut self, bucket: &str) -> Result<(), S3ControlError> {
        let b = self
            .outposts_buckets
            .get_mut(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))?;
        b.policy = None;
        Ok(())
    }

    pub fn get_outposts_bucket_tagging(
        &self,
        bucket: &str,
    ) -> Result<Vec<(String, String)>, S3ControlError> {
        let b = self
            .outposts_buckets
            .get(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))?;
        Ok(b.tags.clone())
    }

    pub fn put_outposts_bucket_tagging(
        &mut self,
        bucket: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), S3ControlError> {
        let b = self
            .outposts_buckets
            .get_mut(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))?;
        b.tags = tags;
        Ok(())
    }

    pub fn delete_outposts_bucket_tagging(&mut self, bucket: &str) -> Result<(), S3ControlError> {
        let b = self
            .outposts_buckets
            .get_mut(bucket)
            .ok_or_else(|| S3ControlError::NoSuchBucket(bucket.to_string()))?;
        b.tags.clear();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Resource Tags
    // -------------------------------------------------------------------------

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        entry.extend(tags);
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    // -------------------------------------------------------------------------
    // Access Point Scopes
    // -------------------------------------------------------------------------

    pub fn get_access_point_scope(
        &self,
        name: &str,
    ) -> Result<Option<&AccessPointScope>, S3ControlError> {
        // Validate the access point exists
        self.get_access_point(name)?;
        Ok(self.access_point_scopes.get(name))
    }

    pub fn put_access_point_scope(
        &mut self,
        name: &str,
        scope: AccessPointScope,
    ) -> Result<(), S3ControlError> {
        self.get_access_point(name)?;
        self.access_point_scopes.insert(name.to_string(), scope);
        Ok(())
    }

    pub fn delete_access_point_scope(&mut self, name: &str) -> Result<(), S3ControlError> {
        self.get_access_point(name)?;
        self.access_point_scopes.remove(name);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Directory Bucket Access Points
    // -------------------------------------------------------------------------

    pub fn list_directory_bucket_access_points(&self) -> Vec<&DirectoryBucketAccessPoint> {
        self.directory_bucket_access_points.values().collect()
    }

    // -------------------------------------------------------------------------
    // Storage Lens Configurations
    // -------------------------------------------------------------------------

    pub fn put_storage_lens_configuration(
        &mut self,
        config_id: &str,
        account_id: &str,
        region: &str,
        is_enabled: bool,
    ) {
        let arn = format!("arn:aws:s3:{region}:{account_id}:storage-lens/{config_id}");
        let config = StorageLensConfig {
            config_id: config_id.to_string(),
            arn,
            home_region: region.to_string(),
            is_enabled,
            tags: Vec::new(),
        };
        self.storage_lens_configs
            .insert(config_id.to_string(), config);
    }

    pub fn get_storage_lens_configuration(
        &self,
        config_id: &str,
    ) -> Result<&StorageLensConfig, S3ControlError> {
        self.storage_lens_configs
            .get(config_id)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensConfiguration(config_id.to_string()))
    }

    pub fn delete_storage_lens_configuration(
        &mut self,
        config_id: &str,
    ) -> Result<(), S3ControlError> {
        if self.storage_lens_configs.remove(config_id).is_none() {
            return Err(S3ControlError::NoSuchStorageLensConfiguration(
                config_id.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_storage_lens_configurations(&self) -> Vec<&StorageLensConfig> {
        self.storage_lens_configs.values().collect()
    }

    pub fn get_storage_lens_configuration_tagging(
        &self,
        config_id: &str,
    ) -> Result<Vec<(String, String)>, S3ControlError> {
        let config = self
            .storage_lens_configs
            .get(config_id)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensConfiguration(config_id.to_string()))?;
        Ok(config.tags.clone())
    }

    pub fn put_storage_lens_configuration_tagging(
        &mut self,
        config_id: &str,
        tags: Vec<(String, String)>,
    ) -> Result<(), S3ControlError> {
        let config = self
            .storage_lens_configs
            .get_mut(config_id)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensConfiguration(config_id.to_string()))?;
        config.tags = tags;
        Ok(())
    }

    pub fn delete_storage_lens_configuration_tagging(
        &mut self,
        config_id: &str,
    ) -> Result<(), S3ControlError> {
        let config = self
            .storage_lens_configs
            .get_mut(config_id)
            .ok_or_else(|| S3ControlError::NoSuchStorageLensConfiguration(config_id.to_string()))?;
        config.tags.clear();
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Outposts Bucket Lifecycle
    // -------------------------------------------------------------------------

    pub fn get_bucket_lifecycle_configuration(
        &self,
        bucket: &str,
    ) -> Result<Option<&BucketLifecycleConfig>, S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        Ok(self.bucket_lifecycle_configs.get(bucket))
    }

    pub fn put_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
        config: BucketLifecycleConfig,
    ) -> Result<(), S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        self.bucket_lifecycle_configs
            .insert(bucket.to_string(), config);
        Ok(())
    }

    pub fn delete_bucket_lifecycle_configuration(
        &mut self,
        bucket: &str,
    ) -> Result<(), S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        self.bucket_lifecycle_configs.remove(bucket);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Outposts Bucket Replication
    // -------------------------------------------------------------------------

    pub fn get_bucket_replication(
        &self,
        bucket: &str,
    ) -> Result<Option<&BucketReplicationConfig>, S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        Ok(self.bucket_replication_configs.get(bucket))
    }

    pub fn put_bucket_replication(
        &mut self,
        bucket: &str,
        config: BucketReplicationConfig,
    ) -> Result<(), S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        self.bucket_replication_configs
            .insert(bucket.to_string(), config);
        Ok(())
    }

    pub fn delete_bucket_replication(&mut self, bucket: &str) -> Result<(), S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        self.bucket_replication_configs.remove(bucket);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // Outposts Bucket Versioning
    // -------------------------------------------------------------------------

    pub fn get_bucket_versioning(
        &self,
        bucket: &str,
    ) -> Result<Option<&BucketVersioningConfig>, S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        Ok(self.bucket_versioning_configs.get(bucket))
    }

    pub fn put_bucket_versioning(
        &mut self,
        bucket: &str,
        config: BucketVersioningConfig,
    ) -> Result<(), S3ControlError> {
        self.get_outposts_bucket(bucket)?;
        self.bucket_versioning_configs
            .insert(bucket.to_string(), config);
        Ok(())
    }

    // -------------------------------------------------------------------------
    // MRAP Routes
    // -------------------------------------------------------------------------

    pub fn get_mrap_routes(&self, mrap_name: &str) -> Result<Vec<&MrapRoute>, S3ControlError> {
        self.get_multi_region_access_point(mrap_name)?;
        Ok(self
            .mrap_routes
            .get(mrap_name)
            .map(|v| v.iter().collect())
            .unwrap_or_default())
    }

    pub fn submit_mrap_routes(
        &mut self,
        mrap_name: &str,
        routes: Vec<MrapRoute>,
    ) -> Result<(), S3ControlError> {
        self.get_multi_region_access_point(mrap_name)?;
        self.mrap_routes.insert(mrap_name.to_string(), routes);
        Ok(())
    }
}
