//! Terraform converters for SageMaker resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_sagemaker::SageMakerService;
use winterbaume_sagemaker::views::{
    AppView, DomainView, EndpointConfigView, EndpointView, FeatureGroupView, JobDefinitionView,
    LifecycleScriptView, ModelPackageGroupView, ModelView, NotebookInstanceLifecycleConfigView,
    NotebookInstanceView, PipelineView, SagemakerStateView, SpaceView, TagPairView,
    UserProfileView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::sagemaker as sagemaker_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags, optional_str};

// ---------------------------------------------------------------------------
// Helper: convert HashMap<String, String> tags to Vec<TagPairView>
// ---------------------------------------------------------------------------

fn tags_map_to_view(tags: &HashMap<String, String>) -> Vec<TagPairView> {
    tags.iter()
        .map(|(k, v)| TagPairView {
            key: k.clone(),
            value: v.clone(),
        })
        .collect()
}

fn tags_view_to_json(tags: &[TagPairView]) -> serde_json::Value {
    let map: serde_json::Map<String, serde_json::Value> = tags
        .iter()
        .map(|t| (t.key.clone(), serde_json::Value::String(t.value.clone())))
        .collect();
    serde_json::Value::Object(map)
}

// ---------------------------------------------------------------------------
// aws_sagemaker_domain
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_domain` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerDomainConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerDomainConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerDomainConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_domain"
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

impl AwsSagemakerDomainConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::DomainTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_domain", e))?;

        let domain_name = model.domain_name.clone();

        let domain_id = model.id.unwrap_or_else(|| domain_name.clone());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:domain/{}",
                region, ctx.default_account_id, domain_id
            )
        });
        let status = model.status.unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let vpc_id = model.vpc_id;
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let app_network_access_type = model.app_network_access_type;
        let auth_mode = model.auth_mode;
        let kms_key_id = model.kms_key_id;
        let home_efs_file_system_id = model.home_efs_file_system_id;
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let url = model.url;
        let _default_user_settings = attrs.get("default_user_settings");
        let default_space_settings = attrs.get("default_space_settings").cloned();
        let domain_settings = attrs.get("domain_settings").cloned();
        let retention_policy = attrs.get("retention_policy").cloned();

        let mut tags_raw = extract_tags(attrs);
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags_raw.entry(k.clone()).or_insert_with(|| s.to_string());
                }
            }
        }

        let domain_view = DomainView {
            domain_id: domain_id.clone(),
            domain_name,
            domain_arn: arn,
            status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&tags_raw),
            vpc_id,
            subnet_ids,
            app_network_access_type,
            auth_mode,
            kms_key_id,
            home_efs_file_system_id,
            security_group_ids,
            url,
            default_space_settings,
            domain_settings,
            retention_policy,
        };

        let mut state_view = SagemakerStateView::default();
        state_view.domains.insert(domain_id, domain_view);
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
        for d in view.domains.values() {
            let attrs = serde_json::json!({
                "id": d.domain_id,
                "domain_name": d.domain_name,
                "arn": d.domain_arn,
                "status": d.status,
                "creation_time": d.creation_time,
                "last_modified_time": d.last_modified_time,
                "vpc_id": d.vpc_id,
                "subnet_ids": d.subnet_ids,
                "app_network_access_type": d.app_network_access_type,
                "auth_mode": d.auth_mode,
                "kms_key_id": d.kms_key_id,
                "home_efs_file_system_id": d.home_efs_file_system_id,
                "security_group_ids": d.security_group_ids,
                "url": d.url,
                "tags": tags_view_to_json(&d.tags),
                "tags_all": tags_view_to_json(&d.tags),
                "default_user_settings": [],
                "default_space_settings": d.default_space_settings,
                "domain_settings": d.domain_settings,
                "retention_policy": d.retention_policy,
            });
            results.push(ExtractedResource {
                name: d.domain_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_endpoint
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_endpoint` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerEndpointConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerEndpointConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerEndpointConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_endpoint"
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

impl AwsSagemakerEndpointConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::EndpointTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_endpoint", e))?;

        let name = model.name.clone();

        let endpoint_config_name = model.endpoint_config_name.unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:endpoint/{}",
                region, ctx.default_account_id, name
            )
        });
        let status = model
            .endpoint_status
            .unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let endpoint_view = EndpointView {
            endpoint_name: name.clone(),
            endpoint_arn: arn,
            endpoint_config_name,
            endpoint_status: status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view.endpoints.insert(name, endpoint_view);
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
        for ep in view.endpoints.values() {
            let attrs = serde_json::json!({
                "id": ep.endpoint_name,
                "name": ep.endpoint_name,
                "arn": ep.endpoint_arn,
                "endpoint_config_name": ep.endpoint_config_name,
                "endpoint_status": ep.endpoint_status,
                "creation_time": ep.creation_time,
                "last_modified_time": ep.last_modified_time,
                "tags": tags_view_to_json(&ep.tags),
            });
            results.push(ExtractedResource {
                name: ep.endpoint_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_endpoint_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_endpoint_configuration` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerEndpointConfigurationConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerEndpointConfigurationConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerEndpointConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_endpoint_configuration"
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

impl AwsSagemakerEndpointConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::EndpointConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_sagemaker_endpoint_configuration", e)
            })?;

        let name = model.name.clone();

        let _tags_all = attrs.get("tags_all");
        let _kms_key_arn = attrs.get("kms_key_arn");
        let production_variants = attrs.get("production_variants").cloned();
        let async_inference_config = attrs.get("async_inference_config").cloned();
        let data_capture_config = attrs.get("data_capture_config").cloned();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:endpoint-config/{}",
                region, ctx.default_account_id, name
            )
        });
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let ec_view = EndpointConfigView {
            endpoint_config_name: name.clone(),
            endpoint_config_arn: arn,
            creation_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
            production_variants,
            async_inference_config,
            data_capture_config,
        };

        let mut state_view = SagemakerStateView::default();
        state_view.endpoint_configs.insert(name, ec_view);
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
        for ec in view.endpoint_configs.values() {
            let attrs = serde_json::json!({
                "id": ec.endpoint_config_name,
                "name": ec.endpoint_config_name,
                "arn": ec.endpoint_config_arn,
                "creation_time": ec.creation_time,
                "tags": tags_view_to_json(&ec.tags),
                "tags_all": tags_view_to_json(&ec.tags),
                "kms_key_arn": serde_json::Value::Null,
                "production_variants": ec.production_variants,
                "async_inference_config": ec.async_inference_config,
                "data_capture_config": ec.data_capture_config,
            });
            results.push(ExtractedResource {
                name: ec.endpoint_config_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_model
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_model` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerModelConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerModelConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerModelConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_model"
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

impl AwsSagemakerModelConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::ModelTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_model", e))?;

        let name = model.name.clone();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:model/{}",
                region, ctx.default_account_id, name
            )
        });
        let execution_role_arn = model.execution_role_arn.unwrap_or_default();
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let _tags_all = attrs.get("tags_all");
        let container = attrs.get("container").cloned();
        let primary_container = attrs.get("primary_container").cloned();
        let inference_execution_config = attrs.get("inference_execution_config").cloned();
        let vpc_config = attrs.get("vpc_config").cloned();

        let model_view = ModelView {
            model_name: name.clone(),
            model_arn: arn,
            execution_role_arn,
            creation_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
            container,
            primary_container,
            inference_execution_config,
            vpc_config,
        };

        let mut state_view = SagemakerStateView::default();
        state_view.models.insert(name, model_view);
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
        for m in view.models.values() {
            let attrs = serde_json::json!({
                "id": m.model_name,
                "name": m.model_name,
                "arn": m.model_arn,
                "execution_role_arn": m.execution_role_arn,
                "creation_time": m.creation_time,
                "tags": tags_view_to_json(&m.tags),
                "tags_all": tags_view_to_json(&m.tags),
                "container": m.container,
                "primary_container": m.primary_container,
                "inference_execution_config": m.inference_execution_config,
                "vpc_config": m.vpc_config,
            });
            results.push(ExtractedResource {
                name: m.model_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_notebook_instance
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_notebook_instance` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerNotebookInstanceConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerNotebookInstanceConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerNotebookInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_notebook_instance"
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

impl AwsSagemakerNotebookInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::NotebookInstanceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_notebook_instance", e))?;

        let name = model.name.clone();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:notebook-instance/{}",
                region, ctx.default_account_id, name
            )
        });
        let instance_type = model
            .instance_type
            .unwrap_or_else(|| "ml.t2.medium".to_string());
        let role_arn = model.role_arn.unwrap_or_default();
        let status = model
            .notebook_instance_status
            .or_else(|| optional_str(attrs, "status"))
            .unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());
        let direct_internet_access = model
            .direct_internet_access
            .unwrap_or_else(|| "Enabled".to_string());
        let volume_size_in_gb = model.volume_size;
        let root_access = model.root_access.unwrap_or_else(|| "Enabled".to_string());
        let url = model.url.unwrap_or_default();
        let _tags_all = attrs.get("tags_all");
        let _kms_key_id = attrs.get("kms_key_id");
        let _lifecycle_config_name = attrs.get("lifecycle_config_name");
        let _subnet_id = attrs.get("subnet_id");
        let instance_metadata_service_configuration = attrs
            .get("instance_metadata_service_configuration")
            .cloned();

        let nb_view = NotebookInstanceView {
            notebook_instance_name: name.clone(),
            notebook_instance_arn: arn,
            notebook_instance_status: status,
            instance_type,
            role_arn,
            creation_time,
            last_modified_time,
            direct_internet_access,
            volume_size_in_gb,
            root_access,
            url,
            instance_metadata_service_configuration,
        };

        let mut state_view = SagemakerStateView::default();
        state_view.notebook_instances.insert(name, nb_view);
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
        for nb in view.notebook_instances.values() {
            let attrs = serde_json::json!({
                "id": nb.notebook_instance_name,
                "name": nb.notebook_instance_name,
                "arn": nb.notebook_instance_arn,
                "notebook_instance_status": nb.notebook_instance_status,
                "instance_type": nb.instance_type,
                "role_arn": nb.role_arn,
                "creation_time": nb.creation_time,
                "last_modified_time": nb.last_modified_time,
                "direct_internet_access": nb.direct_internet_access,
                "volume_size": nb.volume_size_in_gb,
                "root_access": nb.root_access,
                "url": nb.url,
                "network_interface_id": "",
                "tags_all": {},
                "instance_metadata_service_configuration": nb.instance_metadata_service_configuration,
            });
            results.push(ExtractedResource {
                name: nb.notebook_instance_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_app
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_app` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerAppConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerAppConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerAppConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_app"
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

impl AwsSagemakerAppConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::AppTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_sagemaker_app", e))?;

        let domain_id = model.domain_id.clone();
        let app_name = model.app_name.clone();
        let app_type = model.app_type.clone();
        let user_profile_name = model.user_profile_name.clone();
        let space_name = model.space_name.clone();

        let target_segment = user_profile_name
            .clone()
            .or_else(|| space_name.clone())
            .unwrap_or_default();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:app/{}/{}/{}/{}",
                region, ctx.default_account_id, domain_id, target_segment, app_type, app_name
            )
        });
        let status = model.status.unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let app_view = AppView {
            domain_id: domain_id.clone(),
            user_profile_name,
            space_name,
            app_type,
            app_name: app_name.clone(),
            app_arn: arn,
            status,
            creation_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let key = format!(
            "{}/{}/{}",
            domain_id,
            app_view
                .user_profile_name
                .clone()
                .or_else(|| app_view.space_name.clone())
                .unwrap_or_default(),
            app_name,
        );

        let mut state_view = SagemakerStateView::default();
        state_view.apps.insert(key, app_view);
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
        for a in view.apps.values() {
            let attrs = serde_json::json!({
                "id": a.app_arn,
                "domain_id": a.domain_id,
                "user_profile_name": a.user_profile_name,
                "space_name": a.space_name,
                "app_type": a.app_type,
                "app_name": a.app_name,
                "arn": a.app_arn,
                "status": a.status,
                "creation_time": a.creation_time,
                "tags": tags_view_to_json(&a.tags),
                "tags_all": tags_view_to_json(&a.tags),
            });
            results.push(ExtractedResource {
                name: a.app_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_data_quality_job_definition
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_data_quality_job_definition` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerDataQualityJobDefinitionConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerDataQualityJobDefinitionConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerDataQualityJobDefinitionConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_data_quality_job_definition"
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

impl AwsSagemakerDataQualityJobDefinitionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::DataQualityJobDefinitionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_sagemaker_data_quality_job_definition", e)
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:data-quality-job-definition/{}",
                region, ctx.default_account_id, name
            )
        });
        let role_arn = model.role_arn.unwrap_or_default();
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let job_view = JobDefinitionView {
            job_definition_name: name.clone(),
            job_definition_arn: arn,
            role_arn,
            creation_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view
            .data_quality_job_definitions
            .insert(name, job_view);
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
        for j in view.data_quality_job_definitions.values() {
            let attrs = serde_json::json!({
                "id": j.job_definition_arn,
                "name": j.job_definition_name,
                "arn": j.job_definition_arn,
                "role_arn": j.role_arn,
                "creation_time": j.creation_time,
                "tags": tags_view_to_json(&j.tags),
                "tags_all": tags_view_to_json(&j.tags),
            });
            results.push(ExtractedResource {
                name: j.job_definition_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_feature_group
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_feature_group` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerFeatureGroupConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerFeatureGroupConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerFeatureGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_feature_group"
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

impl AwsSagemakerFeatureGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::FeatureGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_feature_group", e))?;

        let name = model.feature_group_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:feature-group/{}",
                region, ctx.default_account_id, name
            )
        });
        let status = model
            .feature_group_status
            .unwrap_or_else(|| "Created".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let fg_view = FeatureGroupView {
            feature_group_name: name.clone(),
            feature_group_arn: arn,
            feature_group_status: status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view.feature_groups.insert(name, fg_view);
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
        for fg in view.feature_groups.values() {
            let attrs = serde_json::json!({
                "id": fg.feature_group_name,
                "feature_group_name": fg.feature_group_name,
                "arn": fg.feature_group_arn,
                "feature_group_status": fg.feature_group_status,
                "creation_time": fg.creation_time,
                "last_modified_time": fg.last_modified_time,
                "tags": tags_view_to_json(&fg.tags),
                "tags_all": tags_view_to_json(&fg.tags),
            });
            results.push(ExtractedResource {
                name: fg.feature_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_model_package_group
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_model_package_group` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerModelPackageGroupConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerModelPackageGroupConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerModelPackageGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_model_package_group"
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

impl AwsSagemakerModelPackageGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::ModelPackageGroupTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_model_package_group", e))?;

        let name = model.model_package_group_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:model-package-group/{}",
                region, ctx.default_account_id, name
            )
        });
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let mpg_view = ModelPackageGroupView {
            model_package_group_name: name.clone(),
            model_package_group_arn: arn,
            model_package_group_status: "Completed".to_string(),
            model_package_group_description: model.model_package_group_description,
            creation_time,
            next_version: 1,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view.model_package_groups.insert(name, mpg_view);
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
        for mpg in view.model_package_groups.values() {
            let attrs = serde_json::json!({
                "id": mpg.model_package_group_name,
                "model_package_group_name": mpg.model_package_group_name,
                "model_package_group_description": mpg.model_package_group_description,
                "arn": mpg.model_package_group_arn,
                "creation_time": mpg.creation_time,
                "tags": tags_view_to_json(&mpg.tags),
                "tags_all": tags_view_to_json(&mpg.tags),
            });
            results.push(ExtractedResource {
                name: mpg.model_package_group_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_notebook_instance_lifecycle_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_notebook_instance_lifecycle_configuration` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerNotebookInstanceLifecycleConfigurationConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerNotebookInstanceLifecycleConfigurationConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerNotebookInstanceLifecycleConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_notebook_instance_lifecycle_configuration"
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

impl AwsSagemakerNotebookInstanceLifecycleConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::NotebookInstanceLifecycleConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error(
                    "aws_sagemaker_notebook_instance_lifecycle_configuration",
                    e,
                )
            })?;

        let name = model.name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:notebook-instance-lifecycle-config/{}",
                region, ctx.default_account_id, name
            )
        });

        let parse_scripts = |key: &str| -> Vec<LifecycleScriptView> {
            attrs
                .get(key)
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            item.get("content").and_then(|c| c.as_str()).map(|s| {
                                LifecycleScriptView {
                                    content: s.to_string(),
                                }
                            })
                        })
                        .collect()
                })
                .unwrap_or_default()
        };
        let on_create = parse_scripts("on_create");
        let on_start = parse_scripts("on_start");

        let creation_time = optional_str(attrs, "creation_time")
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
        let last_modified_time =
            optional_str(attrs, "last_modified_time").unwrap_or_else(|| creation_time.clone());

        let lc_view = NotebookInstanceLifecycleConfigView {
            name: name.clone(),
            arn,
            on_create,
            on_start,
            creation_time,
            last_modified_time,
        };

        let mut state_view = SagemakerStateView::default();
        state_view
            .notebook_instance_lifecycle_configs
            .insert(name, lc_view);
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
        for lc in view.notebook_instance_lifecycle_configs.values() {
            let on_create: Vec<serde_json::Value> = lc
                .on_create
                .iter()
                .map(|s| serde_json::json!({"content": s.content}))
                .collect();
            let on_start: Vec<serde_json::Value> = lc
                .on_start
                .iter()
                .map(|s| serde_json::json!({"content": s.content}))
                .collect();
            let attrs = serde_json::json!({
                "id": lc.name,
                "name": lc.name,
                "arn": lc.arn,
                "on_create": on_create,
                "on_start": on_start,
            });
            results.push(ExtractedResource {
                name: lc.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_pipeline
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_pipeline` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerPipelineConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerPipelineConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerPipelineConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_pipeline"
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

impl AwsSagemakerPipelineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::PipelineTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_pipeline", e))?;

        let name = model.pipeline_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:pipeline/{}",
                region, ctx.default_account_id, name
            )
        });
        let role_arn = model.role_arn.unwrap_or_default();
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let pipeline_view = PipelineView {
            pipeline_name: name.clone(),
            pipeline_arn: arn,
            pipeline_display_name: model.pipeline_display_name,
            pipeline_description: model.pipeline_description,
            pipeline_definition: model.pipeline_definition,
            role_arn,
            creation_time,
            last_modified_time,
            executions: vec![],
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view.pipelines.insert(name, pipeline_view);
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
        for p in view.pipelines.values() {
            let attrs = serde_json::json!({
                "id": p.pipeline_name,
                "pipeline_name": p.pipeline_name,
                "pipeline_display_name": p.pipeline_display_name,
                "pipeline_description": p.pipeline_description,
                "pipeline_definition": p.pipeline_definition,
                "role_arn": p.role_arn,
                "arn": p.pipeline_arn,
                "creation_time": p.creation_time,
                "last_modified_time": p.last_modified_time,
                "tags": tags_view_to_json(&p.tags),
                "tags_all": tags_view_to_json(&p.tags),
            });
            results.push(ExtractedResource {
                name: p.pipeline_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_space
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_space` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerSpaceConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerSpaceConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerSpaceConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_space"
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

impl AwsSagemakerSpaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::SpaceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_space", e))?;

        let domain_id = model.domain_id.clone();
        let space_name = model.space_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:space/{}/{}",
                region, ctx.default_account_id, domain_id, space_name
            )
        });
        let status = model.status.unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let space_view = SpaceView {
            domain_id: domain_id.clone(),
            space_name: space_name.clone(),
            space_arn: arn,
            status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let key = format!("{}/{}", domain_id, space_name);
        let mut state_view = SagemakerStateView::default();
        state_view.spaces.insert(key, space_view);
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
        for s in view.spaces.values() {
            let attrs = serde_json::json!({
                "id": s.space_arn,
                "domain_id": s.domain_id,
                "space_name": s.space_name,
                "arn": s.space_arn,
                "status": s.status,
                "creation_time": s.creation_time,
                "last_modified_time": s.last_modified_time,
                "tags": tags_view_to_json(&s.tags),
                "tags_all": tags_view_to_json(&s.tags),
            });
            results.push(ExtractedResource {
                name: s.space_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_sagemaker_user_profile
// ---------------------------------------------------------------------------

/// Converts `aws_sagemaker_user_profile` Terraform resources to/from SageMaker state.
pub struct AwsSagemakerUserProfileConverter {
    service: Arc<SageMakerService>,
}

impl AwsSagemakerUserProfileConverter {
    pub fn new(service: Arc<SageMakerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSagemakerUserProfileConverter {
    fn resource_type(&self) -> &str {
        "aws_sagemaker_user_profile"
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

impl AwsSagemakerUserProfileConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: sagemaker_gen::UserProfileTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sagemaker_user_profile", e))?;

        let domain_id = model.domain_id.clone();
        let user_profile_name = model.user_profile_name.clone();
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:user-profile/{}/{}",
                region, ctx.default_account_id, domain_id, user_profile_name
            )
        });
        let status = model.status.unwrap_or_else(|| "InService".to_string());
        let creation_time = model
            .creation_time
            .unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time = model
            .last_modified_time
            .unwrap_or_else(|| creation_time.clone());

        let up_view = UserProfileView {
            domain_id: domain_id.clone(),
            user_profile_name: user_profile_name.clone(),
            user_profile_arn: arn,
            status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let key = format!("{}/{}", domain_id, user_profile_name);
        let mut state_view = SagemakerStateView::default();
        state_view.user_profiles.insert(key, up_view);
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
        for up in view.user_profiles.values() {
            let attrs = serde_json::json!({
                "id": up.user_profile_arn,
                "domain_id": up.domain_id,
                "user_profile_name": up.user_profile_name,
                "arn": up.user_profile_arn,
                "status": up.status,
                "creation_time": up.creation_time,
                "last_modified_time": up.last_modified_time,
                "tags": tags_view_to_json(&up.tags),
                "tags_all": tags_view_to_json(&up.tags),
            });
            results.push(ExtractedResource {
                name: up.user_profile_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Warning-only converters
//
// These resource types lack a matching state slot in winterbaume_sagemaker.
// Inject deserialises the TF payload (so type errors are still surfaced)
// then emits a single warning and skips writing into state. Extract returns
// an empty Vec, since there is nothing to project back.
// ---------------------------------------------------------------------------

macro_rules! sagemaker_warning_only_converter {
    (
        struct_name = $struct_name:ident,
        resource_type = $resource_type:expr,
        model_type = $model_type:ident,
        warn_msg = $warn_msg:expr $(,)?
    ) => {
        pub struct $struct_name {
            #[allow(dead_code)]
            service: Arc<SageMakerService>,
        }

        impl $struct_name {
            pub fn new(service: Arc<SageMakerService>) -> Self {
                Self { service }
            }
        }

        impl TerraformResourceConverter for $struct_name {
            fn resource_type(&self) -> &str {
                $resource_type
            }

            fn inject<'a>(
                &'a self,
                instance: &'a ResourceInstance,
                ctx: &'a ConversionContext,
            ) -> Pin<
                Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>,
            > {
                Box::pin(async move { self.do_inject(instance, ctx).await })
            }

            fn extract<'a>(
                &'a self,
                _ctx: &'a ConversionContext,
            ) -> Pin<
                Box<
                    dyn Future<Output = Result<Vec<ExtractedResource>, ConversionError>>
                        + Send
                        + 'a,
                >,
            > {
                Box::pin(async move { Ok(vec![]) })
            }
        }

        impl $struct_name {
            async fn do_inject(
                &self,
                instance: &ResourceInstance,
                ctx: &ConversionContext,
            ) -> Result<ConversionResult, ConversionError> {
                let attrs = &instance.attributes;
                let region = extract_region(attrs, &ctx.default_region);
                let _model: sagemaker_gen::$model_type = serde_json::from_value(attrs.clone())
                    .map_err(|e| classify_deserialize_error($resource_type, e))?;
                eprintln!("warning: {}: {}", $resource_type, $warn_msg);
                Ok(ConversionResult {
                    region,
                    warnings: vec![format!("{}: {}", $resource_type, $warn_msg)],
                })
            }
        }
    };
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerAppImageConfigConverter,
    resource_type = "aws_sagemaker_app_image_config",
    model_type = AppImageConfigTfModel,
    warn_msg = "app-image-config slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerCodeRepositoryConverter,
    resource_type = "aws_sagemaker_code_repository",
    model_type = CodeRepositoryTfModel,
    warn_msg = "code-repository slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerDeviceConverter,
    resource_type = "aws_sagemaker_device",
    model_type = DeviceTfModel,
    warn_msg = "device slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerDeviceFleetConverter,
    resource_type = "aws_sagemaker_device_fleet",
    model_type = DeviceFleetTfModel,
    warn_msg = "device-fleet slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerFlowDefinitionConverter,
    resource_type = "aws_sagemaker_flow_definition",
    model_type = FlowDefinitionTfModel,
    warn_msg = "flow-definition slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerHubConverter,
    resource_type = "aws_sagemaker_hub",
    model_type = HubTfModel,
    warn_msg = "hub slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerHumanTaskUiConverter,
    resource_type = "aws_sagemaker_human_task_ui",
    model_type = HumanTaskUiTfModel,
    warn_msg = "human-task-ui slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerImageConverter,
    resource_type = "aws_sagemaker_image",
    model_type = ImageTfModel,
    warn_msg = "image slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerImageVersionConverter,
    resource_type = "aws_sagemaker_image_version",
    model_type = ImageVersionTfModel,
    warn_msg = "image-version slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerMlflowTrackingServerConverter,
    resource_type = "aws_sagemaker_mlflow_tracking_server",
    model_type = MlflowTrackingServerTfModel,
    warn_msg = "mlflow-tracking-server slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerModelPackageGroupPolicyConverter,
    resource_type = "aws_sagemaker_model_package_group_policy",
    model_type = ModelPackageGroupPolicyTfModel,
    warn_msg = "model-package-group resource_policy field not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerMonitoringScheduleConverter,
    resource_type = "aws_sagemaker_monitoring_schedule",
    model_type = MonitoringScheduleTfModel,
    warn_msg = "monitoring-schedule slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerProjectConverter,
    resource_type = "aws_sagemaker_project",
    model_type = ProjectTfModel,
    warn_msg = "project slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerServicecatalogPortfolioStatusConverter,
    resource_type = "aws_sagemaker_servicecatalog_portfolio_status",
    model_type = ServicecatalogPortfolioStatusTfModel,
    warn_msg = "servicecatalog portfolio status not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerStudioLifecycleConfigConverter,
    resource_type = "aws_sagemaker_studio_lifecycle_config",
    model_type = StudioLifecycleConfigTfModel,
    warn_msg = "studio-lifecycle-config slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerWorkforceConverter,
    resource_type = "aws_sagemaker_workforce",
    model_type = WorkforceTfModel,
    warn_msg = "workforce slot not modelled in winterbaume_sagemaker; inject is a no-op",
}

sagemaker_warning_only_converter! {
    struct_name = AwsSagemakerWorkteamConverter,
    resource_type = "aws_sagemaker_workteam",
    model_type = WorkteamTfModel,
    warn_msg = "workteam slot not modelled in winterbaume_sagemaker; inject is a no-op",
}
