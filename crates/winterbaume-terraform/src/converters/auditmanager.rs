//! Terraform converters for Audit Manager resources.
//!
//! `ControlTfModel` and `FrameworkTfModel` are generated from
//! `specs/auditmanager.toml`. The synthesised UUID identifiers, the
//! `control_type` / `framework_type` constants, and the
//! `control_mapping_sources` / `control_sets` nested arrays are wired
//! up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_auditmanager::AuditManagerService;
use winterbaume_auditmanager::views::{AuditManagerStateView, ControlView, FrameworkView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::auditmanager as auditmanager_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_auditmanager_control
// ---------------------------------------------------------------------------

pub struct AwsAuditManagerControlConverter {
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerControlConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerControlConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_control"
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

impl AwsAuditManagerControlConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: auditmanager_gen::ControlTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_auditmanager_control", e))?;

        let attrs = &instance.attributes;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let control_mapping_sources: Vec<serde_json::Value> = attrs
            .get("control_mapping_sources")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let control_view = ControlView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            control_type: "Custom".to_string(),
            created_at: 0.0,
            tags: model.tags,
            control_mapping_sources,
        };

        let mut state_view = AuditManagerStateView::default();
        state_view.controls.insert(id, control_view);
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
        for ctrl in view.controls.values() {
            let attrs = serde_json::json!({
                "id": ctrl.id,
                "name": ctrl.name,
                "description": ctrl.description,
                "tags": ctrl.tags,
                "tags_all": ctrl.tags,
                "type": ctrl.control_type,
                "control_mapping_sources": ctrl.control_mapping_sources,
            });
            results.push(ExtractedResource {
                name: ctrl.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_auditmanager_framework
// ---------------------------------------------------------------------------

pub struct AwsAuditManagerFrameworkConverter {
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerFrameworkConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerFrameworkConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_framework"
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

impl AwsAuditManagerFrameworkConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: auditmanager_gen::FrameworkTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_auditmanager_framework", e))?;

        let attrs = &instance.attributes;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let control_sets: Vec<serde_json::Value> = attrs
            .get("control_sets")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let framework_view = FrameworkView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            compliance_type: model.compliance_type,
            framework_type: "Custom".to_string(),
            created_at: 0.0,
            tags: model.tags,
            control_sets,
        };

        let mut state_view = AuditManagerStateView::default();
        state_view.frameworks.insert(id, framework_view);
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
        for fw in view.frameworks.values() {
            let attrs = serde_json::json!({
                "id": fw.id,
                "name": fw.name,
                "description": fw.description,
                "compliance_type": fw.compliance_type,
                "tags": fw.tags,
                "control_sets": fw.control_sets,
            });
            results.push(ExtractedResource {
                name: fw.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
