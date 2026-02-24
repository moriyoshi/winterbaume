use std::collections::HashMap;

use serde_json::Value;
use thiserror::Error;

#[derive(Debug, Default)]
pub struct ApplicationInsightsState {
    /// Applications keyed by ResourceGroupName.
    pub applications: HashMap<String, ApplicationRecord>,
    /// Components keyed by (ResourceGroupName, ComponentName).
    pub components: HashMap<(String, String), ComponentRecord>,
    /// Log patterns keyed by (ResourceGroupName, PatternSetName, PatternName).
    pub log_patterns: HashMap<(String, String, String), LogPatternRecord>,
    /// Workloads keyed by (ResourceGroupName, WorkloadId).
    pub workloads: HashMap<(String, String), WorkloadRecord>,
    /// Problems (read-only inventory; default empty, seedable via state view).
    pub problems: Vec<ProblemRecord>,
    /// Observations (read-only).
    pub observations: HashMap<String, Value>,
    /// Tags per ARN.
    pub tags: HashMap<String, HashMap<String, String>>,
    /// Configuration history events (read-only).
    pub configuration_history: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct ApplicationRecord {
    pub resource_group_name: String,
    pub account_id: String,
    pub remarks: Option<String>,
    pub life_cycle: String,
    pub ops_item_sns_topic_arn: Option<String>,
    pub sns_notification_arn: Option<String>,
    pub ops_center_enabled: Option<bool>,
    pub cwe_monitor_enabled: Option<bool>,
    pub auto_config_enabled: Option<bool>,
    pub attach_missing_permission: Option<bool>,
    pub discovery_type: Option<String>,
    pub component_configurations: HashMap<String, Value>,
}

#[derive(Debug, Clone)]
pub struct ComponentRecord {
    pub component_name: String,
    pub resource_type: Option<String>,
    pub os_type: Option<String>,
    pub tier: Option<String>,
    pub monitor: bool,
    pub component_remarks: Option<String>,
    pub configuration: Option<Value>,
    pub auto_configuration_enabled: bool,
    pub resource_list: Vec<String>,
    pub workload_id_per_resource: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct LogPatternRecord {
    pub pattern_set_name: String,
    pub pattern_name: String,
    pub pattern: String,
    pub rank: i32,
}

#[derive(Debug, Clone)]
pub struct WorkloadRecord {
    pub workload_id: String,
    pub component_name: String,
    pub workload_name: String,
    pub tier: String,
    pub workload_remarks: Option<String>,
    pub workload_configuration: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ProblemRecord {
    pub id: String,
    pub resource_group_name: String,
    pub title: String,
    pub status: String,
    pub severity: Option<String>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub affected_resource: Option<String>,
    pub feedback: HashMap<String, String>,
}

#[derive(Debug, Error)]
pub enum ApplicationInsightsError {
    #[error("{resource_type} {key} does not exist.")]
    NotFound {
        resource_type: &'static str,
        key: String,
    },

    #[error("{resource_type} {key} already exists.")]
    AlreadyExists {
        resource_type: &'static str,
        key: String,
    },

    #[error("{message}")]
    Validation { message: String },
}

impl ApplicationInsightsState {
    pub fn create_application(
        &mut self,
        app: ApplicationRecord,
    ) -> Result<&ApplicationRecord, ApplicationInsightsError> {
        if self.applications.contains_key(&app.resource_group_name) {
            return Err(ApplicationInsightsError::AlreadyExists {
                resource_type: "Application",
                key: app.resource_group_name.clone(),
            });
        }
        let key = app.resource_group_name.clone();
        self.applications.insert(key.clone(), app);
        Ok(self.applications.get(&key).unwrap())
    }

    pub fn get_application(
        &self,
        rg: &str,
    ) -> Result<&ApplicationRecord, ApplicationInsightsError> {
        self.applications
            .get(rg)
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Application",
                key: rg.to_string(),
            })
    }

    pub fn update_application(
        &mut self,
        rg: &str,
        update: impl FnOnce(&mut ApplicationRecord),
    ) -> Result<&ApplicationRecord, ApplicationInsightsError> {
        let a = self
            .applications
            .get_mut(rg)
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Application",
                key: rg.to_string(),
            })?;
        update(a);
        Ok(a)
    }

    pub fn delete_application(&mut self, rg: &str) -> Result<(), ApplicationInsightsError> {
        self.applications
            .remove(rg)
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Application",
                key: rg.to_string(),
            })?;
        self.components.retain(|(r, _), _| r != rg);
        self.log_patterns.retain(|(r, _, _), _| r != rg);
        self.workloads.retain(|(r, _), _| r != rg);
        Ok(())
    }

    pub fn list_applications(&self) -> Vec<&ApplicationRecord> {
        let mut items: Vec<&ApplicationRecord> = self.applications.values().collect();
        items.sort_by(|a, b| a.resource_group_name.cmp(&b.resource_group_name));
        items
    }

    pub fn create_component(
        &mut self,
        rg: &str,
        component: ComponentRecord,
    ) -> Result<&ComponentRecord, ApplicationInsightsError> {
        if !self.applications.contains_key(rg) {
            return Err(ApplicationInsightsError::NotFound {
                resource_type: "Application",
                key: rg.to_string(),
            });
        }
        let key = (rg.to_string(), component.component_name.clone());
        if self.components.contains_key(&key) {
            return Err(ApplicationInsightsError::AlreadyExists {
                resource_type: "Component",
                key: component.component_name.clone(),
            });
        }
        self.components.insert(key.clone(), component);
        Ok(self.components.get(&key).unwrap())
    }

    pub fn get_component(
        &self,
        rg: &str,
        name: &str,
    ) -> Result<&ComponentRecord, ApplicationInsightsError> {
        self.components
            .get(&(rg.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Component",
                key: name.to_string(),
            })
    }

    pub fn update_component(
        &mut self,
        rg: &str,
        name: &str,
        update: impl FnOnce(&mut ComponentRecord),
    ) -> Result<&ComponentRecord, ApplicationInsightsError> {
        let c = self
            .components
            .get_mut(&(rg.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Component",
                key: name.to_string(),
            })?;
        update(c);
        Ok(c)
    }

    pub fn delete_component(
        &mut self,
        rg: &str,
        name: &str,
    ) -> Result<(), ApplicationInsightsError> {
        self.components
            .remove(&(rg.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Component",
                key: name.to_string(),
            })?;
        Ok(())
    }

    pub fn list_components(&self, rg: &str) -> Vec<&ComponentRecord> {
        let mut items: Vec<&ComponentRecord> = self
            .components
            .iter()
            .filter(|((r, _), _)| r == rg)
            .map(|(_, v)| v)
            .collect();
        items.sort_by(|a, b| a.component_name.cmp(&b.component_name));
        items
    }

    pub fn create_log_pattern(
        &mut self,
        rg: &str,
        pattern: LogPatternRecord,
    ) -> Result<&LogPatternRecord, ApplicationInsightsError> {
        if !self.applications.contains_key(rg) {
            return Err(ApplicationInsightsError::NotFound {
                resource_type: "Application",
                key: rg.to_string(),
            });
        }
        let key = (
            rg.to_string(),
            pattern.pattern_set_name.clone(),
            pattern.pattern_name.clone(),
        );
        if self.log_patterns.contains_key(&key) {
            return Err(ApplicationInsightsError::AlreadyExists {
                resource_type: "LogPattern",
                key: pattern.pattern_name.clone(),
            });
        }
        self.log_patterns.insert(key.clone(), pattern);
        Ok(self.log_patterns.get(&key).unwrap())
    }

    pub fn get_log_pattern(
        &self,
        rg: &str,
        set: &str,
        name: &str,
    ) -> Result<&LogPatternRecord, ApplicationInsightsError> {
        self.log_patterns
            .get(&(rg.to_string(), set.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "LogPattern",
                key: name.to_string(),
            })
    }

    pub fn delete_log_pattern(
        &mut self,
        rg: &str,
        set: &str,
        name: &str,
    ) -> Result<(), ApplicationInsightsError> {
        self.log_patterns
            .remove(&(rg.to_string(), set.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "LogPattern",
                key: name.to_string(),
            })?;
        Ok(())
    }

    pub fn update_log_pattern(
        &mut self,
        rg: &str,
        set: &str,
        name: &str,
        pattern: String,
        rank: i32,
    ) -> Result<&LogPatternRecord, ApplicationInsightsError> {
        let lp = self
            .log_patterns
            .get_mut(&(rg.to_string(), set.to_string(), name.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "LogPattern",
                key: name.to_string(),
            })?;
        lp.pattern = pattern;
        lp.rank = rank;
        Ok(lp)
    }

    pub fn list_log_patterns(&self, rg: &str, set: Option<&str>) -> Vec<&LogPatternRecord> {
        let mut items: Vec<&LogPatternRecord> = self
            .log_patterns
            .iter()
            .filter(|((r, s, _), _)| r == rg && set.is_none_or(|sn| s == sn))
            .map(|(_, v)| v)
            .collect();
        items.sort_by(|a, b| a.pattern_name.cmp(&b.pattern_name));
        items
    }

    pub fn list_log_pattern_sets(&self, rg: &str) -> Vec<String> {
        let mut sets: Vec<String> = self
            .log_patterns
            .keys()
            .filter(|(r, _, _)| r == rg)
            .map(|(_, s, _)| s.clone())
            .collect();
        sets.sort();
        sets.dedup();
        sets
    }

    pub fn add_workload(
        &mut self,
        rg: &str,
        workload: WorkloadRecord,
    ) -> Result<&WorkloadRecord, ApplicationInsightsError> {
        let key = (rg.to_string(), workload.workload_id.clone());
        self.workloads.insert(key.clone(), workload);
        Ok(self.workloads.get(&key).unwrap())
    }

    pub fn get_workload(
        &self,
        rg: &str,
        id: &str,
    ) -> Result<&WorkloadRecord, ApplicationInsightsError> {
        self.workloads.get(&(rg.to_string(), id.to_string())).ok_or(
            ApplicationInsightsError::NotFound {
                resource_type: "Workload",
                key: id.to_string(),
            },
        )
    }

    pub fn remove_workload(&mut self, rg: &str, id: &str) -> Result<(), ApplicationInsightsError> {
        self.workloads
            .remove(&(rg.to_string(), id.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Workload",
                key: id.to_string(),
            })?;
        Ok(())
    }

    pub fn update_workload(
        &mut self,
        rg: &str,
        id: &str,
        update: impl FnOnce(&mut WorkloadRecord),
    ) -> Result<&WorkloadRecord, ApplicationInsightsError> {
        let w = self
            .workloads
            .get_mut(&(rg.to_string(), id.to_string()))
            .ok_or(ApplicationInsightsError::NotFound {
                resource_type: "Workload",
                key: id.to_string(),
            })?;
        update(w);
        Ok(w)
    }

    pub fn list_workloads(&self, rg: &str) -> Vec<&WorkloadRecord> {
        let mut items: Vec<&WorkloadRecord> = self
            .workloads
            .iter()
            .filter(|((r, _), _)| r == rg)
            .map(|(_, v)| v)
            .collect();
        items.sort_by(|a, b| a.workload_name.cmp(&b.workload_name));
        items
    }
}
