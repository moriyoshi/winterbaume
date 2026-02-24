//! Terraform converters for KMS resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_kms::KmsService;
use winterbaume_kms::views::{AliasView, KeyView, KmsStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_kms_key
// ---------------------------------------------------------------------------

/// Converts `aws_kms_key` Terraform resources to/from KMS key state.
pub struct AwsKmsKeyConverter {
    service: Arc<KmsService>,
}

impl AwsKmsKeyConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_key"
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

impl AwsKmsKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let key_id = optional_str(attrs, "key_id")
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:key/{}",
                region, ctx.default_account_id, key_id
            )
        });

        // Terraform uses `customer_master_key_spec` (older) or `key_spec` (newer)
        let key_spec = optional_str(attrs, "key_spec")
            .or_else(|| optional_str(attrs, "customer_master_key_spec"))
            .unwrap_or_else(|| "SYMMETRIC_DEFAULT".to_string());

        let key_usage = optional_str(attrs, "key_usage").unwrap_or_else(|| {
            if key_spec.contains("HMAC") {
                "GENERATE_VERIFY_MAC".to_string()
            } else if key_spec.starts_with("ECC") || key_spec.starts_with("SM2") {
                "SIGN_VERIFY".to_string()
            } else {
                "ENCRYPT_DECRYPT".to_string()
            }
        });

        let enabled = optional_bool(attrs, "is_enabled").unwrap_or(true);
        let key_rotation_enabled = optional_bool(attrs, "enable_key_rotation").unwrap_or(false);
        let description = optional_str(attrs, "description").unwrap_or_default();
        let _deletion_window_in_days = attrs
            .get("deletion_window_in_days")
            .and_then(|v| v.as_i64())
            .unwrap_or(30);
        let _policy = optional_str(attrs, "policy");
        let multi_region = optional_bool(attrs, "multi_region").unwrap_or(false);

        // Generate fake key material bytes for the mock.
        let key_material = uuid::Uuid::new_v4().as_bytes().to_vec();

        let mut key_tags = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    key_tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let key_view = KeyView {
            key_id: key_id.clone(),
            arn: arn.clone(),
            account_id: ctx.default_account_id.clone(),
            region: region.clone(),
            description,
            key_spec,
            key_usage,
            key_state: if enabled {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            },
            creation_date: Utc::now().to_rfc3339(),
            enabled,
            origin: "AWS_KMS".to_string(),
            key_manager: "CUSTOMER".to_string(),
            deletion_date: None,
            key_rotation_enabled,
            key_material,
            tags: key_tags,
            multi_region,
            public_key_der: None,
        };

        let mut state_view = KmsStateView {
            keys: HashMap::new(),
            aliases: HashMap::new(),
            grants: HashMap::new(),
            key_policies: HashMap::new(),
            key_rotations: HashMap::new(),
            custom_key_stores: HashMap::new(),
            imported_key_material: HashMap::new(),
        };
        state_view.keys.insert(key_id, key_view);
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
        for key in view.keys.values() {
            let attrs = serde_json::json!({
                "id": key.key_id,
                "key_id": key.key_id,
                "arn": key.arn,
                "description": key.description,
                "key_spec": key.key_spec,
                "key_usage": key.key_usage,
                "is_enabled": key.enabled,
                "enable_key_rotation": key.key_rotation_enabled,
                "multi_region": key.multi_region,
                "tags": key.tags,
                "tags_all": key.tags,
            });
            results.push(ExtractedResource {
                name: key.key_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_kms_alias
// ---------------------------------------------------------------------------

/// Converts `aws_kms_alias` Terraform resources to/from KMS alias state.
pub struct AwsKmsAliasConverter {
    service: Arc<KmsService>,
}

impl AwsKmsAliasConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_alias"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_kms_key"]
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

impl AwsKmsAliasConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let alias_name = require_str(attrs, "name", "aws_kms_alias")?;
        let target_key_id = require_str(attrs, "target_key_id", "aws_kms_alias")?;
        let region = extract_region(attrs, &ctx.default_region);

        let alias_arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:{}",
                region, ctx.default_account_id, alias_name
            )
        });

        let alias_view = AliasView {
            alias_name: alias_name.to_string(),
            alias_arn,
            target_key_id: target_key_id.to_string(),
        };

        let mut state_view = KmsStateView {
            keys: HashMap::new(),
            aliases: HashMap::new(),
            grants: HashMap::new(),
            key_policies: HashMap::new(),
            key_rotations: HashMap::new(),
            custom_key_stores: HashMap::new(),
            imported_key_material: HashMap::new(),
        };
        state_view
            .aliases
            .insert(alias_name.to_string(), alias_view);
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
        for alias in view.aliases.values() {
            let attrs = serde_json::json!({
                "id": alias.alias_name,
                "name": alias.alias_name,
                "arn": alias.alias_arn,
                "target_key_id": alias.target_key_id,
                "target_key_arn": alias.alias_arn,
            });
            results.push(ExtractedResource {
                name: alias.alias_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
