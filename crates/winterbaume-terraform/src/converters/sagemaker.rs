//! Terraform converters for SageMaker resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_sagemaker::SageMakerService;
use winterbaume_sagemaker::views::{
    DomainView, EndpointConfigView, EndpointView, ModelView, NotebookInstanceView,
    SagemakerStateView, TagPairView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

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
        let domain_name = require_str(attrs, "domain_name", "aws_sagemaker_domain")?;
        let region = extract_region(attrs, &ctx.default_region);

        let domain_id = optional_str(attrs, "id").unwrap_or_else(|| domain_name.to_string());
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:domain/{}",
                region, ctx.default_account_id, domain_id
            )
        });
        let status = optional_str(attrs, "status").unwrap_or_else(|| "InService".to_string());
        let creation_time =
            optional_str(attrs, "creation_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time =
            optional_str(attrs, "last_modified_time").unwrap_or_else(|| creation_time.clone());

        let vpc_id = optional_str(attrs, "vpc_id");
        let subnet_ids: Vec<String> = attrs
            .get("subnet_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let app_network_access_type = optional_str(attrs, "app_network_access_type");
        let auth_mode = optional_str(attrs, "auth_mode");
        let kms_key_id = optional_str(attrs, "kms_key_id");
        let home_efs_file_system_id = optional_str(attrs, "home_efs_file_system_id");
        let security_group_ids: Vec<String> = attrs
            .get("security_group_ids")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let url = optional_str(attrs, "url");
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
            domain_name: domain_name.to_string(),
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
        let name = require_str(attrs, "name", "aws_sagemaker_endpoint")?;
        let region = extract_region(attrs, &ctx.default_region);

        let endpoint_config_name = optional_str(attrs, "endpoint_config_name").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:endpoint/{}",
                region, ctx.default_account_id, name
            )
        });
        let status =
            optional_str(attrs, "endpoint_status").unwrap_or_else(|| "InService".to_string());
        let creation_time =
            optional_str(attrs, "creation_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time =
            optional_str(attrs, "last_modified_time").unwrap_or_else(|| creation_time.clone());

        let endpoint_view = EndpointView {
            endpoint_name: name.to_string(),
            endpoint_arn: arn,
            endpoint_config_name,
            endpoint_status: status,
            creation_time,
            last_modified_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
        };

        let mut state_view = SagemakerStateView::default();
        state_view.endpoints.insert(name.to_string(), endpoint_view);
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
        let name = require_str(attrs, "name", "aws_sagemaker_endpoint_configuration")?;
        let region = extract_region(attrs, &ctx.default_region);

        let _tags_all = attrs.get("tags_all");
        let _kms_key_arn = optional_str(attrs, "kms_key_arn");
        let production_variants = attrs.get("production_variants").cloned();
        let async_inference_config = attrs.get("async_inference_config").cloned();
        let data_capture_config = attrs.get("data_capture_config").cloned();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:endpoint-config/{}",
                region, ctx.default_account_id, name
            )
        });
        let creation_time =
            optional_str(attrs, "creation_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());

        let ec_view = EndpointConfigView {
            endpoint_config_name: name.to_string(),
            endpoint_config_arn: arn,
            creation_time,
            tags: tags_map_to_view(&extract_tags(attrs)),
            production_variants,
            async_inference_config,
            data_capture_config,
        };

        let mut state_view = SagemakerStateView::default();
        state_view
            .endpoint_configs
            .insert(name.to_string(), ec_view);
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
        let name = require_str(attrs, "name", "aws_sagemaker_model")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:model/{}",
                region, ctx.default_account_id, name
            )
        });
        let execution_role_arn = optional_str(attrs, "execution_role_arn").unwrap_or_default();
        let creation_time =
            optional_str(attrs, "creation_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let _tags_all = attrs.get("tags_all");
        let container = attrs.get("container").cloned();
        let primary_container = attrs.get("primary_container").cloned();
        let inference_execution_config = attrs.get("inference_execution_config").cloned();
        let vpc_config = attrs.get("vpc_config").cloned();

        let model_view = ModelView {
            model_name: name.to_string(),
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
        state_view.models.insert(name.to_string(), model_view);
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
        let name = require_str(attrs, "name", "aws_sagemaker_notebook_instance")?;
        let region = extract_region(attrs, &ctx.default_region);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:sagemaker:{}:{}:notebook-instance/{}",
                region, ctx.default_account_id, name
            )
        });
        let instance_type =
            optional_str(attrs, "instance_type").unwrap_or_else(|| "ml.t2.medium".to_string());
        let role_arn = optional_str(attrs, "role_arn").unwrap_or_default();
        let status = optional_str(attrs, "notebook_instance_status")
            .or_else(|| optional_str(attrs, "status"))
            .unwrap_or_else(|| "InService".to_string());
        let creation_time =
            optional_str(attrs, "creation_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let last_modified_time =
            optional_str(attrs, "last_modified_time").unwrap_or_else(|| creation_time.clone());
        let direct_internet_access =
            optional_str(attrs, "direct_internet_access").unwrap_or_else(|| "Enabled".to_string());
        let volume_size_in_gb = optional_i64(attrs, "volume_size").unwrap_or(5);
        let root_access =
            optional_str(attrs, "root_access").unwrap_or_else(|| "Enabled".to_string());
        let url = optional_str(attrs, "url").unwrap_or_default();
        let _tags_all = attrs.get("tags_all");
        let _kms_key_id = optional_str(attrs, "kms_key_id");
        let _direct_internet_access_raw = optional_str(attrs, "direct_internet_access");
        let _volume_size_raw = attrs.get("volume_size");
        let _lifecycle_config_name = optional_str(attrs, "lifecycle_config_name");
        let _subnet_id = optional_str(attrs, "subnet_id");
        let instance_metadata_service_configuration = attrs
            .get("instance_metadata_service_configuration")
            .cloned();

        let nb_view = NotebookInstanceView {
            notebook_instance_name: name.to_string(),
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
        state_view
            .notebook_instances
            .insert(name.to_string(), nb_view);
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
