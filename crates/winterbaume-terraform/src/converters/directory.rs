//! Terraform converter for Directory Service resources.
//!
//! `DirectoryTfModel` is generated from `specs/directory.toml`. The
//! synthesised `directory_id` (`d-<uuid[..10]>`), the alias / access_url
//! defaults, the DateTime<Utc>-typed `launch_time` /
//! `stage_last_updated_date_time` parses, the `dns_ip_addresses`
//! Vec<String>, and the nested `vpc_settings` / `connect_settings`
//! blocks are wired up here.
//!
//! The seven sibling resources (`aws_directory_service_conditional_forwarder`,
//! `aws_directory_service_log_subscription`,
//! `aws_directory_service_radius_settings`,
//! `aws_directory_service_region`,
//! `aws_directory_service_shared_directory`,
//! `aws_directory_service_shared_directory_accepter`,
//! `aws_directory_service_trust`) have no representation in `DsStateView`
//! (which only persists top-level directories). Their converters
//! deserialize the model for round-trip validation, ensure the parent
//! directory entry exists so dependent resources resolve, and emit a
//! warning that the sibling-specific payload is dropped. Extract returns
//! an empty list — the sibling resource has no view-side counterpart.

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

/// Build a placeholder `DirectoryView` for a directory referenced by a
/// sibling resource (conditional forwarder, log subscription, trust, etc.)
/// when the parent `aws_directory_service_directory` is not part of the
/// terraform state being injected. Mirrors the synthesis applied by
/// `DsState::create_directory` so sibling-only plans still round-trip.
fn placeholder_directory(directory_id: &str) -> DirectoryView {
    let now = chrono::Utc::now();
    DirectoryView {
        directory_id: directory_id.to_string(),
        name: format!("{}.placeholder.local", directory_id),
        short_name: None,
        description: None,
        size: "Small".to_string(),
        directory_type: "SimpleAD".to_string(),
        alias: directory_id.to_string(),
        access_url: format!("{}.awsapps.com", directory_id),
        stage: "Active".to_string(),
        launch_time: now,
        stage_last_updated_date_time: now,
        dns_ip_addrs: vec![],
        vpc_settings: None,
        connect_settings: None,
        ssoid_enabled: false,
    }
}

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

// ---------------------------------------------------------------------------
// aws_directory_service_conditional_forwarder
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_conditional_forwarder` Terraform resources.
///
/// `DsStateView` does not persist conditional forwarder records; only the
/// parent directory is materialised so dependent resources resolve. The
/// `dns_ips` payload and the forwarder `remote_domain_name` mapping are
/// dropped with a warning. Extract returns an empty list.
pub struct AwsDirectoryServiceConditionalForwarderConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceConditionalForwarderConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceConditionalForwarderConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_conditional_forwarder"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceConditionalForwarderConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::ConditionalForwarderTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_directory_service_conditional_forwarder", e)
            })?;

        // `dns_ips` is Vec<String>; read raw and drop (no view counterpart).
        let _dns_ips: Vec<String> = instance
            .attributes
            .get("dns_ips")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = model.remote_domain_name;
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_conditional_forwarder: DsStateView does not persist \
                 conditional forwarders; dns_ips and remote_domain_name dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // conditional forwarders are not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_log_subscription
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_log_subscription` Terraform resources.
///
/// `DsStateView` does not persist log subscription records; only the
/// parent directory is materialised. `log_group_name` is dropped with a
/// warning. Extract returns an empty list.
pub struct AwsDirectoryServiceLogSubscriptionConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceLogSubscriptionConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceLogSubscriptionConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_log_subscription"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceLogSubscriptionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::LogSubscriptionTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_directory_service_log_subscription", e)
            })?;

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = model.log_group_name;
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_log_subscription: DsStateView does not persist \
                 log subscriptions; log_group_name dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // log subscriptions are not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_radius_settings
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_radius_settings` Terraform resources.
///
/// `DsStateView` does not persist RADIUS settings; only the parent
/// directory is materialised. The entire RADIUS payload is dropped with
/// a warning. Extract returns an empty list.
pub struct AwsDirectoryServiceRadiusSettingsConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceRadiusSettingsConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceRadiusSettingsConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_radius_settings"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceRadiusSettingsConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::RadiusSettingsTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_directory_service_radius_settings", e)
            })?;

        // `radius_servers` is Set<String>; read raw and drop.
        let _radius_servers: Vec<String> = instance
            .attributes
            .get("radius_servers")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = (
            model.authentication_protocol,
            model.display_label,
            model.shared_secret,
            model.radius_port,
            model.radius_retries,
            model.radius_timeout,
            model.use_same_username,
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_radius_settings: DsStateView does not persist \
                 RADIUS settings; payload dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // RADIUS settings are not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_region
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_region` Terraform resources.
///
/// `DsStateView` does not persist multi-region replication state; only
/// the parent directory is materialised. The region_name,
/// desired_number_of_domain_controllers, vpc_settings, and tags payload
/// are dropped with a warning. Extract returns an empty list.
pub struct AwsDirectoryServiceRegionConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceRegionConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceRegionConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_region"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceRegionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::RegionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_directory_service_region", e))?;

        // `vpc_settings` is a nested block; read raw and drop.
        let _vpc_settings_raw = instance
            .attributes
            .get("vpc_settings")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = (
            model.region_name,
            model.desired_number_of_domain_controllers,
            model.tags,
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_region: DsStateView does not persist multi-region \
                 replication; region_name, desired_number_of_domain_controllers, vpc_settings, \
                 and tags dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Multi-region replication is not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_shared_directory
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_shared_directory` Terraform resources.
///
/// `DsStateView` does not persist directory sharing records; only the
/// parent directory is materialised. The share method, notes, and target
/// payload are dropped with a warning. Extract returns an empty list.
pub struct AwsDirectoryServiceSharedDirectoryConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceSharedDirectoryConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceSharedDirectoryConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_shared_directory"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceSharedDirectoryConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::SharedDirectoryTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_directory_service_shared_directory", e)
            })?;

        // `target` is a nested block; read raw and drop.
        let _target_raw = instance
            .attributes
            .get("target")
            .cloned()
            .unwrap_or(serde_json::Value::Null);

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = (model.shared_directory_id, model.method, model.notes);
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_shared_directory: DsStateView does not persist directory \
                 sharing; method, notes, and target dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Directory sharing is not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_shared_directory_accepter
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_shared_directory_accepter` Terraform
/// resources.
///
/// `DsStateView` does not persist directory share acceptance records.
/// The accepter resource references a shared_directory_id that was
/// produced by the owner side; no parent directory exists in this
/// account to attach to, so inject is a no-op that emits a warning.
/// Extract returns an empty list.
pub struct AwsDirectoryServiceSharedDirectoryAccepterConverter {
    #[allow(dead_code)]
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceSharedDirectoryAccepterConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceSharedDirectoryAccepterConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_shared_directory_accepter"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_shared_directory"]
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

impl AwsDirectoryServiceSharedDirectoryAccepterConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::SharedDirectoryAccepterTfModel =
            serde_json::from_value(instance.attributes.clone()).map_err(|e| {
                classify_deserialize_error("aws_directory_service_shared_directory_accepter", e)
            })?;

        let _ = (
            model.shared_directory_id,
            model.method,
            model.notes,
            model.owner_account_id,
            model.owner_directory_id,
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_shared_directory_accepter: DsStateView does not persist \
                 directory share acceptance; payload dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Directory share acceptance is not part of the state view; nothing to extract.
        Ok(vec![])
    }
}

// ---------------------------------------------------------------------------
// aws_directory_service_trust
// ---------------------------------------------------------------------------

/// Converts `aws_directory_service_trust` Terraform resources.
///
/// `DsStateView` does not persist trust relationship records; only the
/// parent directory is materialised. All trust attributes are dropped
/// with a warning. Extract returns an empty list.
pub struct AwsDirectoryServiceTrustConverter {
    service: Arc<DirectoryService>,
}

impl AwsDirectoryServiceTrustConverter {
    pub fn new(service: Arc<DirectoryService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsDirectoryServiceTrustConverter {
    fn resource_type(&self) -> &str {
        "aws_directory_service_trust"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_directory_service_directory"]
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

impl AwsDirectoryServiceTrustConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: directory_gen::TrustTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_directory_service_trust", e))?;

        // `conditional_forwarder_ip_addrs` is Set<String>; read raw and drop.
        let _cond_ips: Vec<String> = instance
            .attributes
            .get("conditional_forwarder_ip_addrs")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        state_view
            .directories
            .entry(model.directory_id.clone())
            .or_insert_with(|| placeholder_directory(&model.directory_id));
        self.service
            .restore(&ctx.default_account_id, &region, state_view)
            .await?;

        let _ = (
            model.remote_domain_name,
            model.trust_direction,
            model.trust_password,
            model.trust_type,
            model.selective_auth,
            model.trust_state,
            model.trust_state_reason,
            model.created_date_time,
            model.last_updated_date_time,
            model.state_last_updated_date_time,
            model.delete_associated_conditional_forwarder,
        );
        Ok(ConversionResult {
            region,
            warnings: vec![
                "aws_directory_service_trust: DsStateView does not persist trust relationships; \
                 payload dropped."
                    .to_string(),
            ],
        })
    }

    async fn do_extract(
        &self,
        _ctx: &ConversionContext,
    ) -> Result<Vec<ExtractedResource>, ConversionError> {
        // Trust relationships are not part of the state view; nothing to extract.
        Ok(vec![])
    }
}
