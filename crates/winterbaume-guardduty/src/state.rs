use std::collections::HashMap;

use chrono::Utc;
use uuid::Uuid;

use crate::types::*;

fn now_epoch() -> f64 {
    Utc::now().timestamp() as f64
}

#[derive(Debug, Default)]
pub struct GuardDutyState {
    pub detectors: HashMap<String, Detector>,
    pub admin_accounts: Vec<AdminAccount>,
    /// Per-resource tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// Publishing destinations keyed by detector_id then destination_id.
    pub publishing_destinations: HashMap<String, HashMap<String, PublishingDestination>>,
    /// Malware protection plans keyed by plan ID.
    pub malware_protection_plans: HashMap<String, MalwareProtectionPlan>,
    /// Pending invitations keyed by account ID.
    pub invitations: HashMap<String, Invitation>,
    /// Malware scan records keyed by scan ID.
    pub malware_scans: HashMap<String, MalwareScanRecord>,
    next_set_id: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum GuardDutyError {
    #[error(
        "The request is rejected because the input detectorId is not owned by the current account."
    )]
    DetectorNotFound,

    #[error("The request is rejected because a filter with the name {name} already exists.")]
    FilterAlreadyExists { name: String },

    #[error("The request is rejected because the filter name {filter_name} is not found.")]
    FilterNotFound { filter_name: String },

    #[error(
        "The request is rejected because the input ipSetId {ip_set_id} is not owned by the current account."
    )]
    IpSetNotFound { ip_set_id: String },

    #[error(
        "The request is rejected because the input threatIntelSetId {set_id} is not owned by the current account."
    )]
    ThreatIntelSetNotFound { set_id: String },

    #[error(
        "The request is rejected because the input threatEntitySetId {set_id} is not owned by the current account."
    )]
    ThreatEntitySetNotFound { set_id: String },

    #[error(
        "The request is rejected because the input trustedEntitySetId {set_id} is not owned by the current account."
    )]
    TrustedEntitySetNotFound { set_id: String },

    #[error("The account is already enabled as the GuardDuty delegated administrator.")]
    AdminAccountAlreadyEnabled,

    #[error(
        "The request is rejected because the input destinationId {destination_id} is not valid."
    )]
    PublishingDestinationNotFound { destination_id: String },

    #[error("The malware protection plan ID {plan_id} does not exist.")]
    MalwareProtectionPlanNotFound { plan_id: String },

    #[error("The scan ID {scan_id} does not exist.")]
    MalwareScanNotFound { scan_id: String },
}

impl GuardDutyState {
    fn detector_not_found() -> GuardDutyError {
        GuardDutyError::DetectorNotFound
    }

    pub fn create_detector(
        &mut self,
        enable: bool,
        finding_publishing_frequency: &str,
        tags: HashMap<String, String>,
        data_sources: Option<DataSourcesConfig>,
        features: Option<Vec<DetectorFeature>>,
    ) -> Result<String, GuardDutyError> {
        let detector_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        let status = if enable { "ENABLED" } else { "DISABLED" };

        let detector = Detector {
            detector_id: detector_id.clone(),
            status: status.to_string(),
            finding_publishing_frequency: finding_publishing_frequency.to_string(),
            created_at,
            tags,
            filters: HashMap::new(),
            data_sources,
            features,
            ip_sets: HashMap::new(),
            threat_intel_sets: HashMap::new(),
            threat_entity_sets: HashMap::new(),
            trusted_entity_sets: HashMap::new(),
            findings: HashMap::new(),
            members: HashMap::new(),
            malware_scan_settings: MalwareScanSettings::default(),
            org_config: OrgConfig::default(),
            malware_scans: HashMap::new(),
            administrator_account_id: None,
            master_account_id: None,
            coverage_records: Vec::new(),
        };

        self.detectors.insert(detector_id.clone(), detector);
        Ok(detector_id)
    }

    pub fn get_detector(&self, detector_id: &str) -> Result<&Detector, GuardDutyError> {
        self.detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)
    }

    pub fn update_detector(
        &mut self,
        detector_id: &str,
        enable: Option<bool>,
        finding_publishing_frequency: Option<&str>,
        data_sources: Option<DataSourcesConfig>,
        features: Option<Vec<DetectorFeature>>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if let Some(enabled) = enable {
            detector.status = if enabled {
                "ENABLED".to_string()
            } else {
                "DISABLED".to_string()
            };
        }

        if let Some(freq) = finding_publishing_frequency {
            detector.finding_publishing_frequency = freq.to_string();
        }

        if let Some(ds) = data_sources {
            detector.data_sources = Some(ds);
        }

        if let Some(f) = features {
            detector.features = Some(f);
        }

        Ok(())
    }

    pub fn delete_detector(&mut self, detector_id: &str) -> Result<(), GuardDutyError> {
        if self.detectors.remove(detector_id).is_none() {
            return Err(Self::detector_not_found());
        }
        Ok(())
    }

    pub fn list_detector_ids(&self) -> Vec<&str> {
        self.detectors.keys().map(|s| s.as_str()).collect()
    }

    pub fn create_filter(
        &mut self,
        detector_id: &str,
        name: &str,
        description: &str,
        action: &str,
        rank: i32,
        finding_criteria: FindingCriteria,
        tags: HashMap<String, String>,
    ) -> Result<&str, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.filters.contains_key(name) {
            return Err(GuardDutyError::FilterAlreadyExists {
                name: name.to_string(),
            });
        }

        let filter = Filter {
            name: name.to_string(),
            description: description.to_string(),
            action: action.to_string(),
            rank,
            finding_criteria,
            tags,
        };

        detector.filters.insert(name.to_string(), filter);
        Ok(&detector.filters.get(name).unwrap().name)
    }

    pub fn get_filter(
        &self,
        detector_id: &str,
        filter_name: &str,
    ) -> Result<&Filter, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector
            .filters
            .get(filter_name)
            .ok_or_else(|| GuardDutyError::FilterNotFound {
                filter_name: filter_name.to_string(),
            })
    }

    pub fn delete_filter(
        &mut self,
        detector_id: &str,
        filter_name: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.filters.remove(filter_name).is_none() {
            return Err(GuardDutyError::FilterNotFound {
                filter_name: filter_name.to_string(),
            });
        }
        Ok(())
    }

    pub fn update_filter(
        &mut self,
        detector_id: &str,
        filter_name: &str,
        description: Option<&str>,
        action: Option<&str>,
        rank: Option<i32>,
        finding_criteria: Option<FindingCriteria>,
    ) -> Result<&str, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let filter = detector.filters.get_mut(filter_name).ok_or_else(|| {
            GuardDutyError::FilterNotFound {
                filter_name: filter_name.to_string(),
            }
        })?;

        if let Some(desc) = description {
            filter.description = desc.to_string();
        }
        if let Some(act) = action {
            filter.action = act.to_string();
        }
        if let Some(r) = rank {
            filter.rank = r;
        }
        if let Some(fc) = finding_criteria {
            filter.finding_criteria = fc;
        }

        Ok(&filter.name)
    }

    pub fn list_filter_names(&self, detector_id: &str) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.filters.keys().cloned().collect())
    }

    pub fn enable_organization_admin_account(
        &mut self,
        admin_account_id: &str,
    ) -> Result<(), GuardDutyError> {
        for acct in &self.admin_accounts {
            if acct.admin_account_id == admin_account_id {
                return Err(GuardDutyError::AdminAccountAlreadyEnabled);
            }
        }

        self.admin_accounts.push(AdminAccount {
            admin_account_id: admin_account_id.to_string(),
            admin_status: "ENABLED".to_string(),
        });

        Ok(())
    }

    pub fn list_organization_admin_accounts(&self) -> &[AdminAccount] {
        &self.admin_accounts
    }

    pub fn get_administrator_account(&self) -> Option<&AdminAccount> {
        self.admin_accounts.first()
    }

    // ── IP Set operations ─────────────────────────────────────────────

    pub fn create_ip_set(
        &mut self,
        detector_id: &str,
        name: &str,
        format: &str,
        location: &str,
        activate: bool,
        tags: HashMap<String, String>,
    ) -> Result<String, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let ip_set_id = {
            self.next_set_id += 1;
            format!("{:032x}", self.next_set_id)
        };
        let status = if activate { "ACTIVE" } else { "INACTIVE" };

        let ip_set = IpSet {
            ip_set_id: ip_set_id.clone(),
            name: name.to_string(),
            format: format.to_string(),
            location: location.to_string(),
            status: status.to_string(),
            tags,
        };

        detector.ip_sets.insert(ip_set_id.clone(), ip_set);
        Ok(ip_set_id)
    }

    pub fn get_ip_set(&self, detector_id: &str, ip_set_id: &str) -> Result<&IpSet, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector
            .ip_sets
            .get(ip_set_id)
            .ok_or_else(|| GuardDutyError::IpSetNotFound {
                ip_set_id: ip_set_id.to_string(),
            })
    }

    pub fn update_ip_set(
        &mut self,
        detector_id: &str,
        ip_set_id: &str,
        name: Option<&str>,
        location: Option<&str>,
        activate: Option<bool>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let ip_set =
            detector
                .ip_sets
                .get_mut(ip_set_id)
                .ok_or_else(|| GuardDutyError::IpSetNotFound {
                    ip_set_id: ip_set_id.to_string(),
                })?;

        if let Some(n) = name {
            ip_set.name = n.to_string();
        }
        if let Some(loc) = location {
            ip_set.location = loc.to_string();
        }
        if let Some(act) = activate {
            ip_set.status = if act { "ACTIVE" } else { "INACTIVE" }.to_string();
        }

        Ok(())
    }

    pub fn delete_ip_set(
        &mut self,
        detector_id: &str,
        ip_set_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.ip_sets.remove(ip_set_id).is_none() {
            return Err(GuardDutyError::IpSetNotFound {
                ip_set_id: ip_set_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_ip_set_ids(&self, detector_id: &str) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.ip_sets.keys().cloned().collect())
    }

    // ── Threat Intel Set operations ───────────────────────────────────

    pub fn create_threat_intel_set(
        &mut self,
        detector_id: &str,
        name: &str,
        format: &str,
        location: &str,
        activate: bool,
        tags: HashMap<String, String>,
    ) -> Result<String, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set_id = {
            self.next_set_id += 1;
            format!("{:032x}", self.next_set_id)
        };
        let status = if activate { "ACTIVE" } else { "INACTIVE" };

        let set = ThreatIntelSet {
            threat_intel_set_id: set_id.clone(),
            name: name.to_string(),
            format: format.to_string(),
            location: location.to_string(),
            status: status.to_string(),
            tags,
        };

        detector.threat_intel_sets.insert(set_id.clone(), set);
        Ok(set_id)
    }

    pub fn get_threat_intel_set(
        &self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<&ThreatIntelSet, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.threat_intel_sets.get(set_id).ok_or_else(|| {
            GuardDutyError::ThreatIntelSetNotFound {
                set_id: set_id.to_string(),
            }
        })
    }

    pub fn update_threat_intel_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
        name: Option<&str>,
        location: Option<&str>,
        activate: Option<bool>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set = detector.threat_intel_sets.get_mut(set_id).ok_or_else(|| {
            GuardDutyError::ThreatIntelSetNotFound {
                set_id: set_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            set.name = n.to_string();
        }
        if let Some(loc) = location {
            set.location = loc.to_string();
        }
        if let Some(act) = activate {
            set.status = if act { "ACTIVE" } else { "INACTIVE" }.to_string();
        }

        Ok(())
    }

    pub fn delete_threat_intel_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.threat_intel_sets.remove(set_id).is_none() {
            return Err(GuardDutyError::ThreatIntelSetNotFound {
                set_id: set_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_threat_intel_set_ids(
        &self,
        detector_id: &str,
    ) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.threat_intel_sets.keys().cloned().collect())
    }

    // ── Threat Entity Set operations ──────────────────────────────────

    pub fn create_threat_entity_set(
        &mut self,
        detector_id: &str,
        name: &str,
        format: &str,
        location: &str,
        activate: bool,
        tags: HashMap<String, String>,
    ) -> Result<String, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set_id = {
            self.next_set_id += 1;
            format!("{:032x}", self.next_set_id)
        };
        let status = if activate { "ACTIVE" } else { "INACTIVE" };

        let set = ThreatEntitySet {
            threat_entity_set_id: set_id.clone(),
            name: name.to_string(),
            format: format.to_string(),
            location: location.to_string(),
            status: status.to_string(),
            tags,
        };

        detector.threat_entity_sets.insert(set_id.clone(), set);
        Ok(set_id)
    }

    pub fn get_threat_entity_set(
        &self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<&ThreatEntitySet, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.threat_entity_sets.get(set_id).ok_or_else(|| {
            GuardDutyError::ThreatEntitySetNotFound {
                set_id: set_id.to_string(),
            }
        })
    }

    pub fn update_threat_entity_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
        name: Option<&str>,
        location: Option<&str>,
        activate: Option<bool>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set = detector.threat_entity_sets.get_mut(set_id).ok_or_else(|| {
            GuardDutyError::ThreatEntitySetNotFound {
                set_id: set_id.to_string(),
            }
        })?;

        if let Some(n) = name {
            set.name = n.to_string();
        }
        if let Some(loc) = location {
            set.location = loc.to_string();
        }
        if let Some(act) = activate {
            set.status = if act { "ACTIVE" } else { "INACTIVE" }.to_string();
        }

        Ok(())
    }

    pub fn delete_threat_entity_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.threat_entity_sets.remove(set_id).is_none() {
            return Err(GuardDutyError::ThreatEntitySetNotFound {
                set_id: set_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_threat_entity_set_ids(
        &self,
        detector_id: &str,
    ) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.threat_entity_sets.keys().cloned().collect())
    }

    // ── Trusted Entity Set operations ─────────────────────────────────

    pub fn create_trusted_entity_set(
        &mut self,
        detector_id: &str,
        name: &str,
        format: &str,
        location: &str,
        activate: bool,
        tags: HashMap<String, String>,
    ) -> Result<String, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set_id = {
            self.next_set_id += 1;
            format!("{:032x}", self.next_set_id)
        };
        let status = if activate { "ACTIVE" } else { "INACTIVE" };

        let set = TrustedEntitySet {
            trusted_entity_set_id: set_id.clone(),
            name: name.to_string(),
            format: format.to_string(),
            location: location.to_string(),
            status: status.to_string(),
            tags,
        };

        detector.trusted_entity_sets.insert(set_id.clone(), set);
        Ok(set_id)
    }

    pub fn get_trusted_entity_set(
        &self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<&TrustedEntitySet, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.trusted_entity_sets.get(set_id).ok_or_else(|| {
            GuardDutyError::TrustedEntitySetNotFound {
                set_id: set_id.to_string(),
            }
        })
    }

    pub fn update_trusted_entity_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
        name: Option<&str>,
        location: Option<&str>,
        activate: Option<bool>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let set = detector
            .trusted_entity_sets
            .get_mut(set_id)
            .ok_or_else(|| GuardDutyError::TrustedEntitySetNotFound {
                set_id: set_id.to_string(),
            })?;

        if let Some(n) = name {
            set.name = n.to_string();
        }
        if let Some(loc) = location {
            set.location = loc.to_string();
        }
        if let Some(act) = activate {
            set.status = if act { "ACTIVE" } else { "INACTIVE" }.to_string();
        }

        Ok(())
    }

    pub fn delete_trusted_entity_set(
        &mut self,
        detector_id: &str,
        set_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if detector.trusted_entity_sets.remove(set_id).is_none() {
            return Err(GuardDutyError::TrustedEntitySetNotFound {
                set_id: set_id.to_string(),
            });
        }
        Ok(())
    }

    pub fn list_trusted_entity_set_ids(
        &self,
        detector_id: &str,
    ) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.trusted_entity_sets.keys().cloned().collect())
    }

    // ── Resource tag operations ───────────────────────────────────────

    pub fn tag_resource(&mut self, resource_arn: &str, tags: HashMap<String, String>) {
        let entry = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        for (k, v) in tags {
            entry.insert(k, v);
        }
    }

    pub fn untag_resource(&mut self, resource_arn: &str, tag_keys: &[String]) {
        if let Some(entry) = self.resource_tags.get_mut(resource_arn) {
            for k in tag_keys {
                entry.remove(k);
            }
        }
    }

    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    // ── Findings operations ───────────────────────────────────────────

    /// Create synthetic sample findings for the given types (or defaults if none supplied).
    pub fn create_sample_findings(
        &mut self,
        detector_id: &str,
        account_id: &str,
        region: &str,
        finding_types: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let types: Vec<String> = if finding_types.is_empty() {
            vec!["UnauthorizedAccess:EC2/SSHBruteForce".to_string()]
        } else {
            finding_types.to_vec()
        };

        let now = Utc::now().to_rfc3339();
        for finding_type in types {
            let id = Uuid::new_v4().to_string().replace('-', "");
            let arn = format!(
                "arn:aws:guardduty:{region}:{account_id}:detector/{detector_id}/finding/{id}"
            );
            detector.findings.insert(
                id.clone(),
                StoredFinding {
                    id,
                    arn,
                    account_id: account_id.to_string(),
                    region: region.to_string(),
                    r#type: finding_type,
                    severity: 5.0,
                    title: "Sample Finding".to_string(),
                    description: "This is a sample finding generated for testing.".to_string(),
                    created_at: now.clone(),
                    updated_at: now.clone(),
                    archived: false,
                },
            );
        }
        Ok(())
    }

    pub fn list_findings(
        &self,
        detector_id: &str,
        include_archived: bool,
    ) -> Result<Vec<String>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector
            .findings
            .values()
            .filter(|f| include_archived || !f.archived)
            .map(|f| f.id.clone())
            .collect())
    }

    pub fn get_findings(
        &self,
        detector_id: &str,
        finding_ids: &[String],
    ) -> Result<Vec<&StoredFinding>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(finding_ids
            .iter()
            .filter_map(|id| detector.findings.get(id))
            .collect())
    }

    pub fn archive_findings(
        &mut self,
        detector_id: &str,
        finding_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in finding_ids {
            if let Some(f) = detector.findings.get_mut(id) {
                f.archived = true;
                f.updated_at = now.clone();
            }
        }
        Ok(())
    }

    pub fn unarchive_findings(
        &mut self,
        detector_id: &str,
        finding_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in finding_ids {
            if let Some(f) = detector.findings.get_mut(id) {
                f.archived = false;
                f.updated_at = now.clone();
            }
        }
        Ok(())
    }

    pub fn get_findings_statistics(
        &self,
        detector_id: &str,
    ) -> Result<std::collections::HashMap<String, i32>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let mut count_by_severity: std::collections::HashMap<String, i32> =
            std::collections::HashMap::new();
        for finding in detector.findings.values().filter(|f| !f.archived) {
            let severity_key = format!("{:.1}", finding.severity);
            *count_by_severity.entry(severity_key).or_insert(0) += 1;
        }
        Ok(count_by_severity)
    }

    // --- Publishing destination operations ---

    pub fn create_publishing_destination(
        &mut self,
        detector_id: &str,
        destination_type: &str,
        destination_arn: Option<&str>,
        kms_key_arn: Option<&str>,
        tags: HashMap<String, String>,
    ) -> Result<String, GuardDutyError> {
        if !self.detectors.contains_key(detector_id) {
            return Err(GuardDutyError::DetectorNotFound);
        }
        let destination_id = Uuid::new_v4().to_string().replace('-', "");
        let dest = PublishingDestination {
            destination_id: destination_id.clone(),
            destination_type: destination_type.to_string(),
            status: "PUBLISHING".to_string(),
            destination_arn: destination_arn.map(|s| s.to_string()),
            kms_key_arn: kms_key_arn.map(|s| s.to_string()),
            tags,
        };
        self.publishing_destinations
            .entry(detector_id.to_string())
            .or_default()
            .insert(destination_id.clone(), dest);
        Ok(destination_id)
    }

    pub fn list_publishing_destinations(
        &self,
        detector_id: &str,
    ) -> Result<Vec<&PublishingDestination>, GuardDutyError> {
        if !self.detectors.contains_key(detector_id) {
            return Err(GuardDutyError::DetectorNotFound);
        }
        Ok(self
            .publishing_destinations
            .get(detector_id)
            .map(|m| m.values().collect())
            .unwrap_or_default())
    }

    pub fn get_publishing_destination(
        &self,
        detector_id: &str,
        destination_id: &str,
    ) -> Result<&PublishingDestination, GuardDutyError> {
        if !self.detectors.contains_key(detector_id) {
            return Err(GuardDutyError::DetectorNotFound);
        }
        self.publishing_destinations
            .get(detector_id)
            .and_then(|m| m.get(destination_id))
            .ok_or_else(|| GuardDutyError::PublishingDestinationNotFound {
                destination_id: destination_id.to_string(),
            })
    }

    pub fn update_publishing_destination(
        &mut self,
        detector_id: &str,
        destination_id: &str,
        destination_arn: Option<&str>,
        kms_key_arn: Option<&str>,
    ) -> Result<(), GuardDutyError> {
        if !self.detectors.contains_key(detector_id) {
            return Err(GuardDutyError::DetectorNotFound);
        }
        let dest = self
            .publishing_destinations
            .get_mut(detector_id)
            .and_then(|m| m.get_mut(destination_id))
            .ok_or_else(|| GuardDutyError::PublishingDestinationNotFound {
                destination_id: destination_id.to_string(),
            })?;
        if let Some(arn) = destination_arn {
            dest.destination_arn = Some(arn.to_string());
        }
        if let Some(kms) = kms_key_arn {
            dest.kms_key_arn = Some(kms.to_string());
        }
        Ok(())
    }

    pub fn delete_publishing_destination(
        &mut self,
        detector_id: &str,
        destination_id: &str,
    ) -> Result<(), GuardDutyError> {
        if !self.detectors.contains_key(detector_id) {
            return Err(GuardDutyError::DetectorNotFound);
        }
        let removed = self
            .publishing_destinations
            .get_mut(detector_id)
            .and_then(|m| m.remove(destination_id));
        if removed.is_none() {
            return Err(GuardDutyError::PublishingDestinationNotFound {
                destination_id: destination_id.to_string(),
            });
        }
        Ok(())
    }

    // ── Member operations ─────────────────────────────────────────────

    pub fn create_members(
        &mut self,
        detector_id: &str,
        accounts: Vec<(String, String)>, // (account_id, email)
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for (account_id, email) in accounts {
            detector
                .members
                .entry(account_id.clone())
                .or_insert(Member {
                    account_id,
                    email,
                    relationship_status: "Created".to_string(),
                    invited_at: None,
                    updated_at: now.clone(),
                    detector_features: Vec::new(),
                });
        }
        Ok(())
    }

    pub fn get_members(
        &self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<Vec<&Member>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let members: Vec<&Member> = if account_ids.is_empty() {
            detector.members.values().collect()
        } else {
            account_ids
                .iter()
                .filter_map(|id| detector.members.get(id))
                .collect()
        };
        Ok(members)
    }

    pub fn list_members(&self, detector_id: &str) -> Result<Vec<&Member>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;
        Ok(detector.members.values().collect())
    }

    pub fn delete_members(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        for id in account_ids {
            detector.members.remove(id);
        }
        Ok(())
    }

    pub fn disassociate_members(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in account_ids {
            if let Some(m) = detector.members.get_mut(id) {
                m.relationship_status = "Removed".to_string();
                m.updated_at = now.clone();
            }
        }
        Ok(())
    }

    pub fn invite_members(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in account_ids {
            if let Some(m) = detector.members.get_mut(id) {
                m.relationship_status = "Invited".to_string();
                m.invited_at = Some(now.clone());
                m.updated_at = now.clone();
            }
        }
        Ok(())
    }

    pub fn start_monitoring_members(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in account_ids {
            if let Some(m) = detector.members.get_mut(id) {
                m.relationship_status = "Enabled".to_string();
                m.updated_at = now.clone();
            }
        }
        Ok(())
    }

    pub fn stop_monitoring_members(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let now = Utc::now().to_rfc3339();
        for id in account_ids {
            if let Some(m) = detector.members.get_mut(id) {
                m.relationship_status = "Disabled".to_string();
                m.updated_at = now.clone();
            }
        }
        Ok(())
    }

    // ── Member detector feature configuration ─────────────────────────

    /// Returns references to the requested members together with their stored
    /// detector feature configuration.  If `account_ids` is empty every member
    /// in the detector is returned.
    pub fn get_member_detectors(
        &self,
        detector_id: &str,
        account_ids: &[String],
    ) -> Result<Vec<&Member>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let members: Vec<&Member> = if account_ids.is_empty() {
            detector.members.values().collect()
        } else {
            account_ids
                .iter()
                .filter_map(|id| detector.members.get(id))
                .collect()
        };
        Ok(members)
    }

    /// Applies feature-level overrides to the listed member accounts.
    pub fn update_member_detectors(
        &mut self,
        detector_id: &str,
        account_ids: &[String],
        features: Vec<MemberDetectorFeature>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        for id in account_ids {
            if let Some(m) = detector.members.get_mut(id) {
                m.detector_features = features.clone();
            }
        }
        Ok(())
    }

    // ── Organization configuration ────────────────────────────────────

    pub fn update_org_config(
        &mut self,
        detector_id: &str,
        auto_enable: Option<bool>,
        auto_enable_organization_members: Option<&str>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if let Some(ae) = auto_enable {
            detector.org_config.auto_enable = ae;
        }
        if let Some(s) = auto_enable_organization_members {
            detector.org_config.auto_enable_organization_members = Some(s.to_string());
        }
        Ok(())
    }

    pub fn get_org_config(&self, detector_id: &str) -> Result<&OrgConfig, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;
        Ok(&detector.org_config)
    }

    // ── Org admin account management ──────────────────────────────────

    pub fn disable_organization_admin_account(&mut self, admin_account_id: &str) {
        self.admin_accounts
            .retain(|a| a.admin_account_id != admin_account_id);
    }

    // ── Malware scan settings ─────────────────────────────────────────

    pub fn get_malware_scan_settings(
        &self,
        detector_id: &str,
    ) -> Result<&MalwareScanSettings, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;
        Ok(&detector.malware_scan_settings)
    }

    pub fn update_malware_scan_settings(
        &mut self,
        detector_id: &str,
        ebs_snapshot_preservation: Option<&str>,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        if let Some(v) = ebs_snapshot_preservation {
            detector.malware_scan_settings.ebs_snapshot_preservation = Some(v.to_string());
        }
        Ok(())
    }

    // ── Malware scans ─────────────────────────────────────────────────

    pub fn start_malware_scan(
        &mut self,
        detector_id: &str,
        resource_arn: Option<&str>,
    ) -> Result<String, GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        let scan_id = Uuid::new_v4().to_string().replace('-', "");
        let scan = StoredMalwareScan {
            scan_id: scan_id.clone(),
            detector_id: detector_id.to_string(),
            resource_arn: resource_arn.map(|s| s.to_string()),
            resource_type: None,
            scan_type: "GUARDDUTY_INITIATED".to_string(),
            scan_status: "RUNNING".to_string(),
            scan_started_at: now_epoch(),
            scan_completed_at: None,
        };
        detector.malware_scans.insert(scan_id.clone(), scan);
        Ok(scan_id)
    }

    pub fn get_malware_scan(&self, scan_id: &str) -> Result<&StoredMalwareScan, GuardDutyError> {
        // Search across all detectors
        for detector in self.detectors.values() {
            if let Some(scan) = detector.malware_scans.get(scan_id) {
                return Ok(scan);
            }
        }
        Err(GuardDutyError::MalwareScanNotFound {
            scan_id: scan_id.to_string(),
        })
    }

    pub fn list_malware_scans(
        &self,
        detector_id: &str,
    ) -> Result<Vec<&StoredMalwareScan>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;
        Ok(detector.malware_scans.values().collect())
    }

    // ── Malware Protection Plans ──────────────────────────────────────

    pub fn create_malware_protection_plan(
        &mut self,
        account_id: &str,
        region: &str,
        role: &str,
        s3_bucket_name: Option<&str>,
        s3_object_prefixes: Vec<String>,
        actions: Option<MalwareProtectionPlanActions>,
        tags: HashMap<String, String>,
    ) -> String {
        let plan_id = Uuid::new_v4().to_string().replace('-', "");
        let arn =
            format!("arn:aws:guardduty:{region}:{account_id}:malware-protection-plan/{plan_id}");
        let plan = MalwareProtectionPlan {
            plan_id: plan_id.clone(),
            arn,
            role: role.to_string(),
            protected_resource: MalwareProtectionPlanResource {
                s3_bucket_name: s3_bucket_name.map(|s| s.to_string()),
                s3_object_prefixes,
            },
            actions,
            status: "ACTIVE".to_string(),
            created_at: now_epoch(),
            tags,
        };
        self.malware_protection_plans.insert(plan_id.clone(), plan);
        plan_id
    }

    pub fn get_malware_protection_plan(
        &self,
        plan_id: &str,
    ) -> Result<&MalwareProtectionPlan, GuardDutyError> {
        self.malware_protection_plans.get(plan_id).ok_or_else(|| {
            GuardDutyError::MalwareProtectionPlanNotFound {
                plan_id: plan_id.to_string(),
            }
        })
    }

    pub fn list_malware_protection_plans(&self) -> Vec<&MalwareProtectionPlan> {
        self.malware_protection_plans.values().collect()
    }

    pub fn update_malware_protection_plan(
        &mut self,
        plan_id: &str,
        role: Option<&str>,
        tagging_status: Option<&str>,
    ) -> Result<(), GuardDutyError> {
        let plan = self
            .malware_protection_plans
            .get_mut(plan_id)
            .ok_or_else(|| GuardDutyError::MalwareProtectionPlanNotFound {
                plan_id: plan_id.to_string(),
            })?;

        if let Some(r) = role {
            plan.role = r.to_string();
        }
        if let Some(ts) = tagging_status {
            plan.actions = Some(MalwareProtectionPlanActions {
                tagging_status: Some(ts.to_string()),
            });
        }
        Ok(())
    }

    pub fn delete_malware_protection_plan(&mut self, plan_id: &str) -> Result<(), GuardDutyError> {
        if self.malware_protection_plans.remove(plan_id).is_none() {
            return Err(GuardDutyError::MalwareProtectionPlanNotFound {
                plan_id: plan_id.to_string(),
            });
        }
        Ok(())
    }

    // ── Invitations ───────────────────────────────────────────────────

    pub fn create_invitation(&mut self, account_id: &str) -> String {
        let invitation_id = Uuid::new_v4().to_string().replace('-', "");
        let now = Utc::now().to_rfc3339();
        self.invitations.insert(
            account_id.to_string(),
            Invitation {
                account_id: account_id.to_string(),
                invitation_id: invitation_id.clone(),
                invited_at: now,
                relationship_status: "Invited".to_string(),
            },
        );
        invitation_id
    }

    pub fn list_invitations(&self) -> Vec<&Invitation> {
        self.invitations.values().collect()
    }

    pub fn get_invitations_count(&self) -> usize {
        self.invitations.len()
    }

    pub fn decline_invitations(&mut self, account_ids: &[String]) {
        for id in account_ids {
            self.invitations.remove(id);
        }
    }

    pub fn delete_invitations(&mut self, account_ids: &[String]) {
        for id in account_ids {
            self.invitations.remove(id);
        }
    }

    // ── Invitation / Administrator operations ────────────────────────

    pub fn accept_administrator_invitation(
        &mut self,
        detector_id: &str,
        administrator_id: &str,
        invitation_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.administrator_account_id = Some(administrator_id.to_string());

        // Mark the invitation as accepted if it exists.
        if let Some(inv) = self.invitations.get_mut(invitation_id) {
            inv.relationship_status = "Enabled".to_string();
        }

        Ok(())
    }

    pub fn accept_invitation(
        &mut self,
        detector_id: &str,
        master_id: &str,
        invitation_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.master_account_id = Some(master_id.to_string());

        if let Some(inv) = self.invitations.get_mut(invitation_id) {
            inv.relationship_status = "Enabled".to_string();
        }

        Ok(())
    }

    pub fn disassociate_from_administrator_account(
        &mut self,
        detector_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.administrator_account_id = None;
        Ok(())
    }

    pub fn disassociate_from_master_account(
        &mut self,
        detector_id: &str,
    ) -> Result<(), GuardDutyError> {
        let detector = self
            .detectors
            .get_mut(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        detector.master_account_id = None;
        Ok(())
    }

    pub fn get_master_account_for_detector(
        &self,
        detector_id: &str,
    ) -> Result<Option<&str>, GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(detector.master_account_id.as_deref())
    }

    // ── Coverage operations ──────────────────────────────────────────

    pub fn list_coverage_records(
        &self,
        detector_id: &str,
    ) -> Result<&[CoverageRecord], GuardDutyError> {
        let detector = self
            .detectors
            .get(detector_id)
            .ok_or_else(Self::detector_not_found)?;

        Ok(&detector.coverage_records)
    }

    // ── Malware scan operations ──────────────────────────────────────

    pub fn create_malware_scan_record(&mut self) -> String {
        let scan_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        self.malware_scans.insert(
            scan_id.clone(),
            MalwareScanRecord {
                scan_id: scan_id.clone(),
                status: "COMPLETED".to_string(),
                created_at: now,
            },
        );
        scan_id
    }
}
