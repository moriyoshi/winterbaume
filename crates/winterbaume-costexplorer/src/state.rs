use std::collections::HashMap;

use thiserror::Error;

use crate::types::{
    AnomalyMonitorRecord, AnomalySubscriptionRecord, CostAllocationTagBackfillRecord,
    CostAllocationTagRecord, CostCategoryDefinition, CostCategoryRule, SubscriberRecord,
};

/// State for the Cost Explorer mock service.
#[derive(Debug, Default)]
pub struct CostExplorerState {
    /// Cost category definitions keyed by ARN.
    pub cost_categories: HashMap<String, CostCategoryDefinition>,
    /// Auto-incrementing counter for cost category ARN generation.
    next_cost_category_id: u64,
    /// Anomaly monitors keyed by ARN.
    pub anomaly_monitors: HashMap<String, AnomalyMonitorRecord>,
    /// Anomaly subscriptions keyed by ARN.
    pub anomaly_subscriptions: HashMap<String, AnomalySubscriptionRecord>,
    /// Resource tags keyed by ARN.
    pub resource_tags: HashMap<String, Vec<(String, String)>>,
    /// Cost allocation tags keyed by tag key.
    pub cost_allocation_tags: HashMap<String, CostAllocationTagRecord>,
    /// Cost allocation tag backfill jobs.
    pub cost_allocation_tag_backfills: Vec<CostAllocationTagBackfillRecord>,
    /// Anomaly feedback records keyed by anomaly ID.
    pub anomaly_feedback: HashMap<String, String>,
    /// Auto-incrementing counter for resource ARN generation.
    next_resource_id: u64,
}

#[derive(Debug, Error)]
pub enum CostExplorerError {
    #[error("Cost category '{0}' already exists")]
    CostCategoryAlreadyExists(String),
    #[error("Cost category '{0}' not found")]
    CostCategoryNotFound(String),
    #[error("Monitor '{0}' not found")]
    UnknownMonitor(String),
    #[error("Subscription '{0}' not found")]
    UnknownSubscription(String),
}

impl CostExplorerState {
    pub fn new() -> Self {
        Self::default()
    }

    // --- Cost Category operations ---

    pub fn create_cost_category(
        &mut self,
        name: &str,
        rule_version: &str,
        rules: Vec<CostCategoryRule>,
        effective_start: Option<&str>,
        account_id: &str,
        _region: &str,
    ) -> Result<&CostCategoryDefinition, CostExplorerError> {
        // Check by name
        let existing = self.cost_categories.values().any(|d| d.name == name);
        if existing {
            return Err(CostExplorerError::CostCategoryAlreadyExists(
                name.to_string(),
            ));
        }

        self.next_cost_category_id += 1;
        let id = self.next_cost_category_id;
        let arn = format!("arn:aws:ce::{account_id}:costcategory/{id}");
        let effective = effective_start.unwrap_or("2024-01-01").to_string();

        let definition = CostCategoryDefinition {
            name: name.to_string(),
            cost_category_arn: arn.clone(),
            effective_start: effective,
            rule_version: rule_version.to_string(),
            rules,
        };

        self.cost_categories.insert(arn.clone(), definition);
        Ok(self.cost_categories.get(&arn).unwrap())
    }

    pub fn delete_cost_category(
        &mut self,
        cost_category_arn: &str,
    ) -> Result<CostCategoryDefinition, CostExplorerError> {
        match self.cost_categories.remove(cost_category_arn) {
            Some(d) => Ok(d),
            None => Err(CostExplorerError::CostCategoryNotFound(
                cost_category_arn.to_string(),
            )),
        }
    }

    pub fn describe_cost_category(
        &self,
        cost_category_arn: &str,
    ) -> Result<&CostCategoryDefinition, CostExplorerError> {
        self.cost_categories
            .get(cost_category_arn)
            .ok_or_else(|| CostExplorerError::CostCategoryNotFound(cost_category_arn.to_string()))
    }

    pub fn update_cost_category(
        &mut self,
        cost_category_arn: &str,
        rule_version: &str,
        rules: Vec<CostCategoryRule>,
        effective_start: Option<&str>,
    ) -> Result<(String, String), CostExplorerError> {
        match self.cost_categories.get_mut(cost_category_arn) {
            Some(d) => {
                d.rule_version = rule_version.to_string();
                d.rules = rules;
                if let Some(es) = effective_start {
                    d.effective_start = es.to_string();
                }
                Ok((d.cost_category_arn.clone(), d.effective_start.clone()))
            }
            None => Err(CostExplorerError::CostCategoryNotFound(
                cost_category_arn.to_string(),
            )),
        }
    }

    pub fn list_cost_categories(&self) -> Vec<&CostCategoryDefinition> {
        self.cost_categories.values().collect()
    }

    // --- Anomaly Monitor operations ---

    pub fn create_anomaly_monitor(
        &mut self,
        monitor_name: &str,
        monitor_type: &str,
        monitor_dimension: Option<&str>,
        account_id: &str,
    ) -> String {
        self.next_resource_id += 1;
        let id = self.next_resource_id;
        let arn = format!("arn:aws:ce::{account_id}:anomalymonitor/{id}");
        let record = AnomalyMonitorRecord {
            monitor_arn: arn.clone(),
            monitor_name: monitor_name.to_string(),
            monitor_type: monitor_type.to_string(),
            monitor_dimension: monitor_dimension.map(|s| s.to_string()),
            creation_date: "2024-01-01".to_string(),
            last_updated_date: "2024-01-01".to_string(),
            last_evaluated_date: None,
        };
        self.anomaly_monitors.insert(arn.clone(), record);
        arn
    }

    pub fn delete_anomaly_monitor(&mut self, monitor_arn: &str) -> Result<(), CostExplorerError> {
        if self.anomaly_monitors.remove(monitor_arn).is_none() {
            return Err(CostExplorerError::UnknownMonitor(monitor_arn.to_string()));
        }
        Ok(())
    }

    pub fn update_anomaly_monitor(
        &mut self,
        monitor_arn: &str,
        monitor_name: Option<&str>,
    ) -> Result<String, CostExplorerError> {
        match self.anomaly_monitors.get_mut(monitor_arn) {
            Some(m) => {
                if let Some(name) = monitor_name {
                    m.monitor_name = name.to_string();
                }
                Ok(m.monitor_arn.clone())
            }
            None => Err(CostExplorerError::UnknownMonitor(monitor_arn.to_string())),
        }
    }

    pub fn get_anomaly_monitors(
        &self,
        monitor_arn_list: Option<&Vec<String>>,
    ) -> Vec<&AnomalyMonitorRecord> {
        if let Some(arns) = monitor_arn_list {
            if !arns.is_empty() {
                return self
                    .anomaly_monitors
                    .iter()
                    .filter(|(arn, _)| arns.contains(arn))
                    .map(|(_, m)| m)
                    .collect();
            }
        }
        self.anomaly_monitors.values().collect()
    }

    // --- Anomaly Subscription operations ---

    pub fn create_anomaly_subscription(
        &mut self,
        subscription_name: &str,
        monitor_arn_list: Vec<String>,
        subscribers: Vec<SubscriberRecord>,
        frequency: &str,
        threshold: Option<f64>,
        account_id: &str,
    ) -> String {
        self.next_resource_id += 1;
        let id = self.next_resource_id;
        let arn = format!("arn:aws:ce::{account_id}:anomalysubscription/{id}");
        let record = AnomalySubscriptionRecord {
            subscription_arn: arn.clone(),
            subscription_name: subscription_name.to_string(),
            account_id: account_id.to_string(),
            monitor_arn_list,
            subscribers,
            frequency: frequency.to_string(),
            threshold,
        };
        self.anomaly_subscriptions.insert(arn.clone(), record);
        arn
    }

    pub fn delete_anomaly_subscription(
        &mut self,
        subscription_arn: &str,
    ) -> Result<(), CostExplorerError> {
        if self
            .anomaly_subscriptions
            .remove(subscription_arn)
            .is_none()
        {
            return Err(CostExplorerError::UnknownSubscription(
                subscription_arn.to_string(),
            ));
        }
        Ok(())
    }

    pub fn update_anomaly_subscription(
        &mut self,
        subscription_arn: &str,
        frequency: Option<&str>,
        monitor_arn_list: Option<Vec<String>>,
        subscribers: Option<Vec<SubscriberRecord>>,
        subscription_name: Option<&str>,
        threshold: Option<f64>,
    ) -> Result<String, CostExplorerError> {
        match self.anomaly_subscriptions.get_mut(subscription_arn) {
            Some(s) => {
                if let Some(f) = frequency {
                    s.frequency = f.to_string();
                }
                if let Some(arns) = monitor_arn_list {
                    s.monitor_arn_list = arns;
                }
                if let Some(subs) = subscribers {
                    s.subscribers = subs;
                }
                if let Some(name) = subscription_name {
                    s.subscription_name = name.to_string();
                }
                if threshold.is_some() {
                    s.threshold = threshold;
                }
                Ok(s.subscription_arn.clone())
            }
            None => Err(CostExplorerError::UnknownSubscription(
                subscription_arn.to_string(),
            )),
        }
    }

    pub fn get_anomaly_subscriptions(
        &self,
        subscription_arn_list: Option<&Vec<String>>,
        monitor_arn: Option<&str>,
    ) -> Vec<&AnomalySubscriptionRecord> {
        self.anomaly_subscriptions
            .values()
            .filter(|s| {
                if let Some(arns) = subscription_arn_list {
                    if !arns.is_empty() && !arns.contains(&s.subscription_arn) {
                        return false;
                    }
                }
                if let Some(marn) = monitor_arn {
                    if !s.monitor_arn_list.contains(&marn.to_string()) {
                        return false;
                    }
                }
                true
            })
            .collect()
    }

    // --- Resource Tags operations ---

    pub fn tag_resource(&mut self, resource_arn: &str, tags: Vec<(String, String)>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for (k, v) in tags {
            // Replace or insert
            if let Some(existing) = entry.iter_mut().find(|(ek, _)| *ek == k) {
                existing.1 = v;
            } else {
                entry.push((k, v));
            }
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(tags) = self.resource_tags.get_mut(resource_arn) {
            tags.retain(|(k, _)| !tag_keys.contains(k));
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<(String, String)> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    // --- Cost Allocation Tags operations ---

    pub fn list_cost_allocation_tags(&self) -> Vec<&CostAllocationTagRecord> {
        self.cost_allocation_tags.values().collect()
    }

    pub fn update_cost_allocation_tags_status(
        &mut self,
        updates: &[(String, String)],
    ) -> Vec<String> {
        let mut errors = Vec::new();
        for (tag_key, status) in updates {
            if let Some(record) = self.cost_allocation_tags.get_mut(tag_key) {
                record.status = status.clone();
            } else {
                // Create a new record for previously unknown tags
                self.cost_allocation_tags.insert(
                    tag_key.clone(),
                    CostAllocationTagRecord {
                        tag_key: tag_key.clone(),
                        status: status.clone(),
                        tag_type: "UserDefined".to_string(),
                        last_updated_date: None,
                        last_used_date: None,
                    },
                );
            }
        }
        errors
    }

    pub fn start_cost_allocation_tag_backfill(
        &mut self,
        backfill_from: &str,
    ) -> &CostAllocationTagBackfillRecord {
        let now = {
            let d = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default();
            format!("{}Z", d.as_secs())
        };
        let record = CostAllocationTagBackfillRecord {
            backfill_from: backfill_from.to_string(),
            backfill_status: "PROCESSING".to_string(),
            requested_at: now.clone(),
            completed_at: Some(now.clone()),
            last_updated_at: Some(now),
        };
        self.cost_allocation_tag_backfills.push(record);
        self.cost_allocation_tag_backfills.last().unwrap()
    }

    pub fn list_cost_allocation_tag_backfill_history(&self) -> &[CostAllocationTagBackfillRecord] {
        &self.cost_allocation_tag_backfills
    }

    // --- Anomaly Feedback operations ---

    pub fn provide_anomaly_feedback(&mut self, anomaly_id: &str, feedback: &str) {
        self.anomaly_feedback
            .insert(anomaly_id.to_string(), feedback.to_string());
    }
}
