//! Core trait and types for Terraform resource converters.

use std::future::Future;
use std::pin::Pin;

use winterbaume_tfstate::ResourceInstance;

use crate::error::ConversionError;

/// Context for Terraform ↔ winterbaume conversion.
///
/// Provides default scope when the resource or provider does not
/// specify an account or region.
pub struct ConversionContext {
    /// Default AWS account ID (fallback when resource attrs lack one).
    pub default_account_id: String,
    /// Default AWS region (fallback when resource attrs lack one).
    pub default_region: String,
}

/// Result of a successful resource injection.
pub struct ConversionResult {
    /// The region the resource was injected into.
    pub region: String,
    /// Non-fatal warnings encountered during conversion.
    pub warnings: Vec<String>,
}

/// A resource extracted from the winterbaume service state, ready to be
/// written into a Terraform state file.
#[derive(Debug)]
pub struct ExtractedResource {
    /// The Terraform resource name.
    pub name: String,
    /// The account ID this resource was extracted from.
    pub account_id: String,
    /// The region this resource was extracted from.
    pub region: String,
    /// The Terraform resource attributes as a JSON object.
    pub attributes: serde_json::Value,
}

/// Trait for converting between Terraform resource instances and
/// winterbaume service state.
///
/// Each implementation handles a single Terraform resource type
/// (e.g. `aws_s3_bucket`, `aws_iam_role`).
pub trait TerraformResourceConverter: Send + Sync {
    /// The Terraform resource type this converter handles (e.g. `"aws_s3_bucket"`).
    fn resource_type(&self) -> &str;

    /// Resource types that must be processed before this one.
    fn depends_on_types(&self) -> Vec<&str> {
        vec![]
    }

    /// Inject a single Terraform resource instance into the winterbaume
    /// service state.
    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>>;

    /// Extract all resources of this type from the winterbaume service state.
    fn extract<'a>(
        &'a self,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>;
}
