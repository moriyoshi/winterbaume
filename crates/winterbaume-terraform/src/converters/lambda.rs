//! Terraform converters for Lambda resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_lambda::LambdaService;
use winterbaume_lambda::types::{
    CodeSigningConfigEntry, EventSourceMappingScalingSettings, EventSourceMappingSettings,
    FunctionFileSystemConfig, FunctionImageConfig, FunctionLayer, FunctionLoggingConfig,
    FunctionRecursionConfig, FunctionSettings, FunctionVpcConfig, ProvisionedConcurrencyConfig,
    RuntimeManagementConfig, event_source_mapping_arn,
};
use winterbaume_lambda::views::{
    AliasView, EventSourceMappingView, FunctionEventInvokeConfigView, FunctionUrlConfigView,
    LambdaFunctionView, LambdaStateView, LayerPermissionView, LayerVersionView, PermissionView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::lambda as lambda_gen;
use crate::util::{classify_deserialize_error, extract_region, optional_str};

// ---------------------------------------------------------------------------
// aws_lambda_function
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_function` Terraform resources to/from Lambda state.
pub struct AwsLambdaFunctionConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaFunctionConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaFunctionConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_function"
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

impl AwsLambdaFunctionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let mut model: lambda_gen::LambdaFunctionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_function", e))?;

        // Additional fields for coverage (TF-only / not stored in state).
        let _ = attrs.get("tags_all");
        let _ = attrs.get("publish");
        let _ = attrs.get("s3_bucket");
        let _ = attrs.get("replace_security_groups_on_destroy");
        let _ = attrs.get("skip_destroy");

        // Nested blocks and lists, translated to the API-shaped settings the
        // Lambda handlers serve.
        let settings = function_settings_from_tf(
            attrs,
            model.kms_key_arn.take(),
            model.package_type.take(),
            model.image_uri.take(),
        );

        let function_name = model.function_name.clone();
        let runtime = model.runtime.unwrap_or_else(|| "provided".to_string());
        let handler = model.handler.unwrap_or_default();
        let role = model.role.unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:lambda:{}:{}:function:{}",
                region, ctx.default_account_id, function_name
            )
        });

        let memory_size = model.memory_size as i32;
        let timeout = model.timeout as i32;
        let description = model.description.unwrap_or_default();

        // environment variables
        let environment: HashMap<String, String> = attrs
            .get("environment")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("variables"))
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                    .collect()
            })
            .unwrap_or_default();

        let tags = tags_from_tf(attrs);

        let fn_view = LambdaFunctionView {
            function_name: function_name.clone(),
            function_arn: arn,
            runtime,
            handler,
            role,
            code_sha256: String::new(),
            code_size: 0,
            memory_size,
            timeout,
            environment,
            description,
            last_modified: Utc::now().to_rfc3339(),
            state: "Active".to_string(),
            version: "$LATEST".to_string(),
            tags,
            versions: vec![],
            reserved_concurrent_executions: attrs
                .get("reserved_concurrent_executions")
                .and_then(|v| v.as_i64())
                .map(|v| v as i32),
            code_signing_config_arn: model.code_signing_config_arn,
            settings,
            revision_id: uuid::Uuid::new_v4().to_string(),
        };

        let mut state_view = minimal_lambda_state_view(&ctx.default_account_id, &region);
        state_view.functions.insert(function_name, fn_view);
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
        for func in view.functions.values() {
            let environment_block: serde_json::Value = if func.environment.is_empty() {
                serde_json::json!([])
            } else {
                serde_json::json!([{
                    "variables": func.environment,
                }])
            };
            let mut attrs = serde_json::json!({
                "id": func.function_name,
                "function_name": func.function_name,
                "arn": func.function_arn,
                "runtime": func.runtime,
                "handler": func.handler,
                "role": func.role,
                "memory_size": func.memory_size,
                "timeout": func.timeout,
                "description": func.description,
                "tags": func.tags,
                "code_sha256": func.code_sha256,
                "code_size": func.code_size,
                "last_modified": func.last_modified,
                "state": func.state,
                "version": func.version,
                "reserved_concurrent_executions": func.reserved_concurrent_executions,
                "code_signing_config_arn": func.code_signing_config_arn,
                "environment": environment_block,
                "tags_all": func.tags,
                "invoke_arn": format!("arn:aws:apigateway:{}:lambda:path/2015-03-31/functions/{}/invocations", ctx.default_region, func.function_arn),
                "qualified_arn": format!("{}:$LATEST", func.function_arn),
                "qualified_invoke_arn": "",
                "signing_job_arn": "",
                "signing_profile_version_arn": "",
                "source_code_size": 0,
            });
            // Nested blocks and lists, in the shape the provider stores them.
            let obj = attrs.as_object_mut().unwrap();
            obj.extend(function_settings_to_tf(&func.function_name, &func.settings));
            results.push(ExtractedResource {
                name: func.function_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

/// First element of a Terraform single-block list ( `vpc_config = [{...}]` ).
fn tf_block<'a>(attrs: &'a serde_json::Value, key: &str) -> Option<&'a serde_json::Value> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .and_then(|blocks| blocks.first())
        .filter(|block| block.is_object())
}

fn tf_string(block: &serde_json::Value, key: &str) -> Option<String> {
    block
        .get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(str::to_string)
}

fn tf_string_list(value: Option<&serde_json::Value>) -> Vec<String> {
    value
        .and_then(|v| v.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|item| item.as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Translate the `aws_lambda_function` attributes that the Lambda API calls
/// `VpcConfig`, `Architectures`, `EphemeralStorage`, `TracingConfig`,
/// `DeadLetterConfig`, `KMSKeyArn`, `Layers`, `FileSystemConfigs`,
/// `ImageConfig`, `LoggingConfig`, `SnapStart` and `PackageType` into the
/// API-shaped settings of the Lambda state.
fn function_settings_from_tf(
    attrs: &serde_json::Value,
    kms_key_arn: Option<String>,
    package_type: Option<String>,
    image_uri: Option<String>,
) -> FunctionSettings {
    let non_empty = |v: Option<String>| v.filter(|s| !s.is_empty());
    let architectures = tf_string_list(attrs.get("architectures"));
    FunctionSettings {
        architectures: (!architectures.is_empty()).then_some(architectures),
        dead_letter_target_arn: tf_block(attrs, "dead_letter_config")
            .and_then(|b| tf_string(b, "target_arn")),
        ephemeral_storage_size: tf_block(attrs, "ephemeral_storage")
            .and_then(|b| b.get("size"))
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        file_system_configs: attrs
            .get("file_system_config")
            .and_then(|v| v.as_array())
            .map(|blocks| {
                blocks
                    .iter()
                    .filter(|b| b.is_object())
                    .map(|b| FunctionFileSystemConfig {
                        arn: tf_string(b, "arn").unwrap_or_default(),
                        local_mount_path: tf_string(b, "local_mount_path").unwrap_or_default(),
                    })
                    .collect()
            })
            .unwrap_or_default(),
        image_config: tf_block(attrs, "image_config")
            .map(|b| {
                let list = |key: &str| {
                    let items = tf_string_list(b.get(key));
                    (!items.is_empty()).then_some(items)
                };
                FunctionImageConfig {
                    command: list("command"),
                    entry_point: list("entry_point"),
                    working_directory: tf_string(b, "working_directory"),
                }
            })
            .filter(|c| !c.is_empty()),
        image_uri: non_empty(image_uri),
        kms_key_arn: non_empty(kms_key_arn),
        layers: tf_string_list(attrs.get("layers"))
            .into_iter()
            // The Terraform state records layer ARNs only.
            .map(|arn| FunctionLayer { arn, code_size: 0 })
            .collect(),
        logging_config: tf_block(attrs, "logging_config").map(|b| FunctionLoggingConfig {
            application_log_level: tf_string(b, "application_log_level"),
            log_format: tf_string(b, "log_format"),
            log_group: tf_string(b, "log_group"),
            system_log_level: tf_string(b, "system_log_level"),
        }),
        package_type: non_empty(package_type),
        snap_start_apply_on: tf_block(attrs, "snap_start").and_then(|b| tf_string(b, "apply_on")),
        tracing_mode: tf_block(attrs, "tracing_config").and_then(|b| tf_string(b, "mode")),
        vpc_config: tf_block(attrs, "vpc_config").map(|b| FunctionVpcConfig {
            subnet_ids: tf_string_list(b.get("subnet_ids")),
            security_group_ids: tf_string_list(b.get("security_group_ids")),
            ipv6_allowed_for_dual_stack: b
                .get("ipv6_allowed_for_dual_stack")
                .and_then(|v| v.as_bool())
                .unwrap_or(false),
            vpc_id: tf_string(b, "vpc_id"),
        }),
    }
}

/// The reverse of [`function_settings_from_tf`]: the attributes as the AWS
/// provider writes them to its state after reading the function back, which
/// includes the documented defaults for members that were never set.
fn function_settings_to_tf(
    function_name: &str,
    settings: &FunctionSettings,
) -> serde_json::Map<String, serde_json::Value> {
    use serde_json::json;

    let logging = settings.effective_logging_config(function_name);
    let snap_start_apply_on = settings.effective_snap_start_apply_on();
    let mut out = serde_json::Map::new();
    out.insert(
        "architectures".into(),
        json!(settings.effective_architectures()),
    );
    out.insert(
        "dead_letter_config".into(),
        match &settings.dead_letter_target_arn {
            Some(arn) => json!([{ "target_arn": arn }]),
            None => json!([]),
        },
    );
    out.insert(
        "ephemeral_storage".into(),
        json!([{ "size": settings.effective_ephemeral_storage_size() }]),
    );
    out.insert(
        "file_system_config".into(),
        settings
            .file_system_configs
            .iter()
            .map(|c| json!({ "arn": c.arn, "local_mount_path": c.local_mount_path }))
            .collect(),
    );
    out.insert(
        "image_config".into(),
        match &settings.image_config {
            Some(c) => json!([{
                "command": c.command.clone().unwrap_or_default(),
                "entry_point": c.entry_point.clone().unwrap_or_default(),
                "working_directory": c.working_directory.clone().unwrap_or_default(),
            }]),
            None => json!([]),
        },
    );
    out.insert(
        "image_uri".into(),
        json!(settings.image_uri.clone().unwrap_or_default()),
    );
    out.insert(
        "kms_key_arn".into(),
        json!(settings.kms_key_arn.clone().unwrap_or_default()),
    );
    out.insert(
        "layers".into(),
        settings.layers.iter().map(|l| json!(l.arn)).collect(),
    );
    out.insert(
        "logging_config".into(),
        json!([{
            "application_log_level": logging.application_log_level.unwrap_or_default(),
            "log_format": logging.log_format.unwrap_or_default(),
            "log_group": logging.log_group.unwrap_or_default(),
            "system_log_level": logging.system_log_level.unwrap_or_default(),
        }]),
    );
    out.insert(
        "package_type".into(),
        json!(settings.effective_package_type()),
    );
    out.insert(
        "snap_start".into(),
        // The provider keeps no block for `ApplyOn = None`.
        if snap_start_apply_on == "None" {
            json!([])
        } else {
            json!([{ "apply_on": snap_start_apply_on, "optimization_status": "Off" }])
        },
    );
    out.insert(
        "tracing_config".into(),
        json!([{ "mode": settings.effective_tracing_mode() }]),
    );
    out.insert(
        "vpc_config".into(),
        // The provider keeps no block for a function outside a VPC, whether
        // it never had one or was detached.
        match settings.vpc_config.as_ref().filter(|v| v.is_attached()) {
            Some(vpc) => json!([{
                "ipv6_allowed_for_dual_stack": vpc.ipv6_allowed_for_dual_stack,
                "security_group_ids": vpc.security_group_ids,
                "subnet_ids": vpc.subnet_ids,
                "vpc_id": vpc.vpc_id.clone().unwrap_or_default(),
            }]),
            None => json!([]),
        },
    );
    out
}

// ---------------------------------------------------------------------------
// aws_lambda_alias
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_alias` Terraform resources to/from Lambda state.
pub struct AwsLambdaAliasConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaAliasConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaAliasConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_alias"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaAliasConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaAliasTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_alias", e))?;

        let name = model.name.clone();
        let function_name = model.function_name.clone();
        let function_version = model
            .function_version
            .unwrap_or_else(|| "$LATEST".to_string());
        let description = model.description.unwrap_or_default();

        let alias_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:lambda:{}:{}:function:{}:{}",
                region, ctx.default_account_id, function_name, name
            )
        });

        let key = format!("{}:{}", function_name, name);
        let alias_view = AliasView {
            name,
            function_name,
            function_version,
            description,
            alias_arn,
            revision_id: uuid::Uuid::new_v4().to_string(),
            routing_config: None,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.aliases.insert(key, alias_view);
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
        for alias in view.aliases.values() {
            let attrs = serde_json::json!({
                "id": format!("{}:{}", alias.function_name, alias.name),
                "name": alias.name,
                "function_name": alias.function_name,
                "function_version": alias.function_version,
                "description": alias.description,
                "arn": alias.alias_arn,
            });
            results.push(ExtractedResource {
                name: format!("{}:{}", alias.function_name, alias.name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_permission
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_permission` Terraform resources to/from Lambda state.
pub struct AwsLambdaPermissionConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaPermissionConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaPermissionConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_permission"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaPermissionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaPermissionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_permission", e))?;

        // Terraform state may carry unset optional strings as "".
        let non_empty = |v: Option<String>| v.filter(|s| !s.is_empty());

        let function_name = model.function_name.clone();
        let statement_id = model.statement_id.clone();
        let perm_view = PermissionView {
            statement_id: statement_id.clone(),
            action: model.action,
            principal: model.principal,
            source_arn: model.source_arn,
            source_account: model.source_account,
            principal_org_id: non_empty(model.principal_org_id),
            event_source_token: non_empty(model.event_source_token),
            function_url_auth_type: non_empty(model.function_url_auth_type),
            // Option<bool>, outside the spec vocabulary: read raw.
            invoked_via_function_url: attrs
                .get("invoked_via_function_url")
                .and_then(|v| v.as_bool()),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let perms = state_view
            .function_permissions
            .entry(function_name.clone())
            .or_default();
        perms.retain(|p| p.statement_id != statement_id);
        perms.push(perm_view);
        // Bump the policy revision id to mirror what AddPermission does on the
        // live API: a function with permissions must always carry a non-empty
        // revision id, otherwise GetPolicy returns "" and the AWS provider's
        // optimistic-concurrency contract is broken.
        state_view
            .function_policy_revisions
            .insert(function_name, uuid::Uuid::new_v4().to_string());
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
        for (function_name, perms) in &view.function_permissions {
            for perm in perms {
                let id = format!("{}/{}", function_name, perm.statement_id);
                let attrs = serde_json::json!({
                    "id": perm.statement_id,
                    "statement_id": perm.statement_id,
                    "function_name": function_name,
                    "action": perm.action,
                    "principal": perm.principal,
                    "source_arn": perm.source_arn,
                    "source_account": perm.source_account,
                    "principal_org_id": perm.principal_org_id,
                    "event_source_token": perm.event_source_token,
                    "function_url_auth_type": perm.function_url_auth_type,
                    "invoked_via_function_url": perm.invoked_via_function_url,
                });
                results.push(ExtractedResource {
                    name: id,
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
// aws_lambda_event_source_mapping
// ---------------------------------------------------------------------------

/// Resource tags recorded in a Terraform state instance.
///
/// `tags_all` ( the resource's own tags merged with the provider's
/// `default_tags` ) is what the provider sends to the API, so it goes in
/// first; `tags` then overrides. Both may be null, which is why they are read
/// raw rather than through the typed model.
fn tags_from_tf(attrs: &serde_json::Value) -> HashMap<String, String> {
    let mut tags = HashMap::new();
    for key in ["tags_all", "tags"] {
        if let Some(obj) = attrs.get(key).and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }
    }
    tags
}

/// Build the store-and-echo settings of an event source mapping from the raw
/// Terraform attributes whose types fall outside the spec vocabulary.
///
/// `kms_key_arn` and `starting_position_timestamp` come from the typed model
/// and are filled in by the caller; `logging_config` has no Terraform
/// attribute.
///
/// A null attribute means "never supplied" and stays `None`. So do the two
/// placeholders the provider writes for sources that do not report the
/// member at all and that the API itself can never return:
/// `parallelization_factor = 0` and `maximum_record_age_in_seconds = 0`.
fn esm_settings_from_tf(attrs: &serde_json::Value) -> EventSourceMappingSettings {
    let int = |key: &str| {
        attrs
            .get(key)
            .and_then(|v| v.as_i64().or_else(|| v.as_f64().map(|f| f as i64)))
            .and_then(|v| i32::try_from(v).ok())
    };
    let strings = |key: &str| {
        attrs
            .get(key)
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty())
    };
    let scaling_config = attrs
        .get("scaling_config")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.first())
        .map(|block| EventSourceMappingScalingSettings {
            maximum_concurrency: block
                .get("maximum_concurrency")
                .and_then(|v| v.as_i64())
                .and_then(|v| i32::try_from(v).ok()),
        });

    EventSourceMappingSettings {
        bisect_batch_on_function_error: attrs
            .get("bisect_batch_on_function_error")
            .and_then(|v| v.as_bool()),
        function_response_types: strings("function_response_types"),
        maximum_batching_window_in_seconds: int("maximum_batching_window_in_seconds"),
        maximum_record_age_in_seconds: int("maximum_record_age_in_seconds").filter(|v| *v != 0),
        maximum_retry_attempts: int("maximum_retry_attempts"),
        parallelization_factor: int("parallelization_factor").filter(|v| *v >= 1),
        queues: strings("queues"),
        scaling_config,
        topics: strings("topics"),
        tumbling_window_in_seconds: int("tumbling_window_in_seconds"),
        ..Default::default()
    }
}

/// Terraform keeps `starting_position_timestamp` as an RFC 3339 string; the
/// Lambda API ( and the stored settings ) use epoch seconds.
fn parse_rfc3339_epoch_seconds(value: &str) -> Option<f64> {
    chrono::DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.timestamp_millis() as f64 / 1000.0)
}

fn format_epoch_seconds_rfc3339(seconds: f64) -> Option<String> {
    chrono::DateTime::<Utc>::from_timestamp_millis((seconds * 1000.0).round() as i64)
        .map(|dt| dt.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, true))
}

/// Converts `aws_lambda_event_source_mapping` Terraform resources to/from Lambda state.
pub struct AwsLambdaEventSourceMappingConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaEventSourceMappingConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaEventSourceMappingConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_event_source_mapping"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaEventSourceMappingConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaEventSourceMappingTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_lambda_event_source_mapping", e))?;

        // `arn` is derived from the function ARN and the UUID on every read,
        // so the recorded value needs no storage.
        let _ = &model.arn;

        // Store-and-echo settings surfaced by Get/ListEventSourceMappings.
        let mut settings = esm_settings_from_tf(attrs);
        settings.tags = tags_from_tf(attrs);
        settings.kms_key_arn = model.kms_key_arn.filter(|s| !s.is_empty());
        settings.starting_position_timestamp = model
            .starting_position_timestamp
            .as_deref()
            .and_then(parse_rfc3339_epoch_seconds);

        // Nested blocks — stored for round-trip fidelity.
        let destination_config = attrs.get("destination_config").cloned();
        let filter_criteria = attrs.get("filter_criteria").cloned();
        let self_managed_event_source = attrs.get("self_managed_event_source").cloned();
        let source_access_configuration = attrs.get("source_access_configuration").cloned();
        let self_managed_kafka_event_source_config =
            attrs.get("self_managed_kafka_event_source_config").cloned();
        let amazon_managed_kafka_event_source_config = attrs
            .get("amazon_managed_kafka_event_source_config")
            .cloned();
        let document_db_event_source_config = attrs.get("document_db_event_source_config").cloned();
        let metrics_config = attrs.get("metrics_config").cloned();
        let provisioned_poller_config = attrs.get("provisioned_poller_config").cloned();

        let function_name = model.function_name;
        let event_source_arn = model.event_source_arn;

        // Preserve original lookup chain: uuid -> id -> generate.
        let uuid = model
            .uuid
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        let function_arn = format!(
            "arn:aws:lambda:{}:{}:function:{}",
            region, ctx.default_account_id, function_name
        );

        let batch_size = attrs
            .get("batch_size")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32);
        let enabled = attrs
            .get("enabled")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);

        let mut esm_view = EventSourceMappingView {
            uuid: uuid.clone(),
            event_source_arn,
            function_arn,
            batch_size,
            enabled,
            state: "Enabled".to_string(),
            last_modified: Utc::now().to_rfc3339(),
            starting_position: model.starting_position,
            destination_config,
            filter_criteria,
            self_managed_event_source,
            source_access_configuration,
            self_managed_kafka_event_source_config,
            amazon_managed_kafka_event_source_config,
            document_db_event_source_config,
            metrics_config,
            provisioned_poller_config,
            settings,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        // `logging_config` has no Terraform attribute; keep whatever an
        // already-present mapping carries instead of wiping it.
        if let Some(existing) = state_view.event_source_mappings.get(&uuid) {
            esm_view.settings.logging_config = existing.settings.logging_config.clone();
        }
        state_view.event_source_mappings.insert(uuid, esm_view);
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
        for esm in view.event_source_mappings.values() {
            let settings = &esm.settings;
            // Settings that were never supplied fall back to the values the
            // AWS provider records for an untouched mapping.
            let scaling_config = match &settings.scaling_config {
                Some(c) => serde_json::json!([{"maximum_concurrency": c.maximum_concurrency}]),
                None => serde_json::json!([]),
            };
            let mut attrs = serde_json::json!({
                "id": esm.uuid,
                "uuid": esm.uuid,
                "arn": event_source_mapping_arn(&esm.function_arn, &esm.uuid),
                "function_name": esm.function_arn,
                "function_arn": esm.function_arn,
                "event_source_arn": esm.event_source_arn,
                "batch_size": esm.batch_size,
                "enabled": esm.enabled,
                "starting_position": esm.starting_position,
                "state": esm.state,
                "last_modified": esm.last_modified,
                "state_transition_reason": "USER_INITIATED",
                "maximum_retry_attempts": settings.maximum_retry_attempts.unwrap_or(-1),
                "tags": settings.tags,
                "tags_all": settings.tags,
                "bisect_batch_on_function_error": settings
                    .bisect_batch_on_function_error
                    .unwrap_or(false),
                "function_response_types": settings
                    .function_response_types
                    .clone()
                    .unwrap_or_default(),
                "kms_key_arn": settings.kms_key_arn,
                "maximum_batching_window_in_seconds": settings
                    .maximum_batching_window_in_seconds
                    .unwrap_or(0),
                "maximum_record_age_in_seconds": settings
                    .maximum_record_age_in_seconds
                    .unwrap_or(-1),
                "parallelization_factor": settings.parallelization_factor.unwrap_or(1),
                "queues": settings.queues,
                "topics": settings.topics,
                "tumbling_window_in_seconds": settings.tumbling_window_in_seconds.unwrap_or(0),
                "starting_position_timestamp": settings
                    .starting_position_timestamp
                    .and_then(format_epoch_seconds_rfc3339),
                "scaling_config": scaling_config,
            });
            // Emit nested blocks if present
            let obj = attrs.as_object_mut().unwrap();
            for (key, val) in [
                ("destination_config", &esm.destination_config),
                ("filter_criteria", &esm.filter_criteria),
                ("self_managed_event_source", &esm.self_managed_event_source),
                (
                    "source_access_configuration",
                    &esm.source_access_configuration,
                ),
                (
                    "self_managed_kafka_event_source_config",
                    &esm.self_managed_kafka_event_source_config,
                ),
                (
                    "amazon_managed_kafka_event_source_config",
                    &esm.amazon_managed_kafka_event_source_config,
                ),
                (
                    "document_db_event_source_config",
                    &esm.document_db_event_source_config,
                ),
                ("metrics_config", &esm.metrics_config),
                ("provisioned_poller_config", &esm.provisioned_poller_config),
            ] {
                if let Some(v) = val {
                    obj.insert(key.to_string(), v.clone());
                } else {
                    obj.insert(key.to_string(), serde_json::json!([]));
                }
            }
            results.push(ExtractedResource {
                name: esm.uuid.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_code_signing_config
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_code_signing_config` Terraform resources to/from Lambda state.
pub struct AwsLambdaCodeSigningConfigConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaCodeSigningConfigConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaCodeSigningConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_code_signing_config"
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

impl AwsLambdaCodeSigningConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaCodeSigningConfigTfModel =
            serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_lambda_code_signing_config", e))?;

        // Nested blocks not part of the strongly-typed model.
        let _ = attrs.get("allowed_publishers");
        let _ = attrs.get("policies");

        let config_id = model
            .config_id
            .or_else(|| optional_str(attrs, "id"))
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:lambda:{}:{}:code-signing-config:{}",
                region, ctx.default_account_id, config_id
            )
        });

        // Pull signing-profile ARNs from the nested allowed_publishers block.
        let allowed_publishers: Vec<String> = attrs
            .get("allowed_publishers")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("signing_profile_version_arns"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let untrusted_artifact_on_deployment = attrs
            .get("policies")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("untrusted_artifact_on_deployment"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "Warn".to_string());

        let entry = CodeSigningConfigEntry {
            code_signing_config_id: config_id.clone(),
            code_signing_config_arn: arn,
            description: model.description,
            allowed_publishers,
            untrusted_artifact_on_deployment,
            last_modified: Utc::now().to_rfc3339(),
            functions: vec![],
            tags: tags_from_tf(attrs),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.code_signing_configs.insert(config_id, entry);
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
        for entry in view.code_signing_configs.values() {
            let attrs = serde_json::json!({
                "id": entry.code_signing_config_id,
                "config_id": entry.code_signing_config_id,
                "arn": entry.code_signing_config_arn,
                "description": entry.description,
                "last_modified": entry.last_modified,
                "allowed_publishers": [{
                    "signing_profile_version_arns": entry.allowed_publishers,
                }],
                "policies": [{
                    "untrusted_artifact_on_deployment": entry.untrusted_artifact_on_deployment,
                }],
                "tags": entry.tags,
                "tags_all": entry.tags,
            });
            results.push(ExtractedResource {
                name: entry.code_signing_config_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_function_event_invoke_config
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_function_event_invoke_config` Terraform resources to/from Lambda state.
pub struct AwsLambdaFunctionEventInvokeConfigConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaFunctionEventInvokeConfigConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaFunctionEventInvokeConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_function_event_invoke_config"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaFunctionEventInvokeConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaFunctionEventInvokeConfigTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_lambda_function_event_invoke_config", e)
            })?;

        // TF-only / nested blocks not part of the strongly-typed model.
        let _ = attrs.get("destination_config");

        let function_name = model.function_name.clone();
        let function_arn = format!(
            "arn:aws:lambda:{}:{}:function:{}",
            region, ctx.default_account_id, function_name
        );

        // Pull destination ARNs from the nested destination_config block.
        let destination_config = attrs
            .get("destination_config")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first());
        let on_success_destination = destination_config
            .and_then(|d| d.get("on_success"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("destination"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let on_failure_destination = destination_config
            .and_then(|d| d.get("on_failure"))
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|item| item.get("destination"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let feic_view = FunctionEventInvokeConfigView {
            function_name: function_name.clone(),
            function_arn,
            maximum_retry_attempts: Some(model.maximum_retry_attempts as i32),
            maximum_event_age_in_seconds: Some(model.maximum_event_age_in_seconds as i32),
            on_success_destination,
            on_failure_destination,
            last_modified: Utc::now().to_rfc3339(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .function_event_invoke_configs
            .insert(function_name, feic_view);
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
        for feic in view.function_event_invoke_configs.values() {
            let mut destination_config = serde_json::json!([]);
            if feic.on_success_destination.is_some() || feic.on_failure_destination.is_some() {
                let mut entry = serde_json::Map::new();
                if let Some(arn) = &feic.on_success_destination {
                    entry.insert(
                        "on_success".to_string(),
                        serde_json::json!([{ "destination": arn }]),
                    );
                }
                if let Some(arn) = &feic.on_failure_destination {
                    entry.insert(
                        "on_failure".to_string(),
                        serde_json::json!([{ "destination": arn }]),
                    );
                }
                destination_config =
                    serde_json::Value::Array(vec![serde_json::Value::Object(entry)]);
            }
            let attrs = serde_json::json!({
                "id": feic.function_name,
                "function_name": feic.function_name,
                "maximum_event_age_in_seconds": feic.maximum_event_age_in_seconds.unwrap_or(0),
                "maximum_retry_attempts": feic.maximum_retry_attempts.unwrap_or(2),
                "destination_config": destination_config,
            });
            results.push(ExtractedResource {
                name: feic.function_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_function_recursion_config
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_function_recursion_config` Terraform resources to/from Lambda state.
pub struct AwsLambdaFunctionRecursionConfigConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaFunctionRecursionConfigConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaFunctionRecursionConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_function_recursion_config"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaFunctionRecursionConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaFunctionRecursionConfigTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_lambda_function_recursion_config", e)
            })?;

        let function_name = model.function_name.clone();
        let cfg = FunctionRecursionConfig {
            recursive_loop: model.recursive_loop,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .function_recursion_configs
            .insert(function_name, cfg);
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
        for (function_name, cfg) in &view.function_recursion_configs {
            let attrs = serde_json::json!({
                "id": function_name,
                "function_name": function_name,
                "recursive_loop": cfg.recursive_loop,
            });
            results.push(ExtractedResource {
                name: function_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_function_url
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_function_url` Terraform resources to/from Lambda state.
pub struct AwsLambdaFunctionUrlConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaFunctionUrlConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaFunctionUrlConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_function_url"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaFunctionUrlConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaFunctionUrlTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_function_url", e))?;

        // TF-only / nested blocks not part of the strongly-typed model.
        let _ = attrs.get("cors");
        let _ = attrs.get("url_id");

        let function_name = model.function_name.clone();
        let function_arn = model.function_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:lambda:{}:{}:function:{}",
                region, ctx.default_account_id, function_name
            )
        });
        let function_url = model
            .function_url
            .unwrap_or_else(|| format!("https://placeholder.lambda-url.{}.on.aws/", region));

        let now = Utc::now().to_rfc3339();
        let fuc_view = FunctionUrlConfigView {
            function_name: function_name.clone(),
            function_arn,
            function_url,
            auth_type: model.authorization_type,
            cors: None,
            invoke_mode: model.invoke_mode,
            creation_time: now.clone(),
            last_modified_time: now,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .function_url_configs
            .insert(function_name, fuc_view);
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
        for fuc in view.function_url_configs.values() {
            let attrs = serde_json::json!({
                "id": fuc.function_name,
                "function_name": fuc.function_name,
                "function_arn": fuc.function_arn,
                "function_url": fuc.function_url,
                "authorization_type": fuc.auth_type,
                "invoke_mode": fuc.invoke_mode,
                "url_id": "",
                "creation_time": fuc.creation_time,
                "last_modified_time": fuc.last_modified_time,
                "cors": [],
            });
            results.push(ExtractedResource {
                name: fuc.function_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_invocation
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_invocation` Terraform resources.
///
/// Lambda invocations are ephemeral side-effects with no durable state in
/// winterbaume, so injection only validates the payload (including the
/// `qualifier` against the current Lambda state) and extraction returns
/// nothing. The TF resource itself is a one-shot trigger that does not own
/// a steady-state representation.
pub struct AwsLambdaInvocationConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaInvocationConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaInvocationConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_invocation"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        // Alias is listed alongside function so that an invocation pinned to
        // `qualifier = aws_lambda_alias.<x>.name` can be resolved during
        // injection — without this the alias may not be in state yet when
        // we call resolve_qualifier below.
        vec!["aws_lambda_function", "aws_lambda_alias"]
    }

    fn inject<'a>(
        &'a self,
        instance: &'a ResourceInstance,
        ctx: &'a ConversionContext,
    ) -> Pin<Box<dyn Future<Output = Result<ConversionResult, ConversionError>> + Send + 'a>> {
        Box::pin(async move {
            let attrs = &instance.attributes;
            let region = extract_region(attrs, &ctx.default_region);
            let model: lambda_gen::LambdaInvocationTfModel = serde_json::from_value(attrs.clone())
                .map_err(|e| classify_deserialize_error("aws_lambda_invocation", e))?;

            // Validate the qualifier against the current Lambda state.  Real
            // `terraform apply` would surface an unresolved alias/version as
            // ResourceNotFoundException from the Invoke API; mirror that
            // here so a stale tfstate fails injection loudly rather than
            // appearing to succeed.
            self.service
                .resolve_qualifier(
                    &ctx.default_account_id,
                    &region,
                    &model.function_name,
                    model.qualifier.as_deref(),
                )
                .await
                .map_err(|e| ConversionError::InvalidAttribute {
                    resource_type: "aws_lambda_invocation".to_string(),
                    attribute: "qualifier".to_string(),
                    detail: format!(
                        "function {:?} qualifier {:?}: {}",
                        model.function_name,
                        model.qualifier.as_deref().unwrap_or("$LATEST"),
                        e,
                    ),
                })?;

            Ok(ConversionResult {
                region,
                warnings: vec![
                    "aws_lambda_invocation is a one-shot trigger; no durable state recorded"
                        .to_string(),
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
// aws_lambda_layer_version
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_layer_version` Terraform resources to/from Lambda state.
pub struct AwsLambdaLayerVersionConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaLayerVersionConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaLayerVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_layer_version"
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

impl AwsLambdaLayerVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaLayerVersionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_layer_version", e))?;

        // TF-only fields not part of the strongly-typed model.
        let _ = attrs.get("filename");
        let _ = attrs.get("s3_bucket");
        let _ = attrs.get("s3_key");
        let _ = attrs.get("s3_object_version");
        let _ = attrs.get("source_code_hash");
        let _ = attrs.get("skip_destroy");

        let compatible_runtimes: Vec<String> = attrs
            .get("compatible_runtimes")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let compatible_architectures: Vec<String> = attrs
            .get("compatible_architectures")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let layer_name = model.layer_name.clone();
        let version_num = model
            .version
            .as_deref()
            .and_then(|s| s.parse::<i64>().ok())
            .unwrap_or(1);

        let layer_arn = model.layer_arn.unwrap_or_else(|| {
            format!(
                "arn:aws:lambda:{}:{}:layer:{}",
                region, ctx.default_account_id, layer_name
            )
        });
        let layer_version_arn = model
            .arn
            .unwrap_or_else(|| format!("{}:{}", layer_arn, version_num));

        let lv_view = LayerVersionView {
            layer_name: layer_name.clone(),
            version: version_num,
            layer_arn,
            layer_version_arn,
            description: model.description.unwrap_or_default(),
            compatible_runtimes,
            compatible_architectures,
            license_info: model.license_info,
            created_date: Utc::now().to_rfc3339(),
            code_sha256: attrs
                .get("source_code_hash")
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string(),
            code_size: attrs
                .get("source_code_size")
                .and_then(|v| v.as_i64())
                .unwrap_or(0),
            permissions: vec![],
            policy_revision_id: String::new(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let versions = state_view.layers.entry(layer_name.clone()).or_default();
        versions.retain(|v| v.version != version_num);
        versions.push(lv_view);
        let next = state_view
            .layer_next_version
            .entry(layer_name)
            .or_insert(version_num + 1);
        if *next <= version_num {
            *next = version_num + 1;
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
        for versions in view.layers.values() {
            for lv in versions {
                let attrs = serde_json::json!({
                    "id": lv.layer_version_arn,
                    "layer_name": lv.layer_name,
                    "description": lv.description,
                    "license_info": lv.license_info,
                    "arn": lv.layer_version_arn,
                    "layer_arn": lv.layer_arn,
                    "version": lv.version.to_string(),
                    "compatible_runtimes": lv.compatible_runtimes,
                    "compatible_architectures": lv.compatible_architectures,
                    "created_date": lv.created_date,
                    "source_code_hash": lv.code_sha256,
                    "source_code_size": lv.code_size,
                    "signing_job_arn": "",
                    "signing_profile_version_arn": "",
                    "skip_destroy": false,
                });
                results.push(ExtractedResource {
                    name: lv.layer_version_arn.clone(),
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
// aws_lambda_layer_version_permission
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_layer_version_permission` Terraform resources to/from Lambda state.
pub struct AwsLambdaLayerVersionPermissionConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaLayerVersionPermissionConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaLayerVersionPermissionConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_layer_version_permission"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_layer_version"]
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

impl AwsLambdaLayerVersionPermissionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaLayerVersionPermissionTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_lambda_layer_version_permission", e)
            })?;

        let perm = LayerPermissionView {
            statement_id: model.statement_id.clone(),
            action: model.action,
            principal: model.principal,
            organization_id: model.organization_id,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let mut warnings = vec![];
        if let Some(versions) = state_view.layers.get_mut(&model.layer_name) {
            if let Some(lv) = versions
                .iter_mut()
                .find(|v| v.version == model.version_number)
            {
                lv.permissions
                    .retain(|p| p.statement_id != perm.statement_id);
                lv.permissions.push(perm);
                lv.policy_revision_id = uuid::Uuid::new_v4().to_string();
            } else {
                warnings.push(format!(
                    "layer version {}:{} not found; permission skipped",
                    model.layer_name, model.version_number
                ));
            }
        } else {
            warnings.push(format!(
                "layer {} not found; permission skipped",
                model.layer_name
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
        for versions in view.layers.values() {
            for lv in versions {
                for perm in &lv.permissions {
                    let id = format!("{},{},{}", lv.layer_name, lv.version, perm.statement_id);
                    let attrs = serde_json::json!({
                        "id": id,
                        "layer_name": lv.layer_name,
                        "version_number": lv.version,
                        "statement_id": perm.statement_id,
                        "action": perm.action,
                        "principal": perm.principal,
                        "organization_id": perm.organization_id,
                        "revision_id": "",
                    });
                    results.push(ExtractedResource {
                        name: id,
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
// aws_lambda_provisioned_concurrency_config
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_provisioned_concurrency_config` Terraform resources to/from Lambda state.
pub struct AwsLambdaProvisionedConcurrencyConfigConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaProvisionedConcurrencyConfigConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaProvisionedConcurrencyConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_provisioned_concurrency_config"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function", "aws_lambda_alias"]
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

impl AwsLambdaProvisionedConcurrencyConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaProvisionedConcurrencyConfigTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_lambda_provisioned_concurrency_config", e)
            })?;

        let function_name = model.function_name.clone();
        let qualifier = model.qualifier.clone();
        let function_arn = format!(
            "arn:aws:lambda:{}:{}:function:{}:{}",
            region, ctx.default_account_id, function_name, qualifier
        );
        let key = format!("{}:{}", function_name, qualifier);

        let cfg = ProvisionedConcurrencyConfig {
            function_arn,
            requested: model.provisioned_concurrent_executions as i32,
            last_modified: Utc::now().to_rfc3339(),
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view.provisioned_concurrency.insert(key, cfg);
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
        for (key, cfg) in &view.provisioned_concurrency {
            let mut parts = key.splitn(2, ':');
            let function_name = parts.next().unwrap_or_default().to_string();
            let qualifier = parts.next().unwrap_or_default().to_string();
            let attrs = serde_json::json!({
                "id": key,
                "function_name": function_name,
                "qualifier": qualifier,
                "provisioned_concurrent_executions": cfg.requested,
                "status": "READY",
                "last_modified": cfg.last_modified,
            });
            results.push(ExtractedResource {
                name: key.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_lambda_runtime_management_config
// ---------------------------------------------------------------------------

/// Converts `aws_lambda_runtime_management_config` Terraform resources to/from Lambda state.
pub struct AwsLambdaRuntimeManagementConfigConverter {
    service: Arc<LambdaService>,
}

impl AwsLambdaRuntimeManagementConfigConverter {
    pub fn new(service: Arc<LambdaService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsLambdaRuntimeManagementConfigConverter {
    fn resource_type(&self) -> &str {
        "aws_lambda_runtime_management_config"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_lambda_function"]
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

impl AwsLambdaRuntimeManagementConfigConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);
        let model: lambda_gen::LambdaRuntimeManagementConfigTfModel =
            serde_json::from_value(attrs.clone()).map_err(|e| {
                classify_deserialize_error("aws_lambda_runtime_management_config", e)
            })?;

        let function_name = model.function_name.clone();
        let cfg = RuntimeManagementConfig {
            update_runtime_on: model.update_runtime_on,
            runtime_version_arn: model.runtime_version_arn,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .runtime_management_configs
            .insert(function_name, cfg);
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
        for (function_name, cfg) in &view.runtime_management_configs {
            let attrs = serde_json::json!({
                "id": function_name,
                "function_name": function_name,
                "update_runtime_on": cfg.update_runtime_on,
                "runtime_version_arn": cfg.runtime_version_arn,
            });
            results.push(ExtractedResource {
                name: function_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn minimal_lambda_state_view(_account_id: &str, _region: &str) -> LambdaStateView {
    LambdaStateView {
        functions: HashMap::new(),
        aliases: HashMap::new(),
        event_source_mappings: HashMap::new(),
        layers: HashMap::new(),
        layer_next_version: HashMap::new(),
        function_url_configs: HashMap::new(),
        function_permissions: HashMap::new(),
        function_event_invoke_configs: HashMap::new(),
        code_signing_configs: HashMap::new(),
        provisioned_concurrency: HashMap::new(),
        capacity_providers: HashMap::new(),
        function_recursion_configs: HashMap::new(),
        function_scaling_configs: HashMap::new(),
        runtime_management_configs: HashMap::new(),
        csc_id_counter: 0,
        cp_id_counter: 0,
        ..Default::default()
    }
}
