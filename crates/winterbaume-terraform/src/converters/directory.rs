//! Terraform converter for Directory Service resources.
//!
//! `DirectoryTfModel` is generated from `specs/directory.toml`. The
//! synthesised `directory_id` (`d-<uuid[..10]>`), the alias / access_url
//! defaults, the DateTime<Utc>-typed `launch_time` /
//! `stage_last_updated_date_time` parses, the `dns_ip_addresses`
//! Vec<String>, and the nested `vpc_settings` / `connect_settings`
//! blocks are wired up here.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_directory::DirectoryService;
use winterbaume_directory::views::{
    DirectoryConnectSettingsView, DirectoryView, DirectoryVpcSettingsView, DsStateView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::directory as directory_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_directory_service_directory
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_directory` Terraform resources to/from Directory Service state.
pub struct AwsDirectoryServiceDirectoryConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceDirectoryConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceDirectoryConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_directory"
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

impl AwsDirectoryServiceDirectoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::DirectoryTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_directory_service_directory", e))?;

        let attrs = &instance.attributes;

        let directory_id = model.id.or(model.directory_id).unwrap_or_else(|| {
            format!(
                "d-{}",
                &uuid::Uuid::new_v4().to_string().replace('-', "")[..10]
            )
        });
        let size = model.size.unwrap_or_else(|| "Small".to_string());
        let directory_type = model
            .directory_type
            .unwrap_or_else(|| "SimpleAD".to_string());
        let alias = model.alias.unwrap_or_else(|| directory_id.clone());
        let access_url = model
            .access_url
            .unwrap_or_else(|| format!("{}.awsapps.com", alias));
        let stage = model.stage.unwrap_or_else(|| "Active".to_string());
        let now = chrono::Utc::now();
        let launch_time = attrs
            .get("launch_time")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or(now);
        let stage_last_updated = attrs
            .get("stage_last_updated_date_time")
            .and_then(|v| v.as_str())
            .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&chrono::Utc))
            .unwrap_or(now);
        let dns_ip_addrs = attrs
            .get("dns_ip_addresses")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();
        let ssoid_enabled = model.enable_sso;

        // Parse vpc_settings
        let vpc_settings = attrs.get("vpc_settings").and_then(|v| {
            let obj = if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            };
            obj.map(|vs| {
                let vpc_id = vs
                    .get("vpc_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let subnet_ids = vs
                    .get("subnet_ids")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let security_group_id = vs
                    .get("security_group_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let availability_zones = vs
                    .get("availability_zones")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                DirectoryVpcSettingsView {
                    vpc_id,
                    subnet_ids,
                    security_group_id,
                    availability_zones,
                }
            })
        });

        // Parse connect_settings
        let connect_settings = attrs.get("connect_settings").and_then(|v| {
            let obj = if v.is_array() {
                v.as_array().and_then(|a| a.first())
            } else {
                Some(v)
            };
            obj.map(|cs| {
                let vpc_id = cs
                    .get("vpc_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let subnet_ids = cs
                    .get("subnet_ids")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let customer_dns_ips = cs
                    .get("customer_dns_ips")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let customer_user_name = cs
                    .get("customer_username")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let security_group_id = cs
                    .get("security_group_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default()
                    .to_string();
                let availability_zones = cs
                    .get("availability_zones")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let connect_ips = cs
                    .get("connect_ips")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                DirectoryConnectSettingsView {
                    vpc_id,
                    subnet_ids,
                    customer_dns_ips,
                    customer_user_name,
                    security_group_id,
                    availability_zones,
                    connect_ips,
                }
            })
        });

        let dir_view = DirectoryView {
            directory_id: directory_id.clone(),
            name: model.name,
            short_name: model.short_name,
            description: model.description,
            size,
            directory_type,
            alias,
            access_url,
            stage,
            launch_time,
            stage_last_updated_date_time: stage_last_updated,
            dns_ip_addrs,
            vpc_settings,
            connect_settings,
            ssoid_enabled,
        };

        let mut state_view = DsStateView::default();
        state_view.directories.insert(directory_id, dir_view);
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
        for dir in view.directories.values() {
            let vpc_settings_json = dir.vpc_settings.as_ref().map(|vs| {
                serde_json::json!({
                    "vpc_id": vs.vpc_id,
                    "subnet_ids": vs.subnet_ids,
                    "security_group_id": vs.security_group_id,
                    "availability_zones": vs.availability_zones,
                })
            });
            let connect_settings_json = dir.connect_settings.as_ref().map(|cs| {
                serde_json::json!({
                    "vpc_id": cs.vpc_id,
                    "subnet_ids": cs.subnet_ids,
                    "customer_dns_ips": cs.customer_dns_ips,
                    "customer_username": cs.customer_user_name,
                    "security_group_id": cs.security_group_id,
                    "availability_zones": cs.availability_zones,
                    "connect_ips": cs.connect_ips,
                })
            });
            let attrs = serde_json::json!({
                "id": dir.directory_id,
                "directory_id": dir.directory_id,
                "name": dir.name,
                "short_name": dir.short_name,
                "description": dir.description,
                "size": dir.size,
                "type": dir.directory_type,
                "alias": dir.alias,
                "access_url": dir.access_url,
                "stage": dir.stage,
                "launch_time": dir.launch_time.to_rfc3339(),
                "stage_last_updated_date_time": dir.stage_last_updated_date_time.to_rfc3339(),
                "dns_ip_addresses": dir.dns_ip_addrs,
                "vpc_settings": vpc_settings_json,
                "connect_settings": connect_settings_json,
                "enable_sso": dir.ssoid_enabled,
            });
            results.push(ExtractedResource {
                name: dir.directory_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
