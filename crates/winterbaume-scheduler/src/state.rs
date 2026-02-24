use std::collections::HashMap;

use thiserror::Error;

use crate::types::*;

fn now_epoch_secs() -> String {
    let now = chrono::Utc::now();
    format!("{}.{}", now.timestamp(), now.timestamp_subsec_millis())
}

/// In-memory state for the Scheduler service.
#[derive(Debug, Default)]
pub struct SchedulerState {
    /// Schedule groups keyed by name.
    pub groups: HashMap<String, ScheduleGroup>,
    /// Schedules keyed by (group_name, schedule_name).
    pub schedules: HashMap<(String, String), Schedule>,
}

/// Error type for Scheduler operations.
#[derive(Debug, Error)]
pub enum SchedulerError {
    #[error("Schedule group {0} already exists.")]
    ScheduleGroupAlreadyExists(String),

    #[error("Schedule {0} already exists.")]
    ScheduleAlreadyExists(String),

    #[error("Schedule group {0} does not exist.")]
    ScheduleGroupNotFound(String),

    #[error("Schedule {0} does not exist.")]
    ScheduleNotFound(String),

    #[error("Resource {0} does not exist.")]
    ResourceNotFound(String),
}

impl SchedulerState {
    /// Ensure the "default" group exists for this region.
    fn ensure_default_group(&mut self, account_id: &str, region: &str) {
        if !self.groups.contains_key("default") {
            let arn = format!("arn:aws:scheduler:{region}:{account_id}:schedule-group/default");
            let now = now_epoch_secs();
            self.groups.insert(
                "default".to_string(),
                ScheduleGroup {
                    name: "default".to_string(),
                    arn,
                    state: "ACTIVE".to_string(),
                    creation_date: now.clone(),
                    last_modification_date: now,
                    tags: Vec::new(),
                },
            );
        }
    }

    pub fn create_schedule_group(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
        tags: Vec<Tag>,
    ) -> Result<String, SchedulerError> {
        self.ensure_default_group(account_id, region);
        if self.groups.contains_key(name) {
            return Err(SchedulerError::ScheduleGroupAlreadyExists(name.to_string()));
        }
        let arn = format!("arn:aws:scheduler:{region}:{account_id}:schedule-group/{name}");
        let now = now_epoch_secs();
        self.groups.insert(
            name.to_string(),
            ScheduleGroup {
                name: name.to_string(),
                arn: arn.clone(),
                state: "ACTIVE".to_string(),
                creation_date: now.clone(),
                last_modification_date: now,
                tags,
            },
        );
        Ok(arn)
    }

    pub fn get_schedule_group(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&ScheduleGroup, SchedulerError> {
        self.ensure_default_group(account_id, region);
        self.groups
            .get(name)
            .ok_or_else(|| SchedulerError::ScheduleGroupNotFound(name.to_string()))
    }

    pub fn delete_schedule_group(&mut self, name: &str) -> Result<(), SchedulerError> {
        self.groups
            .remove(name)
            .ok_or_else(|| SchedulerError::ScheduleGroupNotFound(name.to_string()))?;
        // Also remove all schedules in this group
        self.schedules.retain(|(g, _), _| g != name);
        Ok(())
    }

    pub fn list_schedule_groups(
        &mut self,
        account_id: &str,
        region: &str,
        name_prefix: Option<&str>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&ScheduleGroup>, Option<String>) {
        self.ensure_default_group(account_id, region);
        let mut groups: Vec<&ScheduleGroup> = self
            .groups
            .values()
            .filter(|g| match name_prefix {
                Some(prefix) => g.name.starts_with(prefix),
                None => true,
            })
            .collect();
        groups.sort_by(|a, b| a.name.cmp(&b.name));

        // Handle pagination
        let start = if let Some(token) = next_token {
            token.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        if start > 0 {
            groups = groups.into_iter().skip(start).collect();
        }

        let token = if let Some(max) = max_results {
            if groups.len() > max {
                groups.truncate(max);
                Some((start + max).to_string())
            } else {
                None
            }
        } else {
            None
        };

        (groups, token)
    }

    pub fn create_schedule(
        &mut self,
        name: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
        schedule_expression: &str,
        flexible_time_window: FlexibleTimeWindow,
        target: ScheduleTarget,
        state: Option<&str>,
        description: Option<&str>,
        action_after_completion: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<String, SchedulerError> {
        self.ensure_default_group(account_id, region);

        // Check that the group exists
        if !self.groups.contains_key(group_name) {
            return Err(SchedulerError::ScheduleGroupNotFound(
                group_name.to_string(),
            ));
        }

        let key = (group_name.to_string(), name.to_string());
        if self.schedules.contains_key(&key) {
            return Err(SchedulerError::ScheduleAlreadyExists(name.to_string()));
        }

        let arn = format!("arn:aws:scheduler:{region}:{account_id}:schedule/{group_name}/{name}");
        let now = now_epoch_secs();
        let schedule_state = state
            .map(ScheduleState::parse)
            .unwrap_or(ScheduleState::Enabled);

        self.schedules.insert(
            key,
            Schedule {
                name: name.to_string(),
                arn: arn.clone(),
                group_name: group_name.to_string(),
                schedule_expression: schedule_expression.to_string(),
                flexible_time_window,
                target,
                state: schedule_state,
                description: description.map(|s| s.to_string()),
                action_after_completion: action_after_completion.map(|s| s.to_string()),
                start_date: start_date.map(|s| s.to_string()),
                end_date: end_date.map(|s| s.to_string()),
                creation_date: now.clone(),
                last_modification_date: now,
                tags: Vec::new(),
            },
        );

        Ok(arn)
    }

    pub fn get_schedule(
        &mut self,
        name: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<&Schedule, SchedulerError> {
        self.ensure_default_group(account_id, region);
        // Check that the group exists
        if !self.groups.contains_key(group_name) {
            return Err(SchedulerError::ScheduleGroupNotFound(
                group_name.to_string(),
            ));
        }
        let key = (group_name.to_string(), name.to_string());
        self.schedules
            .get(&key)
            .ok_or_else(|| SchedulerError::ScheduleNotFound(name.to_string()))
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_schedule(
        &mut self,
        name: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
        schedule_expression: &str,
        flexible_time_window: FlexibleTimeWindow,
        target: ScheduleTarget,
        state_str: Option<&str>,
        description: Option<&str>,
        action_after_completion: Option<&str>,
        start_date: Option<&str>,
        end_date: Option<&str>,
    ) -> Result<String, SchedulerError> {
        self.ensure_default_group(account_id, region);
        let key = (group_name.to_string(), name.to_string());
        let schedule = self
            .schedules
            .get_mut(&key)
            .ok_or_else(|| SchedulerError::ScheduleNotFound(name.to_string()))?;

        schedule.schedule_expression = schedule_expression.to_string();
        schedule.flexible_time_window = flexible_time_window;
        schedule.target = target;
        // Per AWS UpdateSchedule semantics: optional State omitted -> reset to system default (ENABLED).
        schedule.state = match state_str {
            Some(s) => ScheduleState::parse(s),
            None => ScheduleState::Enabled,
        };
        // Per AWS UpdateSchedule semantics: omitted optional fields are reset to system defaults
        // (i.e., absent / None). See:
        // https://docs.aws.amazon.com/scheduler/latest/APIReference/API_UpdateSchedule.html
        schedule.description = description.map(|s| s.to_string());
        schedule.action_after_completion = action_after_completion.map(|s| s.to_string());
        schedule.start_date = start_date.map(|s| s.to_string());
        schedule.end_date = end_date.map(|s| s.to_string());
        schedule.last_modification_date = now_epoch_secs();

        Ok(schedule.arn.clone())
    }

    pub fn delete_schedule(
        &mut self,
        name: &str,
        group_name: &str,
        account_id: &str,
        region: &str,
    ) -> Result<(), SchedulerError> {
        self.ensure_default_group(account_id, region);
        let key = (group_name.to_string(), name.to_string());
        self.schedules
            .remove(&key)
            .ok_or_else(|| SchedulerError::ScheduleNotFound(name.to_string()))?;
        Ok(())
    }

    pub fn list_schedules(
        &mut self,
        account_id: &str,
        region: &str,
        group_name: Option<&str>,
        state_filter: Option<&str>,
        name_prefix: Option<&str>,
        max_results: Option<usize>,
        next_token: Option<&str>,
    ) -> (Vec<&Schedule>, Option<String>) {
        self.ensure_default_group(account_id, region);
        let mut schedules: Vec<&Schedule> = self
            .schedules
            .values()
            .filter(|s| match group_name {
                Some(g) => s.group_name == g,
                None => true,
            })
            .filter(|s| match state_filter {
                Some(st) => s.state.as_str() == st,
                None => true,
            })
            .filter(|s| match name_prefix {
                Some(prefix) => s.name.starts_with(prefix),
                None => true,
            })
            .collect();
        schedules.sort_by(|a, b| a.name.cmp(&b.name));

        // Handle pagination
        let start = if let Some(token) = next_token {
            token.parse::<usize>().unwrap_or(0)
        } else {
            0
        };

        if start > 0 {
            schedules = schedules.into_iter().skip(start).collect();
        }

        let token = if let Some(max) = max_results {
            if schedules.len() > max {
                schedules.truncate(max);
                Some((start + max).to_string())
            } else {
                None
            }
        } else {
            None
        };

        (schedules, token)
    }

    /// Find a resource (schedule or schedule group) by ARN for tagging operations.
    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: Vec<Tag>,
    ) -> Result<(), SchedulerError> {
        // Try to find in schedule groups
        for group in self.groups.values_mut() {
            if group.arn == resource_arn {
                for tag in &tags {
                    if let Some(existing) = group.tags.iter_mut().find(|t| t.key == tag.key) {
                        existing.value = tag.value.clone();
                    } else {
                        group.tags.push(tag.clone());
                    }
                }
                return Ok(());
            }
        }
        // Try to find in schedules
        for schedule in self.schedules.values_mut() {
            if schedule.arn == resource_arn {
                for tag in &tags {
                    if let Some(existing) = schedule.tags.iter_mut().find(|t| t.key == tag.key) {
                        existing.value = tag.value.clone();
                    } else {
                        schedule.tags.push(tag.clone());
                    }
                }
                return Ok(());
            }
        }
        Err(SchedulerError::ResourceNotFound(resource_arn.to_string()))
    }

    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), SchedulerError> {
        // Try schedule groups
        for group in self.groups.values_mut() {
            if group.arn == resource_arn {
                group.tags.retain(|t| !tag_keys.contains(&t.key));
                return Ok(());
            }
        }
        // Try schedules
        for schedule in self.schedules.values_mut() {
            if schedule.arn == resource_arn {
                schedule.tags.retain(|t| !tag_keys.contains(&t.key));
                return Ok(());
            }
        }
        Err(SchedulerError::ResourceNotFound(resource_arn.to_string()))
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Result<Vec<Tag>, SchedulerError> {
        // Try schedule groups
        for group in self.groups.values() {
            if group.arn == resource_arn {
                return Ok(group.tags.clone());
            }
        }
        // Try schedules
        for schedule in self.schedules.values() {
            if schedule.arn == resource_arn {
                return Ok(schedule.tags.clone());
            }
        }
        Err(SchedulerError::ResourceNotFound(resource_arn.to_string()))
    }
}
