//! Terraform converters for EventBridge Scheduler resources.

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use chrono::Utc;
use winterbaume_core::StatefulService;
use winterbaume_scheduler::SchedulerService;
use winterbaume_scheduler::views::{
    FlexibleTimeWindowView, RetryPolicyView, ScheduleGroupView, ScheduleTargetView, ScheduleView,
    SchedulerStateView, TagView,
};
use winterbaume_tfstate::ResourceInstance;

use crate::converter::{
    ConversionContext, ConversionResult, ExtractedResource, TerraformResourceConverter,
};
use crate::error::ConversionError;
use crate::util::{extract_region, optional_str, require_str};

// ---------------------------------------------------------------------------
// aws_scheduler_schedule_group
// ---------------------------------------------------------------------------

pub struct AwsSchedulerScheduleGroupConverter {
    service: Arc<SchedulerService>,
}

impl AwsSchedulerScheduleGroupConverter {
    pub fn new(service: Arc<SchedulerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSchedulerScheduleGroupConverter {
    fn resource_type(&self) -> &str {
        "aws_scheduler_schedule_group"
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

impl AwsSchedulerScheduleGroupConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_scheduler_schedule_group")?;
        let _tags_all = attrs.get("tags_all");
        let _state = optional_str(attrs, "state");
        let _creation_date = optional_str(attrs, "creation_date");
        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:scheduler:{}:{}:schedule-group/{}",
                region, ctx.default_account_id, name
            )
        });

        let tags: Vec<TagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| TagView {
                            key: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let group_view = ScheduleGroupView {
            name: name.to_string(),
            arn,
            state: "ACTIVE".to_string(),
            creation_date: now.clone(),
            last_modification_date: now,
            tags,
        };

        let mut state_view = SchedulerStateView {
            groups: HashMap::new(),
            schedules: HashMap::new(),
        };
        state_view.groups.insert(name.to_string(), group_view);
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
        for group in view.groups.values() {
            let tags: HashMap<String, String> = group
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let attrs = serde_json::json!({
                "id": group.name,
                "name": group.name,
                "arn": group.arn,
                "state": group.state,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: group.name.clone(),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}

// ---------------------------------------------------------------------------
// aws_scheduler_schedule
// ---------------------------------------------------------------------------

pub struct AwsSchedulerScheduleConverter {
    service: Arc<SchedulerService>,
}

impl AwsSchedulerScheduleConverter {
    pub fn new(service: Arc<SchedulerService>) -> Self {
        Self { service }
    }
}

impl TerraformResourceConverter for AwsSchedulerScheduleConverter {
    fn resource_type(&self) -> &str {
        "aws_scheduler_schedule"
    }

    fn depends_on_types(&self) -> Vec<&str> {
        vec!["aws_scheduler_schedule_group"]
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

impl AwsSchedulerScheduleConverter {
    async fn do_inject(
        &self,
        instance: &ResourceInstance,
        ctx: &ConversionContext,
    ) -> Result<ConversionResult, ConversionError> {
        let attrs = &instance.attributes;
        let region = extract_region(attrs, &ctx.default_region);

        let name = require_str(attrs, "name", "aws_scheduler_schedule")?;
        let group_name = optional_str(attrs, "group_name").unwrap_or_else(|| "default".to_string());
        let key = format!("{}\x00{}", group_name, name);

        let arn = optional_str(attrs, "arn").unwrap_or_else(|| {
            format!(
                "arn:aws:scheduler:{}:{}:schedule/{}/{}",
                region, ctx.default_account_id, group_name, name
            )
        });
        let schedule_expression = optional_str(attrs, "schedule_expression").unwrap_or_default();
        let description = optional_str(attrs, "description");
        let state_str = optional_str(attrs, "state").unwrap_or_else(|| "ENABLED".to_string());

        // flexible_time_window
        let flexible_time_window_mode = attrs
            .get("flexible_time_window")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|f| f.get("mode"))
            .and_then(|v| v.as_str())
            .unwrap_or("OFF")
            .to_string();
        let maximum_window = attrs
            .get("flexible_time_window")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|f| f.get("maximum_window_in_minutes"))
            .and_then(|v| v.as_i64());

        // target
        let target_arn = attrs
            .get("target")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|t| t.get("arn"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let target_role_arn = attrs
            .get("target")
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|t| t.get("role_arn"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let tags: Vec<TagView> = attrs
            .get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| {
                obj.iter()
                    .filter_map(|(k, v)| {
                        v.as_str().map(|s| TagView {
                            key: k.clone(),
                            value: s.to_string(),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default();

        let now = Utc::now().to_rfc3339();
        let schedule_view = ScheduleView {
            name: name.to_string(),
            arn,
            group_name: group_name.clone(),
            schedule_expression,
            flexible_time_window: FlexibleTimeWindowView {
                mode: flexible_time_window_mode,
                maximum_window_in_minutes: maximum_window,
            },
            target: ScheduleTargetView {
                arn: target_arn,
                role_arn: target_role_arn,
                retry_policy: RetryPolicyView {
                    maximum_event_age_in_seconds: 86400,
                    maximum_retry_attempts: 185,
                },
            },
            state: state_str,
            description,
            action_after_completion: None,
            start_date: None,
            end_date: None,
            creation_date: now.clone(),
            last_modification_date: now,
            tags,
        };

        // Snapshot + restore to ensure group exists
        let mut state_view = self
            .service
            .snapshot(&ctx.default_account_id, &region)
            .await;
        if !state_view.groups.contains_key(&group_name) {
            state_view.groups.insert(
                group_name.clone(),
                ScheduleGroupView {
                    name: group_name.clone(),
                    arn: format!(
                        "arn:aws:scheduler:{}:{}:schedule-group/{}",
                        region, ctx.default_account_id, group_name
                    ),
                    state: "ACTIVE".to_string(),
                    creation_date: Utc::now().to_rfc3339(),
                    last_modification_date: Utc::now().to_rfc3339(),
                    tags: vec![],
                },
            );
        }
        state_view.schedules.insert(key, schedule_view);
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
        for schedule in view.schedules.values() {
            let tags: HashMap<String, String> = schedule
                .tags
                .iter()
                .map(|t| (t.key.clone(), t.value.clone()))
                .collect();
            let ftw_block = vec![serde_json::json!({
                "mode": schedule.flexible_time_window.mode,
                "maximum_window_in_minutes": schedule.flexible_time_window.maximum_window_in_minutes,
            })];

            let target_block = vec![serde_json::json!({
                "arn": schedule.target.arn,
                "role_arn": schedule.target.role_arn,
                "retry_policy": [{
                    "maximum_event_age_in_seconds": schedule.target.retry_policy.maximum_event_age_in_seconds,
                    "maximum_retry_attempts": schedule.target.retry_policy.maximum_retry_attempts,
                }],
            })];

            let attrs = serde_json::json!({
                "id": format!("{}/{}", schedule.group_name, schedule.name),
                "name": schedule.name,
                "group_name": schedule.group_name,
                "arn": schedule.arn,
                "schedule_expression": schedule.schedule_expression,
                "state": schedule.state,
                "creation_date": schedule.creation_date,
                "last_modification_date": schedule.last_modification_date,
                "description": schedule.description,
                "kms_key_arn": serde_json::Value::Null,
                "flexible_time_window": ftw_block,
                "target": target_block,
                "tags": tags,
                "tags_all": tags,
            });
            results.push(ExtractedResource {
                name: format!("{}/{}", schedule.group_name, schedule.name),
                account_id: ctx.default_account_id.clone(),
                region: ctx.default_region.clone(),
                attributes: attrs,
            });
        }
        Ok(results)
    }
}
