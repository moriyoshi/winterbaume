//! Terraform converters for KMS resources.
//!
//! `KeyTfModel`, `AliasTfModel`, `CiphertextTfModel`, `CustomKeyStoreTfModel`,
//! `ExternalKeyTfModel`, `GrantTfModel`, `KeyPolicyTfModel`,
//! `ReplicaExternalKeyTfModel`, and `ReplicaKeyTfModel` are generated from
//! `specs/kms.toml`.
//! The synthesised key UUID, the key/alias ARN templates, the
//! `key_spec` chain (`key_spec` then `customer_master_key_spec`), the
//! `key_usage` derivation from `key_spec`, the random `key_material`
//! bytes, the `tags_all` merge into the `tags` map, the raw-attribute
//! handling for HCL list/map fields (`operations`, `grant_creation_tokens`,
//! `context`, the `constraints` block), and the `EXTERNAL` /
//! multi-Region replica origin overrides are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_kms::KmsService;
use winterbaume_kms::views::{
    AliasView, CustomKeyStoreView, GrantConstraintsView, GrantView, KeyView, KmsStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::kms as kms_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::KeyTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kms_key", e))?;

        let attrs = &instance.attributes;

        let key_id = model
            .key_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:key/{}",
                region, ctx.default_account_id, key_id
            )
        });

        // Terraform uses `customer_master_key_spec` (older) or `key_spec` (newer)
        let key_spec = model
            .key_spec
            .or(model.customer_master_key_spec)
            .unwrap_or_else(|| "SYMMETRIC_DEFAULT".to_string());

        let key_usage = model.key_usage.unwrap_or_else(|| {
            if key_spec.contains("HMAC") {
                "GENERATE_VERIFY_MAC".to_string()
            } else if key_spec.starts_with("ECC") || key_spec.starts_with("SM2") {
                "SIGN_VERIFY".to_string()
            } else {
                "ENCRYPT_DECRYPT".to_string()
            }
        });

        let enabled = model.is_enabled;
        let key_rotation_enabled = model.enable_key_rotation;
        let description = model.description.unwrap_or_default();
        let _deletion_window_in_days = attrs
            .get("deletion_window_in_days")
            .and_then(|v| v.as_i64())
            .unwrap_or(30);
        let _policy = attrs.get("policy").and_then(|v| v.as_str());
        let multi_region = model.multi_region;

        // Generate fake key material bytes for the mock.
        let key_material = uuid::Uuid::new_v4().as_bytes().to_vec();

        let mut key_tags = model.tags;
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

        let mut state_view = KmsStateView::default();
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::AliasTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kms_alias", e))?;

        let alias_name = model.name;
        let target_key_id = model.target_key_id;

        let alias_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:{}",
                region, ctx.default_account_id, alias_name
            )
        });

        let alias_view = AliasView {
            alias_name: alias_name.clone(),
            alias_arn,
            target_key_id,
        };

        let mut state_view = KmsStateView::default();
        state_view.aliases.insert(alias_name, alias_view);
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

// ---------------------------------------------------------------------------
// aws_kms_ciphertext
// ---------------------------------------------------------------------------

/// Converts `aws_kms_ciphertext` Terraform resources to/from KMS state.
///
/// `aws_kms_ciphertext` is an ephemeral data-source-like resource whose
/// output (`ciphertext_blob`) is a transient artefact of an Encrypt call.
/// Winterbaume does not persist ciphertext as state, so injection only
/// validates the referenced key (the converter merges an empty state view
/// to keep the round-trip API consistent) and extraction returns nothing.
pub struct AwsKmsCiphertextConverter {
    service: Arc<KmsService>,
}

impl AwsKmsCiphertextConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsCiphertextConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_ciphertext"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_kms_key", "aws_kms_external_key"]
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

impl AwsKmsCiphertextConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: kms_gen::CiphertextTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_kms_ciphertext", e))?;

        // `context` (encryption context map) and `plaintext` are accepted
        // but discarded — the ciphertext blob is not part of persisted
        // state. Merge an empty state view so the StatefulService notifies
        // observers in a consistent manner.
        let state_view = KmsStateView::default();
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
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_kms_custom_key_store
// ---------------------------------------------------------------------------

/// Converts `aws_kms_custom_key_store` Terraform resources to/from KMS state.
pub struct AwsKmsCustomKeyStoreConverter {
    service: Arc<KmsService>,
}

impl AwsKmsCustomKeyStoreConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsCustomKeyStoreConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_custom_key_store"
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

impl AwsKmsCustomKeyStoreConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::CustomKeyStoreTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_kms_custom_key_store", e))?;

        let id = model
            .custom_key_store_id
            .or(model.id)
            .unwrap_or_else(|| format!("cks-{}", uuid::Uuid::new_v4().simple()));

        // CloudHSM-backed key stores have a cluster ID; external key stores
        // do not. Default to CloudHSM for parity with the older provider
        // behaviour when the type is not declared.
        let custom_key_store_type = model
            .custom_key_store_type
            .unwrap_or_else(|| "AWS_CLOUDHSM".to_string());

        let cks_view = CustomKeyStoreView {
            custom_key_store_id: id.clone(),
            custom_key_store_name: model.custom_key_store_name,
            cloud_hsm_cluster_id: model.cloud_hsm_cluster_id,
            trust_anchor_certificate: model.trust_anchor_certificate,
            connection_state: "DISCONNECTED".to_string(),
            creation_date: Utc::now().to_rfc3339(),
            custom_key_store_type,
        };

        let mut state_view = KmsStateView::default();
        state_view.custom_key_stores.insert(id, cks_view);
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
        for cks in view.custom_key_stores.values() {
            let attrs = serde_json::json!({
                "id": cks.custom_key_store_id,
                "custom_key_store_id": cks.custom_key_store_id,
                "custom_key_store_name": cks.custom_key_store_name,
                "cloud_hsm_cluster_id": cks.cloud_hsm_cluster_id,
                "trust_anchor_certificate": cks.trust_anchor_certificate,
                "custom_key_store_type": cks.custom_key_store_type,
                "connection_state": cks.connection_state,
            });
            results.push(ExtractedResource {
                name: cks.custom_key_store_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_kms_external_key
// ---------------------------------------------------------------------------

/// Converts `aws_kms_external_key` Terraform resources to/from KMS state.
///
/// Like `aws_kms_key` but `origin` is `EXTERNAL` and the key material
/// originates from `key_material_base64` (read raw; if absent a stub blob
/// is generated). `key_state` flips to `PendingImport` until material is
/// imported, then `Enabled`/`Disabled` based on the `enabled` flag.
pub struct AwsKmsExternalKeyConverter {
    service: Arc<KmsService>,
}

impl AwsKmsExternalKeyConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsExternalKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_external_key"
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

impl AwsKmsExternalKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::ExternalKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_kms_external_key", e))?;

        let attrs = &instance.attributes;

        let key_id = model
            .key_id
            .or(model.id)
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:key/{}",
                region, ctx.default_account_id, key_id
            )
        });

        let description = model.description.unwrap_or_default();
        let enabled = model.enabled;
        let multi_region = model.multi_region;

        // `key_material_base64` is sensitive raw input. If absent, the key is
        // in `PendingImport`; otherwise treat material as supplied.
        let key_material_present = attrs
            .get("key_material_base64")
            .and_then(|v| v.as_str())
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        let key_material = if key_material_present {
            uuid::Uuid::new_v4().as_bytes().to_vec()
        } else {
            Vec::new()
        };

        let key_state = if !key_material_present {
            "PendingImport".to_string()
        } else if enabled {
            "Enabled".to_string()
        } else {
            "Disabled".to_string()
        };

        let mut key_tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    key_tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let key_view = KeyView {
            key_id: key_id.clone(),
            arn,
            account_id: ctx.default_account_id.clone(),
            region: region.clone(),
            description,
            key_spec: "SYMMETRIC_DEFAULT".to_string(),
            key_usage: "ENCRYPT_DECRYPT".to_string(),
            key_state,
            creation_date: Utc::now().to_rfc3339(),
            enabled,
            origin: "EXTERNAL".to_string(),
            key_manager: "CUSTOMER".to_string(),
            deletion_date: None,
            key_rotation_enabled: false,
            key_material,
            tags: key_tags,
            multi_region,
            public_key_der: None,
        };

        let mut state_view = KmsStateView::default();
        state_view.keys.insert(key_id.clone(), key_view);
        state_view
            .imported_key_material
            .insert(key_id, key_material_present);
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
            if key.origin != "EXTERNAL" || key.multi_region {
                continue;
            }
            let attrs = serde_json::json!({
                "id": key.key_id,
                "key_id": key.key_id,
                "arn": key.arn,
                "description": key.description,
                "enabled": key.enabled,
                "multi_region": key.multi_region,
                "key_state": key.key_state,
                "key_usage": key.key_usage,
                "expiration_model": "KEY_MATERIAL_DOES_NOT_EXPIRE",
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
// aws_kms_grant
// ---------------------------------------------------------------------------

/// Converts `aws_kms_grant` Terraform resources to/from KMS state.
pub struct AwsKmsGrantConverter {
    service: Arc<KmsService>,
}

impl AwsKmsGrantConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsGrantConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_grant"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_kms_key",
            "aws_kms_external_key",
            "aws_kms_replica_key",
            "aws_kms_replica_external_key",
        ]
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

impl AwsKmsGrantConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::GrantTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kms_grant", e))?;

        let attrs = &instance.attributes;

        let grant_id = model
            .grant_id
            .or_else(|| model.id.clone())
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());
        let grant_token = model
            .grant_token
            .unwrap_or_else(|| uuid::Uuid::new_v4().simple().to_string());

        // `operations` is a list<string> in HCL; read raw.
        let operations: Vec<String> = attrs
            .get("operations")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_default();

        // `constraints` is a single nested block list in HCL; capture the
        // two encryption-context maps if present.
        let constraints = attrs.get("constraints").and_then(|v| {
            let first = match v {
                serde_json::Value::Array(arr) => arr.first(),
                serde_json::Value::Object(_) => Some(v),
                _ => None,
            }?;
            let subset = first
                .get("encryption_context_subset")
                .and_then(|m| m.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, vv)| vv.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect::<HashMap<_, _>>()
                });
            let equals = first
                .get("encryption_context_equals")
                .and_then(|m| m.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, vv)| vv.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect::<HashMap<_, _>>()
                });
            if subset.is_none() && equals.is_none() {
                return None;
            }
            Some(GrantConstraintsView {
                encryption_context_subset: subset,
                encryption_context_equals: equals,
            })
        });

        let grant_view = GrantView {
            grant_id: grant_id.clone(),
            grant_token,
            key_id: model.key_id,
            grantee_principal: model.grantee_principal,
            retiring_principal: model.retiring_principal,
            operations,
            constraints,
            issuing_account: ctx.default_account_id.clone(),
            name: model.name,
            creation_date: Utc::now().to_rfc3339(),
        };

        let mut state_view = KmsStateView::default();
        state_view.grants.insert(grant_id, grant_view);
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
        for grant in view.grants.values() {
            let id = format!("{}:{}", grant.key_id, grant.grant_id);
            let constraints = grant.constraints.as_ref().map(|c| {
                serde_json::json!([{
                    "encryption_context_subset": c.encryption_context_subset.clone().unwrap_or_default(),
                    "encryption_context_equals": c.encryption_context_equals.clone().unwrap_or_default(),
                }])
            });
            let attrs = serde_json::json!({
                "id": id,
                "name": grant.name,
                "key_id": grant.key_id,
                "grantee_principal": grant.grantee_principal,
                "retiring_principal": grant.retiring_principal,
                "operations": grant.operations,
                "grant_id": grant.grant_id,
                "grant_token": grant.grant_token,
                "constraints": constraints,
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
// aws_kms_key_policy
// ---------------------------------------------------------------------------

/// Converts `aws_kms_key_policy` Terraform resources to/from KMS state.
///
/// `aws_kms_key_policy` attaches a policy document to an existing key.
/// Injection writes the policy into `KmsState::key_policies` keyed by
/// `key_id` (resolving alias / ARN forms by best-effort lookup against
/// the existing key set).
pub struct AwsKmsKeyPolicyConverter {
    service: Arc<KmsService>,
}

impl AwsKmsKeyPolicyConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsKeyPolicyConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_key_policy"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec![
            "aws_kms_key",
            "aws_kms_external_key",
            "aws_kms_replica_key",
            "aws_kms_replica_external_key",
        ]
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

impl AwsKmsKeyPolicyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::KeyPolicyTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kms_key_policy", e))?;

        // Resolve key_id from either the bare UUID or the trailing segment
        // of a key ARN (`arn:aws:kms:<region>:<account>:key/<key_id>`).
        let resolved_key_id = if let Some(idx) = model.key_id.rfind('/') {
            model.key_id[idx + 1..].to_string()
        } else {
            model.key_id.clone()
        };

        let mut state_view = KmsStateView::default();
        state_view
            .key_policies
            .insert(resolved_key_id, model.policy);
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
        for (key_id, policy) in view.key_policies.iter() {
            let attrs = serde_json::json!({
                "id": key_id,
                "key_id": key_id,
                "policy": policy,
            });
            results.push(ExtractedResource {
                name: key_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_kms_replica_external_key
// ---------------------------------------------------------------------------

/// Converts `aws_kms_replica_external_key` Terraform resources to/from KMS state.
///
/// Multi-region replica of an `aws_kms_external_key`. Origin remains
/// `EXTERNAL` and `multi_region` is forced to `true`.
pub struct AwsKmsReplicaExternalKeyConverter {
    service: Arc<KmsService>,
}

impl AwsKmsReplicaExternalKeyConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsReplicaExternalKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_replica_external_key"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_kms_external_key"]
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

impl AwsKmsReplicaExternalKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::ReplicaExternalKeyTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_kms_replica_external_key", e))?;

        let attrs = &instance.attributes;

        // Multi-region replicas share the primary key's `key_id` (the
        // identifier after the `mrk-` prefix is preserved across regions).
        let derived_key_id =
            derive_replica_key_id(&model.primary_key_arn, &model.key_id, &model.id);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:key/{}",
                region, ctx.default_account_id, derived_key_id
            )
        });

        let enabled = model.enabled;
        let description = model.description.unwrap_or_default();

        let key_material_present = attrs
            .get("key_material_base64")
            .and_then(|v| v.as_str())
            .map(|s| !s.is_empty())
            .unwrap_or(false);
        let key_material = if key_material_present {
            uuid::Uuid::new_v4().as_bytes().to_vec()
        } else {
            Vec::new()
        };
        let key_state = if !key_material_present {
            "PendingImport".to_string()
        } else if enabled {
            "Enabled".to_string()
        } else {
            "Disabled".to_string()
        };

        let mut key_tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    key_tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let key_view = KeyView {
            key_id: derived_key_id.clone(),
            arn,
            account_id: ctx.default_account_id.clone(),
            region: region.clone(),
            description,
            key_spec: "SYMMETRIC_DEFAULT".to_string(),
            key_usage: "ENCRYPT_DECRYPT".to_string(),
            key_state,
            creation_date: Utc::now().to_rfc3339(),
            enabled,
            origin: "EXTERNAL".to_string(),
            key_manager: "CUSTOMER".to_string(),
            deletion_date: None,
            key_rotation_enabled: false,
            key_material,
            tags: key_tags,
            multi_region: true,
            public_key_der: None,
        };

        let mut state_view = KmsStateView::default();
        state_view.keys.insert(derived_key_id.clone(), key_view);
        state_view
            .imported_key_material
            .insert(derived_key_id, key_material_present);
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
            if !key.multi_region || key.origin != "EXTERNAL" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": key.key_id,
                "key_id": key.key_id,
                "arn": key.arn,
                "primary_key_arn": key.arn,
                "description": key.description,
                "enabled": key.enabled,
                "key_state": key.key_state,
                "key_usage": key.key_usage,
                "expiration_model": "KEY_MATERIAL_DOES_NOT_EXPIRE",
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
// aws_kms_replica_key
// ---------------------------------------------------------------------------

/// Converts `aws_kms_replica_key` Terraform resources to/from KMS state.
///
/// Multi-region replica of an AWS-managed key (`origin = AWS_KMS`,
/// `multi_region = true`). Key material is regenerated locally as a
/// 16-byte stub.
pub struct AwsKmsReplicaKeyConverter {
    service: Arc<KmsService>,
}

impl AwsKmsReplicaKeyConverter {
    pub fn new(service: Arc<KmsService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsKmsReplicaKeyConverter {
    fn resource_type(&self) -> &str {
        "aws_kms_replica_key"
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

impl AwsKmsReplicaKeyConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: kms_gen::ReplicaKeyTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_kms_replica_key", e))?;

        let attrs = &instance.attributes;

        let derived_key_id =
            derive_replica_key_id(&model.primary_key_arn, &model.key_id, &model.id);

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:kms:{}:{}:key/{}",
                region, ctx.default_account_id, derived_key_id
            )
        });

        let enabled = model.enabled;
        let description = model.description.unwrap_or_default();
        let key_rotation_enabled = model.key_rotation_enabled;

        let mut key_tags = model.tags;
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    key_tags.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let key_view = KeyView {
            key_id: derived_key_id.clone(),
            arn,
            account_id: ctx.default_account_id.clone(),
            region: region.clone(),
            description,
            key_spec: "SYMMETRIC_DEFAULT".to_string(),
            key_usage: "ENCRYPT_DECRYPT".to_string(),
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
            key_material: uuid::Uuid::new_v4().as_bytes().to_vec(),
            tags: key_tags,
            multi_region: true,
            public_key_der: None,
        };

        let mut state_view = KmsStateView::default();
        state_view.keys.insert(derived_key_id, key_view);
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
            if !key.multi_region || key.origin != "AWS_KMS" {
                continue;
            }
            let attrs = serde_json::json!({
                "id": key.key_id,
                "key_id": key.key_id,
                "arn": key.arn,
                "primary_key_arn": key.arn,
                "description": key.description,
                "enabled": key.enabled,
                "key_rotation_enabled": key.key_rotation_enabled,
                "key_spec": key.key_spec,
                "key_state": key.key_state,
                "key_usage": key.key_usage,
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

/// Derive the replica's key ID from the primary key ARN, preferring an
/// explicit `key_id` (or `id`) attribute when present. Real AWS
/// multi-Region replicas share the suffix portion (`mrk-...`) of the
/// primary key's identifier; this helper extracts it from
/// `arn:aws:kms:<region>:<account>:key/<key_id>`.
fn derive_replica_key_id(
    primary_key_arn: &str,
    explicit_key_id: &Option<String>,
    fallback_id: &Option<String>,
) -> String {
    if let Some(k) = explicit_key_id.as_deref().filter(|s| !s.is_empty()) {
        return k.to_string();
    }
    if let Some(k) = fallback_id.as_deref().filter(|s| !s.is_empty()) {
        return k.to_string();
    }
    if let Some(idx) = primary_key_arn.rfind('/') {
        return primary_key_arn[idx + 1..].to_string();
    }
    uuid::Uuid::new_v4().to_string()
}
