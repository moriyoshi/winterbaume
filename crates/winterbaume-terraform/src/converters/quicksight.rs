//! Terraform converters for QuickSight resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_quicksight::QuickSightService;
use winterbaume_quicksight::views::{
    DashboardView, DataSetView, FolderMemberEntryView, IngestionView, QuickSightAnalysisView,
    QuickSightDataSourceView, QuickSightFolderView, QuickSightGroupView, QuickSightNamespaceView,
    QuickSightStateView, QuickSightTemplateView, QuickSightThemeView, QuickSightUserView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::quicksight as quicksight_gen;
use crate::util::{classify_deserialize_error, extract_region};

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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightDataSourceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_data_source", e))?;

        let attrs = &instance.attributes;
        let data_source_id = model.data_source_id.clone();
        let name = model.name.unwrap_or_else(|| data_source_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:datasource/{}",
                region, ctx.default_account_id, data_source_id
            )
        });
        let ds_type = model.ds_type.unwrap_or_else(|| "MANUAL".to_string());
        let status = model
            .status
            .unwrap_or_else(|| "CREATION_SUCCESSFUL".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

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
            data_source_id: data_source_id.clone(),
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
        state_view.data_sources.insert(data_source_id, ds_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_group", e))?;

        let group_name = model.group_name.clone();
        let namespace = model.namespace.unwrap_or_else(|| "default".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:group/{}/{}",
                region, ctx.default_account_id, namespace, group_name
            )
        });
        let description = model.description.unwrap_or_default();
        let principal_id = model.principal_id.unwrap_or_default();

        let group_view = QuickSightGroupView {
            group_name: group_name.clone(),
            arn,
            description,
            principal_id,
        };

        let mut state_view = QuickSightStateView::default();
        let ns_groups = state_view.groups.entry(namespace).or_default();
        ns_groups.insert(group_name, group_view);
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
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightUserTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_user", e))?;

        let user_name = model.user_name.clone();
        let namespace = model.namespace.unwrap_or_else(|| "default".to_string());
        let email = model.email.unwrap_or_default();
        let role = model.user_role.unwrap_or_else(|| "READER".to_string());
        let identity_type = model.identity_type.unwrap_or_else(|| "IAM".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:user/{}/{}",
                region, ctx.default_account_id, namespace, user_name
            )
        });
        let principal_id = model.principal_id.unwrap_or_default();

        let user_view = QuickSightUserView {
            user_name: user_name.clone(),
            arn,
            email,
            role,
            identity_type,
            active: true,
            principal_id,
        };

        let mut state_view = QuickSightStateView::default();
        let ns_users = state_view.users.entry(namespace).or_default();
        ns_users.insert(user_name, user_view);
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

// ---------------------------------------------------------------------------
// aws_quicksight_account_settings
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_account_settings` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightAccountSettingsConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightAccountSettingsConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightAccountSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_account_settings"
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

impl AwsQuicksightAccountSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightAccountSettingsTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_account_settings", e))?;

        // Snapshot + mutate: update account_settings without clobbering siblings.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(ns) = model.default_namespace {
            state_view.account_settings.default_namespace = ns;
        }
        if let Some(email) = model.notification_email {
            state_view.account_settings.notification_email = email;
        }
        state_view.account_settings.termination_protection_enabled =
            model.termination_protection_enabled;

        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        let s = &view.account_settings;
        // Only emit when the singleton looks meaningfully populated.
        if s.default_namespace.is_empty()
            && s.notification_email.is_empty()
            && s.account_name.is_empty()
        {
            return Ok(vec![]);
        }
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "aws_account_id": ctx.default_account_id,
            "default_namespace": s.default_namespace,
            "notification_email": s.notification_email,
            "termination_protection_enabled": s.termination_protection_enabled,
        });
        Ok(vec![ExtractedResource {
            name: ctx.default_account_id.clone(),
            account_id: ctx.default_account_id.clone(),
            region: ctx.default_region.clone(),
            attributes: attrs,
        }])
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_account_subscription
// ---------------------------------------------------------------------------
//
// Terraform-only resource — QuickSight account subscription is not modelled
// in winterbaume_quicksight state. Inject parses the schema and emits a
// warning; extract returns an empty list.

/// Converts `aws_quicksight_account_subscription` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightAccountSubscriptionConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightAccountSubscriptionConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightAccountSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_account_subscription"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightAccountSubscriptionTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_quicksight_account_subscription", e)
                })?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_account_subscription: no winterbaume state slot for account \
                     subscription; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_analysis
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_analysis` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightAnalysisConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightAnalysisConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightAnalysisConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_analysis"
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

impl AwsQuicksightAnalysisConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightAnalysisTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_analysis", e))?;

        let analysis_id = model.analysis_id.clone();
        let name = model.name.unwrap_or_else(|| analysis_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:analysis/{}",
                region, ctx.default_account_id, analysis_id
            )
        });
        let status = model
            .status
            .unwrap_or_else(|| "CREATION_SUCCESSFUL".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        let view = QuickSightAnalysisView {
            analysis_id: analysis_id.clone(),
            name,
            arn,
            status,
            created_time,
            last_updated_time,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.analyses.insert(analysis_id, view);
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
        for a in view.analyses.values() {
            let attrs = serde_json::json!({
                "id": a.analysis_id,
                "analysis_id": a.analysis_id,
                "aws_account_id": ctx.default_account_id,
                "name": a.name,
                "arn": a.arn,
                "status": a.status,
                "created_time": a.created_time,
                "last_updated_time": a.last_updated_time,
            });
            results.push(ExtractedResource {
                name: a.analysis_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_dashboard
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_dashboard` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightDashboardConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightDashboardConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightDashboardConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_dashboard"
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

impl AwsQuicksightDashboardConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightDashboardTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_dashboard", e))?;

        let dashboard_id = model.dashboard_id.clone();
        let name = model.name.unwrap_or_else(|| dashboard_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:dashboard/{}",
                region, ctx.default_account_id, dashboard_id
            )
        });
        let version_number = if model.version_number > 0 {
            model.version_number
        } else {
            1
        };
        let version_arn = model
            .version_arn
            .unwrap_or_else(|| format!("{}/version/{}", arn, version_number));
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        let view = DashboardView {
            dashboard_id: dashboard_id.clone(),
            name,
            arn,
            version_arn,
            created_time,
            last_updated_time,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.dashboards.insert(dashboard_id, view);
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
        for d in view.dashboards.values() {
            let attrs = serde_json::json!({
                "id": d.dashboard_id,
                "dashboard_id": d.dashboard_id,
                "aws_account_id": ctx.default_account_id,
                "name": d.name,
                "arn": d.arn,
                "version_arn": d.version_arn,
                "created_time": d.created_time,
                "last_updated_time": d.last_updated_time,
            });
            results.push(ExtractedResource {
                name: d.dashboard_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_data_set
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_data_set` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightDataSetConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightDataSetConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightDataSetConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_data_set"
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

impl AwsQuicksightDataSetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightDataSetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_data_set", e))?;

        let data_set_id = model.data_set_id.clone();
        let name = model.name.unwrap_or_else(|| data_set_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:dataset/{}",
                region, ctx.default_account_id, data_set_id
            )
        });
        let import_mode = model.import_mode.unwrap_or_else(|| "SPICE".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        // Read complex physical_table_map straight from attrs (skip if absent).
        let physical_table_map = instance
            .attributes
            .get("physical_table_map")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect::<HashMap<String, serde_json::Value>>()
            })
            .unwrap_or_default();

        let view = DataSetView {
            data_set_id: data_set_id.clone(),
            name,
            arn,
            import_mode,
            physical_table_map,
            created_time,
            last_updated_time,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.data_sets.insert(data_set_id, view);
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
        for ds in view.data_sets.values() {
            let attrs = serde_json::json!({
                "id": ds.data_set_id,
                "data_set_id": ds.data_set_id,
                "aws_account_id": ctx.default_account_id,
                "name": ds.name,
                "arn": ds.arn,
                "import_mode": ds.import_mode,
                "created_time": ds.created_time,
                "last_updated_time": ds.last_updated_time,
                "physical_table_map": ds.physical_table_map,
            });
            results.push(ExtractedResource {
                name: ds.data_set_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_folder
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_folder` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightFolderConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightFolderConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightFolderConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_folder"
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

impl AwsQuicksightFolderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightFolderTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_folder", e))?;

        let folder_id = model.folder_id.clone();
        let name = model.name.unwrap_or_else(|| folder_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:folder/{}",
                region, ctx.default_account_id, folder_id
            )
        });
        let folder_type = model.folder_type.unwrap_or_else(|| "SHARED".to_string());
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        let view = QuickSightFolderView {
            folder_id: folder_id.clone(),
            name,
            arn,
            folder_type,
            created_time,
            last_updated_time,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.folders.insert(folder_id, view);
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
        for f in view.folders.values() {
            let attrs = serde_json::json!({
                "id": f.folder_id,
                "folder_id": f.folder_id,
                "aws_account_id": ctx.default_account_id,
                "name": f.name,
                "arn": f.arn,
                "folder_type": f.folder_type,
                "created_time": f.created_time,
                "last_updated_time": f.last_updated_time,
            });
            results.push(ExtractedResource {
                name: f.folder_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_folder_membership
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_folder_membership` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightFolderMembershipConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightFolderMembershipConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightFolderMembershipConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_folder_membership"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_quicksight_folder"]
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

impl AwsQuicksightFolderMembershipConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightFolderMembershipTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_folder_membership", e))?;

        // Snapshot + mutate to preserve existing folders/members.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let entry = FolderMemberEntryView {
            member_id: model.member_id.clone(),
            member_type: model.member_type,
        };
        let members = state_view
            .folder_members
            .entry(model.folder_id)
            .or_default();
        if !members.iter().any(|m| m.member_id == entry.member_id) {
            members.push(entry);
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for (folder_id, members) in &view.folder_members {
            for m in members {
                let id = format!("{},{},{}", folder_id, m.member_id, m.member_type);
                let attrs = serde_json::json!({
                    "id": id,
                    "folder_id": folder_id,
                    "member_id": m.member_id,
                    "member_type": m.member_type,
                    "aws_account_id": ctx.default_account_id,
                });
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
}

// ---------------------------------------------------------------------------
// aws_quicksight_group_membership
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_group_membership` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightGroupMembershipConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightGroupMembershipConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightGroupMembershipConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_group_membership"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_quicksight_group", "aws_quicksight_user"]
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

impl AwsQuicksightGroupMembershipConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightGroupMembershipTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_group_membership", e))?;

        let namespace = model.namespace.unwrap_or_else(|| "default".to_string());

        // Snapshot + mutate to preserve sibling groups/users/memberships.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let ns_memberships = state_view
            .group_memberships
            .entry(namespace.clone())
            .or_default();
        let members = ns_memberships.entry(model.group_name.clone()).or_default();
        if !members.contains(&model.member_name) {
            members.push(model.member_name.clone());
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
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
        for (namespace, ns_memberships) in &view.group_memberships {
            for (group_name, members) in ns_memberships {
                for member_name in members {
                    let id = format!("{}/{}/{}", namespace, group_name, member_name);
                    let arn = format!(
                        "arn:aws:quicksight:{}:{}:group/{}/{}/{}",
                        ctx.default_region,
                        ctx.default_account_id,
                        namespace,
                        group_name,
                        member_name
                    );
                    let attrs = serde_json::json!({
                        "id": id,
                        "group_name": group_name,
                        "member_name": member_name,
                        "namespace": namespace,
                        "aws_account_id": ctx.default_account_id,
                        "arn": arn,
                    });
                    results.push(ExtractedResource {
                        name: id.clone(),
                        account_id: ctx.default_account_id.clone(),
                        region: ctx.default_region.clone(),
                        attributes: attrs,
                    });
                }
            }
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_iam_policy_assignment
// ---------------------------------------------------------------------------
//
// Terraform-only resource — winterbaume_quicksight does not model IAM policy
// assignments. Warning-only inject.

/// Converts `aws_quicksight_iam_policy_assignment` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightIamPolicyAssignmentConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightIamPolicyAssignmentConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightIamPolicyAssignmentConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_iam_policy_assignment"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightIamPolicyAssignmentTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_quicksight_iam_policy_assignment", e)
                })?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_iam_policy_assignment: no winterbaume state slot for IAM \
                     policy assignments; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_ingestion
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_ingestion` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightIngestionConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightIngestionConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightIngestionConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_ingestion"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_quicksight_data_set"]
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

impl AwsQuicksightIngestionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightIngestionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_ingestion", e))?;

        let data_set_id = model.data_set_id.clone();
        let ingestion_id = model.ingestion_id.clone();
        let key = format!("{}/{}", data_set_id, ingestion_id);
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:dataset/{}/ingestion/{}",
                region, ctx.default_account_id, data_set_id, ingestion_id
            )
        });
        let ingestion_status = model
            .ingestion_status
            .unwrap_or_else(|| "INITIALIZED".to_string());

        let view = IngestionView {
            ingestion_id: ingestion_id.clone(),
            arn,
            ingestion_status,
            data_set_id: data_set_id.clone(),
        };

        let mut state_view = QuickSightStateView::default();
        state_view.ingestions.insert(key, view);
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
        for i in view.ingestions.values() {
            let id = format!("{}/{}", i.data_set_id, i.ingestion_id);
            let attrs = serde_json::json!({
                "id": id,
                "data_set_id": i.data_set_id,
                "ingestion_id": i.ingestion_id,
                "aws_account_id": ctx.default_account_id,
                "arn": i.arn,
                "ingestion_status": i.ingestion_status,
            });
            results.push(ExtractedResource {
                name: id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_namespace
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_namespace` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightNamespaceConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightNamespaceConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightNamespaceConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_namespace"
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

impl AwsQuicksightNamespaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightNamespaceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_namespace", e))?;

        let namespace = model.namespace.clone();
        let identity_store = model
            .identity_store
            .unwrap_or_else(|| "QUICKSIGHT".to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:namespace/{}",
                region, ctx.default_account_id, namespace
            )
        });
        let capacity_region = model.capacity_region.unwrap_or_else(|| region.clone());
        let creation_status = model
            .creation_status
            .unwrap_or_else(|| "CREATED".to_string());

        let view = QuickSightNamespaceView {
            name: namespace.clone(),
            arn,
            capacity_region,
            creation_status,
            identity_store,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.namespaces.insert(namespace, view);
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
        for ns in view.namespaces.values() {
            let attrs = serde_json::json!({
                "id": ns.name,
                "namespace": ns.name,
                "aws_account_id": ctx.default_account_id,
                "identity_store": ns.identity_store,
                "arn": ns.arn,
                "capacity_region": ns.capacity_region,
                "creation_status": ns.creation_status,
            });
            results.push(ExtractedResource {
                name: ns.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_refresh_schedule
// ---------------------------------------------------------------------------
//
// Terraform-only resource — winterbaume_quicksight does not model refresh
// schedules. Warning-only inject.

/// Converts `aws_quicksight_refresh_schedule` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightRefreshScheduleConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightRefreshScheduleConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightRefreshScheduleConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_refresh_schedule"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightRefreshScheduleTfModel =
                serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                    classify_deserialize_error("aws_quicksight_refresh_schedule", e)
                })?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_refresh_schedule: no winterbaume state slot for refresh \
                     schedules; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_role_membership
// ---------------------------------------------------------------------------
//
// Terraform-only resource — winterbaume_quicksight does not model role
// membership. Warning-only inject.

/// Converts `aws_quicksight_role_membership` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightRoleMembershipConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightRoleMembershipConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightRoleMembershipConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_role_membership"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightRoleMembershipTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_quicksight_role_membership", e))?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_role_membership: no winterbaume state slot for role \
                     memberships; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_template
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_template` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightTemplateConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightTemplateConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightTemplateConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_template"
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

impl AwsQuicksightTemplateConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightTemplateTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_template", e))?;

        let template_id = model.template_id.clone();
        let name = model.name.unwrap_or_else(|| template_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:template/{}",
                region, ctx.default_account_id, template_id
            )
        });
        let version_number = if model.version_number > 0 {
            model.version_number
        } else {
            1
        };
        let version_arn = model
            .version_arn
            .unwrap_or_else(|| format!("{}/version/{}", arn, version_number));
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        let view = QuickSightTemplateView {
            template_id: template_id.clone(),
            name,
            arn,
            version_arn,
            created_time,
            last_updated_time,
            version_number,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.templates.insert(template_id, view);
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
        for t in view.templates.values() {
            let attrs = serde_json::json!({
                "id": t.template_id,
                "template_id": t.template_id,
                "aws_account_id": ctx.default_account_id,
                "name": t.name,
                "arn": t.arn,
                "version_number": t.version_number,
                "version_arn": t.version_arn,
                "created_time": t.created_time,
                "last_updated_time": t.last_updated_time,
            });
            results.push(ExtractedResource {
                name: t.template_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_template_alias
// ---------------------------------------------------------------------------
//
// Terraform-only resource — winterbaume_quicksight does not model template
// aliases. Warning-only inject.

/// Converts `aws_quicksight_template_alias` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightTemplateAliasConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightTemplateAliasConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightTemplateAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_template_alias"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_quicksight_template"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightTemplateAliasTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_quicksight_template_alias", e))?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_template_alias: no winterbaume state slot for template \
                     aliases; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_theme
// ---------------------------------------------------------------------------

/// Converts `aws_quicksight_theme` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightThemeConverter {
    service: Arc<QuickSightService>,
}

impl AwsQuicksightThemeConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightThemeConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_theme"
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

impl AwsQuicksightThemeConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: quicksight_gen::QuickSightThemeTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_quicksight_theme", e))?;

        let theme_id = model.theme_id.clone();
        let name = model.name.unwrap_or_else(|| theme_id.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:quicksight:{}:{}:theme/{}",
                region, ctx.default_account_id, theme_id
            )
        });
        let version_number = if model.version_number > 0 {
            model.version_number
        } else {
            1
        };
        let version_arn = model
            .version_arn
            .unwrap_or_else(|| format!("{}/version/{}", arn, version_number));
        let created_time = model
            .created_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_updated_time = model
            .last_updated_time
            .unwrap_or_else(|| created_time.clone());

        let view = QuickSightThemeView {
            theme_id: theme_id.clone(),
            name,
            arn,
            version_arn,
            created_time,
            last_updated_time,
            version_number,
        };

        let mut state_view = QuickSightStateView::default();
        state_view.themes.insert(theme_id, view);
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
        for t in view.themes.values() {
            let attrs = serde_json::json!({
                "id": t.theme_id,
                "theme_id": t.theme_id,
                "aws_account_id": ctx.default_account_id,
                "name": t.name,
                "arn": t.arn,
                "version_number": t.version_number,
                "version_arn": t.version_arn,
                "created_time": t.created_time,
                "last_updated_time": t.last_updated_time,
            });
            results.push(ExtractedResource {
                name: t.theme_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_quicksight_vpc_connection
// ---------------------------------------------------------------------------
//
// Terraform-only resource — winterbaume_quicksight does not model VPC
// connections as first-class state. Warning-only inject.

/// Converts `aws_quicksight_vpc_connection` Terraform resources to/from QuickSight state.
pub struct AwsQuicksightVpcConnectionConverter {
    #[allow(dead_code)]
    service: Arc<QuickSightService>,
}

impl AwsQuicksightVpcConnectionConverter {
    pub fn new(service: Arc<QuickSightService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsQuicksightVpcConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_quicksight_vpc_connection"
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let region = extract_region(&instance.attributes, &ctx.default_region);
            let _model: quicksight_gen::QuickSightVpcConnectionTfModel =
                serde_json::from_value(instance.attributes.clone())
                    .map_err(|e| classify_deserialize_error("aws_quicksight_vpc_connection", e))?;
            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_quicksight_vpc_connection: no winterbaume state slot for VPC \
                     connections; injection is a no-op."
                        .into(),
                ],
            })
        })
    }

    fn extract<'a>(
        &'a self,
        _ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>> + Send + 'a>>
    {
        Box::pin(async move { Ok(vec![]) })
    }
}
