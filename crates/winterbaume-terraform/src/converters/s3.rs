//! Terraform converters for S3 resources.
//!
//! Most generated TfModel structs are imported from
//! `crate::generated::s3` and consumed only as typed deserialize
//! helpers. The bucket sub-resource converters use the
//! snapshot+mutate+restore idiom (mirroring the IAM role/user policy
//! converters): the converter calls `service.snapshot()`, mutates the
//! relevant field on the bucket entry in-place, and writes the result
//! back via `service.restore()`. Nested HCL blocks (`rule`, `filter`,
//! `cors_rule`, ...) live outside the spec vocabulary, so they are
//! read raw from `instance.attributes` and serialised with
//! `serde_json::to_string` before being stored on the bucket view as
//! an `Option<String>` config blob.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_s3::S3Service;
use winterbaume_s3::views::{
    BucketStateView, ObjectView, PublicAccessBlockConfigView, S3StateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::s3 as s3_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

/// Helper: serialise a JSON value to a compact JSON string, or return
/// `None` if the value is null/missing.
fn json_string_opt(v: Option<&serde_json::Value>) -> Option<String> {
    let val = v?;
    if val.is_null() {
        return None;
    }
    serde_json::to_string(val).ok()
}

/// Helper: synthesise a deterministic-ish fake ETag.
fn generate_etag() -> String {
    uuid::Uuid::new_v4().to_string().replace('-', "")
}

// ===========================================================================
// aws_s3_bucket
// ===========================================================================

/// Converts `aws_s3_bucket` Terraform resources to/from S3 service state.
pub struct AwsS3BucketConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketConverter {
    /// Create a new converter backed by the given S3 service.
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3BucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_bucket"
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

impl AwsS3BucketConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: s3_gen::BucketStateTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_s3_bucket", e))?;

        // bucket has a fallback chain (`bucket` -> `id`); both are spec-`string?`.
        let bucket_name = model
            .bucket
            .clone()
            .or_else(|| model.id.clone())
            .ok_or_else(|| ConversionError::MissingAttribute {
                resource_type: "aws_s3_bucket".to_string(),
                attribute: "bucket".to_string(),
            })?;

        // force_destroy is accepted but not stored (Terraform-only lifecycle flag).
        let _force_destroy = attrs.get("force_destroy").and_then(|v| v.as_bool());

        let bucket_view = BucketStateView {
            name: bucket_name.clone(),
            region: region.clone(),
            creation_date: model.creation_date,
            tags: extract_tags(attrs),
            acl: model.acl,
            policy: model.policy,
            versioning_status: model.versioning.or(model.versioning_status),
            accelerate_status: model.acceleration_status,
            request_payment_payer: model
                .request_payer
                .unwrap_or_else(|| "BucketOwner".to_string()),
            encryption_configuration: model.server_side_encryption_configuration,
            logging_configuration: model.logging,
            cors_configuration: model.cors_rule,
            lifecycle_configuration: model.lifecycle_rule,
            website_configuration: model.website,
            replication_configuration: model.replication_configuration,
            object_lock_configuration: model.object_lock_configuration,
            notification_configuration: model.notification_configuration,
            ownership_controls: model.ownership_controls,
            ..Default::default()
        };

        let mut state_view = S3StateView {
            buckets: HashMap::new(),
        };
        state_view.buckets.insert(bucket_name, bucket_view);

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
        for bucket in view.buckets.values() {
            let mut attrs = serde_json::json!({
                "id": bucket.name,
                "bucket": bucket.name,
                "region": bucket.region,
                "arn": format!("arn:aws:s3:::{}", bucket.name),
                "bucket_domain_name": format!("{}.s3.amazonaws.com", bucket.name),
                "bucket_regional_domain_name": format!(
                    "{}.s3.{}.amazonaws.com", bucket.name, bucket.region
                ),
                "request_payer": bucket.request_payment_payer,
            });
            let obj = attrs.as_object_mut().unwrap();
            if !bucket.tags.is_empty() {
                obj.insert("tags".to_string(), serde_json::json!(bucket.tags));
                obj.insert("tags_all".to_string(), serde_json::json!(bucket.tags));
            }
            if let Some(ref v) = bucket.acl {
                obj.insert("acl".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.policy {
                obj.insert("policy".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.versioning_status {
                obj.insert("versioning".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.accelerate_status {
                obj.insert("acceleration_status".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.encryption_configuration {
                obj.insert(
                    "server_side_encryption_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.logging_configuration {
                obj.insert("logging".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.cors_configuration {
                obj.insert("cors_rule".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.lifecycle_configuration {
                obj.insert("lifecycle_rule".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.website_configuration {
                obj.insert("website".to_string(), serde_json::json!(v));
            }
            if let Some(ref v) = bucket.replication_configuration {
                obj.insert(
                    "replication_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.object_lock_configuration {
                obj.insert(
                    "object_lock_configuration".to_string(),
                    serde_json::json!(v),
                );
            }
            if let Some(ref v) = bucket.ownership_controls {
                obj.insert("ownership_controls".to_string(), serde_json::json!(v));
            }
            let attrs = attrs;
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

// ===========================================================================
// Shared helper for single-field bucket sub-resource converters.
// ===========================================================================
//
// Each converter below follows the same pattern:
//   1. Read the bucket name from the TF input.
//   2. Snapshot S3 state.
//   3. Locate the bucket by name. If missing, emit a warning and skip.
//   4. Mutate the relevant field on `BucketStateView`.
//   5. Restore.
//
// The macro keeps the boiler-plate concise. `mutate` is a closure
// that receives `&mut BucketStateView` and the raw `instance.attributes`
// JSON value; it returns the optional warning suffix.

macro_rules! impl_bucket_subresource_converter {
    (
        $struct_name:ident,
        $resource_type:literal,
        $model_ty:ty,
        $mutate:expr,
        $extract_field:expr,
        $extract_attr:literal $(,)?
    ) => {
        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn depends_on_types(&self) -> Vec<&str> {
                vec!["aws_s3_bucket"]
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move {
                    let _model: $model_ty =
                        serde_json::from_value(instance.attributes.clone())
                            .map_err(|e| classify_deserialize_error($resource_type, e))?;

                    let bucket_name = instance
                        .attributes
                        .get("bucket")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ConversionError::MissingAttribute {
                            resource_type: $resource_type.to_string(),
                            attribute: "bucket".to_string(),
                        })?
                        .to_string();

                    let mut view = self
                        .service
                        .snapshot(&ctx.default_account_id, &ctx.default_region)
                        .await;

                    let mut warnings = vec![];
                    if let Some(bucket) = view.buckets.get_mut(&bucket_name) {
                        let mutator: fn(&mut BucketStateView, &serde_json::Value) = $mutate;
                        mutator(bucket, &instance.attributes);
                    } else {
                        warnings.push(format!(
                            "bucket '{}' not found in state; {} skipped",
                            bucket_name, $resource_type
                        ));
                    }

                    self.service
                        .restore(&ctx.default_account_id, &ctx.default_region, view)
                        .await?;

                    Ok(ConversionResult {
                        region: ctx.default_region.clone(),
                        warnings,
                    })
                })
            }

            fn extract<'a>(
                &'a self,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move {
                    let view = self
                        .service
                        .snapshot(&ctx.default_account_id, &ctx.default_region)
                        .await;
                    let mut results = vec![];
                    for bucket in view.buckets.values() {
                        let field_getter: fn(&BucketStateView) -> Option<serde_json::Value> =
                            $extract_field;
                        let Some(field_val) = field_getter(bucket) else {
                            continue;
                        };
                        let mut attrs = serde_json::json!({
                            "id": bucket.name,
                            "bucket": bucket.name,
                        });
                        let obj = attrs.as_object_mut().unwrap();
                        obj.insert($extract_attr.to_string(), field_val);
                        results.push(ExtractedResource {
                            name: bucket.name.clone(),
                            account_id: ctx.default_account_id.clone(),
                            region: ctx.default_region.clone(),
                            attributes: attrs,
                        });
                    }
                    Ok(results)
                })
            }
        }
    };
}

// ===========================================================================
// aws_s3_bucket_accelerate_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_accelerate_configuration` Terraform resources.
///
/// Snapshots+mutates+restores the bucket's `accelerate_status` field.
pub struct AwsS3BucketAccelerateConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketAccelerateConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketAccelerateConfigurationConverter,
    "aws_s3_bucket_accelerate_configuration",
    s3_gen::BucketAccelerateConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(s) = attrs.get("status").and_then(|v| v.as_str()) {
            b.accelerate_status = Some(s.to_string());
        }
    },
    |b: &BucketStateView| b.accelerate_status.as_ref().map(|v| serde_json::json!(v)),
    "status",
);

// ===========================================================================
// aws_s3_bucket_acl
// ===========================================================================

/// Converts `aws_s3_bucket_acl` Terraform resources.
///
/// Inputs are either a canned `acl` string or an `access_control_policy`
/// nested block. The converter JSON-encodes whichever is present and
/// stores it on the bucket's `acl` field.
pub struct AwsS3BucketAclConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketAclConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketAclConverter,
    "aws_s3_bucket_acl",
    s3_gen::BucketAclTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(acl) = attrs.get("acl").and_then(|v| v.as_str()) {
            b.acl = Some(acl.to_string());
        } else if let Some(body) = json_string_opt(attrs.get("access_control_policy")) {
            b.acl = Some(body);
        }
    },
    |b: &BucketStateView| b.acl.as_ref().map(|v| serde_json::json!(v)),
    "acl",
);

// ===========================================================================
// aws_s3_bucket_cors_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_cors_configuration` Terraform resources.
pub struct AwsS3BucketCorsConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketCorsConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketCorsConfigurationConverter,
    "aws_s3_bucket_cors_configuration",
    s3_gen::BucketCorsConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(rules) = json_string_opt(attrs.get("cors_rule")) {
            b.cors_configuration = Some(rules);
        }
    },
    |b: &BucketStateView| {
        b.cors_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "cors_rule",
);

// ===========================================================================
// aws_s3_bucket_lifecycle_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_lifecycle_configuration` Terraform resources.
pub struct AwsS3BucketLifecycleConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketLifecycleConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketLifecycleConfigurationConverter,
    "aws_s3_bucket_lifecycle_configuration",
    s3_gen::BucketLifecycleConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(rules) = json_string_opt(attrs.get("rule")) {
            b.lifecycle_configuration = Some(rules);
        }
    },
    |b: &BucketStateView| {
        b.lifecycle_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "rule",
);

// ===========================================================================
// aws_s3_bucket_logging
// ===========================================================================

/// Converts `aws_s3_bucket_logging` Terraform resources.
///
/// Stores `{target_bucket, target_prefix}` as a JSON object on the
/// bucket's `logging_configuration` field.
pub struct AwsS3BucketLoggingConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketLoggingConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketLoggingConverter,
    "aws_s3_bucket_logging",
    s3_gen::BucketLoggingTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        let body = serde_json::json!({
            "target_bucket": attrs.get("target_bucket").cloned().unwrap_or(serde_json::Value::Null),
            "target_prefix": attrs.get("target_prefix").cloned().unwrap_or(serde_json::Value::Null),
        });
        if let Ok(s) = serde_json::to_string(&body) {
            b.logging_configuration = Some(s);
        }
    },
    |b: &BucketStateView| {
        b.logging_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "logging",
);

// ===========================================================================
// aws_s3_bucket_notification
// ===========================================================================

/// Converts `aws_s3_bucket_notification` Terraform resources.
///
/// Serialises optional `topic`, `queue`, `lambda_function` nested
/// blocks into a JSON object on `notification_configuration`.
pub struct AwsS3BucketNotificationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketNotificationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketNotificationConverter,
    "aws_s3_bucket_notification",
    s3_gen::BucketNotificationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        let body = serde_json::json!({
            "topic": attrs.get("topic").cloned().unwrap_or(serde_json::Value::Null),
            "queue": attrs.get("queue").cloned().unwrap_or(serde_json::Value::Null),
            "lambda_function": attrs
                .get("lambda_function")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
        });
        if let Ok(s) = serde_json::to_string(&body) {
            b.notification_configuration = Some(s);
        }
    },
    |b: &BucketStateView| {
        b.notification_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "notification_configuration",
);

// ===========================================================================
// aws_s3_bucket_object_lock_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_object_lock_configuration` Terraform resources.
pub struct AwsS3BucketObjectLockConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketObjectLockConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketObjectLockConfigurationConverter,
    "aws_s3_bucket_object_lock_configuration",
    s3_gen::BucketObjectLockConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        let body = serde_json::json!({
            "object_lock_enabled": attrs
                .get("object_lock_enabled")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            "rule": attrs.get("rule").cloned().unwrap_or(serde_json::Value::Null),
        });
        if let Ok(s) = serde_json::to_string(&body) {
            b.object_lock_configuration = Some(s);
        }
    },
    |b: &BucketStateView| {
        b.object_lock_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "object_lock_configuration",
);

// ===========================================================================
// aws_s3_bucket_ownership_controls
// ===========================================================================

/// Converts `aws_s3_bucket_ownership_controls` Terraform resources.
pub struct AwsS3BucketOwnershipControlsConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketOwnershipControlsConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketOwnershipControlsConverter,
    "aws_s3_bucket_ownership_controls",
    s3_gen::BucketOwnershipControlsTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(rule) = json_string_opt(attrs.get("rule")) {
            b.ownership_controls = Some(rule);
        }
    },
    |b: &BucketStateView| {
        b.ownership_controls
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "rule",
);

// ===========================================================================
// aws_s3_bucket_policy
// ===========================================================================

/// Converts `aws_s3_bucket_policy` Terraform resources.
///
/// The TF `policy` field is already a JSON document string; it is
/// assigned to the bucket's `policy` field directly without further
/// serialisation.
pub struct AwsS3BucketPolicyConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketPolicyConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketPolicyConverter,
    "aws_s3_bucket_policy",
    s3_gen::BucketPolicyTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(p) = attrs.get("policy").and_then(|v| v.as_str()) {
            b.policy = Some(p.to_string());
        }
    },
    |b: &BucketStateView| b.policy.as_ref().map(|v| serde_json::json!(v)),
    "policy",
);

// ===========================================================================
// aws_s3_bucket_replication_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_replication_configuration` Terraform resources.
pub struct AwsS3BucketReplicationConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketReplicationConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketReplicationConfigurationConverter,
    "aws_s3_bucket_replication_configuration",
    s3_gen::BucketReplicationConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        let body = serde_json::json!({
            "role": attrs.get("role").cloned().unwrap_or(serde_json::Value::Null),
            "rule": attrs.get("rule").cloned().unwrap_or(serde_json::Value::Null),
        });
        if let Ok(s) = serde_json::to_string(&body) {
            b.replication_configuration = Some(s);
        }
    },
    |b: &BucketStateView| {
        b.replication_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "replication_configuration",
);

// ===========================================================================
// aws_s3_bucket_request_payment_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_request_payment_configuration` Terraform resources.
///
/// `request_payment_payer` is a plain `String` (not Option), so the
/// converter assigns directly with no `Some` wrap.
pub struct AwsS3BucketRequestPaymentConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketRequestPaymentConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3BucketRequestPaymentConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_bucket_request_payment_configuration"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_s3_bucket"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let _model: s3_gen::BucketRequestPaymentConfigurationTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_s3_bucket_request_payment_configuration", e)
                })?;

            let bucket_name = instance
                .attributes
                .get("bucket")
                .and_then(|v| v.as_str())
                .ok_or_else(|| ConversionError::MissingAttribute {
                    resource_type: "aws_s3_bucket_request_payment_configuration".to_string(),
                    attribute: "bucket".to_string(),
                })?
                .to_string();
            let payer = instance
                .attributes
                .get("payer")
                .and_then(|v| v.as_str())
                .ok_or_else(|| ConversionError::MissingAttribute {
                    resource_type: "aws_s3_bucket_request_payment_configuration".to_string(),
                    attribute: "payer".to_string(),
                })?
                .to_string();

            let mut view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;

            let mut warnings = vec![];
            if let Some(bucket) = view.buckets.get_mut(&bucket_name) {
                bucket.request_payment_payer = payer;
            } else {
                warnings.push(format!(
                    "bucket '{}' not found in state; aws_s3_bucket_request_payment_configuration skipped",
                    bucket_name
                ));
            }

            self.service
                .restore(&ctx.default_account_id, &ctx.default_region, view)
                .await?;
            Ok(ConversionResult {
                region: ctx.default_region.clone(),
                warnings,
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for bucket in view.buckets.values() {
                // Only emit when the payer has been explicitly set to
                // something other than the AWS default ("BucketOwner").
                if bucket.request_payment_payer == "BucketOwner" {
                    continue;
                }
                let attrs = serde_json::json!({
                    "id": bucket.name,
                    "bucket": bucket.name,
                    "payer": bucket.request_payment_payer,
                });
                results.push(ExtractedResource {
                    name: bucket.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ===========================================================================
// aws_s3_bucket_server_side_encryption_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_server_side_encryption_configuration` Terraform resources.
pub struct AwsS3BucketServerSideEncryptionConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketServerSideEncryptionConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketServerSideEncryptionConfigurationConverter,
    "aws_s3_bucket_server_side_encryption_configuration",
    s3_gen::BucketServerSideEncryptionConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        if let Some(rule) = json_string_opt(attrs.get("rule")) {
            b.encryption_configuration = Some(rule);
        }
    },
    |b: &BucketStateView| {
        b.encryption_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "rule",
);

// ===========================================================================
// aws_s3_bucket_versioning
// ===========================================================================

/// Converts `aws_s3_bucket_versioning` Terraform resources.
///
/// Reads `versioning_configuration.status` from the raw attributes
/// and assigns it to the bucket's `versioning_status` field.
pub struct AwsS3BucketVersioningConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketVersioningConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketVersioningConverter,
    "aws_s3_bucket_versioning",
    s3_gen::BucketVersioningTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        // versioning_configuration is a nested block; status may live
        // at the top level when TF flattens it.
        let vc = attrs.get("versioning_configuration");
        let status = vc
            .and_then(|v| {
                // Sometimes a list, sometimes an object.
                if let Some(arr) = v.as_array() {
                    arr.first()
                        .and_then(|el| el.get("status"))
                        .and_then(|s| s.as_str())
                } else {
                    v.get("status").and_then(|s| s.as_str())
                }
            })
            .or_else(|| attrs.get("status").and_then(|v| v.as_str()));
        if let Some(s) = status {
            b.versioning_status = Some(s.to_string());
        }
    },
    |b: &BucketStateView| {
        b.versioning_status
            .as_ref()
            .map(|v| serde_json::json!([{ "status": v }]))
    },
    "versioning_configuration",
);

// ===========================================================================
// aws_s3_bucket_website_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_website_configuration` Terraform resources.
pub struct AwsS3BucketWebsiteConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketWebsiteConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_subresource_converter!(
    AwsS3BucketWebsiteConfigurationConverter,
    "aws_s3_bucket_website_configuration",
    s3_gen::BucketWebsiteConfigurationTfModel,
    |b: &mut BucketStateView, attrs: &serde_json::Value| {
        let body = serde_json::json!({
            "index_document": attrs
                .get("index_document")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            "error_document": attrs
                .get("error_document")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            "routing_rule": attrs
                .get("routing_rule")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
            "redirect_all_requests_to": attrs
                .get("redirect_all_requests_to")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
        });
        if let Ok(s) = serde_json::to_string(&body) {
            b.website_configuration = Some(s);
        }
    },
    |b: &BucketStateView| {
        b.website_configuration
            .as_ref()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(s).ok())
    },
    "website_configuration",
);

// ===========================================================================
// aws_s3_bucket_public_access_block
// ===========================================================================

/// Converts `aws_s3_bucket_public_access_block` Terraform resources.
///
/// Builds a typed `PublicAccessBlockConfigView` from four boolean
/// inputs. Unlike the other *_configuration converters, the target
/// field on `BucketStateView` is a strongly-typed struct, not a JSON
/// string.
pub struct AwsS3BucketPublicAccessBlockConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketPublicAccessBlockConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3BucketPublicAccessBlockConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_bucket_public_access_block"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_s3_bucket"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let model: s3_gen::BucketPublicAccessBlockTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_s3_bucket_public_access_block", e)
                })?;

            let bucket_name = model.bucket.clone();

            let mut view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut warnings = vec![];
            if let Some(bucket) = view.buckets.get_mut(&bucket_name) {
                bucket.public_access_block = Some(PublicAccessBlockConfigView {
                    block_public_acls: model.block_public_acls,
                    ignore_public_acls: model.ignore_public_acls,
                    block_public_policy: model.block_public_policy,
                    restrict_public_buckets: model.restrict_public_buckets,
                });
            } else {
                warnings.push(format!(
                    "bucket '{}' not found in state; aws_s3_bucket_public_access_block skipped",
                    bucket_name
                ));
            }

            self.service
                .restore(&ctx.default_account_id, &ctx.default_region, view)
                .await?;
            Ok(ConversionResult {
                region: ctx.default_region.clone(),
                warnings,
            })
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move {
            let view = self
                .service
                .snapshot(&ctx.default_account_id, &ctx.default_region)
                .await;
            let mut results = vec![];
            for bucket in view.buckets.values() {
                let Some(pab) = bucket.public_access_block.as_ref() else {
                    continue;
                };
                let attrs = serde_json::json!({
                    "id": bucket.name,
                    "bucket": bucket.name,
                    "block_public_acls": pab.block_public_acls,
                    "ignore_public_acls": pab.ignore_public_acls,
                    "block_public_policy": pab.block_public_policy,
                    "restrict_public_buckets": pab.restrict_public_buckets,
                });
                results.push(ExtractedResource {
                    name: bucket.name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
            Ok(results)
        })
    }
}

// ===========================================================================
// Multi-entry configuration HashMaps
// ===========================================================================
//
// `analytics_configurations`, `intelligent_tiering_configurations` and
// `metrics_configurations` are `HashMap<String, String>` on the
// `BucketStateView` keyed by the configuration name. The converters
// JSON-encode the entire TF input body (minus the bucket name) and
// `insert` it.

macro_rules! impl_bucket_named_config_converter {
    (
        $struct_name:ident,
        $resource_type:literal,
        $model_ty:ty,
        $field:ident $(,)?
    ) => {
        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn depends_on_types(&self) -> Vec<&str> {
                vec!["aws_s3_bucket"]
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move {
                    let _model: $model_ty =
                        serde_json::from_value(instance.attributes.clone())
                            .map_err(|e| classify_deserialize_error($resource_type, e))?;
                    let bucket_name = instance
                        .attributes
                        .get("bucket")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ConversionError::MissingAttribute {
                            resource_type: $resource_type.to_string(),
                            attribute: "bucket".to_string(),
                        })?
                        .to_string();
                    let name = instance
                        .attributes
                        .get("name")
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| ConversionError::MissingAttribute {
                            resource_type: $resource_type.to_string(),
                            attribute: "name".to_string(),
                        })?
                        .to_string();

                    // Build a JSON body from every attribute except
                    // `bucket` (which is positional, not part of the
                    // configuration body).
                    let mut body = serde_json::Map::new();
                    if let Some(obj) = instance.attributes.as_object() {
                        for (k, v) in obj {
                            if k == "bucket" {
                                continue;
                            }
                            body.insert(k.clone(), v.clone());
                        }
                    }
                    let body_json = serde_json::to_string(&serde_json::Value::Object(body))
                        .unwrap_or_else(|_| "{}".to_string());

                    let mut view = self
                        .service
                        .snapshot(&ctx.default_account_id, &ctx.default_region)
                        .await;
                    let mut warnings = vec![];
                    if let Some(bucket) = view.buckets.get_mut(&bucket_name) {
                        bucket.$field.insert(name, body_json);
                    } else {
                        warnings.push(format!(
                            "bucket '{}' not found in state; {} skipped",
                            bucket_name, $resource_type
                        ));
                    }
                    self.service
                        .restore(&ctx.default_account_id, &ctx.default_region, view)
                        .await?;
                    Ok(ConversionResult {
                        region: ctx.default_region.clone(),
                        warnings,
                    })
                })
            }

            fn extract<'a>(
                &'a self,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move {
                    let view = self
                        .service
                        .snapshot(&ctx.default_account_id, &ctx.default_region)
                        .await;
                    let mut results = vec![];
                    for bucket in view.buckets.values() {
                        for (cfg_name, body_str) in &bucket.$field {
                            let body_val: serde_json::Value =
                                serde_json::from_str(body_str).unwrap_or(serde_json::Value::Null);
                            let mut attrs = serde_json::json!({
                                "id": format!("{}:{}", bucket.name, cfg_name),
                                "bucket": bucket.name,
                                "name": cfg_name,
                            });
                            if let Some(obj) = body_val.as_object() {
                                for (k, v) in obj {
                                    if k == "name" {
                                        continue;
                                    }
                                    attrs
                                        .as_object_mut()
                                        .unwrap()
                                        .insert(k.clone(), v.clone());
                                }
                            }
                            results.push(ExtractedResource {
                                name: format!("{}:{}", bucket.name, cfg_name),
                                account_id: ctx.default_account_id.clone(),
                                region: ctx.default_region.clone(),
                                attributes: attrs,
                            });
                        }
                    }
                    Ok(results)
                })
            }
        }
    };
}

// ===========================================================================
// aws_s3_bucket_analytics_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_analytics_configuration` Terraform resources.
pub struct AwsS3BucketAnalyticsConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketAnalyticsConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_named_config_converter!(
    AwsS3BucketAnalyticsConfigurationConverter,
    "aws_s3_bucket_analytics_configuration",
    s3_gen::BucketAnalyticsConfigurationTfModel,
    analytics_configurations,
);

// ===========================================================================
// aws_s3_bucket_intelligent_tiering_configuration
// ===========================================================================

/// Converts `aws_s3_bucket_intelligent_tiering_configuration` Terraform resources.
pub struct AwsS3BucketIntelligentTieringConfigurationConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketIntelligentTieringConfigurationConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_named_config_converter!(
    AwsS3BucketIntelligentTieringConfigurationConverter,
    "aws_s3_bucket_intelligent_tiering_configuration",
    s3_gen::BucketIntelligentTieringConfigurationTfModel,
    intelligent_tiering_configurations,
);

// ===========================================================================
// aws_s3_bucket_metric
// ===========================================================================

/// Converts `aws_s3_bucket_metric` Terraform resources.
pub struct AwsS3BucketMetricConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketMetricConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl_bucket_named_config_converter!(
    AwsS3BucketMetricConverter,
    "aws_s3_bucket_metric",
    s3_gen::BucketMetricTfModel,
    metrics_configurations,
);

// ===========================================================================
// Object resources
// ===========================================================================
//
// `aws_s3_object`, `aws_s3_bucket_object` and `aws_s3_object_copy` all
// stash an `ObjectView` in the bucket's `objects` HashMap. The content
// body is *not* written to BlobStore — only the metadata (size, etag,
// storage class, ...) is captured.

/// Build an `ObjectView` from the raw TF instance attributes.
fn build_object_view_from_attrs(attrs: &serde_json::Value, key: &str) -> ObjectView {
    let content_str = attrs.get("content").and_then(|v| v.as_str());
    let content_length = content_str.map(|s| s.len() as u64).unwrap_or(0);
    let content_type = attrs
        .get("content_type")
        .and_then(|v| v.as_str())
        .unwrap_or("application/octet-stream")
        .to_string();
    let etag = attrs
        .get("etag")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(generate_etag);
    let storage_class = attrs
        .get("storage_class")
        .and_then(|v| v.as_str())
        .unwrap_or("STANDARD")
        .to_string();
    let acl = attrs
        .get("acl")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let version_id = attrs
        .get("version_id")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "null".to_string());

    // Extract metadata map (TF: map of strings).
    let metadata = attrs
        .get("metadata")
        .and_then(|v| v.as_object())
        .map(|obj| {
            obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect::<Vec<(String, String)>>()
        })
        .unwrap_or_default();

    // Extract per-object tags.
    let tags = attrs
        .get("tags")
        .and_then(|v| v.as_object())
        .map(|obj| {
            let mut out = HashMap::new();
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    out.insert(k.clone(), s.to_string());
                }
            }
            out
        })
        .unwrap_or_default();

    ObjectView {
        key: key.to_string(),
        blob_key: String::new(),
        content_length,
        content_type,
        etag,
        last_modified: Some(chrono::Utc::now().to_rfc3339()),
        storage_class,
        metadata,
        tags,
        acl,
        legal_hold_status: None,
        retention_mode: None,
        retain_until_date: None,
        version_id,
        blob_version_id: String::new(),
    }
}

/// Shared inject helper for the three object resources.
async fn inject_object(
    service: &S3Service,
    instance: &ResourceInstance,
    ctx: &ConversionContext,
    resource_type: &str,
) -> Result<ConversionResult, ConversionError> {
    let attrs = &instance.attributes;
    let bucket_name = attrs
        .get("bucket")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ConversionError::MissingAttribute {
            resource_type: resource_type.to_string(),
            attribute: "bucket".to_string(),
        })?
        .to_string();
    let key = attrs
        .get("key")
        .and_then(|v| v.as_str())
        .ok_or_else(|| ConversionError::MissingAttribute {
            resource_type: resource_type.to_string(),
            attribute: "key".to_string(),
        })?
        .to_string();

    let obj_view = build_object_view_from_attrs(attrs, &key);

    let mut view = service
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    let mut warnings = vec![];
    if let Some(bucket) = view.buckets.get_mut(&bucket_name) {
        bucket.objects.insert(key, obj_view);
    } else {
        warnings.push(format!(
            "bucket '{}' not found in state; {} skipped",
            bucket_name, resource_type
        ));
    }
    service
        .restore(&ctx.default_account_id, &ctx.default_region, view)
        .await?;
    Ok(ConversionResult {
        region: ctx.default_region.clone(),
        warnings,
    })
}

/// Shared extract helper. `resource_type` determines whether the
/// emitted attribute set should include the legacy/copy-specific
/// fields, but for now all three produce the same shape.
async fn extract_objects(
    service: &S3Service,
    ctx: &ConversionContext,
) -> Result<Vec<ExtractedResource>, ConversionError> {
    let view = service
        .snapshot(&ctx.default_account_id, &ctx.default_region)
        .await;
    let mut results = vec![];
    for bucket in view.buckets.values() {
        for (key, ov) in &bucket.objects {
            let id = format!("{}/{}", bucket.name, key);
            let mut attrs = serde_json::json!({
                "id": id,
                "bucket": bucket.name,
                "key": key,
                "content_type": ov.content_type,
                "etag": ov.etag,
                "storage_class": ov.storage_class,
                "version_id": ov.version_id,
            });
            let obj = attrs.as_object_mut().unwrap();
            if let Some(ref v) = ov.acl {
                obj.insert("acl".to_string(), serde_json::json!(v));
            }
            if !ov.metadata.is_empty() {
                let m: serde_json::Map<String, serde_json::Value> = ov
                    .metadata
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::json!(v)))
                    .collect();
                obj.insert("metadata".to_string(), serde_json::Value::Object(m));
            }
            if !ov.tags.is_empty() {
                obj.insert("tags".to_string(), serde_json::json!(ov.tags));
            }
            results.push(ExtractedResource {
                name: id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
    }
    Ok(results)
}

// ---------------------------------------------------------------------------
// aws_s3_object
// ---------------------------------------------------------------------------

/// Converts `aws_s3_object` Terraform resources.
///
/// Stashes an `ObjectView` (metadata only — no blob body) in the
/// bucket's `objects` HashMap. ETag/version_id are synthesised when
/// absent.
pub struct AwsS3ObjectConverter {
    service: Arc<S3Service>,
}

impl AwsS3ObjectConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3ObjectConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_object"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_s3_bucket"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let _model: s3_gen::S3ObjectTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_s3_object", e))?;
            inject_object(&self.service, instance, ctx, "aws_s3_object").await
        })
    }

    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { extract_objects(&self.service, ctx).await })
    }
}

// ---------------------------------------------------------------------------
// aws_s3_bucket_object  (legacy alias)
// ---------------------------------------------------------------------------

/// Converts `aws_s3_bucket_object` Terraform resources (legacy alias
/// for `aws_s3_object`).
pub struct AwsS3BucketObjectConverter {
    service: Arc<S3Service>,
}

impl AwsS3BucketObjectConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3BucketObjectConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_bucket_object"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_s3_bucket"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let _model: s3_gen::S3BucketObjectTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_s3_bucket_object", e))?;
            inject_object(&self.service, instance, ctx, "aws_s3_bucket_object").await
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // The legacy alias never round-trips on extract; emitting both
        // resource types for the same stored object would cause
        // duplicates. Prefer `aws_s3_object` as the canonical output.
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_s3_object_copy
// ---------------------------------------------------------------------------

/// Converts `aws_s3_object_copy` Terraform resources.
///
/// Mirrors `aws_s3_object` but the body is sourced from another S3
/// location (`source` is `s3://bucket/key`). The converter only
/// records the destination metadata.
pub struct AwsS3ObjectCopyConverter {
    service: Arc<S3Service>,
}

impl AwsS3ObjectCopyConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3ObjectCopyConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_object_copy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_s3_bucket"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let _model: s3_gen::S3ObjectCopyTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_s3_object_copy", e))?;
            inject_object(&self.service, instance, ctx, "aws_s3_object_copy").await
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Same rationale as aws_s3_bucket_object — only emit one TF
        // representation per stored object.
        Box::pin(async move { Ok(vec![]) })
    }
}

// ===========================================================================
// aws_s3_directory_bucket
// ===========================================================================

/// Converts `aws_s3_directory_bucket` (S3 Express One Zone) Terraform resources.
///
/// Directory buckets share the same `BucketStateView` struct as
/// regular buckets; the directory-specific inputs (`data_redundancy`,
/// `type`, `location`) are accepted but not persisted.
pub struct AwsS3DirectoryBucketConverter {
    service: Arc<S3Service>,
}

impl AwsS3DirectoryBucketConverter {
    pub fn new(service: Arc<S3Service>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsS3DirectoryBucketConverter {
    fn resource_type(&self) -> &str {
        "aws_s3_directory_bucket"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: s3_gen::S3DirectoryBucketTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_s3_directory_bucket", e))?;

            let bucket_name = model.bucket.clone();
            let bucket_view = BucketStateView {
                name: bucket_name.clone(),
                region: region.clone(),
                creation_date: None,
                tags: extract_tags(attrs),
                ..Default::default()
            };
            let mut state_view = S3StateView {
                buckets: HashMap::new(),
            };
            state_view.buckets.insert(bucket_name, bucket_view);
            self.service
                .merge(&ctx.default_account_id, &region, state_view)
                .await?;
            Ok(ConversionResult {
                region,
                warnings: vec![],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        // Directory buckets aren't distinguishable from regular buckets
        // in `BucketStateView` (the `type`/`data_redundancy` inputs are
        // dropped on inject), so we let `aws_s3_bucket`'s extract emit
        // the canonical representation and stay silent here.
        Box::pin(async move { Ok(vec![]) })
    }
}
