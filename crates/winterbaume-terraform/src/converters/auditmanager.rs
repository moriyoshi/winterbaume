//! Terraform converters for Audit Manager resources.
//!
//! `ControlTfModel` and `FrameworkTfModel` are generated from
//! `specs/auditmanager.toml`. The synthesised UUID identifiers, the
//! `control_type` / `framework_type` constants, and the
//! `control_mapping_sources` / `control_sets` nested arrays are wired
//! up here.
//!
//! The four no-op / shim resources
//! (`aws_auditmanager_assessment_delegation`,
//! `aws_auditmanager_assessment_report`,
//! `aws_auditmanager_framework_share`,
//! `aws_auditmanager_organization_admin_account_registration`) deserialise
//! the model purely for input validation. AuditManagerState does not track
//! these sub-resources or org bindings, so they emit nothing on extract.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_auditmanager::AuditManagerService;
use winterbaume_auditmanager::views::{
    AssessmentView, AuditManagerStateView, ControlView, FrameworkView,
};
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

// ---------------------------------------------------------------------------
// aws_auditmanager_account_registration
// ---------------------------------------------------------------------------
// Singleton resource that activates Audit Manager in the current account. The
// optional `delegated_admin_account` / `deregister_on_destroy` / `kms_key`
// inputs do not map to local state and are validated but discarded.

pub struct AwsAuditManagerAccountRegistrationConverter {
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerAccountRegistrationConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerAccountRegistrationConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_account_registration"
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

impl AwsAuditManagerAccountRegistrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: auditmanager_gen::AccountRegistrationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_auditmanager_account_registration", e)
            })?;

        let state_view = AuditManagerStateView {
            account_status: "ACTIVE".to_string(),
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
        if view.account_status != "ACTIVE" {
            return Ok(vec![]);
        }
        let attrs = serde_json::json!({
            "id": ctx.default_account_id,
            "status": view.account_status,
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
// aws_auditmanager_assessment
// ---------------------------------------------------------------------------

pub struct AwsAuditManagerAssessmentConverter {
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerAssessmentConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerAssessmentConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_assessment"
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

impl AwsAuditManagerAssessmentConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: auditmanager_gen::AssessmentTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_auditmanager_assessment", e))?;

        let id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let assessment_view = AssessmentView {
            id: id.clone(),
            name: model.name,
            description: model.description,
            framework_id: model.framework_id,
            status: "ACTIVE".to_string(),
            created_at: 0.0,
            tags: model.tags,
        };

        let state_view = AuditManagerStateView {
            assessments: std::iter::once((id, assessment_view)).collect(),
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
        for assessment in view.assessments.values() {
            let arn = format!(
                "arn:aws:auditmanager:{}:{}:assessment/{}",
                ctx.default_region, ctx.default_account_id, assessment.id
            );
            let attrs = serde_json::json!({
                "id": assessment.id,
                "arn": arn,
                "name": assessment.name,
                "description": assessment.description,
                "framework_id": assessment.framework_id,
                "status": assessment.status,
                "tags": assessment.tags,
                "tags_all": assessment.tags,
            });
            results.push(ExtractedResource {
                name: assessment.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_auditmanager_assessment_delegation
// ---------------------------------------------------------------------------
// Delegation sub-resources are not tracked in AuditManagerState. The
// converter validates the model and is otherwise a no-op.

pub struct AwsAuditManagerAssessmentDelegationConverter {
    #[allow(dead_code)]
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerAssessmentDelegationConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerAssessmentDelegationConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_assessment_delegation"
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

impl AwsAuditManagerAssessmentDelegationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: auditmanager_gen::AssessmentDelegationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_auditmanager_assessment_delegation", e)
            })?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_auditmanager_assessment_report
// ---------------------------------------------------------------------------
// Assessment reports are not tracked in AuditManagerState. The converter
// validates the model and is otherwise a no-op.

pub struct AwsAuditManagerAssessmentReportConverter {
    #[allow(dead_code)]
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerAssessmentReportConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerAssessmentReportConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_assessment_report"
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

impl AwsAuditManagerAssessmentReportConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: auditmanager_gen::AssessmentReportTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_auditmanager_assessment_report", e))?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_auditmanager_framework_share
// ---------------------------------------------------------------------------
// Framework share requests are not tracked in AuditManagerState. The
// converter validates the model and is otherwise a no-op.

pub struct AwsAuditManagerFrameworkShareConverter {
    #[allow(dead_code)]
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerFrameworkShareConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerFrameworkShareConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_framework_share"
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

impl AwsAuditManagerFrameworkShareConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: auditmanager_gen::FrameworkShareTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_auditmanager_framework_share", e))?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}

// ---------------------------------------------------------------------------
// aws_auditmanager_organization_admin_account_registration
// ---------------------------------------------------------------------------
// Organization-level admin delegation is not tracked in AuditManagerState.
// The converter validates the model and is otherwise a no-op.

pub struct AwsAuditManagerOrganizationAdminAccountRegistrationConverter {
    #[allow(dead_code)]
    service: Arc<AuditManagerService>,
}

impl AwsAuditManagerOrganizationAdminAccountRegistrationConverter {
    pub fn new(service: Arc<AuditManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsAuditManagerOrganizationAdminAccountRegistrationConverter {
    fn resource_type(&self) -> &str {
        "aws_auditmanager_organization_admin_account_registration"
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

impl AwsAuditManagerOrganizationAdminAccountRegistrationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let _model: auditmanager_gen::OrganizationAdminAccountRegistrationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_auditmanager_organization_admin_account_registration",
                    e,
                )
            })?;

        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }
}
