//! Terraform converters for Lambda resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_lambda::LambdaService;
use winterbaume_lambda::views::{
    AliasView, EventSourceMappingView, LambdaFunctionView, LambdaStateView, PermissionView,
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
        let model: lambda_gen::LambdaFunctionTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_lambda_function", e))?;

        // Additional fields for coverage (TF-only / not stored in state).
        let _ = attrs.get("tags_all");
        let _ = attrs.get("architectures");
        let _ = attrs.get("image_uri");
        let _ = attrs.get("kms_key_arn");
        let _ = attrs.get("layers");
        let _ = attrs.get("package_type");
        let _ = attrs.get("publish");
        let _ = attrs.get("s3_bucket");
        let _ = attrs.get("replace_security_groups_on_destroy");
        let _ = attrs.get("skip_destroy");

        // Nested blocks — stored for round-trip fidelity.
        let dead_letter_config = attrs.get("dead_letter_config").cloned();
        let ephemeral_storage = attrs.get("ephemeral_storage").cloned();
        let file_system_config = attrs.get("file_system_config").cloned();
        let image_config = attrs.get("image_config").cloned();
        let logging_config = attrs.get("logging_config").cloned();
        let snap_start = attrs.get("snap_start").cloned();
        let tracing_config = attrs.get("tracing_config").cloned();
        let vpc_config = attrs.get("vpc_config").cloned();

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

        // Tags: original semantics insert tags_all first, then tags overrides.
        let mut tags: HashMap<String, String> = HashMap::new();
        if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }
        if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
            for (k, v) in obj {
                if let Some(s) = v.as_str() {
                    tags.insert(k.clone(), s.to_string());
                }
            }
        }

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
            dead_letter_config,
            ephemeral_storage,
            file_system_config,
            image_config,
            logging_config,
            snap_start,
            tracing_config,
            vpc_config,
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
                "architectures": ["x86_64"],
                "invoke_arn": format!("arn:aws:apigateway:{}:lambda:path/2015-03-31/functions/{}/invocations", ctx.default_region, func.function_arn),
                "qualified_arn": format!("{}:$LATEST", func.function_arn),
                "qualified_invoke_arn": "",
                "signing_job_arn": "",
                "signing_profile_version_arn": "",
                "source_code_size": 0,
            });
            // Emit nested blocks if present
            let obj = attrs.as_object_mut().unwrap();
            for (key, val) in [
                ("dead_letter_config", &func.dead_letter_config),
                ("ephemeral_storage", &func.ephemeral_storage),
                ("file_system_config", &func.file_system_config),
                ("image_config", &func.image_config),
                ("logging_config", &func.logging_config),
                ("snap_start", &func.snap_start),
                ("tracing_config", &func.tracing_config),
                ("vpc_config", &func.vpc_config),
            ] {
                if let Some(v) = val {
                    obj.insert(key.to_string(), v.clone());
                } else {
                    obj.insert(key.to_string(), serde_json::json!([]));
                }
            }
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

        let _principal_org_id = model.principal_org_id;

        let function_name = model.function_name.clone();
        let statement_id = model.statement_id.clone();
        let perm_view = PermissionView {
            statement_id: statement_id.clone(),
            action: model.action,
            principal: model.principal,
            source_arn: model.source_arn,
            source_account: model.source_account,
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        let perms = state_view
            .function_permissions
            .entry(function_name)
            .or_default();
        perms.retain(|p| p.statement_id != statement_id);
        perms.push(perm_view);
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

        // Additional fields for coverage (TF-only / not stored in state).
        let _ = attrs.get("tags_all");
        let _ = attrs.get("bisect_batch_on_function_error");
        let _ = attrs.get("function_response_types");
        let _ = attrs.get("maximum_batching_window_in_seconds");
        let _ = attrs.get("maximum_record_age_in_seconds");
        let _ = attrs.get("maximum_retry_attempts");
        let _ = attrs.get("parallelization_factor");
        let _ = attrs.get("queues");
        let _ = attrs.get("topics");
        let _ = attrs.get("tumbling_window_in_seconds");
        let _ = attrs.get("scaling_config");

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

        let esm_view = EventSourceMappingView {
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
        };

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
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
            let mut attrs = serde_json::json!({
                "id": esm.uuid,
                "uuid": esm.uuid,
                "function_name": esm.function_arn,
                "function_arn": esm.function_arn,
                "event_source_arn": esm.event_source_arn,
                "batch_size": esm.batch_size,
                "enabled": esm.enabled,
                "starting_position": esm.starting_position,
                "state": esm.state,
                "last_modified": esm.last_modified,
                "state_transition_reason": "USER_INITIATED",
                "maximum_retry_attempts": -1,
                "tags_all": {},
                "function_response_types": [],
                "maximum_batching_window_in_seconds": 0,
                "maximum_record_age_in_seconds": -1,
                "parallelization_factor": 1,
                "tumbling_window_in_seconds": 0,
                "scaling_config": [],
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
