//! Terraform converter for SSM resources.
//!
//! `ParameterTfModel` is generated from `specs/ssm.toml`. The ARN
//! template, the `type` / `value` / `data_type` defaults, the synthesised
//! initial parameter version, and the constant `tier = "Standard"`
//! extract value are wired up here.
//!
//! The remaining 11 converters cover the missing aws_ssm_* resource
//! types listed in TERRAFORM_RESOURCE_COVERAGE.md. They all use the
//! same model_only generator output and project into the matching
//! `SsmStateView` field; warnings are emitted for nested-block fields
//! the model omits (targets, parameters, approval rules).

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_ssm::SsmService;
use winterbaume_ssm::views::{
    ActivationView, AssociationView, DocumentView, MaintenanceWindowTargetView,
    MaintenanceWindowTaskView, MaintenanceWindowView, ParameterVersionView, ParameterView,
    PatchBaselineView, ResourceDataSyncView, ServiceSettingView, SsmStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::ssm as ssm_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_ssm_parameter
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_parameter` Terraform resources to/from SSM state.
pub struct AwsSsmParameterConverter {
    service: Arc<SsmService>,
}

impl AwsSsmParameterConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmParameterConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_parameter"
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

impl AwsSsmParameterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::ParameterTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ssm_parameter", e))?;

        let name = model.name.clone();
        let param_type = model.r#type.unwrap_or_else(|| "String".to_string());
        let value = model.value.unwrap_or_default();
        let version = model.version;
        let data_type = model.data_type.unwrap_or_else(|| "text".to_string());

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:ssm:{}:{}:parameter{}",
                region, ctx.default_account_id, name
            )
        });

        let param_view = ParameterView {
            name: name.clone(),
            r#type: param_type.clone(),
            value: value.clone(),
            version,
            last_modified_date: None,
            arn,
            data_type,
            tags: model.tags,
        };

        let param_version_view = ParameterVersionView {
            name: name.clone(),
            r#type: param_type,
            value,
            version,
            last_modified_date: None,
            labels: vec![],
        };

        let mut state_view = SsmStateView {
            parameters: HashMap::new(),
            parameter_versions: HashMap::new(),
            documents: HashMap::new(),
            maintenance_windows: HashMap::new(),
            patch_baselines: HashMap::new(),
            default_patch_baselines: HashMap::new(),
            commands: HashMap::new(),
            associations: HashMap::new(),
            resource_tags: HashMap::new(),
            ops_items: HashMap::new(),
            sessions: HashMap::new(),
            activations: HashMap::new(),
            resource_policies: HashMap::new(),
            service_settings: HashMap::new(),
            inventory: HashMap::new(),
            compliance_records: HashMap::new(),
            ops_metadata: HashMap::new(),
            resource_data_syncs: HashMap::new(),
            ..Default::default()
        };
        state_view.parameters.insert(name.clone(), param_view);
        state_view
            .parameter_versions
            .insert(name, vec![param_version_view]);
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
        for param in view.parameters.values() {
            let attrs = serde_json::json!({
                "id": param.name,
                "name": param.name,
                "type": param.r#type,
                "value": param.value,
                "arn": param.arn,
                "version": param.version,
                "last_modified_date": param.last_modified_date,
                "data_type": param.data_type,
                "tags": param.tags,
                "tags_all": param.tags,
                "tier": "Standard",
            });
            results.push(ExtractedResource {
                name: param.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_activation
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_activation` Terraform resources to/from SSM state.
pub struct AwsSsmActivationConverter {
    service: Arc<SsmService>,
}

impl AwsSsmActivationConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmActivationConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_activation"
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

impl AwsSsmActivationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::ActivationTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ssm_activation", e))?;

        // `registration_limit` is i32 in the view but the codegen only
        // supports u32/i64; read it straight from instance.attributes.
        let registration_limit = instance
            .attributes
            .get("registration_limit")
            .and_then(|v| v.as_i64())
            .map(|n| n as i32);

        let activation_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let activation_code = model
            .activation_code
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let activation_view = ActivationView {
            activation_id: activation_id.clone(),
            activation_code,
            iam_role: model.iam_role.unwrap_or_default(),
            description: model.description,
            default_instance_name: model.default_instance_name,
            registration_limit,
            registrations_count: 0,
            expiration_date: model.expiration_date,
            expired: false,
            created_date: Some(Utc::now().to_rfc3339()),
        };

        let mut state_view = SsmStateView::default();
        state_view
            .activations
            .insert(activation_id, activation_view);
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
        for activation in view.activations.values() {
            let attrs = serde_json::json!({
                "id": activation.activation_id,
                "name": activation.default_instance_name,
                "description": activation.description,
                "iam_role": activation.iam_role,
                "default_instance_name": activation.default_instance_name,
                "registration_limit": activation.registration_limit,
                "registration_count": activation.registrations_count,
                "expiration_date": activation.expiration_date,
                "expired": activation.expired,
                "activation_code": activation.activation_code,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: activation.activation_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_association
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_association` Terraform resources to/from SSM state.
pub struct AwsSsmAssociationConverter {
    service: Arc<SsmService>,
}

impl AwsSsmAssociationConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmAssociationConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_association"
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

impl AwsSsmAssociationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::AssociationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_association", e))?;

        let mut warnings = vec![];
        // `targets` and `parameters` are nested blocks not covered by
        // the model_only spec; report them so callers can plan a fix.
        if instance.attributes.get("targets").is_some() {
            warnings.push(
                "aws_ssm_association: nested `targets` blocks were ignored \
                 (no representation in the model_only projection)"
                    .to_string(),
            );
        }
        if instance.attributes.get("parameters").is_some() {
            warnings.push(
                "aws_ssm_association: nested `parameters` map was ignored \
                 (no representation in the model_only projection)"
                    .to_string(),
            );
        }

        let association_id = model
            .association_id
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let assoc_view = AssociationView {
            association_id: association_id.clone(),
            association_name: model.association_name,
            name: model.name,
            document_version: model.document_version,
            targets: vec![],
            schedule_expression: model.schedule_expression,
            parameters: HashMap::new(),
            status: "Success".to_string(),
            detailed_status: "Success".to_string(),
            last_execution_date: None,
            association_version: "1".to_string(),
            created_date: Some(Utc::now().to_rfc3339()),
        };

        let mut state_view = SsmStateView::default();
        state_view.associations.insert(association_id, assoc_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for assoc in view.associations.values() {
            let attrs = serde_json::json!({
                "id": assoc.association_id,
                "association_id": assoc.association_id,
                "association_name": assoc.association_name,
                "name": assoc.name,
                "document_version": assoc.document_version,
                "schedule_expression": assoc.schedule_expression,
                "status": assoc.status,
                "association_version": assoc.association_version,
                "targets": [],
                "parameters": {},
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: assoc.association_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_default_patch_baseline
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_default_patch_baseline` Terraform resources to/from SSM state.
pub struct AwsSsmDefaultPatchBaselineConverter {
    service: Arc<SsmService>,
}

impl AwsSsmDefaultPatchBaselineConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmDefaultPatchBaselineConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_default_patch_baseline"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ssm_patch_baseline"]
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

impl AwsSsmDefaultPatchBaselineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::DefaultPatchBaselineTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_default_patch_baseline", e))?;

        let mut state_view = SsmStateView::default();
        state_view
            .default_patch_baselines
            .insert(model.operating_system, model.baseline_id);
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
        for (os, baseline_id) in view.default_patch_baselines.iter() {
            let attrs = serde_json::json!({
                "id": baseline_id,
                "baseline_id": baseline_id,
                "operating_system": os,
            });
            results.push(ExtractedResource {
                name: format!("{}_{}", os, baseline_id),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_document
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_document` Terraform resources to/from SSM state.
pub struct AwsSsmDocumentConverter {
    service: Arc<SsmService>,
}

impl AwsSsmDocumentConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmDocumentConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_document"
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

impl AwsSsmDocumentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::DocumentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_document", e))?;

        let name = model.name.clone();
        let document_view = DocumentView {
            name: name.clone(),
            content: model.content.unwrap_or_default(),
            document_type: model.document_type.unwrap_or_else(|| "Command".to_string()),
            document_format: model.document_format.unwrap_or_else(|| "JSON".to_string()),
            status: model.status.unwrap_or_else(|| "Active".to_string()),
            owner: model
                .owner
                .unwrap_or_else(|| ctx.default_account_id.clone()),
            default_version: model.default_version.unwrap_or_else(|| "1".to_string()),
            latest_version: model.latest_version.unwrap_or_else(|| "1".to_string()),
            versions: vec![],
            account_permissions: vec![],
            created_date: Some(Utc::now().to_rfc3339()),
        };

        let mut state_view = SsmStateView::default();
        state_view.documents.insert(name, document_view);
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
        for doc in view.documents.values() {
            let arn = format!(
                "arn:aws:ssm:{}:{}:document/{}",
                ctx.default_region, ctx.default_account_id, doc.name
            );
            let attrs = serde_json::json!({
                "id": doc.name,
                "name": doc.name,
                "arn": arn,
                "content": doc.content,
                "document_type": doc.document_type,
                "document_format": doc.document_format,
                "default_version": doc.default_version,
                "latest_version": doc.latest_version,
                "status": doc.status,
                "owner": doc.owner,
                "created_date": doc.created_date,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: doc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_maintenance_window
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_maintenance_window` Terraform resources to/from SSM state.
pub struct AwsSsmMaintenanceWindowConverter {
    service: Arc<SsmService>,
}

impl AwsSsmMaintenanceWindowConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmMaintenanceWindowConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_maintenance_window"
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

impl AwsSsmMaintenanceWindowConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::MaintenanceWindowTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_maintenance_window", e))?;

        let window_id = instance
            .attributes
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("mw-{}", uuid::Uuid::new_v4().simple()));

        let window_view = MaintenanceWindowView {
            window_id: window_id.clone(),
            name: model.name,
            schedule: model.schedule,
            duration: model.duration,
            cutoff: model.cutoff,
            enabled: model.enabled,
            targets: vec![],
            tasks: vec![],
        };

        let mut state_view = SsmStateView::default();
        state_view
            .maintenance_windows
            .insert(window_id, window_view);
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
        for window in view.maintenance_windows.values() {
            let attrs = serde_json::json!({
                "id": window.window_id,
                "name": window.name,
                "schedule": window.schedule,
                "duration": window.duration,
                "cutoff": window.cutoff,
                "enabled": window.enabled,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: window.window_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_maintenance_window_target
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_maintenance_window_target` Terraform resources to/from SSM state.
pub struct AwsSsmMaintenanceWindowTargetConverter {
    service: Arc<SsmService>,
}

impl AwsSsmMaintenanceWindowTargetConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmMaintenanceWindowTargetConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_maintenance_window_target"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ssm_maintenance_window"]
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

impl AwsSsmMaintenanceWindowTargetConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::MaintenanceWindowTargetTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_maintenance_window_target", e))?;

        let mut warnings = vec![];
        if instance.attributes.get("targets").is_some() {
            warnings.push(
                "aws_ssm_maintenance_window_target: nested `targets` blocks \
                 (key/values) were ignored (no representation in the \
                 model_only projection)"
                    .to_string(),
            );
        }

        let window_target_id = instance
            .attributes
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let target_view = MaintenanceWindowTargetView {
            window_target_id: window_target_id.clone(),
            window_id: model.window_id.clone(),
            resource_type: model.resource_type,
            targets: vec![],
        };

        // Targets in the SSM state hang off MaintenanceWindowView, so
        // we have to read-modify-write the existing window. If the
        // window does not exist yet, leave a free-standing window
        // record so the test data is not silently dropped.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(window) = state_view.maintenance_windows.get_mut(&model.window_id) {
            window
                .targets
                .retain(|t| t.window_target_id != window_target_id);
            window.targets.push(target_view);
        } else {
            let placeholder = MaintenanceWindowView {
                window_id: model.window_id.clone(),
                name: model.name.unwrap_or_else(|| model.window_id.clone()),
                schedule: String::new(),
                duration: 0,
                cutoff: 0,
                enabled: true,
                targets: vec![target_view],
                tasks: vec![],
            };
            state_view
                .maintenance_windows
                .insert(model.window_id, placeholder);
            warnings.push(
                "aws_ssm_maintenance_window_target: parent maintenance \
                 window not found; created a placeholder window record"
                    .to_string(),
            );
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for window in view.maintenance_windows.values() {
            for target in &window.targets {
                let attrs = serde_json::json!({
                    "id": target.window_target_id,
                    "window_id": target.window_id,
                    "resource_type": target.resource_type,
                    "targets": [],
                });
                results.push(ExtractedResource {
                    name: target.window_target_id.clone(),
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
// aws_ssm_maintenance_window_task
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_maintenance_window_task` Terraform resources to/from SSM state.
pub struct AwsSsmMaintenanceWindowTaskConverter {
    service: Arc<SsmService>,
}

impl AwsSsmMaintenanceWindowTaskConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmMaintenanceWindowTaskConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_maintenance_window_task"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ssm_maintenance_window"]
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

impl AwsSsmMaintenanceWindowTaskConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::MaintenanceWindowTaskTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_maintenance_window_task", e))?;

        let mut warnings = vec![];
        if instance.attributes.get("targets").is_some() {
            warnings.push(
                "aws_ssm_maintenance_window_task: nested `targets` blocks \
                 were ignored (no representation in the model_only \
                 projection)"
                    .to_string(),
            );
        }
        if instance
            .attributes
            .get("task_invocation_parameters")
            .is_some()
        {
            warnings.push(
                "aws_ssm_maintenance_window_task: nested \
                 `task_invocation_parameters` blocks were ignored"
                    .to_string(),
            );
        }

        let window_task_id = instance
            .attributes
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let task_view = MaintenanceWindowTaskView {
            window_task_id: window_task_id.clone(),
            window_id: model.window_id.clone(),
            task_arn: model.task_arn,
            task_type: model.task_type,
            targets: vec![],
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if let Some(window) = state_view.maintenance_windows.get_mut(&model.window_id) {
            window.tasks.retain(|t| t.window_task_id != window_task_id);
            window.tasks.push(task_view);
        } else {
            let placeholder = MaintenanceWindowView {
                window_id: model.window_id.clone(),
                name: model.name.unwrap_or_else(|| model.window_id.clone()),
                schedule: String::new(),
                duration: 0,
                cutoff: 0,
                enabled: true,
                targets: vec![],
                tasks: vec![task_view],
            };
            state_view
                .maintenance_windows
                .insert(model.window_id, placeholder);
            warnings.push(
                "aws_ssm_maintenance_window_task: parent maintenance window \
                 not found; created a placeholder window record"
                    .to_string(),
            );
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for window in view.maintenance_windows.values() {
            for task in &window.tasks {
                let attrs = serde_json::json!({
                    "id": task.window_task_id,
                    "window_id": task.window_id,
                    "task_arn": task.task_arn,
                    "task_type": task.task_type,
                    "targets": [],
                });
                results.push(ExtractedResource {
                    name: task.window_task_id.clone(),
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
// aws_ssm_patch_baseline
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_patch_baseline` Terraform resources to/from SSM state.
pub struct AwsSsmPatchBaselineConverter {
    service: Arc<SsmService>,
}

impl AwsSsmPatchBaselineConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmPatchBaselineConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_patch_baseline"
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

impl AwsSsmPatchBaselineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::PatchBaselineTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_patch_baseline", e))?;

        let mut warnings = vec![];
        for nested in [
            "approval_rule",
            "global_filter",
            "approved_patches",
            "rejected_patches",
            "source",
        ] {
            if instance.attributes.get(nested).is_some() {
                warnings.push(format!(
                    "aws_ssm_patch_baseline: nested `{nested}` block/list was \
                     ignored (no representation in the model_only projection)"
                ));
            }
        }

        let baseline_id = instance
            .attributes
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| format!("pb-{}", uuid::Uuid::new_v4().simple()));

        let baseline_view = PatchBaselineView {
            baseline_id: baseline_id.clone(),
            name: model.name,
            operating_system: model
                .operating_system
                .unwrap_or_else(|| "WINDOWS".to_string()),
            description: model.description,
            patch_groups: vec![],
        };

        let mut state_view = SsmStateView::default();
        state_view
            .patch_baselines
            .insert(baseline_id, baseline_view);
        self.service
            .merge(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for baseline in view.patch_baselines.values() {
            let arn = format!(
                "arn:aws:ssm:{}:{}:patchbaseline/{}",
                ctx.default_region, ctx.default_account_id, baseline.baseline_id
            );
            let attrs = serde_json::json!({
                "id": baseline.baseline_id,
                "arn": arn,
                "name": baseline.name,
                "operating_system": baseline.operating_system,
                "description": baseline.description,
                "tags": {},
                "tags_all": {},
            });
            results.push(ExtractedResource {
                name: baseline.baseline_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_patch_group
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_patch_group` Terraform resources to/from SSM state.
pub struct AwsSsmPatchGroupConverter {
    service: Arc<SsmService>,
}

impl AwsSsmPatchGroupConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmPatchGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_patch_group"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_ssm_patch_baseline"]
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

impl AwsSsmPatchGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::PatchGroupTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_ssm_patch_group", e))?;

        // patch_group membership lives on PatchBaselineView::patch_groups,
        // so we have to read-modify-write the existing baseline.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(baseline) = state_view.patch_baselines.get_mut(&model.baseline_id) {
            if !baseline.patch_groups.contains(&model.patch_group) {
                baseline.patch_groups.push(model.patch_group);
            }
        } else {
            warnings.push(format!(
                "aws_ssm_patch_group: baseline `{}` not found; patch group \
                 `{}` was dropped",
                model.baseline_id, model.patch_group
            ));
        }
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        Ok(ConversionResult { region, warnings })
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
        for baseline in view.patch_baselines.values() {
            for patch_group in &baseline.patch_groups {
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", patch_group, baseline.baseline_id),
                    "baseline_id": baseline.baseline_id,
                    "patch_group": patch_group,
                });
                results.push(ExtractedResource {
                    name: format!("{}_{}", patch_group, baseline.baseline_id),
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
// aws_ssm_resource_data_sync
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_resource_data_sync` Terraform resources to/from SSM state.
pub struct AwsSsmResourceDataSyncConverter {
    service: Arc<SsmService>,
}

impl AwsSsmResourceDataSyncConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmResourceDataSyncConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_resource_data_sync"
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

impl AwsSsmResourceDataSyncConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::ResourceDataSyncTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_resource_data_sync", e))?;

        // Nested s3_destination block: read scalar fields straight from
        // attributes since the spec keeps the model minimal.
        let attrs = &instance.attributes;
        let s3 = attrs
            .get("s3_destination")
            .and_then(|v| v.as_array())
            .and_then(|a| a.first())
            .cloned()
            .unwrap_or(serde_json::Value::Null);
        let bucket = s3
            .get("bucket_name")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_string();
        let s3_region = s3
            .get("region")
            .and_then(|v| v.as_str())
            .unwrap_or(&region)
            .to_string();
        let prefix = s3
            .get("prefix")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let sync_view = ResourceDataSyncView {
            sync_name: model.name.clone(),
            sync_type: "SyncToDestination".to_string(),
            s3_destination_bucket: bucket,
            s3_destination_region: s3_region,
            s3_destination_prefix: prefix,
            created_time: Some(Utc::now().to_rfc3339()),
            last_status: "Successful".to_string(),
        };

        let mut state_view = SsmStateView::default();
        state_view.resource_data_syncs.insert(model.name, sync_view);
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
        for sync in view.resource_data_syncs.values() {
            let s3_destination = serde_json::json!([{
                "bucket_name": sync.s3_destination_bucket,
                "region": sync.s3_destination_region,
                "prefix": sync.s3_destination_prefix,
            }]);
            let attrs = serde_json::json!({
                "id": sync.sync_name,
                "name": sync.sync_name,
                "s3_destination": s3_destination,
            });
            results.push(ExtractedResource {
                name: sync.sync_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_ssm_service_setting
// ---------------------------------------------------------------------------

/// Converts `aws_ssm_service_setting` Terraform resources to/from SSM state.
pub struct AwsSsmServiceSettingConverter {
    service: Arc<SsmService>,
}

impl AwsSsmServiceSettingConverter {
    pub fn new(service: Arc<SsmService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSsmServiceSettingConverter {
    fn resource_type(&self) -> &str {
        "aws_ssm_service_setting"
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

impl AwsSsmServiceSettingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: ssm_gen::ServiceSettingTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_ssm_service_setting", e))?;

        let setting_id = model.setting_id;
        let arn = format!(
            "arn:aws:ssm:{}:{}:servicesetting{}",
            region, ctx.default_account_id, setting_id
        );

        let setting_view = ServiceSettingView {
            setting_id: setting_id.clone(),
            setting_value: model.setting_value,
            last_modified_time: Some(Utc::now().to_rfc3339()),
            arn,
        };

        let mut state_view = SsmStateView::default();
        state_view.service_settings.insert(setting_id, setting_view);
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
        for setting in view.service_settings.values() {
            let attrs = serde_json::json!({
                "id": setting.setting_id,
                "arn": setting.arn,
                "setting_id": setting.setting_id,
                "setting_value": setting.setting_value,
                "last_modified_date": setting.last_modified_time,
                "status": "Default",
            });
            results.push(ExtractedResource {
                name: setting.setting_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
