use std::collections::HashMap;

use crate::types::*;

#[derive(Debug, Default)]
pub struct ConfigState {
    pub recorders: HashMap<String, ConfigurationRecorder>,
    pub delivery_channels: HashMap<String, DeliveryChannel>,
    pub config_rules: HashMap<String, ConfigRule>,
    pub aggregation_authorizations: HashMap<String, AggregationAuthorizationEntry>,
    pub configuration_aggregators: HashMap<String, ConfigurationAggregatorEntry>,
    pub retention_configurations: HashMap<String, RetentionConfigurationEntry>,
    pub retention_counter: u32,
    pub organization_conformance_packs: HashMap<String, OrganizationConformancePackEntry>,
    pub resource_configs: HashMap<(String, String), ResourceConfigEntry>,
    pub tags: HashMap<String, Vec<TagEntry>>,
    pub evaluations: Vec<EvaluationEntry>,
    pub remediation_configs: HashMap<String, RemediationConfigEntry>,
    pub organization_config_rules: HashMap<String, OrganizationConfigRuleEntry>,
}

#[derive(Debug, Clone)]
pub struct EvaluationEntry {
    pub compliance_resource_type: String,
    pub compliance_resource_id: String,
    pub compliance_type: String,
    pub ordering_timestamp: f64,
    pub annotation: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Configuration recorder name must not be empty.")]
    InvalidConfigurationRecorderName,

    #[error("Cannot find configuration recorder with the specified name '{name}'.")]
    NoSuchConfigurationRecorder { name: String },

    #[error("Cannot find delivery channel with the specified name '{name}'.")]
    NoSuchDeliveryChannel { name: String },

    #[error(
        "The ConfigRule provided in the request is invalid. The rule does not exist for the account in this Region. Please verify the ConfigRule Name: {name}"
    )]
    NoSuchConfigRule { name: String },

    #[error(
        "The configuration aggregator does not exist. Check the configuration aggregator name and try again."
    )]
    NoSuchConfigurationAggregator,

    #[error("Retention configuration with name '{name}' does not exist.")]
    NoSuchRetentionConfiguration { name: String },

    #[error("Organization conformance pack '{name}' does not exist.")]
    NoSuchOrganizationConformancePack { name: String },

    #[error("The organization config rule '{name}' does not exist.")]
    NoSuchOrganizationConfigRule { name: String },
}

impl ConfigState {
    pub fn put_configuration_recorder(
        &mut self,
        name: &str,
        role_arn: &str,
        recording_group: Option<RecordingGroup>,
    ) -> Result<(), ConfigError> {
        if name.is_empty() {
            return Err(ConfigError::InvalidConfigurationRecorderName);
        }

        let existing = self.recorders.get(name);
        let recorder = ConfigurationRecorder {
            name: name.to_string(),
            role_arn: role_arn.to_string(),
            recording_group,
            recording: existing.is_some_and(|r| r.recording),
            last_start_time: existing.and_then(|r| r.last_start_time),
            last_stop_time: existing.and_then(|r| r.last_stop_time),
            recording_mode: existing.and_then(|r| r.recording_mode.clone()),
        };

        self.recorders.insert(name.to_string(), recorder);
        Ok(())
    }

    pub fn describe_configuration_recorders(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&ConfigurationRecorder>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.recorders.get(name) {
                        Some(r) => result.push(r),
                        None => {
                            return Err(ConfigError::NoSuchConfigurationRecorder {
                                name: name.clone(),
                            });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.recorders.values().collect()),
        }
    }

    pub fn delete_configuration_recorder(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.recorders.remove(name).is_none() {
            return Err(ConfigError::NoSuchConfigurationRecorder {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn put_delivery_channel(
        &mut self,
        name: &str,
        s3_bucket_name: &str,
        s3_key_prefix: &str,
    ) -> Result<(), ConfigError> {
        let channel = DeliveryChannel {
            name: name.to_string(),
            s3_bucket_name: s3_bucket_name.to_string(),
            s3_key_prefix: s3_key_prefix.to_string(),
            snapshot_delivery_properties: None,
        };

        self.delivery_channels.insert(name.to_string(), channel);
        Ok(())
    }

    pub fn delete_delivery_channel(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.delivery_channels.remove(name).is_none() {
            return Err(ConfigError::NoSuchDeliveryChannel {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn describe_delivery_channels(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&DeliveryChannel>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.delivery_channels.get(name) {
                        Some(c) => result.push(c),
                        None => {
                            return Err(ConfigError::NoSuchDeliveryChannel { name: name.clone() });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.delivery_channels.values().collect()),
        }
    }

    pub fn put_config_rule(&mut self, rule: ConfigRule) -> Result<(), ConfigError> {
        self.config_rules
            .insert(rule.config_rule_name.clone(), rule);
        Ok(())
    }

    pub fn describe_config_rules(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&ConfigRule>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.config_rules.get(name) {
                        Some(r) => result.push(r),
                        None => {
                            return Err(ConfigError::NoSuchConfigRule { name: name.clone() });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.config_rules.values().collect()),
        }
    }

    pub fn delete_config_rule(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.config_rules.remove(name).is_none() {
            return Err(ConfigError::NoSuchConfigRule {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn put_aggregation_authorization(
        &mut self,
        authorized_account_id: &str,
        authorized_aws_region: &str,
        account_id: &str,
    ) -> &AggregationAuthorizationEntry {
        let key = format!("{authorized_account_id}/{authorized_aws_region}");
        let now = chrono::Utc::now().timestamp() as f64;
        let arn = format!(
            "arn:aws:config:{}:{}:aggregation-authorization/{}/{}",
            authorized_aws_region, account_id, authorized_account_id, authorized_aws_region
        );
        self.aggregation_authorizations
            .entry(key.clone())
            .or_insert_with(|| AggregationAuthorizationEntry {
                authorized_account_id: authorized_account_id.to_string(),
                authorized_aws_region: authorized_aws_region.to_string(),
                aggregation_authorization_arn: arn,
                creation_time: now,
            });
        self.aggregation_authorizations.get(&key).unwrap()
    }

    pub fn describe_aggregation_authorizations(&self) -> Vec<&AggregationAuthorizationEntry> {
        self.aggregation_authorizations.values().collect()
    }

    pub fn delete_aggregation_authorization(
        &mut self,
        authorized_account_id: &str,
        authorized_aws_region: &str,
    ) -> Result<(), ConfigError> {
        let key = format!("{authorized_account_id}/{authorized_aws_region}");
        self.aggregation_authorizations.remove(&key);
        // AWS doesn't error if it doesn't exist
        Ok(())
    }

    pub fn put_configuration_aggregator(
        &mut self,
        name: &str,
        account_aggregation_sources: Option<Vec<AccountAggregationSourceEntry>>,
        organization_aggregation_source: Option<OrganizationAggregationSourceEntry>,
        account_id: &str,
        region: &str,
    ) -> &ConfigurationAggregatorEntry {
        let now = chrono::Utc::now().timestamp() as f64;
        let arn = format!(
            "arn:aws:config:{region}:{account_id}:config-aggregator/config-aggregator-{}",
            &uuid::Uuid::new_v4().to_string()[..8]
        );
        let entry = self
            .configuration_aggregators
            .entry(name.to_string())
            .or_insert_with(|| ConfigurationAggregatorEntry {
                configuration_aggregator_name: name.to_string(),
                configuration_aggregator_arn: arn,
                account_aggregation_sources: None,
                organization_aggregation_source: None,
                creation_time: now,
                last_updated_time: now,
            });
        entry.account_aggregation_sources = account_aggregation_sources;
        entry.organization_aggregation_source = organization_aggregation_source;
        entry.last_updated_time = now;
        self.configuration_aggregators.get(name).unwrap()
    }

    pub fn describe_configuration_aggregators(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&ConfigurationAggregatorEntry>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.configuration_aggregators.get(name) {
                        Some(a) => result.push(a),
                        None => {
                            return Err(ConfigError::NoSuchConfigurationAggregator);
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.configuration_aggregators.values().collect()),
        }
    }

    pub fn delete_configuration_aggregator(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.configuration_aggregators.remove(name).is_none() {
            return Err(ConfigError::NoSuchConfigurationAggregator);
        }
        Ok(())
    }

    pub fn put_retention_configuration(
        &mut self,
        retention_period_in_days: i32,
    ) -> &RetentionConfigurationEntry {
        // AWS Config only supports a single retention configuration named "default"
        let name = if self.retention_configurations.is_empty() {
            self.retention_counter += 1;
            "default".to_string()
        } else {
            self.retention_configurations.keys().next().unwrap().clone()
        };
        let entry = RetentionConfigurationEntry {
            name: name.clone(),
            retention_period_in_days,
        };
        self.retention_configurations.insert(name.clone(), entry);
        self.retention_configurations.get(&name).unwrap()
    }

    pub fn describe_retention_configurations(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&RetentionConfigurationEntry>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.retention_configurations.get(name) {
                        Some(r) => result.push(r),
                        None => {
                            return Err(ConfigError::NoSuchRetentionConfiguration {
                                name: name.clone(),
                            });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.retention_configurations.values().collect()),
        }
    }

    pub fn delete_retention_configuration(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.retention_configurations.remove(name).is_none() {
            return Err(ConfigError::NoSuchRetentionConfiguration {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn start_configuration_recorder(&mut self, name: &str) -> Result<(), ConfigError> {
        match self.recorders.get_mut(name) {
            Some(r) => {
                r.recording = true;
                r.last_start_time = Some(chrono::Utc::now().timestamp() as f64);
                Ok(())
            }
            None => Err(ConfigError::NoSuchConfigurationRecorder {
                name: name.to_string(),
            }),
        }
    }

    pub fn stop_configuration_recorder(&mut self, name: &str) -> Result<(), ConfigError> {
        match self.recorders.get_mut(name) {
            Some(r) => {
                r.recording = false;
                r.last_stop_time = Some(chrono::Utc::now().timestamp() as f64);
                Ok(())
            }
            None => Err(ConfigError::NoSuchConfigurationRecorder {
                name: name.to_string(),
            }),
        }
    }

    pub fn describe_configuration_recorder_status(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&ConfigurationRecorder>, ConfigError> {
        // Same filtering as describe_configuration_recorders
        self.describe_configuration_recorders(names)
    }

    pub fn put_organization_conformance_pack(
        &mut self,
        entry: OrganizationConformancePackEntry,
    ) -> &OrganizationConformancePackEntry {
        let name = entry.organization_conformance_pack_name.clone();
        self.organization_conformance_packs
            .insert(name.clone(), entry);
        self.organization_conformance_packs.get(&name).unwrap()
    }

    pub fn describe_organization_conformance_packs(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&OrganizationConformancePackEntry>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.organization_conformance_packs.get(name) {
                        Some(e) => result.push(e),
                        None => {
                            return Err(ConfigError::NoSuchOrganizationConformancePack {
                                name: name.clone(),
                            });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.organization_conformance_packs.values().collect()),
        }
    }

    pub fn describe_organization_conformance_pack_statuses(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&OrganizationConformancePackEntry>, ConfigError> {
        self.describe_organization_conformance_packs(names)
    }

    pub fn delete_organization_conformance_pack(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.organization_conformance_packs.remove(name).is_none() {
            return Err(ConfigError::NoSuchOrganizationConformancePack {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    pub fn get_organization_conformance_pack_detailed_status(
        &self,
        name: &str,
    ) -> Result<&OrganizationConformancePackEntry, ConfigError> {
        self.organization_conformance_packs
            .get(name)
            .ok_or_else(|| ConfigError::NoSuchOrganizationConformancePack {
                name: name.to_string(),
            })
    }

    pub fn put_resource_config(&mut self, entry: ResourceConfigEntry) {
        let key = (entry.resource_type.clone(), entry.resource_id.clone());
        self.resource_configs.insert(key, entry);
    }

    pub fn delete_resource_config(
        &mut self,
        resource_type: &str,
        resource_id: &str,
    ) -> Result<(), ConfigError> {
        let key = (resource_type.to_string(), resource_id.to_string());
        self.resource_configs.remove(&key);
        // AWS doesn't error if it doesn't exist
        Ok(())
    }

    pub fn list_discovered_resources(&self, resource_type: &str) -> Vec<&ResourceConfigEntry> {
        self.resource_configs
            .values()
            .filter(|r| r.resource_type == resource_type)
            .collect()
    }

    pub fn list_aggregate_discovered_resources(
        &self,
        resource_type: &str,
        _aggregator_name: &str,
    ) -> Vec<&ResourceConfigEntry> {
        self.list_discovered_resources(resource_type)
    }

    pub fn batch_get_resource_config(
        &self,
        keys: &[(String, String)],
    ) -> (Vec<&ResourceConfigEntry>, Vec<(String, String)>) {
        let mut found = Vec::new();
        let mut not_found = Vec::new();
        for (resource_type, resource_id) in keys {
            let key = (resource_type.clone(), resource_id.clone());
            match self.resource_configs.get(&key) {
                Some(entry) => found.push(entry),
                None => not_found.push((resource_type.clone(), resource_id.clone())),
            }
        }
        (found, not_found)
    }

    pub fn get_resource_config_history(
        &self,
        resource_type: &str,
        resource_id: &str,
    ) -> Vec<&ResourceConfigEntry> {
        let key = (resource_type.to_string(), resource_id.to_string());
        match self.resource_configs.get(&key) {
            Some(entry) => vec![entry],
            None => vec![],
        }
    }

    pub fn put_evaluations(&mut self, evaluations: Vec<EvaluationEntry>) -> Vec<EvaluationEntry> {
        // In real AWS, failed evaluations are returned. We accept all.
        self.evaluations.extend(evaluations);
        vec![]
    }

    pub fn tag_resource(&mut self, resource_arn: &str, tags: Vec<TagEntry>) {
        let entry = self.tags.entry(resource_arn.to_string()).or_default();
        for tag in tags {
            // Remove existing tag with same key
            entry.retain(|t| t.key != tag.key);
            entry.push(tag);
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.tags.get_mut(resource_arn) {
            entry.retain(|t| !tag_keys.contains(&t.key));
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> Vec<&TagEntry> {
        match self.tags.get(resource_arn) {
            Some(tags) => tags.iter().collect(),
            None => vec![],
        }
    }

    pub fn select_resource_config(&self, _expression: &str) -> Vec<String> {
        // Minimal stub: return empty results
        vec![]
    }

    pub fn batch_get_aggregate_resource_config(
        &self,
        keys: &[(String, String)],
        _aggregator_name: &str,
    ) -> (Vec<&ResourceConfigEntry>, Vec<(String, String)>) {
        self.batch_get_resource_config(keys)
    }

    // --- Remediation Configuration ---

    pub fn put_remediation_configurations(&mut self, configs: Vec<RemediationConfigEntry>) {
        for config in configs {
            self.remediation_configs
                .insert(config.config_rule_name.clone(), config);
        }
    }

    pub fn describe_remediation_configurations(
        &self,
        config_rule_names: &[String],
    ) -> Vec<&RemediationConfigEntry> {
        if config_rule_names.is_empty() {
            self.remediation_configs.values().collect()
        } else {
            config_rule_names
                .iter()
                .filter_map(|name| self.remediation_configs.get(name))
                .collect()
        }
    }

    pub fn delete_remediation_configuration(
        &mut self,
        config_rule_name: &str,
    ) -> Result<(), ConfigError> {
        self.remediation_configs.remove(config_rule_name);
        // AWS doesn't error if it doesn't exist
        Ok(())
    }

    // --- Organization Config Rules ---

    pub fn put_organization_config_rule(
        &mut self,
        entry: OrganizationConfigRuleEntry,
    ) -> &OrganizationConfigRuleEntry {
        let name = entry.organization_config_rule_name.clone();
        self.organization_config_rules.insert(name.clone(), entry);
        self.organization_config_rules.get(&name).unwrap()
    }

    pub fn describe_organization_config_rules(
        &self,
        names: Option<&[String]>,
    ) -> Result<Vec<&OrganizationConfigRuleEntry>, ConfigError> {
        match names {
            Some(names) => {
                let mut result = Vec::new();
                for name in names {
                    match self.organization_config_rules.get(name) {
                        Some(r) => result.push(r),
                        None => {
                            return Err(ConfigError::NoSuchOrganizationConfigRule {
                                name: name.clone(),
                            });
                        }
                    }
                }
                Ok(result)
            }
            None => Ok(self.organization_config_rules.values().collect()),
        }
    }

    pub fn delete_organization_config_rule(&mut self, name: &str) -> Result<(), ConfigError> {
        if self.organization_config_rules.remove(name).is_none() {
            return Err(ConfigError::NoSuchOrganizationConfigRule {
                name: name.to_string(),
            });
        }
        Ok(())
    }

    // --- Compliance (derived from evaluations) ---

    pub fn get_compliance_details_by_config_rule(
        &self,
        config_rule_name: &str,
    ) -> Vec<&EvaluationEntry> {
        // In the mock, we don't track which rule an evaluation belongs to,
        // so we just return all evaluations for now.
        let _ = config_rule_name;
        self.evaluations.iter().collect()
    }

    pub fn describe_compliance_by_config_rule(
        &self,
        names: Option<&[String]>,
    ) -> Vec<(&ConfigRule, &str)> {
        // Return compliance status for each rule. In mock, all rules are COMPLIANT if
        // they have at least one COMPLIANT evaluation, else NOT_APPLICABLE.
        let rules: Vec<&ConfigRule> = match names {
            Some(names) => names
                .iter()
                .filter_map(|n| self.config_rules.get(n))
                .collect(),
            None => self.config_rules.values().collect(),
        };

        rules
            .into_iter()
            .map(|r| {
                let compliance_type = if self
                    .evaluations
                    .iter()
                    .any(|e| e.compliance_type == "COMPLIANT")
                {
                    "COMPLIANT"
                } else {
                    "NOT_APPLICABLE"
                };
                (r, compliance_type)
            })
            .collect()
    }
}
