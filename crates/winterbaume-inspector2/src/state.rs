use std::collections::{HashMap, HashSet};

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct Inspector2State {
    /// Per-account enabled resource types. Inspector2's `Enable` /
    /// `Disable` / `BatchGetAccountStatus` are inherently cross-account ( the
    /// caller passes `accountIds` and each entry is operated on independently ),
    /// so the set of enabled types is keyed by account id rather than scoped
    /// to a single calling account.
    pub enabled_resource_types: HashMap<String, HashSet<String>>,
    /// Findings stored for this account/region.
    pub findings: Vec<Finding>,
    /// Associated member accounts: account_id -> relationship_status
    pub members: HashMap<String, MemberRecord>,
    /// Filters: filter_arn -> InspectorFilter
    pub filters: HashMap<String, InspectorFilter>,
    /// Delegated admin accounts: account_id -> DelegatedAdminRecord
    pub delegated_admin_accounts: HashMap<String, DelegatedAdminRecord>,
    /// Organization auto-enable configuration
    pub auto_enable: Option<AutoEnableConfig>,
    /// Tags for resources: resource_arn -> tags
    pub resource_tags: HashMap<String, HashMap<String, String>>,
    /// EC2 deep inspection configuration
    pub ec2_deep_inspection: Ec2DeepInspectionConfig,
    /// Org EC2 deep inspection configuration
    pub org_ec2_deep_inspection_package_paths: Vec<String>,
    /// Scanner configuration (ec2 scan mode, ecr rescan)
    pub ec2_scan_mode: String,
    pub ecr_rescan_duration: String,
    /// Encryption key (KMS key id, default is AWS-managed)
    pub encryption_key: String,
    /// Findings reports: report_id -> FindingsReportRecord
    pub findings_reports: HashMap<String, FindingsReportRecord>,
    /// SBOM exports: report_id -> SbomExportRecord
    pub sbom_exports: HashMap<String, SbomExportRecord>,
    /// CIS scan configurations: scan_configuration_arn -> CisScanConfigRecord
    pub cis_scan_configs: HashMap<String, CisScanConfigRecord>,
    /// Code security integrations: integration_arn -> CodeSecurityIntegration
    pub code_security_integrations: HashMap<String, CodeSecurityIntegration>,
    /// Code security scan configurations: scan_config_arn -> CodeSecurityScanConfig
    pub code_security_scan_configs: HashMap<String, CodeSecurityScanConfig>,
    /// Code security scan config associations: (scan_config_arn, resource_id) -> association
    pub code_security_scan_config_associations:
        HashMap<(String, String), CodeSecurityScanConfigAssociation>,
    /// Per-member EC2 deep inspection status: account_id -> status
    pub member_ec2_deep_inspection_status: HashMap<String, MemberEc2DeepInspectionStatus>,
}

#[derive(Debug, thiserror::Error)]
pub enum Inspector2Error {
    #[error("resourceTypes must not be empty")]
    ValidationResourceTypesEmpty,
    #[error("Invalid resource type: {0}")]
    ValidationInvalidResourceType(String),
    #[error("accountId must not be empty")]
    ValidationAccountIdEmpty,
    #[error("name must not be empty")]
    ValidationNameEmpty,
    #[error("delegatedAdminAccountId must not be empty")]
    ValidationDelegatedAdminAccountIdEmpty,
    #[error("Filter not found: {0}")]
    FilterNotFound(String),
    #[error("Delegated admin account not found: {0}")]
    DelegatedAdminAccountNotFound(String),
    #[error("Member account not found: {0}")]
    MemberAccountNotFound(String),
    #[error("Report not found: {0}")]
    ReportNotFound(String),
    #[error("SBOM export not found: {0}")]
    SbomExportNotFound(String),
    #[error("CIS scan configuration not found: {0}")]
    CisScanConfigurationNotFound(String),
    #[error("Code security integration not found: {0}")]
    CodeSecurityIntegrationNotFound(String),
    #[error("Code security scan configuration not found: {0}")]
    CodeSecurityScanConfigNotFound(String),
}

impl Inspector2State {
    /// Enable scanning for the given resource types.
    /// Returns the resource status map after enabling.
    pub fn enable(
        &mut self,
        resource_types: &[String],
        account_id: &str,
    ) -> Result<AccountEnableResult, Inspector2Error> {
        if resource_types.is_empty() {
            return Err(Inspector2Error::ValidationResourceTypesEmpty);
        }

        let entry = self
            .enabled_resource_types
            .entry(account_id.to_string())
            .or_default();
        for rt in resource_types {
            if ResourceType::from_str_value(rt).is_none() {
                return Err(Inspector2Error::ValidationInvalidResourceType(rt.clone()));
            }
            entry.insert(rt.clone());
        }

        Ok(AccountEnableResult {
            account_id: account_id.to_string(),
            resource_status: self.build_resource_status(account_id),
            status: self.compute_account_status(account_id),
        })
    }

    /// Disable scanning for the given resource types.
    pub fn disable(
        &mut self,
        resource_types: &[String],
        account_id: &str,
    ) -> Result<AccountEnableResult, Inspector2Error> {
        if let Some(entry) = self.enabled_resource_types.get_mut(account_id) {
            for rt in resource_types {
                if ResourceType::from_str_value(rt).is_none() {
                    return Err(Inspector2Error::ValidationInvalidResourceType(rt.clone()));
                }
                entry.remove(rt);
            }
        } else {
            for rt in resource_types {
                if ResourceType::from_str_value(rt).is_none() {
                    return Err(Inspector2Error::ValidationInvalidResourceType(rt.clone()));
                }
            }
        }

        Ok(AccountEnableResult {
            account_id: account_id.to_string(),
            resource_status: self.build_resource_status(account_id),
            status: self.compute_account_status(account_id),
        })
    }

    /// Get the account status.
    pub fn get_account_status(&self, account_id: &str) -> AccountEnableResult {
        AccountEnableResult {
            account_id: account_id.to_string(),
            resource_status: self.build_resource_status(account_id),
            status: self.compute_account_status(account_id),
        }
    }

    /// List findings, with optional max_results and pagination token.
    pub fn list_findings(
        &self,
        max_results: Option<usize>,
        _next_token: Option<&str>,
    ) -> (Vec<&Finding>, Option<String>) {
        let limit = max_results.unwrap_or(100).min(100);
        let results: Vec<&Finding> = self.findings.iter().take(limit).collect();
        // Simple mock: no real pagination
        (results, None)
    }

    /// Associate a member account.
    pub fn associate_member(&mut self, account_id: &str) -> Result<String, Inspector2Error> {
        if account_id.is_empty() {
            return Err(Inspector2Error::ValidationAccountIdEmpty);
        }
        self.members.insert(
            account_id.to_string(),
            MemberRecord {
                account_id: account_id.to_string(),
                relationship_status: "ENABLED".to_string(),
                updated_at: chrono::Utc::now(),
            },
        );
        Ok(account_id.to_string())
    }

    /// Create a filter.
    pub fn create_filter(
        &mut self,
        name: &str,
        action: &str,
        description: Option<&str>,
        owner_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> Result<InspectorFilter, Inspector2Error> {
        if name.is_empty() {
            return Err(Inspector2Error::ValidationNameEmpty);
        }
        let arn = format!(
            "arn:aws:inspector2:{region}:{owner_id}:owner/{owner_id}/filter/{}",
            uuid::Uuid::new_v4()
        );
        let now = chrono::Utc::now();
        let filter = InspectorFilter {
            arn: arn.clone(),
            name: name.to_string(),
            action: action.to_string(),
            description: description.map(String::from),
            created_at: now,
            updated_at: now,
            owner_id: owner_id.to_string(),
            tags: tags.clone(),
        };
        self.filters.insert(arn.clone(), filter.clone());
        if let Some(t) = tags {
            self.resource_tags.insert(arn.clone(), t);
        }
        Ok(filter)
    }

    /// Delete a filter by ARN.
    pub fn delete_filter(&mut self, arn: &str) -> Result<String, Inspector2Error> {
        if self.filters.remove(arn).is_none() {
            return Err(Inspector2Error::FilterNotFound(arn.to_string()));
        }
        self.resource_tags.remove(arn);
        Ok(arn.to_string())
    }

    /// List filters.
    pub fn list_filters(&self) -> Vec<&InspectorFilter> {
        self.filters.values().collect()
    }

    /// Enable delegated admin account.
    pub fn enable_delegated_admin(&mut self, account_id: &str) -> Result<String, Inspector2Error> {
        if account_id.is_empty() {
            return Err(Inspector2Error::ValidationDelegatedAdminAccountIdEmpty);
        }
        self.delegated_admin_accounts.insert(
            account_id.to_string(),
            DelegatedAdminRecord {
                account_id: account_id.to_string(),
                status: "ENABLED".to_string(),
            },
        );
        Ok(account_id.to_string())
    }

    /// Disable delegated admin account.
    pub fn disable_delegated_admin(&mut self, account_id: &str) -> Result<String, Inspector2Error> {
        match self.delegated_admin_accounts.get_mut(account_id) {
            Some(record) => {
                record.status = "DISABLED".to_string();
                Ok(account_id.to_string())
            }
            None => Err(Inspector2Error::DelegatedAdminAccountNotFound(
                account_id.to_string(),
            )),
        }
    }

    /// List delegated admin accounts.
    pub fn list_delegated_admin_accounts(&self) -> Vec<&DelegatedAdminRecord> {
        self.delegated_admin_accounts.values().collect()
    }

    /// Tag a resource.
    pub fn tag_resource(
        &mut self,
        resource_arn: &str,
        tags: HashMap<String, String>,
    ) -> Result<(), Inspector2Error> {
        // If the ARN matches a filter, also update filter tags
        if let Some(filter) = self.filters.get_mut(resource_arn) {
            let filter_tags = filter.tags.get_or_insert_with(HashMap::new);
            filter_tags.extend(tags.clone());
        }
        let resource_tags = self
            .resource_tags
            .entry(resource_arn.to_string())
            .or_default();
        resource_tags.extend(tags);
        Ok(())
    }

    /// Untag a resource.
    pub fn untag_resource(
        &mut self,
        resource_arn: &str,
        tag_keys: &[String],
    ) -> Result<(), Inspector2Error> {
        if let Some(filter) = self.filters.get_mut(resource_arn) {
            if let Some(filter_tags) = filter.tags.as_mut() {
                for key in tag_keys {
                    filter_tags.remove(key);
                }
            }
        }
        if let Some(resource_tags) = self.resource_tags.get_mut(resource_arn) {
            for key in tag_keys {
                resource_tags.remove(key);
            }
        }
        Ok(())
    }

    /// Describe organization configuration.
    pub fn describe_organization_configuration(&self) -> OrganizationConfig {
        OrganizationConfig {
            auto_enable: self.auto_enable.clone(),
            max_account_limit_reached: false,
        }
    }

    /// Update organization configuration.
    pub fn update_organization_configuration(
        &mut self,
        auto_enable: AutoEnableConfig,
    ) -> AutoEnableConfig {
        self.auto_enable = Some(auto_enable.clone());
        auto_enable
    }

    /// Disassociate a member account.
    pub fn disassociate_member(&mut self, account_id: &str) -> Result<String, Inspector2Error> {
        if self.members.remove(account_id).is_none() {
            return Err(Inspector2Error::MemberAccountNotFound(
                account_id.to_string(),
            ));
        }
        Ok(account_id.to_string())
    }

    /// Get a specific member account.
    pub fn get_member(&self, account_id: &str) -> Result<&MemberRecord, Inspector2Error> {
        self.members
            .get(account_id)
            .ok_or_else(|| Inspector2Error::MemberAccountNotFound(account_id.to_string()))
    }

    /// List members.
    pub fn list_members(&self) -> Vec<&MemberRecord> {
        self.members.values().collect()
    }

    /// Get tags for a resource.
    pub fn list_tags_for_resource(&self, resource_arn: &str) -> HashMap<String, String> {
        self.resource_tags
            .get(resource_arn)
            .cloned()
            .unwrap_or_default()
    }

    fn build_resource_status(&self, account_id: &str) -> HashMap<String, String> {
        let all_types = ["EC2", "ECR", "LAMBDA", "LAMBDA_CODE", "CODE_REPOSITORY"];
        let enabled = self.enabled_resource_types.get(account_id);
        let mut status = HashMap::new();
        for rt in &all_types {
            let s = match enabled {
                Some(set) if set.contains(*rt) => "ENABLED",
                _ => "DISABLED",
            };
            status.insert(rt.to_string(), s.to_string());
        }
        status
    }

    fn compute_account_status(&self, account_id: &str) -> String {
        match self.enabled_resource_types.get(account_id) {
            Some(set) if !set.is_empty() => "ENABLED".to_string(),
            _ => "DISABLED".to_string(),
        }
    }

    // --- UpdateFilter ---
    pub fn update_filter(
        &mut self,
        filter_arn: &str,
        action: Option<&str>,
        description: Option<&str>,
        name: Option<&str>,
    ) -> Result<String, Inspector2Error> {
        let filter = self
            .filters
            .get_mut(filter_arn)
            .ok_or_else(|| Inspector2Error::FilterNotFound(filter_arn.to_string()))?;
        if let Some(a) = action {
            filter.action = a.to_string();
        }
        if let Some(d) = description {
            filter.description = Some(d.to_string());
        }
        if let Some(n) = name {
            filter.name = n.to_string();
        }
        filter.updated_at = chrono::Utc::now();
        Ok(filter_arn.to_string())
    }

    // --- GetConfiguration / UpdateConfiguration ---
    pub fn get_configuration(&self) -> (&str, &str) {
        // Returns (ec2_scan_mode, ecr_rescan_duration)
        (&self.ec2_scan_mode, &self.ecr_rescan_duration)
    }

    pub fn update_configuration(
        &mut self,
        ec2_scan_mode: Option<String>,
        ecr_rescan_duration: Option<String>,
    ) {
        if let Some(m) = ec2_scan_mode {
            self.ec2_scan_mode = m;
        }
        if let Some(d) = ecr_rescan_duration {
            self.ecr_rescan_duration = d;
        }
    }

    // --- GetDelegatedAdminAccount ---
    pub fn get_delegated_admin_account(&self) -> Option<(&str, &str)> {
        // Return the first enabled delegated admin account
        self.delegated_admin_accounts
            .values()
            .find(|r| r.status == "ENABLED")
            .map(|r| (r.account_id.as_str(), r.status.as_str()))
    }

    // --- GetEc2DeepInspectionConfiguration / UpdateEc2DeepInspectionConfiguration ---
    pub fn get_ec2_deep_inspection_configuration(&self) -> &Ec2DeepInspectionConfig {
        &self.ec2_deep_inspection
    }

    pub fn update_ec2_deep_inspection_configuration(
        &mut self,
        activate: Option<bool>,
        package_paths: Option<Vec<String>>,
    ) {
        if let Some(a) = activate {
            self.ec2_deep_inspection.activate_deep_inspection = a;
        }
        if let Some(p) = package_paths {
            self.ec2_deep_inspection.package_paths = p;
        }
    }

    // --- UpdateOrgEc2DeepInspectionConfiguration ---
    pub fn update_org_ec2_deep_inspection_configuration(&mut self, org_package_paths: Vec<String>) {
        self.org_ec2_deep_inspection_package_paths = org_package_paths;
    }

    // --- GetEncryptionKey / ResetEncryptionKey / UpdateEncryptionKey ---
    pub fn get_encryption_key(&self) -> &str {
        if self.encryption_key.is_empty() {
            "AWS_OWNED_KMS_KEY"
        } else {
            &self.encryption_key
        }
    }

    pub fn reset_encryption_key(&mut self) {
        self.encryption_key = String::new();
    }

    pub fn update_encryption_key(&mut self, kms_key_id: &str) {
        self.encryption_key = kms_key_id.to_string();
    }

    // --- CreateFindingsReport / GetFindingsReportStatus / CancelFindingsReport ---
    pub fn create_findings_report(
        &mut self,
        report_format: &str,
        s3_bucket_name: &str,
        s3_key_prefix: &str,
    ) -> String {
        let report_id = uuid::Uuid::new_v4().to_string();
        self.findings_reports.insert(
            report_id.clone(),
            FindingsReportRecord {
                report_id: report_id.clone(),
                status: "SUCCEEDED".to_string(),
                report_format: report_format.to_string(),
                s3_bucket_name: s3_bucket_name.to_string(),
                s3_key_prefix: s3_key_prefix.to_string(),
            },
        );
        report_id
    }

    pub fn get_findings_report_status(
        &self,
        report_id: Option<&str>,
    ) -> Option<&FindingsReportRecord> {
        if let Some(id) = report_id {
            self.findings_reports.get(id)
        } else {
            self.findings_reports.values().next()
        }
    }

    pub fn cancel_findings_report(&mut self, report_id: &str) -> Result<String, Inspector2Error> {
        let record = self
            .findings_reports
            .get_mut(report_id)
            .ok_or_else(|| Inspector2Error::ReportNotFound(report_id.to_string()))?;
        record.status = "CANCELLED".to_string();
        Ok(report_id.to_string())
    }

    // --- CreateSbomExport / GetSbomExport / CancelSbomExport ---
    pub fn create_sbom_export(
        &mut self,
        report_format: &str,
        s3_bucket_name: &str,
        s3_key_prefix: &str,
    ) -> String {
        let report_id = uuid::Uuid::new_v4().to_string();
        self.sbom_exports.insert(
            report_id.clone(),
            SbomExportRecord {
                report_id: report_id.clone(),
                status: "SUCCEEDED".to_string(),
                report_format: report_format.to_string(),
                s3_bucket_name: s3_bucket_name.to_string(),
                s3_key_prefix: s3_key_prefix.to_string(),
            },
        );
        report_id
    }

    pub fn get_sbom_export(&self, report_id: &str) -> Option<&SbomExportRecord> {
        self.sbom_exports.get(report_id)
    }

    pub fn cancel_sbom_export(&mut self, report_id: &str) -> Result<String, Inspector2Error> {
        let record = self
            .sbom_exports
            .get_mut(report_id)
            .ok_or_else(|| Inspector2Error::SbomExportNotFound(report_id.to_string()))?;
        record.status = "CANCELLED".to_string();
        Ok(report_id.to_string())
    }

    // --- CIS scan configuration CRUD ---
    pub fn create_cis_scan_configuration(
        &mut self,
        scan_name: &str,
        owner_id: &str,
        region: &str,
        tags: Option<HashMap<String, String>>,
    ) -> String {
        let arn = format!(
            "arn:aws:inspector2:{region}:{owner_id}:owner/{owner_id}/cis-scan-configuration/{}",
            uuid::Uuid::new_v4()
        );
        self.cis_scan_configs.insert(
            arn.clone(),
            CisScanConfigRecord {
                scan_configuration_arn: arn.clone(),
                scan_name: scan_name.to_string(),
                owner_id: owner_id.to_string(),
                tags,
            },
        );
        arn
    }

    pub fn delete_cis_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
    ) -> Result<String, Inspector2Error> {
        if self
            .cis_scan_configs
            .remove(scan_configuration_arn)
            .is_none()
        {
            return Err(Inspector2Error::CisScanConfigurationNotFound(
                scan_configuration_arn.to_string(),
            ));
        }
        Ok(scan_configuration_arn.to_string())
    }

    pub fn list_cis_scan_configurations(&self) -> Vec<&CisScanConfigRecord> {
        self.cis_scan_configs.values().collect()
    }

    pub fn update_cis_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
        scan_name: Option<&str>,
    ) -> Result<String, Inspector2Error> {
        let config = self
            .cis_scan_configs
            .get_mut(scan_configuration_arn)
            .ok_or_else(|| {
                Inspector2Error::CisScanConfigurationNotFound(scan_configuration_arn.to_string())
            })?;
        if let Some(n) = scan_name {
            config.scan_name = n.to_string();
        }
        Ok(scan_configuration_arn.to_string())
    }

    // -------------------------------------------------------------------------
    // Code Security Integrations
    // -------------------------------------------------------------------------

    pub fn create_code_security_integration(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> CodeSecurityIntegration {
        let integration_arn = format!(
            "arn:aws:inspector2:{region}:{account_id}:integration/{}",
            uuid::Uuid::new_v4()
        );
        let now = Utc::now();
        let integration = CodeSecurityIntegration {
            integration_arn: integration_arn.clone(),
            name: name.to_string(),
            status: "ACTIVE".to_string(),
            created_at: now,
            updated_at: now,
        };
        self.code_security_integrations
            .insert(integration_arn, integration.clone());
        integration
    }

    pub fn get_code_security_integration(
        &self,
        integration_arn: &str,
    ) -> Result<&CodeSecurityIntegration, Inspector2Error> {
        self.code_security_integrations
            .get(integration_arn)
            .ok_or_else(|| {
                Inspector2Error::CodeSecurityIntegrationNotFound(integration_arn.to_string())
            })
    }

    pub fn delete_code_security_integration(
        &mut self,
        integration_arn: &str,
    ) -> Result<(), Inspector2Error> {
        if self
            .code_security_integrations
            .remove(integration_arn)
            .is_none()
        {
            return Err(Inspector2Error::CodeSecurityIntegrationNotFound(
                integration_arn.to_string(),
            ));
        }
        Ok(())
    }

    pub fn list_code_security_integrations(&self) -> Vec<&CodeSecurityIntegration> {
        self.code_security_integrations.values().collect()
    }

    pub fn update_code_security_integration(
        &mut self,
        integration_arn: &str,
    ) -> Result<&CodeSecurityIntegration, Inspector2Error> {
        let integration = self
            .code_security_integrations
            .get_mut(integration_arn)
            .ok_or_else(|| {
                Inspector2Error::CodeSecurityIntegrationNotFound(integration_arn.to_string())
            })?;
        integration.updated_at = Utc::now();
        Ok(integration)
    }

    // -------------------------------------------------------------------------
    // Code Security Scan Configurations
    // -------------------------------------------------------------------------

    pub fn create_code_security_scan_configuration(
        &mut self,
        name: &str,
        account_id: &str,
        region: &str,
    ) -> CodeSecurityScanConfig {
        let scan_configuration_arn = format!(
            "arn:aws:inspector2:{region}:{account_id}:code-scan-config/{}",
            uuid::Uuid::new_v4()
        );
        let now = Utc::now();
        let config = CodeSecurityScanConfig {
            scan_configuration_arn: scan_configuration_arn.clone(),
            name: name.to_string(),
            created_at: now,
            updated_at: now,
        };
        self.code_security_scan_configs
            .insert(scan_configuration_arn, config.clone());
        config
    }

    pub fn get_code_security_scan_configuration(
        &self,
        scan_configuration_arn: &str,
    ) -> Result<&CodeSecurityScanConfig, Inspector2Error> {
        self.code_security_scan_configs
            .get(scan_configuration_arn)
            .ok_or_else(|| {
                Inspector2Error::CodeSecurityScanConfigNotFound(scan_configuration_arn.to_string())
            })
    }

    pub fn delete_code_security_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
    ) -> Result<(), Inspector2Error> {
        if self
            .code_security_scan_configs
            .remove(scan_configuration_arn)
            .is_none()
        {
            return Err(Inspector2Error::CodeSecurityScanConfigNotFound(
                scan_configuration_arn.to_string(),
            ));
        }
        // Also remove any associations
        self.code_security_scan_config_associations
            .retain(|(arn, _), _| arn != scan_configuration_arn);
        Ok(())
    }

    pub fn list_code_security_scan_configurations(&self) -> Vec<&CodeSecurityScanConfig> {
        self.code_security_scan_configs.values().collect()
    }

    pub fn update_code_security_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
    ) -> Result<&CodeSecurityScanConfig, Inspector2Error> {
        let config = self
            .code_security_scan_configs
            .get_mut(scan_configuration_arn)
            .ok_or_else(|| {
                Inspector2Error::CodeSecurityScanConfigNotFound(scan_configuration_arn.to_string())
            })?;
        config.updated_at = Utc::now();
        Ok(config)
    }

    // -------------------------------------------------------------------------
    // Code Security Scan Configuration Associations
    // -------------------------------------------------------------------------

    pub fn batch_associate_code_security_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
        resource_ids: &[String],
    ) -> Vec<String> {
        let mut failed = Vec::new();
        let now = Utc::now();
        for resource_id in resource_ids {
            let key = (scan_configuration_arn.to_string(), resource_id.to_string());
            self.code_security_scan_config_associations.insert(
                key,
                CodeSecurityScanConfigAssociation {
                    scan_configuration_arn: scan_configuration_arn.to_string(),
                    resource_id: resource_id.clone(),
                    status: "ASSOCIATED".to_string(),
                    updated_at: now,
                },
            );
        }
        failed
    }

    pub fn batch_disassociate_code_security_scan_configuration(
        &mut self,
        scan_configuration_arn: &str,
        resource_ids: &[String],
    ) -> Vec<String> {
        let mut failed = Vec::new();
        for resource_id in resource_ids {
            let key = (scan_configuration_arn.to_string(), resource_id.to_string());
            if self
                .code_security_scan_config_associations
                .remove(&key)
                .is_none()
            {
                failed.push(resource_id.clone());
            }
        }
        failed
    }

    pub fn list_code_security_scan_configuration_associations(
        &self,
    ) -> Vec<&CodeSecurityScanConfigAssociation> {
        self.code_security_scan_config_associations
            .values()
            .collect()
    }

    // -------------------------------------------------------------------------
    // Member EC2 Deep Inspection Status
    // -------------------------------------------------------------------------

    pub fn batch_get_member_ec2_deep_inspection_status(
        &self,
        account_ids: &[String],
    ) -> Vec<&MemberEc2DeepInspectionStatus> {
        account_ids
            .iter()
            .filter_map(|id| self.member_ec2_deep_inspection_status.get(id))
            .collect()
    }

    pub fn batch_update_member_ec2_deep_inspection_status(
        &mut self,
        updates: &[(String, bool)],
    ) -> Vec<String> {
        let mut account_ids = Vec::new();
        for (account_id, activate) in updates {
            let status = if *activate {
                "ACTIVATED"
            } else {
                "DEACTIVATED"
            }
            .to_string();
            self.member_ec2_deep_inspection_status.insert(
                account_id.clone(),
                MemberEc2DeepInspectionStatus {
                    account_id: account_id.clone(),
                    activate_deep_inspection: *activate,
                    status,
                },
            );
            account_ids.push(account_id.clone());
        }
        account_ids
    }
}

/// EC2 deep inspection configuration
#[derive(Debug, Clone, Default)]
pub struct Ec2DeepInspectionConfig {
    pub activate_deep_inspection: bool,
    pub package_paths: Vec<String>,
}

/// Record for a findings report.
#[derive(Debug, Clone)]
pub struct FindingsReportRecord {
    pub report_id: String,
    pub status: String,
    pub report_format: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
}

/// Record for an SBOM export.
#[derive(Debug, Clone)]
pub struct SbomExportRecord {
    pub report_id: String,
    pub status: String,
    pub report_format: String,
    pub s3_bucket_name: String,
    pub s3_key_prefix: String,
}

/// Record for a CIS scan configuration.
#[derive(Debug, Clone)]
pub struct CisScanConfigRecord {
    pub scan_configuration_arn: String,
    pub scan_name: String,
    pub owner_id: String,
    pub tags: Option<HashMap<String, String>>,
}

/// Result of an enable/disable/status operation for an account.
#[derive(Debug)]
pub struct AccountEnableResult {
    pub account_id: String,
    pub resource_status: HashMap<String, String>,
    pub status: String,
}
