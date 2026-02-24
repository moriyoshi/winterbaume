use std::collections::HashMap;

use chrono::Utc;
use thiserror::Error;
use uuid::Uuid;

use crate::types::*;

/// In-memory state for the Application Auto Scaling service.
#[derive(Debug, Default)]
pub struct ApplicationAutoScalingState {
    /// Scalable targets keyed by (service_namespace, resource_id, scalable_dimension).
    pub scalable_targets: HashMap<(String, String, String), ScalableTarget>,
    /// Scaling policies keyed by (service_namespace, resource_id, scalable_dimension, policy_name).
    pub scaling_policies: HashMap<(String, String, String, String), ScalingPolicy>,
    /// Scheduled actions keyed by (service_namespace, resource_id, scalable_dimension, scheduled_action_name).
    pub scheduled_actions: HashMap<(String, String, String, String), ScheduledAction>,
    /// Tags keyed by resource ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
}

/// Domain-specific error type for Application Auto Scaling operations.
#[derive(Debug, Error)]
pub enum AppAutoScalingError {
    #[error(
        "No scalable target found for service namespace: {service_namespace}, resource ID: {resource_id}, scalable dimension: {scalable_dimension}"
    )]
    ScalableTargetNotFound {
        service_namespace: String,
        resource_id: String,
        scalable_dimension: String,
    },

    #[error(
        "No scaling policy found for service namespace: {service_namespace}, resource ID: {resource_id}, scalable dimension: {scalable_dimension}, policy name: {policy_name}"
    )]
    ScalingPolicyNotFound {
        service_namespace: String,
        resource_id: String,
        scalable_dimension: String,
        policy_name: String,
    },

    #[error(
        "No scheduled action found for service namespace: {service_namespace}, resource ID: {resource_id}, scalable dimension: {scalable_dimension}, scheduled action name: {scheduled_action_name}"
    )]
    ScheduledActionNotFound {
        service_namespace: String,
        resource_id: String,
        scalable_dimension: String,
        scheduled_action_name: String,
    },

    #[error("Resource ARN not found: {resource_arn}")]
    ResourceNotFound { resource_arn: String },
}

impl ApplicationAutoScalingState {
    pub fn register_scalable_target(
        &mut self,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
        min_capacity: Option<i64>,
        max_capacity: Option<i64>,
        role_arn: Option<&str>,
        suspended_state: Option<SuspendedState>,
        account_id: &str,
        region: &str,
    ) -> Result<String, AppAutoScalingError> {
        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
        );

        if let Some(existing) = self.scalable_targets.get_mut(&key) {
            // Update existing target
            if let Some(min) = min_capacity {
                existing.min_capacity = min;
            }
            if let Some(max) = max_capacity {
                existing.max_capacity = max;
            }
            if let Some(role) = role_arn {
                existing.role_arn = role.to_string();
            }
            if let Some(ss) = suspended_state {
                existing.suspended_state = Some(ss);
            }
            Ok(existing.scalable_target_arn.clone())
        } else {
            // Create new target with a generated ARN
            let target_id = Uuid::new_v4().to_string().replace('-', "");
            let scalable_target_arn = format!(
                "arn:aws:application-autoscaling:{region}:{account_id}:scalable-target/{target_id}"
            );
            let default_role_arn =
                format!("arn:aws:iam::{account_id}:role/aws-service-role/autoscaling");
            let target = ScalableTarget {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
                min_capacity: min_capacity.unwrap_or(0),
                max_capacity: max_capacity.unwrap_or(1),
                role_arn: role_arn.unwrap_or(&default_role_arn).to_string(),
                creation_time: Utc::now(),
                suspended_state,
                scalable_target_arn: scalable_target_arn.clone(),
            };
            self.scalable_targets.insert(key, target);
            Ok(scalable_target_arn)
        }
    }

    pub fn deregister_scalable_target(
        &mut self,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
    ) -> Result<(), AppAutoScalingError> {
        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
        );

        self.scalable_targets.remove(&key).ok_or_else(|| {
            AppAutoScalingError::ScalableTargetNotFound {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
            }
        })?;

        // Also remove any associated scaling policies
        self.scaling_policies.retain(|k, _| {
            !(k.0 == service_namespace && k.1 == resource_id && k.2 == scalable_dimension)
        });

        Ok(())
    }

    pub fn describe_scalable_targets(
        &self,
        service_namespace: &str,
        resource_ids: Option<&[String]>,
        scalable_dimension: Option<&str>,
    ) -> Vec<&ScalableTarget> {
        self.scalable_targets
            .values()
            .filter(|t| {
                t.service_namespace == service_namespace
                    && resource_ids.is_none_or(|ids| ids.iter().any(|id| id == &t.resource_id))
                    && scalable_dimension.is_none_or(|dim| dim == t.scalable_dimension)
            })
            .collect()
    }

    pub fn put_scaling_policy(
        &mut self,
        policy_name: &str,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
        policy_type: Option<&str>,
        target_tracking_config: Option<serde_json::Value>,
        step_scaling_config: Option<serde_json::Value>,
        account_id: &str,
        region: &str,
    ) -> Result<&ScalingPolicy, AppAutoScalingError> {
        // Verify scalable target exists
        let target_key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
        );
        if !self.scalable_targets.contains_key(&target_key) {
            return Err(AppAutoScalingError::ScalableTargetNotFound {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
            });
        }

        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
            policy_name.to_string(),
        );

        let resolved_type = policy_type.unwrap_or("TargetTrackingScaling");

        let policy_arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:scalingPolicy:{policy_id}:resource/{service_namespace}/{resource_id}:policyName/{policy_name}",
            policy_id = uuid::Uuid::new_v4(),
        );

        let policy = ScalingPolicy {
            policy_arn,
            policy_name: policy_name.to_string(),
            service_namespace: service_namespace.to_string(),
            resource_id: resource_id.to_string(),
            scalable_dimension: scalable_dimension.to_string(),
            policy_type: resolved_type.to_string(),
            creation_time: Utc::now(),
            target_tracking_scaling_policy_configuration: target_tracking_config,
            step_scaling_policy_configuration: step_scaling_config,
            alarms: Vec::new(),
        };

        self.scaling_policies.insert(key.clone(), policy);
        Ok(self.scaling_policies.get(&key).unwrap())
    }

    pub fn describe_scaling_policies(
        &self,
        service_namespace: &str,
        resource_id: Option<&str>,
        scalable_dimension: Option<&str>,
        policy_names: Option<&[String]>,
    ) -> Vec<&ScalingPolicy> {
        self.scaling_policies
            .values()
            .filter(|p| {
                p.service_namespace == service_namespace
                    && resource_id.is_none_or(|rid| rid == p.resource_id)
                    && scalable_dimension.is_none_or(|dim| dim == p.scalable_dimension)
                    && policy_names.is_none_or(|names| names.iter().any(|n| n == &p.policy_name))
            })
            .collect()
    }

    pub fn delete_scaling_policy(
        &mut self,
        policy_name: &str,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
    ) -> Result<(), AppAutoScalingError> {
        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
            policy_name.to_string(),
        );

        self.scaling_policies.remove(&key).ok_or_else(|| {
            AppAutoScalingError::ScalingPolicyNotFound {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
                policy_name: policy_name.to_string(),
            }
        })?;

        Ok(())
    }

    pub fn put_scheduled_action(
        &mut self,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
        scheduled_action_name: &str,
        schedule: Option<&str>,
        start_time: Option<f64>,
        end_time: Option<f64>,
        timezone: Option<&str>,
        scalable_target_action: Option<ScalableTargetAction>,
        account_id: &str,
        region: &str,
    ) -> Result<(), AppAutoScalingError> {
        // Verify scalable target exists
        let target_key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
        );
        if !self.scalable_targets.contains_key(&target_key) {
            return Err(AppAutoScalingError::ScalableTargetNotFound {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
            });
        }

        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
            scheduled_action_name.to_string(),
        );

        let action_id = Uuid::new_v4().to_string().replace('-', "");
        let scheduled_action_arn = format!(
            "arn:aws:autoscaling:{region}:{account_id}:scheduledAction:{service_namespace}:scheduledActionName/{scheduled_action_name}:{action_id}"
        );

        let action = ScheduledAction {
            scheduled_action_name: scheduled_action_name.to_string(),
            scheduled_action_arn,
            service_namespace: service_namespace.to_string(),
            resource_id: resource_id.to_string(),
            scalable_dimension: scalable_dimension.to_string(),
            schedule: schedule.map(|s| s.to_string()),
            start_time,
            end_time,
            timezone: timezone.map(|tz| tz.to_string()),
            scalable_target_action,
            creation_time: Utc::now(),
        };

        self.scheduled_actions.insert(key, action);
        Ok(())
    }

    pub fn delete_scheduled_action(
        &mut self,
        service_namespace: &str,
        resource_id: &str,
        scalable_dimension: &str,
        scheduled_action_name: &str,
    ) -> Result<(), AppAutoScalingError> {
        let key = (
            service_namespace.to_string(),
            resource_id.to_string(),
            scalable_dimension.to_string(),
            scheduled_action_name.to_string(),
        );

        self.scheduled_actions.remove(&key).ok_or_else(|| {
            AppAutoScalingError::ScheduledActionNotFound {
                service_namespace: service_namespace.to_string(),
                resource_id: resource_id.to_string(),
                scalable_dimension: scalable_dimension.to_string(),
                scheduled_action_name: scheduled_action_name.to_string(),
            }
        })?;

        Ok(())
    }

    pub fn describe_scheduled_actions(
        &self,
        service_namespace: &str,
        resource_id: Option<&str>,
        scalable_dimension: Option<&str>,
        scheduled_action_names: Option<&[String]>,
    ) -> Vec<&ScheduledAction> {
        self.scheduled_actions
            .values()
            .filter(|a| {
                a.service_namespace == service_namespace
                    && resource_id.is_none_or(|rid| rid == a.resource_id)
                    && scalable_dimension.is_none_or(|dim| dim == a.scalable_dimension)
                    && scheduled_action_names
                        .is_none_or(|names| names.iter().any(|n| n == &a.scheduled_action_name))
            })
            .collect()
    }

    pub fn describe_scaling_activities(
        &self,
        _service_namespace: &str,
        _resource_id: Option<&str>,
        _scalable_dimension: Option<&str>,
    ) -> Vec<()> {
        // Scaling activities are generated by real AWS infrastructure; we return an empty list
        vec![]
    }

    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), AppAutoScalingError> {
        // Verify the resource ARN belongs to a known scalable target
        let exists = self
            .scalable_targets
            .values()
            .any(|t| t.scalable_target_arn == resource_arn);
        if !exists {
            return Err(AppAutoScalingError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
        Ok(())
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), AppAutoScalingError> {
        let exists = self
            .scalable_targets
            .values()
            .any(|t| t.scalable_target_arn == resource_arn);
        if !exists {
            return Err(AppAutoScalingError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            for key in tag_keys {
                entry.remove(key);
            }
        }
        Ok(())
    }

    pub fn list_tags_for_resource(
        &self,
        resource_arn: &str,
    ) -> Result<HashMap<String, String>, AppAutoScalingError> {
        let exists = self
            .scalable_targets
            .values()
            .any(|t| t.scalable_target_arn == resource_arn);
        if !exists {
            return Err(AppAutoScalingError::ResourceNotFound {
                resource_arn: resource_arn.to_string(),
            });
        }
        Ok(self.tags.get(resource_arn).cloned().unwrap_or_default())
    }
}
