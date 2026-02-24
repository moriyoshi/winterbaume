use std::collections::HashMap;

use chrono::Utc;

use crate::types::*;

#[derive(Debug, Default)]
pub struct Macie2State {
    pub session: Option<MacieSession>,
    /// Members keyed by account_id.
    pub members: HashMap<String, MacieMember>,
    /// Pending invitations received (keyed by sender account_id).
    pub invitations: Vec<MacieInvitation>,
    /// Organization admin accounts.
    pub admin_accounts: Vec<MacieAdminAccount>,
    /// The administrator account relationship for this account.
    pub administrator: Option<MacieAdministrator>,
    /// Per-resource tags keyed by resource ARN.
    pub resource_tags: HashMap<String, HashMap<String, String>>,

    // ── New resource collections ──────────────────────────────────────────────
    /// Custom data identifiers keyed by id.
    pub custom_data_identifiers: HashMap<String, MacieCustomDataIdentifier>,
    /// Allow lists keyed by id.
    pub allow_lists: HashMap<String, MacieAllowList>,
    /// Findings filters keyed by id.
    pub findings_filters: HashMap<String, MacieFindingsFilter>,
    /// Classification jobs keyed by job_id.
    pub classification_jobs: HashMap<String, MacieClassificationJob>,
    /// Sensitivity inspection templates keyed by id.
    pub sensitivity_inspection_templates: HashMap<String, MacieSensitivityInspectionTemplate>,
    /// Automated discovery configuration (optional — None means never configured).
    pub automated_discovery_config: Option<MacieAutomatedDiscoveryConfig>,
    /// Per-account automated discovery overrides keyed by account_id.
    pub automated_discovery_accounts: HashMap<String, MacieAutomatedDiscoveryAccount>,
    /// Reveal configuration.
    pub reveal_config: Option<MacieRevealConfig>,
    /// Organisation configuration.
    pub org_config: Option<MacieOrgConfig>,
    /// Classification export configuration.
    pub classification_export_config: Option<MacieClassificationExportConfig>,
    /// Findings publication configuration.
    pub findings_publication_config: Option<MacieFindingsPublicationConfig>,
    /// Findings keyed by finding ID.
    pub findings: HashMap<String, MacieFinding>,
}

#[derive(Debug, thiserror::Error)]
pub enum Macie2Error {
    #[error("Macie has already been enabled for this account.")]
    MacieAlreadyEnabled,
    #[error("Macie is not enabled for this account.")]
    MacieNotEnabled,
    #[error("The invitation was not found.")]
    InvitationNotFound,
    #[error("The member with accountId {0} was not found.")]
    MemberNotFound(String),
    #[error("The member already exists.")]
    MemberAlreadyExists,
    #[error("The account is already a delegated administrator.")]
    AdminAlreadyDelegated,
    #[error("The resource with id {0} was not found.")]
    ResourceNotFound(String),
}

impl Macie2State {
    pub fn enable_macie(
        &mut self,
        status: Option<&str>,
        finding_publishing_frequency: Option<&str>,
        account_id: &str,
        _region: &str,
    ) -> Result<(), Macie2Error> {
        if self.session.is_some() {
            return Err(Macie2Error::MacieAlreadyEnabled);
        }

        let now = Utc::now();
        let session = MacieSession {
            status: status.unwrap_or("ENABLED").to_string(),
            finding_publishing_frequency: finding_publishing_frequency
                .unwrap_or("SIX_HOURS")
                .to_string(),
            created_at: now,
            updated_at: now,
            service_role: format!(
                "arn:aws:iam::{}:role/aws-service-role/macie.amazonaws.com/AWSServiceRoleForAmazonMacie",
                account_id
            ),
        };

        self.session = Some(session);
        Ok(())
    }

    pub fn get_macie_session(&self) -> Result<&MacieSession, Macie2Error> {
        self.session.as_ref().ok_or(Macie2Error::MacieNotEnabled)
    }

    pub fn update_macie_session(
        &mut self,
        status: Option<&str>,
        finding_publishing_frequency: Option<&str>,
    ) -> Result<(), Macie2Error> {
        let session = self.session.as_mut().ok_or(Macie2Error::MacieNotEnabled)?;
        if let Some(s) = status {
            session.status = s.to_string();
        }
        if let Some(f) = finding_publishing_frequency {
            session.finding_publishing_frequency = f.to_string();
        }
        session.updated_at = Utc::now();
        Ok(())
    }

    pub fn disable_macie(&mut self) -> Result<(), Macie2Error> {
        if self.session.is_none() {
            return Err(Macie2Error::MacieNotEnabled);
        }
        self.session = None;
        Ok(())
    }

    pub fn list_findings(&self) -> Result<Vec<String>, Macie2Error> {
        if self.session.is_none() {
            return Err(Macie2Error::MacieNotEnabled);
        }
        // Mock returns empty findings list
        Ok(Vec::new())
    }

    fn require_enabled(&self) -> Result<(), Macie2Error> {
        if self.session.is_none() {
            return Err(Macie2Error::MacieNotEnabled);
        }
        Ok(())
    }

    pub fn create_invitations(
        &mut self,
        account_ids: &[String],
    ) -> Result<Vec<String>, Macie2Error> {
        self.require_enabled()?;
        let now = Utc::now();
        let mut unprocessed = Vec::new();
        for account_id in account_ids {
            if let Some(member) = self.members.get_mut(account_id) {
                member.relationship_status = "Invited".to_string();
                member.invited_at = Some(now);
                member.updated_at = now;
            } else {
                unprocessed.push(account_id.clone());
            }
        }
        Ok(unprocessed)
    }

    pub fn accept_invitation(
        &mut self,
        administrator_account_id: &str,
        invitation_id: &str,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        // Find and remove the matching invitation
        let pos = self.invitations.iter().position(|inv| {
            inv.account_id == administrator_account_id && inv.invitation_id == invitation_id
        });
        match pos {
            Some(idx) => {
                let inv = self.invitations.remove(idx);
                self.administrator = Some(MacieAdministrator {
                    account_id: inv.account_id,
                    invitation_id: inv.invitation_id,
                    invited_at: inv.invited_at,
                    relationship_status: "Enabled".to_string(),
                });
                Ok(())
            }
            None => Err(Macie2Error::InvitationNotFound),
        }
    }

    pub fn decline_invitations(
        &mut self,
        account_ids: &[String],
    ) -> Result<Vec<String>, Macie2Error> {
        self.require_enabled()?;
        let mut unprocessed = Vec::new();
        for account_id in account_ids {
            let pos = self
                .invitations
                .iter()
                .position(|inv| inv.account_id == *account_id);
            match pos {
                Some(idx) => {
                    self.invitations.remove(idx);
                }
                None => {
                    unprocessed.push(account_id.clone());
                }
            }
        }
        Ok(unprocessed)
    }

    pub fn delete_invitations(
        &mut self,
        account_ids: &[String],
    ) -> Result<Vec<String>, Macie2Error> {
        self.require_enabled()?;
        let mut unprocessed = Vec::new();
        for account_id in account_ids {
            let pos = self
                .invitations
                .iter()
                .position(|inv| inv.account_id == *account_id);
            match pos {
                Some(idx) => {
                    self.invitations.remove(idx);
                }
                None => {
                    unprocessed.push(account_id.clone());
                }
            }
        }
        Ok(unprocessed)
    }

    pub fn delete_member(&mut self, member_id: &str) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        if self.members.remove(member_id).is_some() {
            Ok(())
        } else {
            Err(Macie2Error::MemberNotFound(member_id.to_string()))
        }
    }

    pub fn disassociate_member(&mut self, member_id: &str) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        if let Some(member) = self.members.get_mut(member_id) {
            member.relationship_status = "Removed".to_string();
            member.updated_at = Utc::now();
            Ok(())
        } else {
            Err(Macie2Error::MemberNotFound(member_id.to_string()))
        }
    }

    pub fn enable_organization_admin_account(
        &mut self,
        admin_account_id: &str,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        // Check if already an admin
        if self
            .admin_accounts
            .iter()
            .any(|a| a.account_id == admin_account_id)
        {
            return Err(Macie2Error::AdminAlreadyDelegated);
        }
        self.admin_accounts.push(MacieAdminAccount {
            account_id: admin_account_id.to_string(),
            status: "ENABLED".to_string(),
        });
        Ok(())
    }

    pub fn disable_organization_admin_account(
        &mut self,
        admin_account_id: &str,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.admin_accounts
            .retain(|a| a.account_id != admin_account_id);
        Ok(())
    }

    pub fn get_administrator_account(&self) -> Result<Option<&MacieAdministrator>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.administrator.as_ref())
    }

    pub fn disassociate_from_administrator_account(&mut self) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.administrator = None;
        Ok(())
    }

    pub fn list_invitations(&self) -> Result<&[MacieInvitation], Macie2Error> {
        self.require_enabled()?;
        Ok(&self.invitations)
    }

    pub fn list_members(&self) -> Result<Vec<&MacieMember>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.members.values().collect())
    }

    pub fn list_organization_admin_accounts(&self) -> Result<&[MacieAdminAccount], Macie2Error> {
        self.require_enabled()?;
        Ok(&self.admin_accounts)
    }

    /// Add a member (used by CreateMember, but also useful for test setup).
    pub fn add_member(&mut self, account_id: &str, email: &str) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        if self.members.contains_key(account_id) {
            return Err(Macie2Error::MemberAlreadyExists);
        }
        let now = Utc::now();
        let arn = format!(
            "arn:aws:macie2:us-east-1:{}:member/{}",
            account_id, account_id
        );
        self.members.insert(
            account_id.to_string(),
            MacieMember {
                account_id: account_id.to_string(),
                email: email.to_string(),
                relationship_status: "Created".to_string(),
                invited_at: None,
                updated_at: now,
            },
        );
        Ok(arn)
    }

    /// Update a member's session status.
    pub fn update_member_session(
        &mut self,
        member_id: &str,
        status: &str,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        if let Some(member) = self.members.get_mut(member_id) {
            member.relationship_status = status.to_string();
            member.updated_at = Utc::now();
            Ok(())
        } else {
            Err(Macie2Error::MemberNotFound(member_id.to_string()))
        }
    }

    /// Add an invitation (for test setup).
    pub fn add_invitation(&mut self, sender_account_id: &str, invitation_id: &str) {
        let now = Utc::now();
        self.invitations.push(MacieInvitation {
            account_id: sender_account_id.to_string(),
            invitation_id: invitation_id.to_string(),
            invited_at: now,
            relationship_status: "Invited".to_string(),
        });
    }

    // ── Organisation configuration ────────────────────────────────────────────

    pub fn get_org_config(&self) -> Result<Option<&MacieOrgConfig>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.org_config.as_ref())
    }

    pub fn update_org_config(&mut self, auto_enable: bool) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        let cfg = self.org_config.get_or_insert(MacieOrgConfig {
            auto_enable: false,
            max_account_limit_reached: false,
        });
        cfg.auto_enable = auto_enable;
        Ok(())
    }

    // ── Resource tag operations ───────────────────────────────────────────────

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

    // ── Custom data identifiers ───────────────────────────────────────────────

    fn next_id(prefix: &str) -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        format!("{}-{:08x}", prefix, nanos)
    }

    pub fn create_custom_data_identifier(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        description: Option<String>,
        regex: String,
        keywords: Vec<String>,
        ignore_words: Vec<String>,
        maximum_match_distance: i32,
        severity_levels: Vec<MacieSeverityLevel>,
        tags: HashMap<String, String>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let id = Self::next_id("cdi");
        let arn = format!(
            "arn:aws:macie2:{}:{}:custom-data-identifier/{}",
            region, account_id, id
        );
        let cdi = MacieCustomDataIdentifier {
            id: id.clone(),
            arn: arn.clone(),
            name,
            description,
            regex,
            keywords,
            ignore_words,
            maximum_match_distance,
            severity_levels,
            tags: tags.clone(),
            created_at: Utc::now(),
            deleted: false,
        };
        self.custom_data_identifiers.insert(id.clone(), cdi);
        self.resource_tags.insert(arn, tags);
        Ok(id)
    }

    pub fn get_custom_data_identifier(
        &self,
        id: &str,
    ) -> Result<&MacieCustomDataIdentifier, Macie2Error> {
        self.require_enabled()?;
        self.custom_data_identifiers
            .get(id)
            .filter(|c| !c.deleted)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))
    }

    pub fn delete_custom_data_identifier(&mut self, id: &str) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        let cdi = self
            .custom_data_identifiers
            .get_mut(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))?;
        cdi.deleted = true;
        Ok(())
    }

    pub fn list_custom_data_identifiers(
        &self,
    ) -> Result<Vec<&MacieCustomDataIdentifier>, Macie2Error> {
        self.require_enabled()?;
        Ok(self
            .custom_data_identifiers
            .values()
            .filter(|c| !c.deleted)
            .collect())
    }

    pub fn batch_get_custom_data_identifiers(
        &self,
        ids: &[String],
    ) -> Result<(Vec<&MacieCustomDataIdentifier>, Vec<String>), Macie2Error> {
        self.require_enabled()?;
        let mut found = Vec::new();
        let mut not_found = Vec::new();
        for id in ids {
            match self.custom_data_identifiers.get(id.as_str()) {
                Some(c) => found.push(c),
                None => not_found.push(id.clone()),
            }
        }
        Ok((found, not_found))
    }

    // ── Allow lists ───────────────────────────────────────────────────────────

    pub fn create_allow_list(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        description: Option<String>,
        criteria: MacieAllowListCriteria,
        tags: HashMap<String, String>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let id = Self::next_id("al");
        let arn = format!("arn:aws:macie2:{}:{}:allow-list/{}", region, account_id, id);
        let now = Utc::now();
        let al = MacieAllowList {
            id: id.clone(),
            arn: arn.clone(),
            name,
            description,
            criteria,
            tags: tags.clone(),
            created_at: now,
            updated_at: now,
            status_code: "OK".to_string(),
        };
        self.allow_lists.insert(id.clone(), al);
        self.resource_tags.insert(arn, tags);
        Ok(id)
    }

    pub fn get_allow_list(&self, id: &str) -> Result<&MacieAllowList, Macie2Error> {
        self.require_enabled()?;
        self.allow_lists
            .get(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))
    }

    pub fn update_allow_list(
        &mut self,
        id: &str,
        name: String,
        description: Option<String>,
        criteria: MacieAllowListCriteria,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let al = self
            .allow_lists
            .get_mut(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))?;
        al.name = name;
        al.description = description;
        al.criteria = criteria;
        al.updated_at = Utc::now();
        Ok(al.arn.clone())
    }

    pub fn delete_allow_list(&mut self, id: &str) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.allow_lists
            .remove(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))?;
        Ok(())
    }

    pub fn list_allow_lists(&self) -> Result<Vec<&MacieAllowList>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.allow_lists.values().collect())
    }

    // ── Findings filters ──────────────────────────────────────────────────────

    pub fn create_findings_filter(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        description: Option<String>,
        action: String,
        position: Option<i32>,
        finding_criteria: serde_json::Value,
        tags: HashMap<String, String>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let id = Self::next_id("ff");
        let arn = format!(
            "arn:aws:macie2:{}:{}:findings-filter/{}",
            region, account_id, id
        );
        let ff = MacieFindingsFilter {
            id: id.clone(),
            arn: arn.clone(),
            name,
            description,
            action,
            position: position.unwrap_or(1),
            finding_criteria,
            tags: tags.clone(),
        };
        self.findings_filters.insert(id.clone(), ff);
        self.resource_tags.insert(arn, tags);
        Ok(id)
    }

    pub fn get_findings_filter(&self, id: &str) -> Result<&MacieFindingsFilter, Macie2Error> {
        self.require_enabled()?;
        self.findings_filters
            .get(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))
    }

    pub fn update_findings_filter(
        &mut self,
        id: &str,
        name: Option<String>,
        description: Option<String>,
        action: Option<String>,
        position: Option<i32>,
        finding_criteria: Option<serde_json::Value>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let ff = self
            .findings_filters
            .get_mut(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))?;
        if let Some(n) = name {
            ff.name = n;
        }
        if let Some(d) = description {
            ff.description = Some(d);
        }
        if let Some(a) = action {
            ff.action = a;
        }
        if let Some(p) = position {
            ff.position = p;
        }
        if let Some(c) = finding_criteria {
            ff.finding_criteria = c;
        }
        Ok(ff.arn.clone())
    }

    pub fn delete_findings_filter(&mut self, id: &str) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.findings_filters
            .remove(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))?;
        Ok(())
    }

    pub fn list_findings_filters(&self) -> Result<Vec<&MacieFindingsFilter>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.findings_filters.values().collect())
    }

    // ── Classification jobs ───────────────────────────────────────────────────

    pub fn create_classification_job(
        &mut self,
        account_id: &str,
        region: &str,
        name: String,
        description: Option<String>,
        job_type: String,
        client_token: String,
        s3_job_definition: serde_json::Value,
        allow_list_ids: Vec<String>,
        custom_data_identifier_ids: Vec<String>,
        managed_data_identifier_ids: Vec<String>,
        managed_data_identifier_selector: Option<String>,
        sampling_percentage: Option<i32>,
        schedule_frequency: Option<serde_json::Value>,
        initial_run: bool,
        tags: HashMap<String, String>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let id = Self::next_id("job");
        let arn = format!("arn:aws:macie2:{}:{}:job/{}", region, account_id, id);
        let now = Utc::now();
        // Jobs are immediately completed — no real engine.
        let job = MacieClassificationJob {
            job_id: id.clone(),
            job_arn: arn.clone(),
            name,
            description,
            job_type: job_type.clone(),
            job_status: if job_type == "ONE_TIME" {
                "COMPLETE".to_string()
            } else {
                "RUNNING".to_string()
            },
            client_token,
            s3_job_definition,
            allow_list_ids,
            custom_data_identifier_ids,
            managed_data_identifier_ids,
            managed_data_identifier_selector,
            sampling_percentage,
            schedule_frequency,
            initial_run,
            tags: tags.clone(),
            created_at: now,
            last_run_time: Some(now),
        };
        self.classification_jobs.insert(id.clone(), job);
        self.resource_tags.insert(arn, tags);
        Ok(id)
    }

    pub fn get_classification_job(
        &self,
        job_id: &str,
    ) -> Result<&MacieClassificationJob, Macie2Error> {
        self.require_enabled()?;
        self.classification_jobs
            .get(job_id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(job_id.to_string()))
    }

    pub fn update_classification_job(
        &mut self,
        job_id: &str,
        job_status: &str,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        let job = self
            .classification_jobs
            .get_mut(job_id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(job_id.to_string()))?;
        job.job_status = job_status.to_string();
        Ok(())
    }

    pub fn list_classification_jobs(&self) -> Result<Vec<&MacieClassificationJob>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.classification_jobs.values().collect())
    }

    // ── Sensitivity inspection templates ─────────────────────────────────────

    pub fn get_sensitivity_inspection_template(
        &self,
        id: &str,
    ) -> Result<&MacieSensitivityInspectionTemplate, Macie2Error> {
        self.require_enabled()?;
        self.sensitivity_inspection_templates
            .get(id)
            .ok_or_else(|| Macie2Error::ResourceNotFound(id.to_string()))
    }

    pub fn upsert_sensitivity_inspection_template(
        &mut self,
        account_id: &str,
        id: Option<String>,
        name: String,
        description: Option<String>,
        excludes_managed_data_identifier_ids: Vec<String>,
        includes_allow_list_ids: Vec<String>,
        includes_custom_data_identifier_ids: Vec<String>,
        includes_managed_data_identifier_ids: Vec<String>,
    ) -> Result<String, Macie2Error> {
        self.require_enabled()?;
        let resolved_id = id.unwrap_or_else(|| format!("sit-{}", account_id));
        let tmpl = MacieSensitivityInspectionTemplate {
            id: resolved_id.clone(),
            name,
            description,
            excludes_managed_data_identifier_ids,
            includes_allow_list_ids,
            includes_custom_data_identifier_ids,
            includes_managed_data_identifier_ids,
        };
        self.sensitivity_inspection_templates
            .insert(resolved_id.clone(), tmpl);
        Ok(resolved_id)
    }

    pub fn list_sensitivity_inspection_templates(
        &self,
    ) -> Result<Vec<&MacieSensitivityInspectionTemplate>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.sensitivity_inspection_templates.values().collect())
    }

    // ── Automated discovery ───────────────────────────────────────────────────

    pub fn get_automated_discovery_config(
        &self,
    ) -> Result<Option<&MacieAutomatedDiscoveryConfig>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.automated_discovery_config.as_ref())
    }

    pub fn update_automated_discovery_config(
        &mut self,
        account_id: &str,
        status: String,
        auto_enable_organization_members: Option<String>,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        let now = Utc::now();
        match &mut self.automated_discovery_config {
            Some(cfg) => {
                let was_disabled = cfg.status == "DISABLED";
                cfg.status = status.clone();
                cfg.auto_enable_organization_members = auto_enable_organization_members;
                cfg.last_updated_at = now;
                if status == "DISABLED" {
                    cfg.disabled_at = Some(now);
                } else if was_disabled {
                    cfg.disabled_at = None;
                }
            }
            None => {
                self.automated_discovery_config = Some(MacieAutomatedDiscoveryConfig {
                    status: status.clone(),
                    auto_enable_organization_members,
                    classification_scope_id: format!("cs-{}", account_id),
                    sensitivity_inspection_template_id: format!("sit-{}", account_id),
                    first_enabled_at: if status != "DISABLED" {
                        Some(now)
                    } else {
                        None
                    },
                    disabled_at: if status == "DISABLED" {
                        Some(now)
                    } else {
                        None
                    },
                    last_updated_at: now,
                });
            }
        }
        Ok(())
    }

    pub fn list_automated_discovery_accounts(
        &self,
    ) -> Result<Vec<&MacieAutomatedDiscoveryAccount>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.automated_discovery_accounts.values().collect())
    }

    pub fn batch_update_automated_discovery_accounts(
        &mut self,
        updates: Vec<(String, String)>,
    ) -> Result<Vec<String>, Macie2Error> {
        self.require_enabled()?;
        let mut errors = Vec::new();
        for (account_id, status) in updates {
            if account_id.is_empty() {
                errors.push(account_id);
                continue;
            }
            self.automated_discovery_accounts.insert(
                account_id.clone(),
                MacieAutomatedDiscoveryAccount { account_id, status },
            );
        }
        Ok(errors)
    }

    // ── Reveal configuration ──────────────────────────────────────────────────

    pub fn get_reveal_config(&self) -> Result<Option<&MacieRevealConfig>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.reveal_config.as_ref())
    }

    pub fn update_reveal_config(
        &mut self,
        status: String,
        kms_key_id: Option<String>,
        retrieval_mode: String,
        role_name: Option<String>,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.reveal_config = Some(MacieRevealConfig {
            status,
            kms_key_id,
            retrieval_mode,
            role_name,
        });
        Ok(())
    }

    // ── Classification export configuration ──────────────────────────────────

    pub fn get_classification_export_config(
        &self,
    ) -> Result<Option<&MacieClassificationExportConfig>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.classification_export_config.as_ref())
    }

    pub fn put_classification_export_config(
        &mut self,
        raw: serde_json::Value,
    ) -> Result<serde_json::Value, Macie2Error> {
        self.require_enabled()?;
        self.classification_export_config =
            Some(MacieClassificationExportConfig { raw: raw.clone() });
        Ok(raw)
    }

    // ── Findings publication configuration ───────────────────────────────────

    pub fn get_findings_publication_config(
        &self,
    ) -> Result<Option<&MacieFindingsPublicationConfig>, Macie2Error> {
        self.require_enabled()?;
        Ok(self.findings_publication_config.as_ref())
    }

    pub fn put_findings_publication_config(
        &mut self,
        publish_classification_findings: bool,
        publish_policy_findings: bool,
    ) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        self.findings_publication_config = Some(MacieFindingsPublicationConfig {
            publish_classification_findings,
            publish_policy_findings,
        });
        Ok(())
    }

    // ── Findings operations ──────────────────────────────────────────────────

    pub fn create_sample_findings(&mut self, finding_types: &[String]) -> Result<(), Macie2Error> {
        self.require_enabled()?;
        let now = Utc::now();
        let types_to_create = if finding_types.is_empty() {
            vec![
                "SensitiveData:S3Object/Personal".to_string(),
                "Policy:IAMUser/S3BucketPublic".to_string(),
            ]
        } else {
            finding_types.to_vec()
        };
        for ft in types_to_create {
            let id = Self::next_id("finding");
            let finding = MacieFinding {
                id: id.clone(),
                finding_type: ft.clone(),
                severity: "Medium".to_string(),
                title: format!("Sample finding: {ft}"),
                description: format!("This is a sample finding of type {ft}"),
                created_at: now,
                updated_at: now,
                category: if ft.starts_with("Policy:") {
                    "POLICY".to_string()
                } else {
                    "CLASSIFICATION".to_string()
                },
                sample: true,
            };
            self.findings.insert(id, finding);
        }
        Ok(())
    }

    pub fn get_findings(&self, finding_ids: &[String]) -> Result<Vec<&MacieFinding>, Macie2Error> {
        self.require_enabled()?;
        let results: Vec<&MacieFinding> = finding_ids
            .iter()
            .filter_map(|id| self.findings.get(id))
            .collect();
        Ok(results)
    }

    pub fn get_finding_statistics(&self) -> Result<usize, Macie2Error> {
        self.require_enabled()?;
        Ok(self.findings.len())
    }
}
