//! Terraform converter for Connect resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_connect::ConnectService;
use winterbaume_connect::views::{ConnectInstanceView, ConnectStateView};
use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_bool, optional_str};

// ---------------------------------------------------------------------------
// aws_connect_instance
// ---------------------------------------------------------------------------

/// Converts `aws_connect_instance` Terraform resources to/from Connect state.
pub struct AwsConnectInstanceConverter {
    service: Arc<ConnectService>,
}

impl AwsConnectInstanceConverter {
    pub fn new(service: Arc<ConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsConnectInstanceConverter {
    fn resource_type(&self) -> &str {
        "aws_connect_instance"
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

impl AwsConnectInstanceConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let id = optional_str(attrs, "id").unwrap_or_default();
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:connect:{}:{}:instance/{}",
                region, ctx.default_account_id, id
            )
        });
        let identity_management_type = optional_str(attrs, "identity_management_type")
            .unwrap_or_else(|| "CONNECT_MANAGED".to_string());
        let instance_alias = optional_str(attrs, "instance_alias");
        let instance_status = optional_str(attrs, "status").unwrap_or_else(|| "ACTIVE".to_string());
        let created_time =
            optional_str(attrs, "created_time").unwrap_or_else(|| "1970-01-01T00:00:00Z".into());
        let inbound_calls_enabled = optional_bool(attrs, "inbound_calls_enabled").unwrap_or(true);
        let outbound_calls_enabled = optional_bool(attrs, "outbound_calls_enabled").unwrap_or(true);
        let _contact_flow_logs_enabled =
            optional_bool(attrs, "contact_flow_logs_enabled").unwrap_or(false);
        let _early_media_enabled = optional_bool(attrs, "early_media_enabled").unwrap_or(true);

        let inst_view = ConnectInstanceView {
            id: id.clone(),
            arn,
            identity_management_type,
            instance_alias,
            instance_status,
            created_time,
            inbound_calls_enabled,
            outbound_calls_enabled,
            tags: extract_tags(attrs),
        };

        let state_view = ConnectStateView {
            instances: {
                let mut m = HashMap::new();
                m.insert(id, inst_view);
                m
            },
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
        for inst in view.instances.values() {
            let attrs = serde_json::json!({
                "id": inst.id,
                "arn": inst.arn,
                "identity_management_type": inst.identity_management_type,
                "instance_alias": inst.instance_alias,
                "status": inst.instance_status,
                "created_time": inst.created_time,
                "inbound_calls_enabled": inst.inbound_calls_enabled,
                "outbound_calls_enabled": inst.outbound_calls_enabled,
                "contact_flow_logs_enabled": false,
                "early_media_enabled": true,
                "tags": inst.tags,
            });
            results.push(ExtractedResource {
                name: inst
                    .instance_alias
                    .clone()
                    .unwrap_or_else(|| inst.id.clone()),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
