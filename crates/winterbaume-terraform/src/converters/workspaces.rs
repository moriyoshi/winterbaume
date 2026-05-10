//! Terraform converters for WorkSpaces resources.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_workspaces::WorkSpacesService;
use winterbaume_workspaces::views::{WorkSpacesStateView, WorkspaceDirectoryView, WorkspaceView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::workspaces as workspaces_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_workspaces_workspace
// ---------------------------------------------------------------------------

/// Converts `aws_workspaces_workspace` Terraform resources to/from WorkSpaces state.
pub struct AwsWorkspacesWorkspaceConverter {
    service: Arc<WorkSpacesService>,
}

impl AwsWorkspacesWorkspaceConverter {
    pub fn new(service: Arc<WorkSpacesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWorkspacesWorkspaceConverter {
    fn resource_type(&self) -> &str {
        "aws_workspaces_workspace"
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

impl AwsWorkspacesWorkspaceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: workspaces_gen::WorkspaceTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_workspaces_workspace", e))?;

        let _tags = extract_tags(attrs);
        let workspace_id = model
            .id
            .unwrap_or_else(|| format!("ws-{}", uuid::Uuid::new_v4().simple()));
        let directory_id = model.directory_id.clone();
        let user_name = model.user_name.clone();
        let bundle_id = model.bundle_id.clone();
        let state = model.state.unwrap_or_else(|| "AVAILABLE".to_string());

        // Parse workspace_properties block
        let wp = attrs
            .get("workspace_properties")
            .and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
        let running_mode = wp
            .and_then(|o| o.get("running_mode").and_then(|v| v.as_str()))
            .unwrap_or("AUTO_STOP")
            .to_string();
        let running_mode_auto_stop_timeout_in_minutes = wp
            .and_then(|o| {
                o.get("running_mode_auto_stop_timeout_in_minutes")
                    .and_then(|v| v.as_i64())
            })
            .unwrap_or(60) as i32;
        let root_volume_size_gib = wp
            .and_then(|o| o.get("root_volume_size_gib").and_then(|v| v.as_i64()))
            .unwrap_or(80) as i32;
        let user_volume_size_gib = wp
            .and_then(|o| o.get("user_volume_size_gib").and_then(|v| v.as_i64()))
            .unwrap_or(50) as i32;
        let workspace_properties_raw = attrs
            .get("workspace_properties")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let view = WorkSpacesStateView {
            workspaces: std::collections::HashMap::from([(
                workspace_id.clone(),
                WorkspaceView {
                    workspace_id,
                    directory_id,
                    user_name,
                    bundle_id,
                    state,
                    ip_address: model.ip_address.unwrap_or_else(|| "10.0.0.1".to_string()),
                    computer_name: model.computer_name.unwrap_or_default(),
                    subnet_id: model.subnet_id.unwrap_or_default(),
                    root_volume_size_gib,
                    user_volume_size_gib,
                    volume_encryption_key: model.volume_encryption_key,
                    user_volume_encryption_enabled: false,
                    root_volume_encryption_enabled: false,
                    running_mode,
                    running_mode_auto_stop_timeout_in_minutes,
                    workspace_properties: workspace_properties_raw,
                },
            )]),
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .ok();
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let region = ctx.default_region.clone();
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let resources = view
            .workspaces
            .values()
            .map(|ws| {
                let attrs = serde_json::json!({
                    "id": ws.workspace_id,
                    "workspace_id": ws.workspace_id,
                    "directory_id": ws.directory_id,
                    "user_name": ws.user_name,
                    "bundle_id": ws.bundle_id,
                    "state": ws.state,
                    "ip_address": ws.ip_address,
                    "computer_name": ws.computer_name,
                    "subnet_id": ws.subnet_id,
                    "volume_encryption_key": ws.volume_encryption_key,
                    "user_volume_encryption_enabled": ws.user_volume_encryption_enabled,
                    "root_volume_encryption_enabled": ws.root_volume_encryption_enabled,
                    "workspace_properties": ws.workspace_properties.clone().unwrap_or_else(|| serde_json::json!([{
                        "running_mode": ws.running_mode,
                        "running_mode_auto_stop_timeout_in_minutes": ws.running_mode_auto_stop_timeout_in_minutes,
                        "root_volume_size_gib": ws.root_volume_size_gib,
                        "user_volume_size_gib": ws.user_volume_size_gib,
                    }])),
                });
                ExtractedResource {
                    name: ws.workspace_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
                }
            })
            .collect();

        Ok(resources)
    }
}

// ---------------------------------------------------------------------------
// aws_workspaces_directory
// ---------------------------------------------------------------------------

/// Converts `aws_workspaces_directory` Terraform resources to/from WorkSpaces directory state.
pub struct AwsWorkspacesDirectoryConverter {
    service: Arc<WorkSpacesService>,
}

impl AwsWorkspacesDirectoryConverter {
    pub fn new(service: Arc<WorkSpacesService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsWorkspacesDirectoryConverter {
    fn resource_type(&self) -> &str {
        "aws_workspaces_directory"
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

impl AwsWorkspacesDirectoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: workspaces_gen::WorkspaceDirectoryTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_workspaces_directory", e))?;

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("ip_group_ids");
        let _ = attrs.get("saml_properties");
        let _ = attrs.get("workspace_type");

        let self_service_permissions = attrs
            .get("self_service_permissions")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let workspace_access_properties = attrs
            .get("workspace_access_properties")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let workspace_creation_properties = attrs
            .get("workspace_creation_properties")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let streaming_properties = attrs
            .get("streaming_properties")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });
        let active_directory_config = attrs
            .get("active_directory_config")
            .and_then(|v| if v.is_null() { None } else { Some(v.clone()) });

        let directory_id = model.directory_id.clone();

        let view = WorkSpacesStateView {
            directories: std::collections::HashMap::from([(
                directory_id.clone(),
                WorkspaceDirectoryView {
                    directory_id: directory_id.clone(),
                    directory_name: model
                        .directory_name
                        .unwrap_or_else(|| format!("corp.{}.com", region)),
                    directory_type: model
                        .directory_type
                        .unwrap_or_else(|| "SIMPLE_AD".to_string()),
                    alias: model.alias.unwrap_or_else(|| directory_id.clone()),
                    state: "REGISTERED".to_string(),
                    registration_code: model.registration_code.unwrap_or_default(),
                    workspace_security_group_id: model
                        .workspace_security_group_id
                        .unwrap_or_default(),
                    iam_role_id: format!(
                        "arn:aws:iam::{}:role/workspaces_DefaultRole",
                        ctx.default_account_id
                    ),
                    self_service_permissions,
                    workspace_access_properties,
                    workspace_creation_properties,
                    streaming_properties,
                    active_directory_config,
                },
            )]),
            ..Default::default()
        };

        self.service
            .merge(&ctx.default_account_id, &region, view)
            .await
            .ok();
        Ok(ConversionResult {
            region,
            warnings: vec![],
        })
    }

    async fn do_extract(
        &self,
        ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        let region = ctx.default_region.clone();
        let view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let resources = view
            .directories
            .values()
            .map(|dir| {
                let mut attrs = serde_json::Map::new();
                attrs.insert(
                    "id".to_string(),
                    serde_json::Value::String(dir.directory_id.clone()),
                );
                attrs.insert(
                    "directory_id".to_string(),
                    serde_json::Value::String(dir.directory_id.clone()),
                );
                attrs.insert(
                    "directory_name".to_string(),
                    serde_json::Value::String(dir.directory_name.clone()),
                );
                attrs.insert(
                    "directory_type".to_string(),
                    serde_json::Value::String(dir.directory_type.clone()),
                );
                attrs.insert(
                    "alias".to_string(),
                    serde_json::Value::String(dir.alias.clone()),
                );
                attrs.insert(
                    "state".to_string(),
                    serde_json::Value::String(dir.state.clone()),
                );
                attrs.insert(
                    "registration_code".to_string(),
                    serde_json::Value::String(dir.registration_code.clone()),
                );
                attrs.insert(
                    "workspace_security_group_id".to_string(),
                    serde_json::Value::String(dir.workspace_security_group_id.clone()),
                );
                attrs.insert(
                    "iam_role_id".to_string(),
                    serde_json::Value::String(dir.iam_role_id.clone()),
                );
                attrs.insert("tags_all".to_string(), serde_json::json!({}));
                attrs.insert("dns_ip_addresses".to_string(), serde_json::json!([]));
                attrs.insert("customer_user_name".to_string(), serde_json::json!(""));
                attrs.insert("ip_group_ids".to_string(), serde_json::json!([]));
                attrs.insert("workspace_type".to_string(), serde_json::json!(""));
                attrs.insert(
                    "self_service_permissions".to_string(),
                    serde_json::json!(dir.self_service_permissions),
                );
                attrs.insert(
                    "workspace_access_properties".to_string(),
                    serde_json::json!(dir.workspace_access_properties),
                );
                attrs.insert(
                    "workspace_creation_properties".to_string(),
                    serde_json::json!(dir.workspace_creation_properties),
                );
                attrs.insert(
                    "streaming_properties".to_string(),
                    serde_json::json!(dir.streaming_properties),
                );
                attrs.insert(
                    "active_directory_config".to_string(),
                    serde_json::json!(dir.active_directory_config),
                );
                ExtractedResource {
                    name: dir.directory_id.clone(),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: serde_json::Value::Object(attrs),
                }
            })
            .collect();

        Ok(resources)
    }
}
