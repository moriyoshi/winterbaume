//! Terraform converters for Lake Formation resources.
//!
//! `RegisteredResourceTfModel` and `DataLakeSettingsTfModel` are
//! generated from `specs/lakeformation.toml`. The discarded
//! `hybrid_access_enabled` flag, the `admins` /
//! `authorized_session_tag_value_list` Vec<String>s, and the
//! `create_database_default_permissions` /
//! `create_table_default_permissions` nested-block arrays are wired
//! up here.
//!
//! `LfTagTfModel`, `PermissionsTfModel`, `ResourceLfTagTfModel`, and
//! `ResourceLfTagsTfModel` carry the spec-vocab string fields; the rest
//! of the HCL surface ( Vec<String> tag values, repeated nested
//! resource-selector blocks, repeated `lf_tag` blocks ) is read raw
//! from `instance.attributes` here.
//!
//! `DataCellsFilterTfModel` and `OptInTfModel` have no backing state
//! slot in `winterbaume_lakeformation`; the converters validate the
//! HCL shape and emit a no-op warning.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_lakeformation::LakeFormationService;
use winterbaume_lakeformation::views::{
    DataLakeSettingsView, LFTagPairView, LFTagView, LakeFormationStateView, PermissionGrantView,
    PrincipalPermissionsView, RegisteredResourceView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::lakeformation as lakeformation_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_lakeformation_resource
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_resource` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationResourceConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationResourceConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationResourceConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_resource"
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

impl AwsLakeformationResourceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::RegisteredResourceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_resource", e))?;

        let resource_view = RegisteredResourceView {
            resource_arn: model.resource_arn.clone(),
            role_arn: model.role_arn,
            use_service_linked_role: model.use_service_linked_role,
        };

        let mut state_view = LakeFormationStateView::default();
        state_view
            .resources
            .insert(model.resource_arn, resource_view);
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
        for resource in view.resources.values() {
            let attrs = serde_json::json!({
                "id": resource.resource_arn,
                "arn": resource.resource_arn,
                "role_arn": resource.role_arn,
                "use_service_linked_role": resource.use_service_linked_role,
            });
            results.push(ExtractedResource {
                name: resource.resource_arn.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lakeformation_data_lake_settings
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_data_lake_settings` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationDataLakeSettingsConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationDataLakeSettingsConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationDataLakeSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_data_lake_settings"
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

impl AwsLakeformationDataLakeSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::DataLakeSettingsTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_lakeformation_data_lake_settings", e)
            })?;

        let attrs = &instance.attributes;

        let admins: Vec<String> = attrs
            .get("admins")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let authorized_session_tag_value_list: Vec<String> = attrs
            .get("authorized_session_tag_value_list")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let create_database_default_permissions =
            parse_principal_permissions(attrs.get("create_database_default_permissions"));
        let create_table_default_permissions =
            parse_principal_permissions(attrs.get("create_table_default_permissions"));

        let settings_view = DataLakeSettingsView {
            data_lake_admins: admins,
            allow_external_data_filtering: model.allow_external_data_filtering,
            authorized_session_tag_value_list,
            create_database_default_permissions,
            create_table_default_permissions,
        };

        let state_view = LakeFormationStateView {
            data_lake_settings: settings_view,
            ..Default::default()
        };
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
        let settings = &view.data_lake_settings;

        let create_db_perms: Vec<serde_json::Value> = settings
            .create_database_default_permissions
            .iter()
            .map(|pp| {
                serde_json::json!({
                    "principal": pp.principal,
                    "permissions": pp.permissions,
                })
            })
            .collect();
        let create_table_perms: Vec<serde_json::Value> = settings
            .create_table_default_permissions
            .iter()
            .map(|pp| {
                serde_json::json!({
                    "principal": pp.principal,
                    "permissions": pp.permissions,
                })
            })
            .collect();

        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "admins": settings.data_lake_admins,
            "allow_external_data_filtering": settings.allow_external_data_filtering,
            "authorized_session_tag_value_list": settings.authorized_session_tag_value_list,
            "create_database_default_permissions": create_db_perms,
            "create_table_default_permissions": create_table_perms,
            "tags_all": {},
            "trusted_resource_owners": [],
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

fn parse_principal_permissions(val: Option<&serde_json::Value>) -> Vec<PrincipalPermissionsView> {
    val.and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|item| {
                    let principal = item.get("principal")?.as_str()?.to_string();
                    let permissions = item
                        .get("permissions")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    Some(PrincipalPermissionsView {
                        principal,
                        permissions,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// aws_lakeformation_lf_tag
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_lf_tag` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationLfTagConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationLfTagConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationLfTagConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_lf_tag"
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

impl AwsLakeformationLfTagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::LfTagTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_lf_tag", e))?;

        let tag_values: Vec<String> = instance
            .attributes
            .get("values")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let lf_tag_view = LFTagView {
            tag_key: model.tag_key.clone(),
            tag_values,
            catalog_id: model.catalog_id,
        };

        let mut state_view = LakeFormationStateView::default();
        state_view.lf_tags.insert(model.tag_key, lf_tag_view);
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
        for tag in view.lf_tags.values() {
            let catalog_id = tag
                .catalog_id
                .clone()
                .unwrap_or_else(|| ctx.default_account_id.clone());
            let id = format!("{}:{}", catalog_id, tag.tag_key);
            let attrs = serde_json::json!({
                "id": id,
                "key": tag.tag_key,
                "values": tag.tag_values,
                "catalog_id": catalog_id,
            });
            results.push(ExtractedResource {
                name: tag.tag_key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lakeformation_permissions
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_permissions` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationPermissionsConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationPermissionsConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationPermissionsConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_permissions"
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

impl AwsLakeformationPermissionsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::PermissionsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_permissions", e))?;

        let attrs = &instance.attributes;

        let permissions: Vec<String> = string_list(attrs.get("permissions"));
        let permissions_with_grant_option: Vec<String> =
            string_list(attrs.get("permissions_with_grant_option"));

        let resource = build_permissions_resource(attrs);

        let grant_view = PermissionGrantView {
            principal: model.principal,
            resource,
            permissions,
            permissions_with_grant_option,
        };

        let state_view = LakeFormationStateView {
            permissions: vec![grant_view],
            ..Default::default()
        };
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
        for (idx, grant) in view.permissions.iter().enumerate() {
            let attrs = serde_json::json!({
                "id": format!("{}-{}", grant.principal, idx),
                "principal": grant.principal,
                "permissions": grant.permissions,
                "permissions_with_grant_option": grant.permissions_with_grant_option,
                "catalog_resource": grant.resource.get("catalog").is_some(),
            });
            results.push(ExtractedResource {
                name: format!("{}-{}", grant.principal, idx),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

fn string_list(val: Option<&serde_json::Value>) -> Vec<String> {
    val.and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

/// Build a JSON value capturing the resource selector for an
/// `aws_lakeformation_permissions` block. The HCL surface allows one of
/// `catalog_resource` ( bool ), `data_location`, `database`, `lf_tag`,
/// `lf_tag_policy`, `table`, or `table_with_columns` nested blocks. We
/// stash whichever appears so the grant entry can be inspected later.
fn build_permissions_resource(attrs: &serde_json::Value) -> serde_json::Value {
    if attrs
        .get("catalog_resource")
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
    {
        return serde_json::json!({ "catalog": {} });
    }
    for key in [
        "data_location",
        "database",
        "lf_tag",
        "lf_tag_policy",
        "table",
        "table_with_columns",
    ] {
        if let Some(value) = first_block(attrs.get(key)) {
            return serde_json::json!({ key: value });
        }
    }
    serde_json::Value::Null
}

/// Extract the first element of an HCL "list-of-1" nested block. Terraform
/// state encodes single nested blocks as a one-element array of objects.
fn first_block(val: Option<&serde_json::Value>) -> Option<serde_json::Value> {
    match val {
        Some(serde_json::Value::Array(arr)) => arr.first().cloned(),
        Some(serde_json::Value::Object(_)) => val.cloned(),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// aws_lakeformation_resource_lf_tag
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_resource_lf_tag` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationResourceLfTagConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationResourceLfTagConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationResourceLfTagConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_resource_lf_tag"
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

impl AwsLakeformationResourceLfTagConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::ResourceLfTagTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_resource_lf_tag", e))?;

        let attrs = &instance.attributes;
        let resource_key = build_tag_assignment_key(attrs, &model.catalog_id);

        let tag_key = attrs
            .get("tag_key")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();
        let tag_value = attrs
            .get("tag_value")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        let pair = LFTagPairView {
            catalog_id: model.catalog_id.clone(),
            tag_key,
            tag_values: vec![tag_value],
        };

        let mut state_view = LakeFormationStateView::default();
        state_view.tag_assignments.insert(resource_key, vec![pair]);
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
        for (resource_key, pairs) in &view.tag_assignments {
            for (idx, pair) in pairs.iter().enumerate() {
                if pair.tag_values.len() != 1 {
                    // Multi-value assignments belong to
                    // `aws_lakeformation_resource_lf_tags`.
                    continue;
                }
                let name = format!("{}#{}#{}", resource_key, pair.tag_key, idx);
                let attrs = serde_json::json!({
                    "id": name,
                    "catalog_id": pair.catalog_id,
                    "tag_key": pair.tag_key,
                    "tag_value": pair.tag_values.first().cloned().unwrap_or_default(),
                });
                results.push(ExtractedResource {
                    name,
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lakeformation_resource_lf_tags
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_resource_lf_tags` Terraform resources to/from Lake Formation state.
pub struct AwsLakeformationResourceLfTagsConverter {
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationResourceLfTagsConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationResourceLfTagsConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_resource_lf_tags"
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

impl AwsLakeformationResourceLfTagsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: lakeformation_gen::ResourceLfTagsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_resource_lf_tags", e))?;

        let attrs = &instance.attributes;
        let resource_key = build_tag_assignment_key(attrs, &model.catalog_id);

        let pairs: Vec<LFTagPairView> = attrs
            .get("lf_tag")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| {
                        let tag_key = item.get("key")?.as_str()?.to_string();
                        let value = item.get("value")?.as_str()?.to_string();
                        let catalog_id = item
                            .get("catalog_id")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .or_else(|| model.catalog_id.clone());
                        Some(LFTagPairView {
                            catalog_id,
                            tag_key,
                            tag_values: vec![value],
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = LakeFormationStateView::default();
        state_view.tag_assignments.insert(resource_key, pairs);
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
        for (resource_key, pairs) in &view.tag_assignments {
            if pairs.iter().all(|p| p.tag_values.len() == 1) && pairs.len() <= 1 {
                // A lone single-value assignment is owned by
                // `aws_lakeformation_resource_lf_tag`.
                continue;
            }
            let lf_tags: Vec<serde_json::Value> = pairs
                .iter()
                .map(|pair| {
                    serde_json::json!({
                        "catalog_id": pair.catalog_id,
                        "key": pair.tag_key,
                        "value": pair.tag_values.first().cloned().unwrap_or_default(),
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": resource_key,
                "catalog_id": pairs.first().and_then(|p| p.catalog_id.clone()),
                "lf_tag": lf_tags,
            });
            results.push(ExtractedResource {
                name: resource_key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

/// Build a stable key string for `tag_assignments` from the HCL
/// `database` / `table` / `table_with_columns` nested-block selectors.
fn build_tag_assignment_key(
    attrs: &serde_json::Value,
    fallback_catalog_id: &Option<String>,
) -> String {
    if let Some(db) = first_block(attrs.get("database")) {
        let catalog = db
            .get("catalog_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| fallback_catalog_id.clone())
            .unwrap_or_default();
        let name = db.get("name").and_then(|v| v.as_str()).unwrap_or_default();
        return format!("database:{}:{}", catalog, name);
    }
    if let Some(tbl) = first_block(attrs.get("table")) {
        let catalog = tbl
            .get("catalog_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| fallback_catalog_id.clone())
            .unwrap_or_default();
        let database_name = tbl
            .get("database_name")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let name = tbl.get("name").and_then(|v| v.as_str()).unwrap_or_default();
        return format!("table:{}:{}:{}", catalog, database_name, name);
    }
    if let Some(twc) = first_block(attrs.get("table_with_columns")) {
        let catalog = twc
            .get("catalog_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or_else(|| fallback_catalog_id.clone())
            .unwrap_or_default();
        let database_name = twc
            .get("database_name")
            .and_then(|v| v.as_str())
            .unwrap_or_default();
        let name = twc.get("name").and_then(|v| v.as_str()).unwrap_or_default();
        return format!("table_with_columns:{}:{}:{}", catalog, database_name, name);
    }
    let catalog = fallback_catalog_id.clone().unwrap_or_default();
    format!("unknown:{}", catalog)
}

// ---------------------------------------------------------------------------
// aws_lakeformation_data_cells_filter — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_data_cells_filter` resources (validation-only;
/// no backing state slot in `winterbaume_lakeformation`).
pub struct AwsLakeformationDataCellsFilterConverter {
    #[allow(dead_code)]
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationDataCellsFilterConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationDataCellsFilterConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_data_cells_filter"
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

impl AwsLakeformationDataCellsFilterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: lakeformation_gen::DataCellsFilterTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_lakeformation_data_cells_filter", e)
            })?;
        let warn_msg =
            "no state slot in winterbaume_lakeformation for data cells filters; inject is a no-op"
                .to_string();
        eprintln!("warning: aws_lakeformation_data_cells_filter: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_lakeformation_data_cells_filter: {warn_msg}")],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_lakeformation_opt_in — no state slot
// ---------------------------------------------------------------------------

/// Converts `aws_lakeformation_opt_in` resources (validation-only;
/// no backing state slot in `winterbaume_lakeformation`).
pub struct AwsLakeformationOptInConverter {
    #[allow(dead_code)]
    service: Arc<LakeFormationService>,
}

impl AwsLakeformationOptInConverter {
    pub fn new(service: Arc<LakeFormationService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLakeformationOptInConverter {
    fn resource_type(&self) -> &str {
        "aws_lakeformation_opt_in"
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

impl AwsLakeformationOptInConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: lakeformation_gen::OptInTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_lakeformation_opt_in", e))?;
        let warn_msg =
            "no state slot in winterbaume_lakeformation for opt-ins; inject is a no-op".to_string();
        eprintln!("warning: aws_lakeformation_opt_in: {warn_msg}");
        Ok(ConversionResult {
            region,
            warnings: vec![format!("aws_lakeformation_opt_in: {warn_msg}")],
        })
    }
}
