//! Terraform converters for Secrets Manager resources.
//!
//! `SecretTfModel` and `SecretVersionTfModel` are generated from
//! `specs/secretsmanager.toml`. The ARN fall-back chain
//! (arn → id → templated), the `id`-suffix parsing for the version
//! `version_id`, the `version_stages` raw read, the AWSCURRENT
//! demote/promote logic, and the placeholder-secret creation when a
//! version arrives before its parent secret are wired up here.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_secretsmanager::SecretsManagerService;
use winterbaume_secretsmanager::views::{SecretVersionView, SecretView, SecretsmanagerStateView};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::secretsmanager as secretsmanager_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_secretsmanager_secret
// ---------------------------------------------------------------------------

/// Converts `aws_secretsmanager_secret` Terraform resources to/from Secrets Manager state.
pub struct AwsSecretsmanagerSecretConverter {
    service: Arc<SecretsManagerService>,
}

impl AwsSecretsmanagerSecretConverter {
    pub fn new(service: Arc<SecretsManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecretsmanagerSecretConverter {
    fn resource_type(&self) -> &str {
        "aws_secretsmanager_secret"
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

impl AwsSecretsmanagerSecretConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: secretsmanager_gen::SecretTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_secretsmanager_secret", e))?;

        let name = model.name;
        let arn = model.arn.or(model.id).unwrap_or_else(|| {
            let suffix = &uuid::Uuid::new_v4().to_string()[..6];
            format!(
                "arn:aws:secretsmanager:{}:{}:secret:{}-{}",
                region, ctx.default_account_id, name, suffix
            )
        });

        let now = Utc::now().to_rfc3339();
        let secret_view = SecretView {
            name: name.clone(),
            arn,
            description: model.description.unwrap_or_default(),
            created_date: now.clone(),
            last_changed_date: now,
            versions: HashMap::new(),
            current_version_id: None,
            deleted_date: None,
            tags: model.tags,
            resource_policy: model.policy,
            rotation_enabled: None,
            rotation_lambda_arn: None,
            rotation_rules: None,
            last_rotated_date: None,
            replication_status: vec![],
            primary_region: Some(region.clone()),
        };

        let mut state_view = SecretsmanagerStateView {
            secrets: HashMap::new(),
        };
        state_view.secrets.insert(name, secret_view);
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
        for secret in view.secrets.values() {
            if secret.deleted_date.is_some() {
                continue;
            }
            let attrs = serde_json::json!({
                "id": secret.arn,
                "name": secret.name,
                "arn": secret.arn,
                "description": secret.description,
                "tags": secret.tags,
                "tags_all": secret.tags,
                "name_prefix": serde_json::Value::Null,
            });
            results.push(ExtractedResource {
                name: secret.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_secretsmanager_secret_version
// ---------------------------------------------------------------------------

/// Converts `aws_secretsmanager_secret_version` Terraform resources to/from state.
pub struct AwsSecretsmanagerSecretVersionConverter {
    service: Arc<SecretsManagerService>,
}

impl AwsSecretsmanagerSecretVersionConverter {
    pub fn new(service: Arc<SecretsManagerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSecretsmanagerSecretVersionConverter {
    fn resource_type(&self) -> &str {
        "aws_secretsmanager_secret_version"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_secretsmanager_secret"]
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

impl AwsSecretsmanagerSecretVersionConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: secretsmanager_gen::SecretVersionTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_secretsmanager_secret_version", e))?;

        let secret_id = model.secret_id;

        // Parse version_id from the compound id "<secret_arn>|<version_id>" or use field directly
        let version_id = model.version_id.unwrap_or_else(|| {
            model
                .id
                .as_deref()
                .and_then(|id| id.split_once('|').map(|x| x.1.to_string()))
                .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
        });

        let secret_string = model.secret_string;
        let now = Utc::now().to_rfc3339();

        let attrs = &instance.attributes;
        let version_stages: Vec<String> = attrs
            .get("version_stages")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(String::from)
                    .collect()
            })
            .unwrap_or_else(|| vec!["AWSCURRENT".to_string()]);

        let version_view = SecretVersionView {
            version_id: version_id.clone(),
            secret_string,
            secret_binary: None,
            created_date: now,
            version_stages,
        };

        // Take a snapshot, add the version to the right secret, and restore.
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;

        // Resolve secret_id (could be name or ARN)
        let resolved_name = state_view
            .secrets
            .values()
            .find(|s| s.name == secret_id || s.arn == secret_id)
            .map(|s| s.name.clone());

        let name = match resolved_name {
            Some(n) => n,
            None => {
                // Secret not injected yet - use secret_id as name (best-effort)
                secret_id
            }
        };

        if let Some(secret) = state_view.secrets.get_mut(&name) {
            // If this version is AWSCURRENT, demote any existing AWSCURRENT version
            if version_view
                .version_stages
                .contains(&"AWSCURRENT".to_string())
            {
                if let Some(old_vid) = &secret.current_version_id.clone() {
                    if let Some(old_version) = secret.versions.get_mut(old_vid) {
                        old_version.version_stages.retain(|s| s != "AWSCURRENT");
                        if !old_version
                            .version_stages
                            .contains(&"AWSPREVIOUS".to_string())
                        {
                            old_version.version_stages.push("AWSPREVIOUS".to_string());
                        }
                    }
                }
                secret.current_version_id = Some(version_id.clone());
            }
            secret.versions.insert(version_id, version_view);
        } else {
            // Secret doesn't exist yet; create a minimal placeholder with this version
            let arn = format!(
                "arn:aws:secretsmanager:{}:{}:secret:{}-{}",
                region,
                ctx.default_account_id,
                name,
                &uuid::Uuid::new_v4().to_string()[..6]
            );
            let now2 = Utc::now().to_rfc3339();
            let is_current = version_view
                .version_stages
                .contains(&"AWSCURRENT".to_string());
            let mut versions = HashMap::new();
            versions.insert(version_id.clone(), version_view);
            let secret_view = SecretView {
                name: name.clone(),
                arn,
                description: String::new(),
                created_date: now2.clone(),
                last_changed_date: now2,
                versions,
                current_version_id: if is_current { Some(version_id) } else { None },
                deleted_date: None,
                tags: HashMap::new(),
                resource_policy: None,
                rotation_enabled: None,
                rotation_lambda_arn: None,
                rotation_rules: None,
                last_rotated_date: None,
                replication_status: vec![],
                primary_region: Some(region.clone()),
            };
            state_view.secrets.insert(name, secret_view);
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
        for secret in view.secrets.values() {
            if secret.deleted_date.is_some() {
                continue;
            }
            for version in secret.versions.values() {
                let id = format!("{}|{}", secret.arn, version.version_id);
                let attrs = serde_json::json!({
                    "id": id,
                    "secret_id": secret.arn,
                    "version_id": version.version_id,
                    "secret_string": version.secret_string,
                    "version_stages": version.version_stages,
                    "has_secret_string_wo": false,
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
