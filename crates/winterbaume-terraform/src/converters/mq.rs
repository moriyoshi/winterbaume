//! Terraform converters for MQ resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_mq::MqService;
use winterbaume_mq::views::{
    BrokerView, MqConfigurationRevisionView, MqConfigurationView, MqStateView, MqUserView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::mq as mq_gen;
use crate::util::{classify_deserialize_error, extract_region};

// ---------------------------------------------------------------------------
// aws_mq_broker
// ---------------------------------------------------------------------------

/// Converts `aws_mq_broker` Terraform resources to/from MQ state.
pub struct AwsMqBrokerConverter {
    service: Arc<MqService>,
}

impl AwsMqBrokerConverter {
    pub fn new(service: Arc<MqService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMqBrokerConverter {
    fn resource_type(&self) -> &str {
        "aws_mq_broker"
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

impl AwsMqBrokerConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: mq_gen::BrokerTfModel = serde_json::from_value(instance.attributes.clone())
            .map_err(|e| classify_deserialize_error("aws_mq_broker", e))?;

        let attrs = &instance.attributes;
        let broker_name = model.broker_name.clone();

        let broker_id = model.id.unwrap_or_else(|| format!("b-{}-1", broker_name));
        let engine_type = model.engine_type.unwrap_or_else(|| "ActiveMQ".to_string());
        let engine_version = model.engine_version.unwrap_or_else(|| "5.17.6".to_string());
        let host_instance_type = model
            .host_instance_type
            .unwrap_or_else(|| "mq.m5.large".to_string());
        let deployment_mode = model
            .deployment_mode
            .unwrap_or_else(|| "SINGLE_INSTANCE".to_string());
        let publicly_accessible = model.publicly_accessible;
        let auto_minor_version_upgrade = model.auto_minor_version_upgrade;

        let broker_arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:mq:{}:{}:broker:{}:{}",
                region, ctx.default_account_id, broker_name, broker_id
            )
        });
        let _tags_all = attrs.get("tags_all");
        let _apply_immediately = attrs.get("apply_immediately");
        let _auto_minor_version_upgrade_raw = attrs.get("auto_minor_version_upgrade");
        let maintenance_window_start_time = attrs.get("maintenance_window_start_time").cloned();
        let _publicly_accessible_raw = attrs.get("publicly_accessible");
        let configuration = attrs.get("configuration").cloned();
        let encryption_options = attrs.get("encryption_options").cloned();
        let ldap_server_metadata = attrs.get("ldap_server_metadata").cloned();
        let _logs = attrs.get("logs");
        let _data_replication_mode = attrs.get("data_replication_mode");

        let tags = model.tags.clone();

        // Parse users from the Terraform "user" block
        let users: HashMap<String, MqUserView> = attrs
            .get("user")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|u| {
                        let username = u.get("username")?.as_str()?.to_string();
                        let console_access = u
                            .get("console_access")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let groups: Vec<String> = u
                            .get("groups")
                            .and_then(|v| v.as_array())
                            .map(|a| {
                                a.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        Some((
                            username.clone(),
                            MqUserView {
                                username,
                                console_access,
                                groups,
                                replication_user: false,
                            },
                        ))
                    })
                    .collect()
            })
            .unwrap_or_default();

        let broker_view = BrokerView {
            broker_id: broker_id.clone(),
            broker_name,
            broker_arn,
            broker_state: "RUNNING".to_string(),
            engine_type,
            engine_version,
            host_instance_type,
            deployment_mode,
            created: None,
            publicly_accessible,
            auto_minor_version_upgrade,
            tags,
            users,
            configuration,
            encryption_options,
            ldap_server_metadata,
            maintenance_window_start_time,
        };

        let state_view = MqStateView {
            brokers: {
                let mut m = HashMap::new();
                m.insert(broker_id, broker_view);
                m
            },
            configurations: HashMap::new(),
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
        for broker in view.brokers.values() {
            let users: Vec<serde_json::Value> = broker
                .users
                .values()
                .map(|u| {
                    serde_json::json!({
                        "username": u.username,
                        "console_access": u.console_access,
                        "groups": u.groups,
                    })
                })
                .collect();
            let attrs = serde_json::json!({
                "id": broker.broker_id,
                "broker_name": broker.broker_name,
                "arn": broker.broker_arn,
                "broker_id": broker.broker_id,
                "engine_type": broker.engine_type,
                "engine_version": broker.engine_version,
                "host_instance_type": broker.host_instance_type,
                "deployment_mode": broker.deployment_mode,
                "publicly_accessible": broker.publicly_accessible,
                "auto_minor_version_upgrade": broker.auto_minor_version_upgrade,
                "tags": broker.tags,
                "tags_all": broker.tags,
                "user": users,
                "instances": [],
                "logs": [{"general": false, "audit": serde_json::Value::Null}],
                "configuration": broker.configuration,
                "encryption_options": broker.encryption_options,
                "ldap_server_metadata": broker.ldap_server_metadata,
                "maintenance_window_start_time": broker.maintenance_window_start_time,
            });
            results.push(ExtractedResource {
                name: broker.broker_name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_mq_configuration
// ---------------------------------------------------------------------------

/// Converts `aws_mq_configuration` Terraform resources to/from MQ state.
pub struct AwsMqConfigurationConverter {
    service: Arc<MqService>,
}

impl AwsMqConfigurationConverter {
    pub fn new(service: Arc<MqService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsMqConfigurationConverter {
    fn resource_type(&self) -> &str {
        "aws_mq_configuration"
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

impl AwsMqConfigurationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let region = extract_region(&instance.attributes, &ctx.default_region);
        let model: mq_gen::MqConfigurationTfModel =
            serde_json::from_value(instance.attributes.clone())
                .map_err(|e| classify_deserialize_error("aws_mq_configuration", e))?;

        let name = model.name.clone();

        let config_id = model.id.unwrap_or_else(|| format!("c-{}", name));
        let engine_type = model.engine_type.unwrap_or_else(|| "ActiveMQ".to_string());
        let engine_version = model.engine_version.unwrap_or_else(|| "5.17.6".to_string());
        let description = model.description.unwrap_or_default();
        let authentication_strategy = model
            .authentication_strategy
            .unwrap_or_else(|| "simple".to_string());
        let data = model.data.unwrap_or_default();

        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:mq:{}:{}:configuration:{}",
                region, ctx.default_account_id, config_id
            )
        });

        let tags = model.tags.clone();

        let latest_revision = model.latest_revision as i32;

        let revision_view = MqConfigurationRevisionView {
            revision: latest_revision,
            created: None,
            description: description.clone(),
            data,
        };

        let config_view = MqConfigurationView {
            id: config_id.clone(),
            arn,
            name,
            description,
            engine_type,
            engine_version,
            authentication_strategy,
            created: None,
            tags,
            revisions: vec![revision_view],
        };

        let state_view = MqStateView {
            brokers: HashMap::new(),
            configurations: {
                let mut m = HashMap::new();
                m.insert(config_id, config_view);
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
        for config in view.configurations.values() {
            let latest_revision = config.revisions.last().map(|r| r.revision).unwrap_or(1);
            let latest_data = config
                .revisions
                .last()
                .map(|r| r.data.clone())
                .unwrap_or_default();
            let attrs = serde_json::json!({
                "id": config.id,
                "name": config.name,
                "arn": config.arn,
                "description": config.description,
                "engine_type": config.engine_type,
                "engine_version": config.engine_version,
                "authentication_strategy": config.authentication_strategy,
                "latest_revision": latest_revision,
                "data": latest_data,
                "tags": config.tags,
                "tags_all": config.tags,
            });
            results.push(ExtractedResource {
                name: config.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
