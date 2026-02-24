//! Terraform converter for Direct Connect resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_directconnect::DirectConnectService;
use winterbaume_directconnect::views::{ConnectionView, DirectConnectStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, extract_tags, optional_i64, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_dx_connection
// ---------------------------------------------------------------------------

/// Converts `aws_dx_connection` Terraform resources to/from Direct Connect state.
pub struct AwsDxConnectionConverter {
    service: Arc<DirectConnectService>,
}

impl AwsDxConnectionConverter {
    pub fn new(service: Arc<DirectConnectService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDxConnectionConverter {
    fn resource_type(&self) -> &str {
        "aws_dx_connection"
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

impl AwsDxConnectionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let name = require_str(attrs, "name", "aws_dx_connection")?;
        let region = extract_region(attrs, &ctx.default_region);

        let connection_id = optional_str(attrs, "id").unwrap_or_default();
        let bandwidth = optional_str(attrs, "bandwidth").unwrap_or_else(|| "1Gbps".to_string());
        let location = optional_str(attrs, "location").unwrap_or_default();
        let vlan = optional_i64(attrs, "vlan").unwrap_or(0) as i32;
        let partner_name = optional_str(attrs, "partner_name");
        let owner_account = optional_str(attrs, "owner_account_id")
            .unwrap_or_else(|| ctx.default_account_id.clone());
        let connection_state =
            optional_str(attrs, "connection_state").unwrap_or_else(|| "available".to_string());
        let _tags_all = attrs.get("tags_all");
        let _provider_name = optional_str(attrs, "provider_name");
        let _request_macsec = attrs.get("request_macsec");
        let _skip_destroy = attrs.get("skip_destroy");
        let _ = attrs.get("encryption_mode");

        let conn_view = ConnectionView {
            connection_id: connection_id.clone(),
            connection_name: name.to_string(),
            connection_state,
            region: region.clone(),
            location,
            bandwidth,
            owner_account,
            vlan,
            partner_name,
            loa_issue_time: None,
            tags: extract_tags(attrs),
        };

        let state_view = DirectConnectStateView {
            connections: {
                let mut m = HashMap::new();
                m.insert(connection_id, conn_view);
                m
            },
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
        for c in view.connections.values() {
            let attrs = serde_json::json!({
                "id": c.connection_id,
                "name": c.connection_name,
                "bandwidth": c.bandwidth,
                "location": c.location,
                "vlan": c.vlan,
                "partner_name": c.partner_name,
                "owner_account_id": c.owner_account,
                "connection_state": c.connection_state,
                "tags": c.tags,
                "tags_all": c.tags,
                "has_logical_redundancy": "unknown",
                "aws_device": "",
            });
            results.push(ExtractedResource {
                name: c.connection_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
