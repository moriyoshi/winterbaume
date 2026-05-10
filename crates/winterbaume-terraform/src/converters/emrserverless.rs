//! Terraform converter for EMR Serverless resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use winterbaume_core::StatefulService;
use winterbaume_emrserverless::EmrServerlessService;
use winterbaume_emrserverless::views::{
    ApplicationView, AutoStartConfigView, AutoStopConfigView, EmrServerlessStateView,
    ImageConfigurationView, InitialCapacityConfigView, InteractiveConfigurationView,
    MaximumCapacityView, NetworkConfigurationView, WorkerConfigurationView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::generated::emrserverless as emrserverless_gen;
use crate::util::{classify_deserialize_error, extract_region, extract_tags};

// ---------------------------------------------------------------------------
// aws_emrserverless_application
// ---------------------------------------------------------------------------

/// Converts `aws_emrserverless_application` Terraform resources to/from EMR Serverless state.
pub struct AwsEmrserverlessApplicationConverter {
    service: Arc<EmrServerlessService>,
}

impl AwsEmrserverlessApplicationConverter {
    pub fn new(service: Arc<EmrServerlessService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsEmrserverlessApplicationConverter {
    fn resource_type(&self) -> &str {
        "aws_emrserverless_application"
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

fn extract_string_array(attrs: &serde_json::Value, key: &str) -> Vec<String> {
    attrs
        .get(key)
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

impl AwsEmrserverlessApplicationConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let model: emrserverless_gen::ApplicationTfModel = serde_json::from_value(attrs.clone())
            .map_err(|e| classify_deserialize_error("aws_emrserverless_application", e))?;

        let name = model.name.clone();

        let release_label = model
            .release_label
            .unwrap_or_else(|| "emr-6.15.0".to_string());
        let application_type = model
            .application_type
            .unwrap_or_else(|| "SPARK".to_string());

        let app_id = model.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string());
        let arn = model.arn.unwrap_or_else(|| {
            format!(
                "arn:aws:emr-serverless:{}:{}:/applications/{}",
                region, ctx.default_account_id, app_id
            )
        });

        // auto_start_configuration
        let auto_start = attrs
            .get("auto_start_configuration")
            .and_then(|v| {
                let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
                obj.get("enabled").and_then(|v| v.as_bool())
            })
            .unwrap_or(true);

        // auto_stop_configuration
        let (auto_stop_enabled, idle_timeout) = {
            let obj = attrs
                .get("auto_stop_configuration")
                .and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
            let enabled = obj
                .and_then(|o| o.get("enabled").and_then(|v| v.as_bool()))
                .unwrap_or(true);
            let timeout = obj
                .and_then(|o| o.get("idle_timeout_minutes").and_then(|v| v.as_i64()))
                .unwrap_or(15);
            (enabled, timeout)
        };

        // initial_capacity
        let initial_capacity = attrs.get("initial_capacity").and_then(|v| {
            v.as_array().map(|arr| {
                arr.iter()
                    .filter_map(|ic| {
                        let ic_obj = ic.as_object().or_else(|| {
                            ic.as_array()
                                .and_then(|a| a.first())
                                .and_then(|v| v.as_object())
                        });
                        ic_obj.map(|obj| {
                            let key = obj
                                .get("initial_capacity_type")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Driver")
                                .to_string();
                            let config_obj = obj
                                .get("initial_capacity_config")
                                .and_then(|v| v.as_array().and_then(|a| a.first()).or(Some(v)));
                            let worker_count = config_obj
                                .and_then(|o| o.get("worker_count").and_then(|v| v.as_i64()))
                                .unwrap_or(1);
                            let worker_configuration = config_obj
                                .and_then(|o| o.get("worker_configuration"))
                                .map(|wc_val| {
                                    let wc =
                                        wc_val.as_array().and_then(|a| a.first()).unwrap_or(wc_val);
                                    WorkerConfigurationView {
                                        cpu: wc
                                            .get("cpu")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("1 vCPU")
                                            .to_string(),
                                        memory: wc
                                            .get("memory")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("4 GB")
                                            .to_string(),
                                        disk: wc
                                            .get("disk")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("20 GB")
                                            .to_string(),
                                    }
                                });
                            (
                                key,
                                InitialCapacityConfigView {
                                    worker_count,
                                    worker_configuration,
                                },
                            )
                        })
                    })
                    .collect::<HashMap<String, InitialCapacityConfigView>>()
            })
        });

        // maximum_capacity
        let maximum_capacity = attrs.get("maximum_capacity").map(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            MaximumCapacityView {
                cpu: obj
                    .get("cpu")
                    .and_then(|v| v.as_str())
                    .unwrap_or("4 vCPU")
                    .to_string(),
                memory: obj
                    .get("memory")
                    .and_then(|v| v.as_str())
                    .unwrap_or("16 GB")
                    .to_string(),
                disk: obj
                    .get("disk")
                    .and_then(|v| v.as_str())
                    .unwrap_or("20 GB")
                    .to_string(),
            }
        });

        // network_configuration
        let network_configuration = attrs.get("network_configuration").and_then(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            let subnet_ids = extract_string_array(obj, "subnet_ids");
            let security_group_ids = extract_string_array(obj, "security_group_ids");
            if subnet_ids.is_empty() && security_group_ids.is_empty() {
                None
            } else {
                Some(NetworkConfigurationView {
                    subnet_ids,
                    security_group_ids,
                })
            }
        });

        // image_configuration block
        let image_configuration = attrs.get("image_configuration").and_then(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            obj.get("image_uri")
                .and_then(|u| u.as_str())
                .map(|uri| ImageConfigurationView {
                    image_uri: uri.to_string(),
                })
        });

        // interactive_configuration block
        let interactive_configuration = attrs.get("interactive_configuration").and_then(|v| {
            let obj = v.as_array().and_then(|a| a.first()).unwrap_or(v);
            if v.is_null() {
                None
            } else {
                Some(InteractiveConfigurationView {
                    studio_enabled: obj
                        .get("studio_enabled")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                    livy_endpoint_enabled: obj
                        .get("livy_endpoint_enabled")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false),
                })
            }
        });

        let now = chrono::Utc::now().to_rfc3339();
        let app_view = ApplicationView {
            application_id: app_id.clone(),
            name,
            arn,
            release_label,
            application_type,
            state: "CREATED".to_string(),
            state_details: String::new(),
            auto_start_configuration: AutoStartConfigView {
                enabled: auto_start,
            },
            auto_stop_configuration: AutoStopConfigView {
                enabled: auto_stop_enabled,
                idle_timeout_minutes: idle_timeout,
            },
            initial_capacity,
            maximum_capacity,
            network_configuration,
            tags: extract_tags(attrs),
            created_at: now.clone(),
            updated_at: now,
            image_configuration,
            interactive_configuration,
        };

        let mut state_view = EmrServerlessStateView {
            applications: HashMap::new(),
            job_runs: HashMap::new(),
            ..Default::default()
        };
        state_view.applications.insert(app_id, app_view);
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
        for app in view.applications.values() {
            let mut attrs = serde_json::json!({
                "id": app.application_id,
                "name": app.name,
                "arn": app.arn,
                "release_label": app.release_label,
                "type": app.application_type,
                "auto_start_configuration": [{
                    "enabled": app.auto_start_configuration.enabled,
                }],
                "auto_stop_configuration": [{
                    "enabled": app.auto_stop_configuration.enabled,
                    "idle_timeout_minutes": app.auto_stop_configuration.idle_timeout_minutes,
                }],
                "tags": app.tags,
            });
            if let Some(ic_cfg) = &app.image_configuration {
                attrs["image_configuration"] = serde_json::json!([{
                    "image_uri": ic_cfg.image_uri,
                }]);
            }
            if let Some(iact_cfg) = &app.interactive_configuration {
                attrs["interactive_configuration"] = serde_json::json!([{
                    "studio_enabled": iact_cfg.studio_enabled,
                    "livy_endpoint_enabled": iact_cfg.livy_endpoint_enabled,
                }]);
            }
            if let Some(ic) = &app.initial_capacity {
                let arr: Vec<serde_json::Value> = ic
                    .iter()
                    .map(|(k, v)| {
                        let mut obj = serde_json::json!({
                            "initial_capacity_type": k,
                            "initial_capacity_config": [{
                                "worker_count": v.worker_count,
                            }],
                        });
                        if let Some(wc) = &v.worker_configuration {
                            obj["initial_capacity_config"][0]["worker_configuration"] =
                                serde_json::json!([{
                                    "cpu": wc.cpu,
                                    "memory": wc.memory,
                                    "disk": wc.disk,
                                }]);
                        }
                        obj
                    })
                    .collect();
                attrs["initial_capacity"] = serde_json::Value::Array(arr);
            }
            if let Some(mc) = &app.maximum_capacity {
                attrs["maximum_capacity"] = serde_json::json!([{
                    "cpu": mc.cpu,
                    "memory": mc.memory,
                    "disk": mc.disk,
                }]);
            }
            if let Some(nc) = &app.network_configuration {
                attrs["network_configuration"] = serde_json::json!([{
                    "subnet_ids": nc.subnet_ids,
                    "security_group_ids": nc.security_group_ids,
                }]);
            }
            results.push(ExtractedResource {
                name: app.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
