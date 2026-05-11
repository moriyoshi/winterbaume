//! Terraform converter for S3 Control resources.
//!
//! `AccessPointTfModel` and `BucketTfModel` are generated from
//! `specs/s3control.toml`. ARN templates, the constants
//! (`network_origin = "Internet"`, default `public_access_block_enabled
//! = true`), and the `tags` HashMap-to-`Vec<(String, String)>`
//! conversion are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_s3control::S3ControlService;
use winterbaume_s3control::views::{
    AccessGrantView, AccessGrantsInstanceView, AccessGrantsLocationView,
    AccessGrantsResourcePolicyView, AccessPointScopeView, AccessPointView,
    BucketLifecycleConfigView, MultiRegionAccessPointView, OlAccessPointView, OutpostsBucketView,
    S3ControlStateView, StorageLensConfigView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::s3control as s3control_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_s3control_access_point
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_point` Terraform resources to/from S3 Control state.
pub struct AwsS3controlAccessPointConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessPointConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessPointConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_point"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlAccessPointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessPointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_access_point", e))?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let alias = model.alias.clone().unwrap_or_default();
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3:{}:{}:accesspoint/{}",
                region, account_id, model.name
            )
        });
        let network_origin = model
            .network_origin
            .clone()
            .unwrap_or_else(|| "Internet".to_string());
        let creation_date = model.creation_date.clone().unwrap_or_default();

        let ap_view = AccessPointView {
            name: model.name.clone(),
            bucket: model.bucket,
            account_id: account_id.clone(),
            region: region.clone(),
            alias,
            arn,
            network_origin,
            vpc_id: model.vpc_id,
            block_public_acls: model.block_public_acls,
            ignore_public_acls: model.ignore_public_acls,
            block_public_policy: model.block_public_policy,
            restrict_public_buckets: model.restrict_public_buckets,
            creation_date,
            policy: model.policy,
        };

        let mut state_view = S3ControlStateView::default();
        state_view.access_points.insert(model.name, ap_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ap in view.access_points.values() {
            let attrs = serde_json::json!({
                "id": ap.name,
                "name": ap.name,
                "bucket": ap.bucket,
                "account_id": ap.account_id,
                "region": ap.region,
                "alias": ap.alias,
                "arn": ap.arn,
                "network_origin": ap.network_origin,
                "vpc_id": ap.vpc_id,
                "block_public_acls": ap.block_public_acls,
                "ignore_public_acls": ap.ignore_public_acls,
                "block_public_policy": ap.block_public_policy,
                "restrict_public_buckets": ap.restrict_public_buckets,
                "creation_date": ap.creation_date,
                "policy": ap.policy,
            });
            results.push(ExtractedResource {
                name: ap.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_bucket
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_bucket` Terraform resources to/from S3 Control state.
pub struct AwsS3controlBucketConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlBucketConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlBucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_bucket"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlBucketConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::BucketTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_bucket", e))?;

        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3-outposts:{}:{}:outpost/{}/bucket/{}",
                region, ctx.default_account_id, model.outpost_id, model.bucket
            )
        });
        let creation_date = model.creation_date.clone().unwrap_or_default();
        // public_access_block_enabled is not a typed model field because the
        // TF default is `true` (`bool` types in the spec default to `false`).
        let public_access_block_enabled = instance
            .attributes
            .get("public_access_block_enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let tags: Vec<(String, String)> = model.tags.into_iter().collect();

        let bucket_view = OutpostsBucketView {
            name: model.bucket.clone(),
            arn,
            outpost_id: model.outpost_id,
            creation_date,
            public_access_block_enabled,
            policy: model.policy,
            tags,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .outposts_buckets
            .insert(model.bucket, bucket_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for bucket in view.outposts_buckets.values() {
            let tags_map: HashMap<String, String> = bucket.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": bucket.arn,
                "bucket": bucket.name,
                "arn": bucket.arn,
                "outpost_id": bucket.outpost_id,
                "creation_date": bucket.creation_date,
                "public_access_block_enabled": bucket.public_access_block_enabled,
                "policy": bucket.policy,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: bucket.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_access_grant
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_grant` Terraform resources.
pub struct AwsS3controlAccessGrantConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessGrantConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessGrantConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_grant"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlAccessGrantConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessGrantTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_access_grant", e))?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        // grantee is a nested block in TF; pull from raw attributes.
        let grantee = instance.attributes.get("grantee");
        let grantee_type = grantee
            .and_then(|g| g.get("grantee_type"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let grantee_identifier = grantee
            .and_then(|g| g.get("grantee_identifier"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let grant_id = model.access_grant_id.clone().unwrap_or_default();
        let grant_arn = model.access_grant_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3:{}:{}:access-grants/default/grant/{}",
                region, account_id, grant_id
            )
        });
        let created_at = model.created_at.clone().unwrap_or_default();

        // location_configuration.s3_sub_prefix is a nested block; pull raw.
        let location_configuration_prefix = instance
            .attributes
            .get("location_configuration")
            .and_then(|lc| lc.get("s3_sub_prefix"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let grant_view = AccessGrantView {
            grant_id: grant_id.clone(),
            grant_arn,
            grantee_type,
            grantee_identifier,
            permission: model.permission,
            grant_scope: model.grant_scope,
            location_id: model.access_grants_location_id,
            location_configuration_prefix,
            application_arn: model.application_arn,
            created_at,
        };

        let mut state_view = S3ControlStateView::default();
        state_view.access_grants.insert(grant_id, grant_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for grant in view.access_grants.values() {
            let attrs = serde_json::json!({
                "id": grant.grant_id,
                "access_grant_id": grant.grant_id,
                "access_grant_arn": grant.grant_arn,
                "access_grants_location_id": grant.location_id,
                "permission": grant.permission,
                "grant_scope": grant.grant_scope,
                "application_arn": grant.application_arn,
                "created_at": grant.created_at,
            });
            results.push(ExtractedResource {
                name: grant.grant_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_access_grants_instance
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_grants_instance` Terraform resources.
pub struct AwsS3controlAccessGrantsInstanceConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessGrantsInstanceConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessGrantsInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_grants_instance"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlAccessGrantsInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessGrantsInstanceTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_access_grants_instance", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let instance_id = model
            .access_grants_instance_id
            .clone()
            .unwrap_or_else(|| "default".to_string());
        let instance_arn = model.access_grants_instance_arn.clone().unwrap_or_else(|| {
            format!("arn:aws:s3:{}:{}:access-grants/default", region, account_id)
        });
        let created_at = model.created_at.clone().unwrap_or_default();

        // Preserve existing resource_policy if already attached.
        let existing = self.service.snapshot(&account_id, &region).await;
        let resource_policy = existing
            .access_grants_instance
            .as_ref()
            .and_then(|i| i.resource_policy.clone());

        let instance_view = AccessGrantsInstanceView {
            instance_id,
            instance_arn,
            created_at,
            identity_center_arn: model.identity_center_arn,
            identity_center_instance_arn: model.identity_center_application_arn,
            resource_policy,
        };

        let state_view = S3ControlStateView {
            access_grants_instance: Some(instance_view),
            ..Default::default()
        };
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        if let Some(inst) = view.access_grants_instance {
            let attrs = serde_json::json!({
                "id": inst.instance_id,
                "access_grants_instance_id": inst.instance_id,
                "access_grants_instance_arn": inst.instance_arn,
                "identity_center_arn": inst.identity_center_arn,
                "identity_center_application_arn": inst.identity_center_instance_arn,
                "created_at": inst.created_at,
            });
            results.push(ExtractedResource {
                name: inst.instance_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_access_grants_instance_resource_policy
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_grants_instance_resource_policy` Terraform resources.
///
/// Modifier on the Access Grants Instance: snapshots the existing instance,
/// updates its `resource_policy`, then merges back.
pub struct AwsS3controlAccessGrantsInstanceResourcePolicyConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessGrantsInstanceResourcePolicyConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessGrantsInstanceResourcePolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_grants_instance_resource_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlAccessGrantsInstanceResourcePolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessGrantsInstanceResourcePolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_s3control_access_grants_instance_resource_policy",
                    e,
                )
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let snapshot = self.service.snapshot(&account_id, &region).await;
        let mut instance_view = match snapshot.access_grants_instance.clone() {
            Some(v) => v,
            None => {
                return Ok(ConversionResult {
                    region,
                    warnings: vec![
                        "aws_s3control_access_grants_instance_resource_policy: no aws_s3control_access_grants_instance found; create one before attaching a resource policy".to_string(),
                    ],
                });
            }
        };

        instance_view.resource_policy = Some(AccessGrantsResourcePolicyView {
            policy: model.policy,
            created_at: model.created_at.clone().unwrap_or_default(),
            organization: model.organization,
        });

        let state_view = S3ControlStateView {
            access_grants_instance: Some(instance_view),
            ..Default::default()
        };
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        if let Some(inst) = view.access_grants_instance
            && let Some(policy) = inst.resource_policy
        {
            let attrs = serde_json::json!({
                "id": inst.instance_id,
                "policy": policy.policy,
                "organization": policy.organization,
                "created_at": policy.created_at,
            });
            results.push(ExtractedResource {
                name: inst.instance_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_access_grants_location
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_grants_location` Terraform resources.
pub struct AwsS3controlAccessGrantsLocationConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessGrantsLocationConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessGrantsLocationConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_grants_location"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlAccessGrantsLocationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessGrantsLocationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_access_grants_location", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let location_id = model.access_grants_location_id.clone().unwrap_or_default();
        let location_arn = model.access_grants_location_arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3:{}:{}:access-grants/default/location/{}",
                region, account_id, location_id
            )
        });
        let created_at = model.created_at.clone().unwrap_or_default();

        let loc_view = AccessGrantsLocationView {
            location_id: location_id.clone(),
            location_arn,
            location_scope: model.location_scope,
            iam_role_arn: model.iam_role_arn,
            created_at,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .access_grants_locations
            .insert(location_id, loc_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for loc in view.access_grants_locations.values() {
            let attrs = serde_json::json!({
                "id": loc.location_id,
                "access_grants_location_id": loc.location_id,
                "access_grants_location_arn": loc.location_arn,
                "location_scope": loc.location_scope,
                "iam_role_arn": loc.iam_role_arn,
                "created_at": loc.created_at,
            });
            results.push(ExtractedResource {
                name: loc.location_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_access_point_policy
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_access_point_policy` Terraform resources.
///
/// Modifier on an existing access point: parses the access point name out
/// of the ARN, snapshots, updates `policy`, merges back.
pub struct AwsS3controlAccessPointPolicyConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlAccessPointPolicyConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlAccessPointPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_access_point_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsS3controlAccessPointPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::AccessPointPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_access_point_policy", e))?;

        // ARN format: arn:aws:s3:<region>:<account>:accesspoint/<name>
        let ap_name = model
            .access_point_arn
            .rsplit('/')
            .next()
            .unwrap_or("")
            .to_string();
        if ap_name.is_empty() {
            return Ok(ConversionResult {
                region,
                warnings: vec![format!(
                    "aws_s3control_access_point_policy: could not parse access point name from arn '{}'",
                    model.access_point_arn
                )],
            });
        }

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut ap_view = match snapshot.access_points.get(&ap_name).cloned() {
            Some(v) => v,
            None => {
                return Ok(ConversionResult {
                    region,
                    warnings: vec![format!(
                        "aws_s3control_access_point_policy: no access point '{}' found; create it before attaching a policy",
                        ap_name
                    )],
                });
            }
        };
        ap_view.policy = Some(model.policy);

        let mut state_view = S3ControlStateView::default();
        state_view.access_points.insert(ap_name, ap_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_bucket_lifecycle_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_bucket_lifecycle_configuration` Terraform resources.
///
/// The TF resource's `rule` blocks are kept opaque as a JSON string in the
/// state view's `rules_json` field.
pub struct AwsS3controlBucketLifecycleConfigurationConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlBucketLifecycleConfigurationConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlBucketLifecycleConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_bucket_lifecycle_configuration"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlBucketLifecycleConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::BucketLifecycleConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_bucket_lifecycle_configuration", e)
            })?;

        let rules_json = instance
            .attributes
            .get("rule")
            .map(|v| serde_json::to_string(v).unwrap_or_default())
            .unwrap_or_default();

        let config_view = BucketLifecycleConfigView { rules_json };

        let mut state_view = S3ControlStateView::default();
        state_view
            .bucket_lifecycle_configs
            .insert(model.bucket, config_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (bucket, cfg) in &view.bucket_lifecycle_configs {
            let rule = serde_json::from_str::<serde_json::Value>(&cfg.rules_json)
                .unwrap_or(serde_json::Value::Null);
            let attrs = serde_json::json!({
                "id": bucket,
                "bucket": bucket,
                "rule": rule,
            });
            results.push(ExtractedResource {
                name: bucket.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_bucket_policy
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_bucket_policy` Terraform resources.
///
/// Modifier on an existing Outposts bucket: snapshots, updates `policy`,
/// merges back.
pub struct AwsS3controlBucketPolicyConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlBucketPolicyConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlBucketPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_bucket_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsS3controlBucketPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::BucketPolicyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_s3control_bucket_policy", e))?;

        // The TF `bucket` attribute may be a bare name or an outpost-bucket ARN
        // (arn:aws:s3-outposts:<region>:<account>:outpost/<outpost_id>/bucket/<name>).
        // The state map is keyed by bucket name.
        let bucket_name = model
            .bucket
            .rsplit('/')
            .next()
            .unwrap_or(&model.bucket)
            .to_string();

        let snapshot = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut bucket_view = match snapshot.outposts_buckets.get(&bucket_name).cloned() {
            Some(v) => v,
            None => {
                return Ok(ConversionResult {
                    region,
                    warnings: vec![format!(
                        "aws_s3control_bucket_policy: no Outposts bucket '{}' found; create it before attaching a policy",
                        bucket_name
                    )],
                });
            }
        };
        bucket_view.policy = Some(model.policy);

        let mut state_view = S3ControlStateView::default();
        state_view.outposts_buckets.insert(bucket_name, bucket_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_directory_bucket_access_point_scope
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_directory_bucket_access_point_scope` Terraform resources.
///
/// The `scope` block holds `permissions` and `prefixes` lists; pulled from
/// raw attributes since scalar-only spec cannot model them directly.
pub struct AwsS3controlDirectoryBucketAccessPointScopeConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlDirectoryBucketAccessPointScopeConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlDirectoryBucketAccessPointScopeConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_directory_bucket_access_point_scope"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlDirectoryBucketAccessPointScopeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::DirectoryBucketAccessPointScopeTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_directory_bucket_access_point_scope", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let scope = instance.attributes.get("scope");
        let prefixes: Vec<String> = scope
            .and_then(|s| s.get("prefixes"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let permissions: Vec<String> = scope
            .and_then(|s| s.get("permissions"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let scope_view = AccessPointScopeView {
            prefixes,
            permissions,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .access_point_scopes
            .insert(model.name, scope_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for (name, scope) in &view.access_point_scopes {
            let attrs = serde_json::json!({
                "id": name,
                "name": name,
                "scope": {
                    "permissions": scope.permissions,
                    "prefixes": scope.prefixes,
                },
            });
            results.push(ExtractedResource {
                name: name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_multi_region_access_point
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_multi_region_access_point` Terraform resources.
///
/// The TF resource has a nested `details` block with `name`, `region` list,
/// and `public_access_block`. Scalar-only spec gives us top-level fields;
/// the rest is pulled from raw attributes.
pub struct AwsS3controlMultiRegionAccessPointConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlMultiRegionAccessPointConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlMultiRegionAccessPointConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_multi_region_access_point"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlMultiRegionAccessPointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::MultiRegionAccessPointTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_multi_region_access_point", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let details = instance.attributes.get("details");
        let name = details
            .and_then(|d| d.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            return Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_s3control_multi_region_access_point: missing details.name".to_string(),
                ],
            });
        }

        let pab = details.and_then(|d| d.get("public_access_block"));
        let block_public_acls = pab
            .and_then(|p| p.get("block_public_acls"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let ignore_public_acls = pab
            .and_then(|p| p.get("ignore_public_acls"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let block_public_policy = pab
            .and_then(|p| p.get("block_public_policy"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        let restrict_public_buckets = pab
            .and_then(|p| p.get("restrict_public_buckets"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let regions: Vec<winterbaume_s3control::views::MrapRegionView> = details
            .and_then(|d| d.get("region"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .map(|r| winterbaume_s3control::views::MrapRegionView {
                        bucket: r
                            .get("bucket")
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string(),
                        bucket_account_id: r
                            .get("bucket_account_id")
                            .and_then(|v| v.as_str())
                            .map(String::from),
                    })
                    .collect()
            })
            .unwrap_or_default();

        let alias = model.alias.clone().unwrap_or_default();
        let arn = model
            .arn
            .clone()
            .unwrap_or_else(|| format!("arn:aws:s3::{}:accesspoint/{}", account_id, alias));
        let status = model.status.clone().unwrap_or_else(|| "READY".to_string());

        // Preserve existing policy if set.
        let existing = self.service.snapshot(&account_id, &region).await;
        let policy = existing
            .multi_region_access_points
            .get(&name)
            .and_then(|m| m.policy.clone());

        let mrap_view = MultiRegionAccessPointView {
            name: name.clone(),
            alias,
            arn,
            created_at: String::new(),
            status,
            regions,
            block_public_acls,
            ignore_public_acls,
            block_public_policy,
            restrict_public_buckets,
            policy,
            request_token_arn: String::new(),
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .multi_region_access_points
            .insert(name, mrap_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for mrap in view.multi_region_access_points.values() {
            let attrs = serde_json::json!({
                "id": mrap.name,
                "alias": mrap.alias,
                "arn": mrap.arn,
                "status": mrap.status,
            });
            results.push(ExtractedResource {
                name: mrap.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_multi_region_access_point_policy
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_multi_region_access_point_policy` Terraform resources.
///
/// Modifier on an existing MRAP: pulls the access point name from the
/// nested `details.name` raw attribute, snapshots, updates `policy`,
/// merges back.
pub struct AwsS3controlMultiRegionAccessPointPolicyConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlMultiRegionAccessPointPolicyConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlMultiRegionAccessPointPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_multi_region_access_point_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsS3controlMultiRegionAccessPointPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::MultiRegionAccessPointPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_multi_region_access_point_policy", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let name = instance
            .attributes
            .get("details")
            .and_then(|d| d.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            return Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_s3control_multi_region_access_point_policy: missing details.name"
                        .to_string(),
                ],
            });
        }

        let policy = instance
            .attributes
            .get("details")
            .and_then(|d| d.get("policy"))
            .and_then(|v| v.as_str())
            .map(String::from)
            .or(model.proposed)
            .or(model.established);

        let snapshot = self.service.snapshot(&account_id, &region).await;
        let mut mrap_view = match snapshot.multi_region_access_points.get(&name).cloned() {
            Some(v) => v,
            None => {
                return Ok(ConversionResult {
                    region,
                    warnings: vec![format!(
                        "aws_s3control_multi_region_access_point_policy: no MRAP '{}' found; create it before attaching a policy",
                        name
                    )],
                });
            }
        };
        mrap_view.policy = policy;

        let mut state_view = S3ControlStateView::default();
        state_view
            .multi_region_access_points
            .insert(name, mrap_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_object_lambda_access_point
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_object_lambda_access_point` Terraform resources.
///
/// The TF `configuration` block (transformations, supporting access point)
/// is preserved as an opaque JSON string in `configuration_json`.
pub struct AwsS3controlObjectLambdaAccessPointConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlObjectLambdaAccessPointConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlObjectLambdaAccessPointConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_object_lambda_access_point"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlObjectLambdaAccessPointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::ObjectLambdaAccessPointTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_object_lambda_access_point", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let alias = model.alias.clone().unwrap_or_default();
        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3-object-lambda:{}:{}:accesspoint/{}",
                region, account_id, model.name
            )
        });

        let configuration_json = instance
            .attributes
            .get("configuration")
            .map(|v| serde_json::to_string(v).unwrap_or_default());

        // Preserve existing policy if already attached.
        let existing = self.service.snapshot(&account_id, &region).await;
        let policy = existing
            .object_lambda_access_points
            .get(&model.name)
            .and_then(|p| p.policy.clone());

        let ol_view = OlAccessPointView {
            name: model.name.clone(),
            arn,
            alias,
            alias_status: String::new(),
            access_point_arn: String::new(),
            creation_date: String::new(),
            policy,
            configuration_json,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .object_lambda_access_points
            .insert(model.name, ol_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for ap in view.object_lambda_access_points.values() {
            let configuration = ap
                .configuration_json
                .as_ref()
                .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
                .unwrap_or(serde_json::Value::Null);
            let attrs = serde_json::json!({
                "id": ap.name,
                "name": ap.name,
                "alias": ap.alias,
                "arn": ap.arn,
                "configuration": configuration,
            });
            results.push(ExtractedResource {
                name: ap.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_object_lambda_access_point_policy
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_object_lambda_access_point_policy` Terraform resources.
///
/// Modifier on an existing Object Lambda Access Point: snapshots, updates
/// `policy`, merges back.
pub struct AwsS3controlObjectLambdaAccessPointPolicyConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlObjectLambdaAccessPointPolicyConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlObjectLambdaAccessPointPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_object_lambda_access_point_policy"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

impl AwsS3controlObjectLambdaAccessPointPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::ObjectLambdaAccessPointPolicyTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_object_lambda_access_point_policy", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let snapshot = self.service.snapshot(&account_id, &region).await;
        let mut ol_view = match snapshot
            .object_lambda_access_points
            .get(&model.name)
            .cloned()
        {
            Some(v) => v,
            None => {
                return Ok(ConversionResult {
                    region,
                    warnings: vec![format!(
                        "aws_s3control_object_lambda_access_point_policy: no Object Lambda access point '{}' found; create it before attaching a policy",
                        model.name
                    )],
                });
            }
        };
        ol_view.policy = Some(model.policy);

        let mut state_view = S3ControlStateView::default();
        state_view
            .object_lambda_access_points
            .insert(model.name, ol_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_s3control_storage_lens_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_s3control_storage_lens_configuration` Terraform resources.
///
/// The deeply-nested `storage_lens_configuration` block (account_level,
/// data_export, etc.) is opaque to the state view; we capture only the
/// top-level identifiers, `is_enabled`, and `tags`.
pub struct AwsS3controlStorageLensConfigurationConverter {
    service: Arc<S3ControlService>,
}

impl AwsS3controlStorageLensConfigurationConverter {
    pub fn new(service: Arc<S3ControlService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3controlStorageLensConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_s3control_storage_lens_configuration"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move { self.do_inject(instance, ctx).await })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { self.do_extract(ctx).await })
    }
}

impl AwsS3controlStorageLensConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: s3control_gen::StorageLensConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_s3control_storage_lens_configuration", e)
            })?;

        let account_id = model
            .account_id
            .clone()
            .unwrap_or_else(|| ctx.default_account_id.clone());

        let arn = model.arn.clone().unwrap_or_else(|| {
            format!(
                "arn:aws:s3:{}:{}:storage-lens/{}",
                region, account_id, model.config_id
            )
        });

        let is_enabled = instance
            .attributes
            .get("storage_lens_configuration")
            .and_then(|c| c.get("enabled"))
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let tags: Vec<(String, String)> = model.tags.into_iter().collect();

        let config_view = StorageLensConfigView {
            config_id: model.config_id.clone(),
            arn,
            home_region: region.clone(),
            is_enabled,
            tags,
        };

        let mut state_view = S3ControlStateView::default();
        state_view
            .storage_lens_configs
            .insert(model.config_id, config_view);
        self.service.merge(&account_id, &region, state_view).await?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &ctx.default_region)
            .await;
        let mut results = vec![];
        for cfg in view.storage_lens_configs.values() {
            let tags_map: HashMap<String, String> = cfg.tags.iter().cloned().collect();
            let attrs = serde_json::json!({
                "id": cfg.config_id,
                "config_id": cfg.config_id,
                "arn": cfg.arn,
                "tags": tags_map,
            });
            results.push(ExtractedResource {
                name: cfg.config_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
