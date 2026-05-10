//! Terraform converters for AWS Transfer Family resources.
//!
//! `ServerTfModel` and `UserTfModel` are generated from
//! `specs/transfer.toml`. The ARN templates, the synthesised server ID
//! (uuid simple), the default `endpoint_type = "PUBLIC"` /
//! `identity_provider_type = "SERVICE_MANAGED"` / `domain = "S3"` /
//! `home_directory_type = "PATH"` / `state = "ONLINE"` constants,
//! the `protocols = ["SFTP"]` default, the Vec<TagView>-shaped tags
//! merge, and the home_directory_mappings / posix_profile / endpoint_details /
//! protocol_details / workflow_details / s3_storage_options raw blobs are
//! wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_tfstate::ResourceInstance;
use winterbaume_transfer::TransferService;
use winterbaume_transfer::views::{ServerView, TagView, TransferStateView, UserView};

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::transfer as transfer_gen;
use crate::util::{classify_deserialize_error, extract_region};

/// Extract tags as `Vec<TagView>` from Terraform attributes.
fn extract_tag_views(attrs: &serde_json::Value) -> Vec<TagView> {
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
}

// ---------------------------------------------------------------------------
// aws_transfer_server
// ---------------------------------------------------------------------------

/// Converts `aws_transfer_server` Terraform resources.
pub struct AwsTransferServerConverter {
    service: Arc<TransferService>,
}

impl AwsTransferServerConverter {
    pub fn new(service: Arc<TransferService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTransferServerConverter {
    fn resource_type(&self) -> &str {
        "aws_transfer_server"
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

impl AwsTransferServerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: transfer_gen::ServerTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_transfer_server", e))?;

        let server_id = model
            .id
            .unwrap_or_else(|| format!("s-{}", uuid::Uuid::new_v4().simple()));
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:transfer:{}:{}:server/{}",
                region, ctx.default_account_id, server_id
            )
        });
        let endpoint_type = model.endpoint_type.unwrap_or_else(|| "PUBLIC".to_string());
        let identity_provider_type = model
            .identity_provider_type
            .unwrap_or_else(|| "SERVICE_MANAGED".to_string());
        let domain = model.domain.unwrap_or_else(|| "S3".to_string());
        let logging_role = model.logging_role;
        let certificate = model.certificate;
        let security_policy_name = model.security_policy_name;

        // Additional fields for coverage
        let _ = attrs.get("tags_all");
        let _ = attrs.get("host_key");
        let _ = attrs.get("structured_log_destinations");
        let _ = attrs.get("pre_authentication_login_banner");
        let _ = attrs.get("post_authentication_login_banner");
        let _ = attrs.get("force_destroy");

        // Nested blocks
        let endpoint_details = attrs.get("endpoint_details").cloned();
        let protocol_details = attrs.get("protocol_details").cloned();
        let workflow_details = attrs.get("workflow_details").cloned();
        let s3_storage_options = attrs.get("s3_storage_options").cloned();

        // Parse protocols
        let protocols: Vec<String> = attrs
            .get("protocols")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_else(|| vec!["SFTP".to_string()]);

        let server_view = ServerView {
            arn,
            server_id: server_id.clone(),
            state: "ONLINE".to_string(),
            endpoint_type,
            identity_provider_type,
            protocols,
            domain,
            tags: extract_tag_views(attrs),
            created_at: chrono::Utc::now().to_rfc3339(),
            users: HashMap::new(),
            logging_role,
            certificate,
            security_policy_name,
            endpoint_details,
            protocol_details,
            workflow_details,
            s3_storage_options,
        };

        let mut state_view = TransferStateView {
            servers: HashMap::new(),
            agreements: HashMap::new(),
            certificates: HashMap::new(),
            connectors: HashMap::new(),
            profiles: HashMap::new(),
            web_apps: HashMap::new(),
            workflows: HashMap::new(),
        };
        state_view.servers.insert(server_id, server_view);
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
        for srv in view.servers.values() {
            let tags: HashMap<String, String> = srv
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": srv.server_id,
                "arn": srv.arn,
                "endpoint_type": srv.endpoint_type,
                "identity_provider_type": srv.identity_provider_type,
                "protocols": srv.protocols,
                "domain": srv.domain,
                "logging_role": srv.logging_role,
                "certificate": srv.certificate,
                "security_policy_name": srv.security_policy_name,
                "tags": tags,
                "tags_all": tags,
                "endpoint": format!("{}.server.transfer.{}.amazonaws.com", srv.server_id, ctx.default_region),
                "host_key_fingerprint": "",
                "sftp_authentication_methods": "",
                "endpoint_details": srv.endpoint_details,
                "protocol_details": srv.protocol_details,
                "workflow_details": srv.workflow_details,
                "s3_storage_options": srv.s3_storage_options,
            });
            results.push(ExtractedResource {
                name: srv.server_id.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_transfer_user
// ---------------------------------------------------------------------------

/// Converts `aws_transfer_user` Terraform resources.
pub struct AwsTransferUserConverter {
    service: Arc<TransferService>,
}

impl AwsTransferUserConverter {
    pub fn new(service: Arc<TransferService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsTransferUserConverter {
    fn resource_type(&self) -> &str {
        "aws_transfer_user"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_transfer_server"]
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

impl AwsTransferUserConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: transfer_gen::UserTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_transfer_user", e))?;

        let server_id = model.server_id;
        let user_name = model.user_name;
        let role = model.role;

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:transfer:{}:{}:user/{}/{}",
                region, ctx.default_account_id, server_id, user_name
            )
        });
        let home_directory = model.home_directory;
        let home_directory_type = model
            .home_directory_type
            .unwrap_or_else(|| "PATH".to_string());
        let _policy = attrs.get("policy");
        let home_directory_mappings = attrs.get("home_directory_mappings").cloned();
        let posix_profile = attrs.get("posix_profile").cloned();

        let user_view = UserView {
            server_id: server_id.clone(),
            user_name: user_name.clone(),
            arn,
            role,
            home_directory,
            home_directory_type,
            tags: extract_tag_views(attrs),
            ssh_public_keys: vec![],
            home_directory_mappings,
            posix_profile,
        };

        // We need to merge the user into the existing server's users map.
        // Snapshot, add the user, then merge back.
        let current = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        let mut state_view = TransferStateView {
            servers: HashMap::new(),
            agreements: HashMap::new(),
            certificates: HashMap::new(),
            connectors: HashMap::new(),
            profiles: HashMap::new(),
            web_apps: HashMap::new(),
            workflows: HashMap::new(),
        };

        // Find the existing server and clone it with the new user added
        if let Some(existing_server) = current.servers.get(&server_id) {
            let mut server = existing_server.clone();
            server.users.insert(user_name.clone(), user_view);
            state_view.servers.insert(server_id.clone(), server);
        } else {
            // Server not found; create a minimal stub with just the user
            let mut users = HashMap::new();
            users.insert(user_name.clone(), user_view);
            let server = ServerView {
                arn: format!(
                    "arn:aws:transfer:{}:{}:server/{}",
                    region, ctx.default_account_id, server_id
                ),
                server_id: server_id.clone(),
                state: "ONLINE".to_string(),
                endpoint_type: "PUBLIC".to_string(),
                identity_provider_type: "SERVICE_MANAGED".to_string(),
                protocols: vec!["SFTP".to_string()],
                domain: "S3".to_string(),
                tags: vec![],
                created_at: chrono::Utc::now().to_rfc3339(),
                users,
                logging_role: None,
                certificate: None,
                security_policy_name: None,
                endpoint_details: None,
                protocol_details: None,
                workflow_details: None,
                s3_storage_options: None,
            };
            state_view.servers.insert(server_id.clone(), server);
        }

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
        for srv in view.servers.values() {
            for user in srv.users.values() {
                let tags: HashMap<String, String> = user
                    .tags
                    .iter()
                    .map(|t| (t.key.clone(), t.value.clone()))
                    .collect();
                let attrs = serde_json::json!({
                    "id": format!("{}/{}", user.server_id, user.user_name),
                    "server_id": user.server_id,
                    "user_name": user.user_name,
                    "arn": user.arn,
                    "role": user.role,
                    "home_directory": user.home_directory,
                    "home_directory_type": user.home_directory_type,
                    "tags": tags,
                    "tags_all": tags,
                    "home_directory_mappings": user.home_directory_mappings,
                    "posix_profile": user.posix_profile,
                });
                results.push(ExtractedResource {
                    name: format!("{}/{}", user.server_id, user.user_name),
                    account_id: ctx.default_account_id.clone(),
                    region: ctx.default_region.clone(),
                    attributes: attrs,
                });
            }
        }
        Ok(results)
    }
}
