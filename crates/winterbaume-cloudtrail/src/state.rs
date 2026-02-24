use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

#[derive(Debug, Default)]
pub struct CloudTrailState {
    pub trails: HashMap<String, Trail>,
    /// Event data stores keyed by ARN.
    pub event_data_stores: HashMap<String, EventDataStoreData>,
}

#[derive(Debug, thiserror::Error)]
pub enum CloudTrailError {
    #[error("Trail {0} not found.")]
    TrailNotFound(String),
    #[error("Trail {0} already exists.")]
    TrailAlreadyExists(String),
    #[error("The specified event data store was not found: {0}")]
    EventDataStoreNotFound(String),
    #[error("Cannot delete event data store with termination protection enabled")]
    TerminationProtectionEnabled,
}

impl CloudTrailState {
    fn trail_not_found(name: &str) -> CloudTrailError {
        CloudTrailError::TrailNotFound(name.to_string())
    }

    fn find_trail(&self, name_or_arn: &str) -> Result<&Trail, CloudTrailError> {
        // Try by name first, then by ARN
        if let Some(trail) = self.trails.get(name_or_arn) {
            return Ok(trail);
        }
        for trail in self.trails.values() {
            if trail.trail_arn == name_or_arn {
                return Ok(trail);
            }
        }
        Err(Self::trail_not_found(name_or_arn))
    }

    fn find_trail_mut(&mut self, name_or_arn: &str) -> Result<&mut Trail, CloudTrailError> {
        // Try by name first
        if self.trails.contains_key(name_or_arn) {
            return Ok(self.trails.get_mut(name_or_arn).unwrap());
        }
        // Try by ARN
        for trail in self.trails.values_mut() {
            if trail.trail_arn == name_or_arn {
                return Ok(trail);
            }
        }
        Err(Self::trail_not_found(name_or_arn))
    }

    pub fn create_trail(
        &mut self,
        name: &str,
        s3_bucket_name: &str,
        s3_key_prefix: &str,
        include_global: bool,
        is_multi_region: bool,
        account_id: &str,
        region: &str,
    ) -> Result<&Trail, CloudTrailError> {
        if self.trails.contains_key(name) {
            return Err(CloudTrailError::TrailAlreadyExists(name.to_string()));
        }

        let trail_arn = format!("arn:aws:cloudtrail:{region}:{account_id}:trail/{name}");

        let trail = Trail {
            name: name.to_string(),
            s3_bucket_name: s3_bucket_name.to_string(),
            s3_key_prefix: s3_key_prefix.to_string(),
            include_global_service_events: include_global,
            is_multi_region_trail: is_multi_region,
            trail_arn,
            home_region: region.to_string(),
            is_logging: false,
            latest_delivery_time: None,
            tags: HashMap::new(),
            event_selectors: Vec::new(),
            insight_selectors: Vec::new(),
        };

        self.trails.insert(name.to_string(), trail);
        Ok(self.trails.get(name).unwrap())
    }

    pub fn describe_trails(&self, trail_names: &[String]) -> Vec<&Trail> {
        if trail_names.is_empty() {
            self.trails.values().collect()
        } else {
            trail_names
                .iter()
                .filter_map(|n| self.trails.get(n.as_str()))
                .collect()
        }
    }

    pub fn delete_trail(&mut self, name: &str) -> Result<(), CloudTrailError> {
        if self.trails.remove(name).is_none() {
            return Err(Self::trail_not_found(name));
        }
        Ok(())
    }

    pub fn get_trail(&self, name: &str) -> Result<&Trail, CloudTrailError> {
        self.find_trail(name)
    }

    pub fn get_trail_status(&self, name: &str) -> Result<&Trail, CloudTrailError> {
        self.find_trail(name)
    }

    pub fn list_trails(&self) -> Vec<&Trail> {
        self.trails.values().collect()
    }

    pub fn update_trail(
        &mut self,
        name: &str,
        s3_bucket_name: Option<&str>,
        s3_key_prefix: Option<&str>,
        include_global: Option<bool>,
        is_multi_region: Option<bool>,
    ) -> Result<&Trail, CloudTrailError> {
        let trail = self.find_trail_mut(name)?;
        if let Some(bucket) = s3_bucket_name {
            trail.s3_bucket_name = bucket.to_string();
        }
        if let Some(prefix) = s3_key_prefix {
            trail.s3_key_prefix = prefix.to_string();
        }
        if let Some(global) = include_global {
            trail.include_global_service_events = global;
        }
        if let Some(multi) = is_multi_region {
            trail.is_multi_region_trail = multi;
        }
        let trail = self.find_trail(name)?;
        Ok(trail)
    }

    pub fn add_tags(
        &mut self,
        resource_id: &str,
        tags: &[(String, String)],
    ) -> Result<(), CloudTrailError> {
        let trail = self.find_trail_mut(resource_id)?;
        for (k, v) in tags {
            trail.tags.insert(k.clone(), v.clone());
        }
        Ok(())
    }

    pub fn remove_tags(
        &mut self,
        resource_id: &str,
        tag_keys: &[String],
    ) -> Result<(), CloudTrailError> {
        let trail = self.find_trail_mut(resource_id)?;
        for key in tag_keys {
            trail.tags.remove(key);
        }
        Ok(())
    }

    pub fn list_tags(
        &self,
        resource_ids: &[String],
    ) -> Result<Vec<(&Trail, &HashMap<String, String>)>, CloudTrailError> {
        let mut results = Vec::new();
        for id in resource_ids {
            let trail = self.find_trail(id)?;
            results.push((trail, &trail.tags));
        }
        Ok(results)
    }

    pub fn get_event_selectors(&self, name: &str) -> Result<&Trail, CloudTrailError> {
        self.find_trail(name)
    }

    pub fn put_event_selectors(
        &mut self,
        name: &str,
        event_selectors: Vec<EventSelector>,
    ) -> Result<&Trail, CloudTrailError> {
        let trail = self.find_trail_mut(name)?;
        trail.event_selectors = event_selectors;
        let trail = self.find_trail(name)?;
        Ok(trail)
    }

    pub fn get_insight_selectors(&self, name: &str) -> Result<&Trail, CloudTrailError> {
        self.find_trail(name)
    }

    pub fn put_insight_selectors(
        &mut self,
        name: &str,
        insight_selectors: Vec<InsightSelector>,
    ) -> Result<&Trail, CloudTrailError> {
        let trail = self.find_trail_mut(name)?;
        trail.insight_selectors = insight_selectors;
        let trail = self.find_trail(name)?;
        Ok(trail)
    }

    pub fn start_logging(&mut self, name: &str) -> Result<(), CloudTrailError> {
        let trail = self.find_trail_mut(name)?;
        trail.is_logging = true;
        Ok(())
    }

    pub fn stop_logging(&mut self, name: &str) -> Result<(), CloudTrailError> {
        let trail = self.find_trail_mut(name)?;
        trail.is_logging = false;
        Ok(())
    }

    // --- EventDataStore operations ---

    pub fn create_event_data_store(
        &mut self,
        name: &str,
        multi_region_enabled: bool,
        organization_enabled: bool,
        retention_period: i32,
        termination_protection_enabled: bool,
        tags: Vec<(String, String)>,
        account_id: &str,
        region: &str,
    ) -> Result<&EventDataStoreData, CloudTrailError> {
        let eds_id = Uuid::new_v4().to_string();
        let arn = format!("arn:aws:cloudtrail:{region}:{account_id}:eventdatastore/{eds_id}");

        let eds = EventDataStoreData {
            event_data_store_arn: arn.clone(),
            name: name.to_string(),
            status: "ENABLED".to_string(),
            multi_region_enabled,
            organization_enabled,
            retention_period,
            termination_protection_enabled,
            created_timestamp: Utc::now(),
            updated_timestamp: Utc::now(),
            tags,
        };

        self.event_data_stores.insert(arn.clone(), eds);
        Ok(self.event_data_stores.get(&arn).unwrap())
    }

    pub fn get_event_data_store(
        &self,
        arn_or_id: &str,
    ) -> Result<&EventDataStoreData, CloudTrailError> {
        // Try by full ARN first
        if let Some(eds) = self.event_data_stores.get(arn_or_id) {
            return Ok(eds);
        }
        // Try by ID (last segment of ARN)
        for eds in self.event_data_stores.values() {
            if eds.event_data_store_arn.ends_with(arn_or_id) {
                return Ok(eds);
            }
        }
        Err(CloudTrailError::EventDataStoreNotFound(
            arn_or_id.to_string(),
        ))
    }

    pub fn delete_event_data_store(&mut self, arn_or_id: &str) -> Result<(), CloudTrailError> {
        // Find the actual ARN first
        let actual_arn = self
            .get_event_data_store(arn_or_id)?
            .event_data_store_arn
            .clone();
        let eds = self.event_data_stores.get(&actual_arn).unwrap();
        if eds.termination_protection_enabled {
            return Err(CloudTrailError::TerminationProtectionEnabled);
        }
        self.event_data_stores.remove(&actual_arn);
        Ok(())
    }

    pub fn list_event_data_stores(&self) -> Vec<&EventDataStoreData> {
        self.event_data_stores.values().collect()
    }

    pub fn update_event_data_store(
        &mut self,
        arn_or_id: &str,
        name: Option<&str>,
        multi_region_enabled: Option<bool>,
        organization_enabled: Option<bool>,
        retention_period: Option<i32>,
        termination_protection_enabled: Option<bool>,
    ) -> Result<&EventDataStoreData, CloudTrailError> {
        let actual_arn = self
            .get_event_data_store(arn_or_id)?
            .event_data_store_arn
            .clone();
        let eds = self.event_data_stores.get_mut(&actual_arn).unwrap();
        if let Some(n) = name {
            eds.name = n.to_string();
        }
        if let Some(v) = multi_region_enabled {
            eds.multi_region_enabled = v;
        }
        if let Some(v) = organization_enabled {
            eds.organization_enabled = v;
        }
        if let Some(v) = retention_period {
            eds.retention_period = v;
        }
        if let Some(v) = termination_protection_enabled {
            eds.termination_protection_enabled = v;
        }
        eds.updated_timestamp = Utc::now();
        Ok(self.event_data_stores.get(&actual_arn).unwrap())
    }
}
