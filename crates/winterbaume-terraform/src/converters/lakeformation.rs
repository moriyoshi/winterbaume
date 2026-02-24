//! Terraform converter for Lake Formation resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_lakeformation::LakeFormationService;
use winterbaume_lakeformation::views::{
    DataLakeSettingsView, LakeFormationStateView, PrincipalPermissionsView, RegisteredResourceView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_bool, optional_str, require_str};

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
        let attrs = &instance.attributes;
        let arn = require_str(attrs, "arn", "aws_lakeformation_resource")?;
        let region = extract_region(attrs, &ctx.default_region);
        let role_arn = optional_str(attrs, "role_arn");
        let use_service_linked_role =
            optional_bool(attrs, "use_service_linked_role").unwrap_or(false);
        let _hybrid_access_enabled = optional_bool(attrs, "hybrid_access_enabled").unwrap_or(false);

        let resource_view = RegisteredResourceView {
            resource_arn: arn.to_string(),
            role_arn,
            use_service_linked_role,
        };

        let mut state_view = LakeFormationStateView::default();
        state_view.resources.insert(arn.to_string(), resource_view);
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
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let _external_data_filtering_allow_list = attrs.get("external_data_filtering_allow_list");
        let _trusted_resource_owners = attrs.get("trusted_resource_owners");

        let admins: Vec<String> = attrs
            .get("admins")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let allow_external_data_filtering =
            optional_bool(attrs, "allow_external_data_filtering").unwrap_or(false);

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
            allow_external_data_filtering,
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
