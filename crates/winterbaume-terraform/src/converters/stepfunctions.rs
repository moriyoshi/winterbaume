//! Terraform converter for `aws_sfn_state_machine` resources.
//!
//! `StateMachineTfModel` is generated from `specs/stepfunctions.toml`.
//! The ARN template (with the `id` fallback chain), the merged
//! `tags` / `tags_all` projection into `Vec<TagView>`, and the
//! `logging_configuration` / `tracing_configuration` /
//! `encryption_configuration` nested-block parsing are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_sfn::SfnService;
use winterbaume_sfn::views::{
    CloudWatchLogsLogGroupView, EncryptionConfigurationView, LogDestinationView,
    LoggingConfigurationView, StateMachineView, StepfunctionsStateView, TagView,
    TracingConfigurationView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::stepfunctions as stepfunctions_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Converts `aws_sfn_state_machine` Terraform resources to/from Step Functions state.
pub struct AwsSfnStateMachineConverter {
    service: Arc<SfnService>,
}

impl AwsSfnStateMachineConverter {
    pub fn new(service: Arc<SfnService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSfnStateMachineConverter {
    fn resource_type(&self) -> &str {
        "aws_sfn_state_machine"
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

impl AwsSfnStateMachineConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: stepfunctions_gen::StateMachineTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_sfn_state_machine", e))?;

        let attrs = &instance.attributes;
        let name = model.name.clone();
        let definition = model.definition;
        let role_arn = model.role_arn;

        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            format!(
                "arn:aws:states:{}:{}:stateMachine:{}",
                region, ctx.default_account_id, name
            )
        });

        let sm_type = model.r#type.unwrap_or_else(|| "STANDARD".to_string());

        // Tags from tags / tags_all
        let tags: Vec<TagView> = {
            let mut map: HashMap<String, String> = HashMap::new();
            if let Some(obj) = attrs.get("tags_all").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            if let Some(obj) = attrs.get("tags").and_then(|v| v.as_object()) {
                for (k, v) in obj {
                    if let Some(s) = v.as_str() {
                        map.insert(k.clone(), s.to_string());
                    }
                }
            }
            map.into_iter()
                .map(|(k, v)| TagView { key: k, value: v })
                .collect()
        };

        // Parse logging_configuration block
        let logging_configuration = attrs
            .get("logging_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|block| {
                let level = block
                    .get("level")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let include_execution_data = block
                    .get("include_execution_data")
                    .and_then(|v| v.as_bool());
                // In AWS provider v5.x+, log_destination is a simple string ARN
                let destinations = block
                    .get("log_destination")
                    .and_then(|v| v.as_str())
                    .map(|arn| {
                        vec![LogDestinationView {
                            cloud_watch_logs_log_group: Some(CloudWatchLogsLogGroupView {
                                log_group_arn: Some(arn.to_string()),
                            }),
                        }]
                    })
                    .unwrap_or_default();
                LoggingConfigurationView {
                    level,
                    include_execution_data,
                    destinations,
                }
            });

        // Parse tracing_configuration block
        let tracing_configuration = attrs
            .get("tracing_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|block| {
                let enabled = block
                    .get("enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
                TracingConfigurationView { enabled }
            });

        // Parse encryption_configuration block
        let encryption_configuration = attrs
            .get("encryption_configuration")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .map(|block| {
                let enc_type = block
                    .get("type")
                    .and_then(|v| v.as_str())
                    .unwrap_or("AWS_OWNED_KEY")
                    .to_string();
                let kms_key_id = block
                    .get("kms_key_id")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let kms_data_key_reuse_period_seconds = block
                    .get("kms_data_key_reuse_period_seconds")
                    .and_then(|v| v.as_i64());
                EncryptionConfigurationView {
                    kms_key_id,
                    kms_data_key_reuse_period_seconds,
                    r#type: enc_type,
                }
            });

        let sm_view = StateMachineView {
            name,
            arn: arn.clone(),
            definition,
            role_arn,
            status: "ACTIVE".to_string(),
            creation_date: Utc::now().to_rfc3339(),
            r#type: sm_type,
            tags,
            logging_configuration,
            tracing_configuration,
            encryption_configuration,
        };

        let mut state_view = StepfunctionsStateView::default();
        state_view.state_machines.insert(arn, sm_view);
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
        for sm in view.state_machines.values() {
            let tags: HashMap<String, String> = sm
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();

            let logging_configuration = sm.logging_configuration.as_ref().map(|lc| {
                serde_json::json!([{
                    "level": lc.level,
                    "include_execution_data": lc.include_execution_data,
                    "log_destination": lc.destinations.first()
                        .and_then(|d| d.cloud_watch_logs_log_group.as_ref())
                        .and_then(|cw| cw.log_group_arn.as_deref())
                        .unwrap_or(""),
                }])
            });

            let tracing_configuration = sm
                .tracing_configuration
                .as_ref()
                .map(|tc| serde_json::json!([{"enabled": tc.enabled}]));

            let encryption_configuration = sm.encryption_configuration.as_ref().map(|ec| {
                serde_json::json!([{
                    "type": ec.r#type,
                    "kms_key_id": ec.kms_key_id,
                    "kms_data_key_reuse_period_seconds": ec.kms_data_key_reuse_period_seconds,
                }])
            });

            let mut attrs = serde_json::json!({
                "id": sm.arn,
                "arn": sm.arn,
                "name": sm.name,
                "definition": sm.definition,
                "role_arn": sm.role_arn,
                "type": sm.r#type,
                "status": sm.status,
                "tags": tags,
                "tags_all": tags,
                "revision_id": null,
                "creation_date": "",
            });

            if let Some(lc) = logging_configuration {
                attrs["logging_configuration"] = lc;
            }
            if let Some(tc) = tracing_configuration {
                attrs["tracing_configuration"] = tc;
            }
            if let Some(ec) = encryption_configuration {
                attrs["encryption_configuration"] = ec;
            }

            results.push(ExtractedResource {
                name: sm.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
