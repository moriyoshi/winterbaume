//! Terraform converters for QuickSight resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_quicksight::QuickSightService;
use winterbaume_quicksight::views::{
    QuickSightDataSourceView, QuickSightGroupView, QuickSightStateView, QuickSightUserView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_quicksight_data_source
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_data_source` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightDataSourceConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightDataSourceConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightDataSourceConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_data_source"
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

impl AwsQuicksightDataSourceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let data_source_id = require_str(attrs, "data_source_id", "aws_quicksight_data_source")?;
        let region = extract_region(attrs, &ctx.default_region);

        let name = optional_str(attrs, "name").unwrap_or_else(|| data_source_id.to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:datasource/{}",
                region, ctx.default_account_id, data_source_id
            )
        });
        let ds_type = optional_str(attrs, "type").unwrap_or_else(|| "MANUAL".to_string());
        let status =
            optional_str(attrs, "status").unwrap_or_else(|| "CREATION_SUCCESSFUL".to_string());
        let created_time =
            optional_str(attrs, "created_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_updated_time =
            optional_str(attrs, "last_updated_time").unwrap_or_else(|| created_time.clone());

        let _tags_all = attrs.get("tags_all");
        let _ssl_properties = attrs.get("ssl_properties");
        let credentials = attrs
            .get("credentials")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let parameters = attrs
            .get("parameters")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let permission: Vec<serde_json::Value> = attrs
            .get("permission")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
        let vpc_connection_properties = attrs
            .get("vpc_connection_properties")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let ds_view = QuickSightDataSourceView {
            data_source_id: data_source_id.to_string(),
            name,
            arn,
            r#type: ds_type,
            status,
            created_time,
            last_updated_time,
            credentials,
            parameters,
            permission,
            vpc_connection_properties,
        };

        let mut state_view = QuickSightStateView {
            data_sources: HashMap::new(),
            ..Default::default()
        };
        state_view
            .data_sources
            .insert(data_source_id.to_string(), ds_view);
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
        for ds in view.data_sources.values() {
            let attrs = serde_json::json!({
                "id": ds.data_source_id,
                "data_source_id": ds.data_source_id,
                "name": ds.name,
                "arn": ds.arn,
                "type": ds.r#type,
                "status": ds.status,
                "created_time": ds.created_time,
                "last_updated_time": ds.last_updated_time,
                "tags_all": {},
                "ssl_properties": [],
                "credentials": ds.credentials,
                "parameters": ds.parameters,
                "permission": ds.permission,
                "vpc_connection_properties": ds.vpc_connection_properties,
            });
            results.push(ExtractedResource {
                name: ds.data_source_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_group
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_group` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightGroupConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightGroupConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_group"
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

impl AwsQuicksightGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let group_name = require_str(attrs, "group_name", "aws_quicksight_group")?;
        let region = extract_region(attrs, &ctx.default_region);

        let namespace = optional_str(attrs, "namespace").unwrap_or_else(|| "default".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:group/{}/{}",
                region, ctx.default_account_id, namespace, group_name
            )
        });
        let description = optional_str(attrs, "description").unwrap_or_default();
        let principal_id = optional_str(attrs, "principal_id").unwrap_or_default();

        let group_view = QuickSightGroupView {
            group_name: group_name.to_string(),
            arn,
            description,
            principal_id,
        };

        let mut state_view = QuickSightStateView::default();
        let ns_groups = state_view.groups.entry(namespace).or_default();
        ns_groups.insert(group_name.to_string(), group_view);
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
        for (namespace, ns_groups) in &view.groups {
            for g in ns_groups.values() {
                let attrs = serde_json::json!({
                    "id": g.group_name,
                    "group_name": g.group_name,
                    "namespace": namespace,
                    "arn": g.arn,
                    "description": g.description,
                    "principal_id": g.principal_id,
                });
                results.push(ExtractedResource {
                    name: g.group_name.clone(),
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
// aws_quicksight_user
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_user` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightUserConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightUserConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightUserConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_user"
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

impl AwsQuicksightUserConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let user_name = require_str(attrs, "user_name", "aws_quicksight_user")?;
        let region = extract_region(attrs, &ctx.default_region);

        let namespace = optional_str(attrs, "namespace").unwrap_or_else(|| "default".to_string());
        let email = optional_str(attrs, "email").unwrap_or_default();
        let role = optional_str(attrs, "user_role").unwrap_or_else(|| "READER".to_string());
        let identity_type =
            optional_str(attrs, "identity_type").unwrap_or_else(|| "IAM".to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:user/{}/{}",
                region, ctx.default_account_id, namespace, user_name
            )
        });
        let principal_id = optional_str(attrs, "principal_id").unwrap_or_default();

        let user_view = QuickSightUserView {
            user_name: user_name.to_string(),
            arn,
            email,
            role,
            identity_type,
            active: true,
            principal_id,
        };

        let mut state_view = QuickSightStateView::default();
        let ns_users = state_view.users.entry(namespace).or_default();
        ns_users.insert(user_name.to_string(), user_view);
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
        for (namespace, ns_users) in &view.users {
            for u in ns_users.values() {
                let attrs = serde_json::json!({
                    "id": u.user_name,
                    "user_name": u.user_name,
                    "namespace": namespace,
                    "arn": u.arn,
                    "email": u.email,
                    "user_role": u.role,
                    "identity_type": u.identity_type,
                    "active": u.active,
                    "principal_id": u.principal_id,
                });
                results.push(ExtractedResource {
                    name: u.user_name.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}
